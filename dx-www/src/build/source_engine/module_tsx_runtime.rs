#[derive(Debug, Clone)]
pub struct TsxRuntimeTransform {
    pub transformed_source: String,
    pub export_names: Vec<String>,
}

pub fn transform_tsx_leaf_runtime(source: &str) -> Option<TsxRuntimeTransform> {
    let function = exported_function(source)?;
    let jsx = returned_jsx(function.body)?;
    let element = simple_jsx_element(jsx)?;
    let parameters = strip_parameter_types(function.parameters);
    let default_export = default_export_line(&function);
    let runtime_source = format!(
        "{}\nexport function {}({}) {{\n  return {};\n}}\n{}",
        dx_runtime_helper(),
        function.name,
        parameters,
        element.to_runtime_expression(),
        default_export
    );

    Some(TsxRuntimeTransform {
        transformed_source: runtime_source,
        export_names: vec![function.name.to_string()],
    })
}

pub(super) struct ExportedFunction<'a> {
    pub(super) name: &'a str,
    pub(super) parameters: &'a str,
    pub(super) body: &'a str,
    pub(super) is_default: bool,
}

struct JsxElement<'a> {
    tag: &'a str,
    props: Vec<JsxProp<'a>>,
    child_expression: Option<&'a str>,
}

enum JsxProp<'a> {
    Static { name: &'a str, value: &'a str },
    Boolean { name: &'a str },
}

impl JsxElement<'_> {
    fn to_runtime_expression(&self) -> String {
        let props = if self.props.is_empty() {
            "{}".to_string()
        } else {
            let entries = self
                .props
                .iter()
                .map(|prop| match prop {
                    JsxProp::Static { name, value } => {
                        format!(
                            "{}: {}",
                            prop_key(name),
                            serde_json::to_string(value).expect("serialize prop value")
                        )
                    }
                    JsxProp::Boolean { name } => format!("{}: true", prop_key(name)),
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{ {entries} }}")
        };
        let children = self
            .child_expression
            .map(|child| format!(", {}", child.trim()))
            .unwrap_or_default();
        format!(
            "dxCreateElement({}, {}{})",
            serde_json::to_string(self.tag).expect("serialize tag"),
            props,
            children
        )
    }
}

pub(super) fn exported_function(source: &str) -> Option<ExportedFunction<'_>> {
    let named_start = source.find("export function ");
    let default_start = source.find("export default function ");

    match (named_start, default_start) {
        (Some(named), Some(default)) if named <= default => {
            exported_function_at(source, named, "export function ", false)
        }
        (Some(named), None) => exported_function_at(source, named, "export function ", false),
        (_, Some(default)) => {
            exported_function_at(source, default, "export default function ", true)
        }
        (None, None) => None,
    }
}

fn exported_function_at<'a>(
    source: &'a str,
    marker_start: usize,
    marker: &str,
    is_default: bool,
) -> Option<ExportedFunction<'a>> {
    let start = marker_start + marker.len();
    let name_end = source[start..]
        .find(|character: char| !is_identifier_char(character))
        .map(|offset| start + offset)?;
    let name = &source[start..name_end];
    let open_paren = source[name_end..]
        .find('(')
        .map(|offset| name_end + offset)?;
    let close_paren = matching_delimiter(source, open_paren, '(', ')')?;
    let open_brace = source[close_paren..]
        .find('{')
        .map(|offset| close_paren + offset)?;
    let close_brace = matching_delimiter(source, open_brace, '{', '}')?;

    Some(ExportedFunction {
        name,
        parameters: &source[open_paren + 1..close_paren],
        body: &source[open_brace + 1..close_brace],
        is_default,
    })
}

pub(super) fn default_export_line(function: &ExportedFunction<'_>) -> String {
    if function.is_default {
        format!("export default {};\n", function.name)
    } else {
        String::new()
    }
}

pub(super) fn returned_jsx(body: &str) -> Option<&str> {
    let return_start = body.find("return ")? + "return ".len();
    let returned = body[return_start..].trim_start();
    let end = returned.rfind(';').unwrap_or(returned.len());
    Some(returned[..end].trim())
}

fn simple_jsx_element(source: &str) -> Option<JsxElement<'_>> {
    let source = source.strip_prefix('<')?;
    let open_end = source.find('>')?;
    let opening = source[..open_end].trim();
    let (tag, attrs) = opening
        .split_once(char::is_whitespace)
        .unwrap_or((opening, ""));
    if tag.is_empty() || !tag.chars().all(is_identifier_char) {
        return None;
    }

    let close_tag = format!("</{tag}>");
    let children = source[open_end + 1..].strip_suffix(&close_tag)?.trim();
    Some(JsxElement {
        tag,
        props: parse_static_attrs(attrs)?,
        child_expression: jsx_child_expression(children),
    })
}

fn parse_static_attrs(attrs: &str) -> Option<Vec<JsxProp<'_>>> {
    let mut props = Vec::new();
    let mut rest = attrs.trim();

    while !rest.is_empty() {
        let name_end = rest
            .find(|character: char| character.is_whitespace() || character == '=')
            .unwrap_or(rest.len());
        let name = rest[..name_end].trim();
        if name.is_empty() {
            return None;
        }
        rest = rest[name_end..].trim_start();
        let Some(after_equals) = rest.strip_prefix('=') else {
            props.push(JsxProp::Boolean { name });
            continue;
        };
        let quoted = after_equals.trim_start().strip_prefix('"')?;
        let value_end = quoted.find('"')?;
        props.push(JsxProp::Static {
            name,
            value: &quoted[..value_end],
        });
        rest = quoted[value_end + 1..].trim_start();
    }

    Some(props)
}

fn prop_key(name: &str) -> String {
    if name
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '$')
        && name
            .chars()
            .next()
            .is_some_and(|character| !character.is_ascii_digit())
    {
        name.to_string()
    } else {
        serde_json::to_string(name).expect("serialize prop name")
    }
}

fn jsx_child_expression(children: &str) -> Option<&str> {
    if children.is_empty() {
        return None;
    }
    children.strip_prefix('{')?.strip_suffix('}').map(str::trim)
}

pub(super) fn strip_parameter_types(parameters: &str) -> String {
    let mut output = String::new();
    let mut chars = parameters.chars().peekable();
    while let Some(character) = chars.next() {
        if character == ':' {
            skip_type_annotation(&mut chars);
        } else {
            output.push(character);
        }
    }
    output.trim().to_string()
}

fn skip_type_annotation<I>(chars: &mut std::iter::Peekable<I>)
where
    I: Iterator<Item = char>,
{
    let mut brace_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize;

    while let Some(next) = chars.peek().copied() {
        if brace_depth == 0
            && bracket_depth == 0
            && angle_depth == 0
            && matches!(next, ',' | ')' | '=')
        {
            break;
        }

        match next {
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            _ => {}
        }
        chars.next();
    }
}

pub(super) fn matching_delimiter(
    source: &str,
    open: usize,
    start: char,
    end: char,
) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, character) in source[open..].char_indices() {
        if character == start {
            depth += 1;
        } else if character == end {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(open + offset);
            }
        }
    }
    None
}

pub(super) fn is_identifier_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '$')
}

pub(super) fn dx_runtime_helper() -> &'static str {
    "const dxCreateElement = (tag, props = {}, ...children) => Object.freeze({ kind: \"dx.element\", tag, props: Object.freeze(props), children: Object.freeze(children) });"
}
