use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only RubyGems import surface. It does not run gem, bundle, or rake.
pub fn gem_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Gem,
        &[
            "RubyGems metadata",
            "gemspec",
            "Gemfile.lock metadata",
            "license declaration",
        ],
        &["gem archive", "source file list", "checksum metadata"],
        &[
            "native extension build",
            "Rake task or install hook",
            "dynamic require boundary",
            "missing license or integrity declaration",
        ],
        "dx add gem/<package>",
    )
}
