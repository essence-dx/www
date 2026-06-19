use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only CRAN import surface. It does not run R package install hooks.
pub fn cran_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Cran,
        &[
            "CRAN package metadata",
            "DESCRIPTION",
            "NAMESPACE",
            "license declaration",
        ],
        &[
            "source package tarball",
            "package file list",
            "checksum metadata",
        ],
        &[
            "configure or cleanup script execution",
            "compiled native code in src/",
            "R package installation hook",
            "missing license or integrity declaration",
        ],
        "dx add cran/<package>",
    )
}
