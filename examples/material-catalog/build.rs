//! Generates the catalog's component ids from `assets/catalog.octoscript`.
//!
//! The DSL emits `NAV_BASE + row index` as a tap id, so the host has to map an
//! index back to a screen name -- which means the same strings exist on both
//! sides. Generating removes the second copy instead of testing for drift.

use std::env;

fn main() {
    generate_catalog_screens();
}

/// Write the catalog's component ids out of `catalog.octoscript` and into a Rust
/// constant.
///
/// The DSL emits `NAV_BASE + row index` as a tap id, so the host has to map an
/// index back to a screen name — which means the same 28 strings exist on both
/// sides. Maintaining that by hand is a list that drifts the first time anyone
/// reorders the index, and the symptom would be tapping one row and opening
/// another. Generating it removes the second copy instead of testing for it,
/// which also suits a crate whose tests cannot link on the host.
fn generate_catalog_screens() {
    println!("cargo:rerun-if-changed=assets/catalog.octoscript");
    let src = std::fs::read_to_string("assets/catalog.octoscript").expect("assets/catalog.octoscript");
    let start = src.find("let COMPONENTS = [").expect("COMPONENTS list");
    let rest = &src[start..];
    let end = rest.find("\n]").expect("end of COMPONENTS");
    let ids: Vec<String> = rest[..end]
        .lines()
        .filter_map(|l| l.trim().strip_prefix("[\""))
        .filter_map(|l| l.split('"').next())
        .map(|s| format!("    \"{s}\","))
        .collect();
    assert!(
        !ids.is_empty(),
        "no component ids parsed from catalog.octoscript"
    );

    let out = std::path::PathBuf::from(env::var("OUT_DIR").unwrap()).join("catalog_screens.rs");
    std::fs::write(
        &out,
        format!(
            "/// Component ids, generated from assets/catalog.octoscript at build time.\n\
             pub const CATALOG_SCREENS: [&str; {}] = [\n{}\n];\n",
            ids.len(),
            ids.join("\n")
        ),
    )
    .expect("write catalog_screens.rs");
}
