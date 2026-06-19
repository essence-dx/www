use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only JSR import surface for Deno-first JS/TS packages.
pub fn jsr_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Jsr,
        &[
            "jsr.io package metadata",
            "jsr.json or deno.json metadata",
            "exports metadata",
            "license declaration",
            "advisory declaration",
        ],
        &[
            "source archive",
            "module source files",
            "source integrity metadata",
        ],
        &[
            "Deno permission or unstable API boundary",
            "deno task or generator requirement",
            "dynamic import expression",
            "npm compatibility bridge requirement",
            "missing license or integrity declaration",
        ],
        "dx add jsr/<scope>/<package>",
    )
}
