use super::types::{DxForgeImportEcosystem, DxForgeImportPlanSurface};

/// Plan-only Python package import surface. It does not run pip or setup hooks.
pub fn pip_import_plan_surface() -> DxForgeImportPlanSurface {
    DxForgeImportPlanSurface::non_executing(
        DxForgeImportEcosystem::Pip,
        &[
            "PyPI JSON metadata",
            "wheel METADATA",
            "wheel RECORD",
            "sdist PKG-INFO",
            "pyproject.toml declaration",
        ],
        &["wheel archive", "source distribution archive"],
        &[
            "setup.py execution required",
            "native extension or binary wheel",
            "dynamic version or build backend execution",
            "console script entrypoint side effects",
            "missing license or integrity declaration",
        ],
        "dx add pip/<package>",
    )
}
