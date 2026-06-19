use std::collections::BTreeSet;

use style::parser::{SourceScanDiagnostic, SourceScanDiagnosticKind};

const TAILWIND_UTILITY_PREFIXES: &[&str] = &[
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
    "box-decoration-",
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
    "isolate",
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
    "visible",
    "w-",
    "whitespace-",
    "will-change-",
    "z-",
];

const EXACT_TAILWIND_UTILITY_NAMES: &[&str] = &[
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
    "not-sr-only",
    "prose",
    "relative",
    "sr-only",
    "static",
    "sticky",
    "table",
    "table-cell",
    "table-row",
    "text-shadow-sm",
    "visible",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct UnsupportedScannedClass {
    pub(super) class_name: String,
    pub(super) reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceScanDiagnosticFinding {
    pub(super) source_file: String,
    pub(super) kind: &'static str,
    pub(super) severity: &'static str,
    pub(super) token: String,
    pub(super) line: usize,
    pub(super) column: usize,
    pub(super) reason: String,
}

pub(super) fn class_has_generated_css(class_name: &str) -> bool {
    class_name.starts_with("dx-") || style::core::css_for_class(class_name).is_some()
}

pub(super) fn unsupported_scanned_classes(
    scanned_classes: &BTreeSet<String>,
) -> Vec<UnsupportedScannedClass> {
    unsupported_scanned_classes_with_resolver(scanned_classes, class_has_generated_css)
}

pub(super) fn unsupported_scanned_classes_with_resolver<F>(
    scanned_classes: &BTreeSet<String>,
    has_generated_css: F,
) -> Vec<UnsupportedScannedClass>
where
    F: Fn(&str) -> bool,
{
    scanned_classes
        .iter()
        .filter_map(|class_name| {
            unsupported_scanned_class_reason(class_name, has_generated_css(class_name)).map(
                |reason| UnsupportedScannedClass {
                    class_name: class_name.clone(),
                    reason,
                },
            )
        })
        .collect()
}

pub(super) fn source_scan_diagnostic_findings_for_source(
    source_file: &str,
    source: &str,
) -> Vec<SourceScanDiagnosticFinding> {
    style::parser::extract_classes_fast(source.as_bytes(), 64)
        .source_diagnostics
        .into_iter()
        .map(|diagnostic| source_scan_diagnostic_finding(source_file, diagnostic))
        .collect()
}

fn source_scan_diagnostic_finding(
    source_file: &str,
    diagnostic: SourceScanDiagnostic,
) -> SourceScanDiagnosticFinding {
    SourceScanDiagnosticFinding {
        source_file: source_file.to_string(),
        kind: diagnostic.kind.as_str(),
        severity: source_scan_diagnostic_severity(diagnostic.kind),
        token: diagnostic.token,
        line: diagnostic.line,
        column: diagnostic.column,
        reason: diagnostic.message,
    }
}

fn source_scan_diagnostic_severity(kind: SourceScanDiagnosticKind) -> &'static str {
    match kind {
        SourceScanDiagnosticKind::DynamicFragment | SourceScanDiagnosticKind::UnsafeCandidate => {
            "warning"
        }
        SourceScanDiagnosticKind::ObjectKey
        | SourceScanDiagnosticKind::NonUtilityCandidate
        | SourceScanDiagnosticKind::DuplicateCandidate => "info",
    }
}

fn unsupported_scanned_class_reason(
    class_name: &str,
    has_generated_css: bool,
) -> Option<&'static str> {
    if class_name.starts_with("dx-grouping-error:") {
        return Some(
            "grouped classname syntax is invalid; close the parenthesized group or remove the grouped prefix",
        );
    }

    if !has_generated_css && has_rejected_arbitrary_variant(class_name) {
        return Some("unsafe arbitrary variant syntax was rejected by dx-style");
    }

    if !has_generated_css && has_rejected_arbitrary_value(class_name) {
        return Some("unsafe arbitrary value syntax was rejected by dx-style");
    }

    if !has_generated_css && has_dynamic_class_fragment(class_name) {
        return Some(
            "dynamic class fragment was scanned but cannot be generated statically; use complete class strings or @source inline(...) safelist entries",
        );
    }

    if !has_generated_css && tailwind_like_class_requires_generation(class_name) {
        return Some("tailwind-like class was scanned but dx-style did not generate CSS for it");
    }

    None
}

fn has_dynamic_class_fragment(class_name: &str) -> bool {
    class_name.contains("${") || class_name.contains('$')
}

fn has_rejected_arbitrary_variant(class_name: &str) -> bool {
    variant_parts(class_name)
        .into_iter()
        .any(|part| part.starts_with('[') && unsafe_arbitrary_syntax(part))
}

fn has_rejected_arbitrary_value(class_name: &str) -> bool {
    let base = variant_base_class(class_name)
        .trim_start_matches('!')
        .trim_start_matches('-');
    base.contains('[') && unsafe_arbitrary_syntax(base)
}

fn unsafe_arbitrary_syntax(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    value
        .chars()
        .any(|ch| matches!(ch, '{' | '}' | ';' | '"' | '\''))
        || lower.contains("javascript:")
        || lower.contains("expression(")
        || lower.contains("@import")
        || lower.contains("</")
}

fn variant_parts(class_name: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut start = 0usize;

    for (index, byte) in class_name.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b':' if bracket_depth == 0 && paren_depth == 0 => {
                parts.push(&class_name[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }

    parts
}

fn tailwind_like_class_requires_generation(class_name: &str) -> bool {
    let class_name = variant_base_class(class_name)
        .trim_start_matches('!')
        .trim_start_matches('-');

    exact_tailwind_utility_names().contains(&class_name)
        || TAILWIND_UTILITY_PREFIXES
            .iter()
            .any(|prefix| class_name.starts_with(prefix))
}

fn exact_tailwind_utility_names() -> &'static [&'static str] {
    EXACT_TAILWIND_UTILITY_NAMES
}

fn variant_base_class(class_name: &str) -> &str {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut last_colon = None;

    for (index, byte) in class_name.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b':' if bracket_depth == 0 && paren_depth == 0 => last_colon = Some(index),
            _ => {}
        }
    }

    last_colon.map_or(class_name, |index| &class_name[index + 1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_scan_report_only_flags_tailwind_like_utility_tokens() {
        let scanned_classes = BTreeSet::from([
            "card-shell".to_string(),
            "field-sizing-content".to_string(),
            "mask-conic-from-50%".to_string(),
            "mask-l-from-50%".to_string(),
            "mask-alpha".to_string(),
            "mask-clip-padding".to_string(),
            "mask-linear-[70deg,transparent_10%,black,transparent_80%]".to_string(),
            "mask-linear-from-60%".to_string(),
            "mask-origin-content".to_string(),
            "mask-radial-[100%_100%]".to_string(),
            "mask-radial-from-50%".to_string(),
            "mask-type-alpha".to_string(),
            "font-stretch-condensed".to_string(),
            "forced-color-adjust-auto".to_string(),
            "columns-3".to_string(),
            "break-before-page".to_string(),
            "break-inside-avoid".to_string(),
            "box-decoration-clone".to_string(),
            "bg-blend-multiply".to_string(),
            "bg-origin-border".to_string(),
            "bg-none".to_string(),
            "bg-linear-to-r".to_string(),
            "bg-linear-to-r/oklch".to_string(),
            "bg-linear-to-r/longer".to_string(),
            "bg-radial".to_string(),
            "bg-radial/oklch".to_string(),
            "bg-conic/decreasing".to_string(),
            "bg-conic-180".to_string(),
            "bg-conic-180/shorter".to_string(),
            "bg-conic/[in_hsl_longer_hue]".to_string(),
            "bg-linear-45".to_string(),
            "bg-linear-45/srgb".to_string(),
            "bg-[url('/hero.png')]".to_string(),
            "bg-size-(--dx-bg-size)".to_string(),
            "[@starting-style]:opacity-0".to_string(),
            "[@layer_components]:p-4".to_string(),
            "[@media_(any-hover:hover){&:hover}]:opacity-100".to_string(),
            "[@unknown_rule]:p-4".to_string(),
            "md:hover:text-shadow-sm".to_string(),
            "text-shadow-cyan-500/50".to_string(),
            "hover:not-focus:text-shadow-sky-300/50".to_string(),
            "transform-gpu".to_string(),
            "bg-card/50".to_string(),
            "dark:hover:bg-card/40".to_string(),
            "ps-4".to_string(),
            "-ms-2".to_string(),
            "start-0".to_string(),
            "size-8".to_string(),
            "border-s-2".to_string(),
            "rounded-ee-xl".to_string(),
            "object-[25%_75%]".to_string(),
            "object-(--dx-object-position)".to_string(),
            "origin-top-left".to_string(),
            "skew-x-6".to_string(),
            "-skew-y-3".to_string(),
            "flex-[2_1_0%]".to_string(),
            "grow-[2]".to_string(),
            "shrink-[3]".to_string(),
            "-order-1".to_string(),
            "grid-cols-(--dx-grid-cols)".to_string(),
            "auto-rows-(--dx-auto-rows)".to_string(),
            "drop-shadow-md".to_string(),
            "backdrop-opacity-50".to_string(),
            "backdrop-invert".to_string(),
            "scroll-ms-4".to_string(),
            "scroll-pe-2".to_string(),
            "table-auto".to_string(),
            "table-fixed".to_string(),
            "caption-bottom".to_string(),
            "border-collapse".to_string(),
            "border-spacing-2".to_string(),
            "border-spacing-x-4".to_string(),
            "transform-3d".to_string(),
            "perspective-dramatic".to_string(),
            "perspective-[750px]".to_string(),
            "perspective-origin-top-right".to_string(),
            "transform-(--dx-transform)".to_string(),
            "rotate-x-45".to_string(),
            "-rotate-y-12".to_string(),
            "translate-z-4".to_string(),
            "scale-z-125".to_string(),
            "scale-z-(--dx-scale-z)".to_string(),
            "-scale-z-125".to_string(),
            "inset-shadow-[inset_0_1px_2px_rgb(0_0_0_/_0.1)]".to_string(),
            "shadow-(--dx-shadow)".to_string(),
            "box-border".to_string(),
            "box-content".to_string(),
            "bg-[#1d4ed8]/50".to_string(),
            "bg-(--dx-background)".to_string(),
            "border-(--dx-border)".to_string(),
            "fill-[#0f172a]/80".to_string(),
            "text-[color:var(--dx-foreground)]".to_string(),
            "stroke-[color:var(--dx-stroke)]".to_string(),
            "rounded-(--dx-radius)".to_string(),
            "aspect-(--dx-aspect)".to_string(),
            "leading-(--dx-leading)".to_string(),
            "tracking-(--dx-tracking)".to_string(),
            "items-baseline-last".to_string(),
            "self-baseline".to_string(),
            "place-items-baseline".to_string(),
            "justify-items-center".to_string(),
            "justify-self-end".to_string(),
            "content-normal".to_string(),
            "from-10%".to_string(),
            "via-30%".to_string(),
            "to-90%".to_string(),
            "from-(--dx-gradient-from-position)".to_string(),
            "ring-[#2563eb]/50".to_string(),
            "divide-(--dx-border)".to_string(),
            "p-(--dx-pad)".to_string(),
            "scroll-mt-(--dx-scroll-mt)".to_string(),
            "opacity-(--dx-opacity)".to_string(),
            "z-(--dx-layer)".to_string(),
            "order-(--dx-order)".to_string(),
            "blur-(--dx-blur)".to_string(),
            "brightness-(--dx-brightness)".to_string(),
            "hue-rotate-(--dx-hue-rotate)".to_string(),
            "backdrop-opacity-(--dx-backdrop-opacity)".to_string(),
            "backdrop-blur-(--dx-backdrop-blur)".to_string(),
            "outline-offset-(--dx-outline-offset)".to_string(),
            "outline-[3px]".to_string(),
            "ring-[3px]".to_string(),
            "ring-offset-(--dx-ring-offset-width)".to_string(),
            "prose".to_string(),
        ]);

        let unsupported = unsupported_scanned_classes(&scanned_classes)
            .into_iter()
            .map(|item| item.class_name)
            .collect::<Vec<_>>();

        assert!(unsupported.is_empty(), "{unsupported:?}");
    }

    #[test]
    fn source_scan_diagnostic_receipts_include_locations() {
        let source = r#"
const sizes = ["px-4", "px-4"];
const tone = `text-${tone}-600`;
const map = { "ignored": "grid-cols-2" };
const copy = "card shell copy";
const unsafe_url = "https://example.com";
"#;

        let findings = source_scan_diagnostic_findings_for_source("app/page.tsx", source);

        for expected in [
            SourceScanDiagnosticKind::DynamicFragment.as_str(),
            SourceScanDiagnosticKind::ObjectKey.as_str(),
            SourceScanDiagnosticKind::UnsafeCandidate.as_str(),
            SourceScanDiagnosticKind::NonUtilityCandidate.as_str(),
            SourceScanDiagnosticKind::DuplicateCandidate.as_str(),
        ] {
            assert!(
                findings.iter().any(|finding| finding.kind == expected
                    && finding.source_file == "app/page.tsx"
                    && finding.line > 0
                    && finding.column > 0),
                "expected located source scan diagnostic {expected}: {findings:?}"
            );
        }

        assert!(findings.iter().any(|finding| {
            finding.kind == SourceScanDiagnosticKind::DynamicFragment.as_str()
                && finding.severity == "warning"
                && finding.token.contains("text-${tone}-600")
        }));
        assert!(findings.iter().any(|finding| {
            finding.kind == SourceScanDiagnosticKind::DuplicateCandidate.as_str()
                && finding.severity == "info"
                && finding.token == "px-4"
        }));
    }

    #[test]
    fn unsupported_scan_reports_invalid_grouped_classnames() {
        let scanned_classes = BTreeSet::from(["dx-grouping-error:unclosed-group:md".to_string()]);

        let unsupported = unsupported_scanned_classes(&scanned_classes);

        assert_eq!(
            unsupported,
            vec![UnsupportedScannedClass {
                class_name: "dx-grouping-error:unclosed-group:md".to_string(),
                reason: "grouped classname syntax is invalid; close the parenthesized group or remove the grouped prefix",
            }]
        );
    }

    #[test]
    fn unsupported_scan_reports_specific_unsafe_arbitrary_syntax() {
        let scanned_classes = BTreeSet::from([
            "[&{color:red}]:p-4".to_string(),
            "bg-[url(javascript:alert(1))]".to_string(),
        ]);

        let unsupported = unsupported_scanned_classes(&scanned_classes);

        assert_eq!(
            unsupported,
            vec![
                UnsupportedScannedClass {
                    class_name: "[&{color:red}]:p-4".to_string(),
                    reason: "unsafe arbitrary variant syntax was rejected by dx-style",
                },
                UnsupportedScannedClass {
                    class_name: "bg-[url(javascript:alert(1))]".to_string(),
                    reason: "unsafe arbitrary value syntax was rejected by dx-style",
                },
            ]
        );
    }

    #[test]
    fn unsupported_scan_keeps_text_shadow_color_utilities_honest() {
        let scanned_classes = BTreeSet::from([
            "text-shadow-cyan-500/50".to_string(),
            "text-shadow-unknown-999".to_string(),
        ]);

        let unsupported = unsupported_scanned_classes(&scanned_classes);

        assert_eq!(
            unsupported,
            vec![UnsupportedScannedClass {
                class_name: "text-shadow-unknown-999".to_string(),
                reason: "tailwind-like class was scanned but dx-style did not generate CSS for it",
            }]
        );
    }

    #[test]
    fn unsupported_scan_reports_dynamic_class_fragments_as_source_boundaries() {
        let scanned_classes = BTreeSet::from([
            "bg-${color}-600".to_string(),
            "hover:${state}:opacity-100".to_string(),
            "p-${size}".to_string(),
            "bg-blurple-999".to_string(),
        ]);

        let unsupported = unsupported_scanned_classes(&scanned_classes);

        assert_eq!(
            unsupported,
            vec![
                UnsupportedScannedClass {
                    class_name: "bg-${color}-600".to_string(),
                    reason: "dynamic class fragment was scanned but cannot be generated statically; use complete class strings or @source inline(...) safelist entries",
                },
                UnsupportedScannedClass {
                    class_name: "bg-blurple-999".to_string(),
                    reason: "tailwind-like class was scanned but dx-style did not generate CSS for it",
                },
                UnsupportedScannedClass {
                    class_name: "hover:${state}:opacity-100".to_string(),
                    reason: "dynamic class fragment was scanned but cannot be generated statically; use complete class strings or @source inline(...) safelist entries",
                },
                UnsupportedScannedClass {
                    class_name: "p-${size}".to_string(),
                    reason: "dynamic class fragment was scanned but cannot be generated statically; use complete class strings or @source inline(...) safelist entries",
                },
            ]
        );
    }

    #[test]
    fn unsupported_scan_keeps_gradient_interpolation_modifiers_honest() {
        let scanned_classes = BTreeSet::from([
            "bg-linear-to-r/oklch".to_string(),
            "bg-conic/decreasing".to_string(),
            "bg-conic/[in_hsl_longer_hue]".to_string(),
            "bg-linear-to-r/banana".to_string(),
            "bg-conic/[bad;value]".to_string(),
        ]);

        let unsupported = unsupported_scanned_classes(&scanned_classes);

        assert_eq!(
            unsupported,
            vec![
                UnsupportedScannedClass {
                    class_name: "bg-conic/[bad;value]".to_string(),
                    reason: "unsafe arbitrary value syntax was rejected by dx-style",
                },
                UnsupportedScannedClass {
                    class_name: "bg-linear-to-r/banana".to_string(),
                    reason: "tailwind-like class was scanned but dx-style did not generate CSS for it",
                },
            ]
        );
    }

    #[test]
    fn unsupported_scan_keeps_three_d_transform_utilities_honest() {
        let scanned_classes = BTreeSet::from([
            "perspective-dramatic".to_string(),
            "transform-(--dx-transform)".to_string(),
            "scale-z-(--dx-scale-z)".to_string(),
            "-scale-z-125".to_string(),
            "perspective-impossible".to_string(),
            "transform-[rotate(45deg);color:red]".to_string(),
        ]);

        let unsupported = unsupported_scanned_classes(&scanned_classes);

        assert_eq!(
            unsupported,
            vec![
                UnsupportedScannedClass {
                    class_name: "perspective-impossible".to_string(),
                    reason: "tailwind-like class was scanned but dx-style did not generate CSS for it",
                },
                UnsupportedScannedClass {
                    class_name: "transform-[rotate(45deg);color:red]".to_string(),
                    reason: "unsafe arbitrary value syntax was rejected by dx-style",
                },
            ]
        );
    }
}
