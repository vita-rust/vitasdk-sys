If you're interested in PS Vita development in Rust, please check out the [organization-wide docs](https://github.com/vita-rust) and the [Rust on Sony PlayStation Vita Book](https://vita-rust.github.io/book/).
There are also working examples at [examples](https://github.com/vita-rust/examples).

# vitasdk-sys

[![docs.rs](https://docs.rs/vitasdk-sys/badge.svg)](https://docs.rs/vitasdk-sys/)
[![Crates.io](https://img.shields.io/crates/v/vitasdk-sys.svg)](https://crates.io/crates/vitasdk-sys)
![License: MIT](https://img.shields.io/crates/l/vitasdk-sys.svg)


This crate exports bindings to functions available in [vitasdk](https://vitasdk.org/) and statically links to its stubs libraries based on enabled features. Their official docs are [here](https://docs.vitasdk.org/) and the bindings are automatically generated from vitasdk's [vita-headers](https://github.com/vitasdk/vita-headers) repository. Which features required for which functions can be found on https://docs.rs/vitasdk-sys.

There's an example on how to use this crate at [examples/std-hello-world](examples/std-hello-world).

To be able to use it, you need vitasdk available and the environment variable `VITASDK` set to its location. e.g.:

```
$ export VITASDK=/opt/vitasdk
```

## Features

### Stubs (`*_stub`)

To be able to use any function available in this crate, you'll need to enable
the related features, which will cause the required stub to be linked. You can
find which any function a stub requires by looking at [docs.rs/vitasdk-sys](https://docs.rs/vitasdk-sys).

Example: to find which stub is required to use `sceDisplaySetFrameBuf`, search
for it on docs.rs. You'll find `Available on crate feature SceDisplay_stub only.`
there, which means that you'll need to enable `SceDisplay_stub` to use it.

### `vitasdk-utils`

This feature just enables some functions provided by vitasdk. Just like the
stubs, you can figure which functions require it by looking at docs.rs.

### `bindgen`

The `bindgen` feature makes the crate generate bindings at compile-time
using headers from the system's vitasdk (on `$VITASDK`), instead of using the
pre-generated bindings.

To use this feature you also need [bindgen's requirements](https://rust-lang.github.io/rust-bindgen/requirements.html),
which may already be on your system.

## Manually updating the submodule

## Updating

To update the headers, we have a job that runs on GitHub Actions yearly or on demand that will create a PR. If you want to update manually, you can follow these steps:

### Manually

Clone the repository with submodules (the C headers):

```sh
$ git clone --recurse-submodules https://github.com/pheki/vitasdk-sys.git
```

If the repository is already cloned, update the submodules with:

```sh
$ git submodule update --init --recursive
```

To update the headers, go to the vita-headers submodule and update by the usual means:

```
$ cd vita-headers
$ git pull
$ cd ..
```

Run `cargo run -p vitasdk-sys-build-util -- stub-libs --as-features --all-stubs-feature` and replace stub lib features in vitasdk-sys Cargo.toml with outputed ones.

Then run `cargo run -p vitasdk-sys-build-util -- bindgen`

## Versioning

Usual `semver` rules apply for this crate, but note that there may be differences between the version of the headers used to generate the bindings and the vitasdk version installed on your machine.

## Updating

To update versions you can look for breaking changes at [CHANGELOG.md](CHANGELOG.md).
If you're coming from a version before 0.3, [this comment](https://github.com/vita-rust/vitasdk-sys/issues/20#issuecomment-1782335568) has a migration guide.

## Credits

- [**VitaSDK team**](http://vitasdk.org/) for the toolchain, vitasdk itself, etc.
- [rust-bindgen contributors](https://github.com/rust-lang/rust-bindgen) for making auto generated bindings viable.
- [Martin Larralde](https://github.com/althonos) for [psp2-sys](https://github.com/vita-rust/psp2-sys), which I believe originally inspired me to create this crate.

## License

This crate (library) is distributed under terms of MIT license or Apache License (Version 2.0), at your option.
See `LICENSE-MIT` and `LICENSE-APACHE` for terms.
