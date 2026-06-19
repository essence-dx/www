use std::path::Path;

use crate::config::DxDevServerMode;
use crate::error::{DxError, DxResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DxDevSelectedServerMode {
    Axum,
    MayMinihttp,
}

impl DxDevSelectedServerMode {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Axum => "axum",
            Self::MayMinihttp => "may-minihttp",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxDevServerModeDecision {
    pub(super) selected: DxDevSelectedServerMode,
    pub(super) reason: &'static str,
}

pub(super) fn resolve_dev_server_mode(
    project_root: &Path,
    requested: DxDevServerMode,
    hot_reload: bool,
    devtools: bool,
) -> DxResult<DxDevServerModeDecision> {
    let axum_requirement = project_axum_requirement(project_root);

    match requested {
        DxDevServerMode::Axum => Ok(DxDevServerModeDecision {
            selected: DxDevSelectedServerMode::Axum,
            reason: "explicit axum mode",
        }),
        DxDevServerMode::MayMinihttp => {
            if hot_reload || devtools {
                return Err(dev_server_mode_error(
                    "may-minihttp dev server mode requires hot_reload=false and devtools=false",
                ));
            }
            if axum_requirement.is_some() {
                return Err(dev_server_mode_error(
                    "may-minihttp dev server mode cannot serve projects with route handlers or server sources",
                ));
            }
            Ok(DxDevServerModeDecision {
                selected: DxDevSelectedServerMode::MayMinihttp,
                reason: "explicit may-minihttp mode",
            })
        }
        DxDevServerMode::Auto => {
            if hot_reload {
                return Ok(DxDevServerModeDecision {
                    selected: DxDevSelectedServerMode::Axum,
                    reason: "hot reload uses Axum dev endpoints",
                });
            }
            if devtools {
                return Ok(DxDevServerModeDecision {
                    selected: DxDevSelectedServerMode::Axum,
                    reason: "devtools use Axum dev endpoints",
                });
            }
            if let Some(reason) = axum_requirement {
                return Ok(DxDevServerModeDecision {
                    selected: DxDevSelectedServerMode::Axum,
                    reason,
                });
            }
            Ok(DxDevServerModeDecision {
                selected: DxDevSelectedServerMode::MayMinihttp,
                reason: "static project without Axum-only dev capabilities",
            })
        }
    }
}

fn project_axum_requirement(project_root: &Path) -> Option<&'static str> {
    for relative in ["app/api", "src/app/api", "server"] {
        if project_root.join(relative).is_dir() {
            return Some("server source directory requires Axum");
        }
    }

    if contains_server_route_file(&project_root.join("app"))
        || contains_server_route_file(&project_root.join("src/app"))
    {
        return Some("route handler source requires Axum");
    }

    None
}

fn contains_server_route_file(root: &Path) -> bool {
    if !root.is_dir() {
        return false;
    }

    let Ok(entries) = std::fs::read_dir(root) else {
        return false;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if contains_server_route_file(&path) {
                return true;
            }
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if matches!(
            file_name,
            "route.ts" | "route.tsx" | "route.rs" | "actions.ts" | "actions.tsx"
        ) {
            return true;
        }
    }

    false
}

fn dev_server_mode_error(message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some("dev.server_mode".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_selects_axum_when_hot_reload_is_enabled() {
        let dir = tempfile::tempdir().expect("tempdir");
        let decision =
            resolve_dev_server_mode(dir.path(), DxDevServerMode::Auto, true, false).unwrap();

        assert_eq!(decision.selected, DxDevSelectedServerMode::Axum);
        assert_eq!(decision.reason, "hot reload uses Axum dev endpoints");
    }

    #[test]
    fn auto_selects_tiny_for_static_project_without_dev_capabilities() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("app")).expect("app dir");
        std::fs::write(
            dir.path().join("app/page.tsx"),
            "export default function Page() {}",
        )
        .expect("page");

        let decision =
            resolve_dev_server_mode(dir.path(), DxDevServerMode::Auto, false, false).unwrap();

        assert_eq!(decision.selected, DxDevSelectedServerMode::MayMinihttp);
    }

    #[test]
    fn auto_selects_axum_for_route_handlers() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("app/api/health")).expect("api dir");
        std::fs::write(
            dir.path().join("app/api/health/route.ts"),
            "export function GET() { return Response.json({ ok: true }); }",
        )
        .expect("route");

        let decision =
            resolve_dev_server_mode(dir.path(), DxDevServerMode::Auto, false, false).unwrap();

        assert_eq!(decision.selected, DxDevSelectedServerMode::Axum);
        assert_eq!(decision.reason, "server source directory requires Axum");
    }
}
