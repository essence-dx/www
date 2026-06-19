use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeStaticPagePreviewArtifact {
    pub(super) route: String,
    pub(super) path: PathBuf,
    pub(super) project_relative_path: String,
    pub(super) html_path: PathBuf,
    pub(super) json_path: PathBuf,
    pub(super) write_state: String,
    pub(super) migrated_route: String,
    pub(super) title: String,
    pub(super) status: String,
    pub(super) score: u8,
    pub(super) links: Vec<DxForgeStaticPagePreviewLink>,
    pub(super) manual_review_warning_count: u64,
    pub(super) manual_review_warnings: Vec<String>,
    pub(super) unsafe_html_status: String,
    pub(super) unsafe_html_decision: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeStaticPagePreviewLink {
    pub(super) kind: String,
    pub(super) label: String,
    pub(super) href: String,
    pub(super) description: String,
}

pub(super) struct DxForgeStaticPagePreviewInput {
    pub(super) project: PathBuf,
    pub(super) generated_dir: PathBuf,
    pub(super) slug: String,
    pub(super) title: String,
    pub(super) migrated_route: String,
    pub(super) status: String,
    pub(super) score: u8,
    pub(super) write_state: String,
    pub(super) links: Vec<DxForgeStaticPagePreviewLink>,
    pub(super) manual_review_warnings: Vec<String>,
    pub(super) unsafe_html_status: String,
    pub(super) unsafe_html_decision: Option<String>,
}

pub(super) fn preview_link(
    kind: &str,
    label: &str,
    href: &str,
    description: &str,
) -> DxForgeStaticPagePreviewLink {
    DxForgeStaticPagePreviewLink {
        kind: kind.to_string(),
        label: label.to_string(),
        href: href.to_string(),
        description: description.to_string(),
    }
}

pub(super) fn build_static_page_preview_artifact(
    input: DxForgeStaticPagePreviewInput,
) -> DxForgeStaticPagePreviewArtifact {
    let html_path = input.generated_dir.join("preview/index.html");
    let json_path = input.generated_dir.join("preview/preview.json");
    let route = format!("/forge/migrated-route-preview/{}/", input.slug);
    DxForgeStaticPagePreviewArtifact {
        route,
        path: html_path.clone(),
        project_relative_path: project_relative_path(&input.project, &html_path),
        html_path,
        json_path,
        write_state: input.write_state,
        migrated_route: input.migrated_route,
        title: input.title,
        status: input.status,
        score: input.score,
        links: input.links,
        manual_review_warning_count: input.manual_review_warnings.len() as u64,
        manual_review_warnings: input.manual_review_warnings,
        unsafe_html_status: input.unsafe_html_status,
        unsafe_html_decision: input.unsafe_html_decision,
    }
}

pub(super) fn static_page_preview_json(
    artifact: &DxForgeStaticPagePreviewArtifact,
) -> anyhow::Result<String> {
    Ok(serde_json::to_string_pretty(artifact)?)
}

pub(super) fn static_page_preview_html(artifact: &DxForgeStaticPagePreviewArtifact) -> String {
    let links = artifact
        .links
        .iter()
        .map(|link| {
            format!(
                r#"<li><a href="{href}">{label}</a><span>{kind}</span><p>{description}</p></li>"#,
                href = html_escape(&link.href),
                label = html_escape(&link.label),
                kind = html_escape(&link.kind),
                description = html_escape(&link.description)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let warnings = artifact
        .manual_review_warnings
        .iter()
        .map(|warning| format!("<li>{}</li>", html_escape(warning)))
        .collect::<Vec<_>>()
        .join("\n");
    let decision = artifact
        .unsafe_html_decision
        .as_deref()
        .map(|value| format!("<p><strong>Decision:</strong> {}</p>", html_escape(value)))
        .unwrap_or_else(|| "<p><strong>Decision:</strong> not recorded</p>".to_string());

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>DX Forge Migrated Route Preview - {title}</title>
  <style>
    :root {{ color-scheme: light; font-family: Arial, sans-serif; }}
    body {{ margin: 0; background: #f7f7f5; color: #171717; }}
    main {{ max-width: 960px; margin: 0 auto; padding: 32px 20px 48px; }}
    header, section {{ background: #fff; border: 1px solid #ddd8cc; border-radius: 8px; padding: 20px; margin-bottom: 16px; }}
    h1, h2 {{ margin: 0 0 12px; }}
    ul {{ padding-left: 20px; }}
    li {{ margin: 10px 0; }}
    a {{ color: #005f73; font-weight: 700; }}
    span {{ display: inline-block; margin-left: 8px; color: #5f5f5f; font-size: 12px; text-transform: uppercase; }}
    code {{ background: #f0eee8; padding: 2px 5px; border-radius: 4px; }}
  </style>
</head>
<body>
  <main>
    <header>
      <p>DX Forge Migrated Route Preview</p>
      <h1>{title}</h1>
      <p>Route <code>{route}</code> is staged for public beta review with source, audit, assets, benchmark fixture, and manual-review evidence linked below.</p>
      <p>Status: <strong>{status}</strong> | Score: <strong>{score}/100</strong> | Unsafe HTML: <strong>{unsafe_status}</strong></p>
      {decision}
    </header>
    <section>
      <h2>Reviewer Links</h2>
      <ul>
        {links}
      </ul>
    </section>
    <section id="manual-review">
      <h2>Manual Review Warnings</h2>
      <ul>
        {warnings}
      </ul>
    </section>
  </main>
</body>
</html>
"#,
        title = html_escape(&artifact.title),
        route = html_escape(&artifact.migrated_route),
        status = html_escape(&artifact.status),
        score = artifact.score,
        unsafe_status = html_escape(&artifact.unsafe_html_status),
        decision = decision,
        links = links,
        warnings = warnings
    )
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn project_relative_path(project: &Path, path: &Path) -> String {
    path.strip_prefix(project)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
