# vitasdk-sys

[![Crates.io](https://img.shields.io/crates/v/vitasdk-sys.svg)](https://crates.io/crates/vitasdk-sys)
![License: MIT](https://img.shields.io/crates/l/vitasdk-sys.svg)


This crate exports bindings to functions available in [vitasdk](https://vitasdk.org/) and statically links to all of its stubs libraries. Their official docs are [here](https://docs.vitasdk.org/) and the bindings are automatically generated from vitasdk's [vita-headers](https://github.com/vitasdk/vita-headers.git) repository.

To be able to use it, you need vitasdk available and the environment variable `VITASDK` set to its location. e.g.:

```
$ export VITASDK=/opt/vitasdk
```

There's an example on how to use this crate at [examples/std-hello-world](examples/std-hello-world).

## Manually generating the bindings

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
$ cd generator/vita-headers
$ git pull
$ cd ../..
```

Generate the bindings with:

```sh
$ cd generator
$ cargo run
```

Note that `VITASDK` must me set as explained at the top of this README. It's required to generate bindings because of the platform includes.

Depending on upstream changes you may need to tweak `generator/config.toml` (documented at `generator/src/config.rs`) for the new headers to work. In case you need to, feel free to contribute by opening a pull request.

## Versioning

Even though usual `semver` rules apply for this crate, I believe that `vitasdk` has some sort of versioning and we're not following it right now. We're just keeping updated with the latest version of `vita-headers` and generating bindings for them. If you need bindings for newer versions or wish to improve the current situation, feel free to open an issue.

## Credits

- [**VitaSDK team**](http://vitasdk.org/) for the toolchain, vitasdk itself, etc.
- [rust-bindgen contributors](https://github.com/rust-lang/rust-bindgen) for allowing auto generated bindings.
- [Martin Larralde](https://github.com/althonos) for [psp2-sys](https://github.com/vita-rust/psp2-sys), which I believe originally inspired me to create this crate.

## License

This crate (library) is distributed under terms of MIT license or Apache License (Version 2.0), at your option.
See `LICENSE-MIT` and `LICENSE-APACHE` for terms.
