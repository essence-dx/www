use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Non-executing npm import and acquisition surface.
///
/// Live npm fetching is allowed only through the explicit Forge acquisition
/// path. It still does not run installs or lifecycle scripts.
pub fn npm_import_plan_surface() -> DxForgeImportPlanSurface {
    let mut surface = DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Npm,
        &[
            "npm registry packument",
            "package.json",
            "exports map",
            "license declaration",
            "advisory declaration",
        ],
        &[
            "package tarball",
            "tarball integrity metadata",
            "files list",
        ],
        &[
            "preinstall/install/postinstall lifecycle script",
            "native gyp or prebuild artifact",
            "dynamic require or import expression",
            "obfuscated or minified runtime blob",
            "large unreviewed dependency graph",
            "missing license or integrity declaration",
        ],
        "dx add npm/<package>",
    );
    surface.live_fetching_enabled = true;
    surface
}
