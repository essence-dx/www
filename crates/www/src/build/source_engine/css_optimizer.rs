use crate::parser::style::{AtRule, CssDeclaration, CssRule, ParsedStyle};

use super::css_usage::CssUsage;

#[derive(Debug, Clone)]
pub struct CssOptimization {
    pub style: ParsedStyle,
    pub original_rule_count: usize,
    pub retained_rule_count: usize,
    pub pruned_rule_count: usize,
    pub minified: bool,
}

pub fn optimize_css(style: &ParsedStyle, usage: &CssUsage) -> CssOptimization {
    let original_rule_count = style.rules.len();
    let rules = style
        .rules
        .iter()
        .filter(|rule| reachable_rule(rule, usage))
        .cloned()
        .collect::<Vec<_>>();
    let retained_rule_count = rules.len();

    CssOptimization {
        style: ParsedStyle {
            source: style.source.clone(),
            scoped: style.scoped,
            rules,
            atomic_classes: style.atomic_classes.clone(),
            custom_properties: style.custom_properties.clone(),
            imports: style.imports.clone(),
        },
        original_rule_count,
        retained_rule_count,
        pruned_rule_count: original_rule_count.saturating_sub(retained_rule_count),
        minified: true,
    }
}

pub fn render_minified_css(style: &ParsedStyle) -> String {
    let mut output = String::new();
    for import in &style.imports {
        output.push_str("@import ");
        output.push_str(&json_string(&import.url));
        if let Some(media) = &import.media {
            output.push(' ');
            output.push_str(media.trim());
        }
        output.push(';');
    }

    for rule in &style.rules {
        render_rule(&mut output, rule);
    }

    output
}

fn reachable_rule(rule: &CssRule, usage: &CssUsage) -> bool {
    if usage.is_empty() || usage.has_unknown_class_usage() || root_selector(&rule.selector) {
        return true;
    }

    if matches!(&rule.at_rule, Some(AtRule::Nested { .. })) {
        return conditional_rule_reaches_usage(rule, usage);
    }

    if rule.at_rule.is_some() {
        return true;
    }

    selector_reaches_usage(&rule.selector, usage)
}

fn conditional_rule_reaches_usage(rule: &CssRule, usage: &CssUsage) -> bool {
    selector_reaches_usage(&rule.selector, usage)
}

fn selector_reaches_usage(selector: &str, usage: &CssUsage) -> bool {
    let classes = selector_classes(selector);
    classes.is_empty()
        || classes
            .iter()
            .any(|class_name| usage.contains_class(class_name))
}

fn render_rule(output: &mut String, rule: &CssRule) {
    match &rule.at_rule {
        Some(AtRule::Media { query }) => {
            output.push_str("@media ");
            output.push_str(query.trim());
            output.push('{');
            render_nested_rule(output, rule);
            output.push('}');
        }
        Some(AtRule::Container { query }) => {
            output.push_str("@container ");
            output.push_str(query.trim());
            output.push('{');
            render_nested_rule(output, rule);
            output.push('}');
        }
        Some(AtRule::Keyframes { name }) => {
            output.push_str("@keyframes ");
            output.push_str(name.trim());
            output.push('{');
            render_declarations(output, &rule.declarations);
            output.push('}');
        }
        Some(AtRule::Supports { condition }) => {
            output.push_str("@supports ");
            output.push_str(condition.trim());
            output.push('{');
            render_nested_rule(output, rule);
            output.push('}');
        }
        Some(AtRule::Layer { name }) => {
            output.push_str("@layer");
            if let Some(name) = name {
                output.push(' ');
                output.push_str(name.trim());
            }
            output.push('{');
            render_nested_rule(output, rule);
            output.push('}');
        }
        Some(AtRule::Nested { rules }) => {
            render_minified_at_rule_stack(output, rules, rule);
        }
        Some(AtRule::Import { .. }) => {}
        None => {
            output.push_str(&compact_selector(&rule.selector));
            output.push('{');
            render_declarations(output, &rule.declarations);
            output.push('}');
        }
    }
}

fn render_minified_at_rule_stack(output: &mut String, rules: &[AtRule], rule: &CssRule) {
    let mut close_count = 0usize;
    for at_rule in rules {
        close_count += render_minified_at_rule_start(output, at_rule);
    }

    render_nested_rule(output, rule);
    for _ in 0..close_count {
        output.push('}');
    }
}

fn render_minified_at_rule_start(output: &mut String, at_rule: &AtRule) -> usize {
    match at_rule {
        AtRule::Media { query } => {
            output.push_str("@media ");
            output.push_str(query.trim());
            output.push('{');
            1
        }
        AtRule::Container { query } => {
            output.push_str("@container ");
            output.push_str(query.trim());
            output.push('{');
            1
        }
        AtRule::Supports { condition } => {
            output.push_str("@supports ");
            output.push_str(condition.trim());
            output.push('{');
            1
        }
        AtRule::Layer { name } => {
            output.push_str("@layer");
            if let Some(name) = name {
                output.push(' ');
                output.push_str(name.trim());
            }
            output.push('{');
            1
        }
        AtRule::Nested { rules } => rules
            .iter()
            .map(|rule| render_minified_at_rule_start(output, rule))
            .sum(),
        AtRule::Keyframes { name } => {
            output.push_str("@keyframes ");
            output.push_str(name.trim());
            output.push('{');
            1
        }
        AtRule::Import { .. } => 0,
    }
}

fn render_nested_rule(output: &mut String, rule: &CssRule) {
    let selector = rule.selector.trim();
    if selector.is_empty() || selector.starts_with('@') {
        render_declarations(output, &rule.declarations);
        return;
    }

    output.push_str(&compact_selector(selector));
    output.push('{');
    render_declarations(output, &rule.declarations);
    output.push('}');
}

fn render_declarations(output: &mut String, declarations: &[CssDeclaration]) {
    for declaration in declarations {
        output.push_str(declaration.property.trim());
        output.push(':');
        output.push_str(declaration.value.trim());
        if declaration.important {
            output.push_str("!important");
        }
        output.push(';');
    }
}

fn selector_classes(selector: &str) -> Vec<String> {
    let mut classes = Vec::new();
    let chars = selector.char_indices().collect::<Vec<_>>();
    let mut index = 0usize;

    while index < chars.len() {
        let (byte_index, character) = chars[index];
        if character != '.' {
            index += 1;
            continue;
        }

        let start = byte_index + 1;
        let mut end = start;
        index += 1;
        while index < chars.len() {
            let (next_byte, next) = chars[index];
            if !is_selector_identifier(next) {
                break;
            }
            end = next_byte + next.len_utf8();
            index += 1;
        }
        if end > start {
            classes.push(selector[start..end].to_string());
        }
    }

    classes
}

fn root_selector(selector: &str) -> bool {
    selector
        .split(',')
        .map(str::trim)
        .any(|selector| matches!(selector, ":root" | "html" | "body"))
}

fn compact_selector(selector: &str) -> String {
    selector.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_selector_identifier(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '-' | '_')
}

fn json_string(value: &str) -> String {
    serde_json::to_string(value).expect("serialize css string")
}

#[cfg(test)]
mod tests {
    use crate::parser::style::StyleParser;

    use super::*;
    use crate::build::source_engine::css_usage::collect_css_usage;

    #[test]
    fn prunes_unreachable_class_rules_and_minifies_css() {
        let project = tempfile::tempdir().expect("temp project");
        std::fs::create_dir_all(project.path().join("app")).expect("app dir");
        std::fs::write(
            project.path().join("app/page.tsx"),
            r#"export default function Page() { return <main className="hero"></main>; }"#,
        )
        .expect("page");
        let usage = collect_css_usage(project.path()).expect("usage");
        let parsed = StyleParser::new()
            .parse(
                ":root { --dx-accent: rgb(10 20 30); }\n.hero { display: grid; }\n.unused-card { color: red; }\n",
                false,
                &project.path().join("styles/app.css"),
            )
            .expect("css");

        let optimized = optimize_css(&parsed, &usage);
        let css = render_minified_css(&optimized.style);

        assert_eq!(optimized.original_rule_count, 3);
        assert_eq!(optimized.retained_rule_count, 2);
        assert_eq!(optimized.pruned_rule_count, 1);
        assert_eq!(css, ":root{--dx-accent:rgb(10 20 30);}.hero{display:grid;}");
    }

    #[test]
    fn retains_class_rules_when_static_usage_is_unknown() {
        let project = tempfile::tempdir().expect("temp project");
        std::fs::create_dir_all(project.path().join("app")).expect("app dir");
        std::fs::write(
            project.path().join("app/page.tsx"),
            r#"export default function Page({ status }) { return <main className={`state-${status}`}></main>; }"#,
        )
        .expect("page");
        let usage = collect_css_usage(project.path()).expect("usage");
        let parsed = StyleParser::new()
            .parse(
                ".state-open { display: block; }\n.state-closed { display: none; }\n",
                false,
                &project.path().join("styles/app.css"),
            )
            .expect("css");

        let optimized = optimize_css(&parsed, &usage);
        let css = render_minified_css(&optimized.style);

        assert_eq!(optimized.original_rule_count, 2);
        assert_eq!(optimized.retained_rule_count, 2);
        assert_eq!(optimized.pruned_rule_count, 0);
        assert_eq!(
            css,
            ".state-open{display:block;}.state-closed{display:none;}"
        );
    }

    #[test]
    fn retains_class_rules_when_dynamic_usage_mixes_with_static_tokens() {
        let project = tempfile::tempdir().expect("temp project");
        std::fs::create_dir_all(project.path().join("app")).expect("app dir");
        std::fs::write(
            project.path().join("app/page.tsx"),
            r#"export default function Page({ status }) {
  return <main className="hero"><section className={`state-${status}`}></section></main>;
}"#,
        )
        .expect("page");
        let usage = collect_css_usage(project.path()).expect("usage");
        let parsed = StyleParser::new()
            .parse(
                ".hero { display: grid; }\n.state-open { display: block; }\n.unused-card { color: red; }\n",
                false,
                &project.path().join("styles/app.css"),
            )
            .expect("css");

        let optimized = optimize_css(&parsed, &usage);
        let css = render_minified_css(&optimized.style);

        assert_eq!(optimized.original_rule_count, 3);
        assert_eq!(optimized.retained_rule_count, 3);
        assert_eq!(optimized.pruned_rule_count, 0);
        assert_eq!(
            css,
            ".hero{display:grid;}.state-open{display:block;}.unused-card{color:red;}"
        );
    }

    #[test]
    fn renders_nested_media_rules() {
        let project = tempfile::tempdir().expect("temp project");
        std::fs::create_dir_all(project.path().join("app")).expect("app dir");
        let usage = collect_css_usage(project.path()).expect("usage");
        let parsed = StyleParser::new()
            .parse(
                "@media print { .print-only { color: black; } }",
                false,
                &project.path().join("styles/app.css"),
            )
            .expect("css");

        let optimized = optimize_css(&parsed, &usage);
        let css = render_minified_css(&optimized.style);

        assert_eq!(optimized.original_rule_count, 1);
        assert_eq!(optimized.retained_rule_count, 1);
        assert_eq!(css, "@media print{.print-only{color:black;}}");
    }

    #[test]
    fn renders_nested_supports_and_layer_rules() {
        let project = tempfile::tempdir().expect("temp project");
        std::fs::create_dir_all(project.path().join("app")).expect("app dir");
        let usage = collect_css_usage(project.path()).expect("usage");
        let parsed = StyleParser::new()
            .parse(
                concat!(
                    "@supports (display: grid) { .supports-grid { display: grid; } }",
                    "@layer components { .layer-card { color: red; } }"
                ),
                false,
                &project.path().join("styles/app.css"),
            )
            .expect("css");

        let optimized = optimize_css(&parsed, &usage);
        let css = render_minified_css(&optimized.style);

        assert_eq!(optimized.original_rule_count, 2);
        assert_eq!(optimized.retained_rule_count, 2);
        assert_eq!(
            css,
            "@supports (display: grid){.supports-grid{display:grid;}}@layer components{.layer-card{color:red;}}"
        );
    }

    #[test]
    fn renders_nested_container_rules() {
        let project = tempfile::tempdir().expect("temp project");
        std::fs::create_dir_all(project.path().join("app")).expect("app dir");
        let usage = collect_css_usage(project.path()).expect("usage");
        let parsed = StyleParser::new()
            .parse(
                "@container card (min-width: 32rem) { .container-card { display: grid; } }",
                false,
                &project.path().join("styles/app.css"),
            )
            .expect("css");

        let optimized = optimize_css(&parsed, &usage);
        let css = render_minified_css(&optimized.style);

        assert_eq!(optimized.original_rule_count, 1);
        assert_eq!(optimized.retained_rule_count, 1);
        assert_eq!(
            css,
            "@container card (min-width: 32rem){.container-card{display:grid;}}"
        );
    }

    #[test]
    fn renders_stacked_conditional_rules() {
        let project = tempfile::tempdir().expect("temp project");
        std::fs::create_dir_all(project.path().join("app")).expect("app dir");
        let usage = collect_css_usage(project.path()).expect("usage");
        let parsed = StyleParser::new()
            .parse(
                "@supports (display: grid) { @media (min-width: 40rem) { .stacked-card { display: grid; } } }",
                false,
                &project.path().join("styles/app.css"),
            )
            .expect("css");

        let optimized = optimize_css(&parsed, &usage);
        let css = render_minified_css(&optimized.style);

        assert_eq!(optimized.original_rule_count, 1);
        assert_eq!(optimized.retained_rule_count, 1);
        assert_eq!(
            css,
            "@supports (display: grid){@media (min-width: 40rem){.stacked-card{display:grid;}}}"
        );
    }

    #[test]
    fn prunes_unreachable_nested_conditional_rules() {
        let project = tempfile::tempdir().expect("temp project");
        std::fs::create_dir_all(project.path().join("app")).expect("app dir");
        std::fs::write(
            project.path().join("app/page.tsx"),
            r#"export default function Page() {
  return <main className="hero-card">DX CSS</main>;
}"#,
        )
        .expect("page");
        let usage = collect_css_usage(project.path()).expect("usage");
        let parsed = StyleParser::new()
            .parse(
                concat!(
                    "@container card (min-width: 32rem) {",
                    "@supports (display: grid) {",
                    ".hero-card { display: grid; }",
                    ".unused-card { color: red; }",
                    "}",
                    "}"
                ),
                false,
                &project.path().join("styles/app.css"),
            )
            .expect("css");

        let optimized = optimize_css(&parsed, &usage);
        let css = render_minified_css(&optimized.style);

        assert_eq!(optimized.original_rule_count, 2);
        assert_eq!(optimized.retained_rule_count, 1);
        assert_eq!(optimized.pruned_rule_count, 1);
        assert_eq!(
            css,
            "@container card (min-width: 32rem){@supports (display: grid){.hero-card{display:grid;}}}"
        );
        assert!(!css.contains(".unused-card"));
    }
}
