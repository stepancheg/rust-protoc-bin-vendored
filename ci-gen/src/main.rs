use std::fs;
use std::path::Path;

use gh_actions_gen::actions::cargo_cache;
use gh_actions_gen::actions::cargo_doc;
use gh_actions_gen::actions::cargo_test;
use gh_actions_gen::actions::checkout_sources;
use gh_actions_gen::actions::rust_install_toolchain;
use gh_actions_gen::actions::RustToolchain;
use gh_actions_gen::ghwf::Env;
use gh_actions_gen::ghwf::Job;
use gh_actions_gen::ghwf::Step;
use gh_actions_gen::rustfmt::rustfmt_check_job;
use gh_actions_gen::super_mega_linter::mega_linter_job;

fn crates_list() -> Vec<String> {
    assert!(Path::new("./ci-gen").exists());
    let mut r = Vec::new();
    for p in fs::read_dir(".").unwrap() {
        let p = p.unwrap();
        if Path::new(&format!("{}/Cargo.toml", p.path().display())).exists() {
            r.push(p.path().file_name().unwrap().to_str().unwrap().to_owned());
        }
    }
    r.sort();
    assert!(r.len() > 0);
    r
}

fn steps(_os: Os, channel: RustToolchain) -> Vec<Step> {
    let mut r = vec![
        cargo_cache(),
        checkout_sources(),
        rust_install_toolchain(channel),
    ];
    for c in crates_list() {
        // Not really a crate.
        if c == "protoc-bin-vendored-arch-template" {
            continue;
        }
        let args = format!("--manifest-path={}/Cargo.toml", c);
        let mut step = cargo_test(&format!("cargo test {}", c), &args);
        step.timeout_minutes = Some(5);
        r.push(step);
    }
    r
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Os {
    name: &'static str,
    ghwf: Env,
}

const LINUX: Os = Os {
    name: "linux",
    ghwf: Env::UbuntuLatest,
};
const MACOS: Os = Os {
    name: "macos",
    ghwf: Env::MacosLatest,
};
const WINDOWS: Os = Os {
    name: "windows",
    ghwf: Env::WindowsLatest,
};

fn cargo_doc_job() -> Job {
    let os = LINUX;
    let mut steps = Vec::new();
    steps.push(cargo_cache());
    steps.push(checkout_sources());
    steps.push(rust_install_toolchain(RustToolchain::Stable));
    steps.push(cargo_doc("cargo doc", ""));
    Job {
        id: "cargo-doc".to_owned(),
        name: "cargo doc".to_owned(),
        runs_on: os.ghwf,
        steps,
        ..Default::default()
    }
}

fn jobs() -> Vec<Job> {
    let mut r = Vec::new();
    for &channel in &[
        RustToolchain::Stable,
        RustToolchain::Beta,
        RustToolchain::Nightly,
    ] {
        for &os in &[LINUX, MACOS, WINDOWS] {
            if channel == RustToolchain::Beta && (os == MACOS || os == WINDOWS) {
                // skip some jobs because macos is expensive
                continue;
            }
            r.push(Job {
                id: format!("{}-{}", os.name, channel),
                name: format!("{} {}", os.name, channel),
                runs_on: os.ghwf,
                env: vec![("RUST_BACKTRACE".to_owned(), "1".to_owned())],
                steps: steps(os, channel),
                ..Default::default()
            });
        }
    }

    r.push(cargo_doc_job());

    r.push(rustfmt_check_job());
    r.push(mega_linter_job());

    r
}

fn main() {
    gh_actions_gen::write(jobs());
}
