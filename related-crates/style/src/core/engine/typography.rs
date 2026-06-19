//! Tailwind typography-compatible baseline utilities.
//!
//! This is intentionally a small first-party subset, not a clone of the
//! Tailwind Typography plugin configuration API.

pub fn generate_typography_css(class_name: &str) -> Option<String> {
    match class_name {
        "prose" => Some(prose_baseline()),
        "prose-invert" => Some(prose_invert()),
        _ => prose_size(class_name),
    }
}

fn prose_baseline() -> String {
    [
        "BASE|color: var(--tw-prose-body, rgb(55 65 81)); max-width: 65ch".to_string(),
        typography_nested_rule(" :where(p)", "margin-top: 1.25em; margin-bottom: 1.25em"),
        typography_nested_rule(
            " :where(h1)",
            "color: var(--tw-prose-headings, rgb(17 24 39)); font-size: 2.25em; line-height: 1.1111111; font-weight: 800; margin-top: 0; margin-bottom: 0.8888889em",
        ),
        typography_nested_rule(
            " :where(h2)",
            "color: var(--tw-prose-headings, rgb(17 24 39)); font-size: 1.5em; line-height: 1.3333333; font-weight: 700; margin-top: 2em; margin-bottom: 1em",
        ),
        typography_nested_rule(
            " :where(h3)",
            "color: var(--tw-prose-headings, rgb(17 24 39)); font-size: 1.25em; line-height: 1.6; font-weight: 600; margin-top: 1.6em; margin-bottom: 0.6em",
        ),
        typography_nested_rule(
            " :where(a)",
            "color: var(--tw-prose-links, rgb(37 99 235)); text-decoration: underline; font-weight: 500",
        ),
        typography_nested_rule(
            " :where(strong)",
            "color: var(--tw-prose-bold, rgb(17 24 39)); font-weight: 600",
        ),
        typography_nested_rule(
            " :where(code)",
            "color: var(--tw-prose-code, rgb(17 24 39)); font-weight: 600; font-size: 0.875em",
        ),
        typography_nested_rule(
            " :where(pre)",
            "color: var(--tw-prose-pre-code, rgb(229 231 235)); background-color: var(--tw-prose-pre-bg, rgb(31 41 55)); overflow-x: auto; font-size: 0.875em; line-height: 1.7142857; margin-top: 1.7142857em; margin-bottom: 1.7142857em; border-radius: 0.375rem; padding: 0.8571429em 1.1428571em",
        ),
        typography_nested_rule(
            " :where(blockquote)",
            "font-weight: 500; font-style: italic; color: var(--tw-prose-quotes, rgb(17 24 39)); border-inline-start-width: 0.25rem; border-inline-start-color: var(--tw-prose-quote-borders, rgb(209 213 219)); quotes: \"\\201C\" \"\\201D\" \"\\2018\" \"\\2019\"; margin-top: 1.6em; margin-bottom: 1.6em; padding-inline-start: 1em",
        ),
        typography_nested_rule(
            " :where(ul)",
            "list-style-type: disc; margin-top: 1.25em; margin-bottom: 1.25em; padding-inline-start: 1.625em",
        ),
        typography_nested_rule(
            " :where(ol)",
            "list-style-type: decimal; margin-top: 1.25em; margin-bottom: 1.25em; padding-inline-start: 1.625em",
        ),
        typography_nested_rule(" :where(li)", "margin-top: 0.5em; margin-bottom: 0.5em"),
        typography_nested_rule(
            " :where(hr)",
            "border-color: var(--tw-prose-hr, rgb(229 231 235)); border-top-width: 1px; margin-top: 3em; margin-bottom: 3em",
        ),
        typography_nested_rule(
            " :where(table)",
            "width: 100%; table-layout: auto; text-align: start; margin-top: 2em; margin-bottom: 2em; font-size: 0.875em; line-height: 1.7142857",
        ),
        typography_nested_rule(
            " :where(th)",
            "color: var(--tw-prose-headings, rgb(17 24 39)); font-weight: 600; border-bottom-width: 1px; border-bottom-color: var(--tw-prose-th-borders, rgb(209 213 219)); padding: 0.5714286em",
        ),
        typography_nested_rule(
            " :where(td)",
            "border-bottom-width: 1px; border-bottom-color: var(--tw-prose-td-borders, rgb(229 231 235)); padding: 0.5714286em",
        ),
        typography_nested_rule(
            " :where(img)",
            "margin-top: 2em; margin-bottom: 2em; max-width: 100%; height: auto",
        ),
    ]
    .join("\n")
}

fn prose_invert() -> String {
    [
        "BASE|--tw-prose-body: rgb(209 213 219); --tw-prose-headings: rgb(255 255 255); --tw-prose-links: rgb(147 197 253); --tw-prose-bold: rgb(255 255 255); --tw-prose-code: rgb(255 255 255); --tw-prose-quotes: rgb(243 244 246); --tw-prose-quote-borders: rgb(75 85 99); --tw-prose-hr: rgb(55 65 81); --tw-prose-pre-code: rgb(229 231 235); --tw-prose-pre-bg: rgb(17 24 39); --tw-prose-th-borders: rgb(75 85 99); --tw-prose-td-borders: rgb(55 65 81)",
    ]
    .join("\n")
}

fn prose_size(class_name: &str) -> Option<String> {
    let (font_size, line_height, p_margin, h1_size) = match class_name {
        "prose-sm" => ("0.875rem", "1.7142857", "1.1428571em", "2.1428571em"),
        "prose-lg" => ("1.125rem", "1.7777778", "1.3333333em", "2.6666667em"),
        "prose-xl" => ("1.25rem", "1.8", "1.2em", "2.8em"),
        "prose-2xl" => ("1.5rem", "1.6666667", "1.3333333em", "2.6666667em"),
        _ => return None,
    };

    Some(
        [
            format!("BASE|font-size: {font_size}; line-height: {line_height}"),
            typography_nested_rule(
                " :where(p)",
                &format!("margin-top: {p_margin}; margin-bottom: {p_margin}"),
            ),
            typography_nested_rule(
                " :where(h1)",
                &format!("font-size: {h1_size}; line-height: 1; margin-bottom: 0.85em"),
            ),
        ]
        .join("\n"),
    )
}

fn typography_nested_rule(selector_suffix: &str, declarations: &str) -> String {
    format!("NEST|{selector_suffix}|{declarations}")
}

pub fn typography_element_selector(part: &str) -> Option<&'static str> {
    Some(match part {
        "prose-p" => " :where(p)",
        "prose-lead" => " :where(.lead)",
        "prose-a" => " :where(a)",
        "prose-blockquote" => " :where(blockquote)",
        "prose-figure" => " :where(figure)",
        "prose-figcaption" => " :where(figcaption)",
        "prose-strong" => " :where(strong)",
        "prose-em" => " :where(em)",
        "prose-kbd" => " :where(kbd)",
        "prose-code" => " :where(code)",
        "prose-pre" => " :where(pre)",
        "prose-ol" => " :where(ol)",
        "prose-ul" => " :where(ul)",
        "prose-li" => " :where(li)",
        "prose-table" => " :where(table)",
        "prose-thead" => " :where(thead)",
        "prose-tr" => " :where(tr)",
        "prose-th" => " :where(th)",
        "prose-td" => " :where(td)",
        "prose-img" => " :where(img)",
        "prose-video" => " :where(video)",
        "prose-hr" => " :where(hr)",
        "prose-h1" => " :where(h1)",
        "prose-h2" => " :where(h2)",
        "prose-h3" => " :where(h3)",
        "prose-h4" => " :where(h4)",
        "prose-headings" => " :where(h1, h2, h3, h4)",
        _ => return None,
    })
}
