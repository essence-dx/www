use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only Hex.pm import surface. It does not run Mix, Rebar, or package code.
pub fn hex_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Hex,
        &[
            "hex.pm package metadata",
            "mix.exs",
            "rebar.config metadata",
            "mix.lock metadata",
            "license declaration",
        ],
        &["package tarball", "source archive", "package file list"],
        &[
            "Mix compiler or generated source requirement",
            "Rebar hook or build plugin requirement",
            "NIF or port-driver native code",
            "missing license or integrity declaration",
        ],
        "dx add hex/<package>",
    )
}
