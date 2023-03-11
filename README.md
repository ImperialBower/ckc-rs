# ckc-rs

![Build Status](https://github.com/ImperialBower/ckc-rs/actions/workflows/basic.yaml/badge.svg)
[![Crates.io Version](https://img.shields.io/crates/v/ckc-rs.svg)](https://crates.io/crates/ckc-rs)


[![License: Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE)
[![Build and Test](https://github.com/ImperialBower/ckc-rs/actions/workflows/CI.yaml/badge.svg)](https://github.com/ImperialBower/ckc-rs/actions/workflows/CI.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/ckc-rs.svg)](https://crates.io/crates/ckc-rs)
[![Rustdocs](https://docs.rs/ckc-rs/badge.svg)](https://docs.rs/ckc-rs/)

Implementation of a modified [Cactus Kev's Poker Hand Evaluator](https://suffe.cool/poker/evaluator.html)
library in Rust. Code inspired and lookup tables borrowed from Vladislav Supalov's 
[pokereval-rs](https://github.com/vsupalov/pokereval-rs)
library, which in turn was inspired by [Cactus Kev's](https://suffe.cool)
[work in C](https://suffe.cool/poker/code/).

The primary entity in the library is a `PokerCard`. `PokerCard` is a u32 
variant of Cactus Kev's binary representation of a poker card. The variation 
being that the `Suit` bits order is inverted for easier sorting by other
libraries:

```txt
+--------+--------+--------+--------+
|xxxbbbbb|bbbbbbbb|SHDCrrrr|xxpppppp|
+--------+--------+--------+--------+

p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
SHDC = suit of card (bit turned on based on suit of card)
b = bit turned on depending on rank of card
```

This library contains only the bare bones implementation of the library,
and is [no-std](https://docs.rust-embedded.org/book/intro/no-std.html) to
maintain maximum utility with [embedded](https://docs.rust-embedded.org/)
and [wasm](https://rustwasm.github.io/docs/book/) Rust.

## Possible Dependencies (if needed)

* [libm](https://github.com/rust-lang/libm)
* [Serde](https://serde.rs/)
  * [no-std](https://serde.rs/no-std.html)

## Resources

* [Embedded Rust documentation](https://docs.rust-embedded.org/)
  * [The Embedded Rust Book](https://docs.rust-embedded.org/book/index.html)
  * [Discovery](https://docs.rust-embedded.org/discovery/microbit/) using micro:bit
  * [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/preface.html)
* [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)
* [Rust 🦀 and WebAssembly 🕸](https://rustwasm.github.io/docs/book/)
* [Writing an OS in Rust ](https://os.phil-opp.com/)
  * Concurrency
    * Ferrous Systems
      * [no_std async/await - soon on stable](https://ferrous-systems.com/blog/stable-async-on-embedded/)
      * [Concurrency Patterns in Embedded Rust](https://ferrous-systems.com/blog/embedded-concurrency-patterns/)
* Heads Up
  * [Cepheus Poker Project](http://poker.srv.ualberta.ca/)
  * [HeadsUp Push/Fold Nash Equilibrium](https://www.holdemresources.net/hune)
  * [Limit Hold’em Heads-Up Tournament Strategy](https://www.headsuphyper.com/limit-holdem-heads-up-strategy/)
  * [Nash's ICM calculation](https://poker.stackexchange.com/questions/1022/nashs-icm-calculation)