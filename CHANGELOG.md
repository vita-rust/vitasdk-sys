# Changelog

The format is based on [Common Changelog](https://common-changelog.org/) and [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.3.1] - Unreleased

### Fixed

- Fixed build script on docs.rs and possibly other environments by enabling include path detection (#15).

## [0.3.0] - 2023-09-29

_This release includes a rewrite of the whole binding generation process by @ZetaNumbers._

### Changed

- **Breaking** The bindings are now generated on a flat structure, so now all items are defined at the root of the crate.
- **Breaking** Items are now defined based on features, each feature corresponding to a stub file. Enabling the feature will cause the required stub to be linked.
- **Breaking** Bindings are now generated at build-time, so [bindgen's requirements](https://rust-lang.github.io/rust-bindgen/requirements.html) need to be installed.
- Improvements to CI, including new checks for docs (which uploads generated docs as an artifact).

## [0.2.0] - 2023-09-12

### Changed

- **Breaking:** Stopped generating duplicated struct definitions. A way to add missing imports was added to `generator`.
- **Breaking:** Update vita-headers to 251fb0ba8506766cf8bee4e330e88e2f934b175b (they are moving things a lot, so paths may need to be updated)
