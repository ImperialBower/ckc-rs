# Parallelism Audit

**Date:** 2026-08-07
**Scope:** whole repo (`ckc-rs` 0.1.18, `main` @ 21a15e4)
**Files surveyed:** `src/lib.rs`, `src/cards/mod.rs`, `src/cards/five.rs`,
`src/cards/six.rs`, `src/cards/seven.rs`, `src/cards/two.rs`,
`src/cards/three.rs`, `src/cards/four.rs`, `src/cards/binary_card.rs`,
`src/deck.rs`, `src/hand_rank.rs`, `src/parse.rs`, `src/lookups/*`
**Evidence sources:** **structural-only.** The repo has no benchmark
target, no `criterion` dev-dependency, no `[[bench]]` section
(`Cargo.toml:1-25`), and no profiling artifacts. Every payoff tier below
rests on loop-shape and hand-frequency reasoning, not measurement. Two
runtime claims *were* executed against the real crate and are marked as
such.
**Companion docs:** `README.md` (states the `no_std` / embedded / wasm
positioning that bounds every table-size decision, `README.md:29-33`);
`.superpowers/sdd/2026-07-25-ckc-rs-kernel-extraction/progress.md`
(sibling kernel-extraction effort — source of the exhaustive 5-card
oracle cited in §6).
**Predecessor:** none. This is the first run; no prior parallelism or
SIMD analysis exists in the repo.
**Method:** /smimd (pattern catalog + anchors:
`~/.claude/skills/smimd/references/pattern-catalog.md`)

**TL;DR:** There is exactly one dominant cost in this library and it is
not a vectorization problem — it is `Five::find_in_products`, a
13-iteration binary search over a 19.5 KB table that runs for every
paired hand, which is roughly half of all five-card hands and therefore
~10 times per seven-card evaluation. Replacing that search (branchless
first, perfect hash second) is worth more than every lane-level idea in
this audit combined, and it also removes a confirmed panic. Two cheap
SWAR wins follow: `BinaryCard::peel` linearly scans a 52-entry table
where `leading_zeros` answers in one instruction, and the 21-permutation
loops in `Six`/`Seven` recompute suit information that is constant across
all permutations. Explicit SIMD lanes are **not** recommended: the
per-hand data is five `u32`s, the real cost is table lookups rather than
arithmetic, and cross-hand lanes would need gathers that the crate's
`no_std`/wasm/embedded contract forbids. Thread-level parallelism belongs
to the consumer, not to this kernel — what the kernel owes it is a batch
entry point it currently does not have.

---

## 1. Hot-path architecture

**Card representation.** A card is a `u32` (`CKCNumber`, `src/lib.rs:33`)
packing four independent fields: a 13-bit rank flag (bits 16–28), a
4-bit suit flag (bits 12–15), a 4-bit rank ordinal (bits 8–11) and a
6-bit rank prime (bits 0–5). Every hot operation is a mask and a shift
over those fields (`src/lib.rs:658-690`). This is a good layout and none
of the findings below propose changing it.

**Hot path A — the five-card evaluator**
(`Five::hand_rank_value_and_hand`, `src/cards/five.rs:178-197`). One
evaluation is:

1. `or_rank_bits()` — 5 loads, 4 ORs, 1 shift (`src/cards/five.rs:159-166`)
2. `is_flush()` — 5 loads, 4 ANDs, 1 mask, 1 compare (`src/cards/five.rs:122-129`)
3. flush → `FLUSHES[i]`, one load from a 7937 × `u16` = **15.9 KB** table
   (`src/lookups/flushes.rs:5`)
4. else → `UNIQUE_5[i]`, one load from a second 15.9 KB table
   (`src/lookups/unique5.rs:4`). A non-zero entry means five distinct
   ranks (straight or high card) and the evaluation is done.
5. `UNIQUE_5[i] == 0` means the hand contains at least one pair, and the
   evaluator falls through to `not_unique()`
   (`src/cards/five.rs:105-107`): `multiply_primes()` (5 masks, 4
   multiplies, `src/cards/five.rs:150-157`) feeds
   `Five::find_in_products` (`src/cards/five.rs:85-103`) — a textbook
   binary search over `PRODUCTS`, 4888 × `u32` = **19.5 KB**
   (`src/lookups/products.rs:1`) — and the resulting index reads
   `VALUES`, 4888 × `u16` = 9.8 KB (`src/lookups/values.rs:1`).

Step 5 is the entire performance story. Steps 1–4 are ~15 ALU ops and one
cached load. Step 5 is ~13 iterations of a data-dependent compare-and-
branch, each iteration touching a different cache line of a 19.5 KB array
— an unpredictable branch and a likely L1 miss per iteration. It runs
for **every paired hand**: of the 2,598,960 five-card hands, 1,296,420
(49.9%) contain no pair and exit at step 4; the other 50.1% pay step 5.

**Hot path B — the seven-card evaluator**
(`Seven::hand_rank_value_and_hand`, `src/cards/seven.rs:139-154`). The
best five-card hand out of seven is found by brute force over the 21
`FIVE_CARD_PERMUTATIONS` (`src/cards/seven.rs:14-35`), each iteration
materializing a `Five` (`five_from_permutation`,
`src/cards/seven.rs:197-207`) and running hot path A end to end.
`Six` is the same shape over 6 permutations (`src/cards/six.rs:114-128`).

This is the multiplier that turns hot path A's step 5 from "a slow branch"
into "the dominant cost of the library": one seven-card evaluation runs
21 five-card evaluations, of which ~10 fall into the binary search,
costing on the order of **130 mispredict-prone, cache-scattered loads per
hold'em hand**.

**Hot path C — the validation path**
(`HandValidator::is_valid`, `src/cards/mod.rs:52-54`). Every
`*_validated` entry point — including the crate's headline
`evaluate::five_cards` (`src/lib.rs:332-334`) — runs `are_unique()` (a
full sort for `Six`/`Seven`, `src/cards/seven.rs:165-176`) plus
`is_corrupt()`, which calls the 52-arm `CardNumber::filter` match
(`src/lib.rs:475-529`) once per card. This is a fixed per-evaluation
cost, paid once, not once per permutation — which is exactly why it
ranks below hot paths A and B.

**Hot path D — bitset iteration** (`BinaryCard`, `src/cards/binary_card.rs`).
A `u64` bitset of the 52 cards, already SWAR-shaped: `number_of_cards`
is `count_ones` (`src/cards/binary_card.rs:288-290`), `has` is a mask
test (`:273-275`), `fold_in` is an OR (`:257-259`). The one exception is
`peel` (`:298-306`), the primitive that extracts cards one at a time.

## 2. Findings

### F1. `Five::find_in_products` — replace the binary search

- **Class:** algorithm-first
- **Hot path & evidence tier:** hot paths A and B (amplified 21× by
  `Seven`); **structural** — loop shape plus exact hand-frequency
  combinatorics. No benchmark exists to confirm the tier.
- **Payoff:** **dominant-cost** — "The finding sits in the profile's (or
  structural analysis's) single largest cost center; success visibly
  moves the headline benchmark"
- **Risk:** tiered. Tier 1 is **safe-stable** — "Stable toolchain, no new
  dependencies, no unsafe, output bit-identical, small diff". Tier 2 is
  **feature-gated** — "New dependency or nightly feature, gated off by
  default; scalar path remains the shipped default until benchmarks
  justify flipping". Tier 2 sits between `safe-stable` and `heavy`: it
  adds a generated table but *replaces* two existing ones, so the memory
  budget does not grow. I take the more conservative of the two anchors
  because a generated table is a new provenance obligation, not because
  of its size.
- **Evidence:** `src/cards/five.rs:85-103` (the search),
  `src/cards/five.rs:105-107` (its only caller),
  `src/cards/five.rs:190-193` (the `UNIQUE_5[i] == 0` fall-through that
  gates it), `src/lookups/products.rs:1` (4888 entries → ⌈log₂ 4888⌉ = 13
  iterations worst case), `src/cards/seven.rs:144-151` (the 21× loop).
  Frequency: 1,302,540 of 2,598,960 five-card hands (50.1%) are paired
  and reach the search.
- **Ordering discipline:**
  - *Algorithm-first:* this **is** the algorithm-first finding — a
    data-dependent search over a sorted array, the catalog's canonical
    "perfect hash replacing search" case. No lane work is proposed.
  - *Amdahl:* the serial tail is the surrounding ~15 ALU ops of hot path
    A, which are perhaps 10% of a paired-hand evaluation. The tail does
    not dominate; the search does.
  - *Auto-vectorization:* not applicable and not wanted — a
    data-dependent binary search is precisely what the compiler cannot
    and should not vectorize.
  - *Data size:* not applicable; this is a latency finding on a single
    lookup, not a throughput finding over a large array.
- **Recommended transform (tiered — do them in this order):**
  1. **Branchless / Eytzinger search, safe-stable, no table change.**
     Keep `PRODUCTS` as-is and make the search predictable: replace the
     `if/else if/else` ladder with a conditional-move formulation
     (`base += (key > PRODUCTS[base + half]) as usize * half`) over a
     fixed 13 iterations, or re-lay `PRODUCTS` in Eytzinger (BFS) order
     so each probe is prefetch-friendly. Same 19.5 KB, no new
     dependency, bit-identical output. This alone removes the branch
     mispredictions, which are the larger half of the cost.
  2. **Senzee-style perfect hash on the prime product, feature-gated.**
     The classic improvement to exactly this evaluator (documented
     alongside Cactus Kev's original C, which this crate descends from —
     `src/lookups/mod.rs:1-6`): hash the prime product with a fixed
     multiply/shift/xor chain into a direct index, turning ~13 dependent
     branchy loads into one arithmetic chain and one load. It replaces
     both `PRODUCTS` (19.5 KB) and `VALUES` (9.8 KB) with a hash-value
     table of comparable size, so the crate's embedded/wasm memory
     budget is unaffected — worth stating explicitly in the PR, because
     "perfect hash" reads like a memory regression and here it is not.
  3. Do **not** stop at 1 if benchmarks show the search still visible;
     do not jump to 2 before the harness in TODO-PAR-1 exists.
- **Confirmed defect surfaced by this finding (not a parallelism item).**
  `find_in_products` underflows when `key` is smaller than
  `PRODUCTS[0]` (= 48): `high = mid - 1` at `src/cards/five.rs:95` with
  `mid == 0` wraps `usize`. This is reachable from safe public API — a
  `Five` containing `CardNumber::BLANK` has a prime product of 0, and
  `Five::default().hand_rank_value()` walks straight into it, because
  `hand_rank_value()` (unlike `hand_rank_value_validated()`) does not
  validate. Executed against this working tree:
  - debug: `panicked at src/cards/five.rs:95:24: attempt to subtract with overflow`
  - release: `panicked at src/cards/five.rs:93:27: index out of bounds: the len is 4888 but the index is 9223372036854775807`

  Both tiers of the transform above eliminate the underflow structurally.
  In a `no_std` crate aimed at embedded targets a reachable panic is
  worth fixing on its own schedule — see TODO-PAR-8.

### F2. Hoist suit analysis out of the 21-permutation loops

- **Class:** algorithm-first (with a SWAR suit histogram)
- **Hot path & evidence tier:** hot path B; **structural**
- **Payoff:** **significant** — "Hot path, but one cost among several;
  success moves a named benchmark measurably, not the headline"
- **Risk:** **safe-stable** — "Stable toolchain, no new dependencies, no
  unsafe, output bit-identical, small diff"
- **Evidence:** `src/cards/seven.rs:139-154` and `src/cards/six.rs:114-128`
  (the loops), `src/cards/five.rs:122-129` (`is_flush`/`and_bits` — 5
  loads and 5 ANDs recomputed inside every one of the 21 iterations),
  `src/cards/five.rs:184` (the branch it feeds).
- **Ordering discipline:**
  - *Algorithm-first:* yes — the win is removing work, not
    parallelizing it. Whether a flush is possible is a property of the
    **seven cards**, invariant across all 21 permutations, yet it is
    recomputed 21 times from scratch.
  - *Amdahl:* the serial tail here is F1's binary search, which
    dominates. This finding is explicitly ranked below F1 for that
    reason: land F1 first, or this one's measurable share stays small.
  - *Auto-vectorization:* LLVM cannot hoist `is_flush` out of the loop —
    each iteration builds a fresh `Five` from a different permutation, so
    the AND-reduction is over different values each time. The invariance
    is a domain fact, not one the compiler can see.
  - *Data size:* 21 iterations. Far too small for lanes; correct as a
    scalar restructure.
- **Recommended transform:** compute a 4-entry suit histogram once per
  `Seven` — four masked `count_ones` over the seven suit bits, pure SWAR
  — and branch on it:
  - **No suit reaches 5** (~97% of seven-card hands): no permutation can
    be a flush. Take a non-flush evaluation path through the 21
    permutations that skips `and_bits`/`is_flush` entirely — 21 × (5
    loads + 5 ANDs + compare) removed per hand.
  - **Some suit reaches 5** (~3%): the answer is a flush or straight
    flush **in that suit only**, resolvable in O(1) from that suit's
    OR'd rank bits (straight-flush check via the existing
    `STRAIGHT_PADDING`/`WHEEL_OR_BITS` logic at
    `src/cards/five.rs:12-15,131-137`, else the top five rank bits) —
    skipping all 21 evaluations. Rare, but it is the branch where the
    current loop wastes the most.

  Both branches are bit-identical to today's output and must be proved so
  against the exhaustive oracle (§6). The second branch is the one that
  needs the careful review: with six or seven suited cards the naive
  `FLUSHES[or_rank_bits]` index has too many bits set and reads a zero
  entry, so the "pick the best five" step must be explicit.

### F3. `BinaryCard::peel` — 52-entry linear scan where `leading_zeros` answers

- **Class:** SWAR
- **Hot path & evidence tier:** hot path D; **structural**, and weaker
  than F1/F2's structural case — nothing *inside* this crate calls `peel`
  in a loop. It is a public primitive whose only sensible use is
  iterating a bitset card by card, which is inherently a loop at the
  consumer.
- **Payoff:** **significant** — "Hot path, but one cost among several;
  success moves a named benchmark measurably, not the headline"
- **Risk:** **safe-stable** — "Stable toolchain, no new dependencies, no
  unsafe, output bit-identical, small diff". This is the smallest diff
  in the audit: roughly six lines.
- **Evidence:** `src/cards/binary_card.rs:298-306` — iterates
  `BinaryCard::DECK` (52 × `u64`, `:92`), testing `*self & bc == bc` per
  entry, averaging ~26 iterations and 26 loads to extract one card. The
  neighbouring methods already use the right idiom:
  `number_of_cards` is `count_ones` (`:288-290`), `is_valid` masks
  `OVERFLOW` (`:283-285`).
- **Ordering discipline:**
  - *Algorithm-first:* satisfied by the SWAR transform itself — `DECK`
    is a descending sequence of single bits, so the scan is an
    open-coded "find the highest set bit", which is one instruction.
  - *Amdahl:* no serial tail; the scan is the whole function.
  - *Auto-vectorization:* LLVM will not turn a table scan with an early
    return into `lzcnt`. It cannot know `DECK` enumerates single bits in
    descending order.
  - *Data size:* 64 bits — one register. This is the catalog's SWAR
    shape exactly, not a lane candidate.
- **Recommended transform:** mask off the reserved high bits with the
  existing `BinaryCard::OVERFLOW` (`:72`), then
  `1u64 << (63 - masked.leading_zeros())` for the extracted bit, XOR it
  out, and return `BLANK` when the masked value is zero. Constant time,
  branch-free apart from the empty check, bit-identical for every input
  — including the currently-untested case of a value with `OVERFLOW`
  bits set, whose behaviour must be pinned by a test *before* the change
  (today's scan silently ignores those bits; the naive `leading_zeros`
  version would not).

### F4. No batch entry point — the API shape blocks both MIMD and SIMD

- **Class:** composition (threads × lanes) — as an *enabler*, not an
  implementation
- **Hot path & evidence tier:** hot paths A and B, viewed from the
  consumer; **structural**
- **Payoff:** **significant** — "Hot path, but one cost among several;
  success moves a named benchmark measurably, not the headline". Nothing
  gets faster on the day this lands; it is the prerequisite that makes
  every consumer-side parallelism strategy expressible.
- **Risk:** **safe-stable** — "Stable toolchain, no new dependencies, no
  unsafe, output bit-identical, small diff". Purely additive API.
- **Evidence:** every evaluation entry point is one hand per call through
  `&self` — `src/lib.rs:332-334` (`evaluate::five_cards` takes a single
  `[CKCNumber; 5]`), `src/cards/mod.rs:13-30` (the `HandRanker` trait has
  no slice form). The README's own use cases
  (`README.md:53-58` — heads-up equity, Nash push/fold, ICM) are
  Monte-Carlo workloads that evaluate millions of independent hands, and
  there is no shape in this API through which a caller can hand the
  library a batch.
- **Ordering discipline:**
  - *Algorithm-first:* nothing algorithmic to fix; this is an interface
    gap.
  - *Amdahl:* the serial tail *is* F1 — batching a slow scalar kernel
    multiplies a slow kernel. Sequence this after F1.
  - *Auto-vectorization:* a batch loop over independent hands is the one
    shape in this crate LLVM might partially vectorize (the mask/OR
    prologue), and the only shape where later explicit lanes could ever
    apply. Today no such loop exists for it to try.
  - *Data size:* the whole point — a batch API is what makes "thousands
    of elements per call" possible at all.
- **Recommended transform:** add slice-shaped entry points alongside the
  existing scalar ones —
  `evaluate::five_cards_batch(hands: &[[CKCNumber; 5]], out: &mut [HandRankValue])`
  and the seven-card equivalent — implemented today as a plain scalar
  loop. Caller-provided output slices keep the crate `no_std` and
  allocation-free (`src/lib.rs:1`, `:5`). Deliberately **do not** add
  rayon: this crate is `no_std` with only `extern crate alloc`, rayon
  requires `std`, and thread-level parallelism is correctly the
  consumer's layer — the catalog's MIMD transform (indexed
  `into_par_iter` with per-worker accumulators) belongs in whatever
  simulator drives this kernel. What the kernel owes that consumer is the
  batch shape, and the structure-of-arrays option it opens up later
  (see DP-2).

### F5. Validation path — 52-arm match per card, full sort per hand

- **Class:** algorithm-first / SWAR
- **Hot path & evidence tier:** hot path C; **structural**
- **Payoff:** **marginal** — "Real but small: guaranteed-vs-hoped
  auto-vectorization, allocation shaving, cold-adjacent cleanup". It is
  paid once per evaluation, not once per permutation, so against hot path
  B's 21 evaluations it is a few percent at most.
- **Risk:** **safe-stable** — "Stable toolchain, no new dependencies, no
  unsafe, output bit-identical, small diff"
- **Evidence:** `src/cards/mod.rs:48-50` (`is_corrupt` calls
  `CardNumber::filter` per card), `src/lib.rs:475-529` (the 52-arm match
  over sparse `u32` constants — LLVM cannot build a jump table from
  these, so it emits a compare tree), `src/cards/seven.rs:165-176`
  (`are_unique` sorts all seven cards to detect duplicates),
  `src/cards/five.rs:206-208` (`Five::are_unique` is O(n²) over 5
  elements, which is fine at that size).
- **Ordering discipline:**
  - *Algorithm-first:* yes on both halves. Validity is a *derivable*
    property, not a set-membership question: a `CKCNumber` is valid iff
    its rank flag and suit flag are each a single bit in range and its
    prime/ordinal fields match the canonical values for that rank — two
    13-entry const arrays indexed by `rank_bit.trailing_zeros()` plus two
    `count_ones` checks replace the compare tree. Uniqueness for
    `Six`/`Seven` is a popcount question, not a sorting question.
  - *Amdahl:* dominated by hot paths A and B. This is why the tier is
    marginal rather than significant.
  - *Auto-vectorization:* not applicable.
  - *Data size:* 5–7 cards. Scalar restructure only.
  - **Caveat that keeps this last:** the obvious SWAR uniqueness check —
    `BinaryCard::from_seven(seven).number_of_cards() == 7` — is a trap.
    `from_seven` calls `from_ckc` per card (`src/cards/binary_card.rs:148`,
    `:246-255`), which is *another* 52-arm match, so it would be slower
    than the sort it replaces. The right form derives the bit
    arithmetically from the rank and suit fields rather than matching.
- **Recommended transform:** replace `CardNumber::filter`'s match with
  field derivation as above (keeping the public `filter` signature and
  semantics unchanged), and replace `Six`/`Seven`'s sort-based
  `are_unique` with an OR-accumulate over arithmetically-derived card
  bits plus one `count_ones`. Do this only after TODO-PAR-1 exists and
  only if the harness shows validation is visible at all — it may
  legitimately measure as noise, in which case this finding retires.

## 3. Not worth it

- **SIMD lanes inside one five-card evaluation.** Five `u32`s is below
  any useful lane width, and the arithmetic (4 ORs, 4 ANDs, 4 multiplies)
  is already ~2–3 cycles. The cost is the table lookup, not the math.
  This is the catalog's named trap verbatim — "vectorizing the 10% around
  a serial table lookup". Fix F1 first; there is nothing left for lanes
  afterwards either.
- **SIMD across the 21 permutations of a `Seven`.** Fails step 3 of the
  five-step shape: each lane would need its own data-dependent table
  index, i.e. a gather. `core::simd`'s gather is nightly, and the crate's
  `no_std`/wasm/embedded contract (`README.md:29-33`, `src/lib.rs:1`)
  rules out `core::arch` gather intrinsics on the default path. Routed to
  DP-2 as a decision, not a finding.
- **rayon anywhere in this crate.** `#![no_std]` with only `alloc`
  (`src/lib.rs:1-5`); rayon requires `std`. Adding it would forfeit the
  crate's entire stated positioning to move parallelism one layer down
  from where it belongs. MIMD is the consumer's job; F4 is what the
  kernel owes it.
- **`Five::sort_in_place`** (`src/cards/five.rs:220-223`). A 5-element
  `sort_unstable` + `reverse`. A sorting network would be faster in
  isolation, but it runs **once** per `Seven` evaluation — on the winning
  hand only (`src/cards/seven.rs:153`) — against 21 evaluations. Small-N
  and Amdahl-dominated.
- **`HandRank::determine_class`** (`src/hand_rank.rs:46-...`, several
  hundred match arms). Presentational: it converts a computed
  `HandRankValue` into a name for display, and the arms are dense integer
  ranges that LLVM compiles to a compact search tree. Cold-adjacent to
  every hot path — an evaluator loop calls `hand_rank_value()`, not
  `hand_rank()`.
- **Auto-vectorizing `or_bits`/`and_bits`** (`src/cards/five.rs:118-166`).
  LLVM will not vectorize a 5-element reduction over struct fields, and
  making it "guaranteed" would win nothing measurable — the marginal
  anchor's "guaranteed-vs-hoped auto-vectorization" case, below the bar
  for a finding.
- **`Deck` / `POKER_DECK`** (`src/deck.rs`). A `const` array with an
  O(1) indexed accessor. Nothing to parallelize.
- **`BinaryCard::from_index`** (`src/cards/binary_card.rs:262-271`).
  String parsing; a setup path, not a data plane.

## 4. Decision points

**DP-1 — A direct seven-card evaluator (big-table architecture swap).**
The brute-force 21-permutation loop (`src/cards/seven.rs:144`) is the
structural reason hold'em evaluation costs 21 five-card evaluations. The
known alternatives — the Two Plus Two 32-million-entry state-machine
table (~130 MB as `u32`) or a Senzee-style seven-card perfect hash over
the 52-bit card bitset (~100 MB) — answer in a handful of dependent
loads. Both are flatly incompatible with the crate's stated embedded/wasm
budget (`README.md:29-33`).
**Trigger:** a throughput target that F1 + F2 demonstrably cannot meet
(measured, per §6), *and* a consumer that can afford ~100 MB. If taken,
it should be a separate crate or a non-default feature that ships the
generator rather than the table, never the default `ckc-rs` build. Route
to `/epic`.

**DP-2 — SIMD across independent hands (structure-of-arrays + gather).**
The only genuinely SIMD-shaped opportunity in this domain: transpose N
hands into SoA lanes and evaluate 8 at a time. The mask/OR/multiply
prologue vectorizes cleanly; the table lookups need gather instructions.
**Trigger:** all of — F1 landed, F4's batch API landed, benchmarks show
the *table-load* latency (not the search) dominant, and the consumer
targets AVX2 or aarch64 exclusively. Would ship as a feature-gated
nightly `core::simd` path with the scalar path remaining default, and
must not regress the `no_std`/wasm build. Route to `/epic` if taken.

**DP-3 — Generated-table provenance.** If F1 tier 2 lands, `PRODUCTS` and
`VALUES` are replaced by a table nobody can review by eye. Decide up front
whether the generator lives in the repo with a table-identity test, or
whether the table is checked in as an opaque artifact. There is direct
precedent to copy: the sibling kernel-extraction effort built exactly
this pair — a `tools/oracle-gen` generator and a `tests/table_identity.rs`
that re-derives and compares
(`.superpowers/sdd/2026-07-25-ckc-rs-kernel-extraction/progress.md`).
Note that `tools/oracle-gen/` currently exists in this working tree as
untracked leftovers (a `Cargo.lock` and a `target/` directory, with no
source), which is itself worth resolving before it confuses someone.
**Trigger:** the moment TODO-PAR-4 is scheduled. This is a decision, not
code — make it before writing the generator, not after.

## 5. TODO checklist

Ordered by payoff per unit of risk. IDs are identity, not order.

- [ ] **TODO-PAR-1** — Add a benchmark harness. `criterion` as a
  dev-dependency only (dev-deps do not affect the `no_std` contract or
  consumer builds — the sibling effort confirmed `cargo tree -e normal`
  is the correct zero-dep assertion, plain `cargo tree` counts dev
  edges). Three benches: `five_cards` over a fixed shuffled corpus,
  `seven_cards` over a fixed corpus, and a **paired-hands-only** corpus
  that isolates F1's binary search. *Nothing else on this list should be
  merged before this exists* — every tier in this audit is
  structural-only and needs confirming.
  Files: `Cargo.toml`, `benches/`.
- [ ] **TODO-PAR-2** — F3: rewrite `BinaryCard::peel` using
  `leading_zeros` over the `OVERFLOW`-masked value. Pin current
  behaviour for `OVERFLOW`-bit inputs with a test *first*.
  Files: `src/cards/binary_card.rs:298-306`. Verify: exhaustive — all 52
  single-bit values, plus randomized multi-card sets peeled to empty,
  compared against the current implementation.
- [ ] **TODO-PAR-8** — Fix the confirmed `find_in_products` underflow
  (`src/cards/five.rs:95`) as a standalone correctness change, ahead of
  any restructuring, so the fix is not entangled with a performance diff.
  Reachable from safe public API via `Five::default().hand_rank_value()`.
  Verify: regression test asserting a defined return for
  `find_in_products(0)` and for every `Five` containing
  `CardNumber::BLANK`, under both debug and release.
- [ ] **TODO-PAR-3** — F1 tier 1: branchless or Eytzinger
  `find_in_products`, no table change.
  Files: `src/cards/five.rs:85-103` (+ `src/lookups/products.rs` if
  Eytzinger). Verify: exhaustive five-card oracle, plus before/after on
  TODO-PAR-1's paired-hands bench.
- [ ] **TODO-PAR-4** — F1 tier 2: Senzee-style perfect hash replacing
  `PRODUCTS` + `VALUES`, behind a feature gate, scalar search remaining
  default until the bench justifies flipping. Resolve **DP-3** first.
  Files: `src/cards/five.rs`, `src/lookups/`, generator under `tools/`.
- [ ] **TODO-PAR-5** — F2: hoist the suit histogram out of the
  `Six`/`Seven` permutation loops; add the O(1) flush/straight-flush
  resolution branch.
  Files: `src/cards/seven.rs:139-154`, `src/cards/six.rs:114-128`,
  `src/cards/five.rs:122-137`. Verify: exhaustive seven-card sweep
  (C(52,7) = 133,784,560) offline; sampled in CI.
- [ ] **TODO-PAR-6** — F4: add `*_batch` slice entry points with
  caller-provided output slices. No rayon, no allocation.
  Files: `src/lib.rs:322-349`, `src/cards/mod.rs`. Verify: batch output
  equals per-hand scalar output for a large corpus.
- [ ] **TODO-PAR-7** — F5: derive validity arithmetically in
  `CardNumber::filter`; replace `Six`/`Seven` sort-based `are_unique`
  with an OR + `count_ones` over derived bits. Only if TODO-PAR-1 shows
  validation is measurable.
  Files: `src/lib.rs:475-529`, `src/cards/seven.rs:165-176`,
  `src/cards/six.rs:140-...`. Verify: `filter` agrees with the current
  match on all 2³² inputs (a few seconds, exhaustive and worth doing).

### Superseded

None — first run.

## 6. Verification

- **Invariance.** Every finding above must be **bit-identical**, not
  merely "still passes the unit tests". The instrument already exists in
  the sibling kernel-extraction work: an exhaustive five-card oracle over
  all C(52,5) = 2,598,960 hands, independently validated (5,197,920
  bytes; 7,462 distinct ranks; exactly 4 hands at rank 1; zero
  zero-values; sha256 `c6f163de…20b8d`, per
  `.superpowers/sdd/2026-07-25-ckc-rs-kernel-extraction/progress.md`).
  Regenerate it into this repo as a golden fixture and gate TODO-PAR-3,
  -4, -5 and -7 on an exact match. For hot path B, the equivalent
  seven-card sweep is C(52,7) = 133,784,560 — run it in full offline for
  TODO-PAR-5, and a fixed-seed sample of it in CI. TODO-PAR-2's
  invariance is small enough to be exhaustive outright.
- **Measurement.** There is no harness today, which is why TODO-PAR-1 is
  first. The minimum worth adding: `criterion` (dev-dependency only, so
  the `no_std` contract and the zero-normal-dependency property are
  untouched — assert with `cargo tree -e normal`, not plain `cargo tree`)
  with a `five_cards` bench, a `seven_cards` bench, and a paired-hands
  bench that isolates F1. Record a baseline on `main` before the first
  change, and report before/after on the specific bench each TODO
  targets — TODO-PAR-3/-4 against the paired-hands bench, TODO-PAR-5
  against `seven_cards`, TODO-PAR-6 against a new batch bench. Since
  every payoff tier in this audit is structural-only, the first
  measurement may reorder this list; that is the point, and a re-run of
  `/smimd` afterwards should record the `Δ`.
- **Feature-gate policy.** The scalar, stable, `no_std` path stays the
  shipped default until benchmarks justify flipping. Named gates:
  TODO-PAR-4 ships behind a `perfect-hash` feature, off by default, with
  the existing binary search as the default path. TODO-PAR-2, -3, -5, -6,
  -7 and -8 are unconditional (no gate) because each is stable, safe, and
  output-identical — but each still needs its before/after numbers before
  merge. DP-2, if ever taken, ships behind a nightly-only feature that
  must never be implied by a default build, and the `no_std`/wasm target
  must keep building with it absent.

## Notes (human)

<!-- Preserved verbatim across regenerations. Never edited by /smimd. -->
