use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only SwiftPM import surface. It does not run swift package or build.
pub fn swift_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Swift,
        &[
            "Swift Package Index metadata",
            "Package.swift",
            "Package.resolved metadata",
            "license declaration",
        ],
        &["source archive", "source file list", "checksum metadata"],
        &[
            "Swift package plugin execution",
            "system library or binary target",
            "macro plugin or generated source requirement",
            "missing license or integrity declaration",
        ],
        "dx add swift/<package>",
    )
}
