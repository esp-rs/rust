# The Rust Programming Language for Espressif chips

This fork enables projects to be built for the Xtensa-based ESP32, ESP32-SXX and ESP8266 using [Espressif's llvm fork](https://github.com/espressif/llvm-project). (RiscV chips like ESP32-CXX are already supported in stock Rust.)

Moreover, this fork enables Rust STD support (networking, threads, and filesystem) for all chips in the ESP32 family (Xtensa and RiscV), by optionally linking with the ESP-IDF framework.

The [esp-rs](https://github.com/esp-rs) organization has been formed to develop runtime, pac and hal crates for the Espressif chips (bare-metal as well as ESP-IDF based).

Join in on the discussion: https://matrix.to/#/#esp-rs:matrix.org!

## Installation

Please see the most up to date instructions in the [esp rust book](https://esp-rs.github.io/book/).

## Building from source

If you wish to build this fork from source, the instructions are almost identical to the ones upstream ([follow here](https://github.com/rust-lang/rust#installing-from-source)), however before beginning the build, run the following `./configure` command:

```
./configure --experimental-targets=Xtensa --release-channel=nightly --enable-extended --tools=clippy,cargo,rustfmt --enable-lld
```

## License

Rust is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
