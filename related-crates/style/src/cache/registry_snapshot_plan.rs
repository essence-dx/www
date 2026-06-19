//! Registry snapshot cache contract for dx-style.
//!
//! This module intentionally defines the launch-safe shape before wiring a new
//! binary reader. The public output path remains normal generated CSS.

/// Increment this when the registry snapshot byte layout changes.
pub const REGISTRY_SNAPSHOT_FORMAT_VERSION: u32 = 1;

/// Default app-local path for the future class registry and theme token snapshot.
pub const REGISTRY_SNAPSHOT_PATH: &str = ".dx/style/registry.snapshot";

/// Evidence-backed plan for a small cached registry snapshot lane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrySnapshotPlan {
    /// Stable format version for compatibility checks.
    pub format_version: u32,
    /// App-local snapshot path.
    pub path: &'static str,
    /// Human-readable storage strategy.
    pub storage: &'static str,
    /// Data that the snapshot is allowed to contain.
    pub includes: &'static [&'static str],
    /// Guardrails that keep the launch story honest.
    pub guardrails: &'static [&'static str],
}

/// Return the current plan for a memmap2-backed registry snapshot.
pub fn registry_snapshot_plan() -> RegistrySnapshotPlan {
    RegistrySnapshotPlan {
        format_version: REGISTRY_SNAPSHOT_FORMAT_VERSION,
        path: REGISTRY_SNAPSHOT_PATH,
        storage: "memmap2-backed read-only memory map after an atomic snapshot write",
        includes: &[
            "class registry utility keys",
            "theme token names and resolved values",
            "screen and container query token names",
            "cache checksum and generated CSS input hash",
        ],
        guardrails: &[
            "normal generated CSS remains the default public output",
            "rkyv is not wired into dx-style",
            "do not make binary style output part of the public launch pitch",
            "compile and CLI smoke must run before this becomes active I/O",
        ],
    }
}
