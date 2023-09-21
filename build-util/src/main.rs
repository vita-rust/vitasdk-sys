use std::{
    collections::{BTreeSet, HashSet},
    process::ExitCode,
};

use build_util::vita_headers_db::{missing_features_filter, missing_libs_filter, VitaDb};

const VITA_HEADERS_DB_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../vita-headers/db");

fn print_help() {
    eprintln!(
        "\
Build utilities for vitasdk-sys crate

USAGE:
    build-util [OPTIONS] <COMMAND>

Commands:
    stub-libs   Print all stub lib names
    help        Print help

Options
    -h, --help  Print help
"
    )
}

fn main() -> ExitCode {
    env_logger::init();

    let cmd = std::env::args().nth(1);
    match cmd.as_deref() {
        Some("stub-libs") => stub_libs(),
        Some("help" | "--help" | "-h") => {
            print_help();
            ExitCode::SUCCESS
        }
        _ => {
            print_help();
            ExitCode::FAILURE
        }
    }
}

fn stub_libs() -> ExitCode {
    fn print_help() {
        eprintln!(
            "\
Print stub lib names

USAGE:
    build-util stub-libs [OPTIONS]

Options:
    -h, --help          Print help
    -u, --user          Print only user stub libs. Mutually exclusive with `--kernel`.
    -k, --kernel        Print only kernel stub libs. Mutually exclusive with `--user`.
    -c, --conflicting   Print only stub libs with conflicting symbols
    --with-conflicting  Include stub libs with conflicting symbols
    --missing-features  Print only undefined vitasdk-sys features
    --as-features       Print stub libs as cargo features
    --missing-libs      Print only stub libs which do not exist in `$VITASDK/arm-vita-eabi/lib`
"
        )
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    enum Flag {
        Help,
        User,
        Kernel,
        Conflicting,
        WithConflicting,
        MissingFeatures,
        AsFeatures,
        MissingLibs,
    }

    #[derive(Debug)]
    struct ParseFlagError;

    impl std::str::FromStr for Flag {
        type Err = ParseFlagError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "-h" | "--help" => Ok(Flag::Help),
                "-u" | "--user" => Ok(Flag::User),
                "-k" | "--kernel" => Ok(Flag::Kernel),
                "-c" | "--conflicting" => Ok(Flag::Conflicting),
                "--with-conflicting" => Ok(Flag::WithConflicting),
                "--missing-features" => Ok(Flag::MissingFeatures),
                "--as-features" => Ok(Flag::AsFeatures),
                "--missing-libs" => Ok(Flag::MissingLibs),
                _ => Err(ParseFlagError),
            }
        }
    }

    let Some(options) = std::env::args()
        .skip(2)
        .map(|op| op.parse())
        .collect::<Result<HashSet<Flag>, ParseFlagError>>()
        .ok()
        .filter(|ops| !ops.contains(&Flag::Kernel) || !ops.contains(&Flag::User))
    else {
        print_help();
        return ExitCode::FAILURE;
    };

    if options.contains(&Flag::Help) {
        print_help();
        return ExitCode::SUCCESS;
    }

    let mut db = VitaDb::load(VITA_HEADERS_DB_PATH.as_ref());
    if options.contains(&Flag::Conflicting) {
        db = db.split_conflicting();
    } else if !options.contains(&Flag::WithConflicting) {
        db.split_conflicting();
    }

    if options.contains(&Flag::User) {
        db.split_kernel();
    } else if options.contains(&Flag::Kernel) {
        db = db.split_kernel();
    }

    let stub_libs: BTreeSet<_> = {
        let mut missing_features_filter = options
            .contains(&Flag::MissingFeatures)
            .then(missing_features_filter);
        let mut missing_libs_filter = options
            .contains(&Flag::MissingLibs)
            .then(missing_libs_filter);

        db.stub_lib_names()
            .filter(|s| {
                missing_features_filter
                    .as_mut()
                    .map(|f| f(s))
                    .unwrap_or(true)
            })
            .filter(|s| missing_libs_filter.as_mut().map(|f| f(s)).unwrap_or(true))
            .collect()
    };

    if options.contains(&Flag::AsFeatures) {
        stub_libs
            .iter()
            .for_each(|stub_lib| println!("{stub_lib} = []"));
    } else {
        stub_libs.iter().for_each(|stub_lib| println!("{stub_lib}"));
    }

    ExitCode::SUCCESS
}
