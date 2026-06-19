use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only Composer import surface. It does not run composer or PHP scripts.
pub fn composer_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Composer,
        &[
            "Packagist metadata",
            "composer.json",
            "composer.lock metadata",
            "license declaration",
        ],
        &["package archive", "dist/source file list"],
        &[
            "Composer script execution",
            "PHP extension or native module requirement",
            "autoload side-effect boundary",
            "missing license or integrity declaration",
        ],
        "dx add composer/<vendor/package>",
    )
}
