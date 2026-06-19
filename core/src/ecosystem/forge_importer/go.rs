use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only Go module import surface. It does not run go get or go generate.
pub fn go_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Go,
        &[
            "Go module proxy metadata",
            "go.mod",
            "go.sum metadata",
            "module version declaration",
            "license declaration",
        ],
        &["module zip", "module file list"],
        &[
            "go generate directive",
            "cgo requirement",
            "replace directive requiring local context",
            "private module source",
            "vendored binary asset",
            "missing license or integrity declaration",
        ],
        "dx add go/<module>",
    )
}
