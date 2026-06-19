use super::{css_data, protocol::DEVTOOLS_ROOT};

pub(super) const DX_DEVTOOLS_RUNTIME_PATH: &str = "/_dx/devtools/runtime.js";
pub(super) const DX_DEVTOOLS_CSS_PATH: &str = "/_dx/devtools/devtools.css";
pub(super) const DX_DEVTOOLS_CSS_DATA_PATH: &str = "/_dx/devtools/css-data";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DxDevtoolsAsset {
    pub(super) content_type: &'static str,
    pub(super) body: &'static str,
}

pub(super) fn devtools_injection_tags() -> &'static str {
    r#"<link rel="stylesheet" href="/_dx/devtools/devtools.css" data-dx-devtools-style>
<script type="module" src="/_dx/devtools/runtime.js" data-dx-devtools-runtime></script>"#
}

pub(super) fn asset(request_path: &str) -> Option<DxDevtoolsAsset> {
    match request_path.split('?').next().unwrap_or(request_path) {
        DX_DEVTOOLS_RUNTIME_PATH => Some(DxDevtoolsAsset {
            content_type: "text/javascript; charset=utf-8",
            body: RUNTIME_JS,
        }),
        DX_DEVTOOLS_CSS_PATH => Some(DxDevtoolsAsset {
            content_type: "text/css; charset=utf-8",
            body: DEVTOOLS_CSS,
        }),
        DX_DEVTOOLS_CSS_DATA_PATH => Some(DxDevtoolsAsset {
            content_type: "application/json; charset=utf-8",
            body: css_data::CSS_DATA_JSON,
        }),
        DEVTOOLS_ROOT => Some(DxDevtoolsAsset {
            content_type: "text/html; charset=utf-8",
            body: DEVTOOLS_INDEX,
        }),
        _ => None,
    }
}

const DEVTOOLS_INDEX: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>DX Devtools</title>
  <link rel="stylesheet" href="/_dx/devtools/devtools.css">
</head>
<body>
  <main class="dx-devtools-standalone">
    <h1>DX Devtools</h1>
    <p>Open a DX WWW page during dx dev. The framework injects this runtime automatically.</p>
  </main>
  <script type="module" src="/_dx/devtools/runtime.js"></script>
</body>
</html>"#;

const RUNTIME_JS: &str = concat!(
    include_str!("assets/runtime/part-01-boot.ts"),
    include_str!("assets/runtime/part-02-protocol.ts"),
    include_str!("assets/runtime/part-03-controls.ts"),
    include_str!("assets/runtime/part-04-render.ts"),
    include_str!("assets/runtime/part-05-events.ts"),
);

const DEVTOOLS_CSS: &str = include_str!("assets/devtools.css");
