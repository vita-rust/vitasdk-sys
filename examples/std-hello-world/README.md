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

## cargo-vita

We use [cargo-vita](https://github.com/vita-rust/cargo-vita) to build the vpk. To install cargo vita run
```sh
cargo install cargo-vita
```

You'll need to customize TITLE and TITLEID when making your own app / game.

For more details on tools requirements and usage see the tools [README](https://github.com/vita-rust/cargo-vita) page.

# Building and running

To create a debug build, just run:

```
cargo vita build vpk
```

To create a release build, run:

```
cargo vita build vpk --release
```

You can also upload the built vpk to ux0:/download. To do that you must run an ftp server on your VITA
either by using [VitaShell](https://github.com/TheOfficialFloW/VitaShell) or [vitacompanion](https://github.com/devnoname120/vitacompanion).

When a server is running use `--upload` flag:

```
cargo vita build vpk --upload --release
```

Or update the `eboot.bin` of an already installed vpk and run it. Running requires [vitacompanion](https://github.com/devnoname120/vitacompanion) installed.

```
cargo vita build eboot --update --run --release
```