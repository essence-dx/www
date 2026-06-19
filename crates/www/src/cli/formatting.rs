pub(super) fn markdown_table_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

pub(super) fn optional_string(value: Option<&str>) -> String {
    value.unwrap_or("n/a").to_string()
}

pub(super) fn optional_u64(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "n/a".to_string())
}

pub(super) fn optional_f64(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3}"))
        .unwrap_or_else(|| "n/a".to_string())
}

pub(super) fn count_substrings(value: &str, needle: &str) -> usize {
    value.match_indices(needle).count()
}

pub(super) fn html_href_values(html: &str) -> Vec<String> {
    let mut hrefs = Vec::new();
    let mut rest = html;
    while let Some(index) = rest.find("href=\"") {
        rest = &rest[index + "href=\"".len()..];
        let Some(end) = rest.find('"') else {
            break;
        };
        hrefs.push(rest[..end].to_string());
        rest = &rest[end + 1..];
    }
    hrefs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_table_cell_escapes_table_breakers() {
        assert_eq!(
            markdown_table_cell("alpha|beta\ngamma"),
            "alpha\\|beta gamma"
        );
    }

    #[test]
    fn optional_values_render_stable_cli_placeholders() {
        assert_eq!(optional_string(Some("ready")), "ready");
        assert_eq!(optional_string(None), "n/a");
        assert_eq!(optional_u64(Some(42)), "42");
        assert_eq!(optional_u64(None), "n/a");
        assert_eq!(optional_f64(Some(12.34567)), "12.346");
        assert_eq!(optional_f64(None), "n/a");
    }

    #[test]
    fn html_href_values_preserves_existing_double_quote_behavior() {
        let html = r#"<a href="/one">one</a><a data-x="y" href="/two?x=1">two</a>"#;
        assert_eq!(html_href_values(html), vec!["/one", "/two?x=1"]);
    }

    #[test]
    fn count_substrings_uses_non_overlapping_matches() {
        assert_eq!(count_substrings("h1 h1 h2", "h1"), 2);
        assert_eq!(count_substrings("aaaa", "aa"), 2);
    }
}
