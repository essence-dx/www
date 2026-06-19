use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only Maven import surface. It does not run Maven or Gradle commands.
pub fn maven_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Maven,
        &[
            "Maven Central metadata",
            "pom.xml",
            "Gradle module metadata",
            "license declaration",
        ],
        &[
            "jar source archive",
            "source file list",
            "checksum metadata",
        ],
        &[
            "Maven plugin execution",
            "Gradle build script execution",
            "JNI or native library requirement",
            "annotation processor or code generator requirement",
            "missing license or integrity declaration",
        ],
        "dx add maven/<group.artifact>",
    )
}
