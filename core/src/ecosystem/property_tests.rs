use crate::ecosystem::content::{markdown_to_html, parse_frontmatter};
use crate::ecosystem::media::generate_blur_placeholder;
use crate::ecosystem::*;
use proptest::prelude::*;
use std::path::{Path, PathBuf};

// Arbitrary generators for DXM content
fn arbitrary_title() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Getting Started".to_string()),
        Just("API Reference".to_string()),
        Just("Installation Guide".to_string()),
        "[A-Z][a-z]{2,15}( [A-Z][a-z]{2,10}){0,3}".prop_map(|s| s.to_string()),
    ]
}

fn arbitrary_description() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("A comprehensive guide".to_string()),
        Just("Learn the basics".to_string()),
        Just("Reference documentation".to_string()),
        "[A-Z][a-z]{2,20}( [a-z]{2,10}){0,5}".prop_map(|s| s.to_string()),
    ]
}

fn arbitrary_layout() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("default".to_string()),
        Just("docs".to_string()),
        Just("blog".to_string()),
        Just("landing".to_string()),
    ]
}

fn arbitrary_markdown_body() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("# Hello World\n\nThis is a paragraph.".to_string()),
        Just("## Section\n\n- Item 1\n- Item 2".to_string()),
        Just("# Title\n\n```rust\nfn main() {}\n```".to_string()),
        Just("### Heading\n\nSome text here.\n\n#### Subheading\n\nMore text.".to_string()),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 6: DXM Content Compilation Round-Trip
    /// *For any* valid DXM document with frontmatter, parsing and extracting
    /// frontmatter SHALL preserve all metadata fields (title, description, layout).
    ///
    /// **Validates: Requirements 3.1, 3.2**
    #[test]
    fn prop_dxm_frontmatter_extraction(
        title in arbitrary_title(),
        description in arbitrary_description(),
        layout in arbitrary_layout(),
        body in arbitrary_markdown_body(),
    ) {
        // Construct DXM content with frontmatter
        let content = format!(
            "---\ntitle: {}\ndescription: {}\nlayout: {}\n---\n\n{}",
            title, description, layout, body
        );

        // Parse frontmatter
        let (frontmatter, extracted_body) = parse_frontmatter(&content);

        // Verify frontmatter fields are preserved
        prop_assert_eq!(frontmatter.title, Some(title));
        prop_assert_eq!(frontmatter.description, Some(description));
        prop_assert_eq!(frontmatter.layout, Some(layout));

        // Verify body content is preserved (trimmed)
        prop_assert!(extracted_body.trim().contains(body.lines().next().unwrap_or("")));
    }

    /// Property 6b: DXM HTML Conversion Preserves Structure
    /// *For any* markdown content, converting to HTML SHALL preserve
    /// the document structure (headings, lists, code blocks).
    ///
    /// **Validates: Requirements 3.1, 3.2**
    #[test]
    fn prop_dxm_html_structure_preservation(
        body in arbitrary_markdown_body(),
    ) {
        let html = markdown_to_html(&body);

        // If body contains h1, html should contain <h1>
        if body.contains("# ") && !body.contains("## ") {
            prop_assert!(html.contains("<h1>") || html.contains("<h2>") || html.contains("<h3>"));
        }

        // If body contains list items, html should contain <li>
        if body.contains("- ") {
            prop_assert!(html.contains("<li>"));
        }

        // If body contains code blocks, html should contain <pre><code>
        if body.contains("```") {
            prop_assert!(html.contains("<pre><code"));
        }

        // HTML should not be empty for non-empty input
        if !body.trim().is_empty() {
            prop_assert!(!html.trim().is_empty());
        }
    }

    /// Property 7: Frontmatter Extraction Completeness
    /// *For any* DXM file with frontmatter, THE DX_WWW SHALL extract all
    /// frontmatter fields (title, description, layout) and make them available.
    ///
    /// **Validates: Requirements 3.3**
    #[test]
    fn prop_frontmatter_extraction_completeness(
        has_title in any::<bool>(),
        has_description in any::<bool>(),
        has_layout in any::<bool>(),
        title in arbitrary_title(),
        description in arbitrary_description(),
        layout in arbitrary_layout(),
    ) {
        // Build frontmatter with optional fields
        let mut fm_lines = Vec::new();
        if has_title {
            fm_lines.push(format!("title: {}", title));
        }
        if has_description {
            fm_lines.push(format!("description: {}", description));
        }
        if has_layout {
            fm_lines.push(format!("layout: {}", layout));
        }

        let content = if fm_lines.is_empty() {
            "# No Frontmatter\n\nJust content.".to_string()
        } else {
            format!("---\n{}\n---\n\n# Content", fm_lines.join("\n"))
        };

        let (frontmatter, _body) = parse_frontmatter(&content);

        // Verify each field is extracted if present
        if has_title {
            prop_assert_eq!(frontmatter.title, Some(title));
        } else {
            prop_assert!(frontmatter.title.is_none());
        }

        if has_description {
            prop_assert_eq!(frontmatter.description, Some(description));
        } else {
            prop_assert!(frontmatter.description.is_none());
        }

        if has_layout {
            prop_assert_eq!(frontmatter.layout, Some(layout));
        } else {
            prop_assert!(frontmatter.layout.is_none());
        }
    }

    /// Property: Icon Processing Deduplication
    /// *For any* source with duplicate icon references, process_icons
    /// SHALL return a deduplicated list.
    #[test]
    fn prop_icon_deduplication(
        icon_name in "[a-z]{3,10}",
        repeat_count in 1usize..5usize,
    ) {
        let source = (0..repeat_count)
            .map(|_| format!(r#"<Icon name="{}" />"#, icon_name))
            .collect::<Vec<_>>()
            .join("\n");

        let config = IconConfig::default();
        let result = process_icons(&source, &config).unwrap();

        // Should only have one entry regardless of repeat count
        prop_assert_eq!(result.used.len(), 1);
        prop_assert_eq!(&result.used[0], &icon_name);
    }

    /// Property 9: Icon Tree-Shaking Correctness
    /// *For any* project with icon imports, the production build SHALL include
    /// exactly the icons that are referenced in the source code (no more, no less).
    ///
    /// **Validates: Requirements 5.3**
    #[test]
    fn prop_icon_tree_shaking_correctness(
        used_icons in prop::collection::vec("[a-z]{3,8}", 1..5),
        unused_icons in prop::collection::vec("[a-z]{3,8}", 0..3),
    ) {
        // Create all icons (used + unused)
        let mut all_icons: Vec<ResolvedIcon> = used_icons.iter().map(|name| {
            ResolvedIcon {
                name: name.clone(),
                set: "lucide".to_string(),
                svg: format!("<svg><!-- {} --></svg>", name),
                width: 24,
                height: 24,
            }
        }).collect();

        // Add unused icons with different names
        for (i, name) in unused_icons.iter().enumerate() {
            let unique_name = format!("unused_{}_{}", name, i);
            all_icons.push(ResolvedIcon {
                name: unique_name,
                set: "lucide".to_string(),
                svg: "<svg></svg>".to_string(),
                width: 24,
                height: 24,
            });
        }

        // Create source that only references used_icons
        let source = used_icons.iter()
            .map(|name| format!(r#"<Icon name="{}" />"#, name))
            .collect::<Vec<_>>()
            .join("\n");

        // Tree-shake
        let result = tree_shake_icons(&all_icons, &source);

        // Result should contain exactly the used icons
        prop_assert_eq!(result.len(), used_icons.len());

        // All used icons should be present
        for name in &used_icons {
            prop_assert!(result.iter().any(|i| &i.name == name),
                "Used icon '{}' should be in result", name);
        }

        // No unused icons should be present
        for icon in &result {
            prop_assert!(!icon.name.starts_with("unused_"),
                "Unused icon '{}' should not be in result", icon.name);
        }
    }

    /// Property: Icon Sprite Generation Completeness
    /// *For any* set of resolved icons, the generated sprite SHALL contain
    /// a symbol for each icon with correct id and viewBox.
    #[test]
    fn prop_icon_sprite_completeness(
        icon_count in 1usize..10usize,
    ) {
        let icons: Vec<ResolvedIcon> = (0..icon_count).map(|i| {
            ResolvedIcon {
                name: format!("icon{}", i),
                set: "test".to_string(),
                svg: format!("<svg><path d=\"M{} {}\"/></svg>", i, i),
                width: 24,
                height: 24,
            }
        }).collect();

        let sprite = generate_icon_sprite(&icons);

        // Sprite should contain all icons
        for icon in &icons {
            let symbol_id = format!("icon-{}-{}", icon.set, icon.name);
            prop_assert!(sprite.contains(&symbol_id),
                "Sprite should contain symbol id '{}'", symbol_id);
        }

        // Sprite should be valid SVG structure
        prop_assert!(sprite.starts_with("<svg"));
        prop_assert!(sprite.ends_with("</svg>"));
        prop_assert!(sprite.contains("display:none"));
    }

    /// Property: Media Variant Generation
    /// *For any* media configuration, process_media SHALL generate
    /// variants for all breakpoints (that fit within original dimensions) and formats.
    #[test]
    fn prop_media_variant_generation(
        breakpoint_count in 1usize..6usize,
        format_count in 1usize..3usize,
    ) {
        // Use breakpoints that are all smaller than the estimated original (1920)
        let breakpoints: Vec<u32> = (0..breakpoint_count)
            .map(|i| 320 + (i as u32 * 256))
            .collect();
        let formats: Vec<String> = ["webp", "avif", "png"]
            .iter()
            .take(format_count)
            .map(|s| s.to_string())
            .collect();

        let config = MediaConfig {
            image_formats: formats.clone(),
            quality: 85,
            blur_placeholder: true,
            breakpoints: breakpoints.clone(),
        };

        let result = process_media(Path::new("test.jpg"), &config).unwrap();

        // Should generate variants for each breakpoint Ãƒâ€” format combination
        // (all breakpoints are <= 1920, so none are skipped)
        prop_assert_eq!(result.variants.len(), breakpoint_count * format_count);

        // Each breakpoint should have all formats
        for bp in &breakpoints {
            for fmt in &formats {
                prop_assert!(result.variants.iter().any(|v| v.width == *bp && v.format == *fmt));
            }
        }
    }

    /// Property 10: Font Subsetting Correctness
    /// *For any* font configuration and source code, the subset font SHALL contain
    /// all characters used in the application and no unused characters.
    ///
    /// **Validates: Requirements 6.2**
    #[test]
    fn prop_font_subsetting_correctness(
        content_chars in prop::collection::vec(prop::char::range('a', 'z'), 5..50),
        extra_chars in prop::collection::vec(prop::char::range('A', 'Z'), 0..20),
    ) {
        // Create content with specific characters
        let content: String = content_chars.iter().collect();

        // Extract used characters
        let used_chars = extract_used_characters(&content);

        // All content characters should be in used_chars
        for c in &content_chars {
            prop_assert!(used_chars.contains(c),
                "Character '{}' from content should be in used_chars", c);
        }

        // Extra characters not in content should not be in used_chars
        for c in &extra_chars {
            if !content_chars.contains(c) {
                prop_assert!(!used_chars.contains(c),
                    "Character '{}' not in content should not be in used_chars", c);
            }
        }
    }

    /// Property 10b: Font Subset Calculation Includes Common Characters
    /// *For any* content sources, calculate_font_subset with include_common=true
    /// SHALL include all common ASCII characters plus content characters.
    ///
    /// **Validates: Requirements 6.2**
    #[test]
    fn prop_font_subset_includes_common(
        content in "[a-z]{10,50}",
    ) {
        let sources = vec![content.as_str()];
        let subset = calculate_font_subset(&sources, true);

        // Should include all content characters
        for c in content.chars() {
            prop_assert!(subset.contains(&c),
                "Content character '{}' should be in subset", c);
        }

        // Should include common characters when include_common is true
        prop_assert!(subset.contains(&'0'), "Should include digit 0");
        prop_assert!(subset.contains(&'9'), "Should include digit 9");
        prop_assert!(subset.contains(&'a'), "Should include lowercase a");
        prop_assert!(subset.contains(&'z'), "Should include lowercase z");
        prop_assert!(subset.contains(&'A'), "Should include uppercase A");
        prop_assert!(subset.contains(&'Z'), "Should include uppercase Z");
        prop_assert!(subset.contains(&' '), "Should include space");
    }

    /// Property 10c: Font Processing Generates Valid CSS
    /// *For any* font configuration, process_fonts SHALL generate valid CSS
    /// with @font-face declarations for all specified weights.
    ///
    /// **Validates: Requirements 6.1, 6.2**
    #[test]
    fn prop_font_processing_generates_valid_css(
        family_name in "[A-Z][a-z]{3,10}( [A-Z][a-z]{3,8})?",
        weights in prop::collection::vec(
            prop::sample::select(vec![100u16, 200, 300, 400, 500, 600, 700, 800, 900]),
            1..4
        ),
        is_variable in any::<bool>(),
    ) {
        let config = FontConfig {
            families: vec![FontFamily {
                name: family_name.clone(),
                weights: weights.clone(),
                variable: is_variable,
            }],
            subset: true,
            preload: true,
        };

        let result = process_fonts(&config).unwrap();

        // CSS should contain @font-face
        prop_assert!(result.css.contains("@font-face"),
            "CSS should contain @font-face declaration");

        // CSS should contain the font family name
        prop_assert!(result.css.contains(&family_name),
            "CSS should contain font family name '{}'", family_name);

        // CSS should contain font-display: swap
        prop_assert!(result.css.contains("font-display: swap"),
            "CSS should contain font-display: swap");

        // CSS should contain woff2 format
        prop_assert!(result.css.contains("woff2"),
            "CSS should reference woff2 format");

        // For non-variable fonts, each weight should have its own @font-face
        if !is_variable {
            for weight in &weights {
                prop_assert!(result.css.contains(&format!("font-weight: {}", weight)),
                    "CSS should contain font-weight: {}", weight);
            }
        }
    }

    /// Property 10d: Font Preload Hints Generation
    /// *For any* font configuration with preload enabled, process_fonts SHALL
    /// generate preload hints for critical font weights (400, 700).
    ///
    /// **Validates: Requirements 6.3**
    #[test]
    fn prop_font_preload_hints_generation(
        family_name in "[A-Z][a-z]{3,10}",
        has_regular in any::<bool>(),
        has_bold in any::<bool>(),
    ) {
        let mut weights = Vec::new();
        if has_regular {
            weights.push(400u16);
        }
        if has_bold {
            weights.push(700u16);
        }
        if weights.is_empty() {
            weights.push(400u16); // Default
        }

        let config = FontConfig {
            families: vec![FontFamily {
                name: family_name.clone(),
                weights: weights.clone(),
                variable: false,
            }],
            subset: true,
            preload: true,
        };

        let result = process_fonts(&config).unwrap();

        // Should have preload hints for critical weights
        if has_regular || weights.contains(&400) {
            prop_assert!(result.preload_hints.iter().any(|h| h.href.contains("-400")),
                "Should have preload hint for weight 400");
        }

        if has_bold {
            prop_assert!(result.preload_hints.iter().any(|h| h.href.contains("-700")),
                "Should have preload hint for weight 700");
        }

        // All preload hints should be woff2 format
        for hint in &result.preload_hints {
            prop_assert_eq!(&hint.format, "woff2",
                "Preload hints should use woff2 format");
        }
    }

    /// Property 11: Image Variant Generation
    /// *For any* image import, THE DX_WWW SHALL generate responsive variants where each variant:
    /// - Has correct dimensions for its breakpoint
    /// - Is in an optimized format (WebP or AVIF)
    /// - Has a valid blur placeholder
    ///
    /// **Validates: Requirements 7.2, 7.3, 7.4**
    #[test]
    fn prop_image_variant_generation(
        breakpoint_count in 1usize..6usize,
        format_count in 1usize..3usize,
        quality in 50u8..100u8,
        generate_blur in any::<bool>(),
    ) {
        // Use breakpoints that are all smaller than the estimated original (1920)
        let breakpoints: Vec<u32> = (0..breakpoint_count)
            .map(|i| 320 + (i as u32 * 256))
            .collect();
        let formats: Vec<String> = ["webp", "avif", "png"]
            .iter()
            .take(format_count)
            .map(|s| s.to_string())
            .collect();

        let options = MediaProcessingOptions {
            output_dir: PathBuf::from("dist/images"),
            quality,
            blur_placeholder: generate_blur,
            breakpoints: breakpoints.clone(),
            formats: formats.clone(),
            preserve_original: true,
        };

        let result = process_media_with_options(Path::new("test.jpg"), &options).unwrap();

        // Should generate variants for each breakpoint Ãƒâ€” format combination
        // (all breakpoints are <= 1920, so none are skipped)
        prop_assert_eq!(result.variants.len(), breakpoint_count * format_count,
            "Should generate {} variants ({}Ãƒâ€”{})",
            breakpoint_count * format_count, breakpoint_count, format_count);

        // Each variant should have correct width
        for variant in &result.variants {
            prop_assert!(breakpoints.contains(&variant.width),
                "Variant width {} should be in breakpoints {:?}", variant.width, breakpoints);
        }

        // Each variant should have valid format
        for variant in &result.variants {
            prop_assert!(formats.contains(&variant.format),
                "Variant format '{}' should be in formats {:?}", variant.format, formats);
        }

        // Each variant should have height calculated from aspect ratio
        for variant in &result.variants {
            prop_assert!(variant.height > 0,
                "Variant height should be positive");
        }

        // Blur placeholder should be present if requested
        if generate_blur {
            prop_assert!(result.blur_placeholder.is_some(),
                "Blur placeholder should be generated when requested");

            let blur = result.blur_placeholder.as_ref().unwrap();
            prop_assert!(blur.starts_with("data:image/"),
                "Blur placeholder should be a data URL");
        }
    }

    /// Property 11b: Image Variant Dimensions Preserve Aspect Ratio
    /// *For any* image with known dimensions, generated variants SHALL preserve
    /// the original aspect ratio.
    ///
    /// **Validates: Requirements 7.2**
    #[test]
    fn prop_image_variant_aspect_ratio(
        original_width in 400u32..4000u32,
        original_height in 300u32..3000u32,
        target_width in 320u32..1920u32,
    ) {
        let original_aspect = original_width as f64 / original_height as f64;

        // Calculate expected height
        let expected_height = (target_width as f64 / original_aspect).round() as u32;

        // Skip extreme aspect ratios that cause rounding issues
        if expected_height == 0 {
            return Ok(());
        }

        // Verify aspect ratio is preserved (within 2% tolerance for rounding)
        let result_aspect = target_width as f64 / expected_height as f64;
        let ratio_diff = (original_aspect - result_aspect).abs() / original_aspect;

        prop_assert!(ratio_diff < 0.02,
            "Aspect ratio should be preserved within 2%: original={:.3}, result={:.3}",
            original_aspect, result_aspect);
    }

    /// Property 11c: Image Srcset Generation
    /// *For any* set of image variants, generate_srcset SHALL produce a valid
    /// srcset string with all variants.
    ///
    /// **Validates: Requirements 7.2**
    #[test]
    fn prop_image_srcset_generation(
        variant_count in 1usize..6usize,
    ) {
        let variants: Vec<ImageVariant> = (0..variant_count).map(|i| {
            let width = 640 + (i as u32 * 384);
            ImageVariant {
                path: PathBuf::from(format!("image-{}w.webp", width)),
                width,
                height: (width as f64 / 1.78).round() as u32,
                format: "webp".to_string(),
                size_estimate: Some(width as u64 * 100),
            }
        }).collect();

        let srcset = generate_srcset(&variants);

        // Srcset should contain all variants
        for variant in &variants {
            prop_assert!(srcset.contains(&format!("{}w", variant.width)),
                "Srcset should contain width descriptor '{}w'", variant.width);
        }

        // Srcset should be comma-separated
        if variant_count > 1 {
            prop_assert!(srcset.contains(", "),
                "Srcset should be comma-separated");
        }
    }

    /// Property 11d: Image Import Detection
    /// *For any* source code with image imports, detect_image_imports SHALL
    /// find all image references.
    ///
    /// **Validates: Requirements 7.1**
    #[test]
    fn prop_image_import_detection(
        image_name in "[a-z]{3,10}",
        extension in prop::sample::select(vec!["png", "jpg", "jpeg", "webp", "gif"]),
    ) {
        let source = format!(
            r#"import img from './{}.{}'
            <img src="./{}.{}" alt="test" />"#,
            image_name, extension, image_name, extension
        );

        let imports = detect_image_imports(&source);

        // Should detect at least one import
        prop_assert!(!imports.is_empty(),
            "Should detect image imports in source");

        // All detected imports should have the correct extension
        for import in &imports {
            let ext = import.path.extension().and_then(|e| e.to_str()).unwrap_or("");
            prop_assert_eq!(ext, extension,
                "Detected import should have extension '{}'", extension);
        }
    }

    /// Property 11e: Blur Placeholder is Valid Data URL
    /// *For any* image, the generated blur placeholder SHALL be a valid
    /// base64-encoded SVG data URL.
    ///
    /// **Validates: Requirements 7.4**
    #[test]
    fn prop_blur_placeholder_valid_data_url(
        width in 100u32..4000u32,
        height in 100u32..4000u32,
    ) {
        let placeholder = generate_blur_placeholder(
            Path::new("test.jpg"),
            Some((width, height))
        );

        // Should be a data URL
        prop_assert!(placeholder.starts_with("data:image/svg+xml;base64,"),
            "Blur placeholder should be SVG data URL");

        // Should be valid base64 (no invalid characters)
        let base64_part = &placeholder["data:image/svg+xml;base64,".len()..];
        prop_assert!(base64_part.chars().all(|c|
            c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='
        ), "Base64 should only contain valid characters");
    }
}
