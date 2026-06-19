use ahash::AHashSet;

use super::{SourceScanDiagnostic, SourceScanDiagnosticKind};

const PLAIN_TEXT_SINGLE_WORD_UTILITIES: &[&str] = &[
    "absolute",
    "block",
    "collapse",
    "container",
    "contents",
    "fixed",
    "flex",
    "flow-root",
    "grid",
    "hidden",
    "inline",
    "inline-block",
    "inline-flex",
    "inline-grid",
    "invisible",
    "isolate",
    "relative",
    "sr-only",
    "static",
    "sticky",
    "table",
    "table-cell",
    "table-row",
    "visible",
];

const PLAIN_TEXT_UTILITY_PREFIXES: &[&str] = &[
    "accent-",
    "align-",
    "animate-",
    "appearance-",
    "aspect-",
    "backdrop-",
    "backface-",
    "basis-",
    "bg-",
    "blur",
    "border",
    "border-spacing-",
    "bottom-",
    "box-",
    "break-",
    "brightness-",
    "caption-",
    "caret-",
    "clear",
    "col-",
    "columns-",
    "content-",
    "contrast-",
    "cursor-",
    "decoration-",
    "delay-",
    "divide-",
    "drop-shadow",
    "duration-",
    "ease-",
    "end-",
    "field-sizing-",
    "fill-",
    "filter",
    "flex-",
    "float",
    "font-",
    "forced-color-adjust-",
    "gap-",
    "gradient-",
    "grayscale",
    "grid-",
    "grow",
    "h-",
    "hue-rotate",
    "hyphens-",
    "inset-",
    "inset-shadow",
    "invert",
    "items-",
    "justify-",
    "leading-",
    "left-",
    "line-clamp-",
    "list-",
    "m-",
    "mask-",
    "max-",
    "mb-",
    "me-",
    "min-",
    "mix-blend-",
    "ml-",
    "mr-",
    "ms-",
    "mt-",
    "mx-",
    "my-",
    "object-",
    "opacity-",
    "order-",
    "origin-",
    "outline",
    "overflow-",
    "overscroll-",
    "p-",
    "perspective-",
    "place-",
    "pointer-events-",
    "pr-",
    "pe-",
    "ps-",
    "px-",
    "py-",
    "resize",
    "right-",
    "ring",
    "rotate-",
    "rounded",
    "row-",
    "saturate",
    "scale-",
    "scroll-",
    "select-",
    "sepia",
    "shadow",
    "shrink",
    "size-",
    "skew-",
    "snap-",
    "space-",
    "start-",
    "stroke-",
    "table-",
    "text-",
    "to-",
    "top-",
    "touch-",
    "tracking-",
    "transform-",
    "transition",
    "translate-",
    "underline-offset-",
    "via-",
    "w-",
    "whitespace-",
    "will-change-",
    "z-",
];

#[derive(Debug)]
struct StaticStringLiteral {
    value: String,
    previous: Option<char>,
    next: Option<char>,
    line: usize,
    column: usize,
}

pub(crate) struct PlainTextScanReport {
    pub tokens: Vec<String>,
    pub diagnostics: Vec<SourceScanDiagnostic>,
}

enum PlainTextCandidateVerdict {
    Accepted,
    Rejected(SourceScanDiagnosticKind),
}

#[cfg(test)]
pub(crate) fn extract_plain_text_class_tokens(source: &str) -> Vec<String> {
    scan_plain_text_class_tokens(source).tokens
}

pub(crate) fn scan_plain_text_class_tokens(source: &str) -> PlainTextScanReport {
    let mut tokens = AHashSet::default();
    let mut diagnostics = Vec::new();

    for literal in static_quoted_string_literals(source) {
        if literal.value.contains("${") || literal.value.contains('$') {
            diagnostics.push(diagnostic_for_literal(
                SourceScanDiagnosticKind::DynamicFragment,
                &literal,
                &literal.value,
            ));
            continue;
        }

        if is_object_key(&literal) {
            diagnostics.push(diagnostic_for_literal(
                SourceScanDiagnosticKind::ObjectKey,
                &literal,
                &literal.value,
            ));
            continue;
        }

        let mut expanded = AHashSet::default();
        let mut collector = super::GroupCollector::default();
        super::expand_grouping_into(&literal.value, &mut expanded, &mut collector);
        let mut candidates = expanded
            .into_iter()
            .map(|token| trim_plain_text_class_candidate(&token))
            .filter(|token| !token.is_empty())
            .collect::<Vec<_>>();
        candidates.sort();

        for token in candidates {
            match plain_text_candidate_verdict(&token) {
                PlainTextCandidateVerdict::Accepted => {
                    if !tokens.insert(token.clone()) {
                        diagnostics.push(diagnostic_for_token(
                            SourceScanDiagnosticKind::DuplicateCandidate,
                            &literal,
                            &token,
                        ));
                    }
                }
                PlainTextCandidateVerdict::Rejected(kind) => {
                    diagnostics.push(diagnostic_for_token(kind, &literal, &token));
                }
            }
        }
    }

    let mut tokens = tokens.into_iter().collect::<Vec<_>>();
    tokens.sort();
    PlainTextScanReport {
        tokens,
        diagnostics,
    }
}

fn static_quoted_string_literals(source: &str) -> Vec<StaticStringLiteral> {
    let chars = source.chars().collect::<Vec<_>>();
    let mut literals = Vec::new();
    let mut index = 0usize;

    while index < chars.len() {
        let quote = chars[index];
        if !matches!(quote, '"' | '\'' | '`') {
            index += 1;
            continue;
        }

        let previous = previous_nonspace(&chars, index);
        let (line, column) = line_column_at(&chars, index);
        index += 1;
        let start = index;
        let mut escaped = false;

        while index < chars.len() {
            let ch = chars[index];
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                break;
            }
            index += 1;
        }

        if start < index && index < chars.len() {
            literals.push(StaticStringLiteral {
                value: chars[start..index].iter().collect(),
                previous,
                next: next_nonspace(&chars, index + 1),
                line,
                column,
            });
        }

        index += 1;
    }

    literals
}

fn previous_nonspace(chars: &[char], before: usize) -> Option<char> {
    chars[..before]
        .iter()
        .rev()
        .copied()
        .find(|ch| !ch.is_whitespace())
}

fn next_nonspace(chars: &[char], after: usize) -> Option<char> {
    chars[after..]
        .iter()
        .copied()
        .find(|ch| !ch.is_whitespace())
}

fn is_object_key(literal: &StaticStringLiteral) -> bool {
    matches!(literal.previous, Some('{') | Some(',')) && matches!(literal.next, Some(':'))
}

fn trim_plain_text_class_candidate(token: &str) -> String {
    token
        .trim_matches(|ch: char| matches!(ch, '`' | '"' | '\'' | ',' | ';'))
        .to_string()
}

#[cfg(test)]
fn is_plain_text_class_candidate(token: &str) -> bool {
    matches!(
        plain_text_candidate_verdict(token),
        PlainTextCandidateVerdict::Accepted
    )
}

fn plain_text_candidate_verdict(token: &str) -> PlainTextCandidateVerdict {
    if token.len() < 2 || token.len() > 180 {
        return PlainTextCandidateVerdict::Rejected(SourceScanDiagnosticKind::UnsafeCandidate);
    }
    if token.contains("${")
        || token.contains('$')
        || token.contains("://")
        || token.contains('\\')
        || token
            .chars()
            .any(|ch| ch.is_control() || matches!(ch, '<' | '>' | ';' | '{' | '}'))
    {
        return PlainTextCandidateVerdict::Rejected(SourceScanDiagnosticKind::UnsafeCandidate);
    }

    let base = plain_text_candidate_base(token);
    let base = base.strip_prefix('!').unwrap_or(base);
    let base = base.strip_prefix('-').unwrap_or(base);
    if base.starts_with('[') && base.ends_with(']') {
        if is_bracketed_attribute_selector(base) {
            return PlainTextCandidateVerdict::Rejected(
                SourceScanDiagnosticKind::NonUtilityCandidate,
            );
        }
        return PlainTextCandidateVerdict::Accepted;
    }
    if base.starts_with('@') {
        if base.contains(':') || base.contains('-') {
            return PlainTextCandidateVerdict::Accepted;
        }
        return PlainTextCandidateVerdict::Rejected(SourceScanDiagnosticKind::NonUtilityCandidate);
    }
    if PLAIN_TEXT_SINGLE_WORD_UTILITIES.contains(&base) {
        return PlainTextCandidateVerdict::Accepted;
    }

    if let Some(accepted) = precise_plain_text_utility_prefix_match(base) {
        return if accepted {
            PlainTextCandidateVerdict::Accepted
        } else {
            PlainTextCandidateVerdict::Rejected(SourceScanDiagnosticKind::NonUtilityCandidate)
        };
    }

    if PLAIN_TEXT_UTILITY_PREFIXES
        .iter()
        .any(|prefix| base.starts_with(prefix))
    {
        PlainTextCandidateVerdict::Accepted
    } else {
        PlainTextCandidateVerdict::Rejected(SourceScanDiagnosticKind::NonUtilityCandidate)
    }
}

fn precise_plain_text_utility_prefix_match(base: &str) -> Option<bool> {
    for prefix in [
        "bottom-", "end-", "inset-", "left-", "right-", "start-", "top-",
    ] {
        if let Some(value) = base.strip_prefix(prefix) {
            return Some(is_plain_text_spacing_utility_value(value));
        }
    }
    for prefix in [
        "m-",
        "mb-",
        "me-",
        "ml-",
        "mr-",
        "ms-",
        "mt-",
        "mx-",
        "my-",
        "p-",
        "pe-",
        "pl-",
        "pr-",
        "ps-",
        "pt-",
        "px-",
        "py-",
        "scroll-m-",
        "scroll-mb-",
        "scroll-me-",
        "scroll-ml-",
        "scroll-mr-",
        "scroll-ms-",
        "scroll-mt-",
        "scroll-mx-",
        "scroll-my-",
        "scroll-p-",
        "scroll-pb-",
        "scroll-pe-",
        "scroll-pl-",
        "scroll-pr-",
        "scroll-ps-",
        "scroll-pt-",
        "scroll-px-",
        "scroll-py-",
    ] {
        if let Some(value) = base.strip_prefix(prefix) {
            return Some(is_plain_text_spacing_utility_value(value));
        }
    }
    if let Some(value) = base.strip_prefix("clear-") {
        return Some(matches!(
            value,
            "left" | "right" | "both" | "start" | "end" | "none"
        ));
    }
    if let Some(value) = base.strip_prefix("scale-") {
        return Some(is_plain_text_numeric_or_arbitrary_value(value));
    }
    if let Some(value) = base.strip_prefix("scroll-") {
        return Some(matches!(value, "auto" | "smooth"));
    }
    if base.starts_with("transition") {
        return Some(
            base == "transition"
                || base
                    .strip_prefix("transition-")
                    .is_some_and(is_plain_text_transition_value),
        );
    }
    None
}

fn is_bracketed_attribute_selector(value: &str) -> bool {
    let Some(inner) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    else {
        return false;
    };
    let lower = inner.to_ascii_lowercase();
    inner.contains('=')
        && (lower.starts_with("data-")
            || lower.starts_with("aria-")
            || lower.starts_with("role")
            || lower.starts_with("slot"))
}

fn is_plain_text_spacing_utility_value(value: &str) -> bool {
    matches!(
        value,
        "0" | "px"
            | "auto"
            | "full"
            | "screen"
            | "svw"
            | "lvw"
            | "dvw"
            | "svh"
            | "lvh"
            | "dvh"
            | "min"
            | "max"
            | "fit"
    ) || value.starts_with('[')
        || value.starts_with('(')
        || value.starts_with("token(")
        || is_plain_text_numeric_or_arbitrary_value(value)
}

fn is_plain_text_numeric_or_arbitrary_value(value: &str) -> bool {
    value.starts_with('[')
        || value.starts_with('(')
        || value.starts_with("token(")
        || is_plain_text_decimal_number(value)
        || is_plain_text_fraction(value)
}

fn is_plain_text_decimal_number(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_digit() || matches!(ch, '.' | '/'))
        && value.chars().any(|ch| ch.is_ascii_digit())
}

fn is_plain_text_fraction(value: &str) -> bool {
    let Some((numerator, denominator)) = value.split_once('/') else {
        return false;
    };
    numerator.chars().all(|ch| ch.is_ascii_digit())
        && denominator.chars().all(|ch| ch.is_ascii_digit())
        && !numerator.is_empty()
        && !denominator.is_empty()
}

fn is_plain_text_transition_value(value: &str) -> bool {
    matches!(
        value,
        "none" | "all" | "colors" | "opacity" | "shadow" | "transform"
    ) || value.starts_with('[')
        || value.starts_with('(')
        || value.starts_with("token(")
}

fn plain_text_candidate_base(token: &str) -> &str {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut last_colon = None;

    for (index, byte) in token.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b':' if bracket_depth == 0 && paren_depth == 0 => last_colon = Some(index),
            _ => {}
        }
    }

    last_colon.map_or(token, |index| &token[index + 1..])
}

fn diagnostic_for_literal(
    kind: SourceScanDiagnosticKind,
    literal: &StaticStringLiteral,
    token: &str,
) -> SourceScanDiagnostic {
    SourceScanDiagnostic {
        kind,
        token: token.to_string(),
        line: literal.line,
        column: literal.column,
        message: format!("{}: {}", kind.as_str(), source_scan_message(kind, token)),
    }
}

fn diagnostic_for_token(
    kind: SourceScanDiagnosticKind,
    literal: &StaticStringLiteral,
    token: &str,
) -> SourceScanDiagnostic {
    SourceScanDiagnostic {
        kind,
        token: token.to_string(),
        line: literal.line,
        column: literal.column + token_column_offset(&literal.value, token),
        message: format!("{}: {}", kind.as_str(), source_scan_message(kind, token)),
    }
}

fn source_scan_message(kind: SourceScanDiagnosticKind, token: &str) -> &'static str {
    match kind {
        SourceScanDiagnosticKind::DynamicFragment => {
            "dynamic class fragment cannot be generated by static source scanning"
        }
        SourceScanDiagnosticKind::ObjectKey => {
            "object map key skipped; static class scanning reads values, not keys"
        }
        SourceScanDiagnosticKind::UnsafeCandidate => {
            "plain-text candidate rejected before utility generation because it is unsafe"
        }
        SourceScanDiagnosticKind::NonUtilityCandidate => {
            if token.is_empty() {
                "empty plain-text candidate rejected"
            } else {
                "plain-text candidate is not a Tailwind-like utility"
            }
        }
        SourceScanDiagnosticKind::DuplicateCandidate => {
            "duplicate static class candidate skipped after first occurrence"
        }
    }
}

fn token_column_offset(literal_value: &str, token: &str) -> usize {
    literal_value.find(token).map_or(1, |index| index + 1)
}

fn line_column_at(chars: &[char], at: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut column = 1usize;

    for ch in &chars[..at] {
        if *ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    (line, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_extraction_reads_static_arrays_object_maps_and_helpers() {
        let source = r#"
const variants = {
  primary: "bg-blue-600 text-white",
  disabled: 'disabled:opacity-50',
  layout: "grid-cols-2",
};
const states = ["grid", "data-[state=open]:block"];
const helper = cn("px-4", selected && "hover:bg-blue-500", `md:grid`);
"#;

        let tokens = extract_plain_text_class_tokens(source);

        for expected in [
            "bg-blue-600",
            "text-white",
            "disabled:opacity-50",
            "grid-cols-2",
            "grid",
            "data-[state=open]:block",
            "px-4",
            "hover:bg-blue-500",
            "md:grid",
        ] {
            assert!(
                tokens.contains(&expected.to_string()),
                "expected static literal token {expected}: {tokens:?}"
            );
        }

        assert!(is_plain_text_class_candidate("data-[state=open]:block"));
    }

    #[test]
    fn plain_text_extraction_rejects_dynamic_fragments_and_prose() {
        let source = r#"
const variants = { "grid": "grid-cols-2" };
const dynamic = `text-${tone}-600`;
const copy = "card shell copy";
const url = "https://example.com";
const css = "color: red;";
"#;

        let tokens = extract_plain_text_class_tokens(source);

        assert!(tokens.contains(&"grid-cols-2".to_string()));
        assert!(!tokens.contains(&"grid".to_string()));
        assert!(!tokens.iter().any(|token| token.contains("${")));
        assert!(!tokens.iter().any(|token| token.contains('$')));
        assert!(!tokens.contains(&"card".to_string()));
        assert!(!tokens.contains(&"shell".to_string()));
        assert!(!tokens.contains(&"copy".to_string()));
        assert!(!tokens.iter().any(|token| token.contains("://")));
        assert!(!is_plain_text_class_candidate("card"));
    }

    #[test]
    fn plain_text_diagnostics_report_dynamic_object_key_unsafe_prose_and_duplicates() {
        let source = r#"
const variants = { "grid": "grid-cols-2" };
const dynamic = `text-${tone}-600`;
const prose = "card shell copy";
const url = "https://example.com";
const duplicate = ["px-4", "px-4"];
"#;

        let report = scan_plain_text_class_tokens(source);
        let diagnostic_kinds = report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.kind)
            .collect::<Vec<_>>();

        for expected in [
            SourceScanDiagnosticKind::ObjectKey,
            SourceScanDiagnosticKind::DynamicFragment,
            SourceScanDiagnosticKind::NonUtilityCandidate,
            SourceScanDiagnosticKind::UnsafeCandidate,
            SourceScanDiagnosticKind::DuplicateCandidate,
        ] {
            assert!(
                diagnostic_kinds.contains(&expected),
                "expected {expected:?} in {:?}",
                report.diagnostics
            );
        }

        let dynamic = report
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.kind == SourceScanDiagnosticKind::DynamicFragment)
            .expect("dynamic fragment diagnostic");
        assert_eq!(dynamic.token, "text-${tone}-600");
        assert!(dynamic.line > 1);
        assert!(dynamic.column > 1);
        assert!(dynamic.message.contains("dynamic class fragment"));

        assert!(report.tokens.contains(&"grid-cols-2".to_string()));
        assert!(report.tokens.contains(&"px-4".to_string()));
    }

    #[test]
    fn plain_text_extraction_ignores_metadata_selectors_and_prose_like_strings() {
        let source = r#"
const attrs = ['[data-dx-component="template-worker-registry"]', '[data-trpc-interaction="health-query"]'];
const copy = ["bottom-right", "clear-boundary-review", "end-to-end", "my-app", "scale-in", "scroll-linked", "transitions"];
const classes = ["bottom-4", "my-4", "scale-95", "scroll-smooth", "transition-colors"];
"#;

        let tokens = extract_plain_text_class_tokens(source);

        for rejected in [
            "[data-dx-component=\"template-worker-registry\"]",
            "[data-trpc-interaction=\"health-query\"]",
            "bottom-right",
            "clear-boundary-review",
            "end-to-end",
            "my-app",
            "scale-in",
            "scroll-linked",
            "transitions",
        ] {
            assert!(
                !tokens.contains(&rejected.to_string()),
                "plain-text scanner should ignore {rejected}: {tokens:?}"
            );
        }

        for accepted in [
            "bottom-4",
            "my-4",
            "scale-95",
            "scroll-smooth",
            "transition-colors",
        ] {
            assert!(
                tokens.contains(&accepted.to_string()),
                "plain-text scanner should keep {accepted}: {tokens:?}"
            );
        }
    }
}
