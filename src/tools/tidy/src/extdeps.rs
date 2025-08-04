//! Check for external package sources. Allow only vendorable packages.

use std::fs;
use std::path::Path;

use crate::deps::WorkspaceInfo;

/// List of allowed sources for packages.
const ALLOWED_SOURCES: &[&str] = &[
    r#""registry+https://github.com/rust-lang/crates.io-index""#,
    // This is `rust_team_data` used by `site` in src/tools/rustc-perf,
    r#""git+https://github.com/rust-lang/team#a5260e76d3aa894c64c56e6ddc8545b9a98043ec""#,
    r#""git+https://github.com/dpaoliello/stacker.git?branch=arm64ec#4e65067c0083c9ce58a96783ce79f67364e209bd""#,
    r#""git+https://github.com/nushell/nu-ansi-term#f659d23f4c6fd3466e4cc96b5653b7afe0876887""#,
    r#""git+https://github.com/dpaoliello/cc-rs.git?branch=sdktools#05ff2ab3fb834896410d384ed2b8f2e7004931a9""#,
];

/// Checks for external package sources. `root` is the path to the directory that contains the
/// workspace `Cargo.toml`.
pub fn check(root: &Path, bad: &mut bool) {
    for &WorkspaceInfo { path, submodules, .. } in crate::deps::WORKSPACES {
        if crate::deps::has_missing_submodule(root, submodules) {
            continue;
        }

        // FIXME check other workspaces too
        // `Cargo.lock` of rust.
        let lockfile = root.join(path).join("Cargo.lock");

        if !lockfile.exists() {
            tidy_error!(bad, "the `{path}` workspace doesn't have a Cargo.lock");
            continue;
        }

        // Open and read the whole file.
        let cargo_lock = t!(fs::read_to_string(&lockfile));

        // Process each line.
        for line in cargo_lock.lines() {
            // Consider only source entries.
            if !line.starts_with("source = ") {
                continue;
            }

            // Extract source value.
            let source = line.split_once('=').unwrap().1.trim();

            // Ensure source is allowed.
            if !ALLOWED_SOURCES.contains(&source) {
                tidy_error!(bad, "invalid source: {}", source);
            }
        }
    }
}
