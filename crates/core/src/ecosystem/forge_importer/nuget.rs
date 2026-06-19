use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only NuGet import surface. It does not run dotnet restore or build.
pub fn nuget_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Nuget,
        &[
            "NuGet registration metadata",
            "nuspec metadata",
            "project file metadata",
            "license declaration",
        ],
        &[
            "nupkg archive",
            "symbols/source package",
            "checksum metadata",
        ],
        &[
            "MSBuild target or props execution",
            "native runtime asset",
            "source generator or analyzer execution",
            "missing license or integrity declaration",
        ],
        "dx add nuget/<package>",
    )
}
