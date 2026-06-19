#[cfg(test)]
mod tests {
    use crate::ecosystem::content::{markdown_to_html, parse_frontmatter};
    use crate::ecosystem::media::base64_encode;
    use crate::ecosystem::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_init_default() {
        let config = init_default();
        assert!(config.icon_config.is_some());
        assert!(config.font_config.is_some());
        assert!(config.media_config.is_some());
    }

    #[test]
    fn test_process_icons() {
        let source = r#"
            <Icon name="home" />
            <Icon name="settings" />
            <Icon name="home" />
        "#;

        let config = IconConfig::default();
        let result = process_icons(source, &config).unwrap();

        assert_eq!(result.used.len(), 2); // Deduplicated
        assert!(result.used.contains(&"home".to_string()));
        assert!(result.used.contains(&"settings".to_string()));
    }

    #[test]
    fn test_process_icons_with_set() {
        let source = r#"
            <Icon name="mdi:home" />
            <Icon set="heroicons" name="arrow-left" />
            <dx-icon name="pack:auth" />
        "#;

        let config = IconConfig {
            sets: vec![
                "mdi".to_string(),
                "heroicons".to_string(),
                "pack".to_string(),
            ],
            ..Default::default()
        };
        let result = process_icons(source, &config).unwrap();

        assert_eq!(result.used.len(), 3);
        assert!(result.used.contains(&"home".to_string()));
        assert!(result.used.contains(&"arrow-left".to_string()));
        assert!(result.used.contains(&"auth".to_string()));
        assert!(
            result
                .resolved
                .iter()
                .any(|icon| icon.set == "pack" && icon.name == "auth")
        );
    }

    #[test]
    fn test_generate_icon_sprite() {
        let icons = vec![ResolvedIcon {
            name: "home".to_string(),
            set: "lucide".to_string(),
            svg: "<svg><path d=\"M1 1\"/></svg>".to_string(),
            width: 24,
            height: 24,
        }];

        let sprite = generate_icon_sprite(&icons);
        assert!(sprite.contains("icon-lucide-home"));
        assert!(sprite.contains("<symbol"));
        assert!(sprite.contains("viewBox"));
    }

    #[test]
    fn test_tree_shake_icons() {
        let all_icons = vec![
            ResolvedIcon {
                name: "home".to_string(),
                set: "lucide".to_string(),
                svg: "<svg></svg>".to_string(),
                width: 24,
                height: 24,
            },
            ResolvedIcon {
                name: "settings".to_string(),
                set: "lucide".to_string(),
                svg: "<svg></svg>".to_string(),
                width: 24,
                height: 24,
            },
            ResolvedIcon {
                name: "unused".to_string(),
                set: "lucide".to_string(),
                svg: "<svg></svg>".to_string(),
                width: 24,
                height: 24,
            },
        ];

        let source = r#"<Icon name="home" /><Icon name="settings" />"#;
        let used = tree_shake_icons(&all_icons, source);

        assert_eq!(used.len(), 2);
        assert!(used.iter().any(|i| i.name == "home"));
        assert!(used.iter().any(|i| i.name == "settings"));
        assert!(!used.iter().any(|i| i.name == "unused"));
    }

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
title: Test Page
description: A test page
layout: default
---

# Hello World
"#;

        let (fm, body) = parse_frontmatter(content);
        assert_eq!(fm.title, Some("Test Page".to_string()));
        assert_eq!(fm.description, Some("A test page".to_string()));
        assert_eq!(fm.layout, Some("default".to_string()));
        assert!(body.contains("# Hello World"));
    }

    #[test]
    fn test_markdown_to_html() {
        let md = "# Title\n\nParagraph text\n\n- Item 1\n- Item 2";
        let html = markdown_to_html(md);

        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<p>Paragraph text</p>"));
        assert!(html.contains("<li>Item 1</li>"));
    }

    #[test]
    fn test_generate_srcset() {
        let variants = vec![
            ImageVariant {
                path: PathBuf::from("image-640w.webp"),
                width: 640,
                height: 360,
                format: "webp".to_string(),
                size_estimate: None,
            },
            ImageVariant {
                path: PathBuf::from("image-1024w.webp"),
                width: 1024,
                height: 576,
                format: "webp".to_string(),
                size_estimate: None,
            },
        ];

        let srcset = generate_srcset(&variants);
        assert!(srcset.contains("640w"));
        assert!(srcset.contains("1024w"));
    }

    #[test]
    fn test_process_fonts_basic() {
        let config = FontConfig {
            families: vec![FontFamily {
                name: "Inter".to_string(),
                weights: vec![400, 700],
                variable: false,
            }],
            subset: true,
            preload: true,
        };

        let result = process_fonts(&config).unwrap();

        assert_eq!(result.fonts.len(), 1);
        assert!(result.css.contains("Inter"));
        assert!(result.css.contains("font-weight: 400"));
        assert!(result.css.contains("font-weight: 700"));
        assert!(!result.preload_hints.is_empty());
    }

    #[test]
    fn test_process_fonts_variable() {
        let config = FontConfig {
            families: vec![FontFamily {
                name: "Roboto Flex".to_string(),
                weights: vec![100, 900],
                variable: true,
            }],
            subset: true,
            preload: true,
        };

        let result = process_fonts(&config).unwrap();

        assert!(result.css.contains("woff2-variations"));
        assert!(result.css.contains("font-weight: 100 900"));
    }

    #[test]
    fn test_extract_used_characters() {
        let source = "Hello World! 123";
        let chars = extract_used_characters(source);

        assert!(chars.contains(&'H'));
        assert!(chars.contains(&'e'));
        assert!(chars.contains(&' '));
        assert!(chars.contains(&'!'));
        assert!(chars.contains(&'1'));
        assert!(!chars.contains(&'Z'));
    }

    #[test]
    fn test_calculate_font_subset_with_common() {
        let sources = vec!["Hello"];
        let subset = calculate_font_subset(&sources, true);

        // Should include content chars
        assert!(subset.contains(&'H'));
        assert!(subset.contains(&'e'));

        // Should include common chars
        assert!(subset.contains(&'a'));
        assert!(subset.contains(&'0'));
        assert!(subset.contains(&' '));
    }

    #[test]
    fn test_calculate_font_subset_without_common() {
        let sources = vec!["Hello"];
        let subset = calculate_font_subset(&sources, false);

        // Should include only content chars
        assert!(subset.contains(&'H'));
        assert!(subset.contains(&'e'));
        assert!(subset.contains(&'l'));
        assert!(subset.contains(&'o'));

        // Should NOT include chars not in content
        assert!(!subset.contains(&'Z'));
        assert!(!subset.contains(&'9'));
    }

    #[test]
    fn test_generate_font_preload_html() {
        let hints = vec![PreloadHint {
            href: "/fonts/inter-400.woff2".to_string(),
            format: "woff2".to_string(),
            crossorigin: true,
        }];

        let html = generate_font_preload_html(&hints);

        assert!(html.contains("rel=\"preload\""));
        assert!(html.contains("as=\"font\""));
        assert!(html.contains("/fonts/inter-400.woff2"));
        assert!(html.contains("type=\"font/woff2\""));
    }

    #[test]
    fn test_detect_image_imports_es_module() {
        let source = r#"
            import heroImage from './hero.png'
            import logo from '../assets/logo.jpg'
        "#;

        let imports = detect_image_imports(source);

        assert_eq!(imports.len(), 2);
        assert!(
            imports
                .iter()
                .any(|i| i.path.to_string_lossy().contains("hero.png"))
        );
        assert!(
            imports
                .iter()
                .any(|i| i.path.to_string_lossy().contains("logo.jpg"))
        );
    }

    #[test]
    fn test_detect_image_imports_jsx() {
        let source = r#"
            <img src="./photo.webp" alt="Photo" />
            <Image src="./banner.avif" />
        "#;

        let imports = detect_image_imports(source);

        assert_eq!(imports.len(), 2);
        assert!(
            imports
                .iter()
                .any(|i| i.path.to_string_lossy().contains("photo.webp"))
        );
        assert!(
            imports
                .iter()
                .any(|i| i.path.to_string_lossy().contains("banner.avif"))
        );
    }

    #[test]
    fn test_process_media_with_options() {
        let options = MediaProcessingOptions {
            output_dir: PathBuf::from("dist/images"),
            quality: 85,
            blur_placeholder: true,
            breakpoints: vec![640, 1024],
            formats: vec!["webp".to_string()],
            preserve_original: true,
        };

        let result = process_media_with_options(Path::new("test.jpg"), &options).unwrap();

        // Should generate 2 variants (2 breakpoints Ã— 1 format)
        assert_eq!(result.variants.len(), 2);

        // Should have blur placeholder
        assert!(result.blur_placeholder.is_some());

        // Variants should have correct widths
        assert!(result.variants.iter().any(|v| v.width == 640));
        assert!(result.variants.iter().any(|v| v.width == 1024));
    }

    #[test]
    fn test_generate_picture_html() {
        let media = ProcessedMedia {
            original: PathBuf::from("test.jpg"),
            variants: vec![
                ImageVariant {
                    path: PathBuf::from("test-640w.webp"),
                    width: 640,
                    height: 360,
                    format: "webp".to_string(),
                    size_estimate: Some(50000),
                },
                ImageVariant {
                    path: PathBuf::from("test-640w.avif"),
                    width: 640,
                    height: 360,
                    format: "avif".to_string(),
                    size_estimate: Some(40000),
                },
            ],
            blur_placeholder: Some("data:image/svg+xml;base64,test".to_string()),
            original_dimensions: Some((1920, 1080)),
            dominant_color: Some("#808080".to_string()),
        };

        let html = generate_picture_html(&media, "Test image");

        assert!(html.contains("<picture>"));
        assert!(html.contains("</picture>"));
        assert!(html.contains("<source"));
        assert!(html.contains("type=\"image/avif\""));
        assert!(html.contains("type=\"image/webp\""));
        assert!(html.contains("alt=\"Test image\""));
        assert!(html.contains("loading=\"lazy\""));
    }

    #[test]
    fn test_generate_sizes() {
        let breakpoints = vec![(640, "100vw"), (1024, "50vw"), (0, "33vw")];

        let sizes = generate_sizes(&breakpoints);

        assert!(sizes.contains("(max-width: 640px) 100vw"));
        assert!(sizes.contains("(max-width: 1024px) 50vw"));
        assert!(sizes.contains("33vw"));
    }

    #[test]
    fn test_base64_encode() {
        let input = "Hello, World!";
        let encoded = base64_encode(input.as_bytes());

        // Should produce valid base64
        assert!(!encoded.is_empty());
        assert!(
            encoded
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
        );
    }

    #[test]
    fn test_compile_dxm_content_from_string() {
        let content = r#"---
title: Getting Started
description: Learn how to use DX-WWW
layout: docs
---

# Getting Started

Welcome to DX-WWW!

## Installation

Run the following command:

```bash
dx www new my-app
```

- Step 1: Create project
- Step 2: Install dependencies
"#;

        let (frontmatter, body) = parse_frontmatter(content);
        let html = markdown_to_html(&body);

        // Verify frontmatter extraction
        assert_eq!(frontmatter.title, Some("Getting Started".to_string()));
        assert_eq!(
            frontmatter.description,
            Some("Learn how to use DX-WWW".to_string())
        );
        assert_eq!(frontmatter.layout, Some("docs".to_string()));

        // Verify HTML conversion
        assert!(html.contains("<h1>Getting Started</h1>"));
        assert!(html.contains("<h2>Installation</h2>"));
        assert!(html.contains("<li>Step 1: Create project</li>"));
        assert!(html.contains("language-bash"));
    }

    #[test]
    fn test_frontmatter_without_yaml() {
        let content = "# Simple Page\n\nJust content, no frontmatter.";
        let (frontmatter, body) = parse_frontmatter(content);

        assert!(frontmatter.title.is_none());
        assert!(frontmatter.description.is_none());
        assert!(frontmatter.layout.is_none());
        assert!(body.contains("# Simple Page"));
    }

    #[test]
    fn test_markdown_code_blocks() {
        let md = "```rust\nfn main() {\n    println!(\"<Hello>\");\n}\n```";
        let html = markdown_to_html(md);

        assert!(html.contains("language-rust"));
        assert!(html.contains("fn main()"));
        assert!(html.contains("&lt;Hello&gt;")); // HTML escaped
    }
}

// ============================================================================
// Property-Based Tests
// ============================================================================
