use super::*;

// ============================================================================
// Ecosystem Configuration
// ============================================================================

/// Ecosystem configuration loaded from dx.config
#[derive(Debug, Clone)]
pub struct EcosystemConfig {
    /// Project root directory
    pub project_root: PathBuf,
    /// Loaded www configuration
    pub www_config: DxWwwConfig,
    /// Icon configuration (if enabled)
    pub icon_config: Option<IconConfig>,
    /// Font configuration (if enabled)
    pub font_config: Option<FontConfig>,
    /// Media configuration (if enabled)
    pub media_config: Option<MediaConfig>,
}

/// Icon processing configuration
#[derive(Debug, Clone)]
pub struct IconConfig {
    /// Icon sets to include
    pub sets: Vec<String>,
    /// Custom icon directory
    pub custom_dir: Option<PathBuf>,
    /// Enable tree-shaking
    pub tree_shake: bool,
}

impl Default for IconConfig {
    fn default() -> Self {
        Self {
            sets: Vec::new(),
            custom_dir: None,
            tree_shake: true,
        }
    }
}

/// Font processing configuration
#[derive(Debug, Clone)]
pub struct FontConfig {
    /// Font families to include
    pub families: Vec<FontFamily>,
    /// Enable subsetting
    pub subset: bool,
    /// Preload fonts
    pub preload: bool,
}

/// Font family definition
#[derive(Debug, Clone)]
pub struct FontFamily {
    /// Font family name
    pub name: String,
    /// Font weights to include
    pub weights: Vec<u16>,
    /// Is this a variable font
    pub variable: bool,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            families: Vec::new(),
            subset: true,
            preload: true,
        }
    }
}

/// Media processing configuration
#[derive(Debug, Clone)]
pub struct MediaConfig {
    /// Image output formats
    pub image_formats: Vec<String>,
    /// Image quality (1-100)
    pub quality: u8,
    /// Generate blur placeholders
    pub blur_placeholder: bool,
    /// Responsive breakpoints
    pub breakpoints: Vec<u32>,
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            image_formats: vec!["webp".to_string(), "avif".to_string()],
            quality: 85,
            blur_placeholder: true,
            breakpoints: vec![640, 768, 1024, 1280, 1536],
        }
    }
}

// ============================================================================
// Initialization
// ============================================================================

/// Initialize ecosystem integrations from project root
pub fn init(project_root: &Path) -> Result<EcosystemConfig> {
    // Load configuration
    let www_config =
        www_config::load_config_from_root(project_root).unwrap_or_else(|_| DxWwwConfig::default());

    // Build icon config from www_config
    let icon_config = www_config.assets.icons.as_ref().map(|icons| IconConfig {
        sets: icons.sets.clone(),
        custom_dir: icons.custom_dir.clone(),
        tree_shake: true,
    });

    // Build font config from www_config
    let font_config = www_config.assets.fonts.as_ref().map(|fonts| FontConfig {
        families: fonts
            .families
            .iter()
            .map(|f| FontFamily {
                name: f.name.clone(),
                weights: f.weights.clone(),
                variable: f.variable,
            })
            .collect(),
        subset: fonts.subset,
        preload: true,
    });

    // Build media config from www_config
    let media_config = www_config.assets.media.as_ref().and_then(|media| {
        media.images.as_ref().map(|images| MediaConfig {
            image_formats: images.formats.clone(),
            quality: images.quality,
            blur_placeholder: images.blur_placeholder,
            breakpoints: vec![640, 768, 1024, 1280, 1536],
        })
    });

    Ok(EcosystemConfig {
        project_root: project_root.to_path_buf(),
        www_config,
        icon_config,
        font_config,
        media_config,
    })
}

/// Initialize with default configuration (for testing)
pub fn init_default() -> EcosystemConfig {
    EcosystemConfig {
        project_root: PathBuf::from("."),
        www_config: DxWwwConfig::default(),
        icon_config: Some(IconConfig::default()),
        font_config: Some(FontConfig::default()),
        media_config: Some(MediaConfig::default()),
    }
}

// ============================================================================
// Icon Processing
// ============================================================================
