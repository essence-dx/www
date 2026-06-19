use super::module_tsx_runtime::{is_identifier_char, matching_delimiter};

#[derive(Debug, Clone)]
pub struct RuntimeJsxNode {
    kind: RuntimeJsxNodeKind,
}

#[derive(Debug, Clone)]
enum RuntimeJsxNodeKind {
    Element(RuntimeJsxElement),
    ComponentCall(RuntimeJsxElement),
    Text(String),
    Expression(String),
}

#[derive(Debug, Clone)]
struct RuntimeJsxElement {
    tag: String,
    props: Vec<RuntimeJsxProp>,
    children: Vec<RuntimeJsxNode>,
}

#[derive(Debug, Clone)]
enum RuntimeJsxProp {
    Static { name: String, value: String },
    Boolean { name: String },
    Expression { name: String, expression: String },
}

impl RuntimeJsxNode {
    pub fn to_runtime_expression(&self) -> String {
        match &self.kind {
            RuntimeJsxNodeKind::Element(element) => {
                let children = element
                    .children
                    .iter()
                    .map(RuntimeJsxNode::to_runtime_expression)
                    .collect::<Vec<_>>();
                let children = if children.is_empty() {
                    String::new()
                } else {
                    format!(", {}", children.join(", "))
                };
                format!(
                    "dxCreateElement({}, {}{})",
                    json_string(&element.tag),
                    props_object(&element.props),
                    children
                )
            }
            RuntimeJsxNodeKind::ComponentCall(element) => {
                format!("{}({})", element.tag, props_object(&element.props))
            }
            RuntimeJsxNodeKind::Text(text) => json_string(text),
            RuntimeJsxNodeKind::Expression(expression) => expression.trim().to_string(),
        }
    }
}

pub fn parse_runtime_jsx(source: &str) -> Option<RuntimeJsxNode> {
    let source = source.trim();
    let (node, consumed) = parse_node(source, 0)?;
    if source[consumed..].trim().is_empty() {
        Some(node)
    } else {
        None
    }
}

fn parse_node(source: &str, index: usize) -> Option<(RuntimeJsxNode, usize)> {
    let index = skip_whitespace(source, index);
    if source[index..].starts_with('<') {
        parse_element(source, index)
    } else if source[index..].starts_with('{') {
        parse_expression(source, index)
    } else {
        parse_text(source, index)
    }
}

fn parse_element(source: &str, index: usize) -> Option<(RuntimeJsxNode, usize)> {
    if source[index..].starts_with("</") {
        return None;
    }

    let open_end = find_open_tag_end(source, index)?;
    let opening = source[index + 1..open_end].trim();
    let self_closing = opening.ends_with('/');
    let opening = opening.trim_end_matches('/').trim_end();
    let (tag, attrs) = opening
        .split_once(char::is_whitespace)
        .unwrap_or((opening, ""));
    if tag.is_empty() || !tag.chars().all(is_identifier_char) {
        return None;
    }
    let props = parse_props(attrs)?;

    if self_closing {
        let element = RuntimeJsxElement {
            tag: tag.to_string(),
            props,
            children: Vec::new(),
        };
        return Some((node_for_element(element), open_end + 1));
    }

    let mut children = Vec::new();
    let mut cursor = open_end + 1;
    let close_tag = format!("</{tag}>");
    while cursor < source.len() {
        cursor = skip_whitespace(source, cursor);
        if source[cursor..].starts_with(&close_tag) {
            let element = RuntimeJsxElement {
                tag: tag.to_string(),
                props,
                children,
            };
            return Some((
                RuntimeJsxNode {
                    kind: RuntimeJsxNodeKind::Element(element),
                },
                cursor + close_tag.len(),
            ));
        }

        let (child, consumed) = parse_node(source, cursor)?;
        children.push(child);
        cursor = consumed;
    }

    None
}

fn node_for_element(element: RuntimeJsxElement) -> RuntimeJsxNode {
    let kind = if element
        .tag
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_uppercase())
    {
        RuntimeJsxNodeKind::ComponentCall(element)
    } else {
        RuntimeJsxNodeKind::Element(element)
    };
    RuntimeJsxNode { kind }
}

fn parse_props(attrs: &str) -> Option<Vec<RuntimeJsxProp>> {
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
            props.push(RuntimeJsxProp::Boolean {
                name: name.to_string(),
            });
            continue;
        };
        rest = after_equals.trim_start();
        if let Some(quoted) = rest.strip_prefix('"') {
            let value_end = quoted.find('"')?;
            props.push(RuntimeJsxProp::Static {
                name: name.to_string(),
                value: quoted[..value_end].to_string(),
            });
            rest = quoted[value_end + 1..].trim_start();
        } else if rest.starts_with('{') {
            let end = matching_delimiter(rest, 0, '{', '}')?;
            props.push(RuntimeJsxProp::Expression {
                name: name.to_string(),
                expression: rest[1..end].trim().to_string(),
            });
            rest = rest[end + 1..].trim_start();
        } else {
            return None;
        }
    }

    Some(props)
}

fn parse_expression(source: &str, index: usize) -> Option<(RuntimeJsxNode, usize)> {
    let end = matching_delimiter(source, index, '{', '}')?;
    Some((
        RuntimeJsxNode {
            kind: RuntimeJsxNodeKind::Expression(source[index + 1..end].trim().to_string()),
        },
        end + 1,
    ))
}

fn parse_text(source: &str, index: usize) -> Option<(RuntimeJsxNode, usize)> {
    let end = source[index..]
        .find(['<', '{'])
        .map(|offset| index + offset)
        .unwrap_or(source.len());
    let text = source[index..end].trim();
    if text.is_empty() {
        None
    } else {
        Some((
            RuntimeJsxNode {
                kind: RuntimeJsxNodeKind::Text(text.to_string()),
            },
            end,
        ))
    }
}

fn find_open_tag_end(source: &str, index: usize) -> Option<usize> {
    let mut quote = None;
    let mut brace_depth = 0usize;
    for (offset, character) in source[index..].char_indices() {
        if offset == 0 {
            continue;
        }
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            }
            continue;
        }
        match character {
            '"' | '\'' => quote = Some(character),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '>' if brace_depth == 0 => return Some(index + offset),
            _ => {}
        }
    }
    None
}

fn props_object(props: &[RuntimeJsxProp]) -> String {
    if props.is_empty() {
        return "{}".to_string();
    }

    let entries = props
        .iter()
        .map(|prop| match prop {
            RuntimeJsxProp::Static { name, value } => {
                format!("{}: {}", prop_key(name), json_string(value))
            }
            RuntimeJsxProp::Boolean { name } => {
                format!("{}: true", prop_key(name))
            }
            RuntimeJsxProp::Expression { name, expression } => {
                format!("{}: {}", prop_key(name), expression.trim())
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("{{ {entries} }}")
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
        json_string(name)
    }
}

fn json_string(value: &str) -> String {
    serde_json::to_string(value).expect("serialize runtime string")
}

fn skip_whitespace(source: &str, mut index: usize) -> usize {
    while index < source.len() {
        let Some(character) = source[index..].chars().next() else {
            break;
        };
        if !character.is_whitespace() {
            break;
        }
        index += character.len_utf8();
    }
    index
}
