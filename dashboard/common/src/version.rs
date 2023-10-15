//! Load version information from Cargo.toml at compile time.

use once_cell::sync::Lazy;
use semver::Version;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// The version of the dashboard
pub static DASHBOARD_VERSION: Lazy<Version> = Lazy::new(|| {
    semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Could not parse version")
});

/// Consistent across architectures, might not be consistent across different compiler versions
#[must_use]
pub fn hash_string(string: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}

/// Check if the current build uses the nightly compiler
#[must_use]
pub fn is_nightly() -> bool {
    DASHBOARD_VERSION.build.contains("nightly")
}

/// Check if the current build is stable
#[must_use]
pub fn is_stable() -> bool {
    DASHBOARD_VERSION.pre.is_empty() && !is_nightly()
}
