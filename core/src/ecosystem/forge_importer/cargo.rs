use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only Rust crate import surface. It does not run cargo commands.
pub fn cargo_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Cargo,
        &[
            "crates.io metadata",
            "Cargo.toml",
            "Cargo.lock metadata",
            "crate checksum declaration",
            "license declaration",
        ],
        &["crate archive", "crate file manifest"],
        &[
            "build.rs script",
            "proc-macro crate",
            "native links or bindgen requirement",
            "git dependency",
            "unreviewed unsafe or FFI-heavy surface",
            "missing license or integrity declaration",
        ],
        "dx add cargo/<crate>",
    )
}
