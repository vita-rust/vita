# `vita`

*Rustacean façade to PS Vita internal functions*

[![TravisCI](https://img.shields.io/travis/vita-rust/vita/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/vita-rust/vita/builds)
[![Codecov](https://img.shields.io/codecov/c/github/vita-rust/vita.svg?maxAge=600&style=flat-square)](https://codecov.io/github/vita-rust/vita)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=86400&style=flat-square)](https://github.com/vita-rust/vita)
[![CargoMake](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=86400&style=flat-square)](https://sagiegurari.github.io/cargo-make)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=86400&style=flat-square)](http://keepachangelog.com/)
[![Crate](https://img.shields.io/crates/v/vita.svg?maxAge=86400&style=flat-square)](https://crates.io/crates/vita)
[![Documentation](https://img.shields.io/badge/docs-latest-4d76ae.svg?maxAge=86400&style=flat-square)](https://docs.rs/vita)


## Introduction

The PS Vita exposes a quite comprehensive library of kernel functions
that provide much of the features needed for a Rust program to work.
As such, this crate intends to expose them to developers in an
interface mimicking the `std` crate, allowing `std` code to be compiled
for the PS Vita.

## Usage

Add this crate to `Cargo.toml`:
```toml
[dependencies]
vita = '^0.1'
```

## Cross-compiling

You'll need to have the `armv7-vita-eabihf` target specification in your
`$RUST_TARGET_PATH`. If you don't have it, you can find it in its
dedicated [git repository](https://github.com/vita-rust/common). Then,
you can use [`xargo`](https://github.com/japaric/xargo) to cross-compile.

You'll also need to have the [`vitasdk`](https://vitasdk.org) set up and
the `$VITASDK` environment variable set. See the [`psp2-sys`](https://github.com/vita-rust/psp2-sys)
repository for more details.


## Credits

* [**VitaSDK team**](http://vitasdk.org/) for the `arm-vita-eabi` toolchain, `psp2` headers, ...
* [**Team Molecule**](http://henkaku.xyz/) for the `Henkaku` hard work.
* [**@japaric**](https://github.com/japaric) for `xargo` as well as his
  various guides about cross-compilation in Rust.


## Disclaimer

*`vita` is not affiliated, sponsored, or otherwise endorsed by Sony
Interactive Entertainment, LLC. PlayStation and PS Vita are trademarks or
registered trademarks of Sony Interactive Entertainment, LLC. This software
is provided "as is" without warranty of any kind under the MIT License.
