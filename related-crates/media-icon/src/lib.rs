//! DX media-icon cache and search components
//!
//! This crate contains local, receipt-backed media-icon search and machine-cache
//! work for DX. JSON icon packs remain the source of truth, generated `.machine`
//! caches are advisory, and a same-machine upstream baseline has not been measured yet.

#![recursion_limit = "512"]

pub mod avx_search;
pub mod bloom;
pub mod builder;
pub mod engine;
pub mod machine_catalog;
pub mod machine_manifest;
pub mod machine_pack_body;
pub mod machine_precomputed;
pub mod machine_readiness;
// GPU module commented out - not useful for most use cases
// #[cfg(feature = "gpu")]
// pub mod gpu;
pub mod index;
pub mod multipattern;
pub mod optimized;
pub mod parser;
pub mod perfect_hash;
pub mod precomputed;
pub mod search;
pub mod types;
pub mod zero_alloc;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use engine::IconSearchEngine;
pub use search::SearchResult;
pub use types::{IconMetadata, IconPack};

/// Return a compatibility icon reader for DX-WWW build/compiler integrations.
///
/// The historical DX-WWW crates expected a tiny `dx_icon::icons().get(set, name)`
/// API. The newer icon search crate is index-driven, so this adapter preserves
/// that compile-time contract until the indexed icon database is connected to
/// this compatibility API.
pub fn icons() -> IconReader {
    IconReader
}

/// Compatibility reader for DX-WWW icon resolution.
pub struct IconReader;

impl IconReader {
    /// Resolve an icon by set and name.
    pub fn get(&mut self, set: &str, name: &str) -> Option<ResolvedIcon> {
        let body = compat_icon_body(set, name)?;
        Some(ResolvedIcon {
            set: set.to_string(),
            name: name.to_string(),
            width: Some(24),
            height: Some(24),
            body: body.to_string(),
        })
    }
}

/// Resolved icon data used by the DX-WWW compatibility API.
pub struct ResolvedIcon {
    /// Icon set name.
    pub set: String,
    /// Icon name within the set.
    pub name: String,
    /// Icon width.
    pub width: Option<u32>,
    /// Icon height.
    pub height: Option<u32>,
    body: String,
}

impl ResolvedIcon {
    /// Render the icon as SVG at a target size.
    pub fn to_svg(&self, size: u32) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 {} {}" aria-hidden="true">{}</svg>"#,
            self.width.unwrap_or(24),
            self.height.unwrap_or(24),
            self.body
        )
    }
}

fn compat_icon_body(set: &str, name: &str) -> Option<&'static str> {
    match (set, name) {
        ("whiteboard", "align") => Some(
            r#"<path d="M12 3v18"/><rect width="6" height="4" x="3" y="5" rx="1"/><rect width="6" height="4" x="15" y="15" rx="1"/>"#,
        ),
        ("whiteboard", "arrow") => Some(r#"<path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>"#),
        ("whiteboard", "audio") => Some(
            r#"<path d="M11 5 6 9H2v6h4l5 4Z"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/>"#,
        ),
        ("whiteboard", "copy") => Some(
            r#"<rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#,
        ),
        ("whiteboard", "diamond") => Some(
            r#"<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.4l7.6 7.6a2.41 2.41 0 0 0 3.4 0l7.6-7.6a2.41 2.41 0 0 0 0-3.4l-7.6-7.6a2.41 2.41 0 0 0-3.4 0Z"/>"#,
        ),
        ("whiteboard", "download") => Some(
            r#"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="M7 10l5 5 5-5"/><path d="M12 15V3"/>"#,
        ),
        ("whiteboard", "ellipse") => Some(r#"<ellipse cx="12" cy="12" rx="9" ry="6"/>"#),
        ("whiteboard", "eraser") => Some(
            r#"<path d="m7 21-4-4 9.5-9.5 4 4L7 21Z"/><path d="m14.5 5.5 2-2a2.12 2.12 0 0 1 3 3l-2 2"/><path d="M12 21h9"/>"#,
        ),
        ("whiteboard", "eye") => Some(
            r#"<path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12Z"/><circle cx="12" cy="12" r="3"/>"#,
        ),
        ("whiteboard", "eye-off") => Some(
            r#"<path d="m3 3 18 18"/><path d="M10.6 10.6A3 3 0 0 0 14 14"/><path d="M9.9 5.2A9.8 9.8 0 0 1 12 5c6.5 0 10 7 10 7a18.6 18.6 0 0 1-2.2 3.2"/><path d="M6.1 6.1C3.4 7.9 2 12 2 12s3.5 7 10 7a9.7 9.7 0 0 0 4.1-.9"/>"#,
        ),
        ("whiteboard", "fit") => Some(
            r#"<path d="M8 3H5a2 2 0 0 0-2 2v3"/><path d="M21 8V5a2 2 0 0 0-2-2h-3"/><path d="M3 16v3a2 2 0 0 0 2 2h3"/><path d="M16 21h3a2 2 0 0 0 2-2v-3"/>"#,
        ),
        ("whiteboard", "grid") => Some(
            r#"<path d="M3 3h18v18H3z"/><path d="M3 9h18"/><path d="M3 15h18"/><path d="M9 3v18"/><path d="M15 3v18"/>"#,
        ),
        ("whiteboard", "hand") => Some(
            r#"<path d="M18 11V6a2 2 0 0 0-4 0"/><path d="M14 10V4a2 2 0 0 0-4 0v2"/><path d="M10 10.5V6a2 2 0 0 0-4 0v8"/><path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.8-1-6-3l-2.7-4.5a2 2 0 0 1 3.4-2L8 14"/>"#,
        ),
        ("whiteboard", "icons") => Some(
            r#"<path d="M8.3 10a.7.7 0 0 1-.63-1.08L11.4 3.5a.7.7 0 0 1 1.25.03l3.58 5.42a.7.7 0 0 1-.59 1.05Z"/><rect width="7" height="7" x="3" y="14" rx="1"/><circle cx="17.5" cy="17.5" r="3.5"/>"#,
        ),
        ("whiteboard", "image") => Some(
            r#"<rect width="18" height="18" x="3" y="3" rx="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.1-3.1a2 2 0 0 0-2.8 0L6 21"/>"#,
        ),
        ("whiteboard", "keyboard") => Some(
            r#"<rect width="20" height="16" x="2" y="4" rx="2"/><path d="M6 8h.01"/><path d="M10 8h.01"/><path d="M14 8h.01"/><path d="M18 8h.01"/><path d="M8 12h.01"/><path d="M12 12h.01"/><path d="M16 12h.01"/><path d="M7 16h10"/>"#,
        ),
        ("whiteboard", "library") => Some(
            r#"<path d="M4 19.5V5a2 2 0 0 1 2-2h14v18H6a2 2 0 0 1-2-1.5Z"/><path d="M8 7h8"/><path d="M8 11h8"/><path d="M8 15h5"/>"#,
        ),
        ("whiteboard", "line") => Some(r#"<path d="M7 17 17 7"/>"#),
        ("whiteboard", "lock") => Some(
            r#"<rect width="18" height="11" x="3" y="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/>"#,
        ),
        ("whiteboard", "math") => Some(r#"<path d="M18 7V4H6l6 8-6 8h12v-3"/>"#),
        ("whiteboard", "move") => Some(
            r#"<path d="M12 2v20"/><path d="m15 19-3 3-3-3"/><path d="m15 5-3-3-3 3"/><path d="M2 12h20"/><path d="m19 15 3-3-3-3"/><path d="m5 15-3-3 3-3"/>"#,
        ),
        ("whiteboard", "panel") => {
            Some(r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M15 3v18"/>"#)
        }
        ("whiteboard", "pen") => Some(
            r#"<path d="M21.17 6.81a1 1 0 0 0-3.98-3.98L3.84 16.17a2 2 0 0 0-.5.83L2 22l5-1.34a2 2 0 0 0 .83-.5Z"/><path d="m15 5 4 4"/>"#,
        ),
        ("whiteboard", "rectangle") => Some(r#"<rect width="18" height="12" x="3" y="6" rx="2"/>"#),
        ("whiteboard", "redo") => Some(
            r#"<path d="m15 14 5-5-5-5"/><path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5 5.5 5.5 0 0 0 9.5 20H13"/>"#,
        ),
        ("whiteboard", "select") => Some(
            r#"<path d="m4 4 7.07 17 2.51-7.39L21 11.07Z"/><path d="m11.07 11.07 5.66 5.66"/>"#,
        ),
        ("whiteboard", "snap") => Some(
            r#"<path d="M4 6V4h2"/><path d="M18 4h2v2"/><path d="M20 18v2h-2"/><path d="M6 20H4v-2"/><path d="M9 12h6"/><path d="M12 9v6"/>"#,
        ),
        ("whiteboard", "table") => Some(
            r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M3 15h18"/><path d="M12 3v18"/>"#,
        ),
        ("whiteboard", "text") => Some(
            r#"<path d="M4 7V5a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v2"/><path d="M12 4v16"/><path d="M9 20h6"/>"#,
        ),
        ("whiteboard", "trash") => Some(
            r#"<path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><path d="m19 6-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/>"#,
        ),
        ("whiteboard", "undo") => Some(
            r#"<path d="m9 14-5-5 5-5"/><path d="M4 9h10.5A5.5 5.5 0 0 1 20 14.5 5.5 5.5 0 0 1 14.5 20H11"/>"#,
        ),
        ("whiteboard", "unlock") => Some(
            r#"<rect width="18" height="11" x="3" y="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 9.9-1"/>"#,
        ),
        ("whiteboard", "video") => Some(
            r#"<rect width="14" height="12" x="2" y="6" rx="2"/><path d="m16 13 5.2 3.5a.5.5 0 0 0 .8-.4V7.9a.5.5 0 0 0-.8-.4L16 11"/>"#,
        ),
        ("whiteboard", "zoom-in") => Some(
            r#"<circle cx="11" cy="11" r="8"/><path d="M21 21l-4.3-4.3"/><path d="M11 8v6"/><path d="M8 11h6"/>"#,
        ),
        ("whiteboard", "zoom-out") => {
            Some(r#"<circle cx="11" cy="11" r="8"/><path d="M21 21l-4.3-4.3"/><path d="M8 11h6"/>"#)
        }
        ("action", "check") | ("status", "check") | ("pack", "check") => {
            Some(r#"<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><path d="m9 11 3 3L22 4"/>"#)
        }
        ("action", "login") => Some(
            r#"<path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/><path d="m10 17 5-5-5-5"/><path d="M15 12H3"/>"#,
        ),
        ("action", "logout") => Some(
            r#"<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><path d="m16 17 5-5-5-5"/><path d="M21 12H9"/>"#,
        ),
        ("action", "menu") => {
            Some(r#"<path d="M4 6h16"/><path d="M4 12h16"/><path d="M4 18h16"/>"#)
        }
        ("action", "open") => Some(r#"<path d="M7 7h10v10M7 17 17 7"/>"#),
        ("action", "rocket") | ("pack", "motion") => Some(
            r#"<path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09Z"/><path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2Z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"/><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/>"#,
        ),
        ("action", "tools") | ("pack", "settings") => Some(
            r#"<path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94Z"/>"#,
        ),
        ("ai", "brain") | ("pack", "ai") => Some(
            r#"<path d="M12 5a3 3 0 0 0-5.83-1M12 5a3 3 0 0 1 5.83-1M12 5v14"/><path d="M7 8H4a2 2 0 0 0 0 4h1"/><path d="M17 8h3a2 2 0 0 1 0 4h-1"/><path d="M7 16H5a2 2 0 1 0 0 4h2"/><path d="M17 16h2a2 2 0 1 1 0 4h-2"/>"#,
        ),
        ("api", "trpc") => Some(
            r#"<path d="M4 7h6"/><path d="M14 7h6"/><path d="M10 7a2 2 0 1 0 4 0 2 2 0 0 0-4 0"/><path d="M12 9v6"/><path d="M7 17h10"/><path d="M7 17a2 2 0 1 0-4 0 2 2 0 0 0 4 0"/><path d="M21 17a2 2 0 1 1-4 0 2 2 0 0 1 4 0"/>"#,
        ),
        ("commerce", "card") | ("pack", "payments") => Some(
            r#"<rect width="20" height="14" x="2" y="5" rx="2"/><path d="M2 10h20"/><path d="M6 15h2"/><path d="M10 15h4"/>"#,
        ),
        ("content", "folder-check") | ("pack", "fumadocs") => Some(
            r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.7-.9L9.6 3.9A2 2 0 0 0 7.9 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="m9 13 2 2 4-4"/>"#,
        ),
        ("database", "supabase") | ("pack", "database") => Some(
            r#"<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5v14c0 1.66 4.03 3 9 3s9-1.34 9-3V5"/><path d="M3 12c0 1.66 4.03 3 9 3s9-1.34 9-3"/>"#,
        ),
        ("i18n", "languages") | ("pack", "i18n") => Some(
            r#"<path d="m5 8 6 6"/><path d="m4 14 6-6 2-3"/><path d="M2 5h12"/><path d="M7 2h1"/><path d="m22 22-5-10-5 10"/><path d="M14 18h6"/>"#,
        ),
        ("nav", "dashboard") | ("pack", "ui-components") => Some(
            r#"<rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/>"#,
        ),
        ("nav", "reports") | ("pack", "state") => Some(
            r#"<path d="M3 3v18h18"/><path d="M18 17V9"/><path d="M13 17V5"/><path d="M8 17v-3"/>"#,
        ),
        ("nav", "settings") => Some(
            r#"<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.38a2 2 0 0 0-.73-2.73l-.15-.09a2 2 0 0 1-1-1.74v-.51a2 2 0 0 1 1-1.72l.15-.1a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2Z"/><circle cx="12" cy="12" r="3"/>"#,
        ),
        ("nav", "team") => Some(
            r#"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/>"#,
        ),
        ("pack", "auth") => Some(
            r#"<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/><path d="m16 11 2 2 4-4"/>"#,
        ),
        ("pack", "forms") | ("status", "list-checks") => Some(
            r#"<path d="m3 7 2 2 4-4"/><path d="m3 17 2 2 4-4"/><path d="M13 6h8"/><path d="M13 12h8"/><path d="M13 18h8"/>"#,
        ),
        ("pack", "n8n") | ("pack", "query") | ("pack", "tanstack-query") => {
            Some(r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/>"#)
        }
        ("pack", "play") => Some(r#"<circle cx="12" cy="12" r="10"/><path d="m10 8 6 4-6 4V8Z"/>"#),
        ("pack", "three-scene") | ("pack", "workspace") => Some(
            r#"<path d="M2.97 12.92 12 18.15l9.03-5.23"/><path d="M2.97 7.08 12 12.31l9.03-5.23L12 1.85 2.97 7.08Z"/><path d="M2.97 7.08v5.84L12 18.15v-5.84"/><path d="M21.03 7.08v5.84L12 18.15"/><path d="M12 22.15v-4"/>"#,
        ),
        ("pack", "validation") => Some(
            r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.77 4 4 0 0 1 0 6.76 4 4 0 0 1-4.78 4.77 4 4 0 0 1-6.74 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="m9 12 2 2 4-4"/>"#,
        ),
        ("pack", "wasm-bindgen") | ("state", "bolt") => {
            Some(r#"<path d="M13 2 3 14h7l-1 8 10-12h-7l1-8Z"/>"#)
        }
        ("theme", "moon") => Some(r#"<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>"#),
        ("theme", "sun") => Some(
            r#"<circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>"#,
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::icons;

    const WHITEBOARD_TOOLBAR_ICON_NAMES: &[&str] = &[
        "align",
        "arrow",
        "copy",
        "diamond",
        "download",
        "ellipse",
        "eraser",
        "eye",
        "eye-off",
        "fit",
        "grid",
        "hand",
        "image",
        "library",
        "line",
        "lock",
        "move",
        "pen",
        "rectangle",
        "redo",
        "select",
        "snap",
        "text",
        "undo",
        "unlock",
        "zoom-in",
        "zoom-out",
    ];

    #[test]
    fn whiteboard_move_icon_resolves_to_svg_paths() {
        let mut reader = icons();
        let icon = reader
            .get("whiteboard", "move")
            .expect("whiteboard move icon should resolve");
        let svg = icon.to_svg(24);

        assert!(!svg.contains(r#"data-dx-icon-set"#));
        assert!(svg.contains(r#"<path d="M12 2v20"/>"#));
        assert!(svg.contains(r#"<path d="M2 12h20"/>"#));
    }

    #[test]
    fn whiteboard_toolbar_icon_names_resolve_to_svg_paths() {
        let mut reader = icons();

        for name in WHITEBOARD_TOOLBAR_ICON_NAMES {
            let icon = reader
                .get("whiteboard", name)
                .unwrap_or_else(|| panic!("whiteboard toolbar icon `{name}` should resolve"));
            let svg = icon.to_svg(24);

            assert!(
                svg.contains("<path")
                    || svg.contains("<rect")
                    || svg.contains("<ellipse")
                    || svg.contains("<circle"),
                "whiteboard toolbar icon `{name}` should render SVG geometry"
            );
            assert!(
                !svg.contains("data-dx-icon-missing") && !svg.contains("data-dx-icon-set"),
                "whiteboard toolbar icon `{name}` should not use fallback geometry"
            );
        }
    }

    #[test]
    fn unknown_icon_names_are_explicitly_missing() {
        let mut reader = icons();

        assert!(reader.get("whiteboard", "unknown-toolbar-icon").is_none());
        assert!(reader.get("unknown-set", "move").is_none());
    }
}
