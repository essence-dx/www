use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeStaticPageUnsafeHtmlPolicy {
    pub(super) status: String,
    pub(super) blocked: bool,
    pub(super) decision: Option<String>,
    pub(super) review_count: u64,
    pub(super) reviews: Vec<DxForgeStaticPageUnsafeHtmlReview>,
    pub(super) required_decision_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeStaticPageUnsafeHtmlReview {
    pub(super) code: String,
    pub(super) severity: String,
    pub(super) evidence: String,
    pub(super) action: String,
}

pub(super) fn build_static_page_unsafe_html_policy(
    text: &str,
    manual_review_decision: Option<&str>,
) -> DxForgeStaticPageUnsafeHtmlPolicy {
    let reviews = detect_static_page_unsafe_html_reviews(text);
    let decision = manual_review_decision
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let blocked = !reviews.is_empty() && decision.is_none();
    let status = if blocked {
        "blocked"
    } else if !reviews.is_empty() {
        "manual-review-recorded"
    } else {
        "clear"
    }
    .to_string();
    let required_decision_hint = blocked.then(|| {
        "Rerun with --unsafe-html-review <decision> after reviewing scripts, events, embeds, forms, and shortcodes."
            .to_string()
    });

    DxForgeStaticPageUnsafeHtmlPolicy {
        status,
        blocked,
        decision,
        review_count: reviews.len() as u64,
        reviews,
        required_decision_hint,
    }
}

pub(super) fn unsafe_html_manual_review_items(
    policy: &DxForgeStaticPageUnsafeHtmlPolicy,
) -> Vec<String> {
    let mut items = policy
        .reviews
        .iter()
        .map(|review| review.action.clone())
        .collect::<Vec<_>>();
    if let Some(decision) = &policy.decision {
        items.push(format!("Unsafe HTML manual review decision: {decision}"));
    }
    items
}

fn detect_static_page_unsafe_html_reviews(text: &str) -> Vec<DxForgeStaticPageUnsafeHtmlReview> {
    let mut reviews = Vec::new();
    push_review_if_match(
        &mut reviews,
        text,
        "script-tag",
        "high",
        r#"(?is)<script\b[^>]*>.*?</script>|<script\b[^>]*>"#,
        "Remove or replace script tags with source-owned application code before production.",
    );
    push_review_if_match(
        &mut reviews,
        text,
        "inline-event-handler",
        "high",
        r#"(?is)\son[a-z]+\s*="#,
        "Replace inline event handlers with reviewed component logic.",
    );
    push_review_if_match(
        &mut reviews,
        text,
        "embed-tag",
        "medium",
        r#"(?is)<(iframe|embed|object)\b[^>]*>"#,
        "Allowlist or replace embedded content before production.",
    );
    push_review_if_match(
        &mut reviews,
        text,
        "legacy-form",
        "high",
        r#"(?is)<form\b[^>]*>"#,
        "Replace legacy forms with a real application-owned handler.",
    );
    push_review_if_match(
        &mut reviews,
        text,
        "wordpress-shortcode-leftover",
        "medium",
        r#"(?is)\[[a-z][a-z0-9_-]+[^\]]*\]"#,
        "Replace shortcode leftovers with source-owned page behavior.",
    );
    reviews
}

fn push_review_if_match(
    reviews: &mut Vec<DxForgeStaticPageUnsafeHtmlReview>,
    text: &str,
    code: &str,
    severity: &str,
    pattern: &str,
    action: &str,
) {
    let Ok(regex) = Regex::new(pattern) else {
        return;
    };
    let Some(matched) = regex.find(text) else {
        return;
    };
    reviews.push(DxForgeStaticPageUnsafeHtmlReview {
        code: code.to_string(),
        severity: severity.to_string(),
        evidence: compact_evidence(matched.as_str()),
        action: action.to_string(),
    });
}

fn compact_evidence(value: &str) -> String {
    let trimmed = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if trimmed.len() > 120 {
        format!("{}...", &trimmed[..120])
    } else {
        trimmed
    }
}
