use super::module_runtime_analyzer::{
    exported_function_names, has_runtime_imports, looks_like_jsx,
};
use super::module_runtime_typescript::strip_typescript_runtime;
use super::module_tsx_component_runtime::transform_tsx_component_runtime;
use super::module_tsx_runtime::transform_tsx_leaf_runtime;

#[derive(Debug, Clone)]
pub struct ModuleRuntimeTransform {
    pub transform_kind: String,
    pub transformed_source: Option<String>,
    pub export_names: Vec<String>,
}

pub fn transform_runtime_source(kind: &str, source: &str) -> ModuleRuntimeTransform {
    if matches!(kind, "tsx" | "jsx") {
        if has_runtime_imports(source) {
            return transform_tsx_component_runtime(source)
                .map(|transform| ModuleRuntimeTransform {
                    transform_kind: "tsx-component-runtime".to_string(),
                    transformed_source: Some(transform.transformed_source),
                    export_names: transform.export_names,
                })
                .unwrap_or_else(metadata_only);
        }

        return transform_tsx_leaf_runtime(source)
            .map(|transform| ModuleRuntimeTransform {
                transform_kind: "tsx-leaf-runtime".to_string(),
                transformed_source: Some(transform.transformed_source),
                export_names: transform.export_names,
            })
            .unwrap_or_else(metadata_only);
    }

    if has_runtime_imports(source) {
        return metadata_only();
    }

    if !matches!(kind, "ts" | "js") || looks_like_jsx(source) {
        return metadata_only();
    }

    let export_names = exported_function_names(source);
    if export_names.is_empty() {
        return metadata_only();
    }

    let transform_kind = match kind {
        "ts" => "typescript-helper-runtime",
        _ => "javascript-helper-runtime",
    };

    ModuleRuntimeTransform {
        transform_kind: transform_kind.to_string(),
        transformed_source: Some(ensure_trailing_newline(
            strip_typescript_runtime(source).trim_end(),
        )),
        export_names,
    }
}

fn metadata_only() -> ModuleRuntimeTransform {
    ModuleRuntimeTransform {
        transform_kind: "metadata-only".to_string(),
        transformed_source: None,
        export_names: Vec::new(),
    }
}

fn ensure_trailing_newline(source: &str) -> String {
    let mut source = source.to_string();
    source.push('\n');
    source
}

#[cfg(test)]
mod tests {
    use super::transform_runtime_source;

    #[test]
    fn transforms_standalone_typescript_function_helpers() {
        let transform = transform_runtime_source(
            "ts",
            "export function formatLabel(value: string): string {\n  return value.toUpperCase();\n}\n",
        );

        assert_eq!(transform.transform_kind, "typescript-helper-runtime");
        assert_eq!(transform.export_names, vec!["formatLabel".to_string()]);
        let transformed = transform.transformed_source.expect("transformed source");
        assert!(transformed.contains("export function formatLabel(value)"));
        assert!(!transformed.contains("value: string"));
        assert!(!transformed.contains("): string"));
    }

    #[test]
    fn leaves_imported_modules_as_metadata_until_import_rewriting_exists() {
        let transform = transform_runtime_source(
            "ts",
            "import { value } from './value';\nexport function formatLabel(input: string) { return value(input); }\n",
        );

        assert_eq!(transform.transform_kind, "metadata-only");
        assert!(transform.transformed_source.is_none());
        assert!(transform.export_names.is_empty());
    }

    #[test]
    fn transforms_imported_tsx_component_graphs() {
        let transform = transform_runtime_source(
            "tsx",
            "import { Badge } from './Badge';\nimport { formatLabel } from '../lib/formatLabel';\n\nexport function Hero() {\n  return <section><h1>DX Build System</h1><Badge label={formatLabel(\"linked modules\")} /></section>;\n}\n",
        );

        assert_eq!(transform.transform_kind, "tsx-component-runtime");
        assert_eq!(transform.export_names, vec!["Hero".to_string()]);
        let transformed = transform.transformed_source.expect("transformed source");
        assert!(transformed.contains("export function Hero()"));
        assert!(transformed.contains("dxCreateElement(\"section\""));
        assert!(transformed.contains("Badge({ label: formatLabel(\"linked modules\") })"));
    }

    #[test]
    fn transforms_simple_tsx_leaf_components() {
        let transform = transform_runtime_source(
            "tsx",
            "export function Badge(props: { label: string }) {\n  return <p data-badge=\"source-linked\">{props.label}</p>;\n}\n",
        );

        assert_eq!(transform.transform_kind, "tsx-leaf-runtime");
        assert_eq!(transform.export_names, vec!["Badge".to_string()]);
        let transformed = transform.transformed_source.expect("transformed source");
        assert!(transformed.contains("export function Badge(props)"));
        assert!(transformed.contains("dxCreateElement(\"p\""));
        assert!(transformed.contains("props.label"));
    }

    #[test]
    fn transforms_boolean_jsx_attributes() {
        let transform = transform_runtime_source(
            "tsx",
            "export function SubmitButton() {\n  return <button disabled data-action=\"submit\">Save</button>;\n}\n",
        );

        assert_eq!(transform.transform_kind, "tsx-leaf-runtime");
        let transformed = transform.transformed_source.expect("transformed source");
        assert!(transformed.contains("export function SubmitButton()"));
        assert!(transformed.contains("disabled: true"));
        assert!(transformed.contains("\"data-action\": \"submit\""));
    }
}
