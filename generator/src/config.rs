use serde::Deserialize;

/// The Config structure is parsed from `config.toml` and is used to customize
/// the bindings generated from `vita-headers`. Allowlists are used to define
/// which types, functions and variables are going to be translated or ignored.
/// They use the `regex` format as defined in the [regex](docs.rs/regex) crate
/// and are implemented by calling [allowlist_*](https://docs.rs/bindgen/*/bindgen/struct.Builder.html#method.allowlist_type)
/// functions in bindgen.
#[derive(Deserialize)]
pub struct Config {
    /// The lists array contain file-specific allowlists to be used. Multiple
    /// files can use the same lists, but specifying the same file muliple times
    /// is invalid.
    pub lists: Vec<Lists>,
}

/// Allowlists for a file group of files.
#[derive(Deserialize)]
pub struct Lists {
    /// File paths for which these allowlists apply. Relative to the
    /// `vita-headers/include` directory.
    pub files: Vec<String>,
    /// Allowlists specific for these files. Note that the default allowlists
    /// will also be included unless `exclude_default` is set.
    #[serde(default)]
    pub allowlists: Vec<String>,
    /// Type allowlists specific for these files. Note that the default
    /// allowlists will also be included unless `exclude_default` is set.
    #[serde(default)]
    pub allowlists_type: Vec<String>,
    /// Function allowlists specific for these files. Note that the default
    /// allowlists will also be included unless `exclude_default` is set.
    #[serde(default)]
    pub allowlists_function: Vec<String>,
    /// Variable allowlists specific for these files. Note that the default
    /// allowlists will also be included unless `exclude_default` is set.
    #[serde(default)]
    pub allowlists_var: Vec<String>,
    /// Whether or not to exlude (not use) the default allowlist_file for these
    /// files.
    #[serde(default)]
    pub exclude_default: bool,
    /// Whether or not to skip these files.
    #[serde(default)]
    pub skip: bool,
}
