//! Style processing for the DX-WWW build pipeline.
//!
//! Browser delivery is CSS-first by default: source CSS is normalized into generated
//! `.css` artifacts that browsers can parse natively. Binary Dawn output remains
//! available as an explicit sidecar mode for routes where the decoder cost is proven
//! to beat native CSS after compression.

use crate::error::{BuildError, Result};
use crate::hash::content_hash;
use crate::{ArtifactType, BuildArtifact, BuildCache, CacheEntry, CacheKey};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// Style processor for browser-ready CSS and optional Binary Dawn sidecars.
pub struct StyleProcessor {
    /// Configuration for style processing.
    config: StyleConfig,
}

/// Browser delivery mode for processed styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StyleDeliveryMode {
    /// Emit regular generated CSS. This is the default browser delivery path.
    GeneratedCss,
    /// Emit Binary Dawn CSS only.
    BinaryDawn,
    /// Emit generated CSS plus Binary Dawn sidecars.
    Dual,
}

/// Configuration for style processing.
#[derive(Debug, Clone)]
pub struct StyleConfig {
    /// Input directory containing CSS files (e.g., www/styles/).
    pub input_dir: PathBuf,
    /// Output directory for generated style artifacts.
    pub output_dir: PathBuf,
    /// Style artifact delivery mode.
    pub delivery_mode: StyleDeliveryMode,
    /// Enable auto-grouping with similarity detection.
    pub auto_grouping: bool,
    /// Similarity threshold for auto-grouping (0.0 to 1.0).
    pub similarity_threshold: f32,
    /// Compression level (0-9, where 9 is maximum compression).
    pub compression_level: u8,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::from("www/styles"),
            output_dir: PathBuf::from("dist/styles"),
            delivery_mode: StyleDeliveryMode::GeneratedCss,
            auto_grouping: true,
            similarity_threshold: 0.7,
            compression_level: 9,
        }
    }
}

/// Bundle of compiled Binary Dawn CSS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryStyleBundle {
    /// Binary CSS data.
    pub data: Vec<u8>,
    /// Content hash of the binary data.
    pub hash: String,
    /// Size in bytes.
    pub size: usize,
    /// Source CSS files that were compiled.
    pub sources: Vec<PathBuf>,
}

/// Metadata about a compiled style file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleArtifactMetadata {
    /// Source CSS file path.
    pub source: PathBuf,
    /// Output style asset path.
    pub output: PathBuf,
    /// Content hash of source.
    pub source_hash: String,
    /// Content hash of output.
    pub output_hash: String,
    /// Size of output in bytes.
    pub size: usize,
}

impl StyleProcessor {
    /// Create a new style processor.
    pub fn new(config: StyleConfig) -> Self {
        Self { config }
    }

    /// Compile styles for browser delivery according to [`StyleConfig::delivery_mode`].
    ///
    /// # Errors
    ///
    /// Returns an error if input discovery, generated CSS output, Binary Dawn output,
    /// or cache persistence fails.
    pub fn compile_for_browser(&self, cache: &mut BuildCache) -> Result<Vec<BuildArtifact>> {
        match self.config.delivery_mode {
            StyleDeliveryMode::GeneratedCss => self.compile_to_css(cache),
            StyleDeliveryMode::BinaryDawn => self.compile_to_binary(cache),
            StyleDeliveryMode::Dual => {
                let mut artifacts = self.compile_to_css(cache)?;
                artifacts.extend(self.compile_to_binary(cache)?);
                Ok(artifacts)
            }
        }
    }

    /// Compile all CSS files to browser-native generated CSS.
    ///
    /// This method:
    /// 1. Discovers all CSS files in the input directory.
    /// 2. Checks cache for each file.
    /// 3. Normalizes CSS into compact generated `.css` files.
    /// 4. Updates cache with new artifacts.
    ///
    /// # Errors
    ///
    /// Returns an error if the input directory, source CSS, output directory, or
    /// cache index cannot be read or written.
    pub fn compile_to_css(&self, cache: &mut BuildCache) -> Result<Vec<BuildArtifact>> {
        self.ensure_input_dir()?;
        self.ensure_output_dir()?;

        let css_files = self.discover_css_files()?;

        if css_files.is_empty() {
            return Ok(Vec::new());
        }

        let mut artifacts = Vec::new();

        for css_file in css_files {
            let artifact = self.process_css_file_as_generated_css(&css_file, cache)?;
            artifacts.push(artifact);
        }

        Ok(artifacts)
    }

    /// Compile all CSS files to Binary Dawn format.
    ///
    /// This method:
    /// 1. Discovers all CSS files in the input directory.
    /// 2. Checks cache for each file.
    /// 3. Compiles changed files using the dx-style CLI.
    /// 4. Generates `.dxbd` binary format files.
    /// 5. Updates cache with new artifacts.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The input directory doesn't exist.
    /// - CSS files cannot be read.
    /// - dx-style CLI fails.
    /// - Output files cannot be written.
    pub fn compile_to_binary(&self, cache: &mut BuildCache) -> Result<Vec<BuildArtifact>> {
        self.ensure_input_dir()?;
        self.ensure_output_dir()?;

        let css_files = self.discover_css_files()?;

        if css_files.is_empty() {
            return Ok(Vec::new());
        }

        let mut artifacts = Vec::new();

        for css_file in css_files {
            let artifact = self.process_css_file_as_binary(&css_file, cache)?;
            artifacts.push(artifact);
        }

        Ok(artifacts)
    }

    /// Discover all CSS files in the input directory.
    ///
    /// # Errors
    ///
    /// Returns an error if walking the input directory fails.
    pub fn discover_css_files(&self) -> Result<Vec<PathBuf>> {
        let mut css_files = Vec::new();

        for entry in WalkDir::new(&self.config.input_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("css") {
                css_files.push(path.to_path_buf());
            }
        }

        Ok(css_files)
    }

    /// Generate a tiny stylesheet loader for CSS-first delivery.
    ///
    /// The default loader uses native CSS. Binary sidecar experiments can layer on
    /// top without changing the normal browser path.
    ///
    /// # Errors
    ///
    /// Returns an error if the loader template cannot be generated.
    pub fn generate_loader(&self) -> Result<String> {
        Ok(r#"
// DX Style generated CSS loader
// Native CSS is the default browser path. Binary sidecars are optional.

export function loadGeneratedStylesheet(url, options = {}) {
    const existing = document.querySelector(`link[data-dx-style="${url}"]`);
    if (existing) {
        return existing;
    }

    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href = url;
    link.dataset.dxStyle = url;

    if (options.media) {
        link.media = options.media;
    }

    document.head.appendChild(link);
    return link;
}

export async function loadBinaryStyleSidecar(url, decode) {
    const response = await fetch(url);
    if (!response.ok) {
        throw new Error(`DX style sidecar failed: ${response.status}`);
    }

    const buffer = await response.arrayBuffer();
    return decode(buffer);
}
"#
        .to_string())
    }

    /// Get all tracked style artifacts.
    pub fn get_artifacts(&self, _cache: &BuildCache) -> Vec<StyleArtifactMetadata> {
        Vec::new()
    }

    fn ensure_input_dir(&self) -> Result<()> {
        if !self.config.input_dir.exists() {
            return Err(BuildError::FileNotFound(self.config.input_dir.clone()));
        }

        Ok(())
    }

    fn ensure_output_dir(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config.output_dir).map_err(|e| BuildError::Io {
            path: self.config.output_dir.clone(),
            source: e,
        })
    }

    fn process_css_file_as_generated_css(
        &self,
        css_file: &Path,
        cache: &mut BuildCache,
    ) -> Result<BuildArtifact> {
        let cache_key = CacheKey::from_file(css_file, "style-generated-css-v1".to_string())?;

        if let Some(cached) = cache.get(&cache_key) {
            if cached.is_valid() {
                return Ok(BuildArtifact {
                    artifact_type: ArtifactType::Style,
                    path: cached.output_path.clone(),
                    hash: cached.output_hash.clone(),
                    size: cached.size,
                });
            }
        }

        let output_path = self.get_css_output_path(css_file)?;
        self.write_generated_css_file(css_file, &output_path)?;

        let output_data = std::fs::read(&output_path).map_err(|e| BuildError::Io {
            path: output_path.clone(),
            source: e,
        })?;
        let output_hash = content_hash(&output_data);
        let size = output_data.len();

        let cache_entry =
            CacheEntry::new(cache_key, output_path.clone(), output_hash.clone(), size);
        cache.insert(cache_entry)?;

        Ok(BuildArtifact {
            artifact_type: ArtifactType::Style,
            path: output_path,
            hash: output_hash,
            size,
        })
    }

    fn process_css_file_as_binary(
        &self,
        css_file: &Path,
        cache: &mut BuildCache,
    ) -> Result<BuildArtifact> {
        let cache_key = CacheKey::from_file(css_file, "style-binary-dawn".to_string())?;

        if let Some(cached) = cache.get(&cache_key) {
            if cached.is_valid() {
                return Ok(BuildArtifact {
                    artifact_type: ArtifactType::Style,
                    path: cached.output_path.clone(),
                    hash: cached.output_hash.clone(),
                    size: cached.size,
                });
            }
        }

        let output_path = self.get_output_path(css_file)?;
        self.compile_css_file(css_file, &output_path)?;

        let output_data = std::fs::read(&output_path).map_err(|e| BuildError::Io {
            path: output_path.clone(),
            source: e,
        })?;
        let output_hash = content_hash(&output_data);
        let size = output_data.len();

        let cache_entry =
            CacheEntry::new(cache_key, output_path.clone(), output_hash.clone(), size);
        cache.insert(cache_entry)?;

        Ok(BuildArtifact {
            artifact_type: ArtifactType::Style,
            path: output_path,
            hash: output_hash,
            size,
        })
    }

    /// Get the Binary Dawn output path for a CSS file.
    fn get_output_path(&self, css_file: &Path) -> Result<PathBuf> {
        let relative = pathdiff::diff_paths(css_file, &self.config.input_dir).ok_or_else(|| {
            BuildError::Style(format!(
                "Failed to compute relative path for {:?}",
                css_file
            ))
        })?;

        let mut output_path = self.config.output_dir.join(relative);
        output_path.set_extension("dxbd");

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| BuildError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        Ok(output_path)
    }

    /// Get the generated CSS output path for a CSS file.
    fn get_css_output_path(&self, css_file: &Path) -> Result<PathBuf> {
        let relative = pathdiff::diff_paths(css_file, &self.config.input_dir).ok_or_else(|| {
            BuildError::Style(format!(
                "Failed to compute relative path for {:?}",
                css_file
            ))
        })?;

        let mut output_path = self.config.output_dir.join(relative);
        output_path.set_extension("css");

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| BuildError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        Ok(output_path)
    }

    fn write_generated_css_file(&self, input: &Path, output: &Path) -> Result<()> {
        let source = std::fs::read_to_string(input).map_err(|e| BuildError::Io {
            path: input.to_path_buf(),
            source: e,
        })?;
        let generated = minify_css(&source);

        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(|e| BuildError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        std::fs::write(output, generated).map_err(|e| BuildError::Io {
            path: output.to_path_buf(),
            source: e,
        })
    }

    fn compile_css_file(&self, input: &Path, output: &Path) -> Result<()> {
        let mut cmd = Command::new("cargo");
        cmd.arg("run")
            .arg("--bin")
            .arg("dx-style")
            .arg("--")
            .arg("compile")
            .arg(input)
            .arg("--output")
            .arg(output)
            .arg("--format")
            .arg("binary");

        if self.config.auto_grouping {
            cmd.arg("--auto-group")
                .arg("--similarity-threshold")
                .arg(self.config.similarity_threshold.to_string());
        }

        cmd.arg("--compression")
            .arg(self.config.compression_level.to_string());

        let output_result = cmd
            .output()
            .map_err(|e| BuildError::Style(format!("Failed to execute dx-style CLI: {}", e)))?;

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(BuildError::Style(format!(
                "dx-style compilation failed for {:?}: {}",
                input, stderr
            )));
        }

        Ok(())
    }
}

fn minify_css(css: &str) -> String {
    let without_comments = remove_css_comments(css);
    let mut output = String::with_capacity(without_comments.len());
    let mut quote = None;
    let mut escaped = false;
    let mut pending_space = false;

    for ch in without_comments.chars() {
        if let Some(quote_char) = quote {
            output.push(ch);

            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote_char {
                quote = None;
            }

            continue;
        }

        if ch == '"' || ch == '\'' {
            if pending_space && should_keep_space(output.chars().last(), ch) {
                output.push(' ');
            }
            pending_space = false;
            quote = Some(ch);
            output.push(ch);
            continue;
        }

        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }

        if is_css_separator(ch) {
            while output.ends_with(' ') {
                output.pop();
            }
            output.push(ch);
            pending_space = false;
            continue;
        }

        if pending_space && should_keep_space(output.chars().last(), ch) {
            output.push(' ');
        }

        pending_space = false;
        output.push(ch);
    }

    output.trim().to_string()
}

fn remove_css_comments(css: &str) -> String {
    let mut output = String::with_capacity(css.len());
    let mut chars = css.chars().peekable();
    let mut quote = None;
    let mut escaped = false;

    while let Some(ch) = chars.next() {
        if let Some(quote_char) = quote {
            output.push(ch);

            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote_char {
                quote = None;
            }

            continue;
        }

        if ch == '"' || ch == '\'' {
            quote = Some(ch);
            output.push(ch);
            continue;
        }

        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            let mut previous = '\0';
            for comment_char in chars.by_ref() {
                if previous == '*' && comment_char == '/' {
                    break;
                }
                previous = comment_char;
            }
            continue;
        }

        output.push(ch);
    }

    output
}

fn should_keep_space(previous: Option<char>, next: char) -> bool {
    matches!(previous, Some(ch) if !is_css_separator(ch) && !is_css_separator(next))
}

fn is_css_separator(ch: char) -> bool {
    matches!(ch, '{' | '}' | ':' | ';' | ',' | '>' | '+' | '~')
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_style_config_default() {
        let config = StyleConfig::default();
        assert_eq!(config.input_dir, PathBuf::from("www/styles"));
        assert_eq!(config.output_dir, PathBuf::from("dist/styles"));
        assert_eq!(config.delivery_mode, StyleDeliveryMode::GeneratedCss);
        assert!(config.auto_grouping);
        assert_eq!(config.similarity_threshold, 0.7);
        assert_eq!(config.compression_level, 9);
    }

    #[test]
    fn test_style_processor_creation() {
        let config = StyleConfig::default();
        let processor = StyleProcessor::new(config);
        assert!(processor.config.auto_grouping);
    }

    #[test]
    fn test_discover_css_files_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let styles_dir = temp_dir.path().join("styles");
        std::fs::create_dir(&styles_dir).unwrap();

        let config = StyleConfig {
            input_dir: styles_dir,
            output_dir: temp_dir.path().join("dist"),
            ..Default::default()
        };

        let processor = StyleProcessor::new(config);
        let files = processor.discover_css_files();
        assert!(files.is_ok());
        assert_eq!(files.unwrap().len(), 0);
    }

    #[test]
    fn test_discover_css_files_with_css() {
        let temp_dir = TempDir::new().unwrap();
        let styles_dir = temp_dir.path().join("styles");
        std::fs::create_dir(&styles_dir).unwrap();

        std::fs::write(styles_dir.join("main.css"), "body { color: red; }").unwrap();
        std::fs::write(styles_dir.join("theme.css"), ".theme { background: blue; }").unwrap();

        let config = StyleConfig {
            input_dir: styles_dir,
            output_dir: temp_dir.path().join("dist"),
            ..Default::default()
        };

        let processor = StyleProcessor::new(config);
        let files = processor.discover_css_files();
        assert!(files.is_ok());
        let files = files.unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_discover_css_files_nested() {
        let temp_dir = TempDir::new().unwrap();
        let styles_dir = temp_dir.path().join("styles");
        std::fs::create_dir(&styles_dir).unwrap();

        let components_dir = styles_dir.join("components");
        std::fs::create_dir(&components_dir).unwrap();

        std::fs::write(styles_dir.join("main.css"), "body { color: red; }").unwrap();
        std::fs::write(components_dir.join("button.css"), ".btn { padding: 10px; }").unwrap();

        let config = StyleConfig {
            input_dir: styles_dir,
            output_dir: temp_dir.path().join("dist"),
            ..Default::default()
        };

        let processor = StyleProcessor::new(config);
        let files = processor.discover_css_files();
        assert!(files.is_ok());
        let files = files.unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_get_output_path() {
        let temp_dir = TempDir::new().unwrap();
        let styles_dir = temp_dir.path().join("styles");
        let output_dir = temp_dir.path().join("dist");

        let config = StyleConfig {
            input_dir: styles_dir.clone(),
            output_dir: output_dir.clone(),
            ..Default::default()
        };

        let processor = StyleProcessor::new(config);

        let input = styles_dir.join("main.css");
        let output = processor.get_output_path(&input);
        assert!(output.is_ok());
        let output = output.unwrap();
        assert_eq!(output, output_dir.join("main.dxbd"));
    }

    #[test]
    fn test_get_output_path_nested() {
        let temp_dir = TempDir::new().unwrap();
        let styles_dir = temp_dir.path().join("styles");
        let output_dir = temp_dir.path().join("dist");

        let config = StyleConfig {
            input_dir: styles_dir.clone(),
            output_dir: output_dir.clone(),
            ..Default::default()
        };

        let processor = StyleProcessor::new(config);

        let input = styles_dir.join("components").join("button.css");
        let output = processor.get_output_path(&input);
        assert!(output.is_ok());
        let output = output.unwrap();
        assert_eq!(output, output_dir.join("components").join("button.dxbd"));
    }

    #[test]
    fn test_get_css_output_path() {
        let temp_dir = TempDir::new().unwrap();
        let styles_dir = temp_dir.path().join("styles");
        let output_dir = temp_dir.path().join("dist");

        let config = StyleConfig {
            input_dir: styles_dir.clone(),
            output_dir: output_dir.clone(),
            ..Default::default()
        };

        let processor = StyleProcessor::new(config);

        let input = styles_dir.join("components").join("button.css");
        let output = processor.get_css_output_path(&input);
        assert!(output.is_ok());
        let output = output.unwrap();
        assert_eq!(output, output_dir.join("components").join("button.css"));
    }

    #[test]
    fn test_generate_loader() {
        let config = StyleConfig::default();
        let processor = StyleProcessor::new(config);
        let loader = processor.generate_loader();
        assert!(loader.is_ok());
        let loader = loader.unwrap();
        assert!(loader.contains("loadGeneratedStylesheet"));
        assert!(loader.contains("loadBinaryStyleSidecar"));
    }

    #[test]
    fn test_minify_css_keeps_selector_spaces_and_strings() {
        let css = r#"
          /* launch skin */
          .card .title {
            color: red;
            content: "hello   world";
            padding: 4px 8px;
          }
        "#;

        let minified = minify_css(css);

        assert!(!minified.contains("launch skin"));
        assert!(minified.contains(".card .title{"));
        assert!(minified.contains(r#"content:"hello   world";"#));
        assert!(minified.contains("padding:4px 8px;"));
    }

    #[test]
    fn test_compile_to_css_generates_browser_css() {
        let temp_dir = TempDir::new().unwrap();
        let styles_dir = temp_dir.path().join("styles");
        let output_dir = temp_dir.path().join("dist");
        std::fs::create_dir(&styles_dir).unwrap();
        std::fs::write(
            styles_dir.join("main.css"),
            "/* demo */\n.card { color: red; padding: 4px 8px; }\n",
        )
        .unwrap();

        let config = StyleConfig {
            input_dir: styles_dir,
            output_dir: output_dir.clone(),
            ..Default::default()
        };

        let processor = StyleProcessor::new(config);
        let mut cache = BuildCache::new(temp_dir.path()).unwrap();
        let artifacts = processor.compile_for_browser(&mut cache).unwrap();

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].path, output_dir.join("main.css"));

        let generated = std::fs::read_to_string(&artifacts[0].path).unwrap();
        assert_eq!(generated, ".card{color:red;padding:4px 8px;}");
    }

    #[test]
    fn test_compile_to_binary_missing_input_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config = StyleConfig {
            input_dir: temp_dir.path().join("nonexistent"),
            output_dir: temp_dir.path().join("dist"),
            ..Default::default()
        };

        let processor = StyleProcessor::new(config);
        let mut cache = BuildCache::new(temp_dir.path()).unwrap();

        let result = processor.compile_to_binary(&mut cache);
        assert!(result.is_err());
        match result {
            Err(BuildError::FileNotFound(_)) => {}
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_compile_to_binary_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let styles_dir = temp_dir.path().join("styles");
        std::fs::create_dir(&styles_dir).unwrap();

        let config = StyleConfig {
            input_dir: styles_dir,
            output_dir: temp_dir.path().join("dist"),
            delivery_mode: StyleDeliveryMode::BinaryDawn,
            ..Default::default()
        };

        let processor = StyleProcessor::new(config);
        let mut cache = BuildCache::new(temp_dir.path()).unwrap();

        let result = processor.compile_to_binary(&mut cache);
        assert!(result.is_ok());
        let artifacts = result.unwrap();
        assert_eq!(artifacts.len(), 0);
    }
}
