.PHONY: clean build test build_test fmt fmt-check clippy clippy-pedantic create_docs ayce default help docs test-nightly clippy-nightly nightly tree tree-duplicates audit unused-deps install-tools watch install-watch nextest heavy features no-std zero-deps semver msrv package oracle oracle-verify mutants mutants-diff coverage coverage-open ci ci-fresh ci-local

# Default target
default: ayce

# Display help information
help:
	@echo "Available targets:"
	@echo "  make (default)       - Run ayce"
	@echo "  make build           - Build the project"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make test            - Run tests"
	@echo "  make ci              - Mirror GitHub Actions test job (RUSTFLAGS=-Dwarnings)"
	@echo "  make ci-fresh        - Like ci, but 'cargo update' first to match CI's fresh deps"
	@echo "  make ci-local        - Run every GitHub Actions job locally (all 9 gates)"
	@echo "  make nextest         - Run tests with cargo-nextest (installs if missing)"
	@echo "  make heavy           - Run the ignored exhaustive seven-card test (slow)"
	@echo "  make features        - Test each feature combination the crate ships"
	@echo "  make build_test      - Clean, build, nextest, and doc tests"
	@echo "  make fmt             - Format code"
	@echo "  make fmt-check       - Check formatting without writing (CI's Fmt job)"
	@echo "  make clippy          - Run clippy linter"
	@echo "  make clippy-pedantic - Clippy at CI strictness (--all-features --all-targets)"
	@echo "  make create_docs     - Build documentation"
	@echo "  make docs            - Build docs and open in browser"
	@echo "  make ayce            - Run fmt, build_test, clippy, kernel gates, and docs"
	@echo "  make help            - Display this help message"
	@echo ""
	@echo "Kernel gates (EPIC-80 — what makes this a kernel, not just a library):"
	@echo "  make no-std          - Build for thumbv7em-none-eabi and wasm32-unknown-unknown"
	@echo "  make zero-deps       - Assert the default dependency tree is exactly this crate"
	@echo "  make msrv            - Build and test on the declared MSRV (1.85)"
	@echo "  make semver          - Check the public API against the last release"
	@echo "  make package         - Package the crate and verify it builds from the tarball"
	@echo ""
	@echo "Golden oracle:"
	@echo "  make oracle-verify   - Check tests/golden/five_card_ranks.bin against its sha256"
	@echo "  make oracle          - REGENERATE the fixture from published ckc-rs 0.1.18"
	@echo ""
	@echo "Nightly:"
	@echo "  make test-nightly    - Run all tests with nightly"
	@echo "  make clippy-nightly  - Run clippy with nightly and deny warnings"
	@echo "  make nightly         - Run nightly test and clippy checks"
	@echo "  make unused-deps     - Find unused dependencies with cargo-udeps"
	@echo ""
	@echo "Dependencies and Security:"
	@echo "  make tree            - Show dependency tree"
	@echo "  make tree-duplicates - Show duplicate dependencies"
	@echo "  make audit           - Run advisory-only security audit"
	@echo ""
	@echo "Tools and Workflow:"
	@echo "  make install-tools   - Install nextest, semver-checks, mutants, llvm-cov, deny"
	@echo "  make watch           - Run cargo-watch for check/test loop"
	@echo "  make install-watch   - Install cargo-watch"
	@echo "  make mutants         - Run cargo-mutants on the whole codebase (slow)"
	@echo "  make mutants-diff    - Run cargo-mutants only on files changed vs main"
	@echo "  make coverage        - Generate HTML code coverage report"
	@echo "  make coverage-open   - Generate HTML coverage report and open in browser"
	@echo ""

# Clean build artifacts
clean:
	cargo clean

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test

# Mirror the GitHub Actions test job exactly: warnings are hard errors and
# incremental compilation is off. Uses the CURRENT Cargo.lock.
ci:
	RUSTFLAGS="-Dwarnings" CARGO_INCREMENTAL=0 cargo test --all

# Like `ci`, but first re-resolves dependencies to the latest compatible
# versions. Cargo.lock is gitignored, so CI resolves fresh on every run — this
# is what catches breakage arriving from newer transitive deps before it
# reaches GitHub. This exact failure mode killed the old 1.70.0 matrix leg,
# where a freshly-resolved edition-2024 `toml_edit` (via rstest_macros) was
# unparseable by that toolchain's Cargo.
ci-fresh:
	cargo update
	RUSTFLAGS="-Dwarnings" CARGO_INCREMENTAL=0 cargo test --all

# Run every gate in .github/workflows/CI.yaml locally, in the same order.
# Cheap checks first so a failure surfaces fast.
ci-local: fmt-check zero-deps clippy-pedantic ci no-std semver
	@echo ""
	@echo "✓ All CI gates passed locally."

# Run the ignored exhaustive seven-card test. Not in CI because of runtime;
# this is the strongest seven-card gate the repo has, so run it before a release.
heavy:
	cargo test --test seven_card -- --ignored --nocapture

# Exercise every feature combination the crate ships. `standard52` alone is the
# no_std kernel; the default set adds std; --all-features adds serde.
features:
	@echo "--- no-default-features, standard52 only (the no_std kernel) ---"
	cargo test --no-default-features --features standard52
	@echo "--- default features (standard52 + std) ---"
	cargo test
	@echo "--- all features (adds serde) ---"
	cargo test --all-features

# Run tests with cargo-nextest (installs if not present)
nextest:
	@if ! cargo nextest --version >/dev/null 2>&1; then \
		echo "cargo-nextest is not installed."; \
		printf "Would you like to install it now? [y/N] "; \
		read answer; \
		if [ "$$answer" = "y" ] || [ "$$answer" = "Y" ]; then \
			cargo install --locked cargo-nextest; \
		else \
			echo "Skipping. Run 'cargo install cargo-nextest' to install manually."; \
			exit 1; \
		fi; \
	fi
	cargo nextest run --all-features

# Clean once, then build, run nextest, and run doc tests
build_test: clean build nextest
	cargo test --doc --all-features

# Format code
fmt:
	cargo fmt --all

# CI's Fmt job: fail rather than rewrite
fmt-check:
	cargo fmt --all -- --check

# Run clippy linter
clippy:
	cargo clippy -- -W clippy::pedantic

# CI's `clippy pedantic` job, verbatim. Strictly dominates the plain `clippy`
# target: --all-targets lints test code too, and -Dwarnings alone would NOT
# enable the pedantic group.
clippy-pedantic:
	cargo clippy --all-features --all-targets -- -Dclippy::all -Dclippy::pedantic -Dwarnings

test-nightly:
	cargo +nightly test --all-targets --all-features

clippy-nightly:
	cargo +nightly clippy --lib --all-features -- -D warnings

nightly: test-nightly clippy-nightly

# ---------------------------------------------------------------------------
# Kernel gates. These lock in the properties that make ckc-rs a kernel rather
# than just a library: it builds without an OS, it pulls in nothing, and its
# public surface changes only when the version says it does. Each mirrors a
# job in .github/workflows/CI.yaml.
# ---------------------------------------------------------------------------

# CI's `no_std targets` job. `rustup target add` is idempotent, so this is
# safe to re-run; it also means a fresh clone needs no manual setup.
no-std:
	rustup target add thumbv7em-none-eabi wasm32-unknown-unknown
	@echo "--- bare metal (thumbv7em-none-eabi) ---"
	cargo build --no-default-features --features standard52 --target thumbv7em-none-eabi
	@echo "--- wasm32 (wasm32-unknown-unknown) ---"
	cargo build --no-default-features --features standard52 --target wasm32-unknown-unknown

# CI's `zero default dependencies` job. `-e normal` is load-bearing: without it
# `cargo tree` also walks dev-dependency edges, so rstest and its ~47
# transitive crates get counted and the assertion is permanently red.
zero-deps:
	@cargo tree -e normal --no-default-features --features standard52
	@count=$$(cargo tree -e normal --no-default-features --features standard52 | wc -l | tr -d ' '); \
	echo "dependency tree lines: $$count"; \
	if [ "$$count" -ne 1 ]; then \
		echo "::error::Zero-dependency gate failed — the default tree must be exactly ckc-rs."; \
		exit 1; \
	fi; \
	echo "Zero-dependency gate passed."

# Build and test on the declared MSRV. rust-toolchain.toml pins 1.85 and
# outranks `rustup default`, so an explicit `+1.85` here is belt-and-braces
# rather than load-bearing — but it documents intent and survives the pin
# being raised for local development.
msrv:
	rustup toolchain install 1.85 --component clippy rustfmt
	cargo +1.85 test --all

# CI runs cargo-semver-checks inside the `clippy` job. Note that job carries
# `if: github.event_name != 'pull_request'`, so on a PR this gate does NOT
# run on GitHub — making the local target the only place it happens.
semver:
	@if ! cargo semver-checks --version >/dev/null 2>&1; then \
		echo "cargo-semver-checks is not installed."; \
		printf "Would you like to install it now? [y/N] "; \
		read answer; \
		if [ "$$answer" = "y" ] || [ "$$answer" = "Y" ]; then \
			cargo install --locked cargo-semver-checks; \
		else \
			echo "Skipping. Run 'cargo install cargo-semver-checks' to install manually."; \
			exit 1; \
		fi; \
	fi
	cargo semver-checks check-release

# Package the crate and compile it from the resulting tarball. Catches the
# class of bug where an `exclude` in Cargo.toml removes a file the build or
# a test actually needs — `tests/golden/*` and `tools/*` are both excluded.
package:
	cargo package

# ---------------------------------------------------------------------------
# Golden oracle. tests/golden/five_card_ranks.bin holds the hand rank of every
# one of the 2,598,960 five-card hands as produced by the frozen ckc-rs 0.1.18.
# It is the fixed point of the whole 0.2 migration.
# ---------------------------------------------------------------------------

# Verify the fixture is byte-for-byte what it was when the migration was
# proven. Portable across macOS (shasum) and Linux (sha256sum).
oracle-verify:
	@if command -v sha256sum >/dev/null 2>&1; then \
		sha256sum -c tests/golden/five_card_ranks.sha256; \
	elif command -v shasum >/dev/null 2>&1; then \
		shasum -a 256 -c tests/golden/five_card_ranks.sha256; \
	else \
		echo "No sha256 tool found (tried sha256sum and shasum)."; \
		exit 1; \
	fi

# REGENERATE the fixture from the PUBLISHED ckc-rs 0.1.18, then re-verify.
# You should almost never need this — the fixture is committed, and rebuilding
# it is only correct if it was lost, never to make a failing test pass. If the
# regenerated file does not match the committed sha256, that is a finding, not
# a reason to update the sha256.
oracle:
	@printf "This regenerates the golden oracle from ckc-rs 0.1.18. Continue? [y/N] "; \
	read answer; \
	if [ "$$answer" != "y" ] && [ "$$answer" != "Y" ]; then \
		echo "Aborted."; \
		exit 1; \
	fi
	cargo run --release --manifest-path tools/oracle-gen/Cargo.toml
	@$(MAKE) oracle-verify

# Show dependency tree
tree:
	@echo "Showing dependency tree..."
	cargo tree

# Show duplicate dependencies
tree-duplicates:
	@echo "Showing duplicate dependencies..."
	cargo tree --duplicates

# Security audit with cargo-deny (advisories only). With zero normal
# dependencies this only ever covers the dev-dependency tree — which is still
# worth knowing, since that tree is what broke the old MSRV leg.
audit:
	@echo "Running security audit..."
	cargo deny check advisories

# Check for unused dependencies (requires nightly)
unused-deps:
	@echo "Checking for unused dependencies..."
	cargo +nightly udeps --all-features

# Create documentation
create_docs:
	cargo doc --no-deps --all-features

# Open documentation in browser
docs: create_docs
	@DOC_PATH="./target/doc/ckc_rs/index.html"; \
	if command -v xdg-open >/dev/null 2>&1; then \
		xdg-open "$$DOC_PATH"; \
	elif command -v open >/dev/null 2>&1; then \
		open "$$DOC_PATH"; \
	else \
		echo "No supported opener found (tried xdg-open and open)."; \
		echo "Open $$DOC_PATH manually."; \
		exit 1; \
	fi

# All You Can Eat - Run all checks at CI strictness.
# Target-specific exports propagate to every prerequisite recipe (build,
# nextest, doc tests, clippy, the kernel gates), so warnings become hard errors
# exactly like the GitHub Actions jobs. Standalone targets (e.g. `make test`)
# stay lenient.
ayce: export RUSTFLAGS := -Dwarnings
ayce: export CARGO_INCREMENTAL := 0
ayce: fmt build_test clippy-pedantic no-std zero-deps oracle-verify create_docs

# Install required tools
install-tools:
	@echo "Installing development tools..."
	cargo install --locked cargo-nextest
	cargo install --locked cargo-semver-checks
	cargo install --locked cargo-mutants
	cargo install cargo-deny
	cargo install cargo-udeps
	cargo install cargo-llvm-cov
	rustup component add llvm-tools
	rustup target add thumbv7em-none-eabi wasm32-unknown-unknown
	@echo ""
	@echo "✓ Tools installed!"
	@echo ""

# Watch mode for development (requires cargo-watch)
watch:
	cargo watch -x check -x test

# Install cargo-watch
install-watch:
	cargo install cargo-watch

# Run mutation testing on the full codebase (slow — can take hours)
mutants:
	@if ! cargo mutants --version >/dev/null 2>&1; then \
		echo "cargo-mutants is not installed."; \
		printf "Would you like to install it now? [y/N] "; \
		read answer; \
		if [ "$$answer" = "y" ] || [ "$$answer" = "Y" ]; then \
			cargo install --locked cargo-mutants; \
		else \
			echo "Skipping. Run 'cargo install cargo-mutants' to install manually."; \
			exit 1; \
		fi; \
	fi
	cargo mutants

# Run mutation testing only on files changed vs main (faster, good before pushing)
mutants-diff:
	@if ! cargo mutants --version >/dev/null 2>&1; then \
		echo "cargo-mutants is not installed. Run 'make install-tools' first."; \
		exit 1; \
	fi
	git diff main..HEAD > /tmp/ckc-rs-diff.txt
	cargo mutants --in-diff /tmp/ckc-rs-diff.txt

# Generate HTML code coverage report using cargo-llvm-cov
coverage:
	@if ! cargo llvm-cov --version >/dev/null 2>&1; then \
		echo "cargo-llvm-cov is not installed."; \
		printf "Would you like to install it now? [y/N] "; \
		read answer; \
		if [ "$$answer" = "y" ] || [ "$$answer" = "Y" ]; then \
			cargo install cargo-llvm-cov; \
			rustup component add llvm-tools; \
		else \
			echo "Skipping. Run 'cargo install cargo-llvm-cov && rustup component add llvm-tools' to install manually."; \
			exit 1; \
		fi; \
	fi
	cargo llvm-cov --all-features --html
	@echo "Coverage report: target/llvm-cov/html/index.html"

# Generate HTML coverage report and open in browser
coverage-open: coverage
	@COV_PATH="./target/llvm-cov/html/index.html"; \
	if command -v xdg-open >/dev/null 2>&1; then \
		xdg-open "$$COV_PATH"; \
	elif command -v open >/dev/null 2>&1; then \
		open "$$COV_PATH"; \
	else \
		echo "No supported opener found (tried xdg-open and open)."; \
		echo "Open $$COV_PATH manually."; \
		exit 1; \
	fi
