use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only Dart/pub import surface. It does not run dart or Flutter commands.
pub fn pub_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Pub,
        &[
            "pub.dev package metadata",
            "pubspec.yaml",
            "pubspec.lock metadata",
            "license declaration",
        ],
        &["package archive", "package file list"],
        &[
            "build_runner or generated source requirement",
            "Flutter plugin native platform channel",
            "FFI or native asset requirement",
            "missing license or integrity declaration",
        ],
        "dx add pub/<package>",
    )
}
