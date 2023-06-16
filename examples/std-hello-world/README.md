# Requirements

## Vitasdk

As stated on the README at the top of this repository, you need vitasdk
available and the environment variable `VITASDK` set up in order to use this
library. You can follow the instructions at https://vitasdk.org/.

## Nightly compiler and rust-src component

As the vita is a tier 3 target, we have to use cargo's build-std feature, which
is only available for nightly rust. build-std also requires the rust-src
component.

To install the nightly toolchain with rust-src run:

```
$ rustup toolchain install nightly-x86_64-unknown-linux-gnu --component rust-src
```

## cargo-make

We use `cargo-make` to strip the elf, build the velf and the vpk. To install it
follow the instructions at the
[cargo-make](https://github.com/sagiegurari/cargo-make) repository.

The instructions for cargo-make are at Makefile.toml. You'll need to customize
TITLE and TITLEID when making your own app / game.

# Building and running

To create a debug build, just run:

```
$ cargo make vpk
```

To create a release build, run:

```
$ cargo make --profile release vpk
```

The vpk will be at `target/armv7-sony-vita-newlibeabihf/debug/std-hello-world.vpk`
or `target/armv7-sony-vita-newlibeabihf/release/std-hello-world.vpk`, depending
on the profile.
