use super::module_tsx_component_parser::parse_runtime_jsx;
use super::module_tsx_runtime::{
    TsxRuntimeTransform, default_export_line, dx_runtime_helper, exported_function, returned_jsx,
    strip_parameter_types,
};

pub fn transform_tsx_component_runtime(source: &str) -> Option<TsxRuntimeTransform> {
    let function = exported_function(source)?;
    let jsx = returned_jsx(function.body)?;
    let node = parse_runtime_jsx(jsx)?;
    let parameters = strip_parameter_types(function.parameters);
    let default_export = default_export_line(&function);
    let runtime_source = format!(
        "{}\nexport function {}({}) {{\n  return {};\n}}\n{}",
        dx_runtime_helper(),
        function.name,
        parameters,
        node.to_runtime_expression(),
        default_export
    );

    Some(TsxRuntimeTransform {
        transformed_source: runtime_source,
        export_names: vec![function.name.to_string()],
    })
}

#[cfg(test)]
mod tests {
    use super::transform_tsx_component_runtime;

    #[test]
    fn transforms_imported_component_trees() {
        let transform = transform_tsx_component_runtime(
            r#"import { Badge } from "./Badge";
import { formatLabel } from "../lib/formatLabel";

export function Hero() {
  return <section><h1>DX Build System</h1><Badge label={formatLabel("linked modules")} /></section>;
}
"#,
        )
        .expect("component transform");

        assert_eq!(transform.export_names, vec!["Hero".to_string()]);
        assert!(
            transform
                .transformed_source
                .contains("export function Hero()")
        );
        assert!(
            transform
                .transformed_source
                .contains("dxCreateElement(\"section\"")
        );
        assert!(
            transform
                .transformed_source
                .contains("dxCreateElement(\"h1\", {}, \"DX Build System\")")
        );
        assert!(
            transform
                .transformed_source
                .contains("Badge({ label: formatLabel(\"linked modules\") })")
        );
    }

    #[test]
    fn preserves_default_route_entry_export() {
        let transform = transform_tsx_component_runtime(
            r#"import { Hero } from "../components/Hero";
import "../styles/app.css";

export default function Page() {
  return <main className="hero"><Hero /><img src="/icons/mark.svg" alt="DX" /></main>;
}
"#,
        )
        .expect("page transform");

        assert_eq!(transform.export_names, vec!["Page".to_string()]);
        assert!(
            transform
                .transformed_source
                .contains("export function Page()")
        );
        assert!(
            transform
                .transformed_source
                .contains("export default Page;")
        );
        assert!(transform.transformed_source.contains("Hero({})"));
        assert!(
            transform
                .transformed_source
                .contains("dxCreateElement(\"img\"")
        );
    }
}
