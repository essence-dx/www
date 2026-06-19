use std::collections::BTreeSet;
use std::sync::LazyLock;

use regex::Regex;

use super::super::directives::collect_top_level_directives;
use super::inline_server_actions::collect_inline_server_actions;

static EXPORT_FUNCTION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)\bexport\s+(async\s+)?function\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*\("#)
        .expect("export function regex")
});
static EXPORT_CONST_CALLABLE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?m)\bexport\s+const\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(async\s+)?(?:\([^=;]*\)|[A-Za-z_$][A-Za-z0-9_$]*)\s*=>"#,
    )
    .expect("export const callable regex")
});

/// balanced-server-actions scanner for Lane 11 Next custom-transform receipts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ServerActionDetection {
    pub(super) export_name: String,
    pub(super) async_export: bool,
    pub(super) directive_location: &'static str,
    pub(super) export_kind: &'static str,
    pub(super) action_id_strategy: &'static str,
    pub(super) next_proxy_generated: bool,
    pub(super) source_owned_detection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleDirectiveExport {
    name: String,
    async_export: bool,
    export_kind: &'static str,
}

pub(super) fn collect_server_action_detections(source: &str) -> Vec<ServerActionDetection> {
    let mut actions = Vec::new();
    let mut seen = BTreeSet::new();
    let directives = collect_top_level_directives(source);

    if directives.contains("use server") {
        for export in collect_module_directive_exports(source) {
            push_server_action(
                &mut actions,
                &mut seen,
                &export.name,
                export.async_export,
                "module-directive-export",
                export.export_kind,
            );
        }
    }

    for inline in collect_inline_server_actions(source) {
        push_server_action(
            &mut actions,
            &mut seen,
            &inline.name,
            inline.async_export,
            "inline-function-directive",
            inline.export_kind,
        );
    }

    actions.sort_by(|left, right| {
        left.export_name
            .cmp(&right.export_name)
            .then(left.directive_location.cmp(right.directive_location))
    });
    actions
}

fn collect_module_directive_exports(source: &str) -> Vec<ModuleDirectiveExport> {
    let mut exports = Vec::new();
    for capture in EXPORT_FUNCTION_RE.captures_iter(source) {
        let Some(name) = capture.get(2).map(|value| value.as_str().to_string()) else {
            continue;
        };
        exports.push(ModuleDirectiveExport {
            name,
            async_export: capture.get(1).is_some(),
            export_kind: "function",
        });
    }
    for capture in EXPORT_CONST_CALLABLE_RE.captures_iter(source) {
        let Some(name) = capture.get(1).map(|value| value.as_str().to_string()) else {
            continue;
        };
        exports.push(ModuleDirectiveExport {
            name,
            async_export: capture.get(2).is_some(),
            export_kind: "const-arrow",
        });
    }
    exports.sort_by(|left, right| left.name.cmp(&right.name));
    exports.dedup_by(|left, right| left.name == right.name);
    exports
}

fn push_server_action(
    actions: &mut Vec<ServerActionDetection>,
    seen: &mut BTreeSet<String>,
    name: &str,
    async_export: bool,
    directive_location: &'static str,
    export_kind: &'static str,
) {
    let key = format!("{name}#{directive_location}");
    if !seen.insert(key) {
        return;
    }
    actions.push(ServerActionDetection {
        export_name: name.to_string(),
        async_export,
        directive_location,
        export_kind,
        action_id_strategy: "dx-source-receipt-only",
        next_proxy_generated: false,
        source_owned_detection: true,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_module_directive_exports_without_proxy_generation() {
        let actions = collect_server_action_detections(
            r#""use server";
export async function save() {
  return { ok: true };
}
export const remove = async (id) => {
  return { ok: Boolean(id) };
};
"#,
        );

        assert_eq!(actions.len(), 2);
        assert!(actions.iter().any(|action| {
            action.export_name == "save"
                && action.directive_location == "module-directive-export"
                && action.async_export
                && !action.next_proxy_generated
        }));
        assert!(actions.iter().any(|action| {
            action.export_name == "remove" && action.export_kind == "const-arrow"
        }));
    }

    #[test]
    fn keeps_inline_actions_as_source_owned_receipts() {
        let actions = collect_server_action_detections(
            r#"export default function Page() {
  async function save() {
    "use server";
    return { ok: true };
  }
  return null;
}
"#,
        );

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].export_name, "save");
        assert_eq!(actions[0].directive_location, "inline-function-directive");
        assert_eq!(actions[0].action_id_strategy, "dx-source-receipt-only");
        assert!(actions[0].source_owned_detection);
    }
}
