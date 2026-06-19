const FORGE_UI_REGISTRY_SCHEMA_VERSION: &str = "dx.forge.ui.registry.v1";
const FORGE_UI_REGISTRY_MAX_INCLUDE_DEPTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryCatalog {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub items: Vec<DxForgeUiRegistryItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxForgeUiRegistryItem {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_library: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub item_type: DxForgeUiRegistryItemType,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub dev_dependencies: Vec<String>,
    #[serde(default)]
    pub registry_dependencies: Vec<String>,
    #[serde(default)]
    pub files: Vec<DxForgeUiRegistryItemFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tailwind: Option<DxForgeUiRegistryTailwind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub css_vars: Option<DxForgeUiRegistryCssVars>,
    #[serde(default)]
    pub css: BTreeMap<String, serde_json::Value>,
    #[serde(default)]
    pub env_vars: BTreeMap<String, String>,
    #[serde(default)]
    pub meta: BTreeMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<DxForgeUiRegistryRawConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<DxForgeUiRegistryFont>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxForgeUiRegistryItemFile {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "type")]
    pub file_type: DxForgeUiRegistryItemType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DxForgeUiRegistryItemType {
    #[serde(rename = "registry:lib")]
    Lib,
    #[serde(rename = "registry:block")]
    Block,
    #[serde(rename = "registry:component")]
    Component,
    #[serde(rename = "registry:ui")]
    Ui,
    #[serde(rename = "registry:hook")]
    Hook,
    #[serde(rename = "registry:page")]
    Page,
    #[serde(rename = "registry:file")]
    File,
    #[serde(rename = "registry:theme")]
    Theme,
    #[serde(rename = "registry:style")]
    Style,
    #[serde(rename = "registry:item")]
    Item,
    #[serde(rename = "registry:base")]
    Base,
    #[serde(rename = "registry:font")]
    Font,
    #[serde(rename = "registry:example")]
    Example,
    #[serde(rename = "registry:internal")]
    Internal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DxForgeUiRegistryTailwind {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<DxForgeUiRegistryTailwindConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DxForgeUiRegistryTailwindConfig {
    #[serde(default)]
    pub content: Vec<String>,
    #[serde(default)]
    pub theme: BTreeMap<String, serde_json::Value>,
    #[serde(default)]
    pub plugins: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DxForgeUiRegistryCssVars {
    #[serde(default)]
    pub theme: BTreeMap<String, String>,
    #[serde(default)]
    pub light: BTreeMap<String, String>,
    #[serde(default)]
    pub dark: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryFont {
    pub family: String,
    pub provider: DxForgeUiRegistryFontProvider,
    #[serde(rename = "import")]
    pub import_name: String,
    pub variable: String,
    #[serde(default)]
    pub weight: Vec<String>,
    #[serde(default)]
    pub subsets: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeUiRegistryFontProvider {
    Google,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DxForgeUiRegistryRawConfig {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsc: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tsx: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tailwind: Option<DxForgeUiRegistryRawTailwindConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_library: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu_accent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<DxForgeUiRegistryRawAliases>,
    #[serde(default)]
    pub registries: BTreeMap<String, DxForgeUiRegistryConfigItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DxForgeUiRegistryRawTailwindConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub css_variables: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DxForgeUiRegistryRawAliases {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utils: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ui: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lib: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DxForgeUiRegistryConfigItem {
    Url(String),
    Advanced {
        url: String,
        #[serde(default)]
        params: BTreeMap<String, String>,
        #[serde(default)]
        headers: BTreeMap<String, String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryValidationReport {
    pub schema_version: String,
    pub valid: bool,
    pub item_count: usize,
    pub file_count: usize,
    pub include_count: usize,
    pub dependency_count: usize,
    pub dev_dependency_count: usize,
    pub registry_dependency_count: usize,
    pub env_var_count: usize,
    pub docs_count: usize,
    pub item_types: BTreeMap<DxForgeUiRegistryItemType, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryContentEmbeddingReport {
    pub schema_version: String,
    pub source_root: PathBuf,
    pub item_count: usize,
    pub file_count: usize,
    pub embedded_file_count: usize,
    pub preserved_inline_content_file_count: usize,
    pub files: Vec<DxForgeUiRegistryContentEmbeddingFileEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryContentEmbeddingFileEvidence {
    pub item_name: String,
    pub registry_path: String,
    pub file_type: DxForgeUiRegistryItemType,
    pub status: DxForgeUiRegistryContentEmbeddingStatus,
    pub source_path: Option<String>,
    pub byte_count: usize,
    pub hash_algorithm: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeUiRegistryContentEmbeddingStatus {
    EmbeddedFromSource,
    PreservedInlineContent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryItemPlanReport {
    pub schema_version: String,
    pub item_name: String,
    pub item_type: DxForgeUiRegistryItemType,
    pub project: PathBuf,
    pub passed: bool,
    pub score: u8,
    pub file_count: usize,
    pub write_file_count: usize,
    pub inline_content_file_count: usize,
    pub missing_inline_content_count: usize,
    pub dependency_count: usize,
    pub dev_dependency_count: usize,
    pub registry_dependency_count: usize,
    pub external_registry_references: Vec<DxForgeUiRegistryExternalReferenceEvidence>,
    pub env_var_count: usize,
    pub css_var_count: usize,
    pub css_rule_count: usize,
    pub tailwind_config_present: bool,
    pub font_present: bool,
    pub config_present: bool,
    pub registry_dependency_order: Vec<String>,
    pub registry_dependency_edges: Vec<DxForgeUiRegistryDependencyEdge>,
    pub no_package_manager_execution: bool,
    pub forbidden_commands: Vec<String>,
    pub files: Vec<DxForgeUiRegistryPlannedFile>,
    pub decisions: Vec<DxForgeUiRegistryPlanDecision>,
    pub warnings: Vec<String>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryExternalReferenceEvidence {
    pub from_item: String,
    pub reference: String,
    pub source_kind: DxForgeUiRegistryExternalReferenceKind,
    pub item_name: String,
    pub requires_bridge: bool,
    pub no_network_request: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeUiRegistryExternalReferenceKind {
    RemoteRegistryUrl,
    NamespacedRegistry,
    GithubSourceRegistry,
    LocalRegistryFile,
    UnknownExternalRegistry,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryItemDocsReport {
    pub schema_version: String,
    pub item_name: String,
    pub item_type: DxForgeUiRegistryItemType,
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub docs: Option<String>,
    pub has_docs: bool,
    pub file_count: usize,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub registry_dependencies: Vec<String>,
    pub registry_dependency_order: Vec<String>,
    pub files: Vec<DxForgeUiRegistryPlannedFile>,
    pub env_vars: Vec<String>,
    pub css_var_count: usize,
    pub css_rule_count: usize,
    pub tailwind_config_present: bool,
    pub font_present: bool,
    pub config_present: bool,
    pub base_style: Option<String>,
    pub base_icon_library: Option<String>,
    pub base_color: Option<String>,
    pub base_theme: Option<String>,
    pub categories: Vec<String>,
    pub no_package_manager_execution: bool,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DxForgeUiRegistryResolvedReference {
    pub catalog: DxForgeUiRegistryCatalog,
    pub registry_file: PathBuf,
    pub item_name: String,
    pub source_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryPlannedFile {
    pub item_name: String,
    pub source_path: String,
    pub target_path: String,
    pub file_type: DxForgeUiRegistryItemType,
    pub has_inline_content: bool,
    pub action: DxForgeUiRegistryPlanAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeUiRegistryPlanAction {
    Materialize,
    NeedsReviewedContent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryDependencyEdge {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUiRegistryPlanDecision {
    pub subject: String,
    pub decision: DxForgeUiRegistryPlanDecisionKind,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeUiRegistryPlanDecisionKind {
    Materialize,
    ResolveRegistryDependency,
    BridgeDependency,
    IgnoreDevDependency,
    RequireEnvironment,
    MergeStyle,
    MergeConfig,
    RegisterFont,
}

pub fn parse_forge_ui_registry_json(json: &str) -> Result<DxForgeUiRegistryCatalog> {
    let registry: DxForgeUiRegistryCatalog =
        serde_json::from_str(json).context("parse Forge UI registry json")?;
    validate_registry_has_items_or_includes(&registry)?;
    Ok(registry)
}

fn forge_ui_registry_item_has_top_level_base_metadata(item: &DxForgeUiRegistryItem) -> bool {
    item.style.is_some()
        || item.icon_library.is_some()
        || item.base_color.is_some()
        || item.theme.is_some()
}

fn forge_ui_registry_item_has_base_configuration(item: &DxForgeUiRegistryItem) -> bool {
    item.config.is_some() || forge_ui_registry_item_has_top_level_base_metadata(item)
}

fn validate_forge_ui_registry_config_aliases(
    item_name: &str,
    config: &DxForgeUiRegistryRawConfig,
) -> Result<()> {
    let Some(aliases) = config.aliases.as_ref() else {
        return Ok(());
    };
    for (alias_name, alias_value) in [
        ("components", aliases.components.as_deref()),
        ("utils", aliases.utils.as_deref()),
        ("ui", aliases.ui.as_deref()),
        ("lib", aliases.lib.as_deref()),
        ("hooks", aliases.hooks.as_deref()),
    ] {
        let Some(alias_value) = alias_value else {
            continue;
        };
        let target = forge_ui_registry_alias_target_path(alias_value).with_context(|| {
            format!("validate Forge UI registry alias `{alias_name}` on item `{item_name}`")
        })?;
        validate_project_relative_path(&target).with_context(|| {
            format!(
                "validate Forge UI registry alias `{alias_name}` target `{alias_value}` on item `{item_name}`"
            )
        })?;
    }
    Ok(())
}

fn validate_forge_ui_registry_env_vars(
    item_name: &str,
    env_vars: &BTreeMap<String, String>,
) -> Result<()> {
    for (name, value) in env_vars {
        if !forge_ui_registry_env_var_name(name) {
            bail!(
                "Forge UI registry item `{item_name}` has unsafe envVars entry `{name}`; env var names must use A-Z, 0-9, and underscores, and must not start with a digit"
            );
        }
        if value.contains('\r') || value.contains('\n') || value.contains('\0') {
            bail!("Forge UI registry item `{item_name}` has unsafe envVars value for `{name}`");
        }
    }
    Ok(())
}

fn forge_ui_registry_env_var_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_uppercase()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_uppercase() || ch.is_ascii_digit())
}

fn validate_forge_ui_registry_tailwind(
    item_name: &str,
    tailwind: &Option<DxForgeUiRegistryTailwind>,
) -> Result<()> {
    let Some(config) = tailwind
        .as_ref()
        .and_then(|tailwind| tailwind.config.as_ref())
    else {
        return Ok(());
    };
    let mut seen = BTreeSet::new();
    for plugin in &config.plugins {
        let trimmed = plugin.trim();
        if trimmed.is_empty() {
            bail!(
                "Forge UI registry item `{item_name}` has an empty tailwind.config.plugins entry"
            );
        }
        if plugin != trimmed || forge_ui_registry_external_dependency_entry_is_unsafe(trimmed) {
            bail!(
                "Forge UI registry item `{item_name}` has unsafe tailwind.config.plugins entry `{plugin}`; Tailwind plugin metadata must be package names, not package-manager commands"
            );
        }
        if !seen.insert(trimmed.to_string()) {
            bail!(
                "Forge UI registry item `{item_name}` declares duplicate tailwind.config.plugins entry `{trimmed}`"
            );
        }
    }
    Ok(())
}

fn validate_forge_ui_registry_config_registries(
    item_name: &str,
    config: &DxForgeUiRegistryRawConfig,
) -> Result<()> {
    for (registry_name, registry_config) in &config.registries {
        validate_forge_ui_registry_config_registry_name(item_name, registry_name)?;
        validate_forge_ui_registry_config_registry_target(
            item_name,
            registry_name,
            registry_config,
        )?;
    }
    Ok(())
}

fn validate_forge_ui_registry_config_registry_name(
    item_name: &str,
    registry_name: &str,
) -> Result<()> {
    let trimmed = registry_name.trim();
    if trimmed.is_empty() || trimmed != registry_name {
        bail!(
            "Forge UI registry item `{item_name}` has an invalid configured registry name `{registry_name}`"
        );
    }
    let unscoped = trimmed.strip_prefix('@').unwrap_or(trimmed);
    if unscoped.is_empty()
        || unscoped.chars().any(|ch| {
            ch.is_whitespace() || matches!(ch, '/' | '\\' | ':' | ';' | '&' | '|' | '<' | '>' | '`')
        })
    {
        bail!(
            "Forge UI registry item `{item_name}` has unsafe configured registry `{registry_name}`; registry names must be simple aliases"
        );
    }
    Ok(())
}

fn validate_forge_ui_registry_config_registry_target(
    item_name: &str,
    registry_name: &str,
    registry_config: &DxForgeUiRegistryConfigItem,
) -> Result<()> {
    let target = configured_forge_ui_registry_reference_url(registry_config).trim();
    if target.is_empty() {
        bail!(
            "Forge UI registry item `{item_name}` has empty configured registry `{registry_name}` target"
        );
    }
    if forge_ui_registry_config_registry_target_is_command(target) {
        bail!(
            "Forge UI registry item `{item_name}` has unsafe configured registry `{registry_name}` target `{target}`; package-manager commands are not registry targets"
        );
    }
    if target.starts_with("http://") || target.starts_with("https://") {
        return Ok(());
    }
    if target.contains("://") {
        bail!(
            "Forge UI registry item `{item_name}` has unsupported configured registry `{registry_name}` target `{target}`; expected http(s) registry URL or project-relative registry.json"
        );
    }

    let normalized = normalize_forge_ui_registry_path(target).with_context(|| {
        format!("validate Forge UI registry `{registry_name}` target on item `{item_name}`")
    })?;
    validate_project_relative_path(&normalized).with_context(|| {
        format!(
            "validate Forge UI registry `{registry_name}` target `{target}` on item `{item_name}`"
        )
    })?;
    if !normalized
        .rsplit('/')
        .next()
        .is_some_and(|name| name == "registry.json")
    {
        bail!(
            "Forge UI registry item `{item_name}` configured registry `{registry_name}` target `{target}` must name registry.json"
        );
    }
    Ok(())
}

fn validate_forge_ui_registry_advanced_registry_metadata(
    item_name: &str,
    registry_name: &str,
    registry_config: &DxForgeUiRegistryConfigItem,
) -> Result<()> {
    let DxForgeUiRegistryConfigItem::Advanced {
        params, headers, ..
    } = registry_config
    else {
        return Ok(());
    };

    for (name, value) in params {
        validate_forge_ui_registry_config_param_name(item_name, registry_name, name)?;
        validate_forge_ui_registry_config_metadata_value(
            item_name,
            registry_name,
            "param",
            name,
            value,
        )?;
    }
    for (name, value) in headers {
        validate_forge_ui_registry_config_header_name(item_name, registry_name, name)?;
        validate_forge_ui_registry_config_metadata_value(
            item_name,
            registry_name,
            "header",
            name,
            value,
        )?;
    }
    Ok(())
}

fn validate_forge_ui_registry_config_param_name(
    item_name: &str,
    registry_name: &str,
    name: &str,
) -> Result<()> {
    if !forge_ui_registry_simple_token(name) {
        bail!(
            "Forge UI registry item `{item_name}` configured registry `{registry_name}` has unsafe param `{name}`"
        );
    }
    Ok(())
}

fn validate_forge_ui_registry_config_header_name(
    item_name: &str,
    registry_name: &str,
    name: &str,
) -> Result<()> {
    if !forge_ui_registry_http_header_name(name) {
        bail!(
            "Forge UI registry item `{item_name}` configured registry `{registry_name}` has unsafe header `{name}`"
        );
    }
    Ok(())
}

fn validate_forge_ui_registry_config_metadata_value(
    item_name: &str,
    registry_name: &str,
    field_kind: &str,
    name: &str,
    value: &str,
) -> Result<()> {
    if value.contains('\r') || value.contains('\n') || value.contains('\0') {
        bail!(
            "Forge UI registry item `{item_name}` configured registry `{registry_name}` has unsafe {field_kind} `{name}` value"
        );
    }
    Ok(())
}

fn forge_ui_registry_simple_token(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.'))
}

fn forge_ui_registry_http_header_name(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && value.chars().all(|ch| {
            ch.is_ascii_alphanumeric()
                || matches!(
                    ch,
                    '!' | '#'
                        | '$'
                        | '%'
                        | '&'
                        | '\''
                        | '*'
                        | '+'
                        | '-'
                        | '.'
                        | '^'
                        | '_'
                        | '`'
                        | '|'
                        | '~'
                )
        })
}

fn forge_ui_registry_config_registry_target_is_command(target: &str) -> bool {
    let lower = target.to_ascii_lowercase();
    lower.starts_with("npm ")
        || lower.starts_with("pnpm ")
        || lower.starts_with("yarn ")
        || lower.starts_with("bun ")
        || lower.starts_with("npx ")
        || lower.starts_with("cargo ")
        || lower.starts_with("pip ")
        || lower.starts_with("uv ")
        || lower.starts_with("go get ")
        || lower.starts_with("dart ")
        || target
            .chars()
            .any(|ch| matches!(ch, ';' | '&' | '|' | '<' | '>' | '`'))
}

pub fn validate_forge_ui_registry_catalog(
    registry: &DxForgeUiRegistryCatalog,
) -> Result<DxForgeUiRegistryValidationReport> {
    validate_registry_has_items_or_includes(registry)?;
    let mut names = BTreeSet::new();
    let mut file_count = 0;
    let mut dependency_count = 0;
    let mut dev_dependency_count = 0;
    let mut registry_dependency_count = 0;
    let mut env_var_count = 0;
    let mut docs_count = 0;
    let mut item_types = BTreeMap::new();

    for include in &registry.include {
        validate_forge_ui_registry_include_path(include)?;
    }

    for item in &registry.items {
        if item.name.trim().is_empty() {
            bail!("Forge UI registry item name cannot be empty");
        }
        if !names.insert(item.name.clone()) {
            bail!("duplicate Forge UI registry item `{}`", item.name);
        }
        if item.item_type == DxForgeUiRegistryItemType::Base
            && !forge_ui_registry_item_has_base_configuration(item)
        {
            bail!(
                "registry:base item `{}` should define config or top-level base metadata",
                item.name
            );
        }
        if item.item_type != DxForgeUiRegistryItemType::Base
            && forge_ui_registry_item_has_top_level_base_metadata(item)
        {
            bail!(
                "Forge UI registry item `{}` declares base metadata, but `{}` is only valid for registry:base items",
                item.name,
                forge_ui_registry_item_type_label(item.item_type)
            );
        }
        if item.item_type == DxForgeUiRegistryItemType::Font && item.font.is_none() {
            bail!("registry:font item `{}` requires font metadata", item.name);
        }
        if let Some(config) = item.config.as_ref() {
            validate_forge_ui_registry_config_aliases(&item.name, config)?;
            validate_forge_ui_registry_config_registries(&item.name, config)?;
            for (registry_name, registry_config) in &config.registries {
                validate_forge_ui_registry_advanced_registry_metadata(
                    &item.name,
                    registry_name,
                    registry_config,
                )?;
            }
        }
        validate_forge_ui_registry_env_vars(&item.name, &item.env_vars)?;
        validate_forge_ui_registry_tailwind(&item.name, &item.tailwind)?;
        validate_forge_ui_registry_external_dependencies(
            &item.name,
            "dependencies",
            &item.dependencies,
        )?;
        validate_forge_ui_registry_external_dependencies(
            &item.name,
            "devDependencies",
            &item.dev_dependencies,
        )?;

        *item_types.entry(item.item_type).or_insert(0) += 1;
        dependency_count += item.dependencies.len();
        dev_dependency_count += item.dev_dependencies.len();
        registry_dependency_count += item.registry_dependencies.len();
        registry_dependency_count += usize::from(item.extends.is_some());
        env_var_count += item.env_vars.len();
        docs_count += usize::from(item.docs.is_some());

        if let Some(extends) = &item.extends {
            if extends.trim().is_empty() {
                bail!(
                    "Forge UI registry item `{}` has an empty extends target",
                    item.name
                );
            }
            if extends == &item.name {
                bail!(
                    "Forge UI registry item `{}` cannot extend itself",
                    item.name
                );
            }
        }

        let mut item_registry_dependencies = BTreeSet::new();
        if let Some(extends) = &item.extends {
            item_registry_dependencies.insert(extends.clone());
        }
        for dependency in &item.registry_dependencies {
            if dependency.trim().is_empty() {
                bail!(
                    "Forge UI registry item `{}` has an empty registry dependency",
                    item.name
                );
            }
            validate_forge_ui_registry_registry_dependency_reference(&item.name, dependency)?;
            if !item_registry_dependencies.insert(dependency.clone()) {
                bail!(
                    "Forge UI registry item `{}` declares duplicate registry relationship `{dependency}`",
                    item.name
                );
            }
        }

        for file in &item.files {
            validate_forge_ui_registry_file(&item.name, file)?;
            file_count += 1;
        }
    }

    Ok(DxForgeUiRegistryValidationReport {
        schema_version: FORGE_UI_REGISTRY_SCHEMA_VERSION.to_string(),
        valid: true,
        item_count: registry.items.len(),
        file_count,
        include_count: registry.include.len(),
        dependency_count,
        dev_dependency_count,
        registry_dependency_count,
        env_var_count,
        docs_count,
        item_types,
    })
}

pub fn embed_forge_ui_registry_catalog_file_contents(
    registry: &DxForgeUiRegistryCatalog,
    source_root: impl AsRef<Path>,
) -> Result<(
    DxForgeUiRegistryCatalog,
    DxForgeUiRegistryContentEmbeddingReport,
)> {
    validate_forge_ui_registry_catalog(registry)?;
    validate_forge_ui_registry_dependency_graphs(registry)?;

    let source_root = source_root.as_ref();
    let source_root = source_root.canonicalize().with_context(|| {
        format!(
            "canonicalize Forge UI registry source root `{}`",
            source_root.display()
        )
    })?;
    let mut embedded = registry.clone();
    let mut file_count = 0usize;
    let mut embedded_file_count = 0usize;
    let mut preserved_inline_content_file_count = 0usize;
    let mut files = Vec::new();

    for item in &mut embedded.items {
        for file in &mut item.files {
            file_count += 1;
            validate_forge_ui_registry_file(&item.name, file)?;
            let normalized_path = normalize_forge_ui_registry_path(&file.path)?;
            if file
                .content
                .as_ref()
                .is_some_and(|content| !content.is_empty())
            {
                preserved_inline_content_file_count += 1;
                let content = file.content.as_ref().expect("inline content");
                files.push(DxForgeUiRegistryContentEmbeddingFileEvidence {
                    item_name: item.name.clone(),
                    registry_path: normalized_path.clone(),
                    file_type: file.file_type,
                    status: DxForgeUiRegistryContentEmbeddingStatus::PreservedInlineContent,
                    source_path: None,
                    byte_count: content.len(),
                    hash_algorithm: "BLAKE3".to_string(),
                    content_hash: forge_ui_registry_blake3_hex(content.as_bytes()),
                });
                continue;
            }
            let source_file = source_root.join(&normalized_path);
            let canonical_source_file = source_file.canonicalize().with_context(|| {
                format!(
                    "read Forge UI registry source file `{}` for item `{}`",
                    source_file.display(),
                    item.name
                )
            })?;
            if !canonical_source_file.starts_with(&source_root) {
                bail!(
                    "Forge UI registry source file `{}` escapes source root `{}`",
                    canonical_source_file.display(),
                    source_root.display()
                );
            }
            let content = fs::read_to_string(&canonical_source_file).with_context(|| {
                format!(
                    "read Forge UI registry source file `{}` for item `{}`",
                    canonical_source_file.display(),
                    item.name
                )
            })?;
            file.content = Some(content);
            embedded_file_count += 1;
            let content = file.content.as_ref().expect("embedded content");
            files.push(DxForgeUiRegistryContentEmbeddingFileEvidence {
                item_name: item.name.clone(),
                registry_path: normalized_path.clone(),
                file_type: file.file_type,
                status: DxForgeUiRegistryContentEmbeddingStatus::EmbeddedFromSource,
                source_path: Some(normalized_path),
                byte_count: content.len(),
                hash_algorithm: "BLAKE3".to_string(),
                content_hash: forge_ui_registry_blake3_hex(content.as_bytes()),
            });
        }
    }

    validate_forge_ui_registry_catalog(&embedded)?;
    validate_forge_ui_registry_dependency_graphs(&embedded)?;

    Ok((
        embedded,
        DxForgeUiRegistryContentEmbeddingReport {
            schema_version: FORGE_UI_REGISTRY_SCHEMA_VERSION.to_string(),
            source_root,
            item_count: registry.items.len(),
            file_count,
            embedded_file_count,
            preserved_inline_content_file_count,
            files,
        },
    ))
}

fn forge_ui_registry_blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

pub fn plan_forge_ui_registry_item(
    registry: &DxForgeUiRegistryCatalog,
    item_name: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeUiRegistryItemPlanReport> {
    validate_forge_ui_registry_catalog(registry)?;
    let item = forge_ui_registry_item_by_name(registry, item_name)?;
    let dependency_graph = resolve_forge_ui_registry_dependency_graph(registry, item_name)?;
    let project = project.as_ref().to_path_buf();
    let mut decisions = Vec::new();
    let mut warnings = Vec::new();
    let mut next_actions = vec![
        format!("Review Forge registry plan for `{}`.", item.name),
        "Run `dx forge registry validate` before any write path.".to_string(),
        "Materialize only after reviewed content, receipts, and local ownership are accepted."
            .to_string(),
    ];
    let mut files = Vec::new();
    let mut inline_content_file_count = 0usize;
    let mut missing_inline_content_count = 0usize;
    let mut dependency_count = 0usize;
    let mut dev_dependency_count = 0usize;
    let mut env_var_count = 0usize;
    let mut css_var_count = 0usize;
    let mut css_rule_count = 0usize;
    let mut tailwind_config_present = false;
    let mut font_present = false;
    let mut config_present = false;
    let mut planned_targets = BTreeMap::new();
    let mut external_registry_references = Vec::new();
    let external_registry_dependency_count = dependency_graph
        .edges
        .iter()
        .filter(|edge| forge_ui_registry_registry_dependency_is_external_reference(&edge.to))
        .count();

    for edge in &dependency_graph.edges {
        if forge_ui_registry_registry_dependency_is_external_reference(&edge.to) {
            external_registry_references.push(classify_forge_ui_registry_external_reference(
                &edge.from, &edge.to,
            )?);
            warnings.push(format!(
                "`{}` declares external registry dependency `{}`; Forge records the edge but does not fetch external registry code during planning.",
                edge.from, edge.to
            ));
            decisions.push(DxForgeUiRegistryPlanDecision {
                subject: format!("{} -> {}", edge.from, edge.to),
                decision: DxForgeUiRegistryPlanDecisionKind::BridgeDependency,
                reason:
                    "External registry dependency stays behind a reviewed Forge registry bridge before any write path."
                        .to_string(),
            });
        } else {
            decisions.push(DxForgeUiRegistryPlanDecision {
                subject: format!("{} -> {}", edge.from, edge.to),
                decision: DxForgeUiRegistryPlanDecisionKind::ResolveRegistryDependency,
                reason:
                    "Resolved through the Forge registry dependency graph before any write path."
                        .to_string(),
            });
        }
    }

    for planned_item_name in &dependency_graph.ordered_item_names {
        let planned_item = forge_ui_registry_item_by_name(registry, planned_item_name)?;
        dependency_count += planned_item.dependencies.len();
        dev_dependency_count += planned_item.dev_dependencies.len();
        env_var_count += planned_item.env_vars.len();
        css_rule_count += planned_item.css.len();
        tailwind_config_present |= planned_item.tailwind.is_some();
        font_present |= planned_item.font.is_some();
        config_present |= forge_ui_registry_item_has_base_configuration(planned_item);
        css_var_count += planned_item
            .css_vars
            .as_ref()
            .map(|vars| vars.theme.len() + vars.light.len() + vars.dark.len())
            .unwrap_or(0);

        for file in &planned_item.files {
            let target_path = forge_ui_registry_materialized_target(registry, planned_item, file)?;
            if let Some(existing_item) =
                planned_targets.insert(target_path.clone(), planned_item.name.clone())
            {
                bail!(
                    "Forge UI registry target `{target_path}` is produced by both `{existing_item}` and `{}`",
                    planned_item.name
                );
            }
            let has_inline_content = file
                .content
                .as_ref()
                .is_some_and(|content| !content.is_empty());
            if has_inline_content {
                inline_content_file_count += 1;
            } else {
                missing_inline_content_count += 1;
                warnings.push(format!(
                    "`{}` requires reviewed content before Forge can write `{}`.",
                    file.path, target_path
                ));
            }

            let action = if has_inline_content {
                DxForgeUiRegistryPlanAction::Materialize
            } else {
                DxForgeUiRegistryPlanAction::NeedsReviewedContent
            };
            decisions.push(DxForgeUiRegistryPlanDecision {
                subject: target_path.clone(),
                decision: DxForgeUiRegistryPlanDecisionKind::Materialize,
                reason: format!(
                    "Registry item `{}` file `{}` maps to Forge-owned project target `{}`.",
                    planned_item.name, file.path, target_path
                ),
            });
            files.push(DxForgeUiRegistryPlannedFile {
                item_name: planned_item.name.clone(),
                source_path: file.path.clone(),
                target_path,
                file_type: file.file_type,
                has_inline_content,
                action,
            });
        }

        for dependency in &planned_item.dependencies {
            warnings.push(format!(
                "`{}` from `{}` is an external runtime dependency and needs a Forge package, bridge, or explicit policy decision.",
                dependency, planned_item.name
            ));
            decisions.push(DxForgeUiRegistryPlanDecision {
                subject: format!("{}:{}", planned_item.name, dependency),
                decision: DxForgeUiRegistryPlanDecisionKind::BridgeDependency,
                reason:
                    "External dependency is not installed by registry planning; it stays behind a reviewed Forge bridge."
                        .to_string(),
            });
        }
        for dependency in &planned_item.dev_dependencies {
            decisions.push(DxForgeUiRegistryPlanDecision {
                subject: format!("{}:{}", planned_item.name, dependency),
                decision: DxForgeUiRegistryPlanDecisionKind::IgnoreDevDependency,
                reason:
                    "Dev dependencies are recorded as upstream context and are not installed by Forge planning."
                        .to_string(),
            });
        }
        for env_var in planned_item.env_vars.keys() {
            decisions.push(DxForgeUiRegistryPlanDecision {
                subject: format!("{}:{}", planned_item.name, env_var),
                decision: DxForgeUiRegistryPlanDecisionKind::RequireEnvironment,
                reason:
                    "Environment requirement must be surfaced in Forge receipts before runtime use."
                        .to_string(),
            });
        }
        if planned_item.css_vars.is_some()
            || !planned_item.css.is_empty()
            || planned_item.tailwind.is_some()
        {
            decisions.push(DxForgeUiRegistryPlanDecision {
                subject: planned_item.name.clone(),
                decision: DxForgeUiRegistryPlanDecisionKind::MergeStyle,
                reason:
                    "Style data must merge through dx-style or a reviewed CSS artifact instead of hidden postinstall transforms."
                        .to_string(),
            });
        }
        if forge_ui_registry_item_has_base_configuration(planned_item) {
            decisions.push(DxForgeUiRegistryPlanDecision {
                subject: planned_item.name.clone(),
                decision: DxForgeUiRegistryPlanDecisionKind::MergeConfig,
                reason:
                    "Base registry config is a project policy change and must be reviewed before write."
                        .to_string(),
            });
        }
        if planned_item.font.is_some() {
            decisions.push(DxForgeUiRegistryPlanDecision {
                subject: planned_item.name.clone(),
                decision: DxForgeUiRegistryPlanDecisionKind::RegisterFont,
                reason:
                    "Font registry metadata is recorded without downloading remote font packages in this plan."
                        .to_string(),
            });
        }
    }

    if !dependency_graph.edges.is_empty() {
        next_actions.push(format!(
            "Materialize registry items in dependency order: {}.",
            dependency_graph.ordered_item_names.join(" -> ")
        ));
    }
    if dependency_count > 0 {
        next_actions.push(
            "Convert, bridge, or reject external dependencies through Forge import before writes."
                .to_string(),
        );
    }
    if external_registry_dependency_count > 0 {
        next_actions.push(
            "Resolve external registry dependencies through a configured Forge registry bridge before writes."
                .to_string(),
        );
    }

    let score = forge_ui_registry_item_plan_score(
        !files.is_empty(),
        missing_inline_content_count,
        dependency_count,
        external_registry_dependency_count,
    );
    let passed = score >= 80 && missing_inline_content_count == 0;

    Ok(DxForgeUiRegistryItemPlanReport {
        schema_version: FORGE_UI_REGISTRY_SCHEMA_VERSION.to_string(),
        item_name: item.name.clone(),
        item_type: item.item_type,
        project,
        passed,
        score,
        file_count: files.len(),
        write_file_count: inline_content_file_count,
        inline_content_file_count,
        missing_inline_content_count,
        dependency_count,
        dev_dependency_count,
        registry_dependency_count: dependency_graph.edges.len(),
        external_registry_references,
        env_var_count,
        css_var_count,
        css_rule_count,
        tailwind_config_present,
        font_present,
        config_present,
        registry_dependency_order: dependency_graph.ordered_item_names,
        registry_dependency_edges: dependency_graph.edges,
        no_package_manager_execution: true,
        forbidden_commands: vec![
            "npm install".to_string(),
            "pnpm install".to_string(),
            "bun install".to_string(),
            "yarn install".to_string(),
            "npx shadcn".to_string(),
        ],
        files,
        decisions,
        warnings,
        next_actions,
    })
}

pub fn describe_forge_ui_registry_item(
    registry: &DxForgeUiRegistryCatalog,
    item_name: &str,
) -> Result<DxForgeUiRegistryItemDocsReport> {
    validate_forge_ui_registry_catalog(registry)?;
    let item = forge_ui_registry_item_by_name(registry, item_name)?;
    let dependency_graph = resolve_forge_ui_registry_dependency_graph(registry, item_name)?;
    let plan = plan_forge_ui_registry_item(registry, item_name, ".")?;
    let mut env_vars = item.env_vars.keys().cloned().collect::<Vec<_>>();
    env_vars.sort();

    let css_var_count = item.css_vars.as_ref().map_or(0, |vars| {
        vars.theme.len() + vars.light.len() + vars.dark.len()
    });

    Ok(DxForgeUiRegistryItemDocsReport {
        schema_version: FORGE_UI_REGISTRY_SCHEMA_VERSION.to_string(),
        item_name: item.name.clone(),
        item_type: item.item_type,
        title: item.title.clone(),
        author: item.author.clone(),
        description: item.description.clone(),
        docs: item.docs.clone(),
        has_docs: item
            .docs
            .as_ref()
            .is_some_and(|docs| !docs.trim().is_empty()),
        file_count: item.files.len(),
        dependencies: item.dependencies.clone(),
        dev_dependencies: item.dev_dependencies.clone(),
        registry_dependencies: item.registry_dependencies.clone(),
        registry_dependency_order: dependency_graph.ordered_item_names,
        files: plan
            .files
            .into_iter()
            .filter(|file| file.item_name == item.name)
            .collect(),
        env_vars,
        css_var_count,
        css_rule_count: item.css.len(),
        tailwind_config_present: item.tailwind.is_some(),
        font_present: item.font.is_some(),
        config_present: forge_ui_registry_item_has_base_configuration(item),
        base_style: item.style.clone(),
        base_icon_library: item.icon_library.clone(),
        base_color: item.base_color.clone(),
        base_theme: item.theme.clone(),
        categories: item.categories.clone(),
        no_package_manager_execution: true,
        next_actions: vec![
            format!("dx forge registry plan --item {}", item.name),
            format!("dx forge registry apply --item {} --dry-run", item.name),
        ],
    })
}

fn validate_forge_ui_registry_registry_dependency_reference(
    item_name: &str,
    dependency: &str,
) -> Result<()> {
    let trimmed = dependency.trim();
    if trimmed.is_empty() {
        bail!("Forge UI registry item `{item_name}` has an empty registry dependency");
    }
    if dependency != trimmed
        || dependency.contains('\0')
        || dependency.chars().any(char::is_whitespace)
        || dependency.starts_with('-')
        || forge_ui_registry_dependency_reference_is_command(trimmed)
    {
        bail!(
            "Forge UI registry item `{item_name}` has unsafe registry dependency `{dependency}`; registry dependencies must be item names, namespaced items, GitHub registry addresses, local registry files, or reviewed registry URLs"
        );
    }
    if trimmed.starts_with("./") || trimmed.starts_with("../") {
        let normalized = normalize_forge_ui_registry_path(trimmed)?;
        validate_project_relative_path(&normalized).with_context(|| {
            format!("validate Forge UI registry dependency `{dependency}` on item `{item_name}`")
        })?;
    }
    Ok(())
}

fn forge_ui_registry_dependency_reference_is_command(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    lower.starts_with("npm ")
        || lower.starts_with("pnpm ")
        || lower.starts_with("yarn ")
        || lower.starts_with("bun ")
        || lower.starts_with("npx ")
        || lower.starts_with("cargo ")
        || lower.starts_with("pip ")
        || lower.starts_with("uv ")
        || lower.starts_with("go get ")
        || lower.starts_with("dart ")
        || reference
            .chars()
            .any(|ch| matches!(ch, ';' | '&' | '|' | '<' | '>' | '`'))
}

fn classify_forge_ui_registry_external_reference(
    from_item: &str,
    reference: &str,
) -> Result<DxForgeUiRegistryExternalReferenceEvidence> {
    validate_forge_ui_registry_registry_dependency_reference(from_item, reference)?;
    Ok(DxForgeUiRegistryExternalReferenceEvidence {
        from_item: from_item.to_string(),
        reference: reference.to_string(),
        source_kind: forge_ui_registry_external_reference_kind(reference),
        item_name: forge_ui_registry_external_reference_item_name(reference)?,
        requires_bridge: true,
        no_network_request: true,
    })
}

fn forge_ui_registry_external_reference_kind(
    reference: &str,
) -> DxForgeUiRegistryExternalReferenceKind {
    if reference.starts_with("http://") || reference.starts_with("https://") {
        return DxForgeUiRegistryExternalReferenceKind::RemoteRegistryUrl;
    }
    if reference.starts_with("./") || reference.starts_with("../") {
        return DxForgeUiRegistryExternalReferenceKind::LocalRegistryFile;
    }
    if reference.starts_with('@') && reference.contains('/') {
        return DxForgeUiRegistryExternalReferenceKind::NamespacedRegistry;
    }
    if forge_ui_registry_registry_dependency_is_github_address(reference) {
        return DxForgeUiRegistryExternalReferenceKind::GithubSourceRegistry;
    }
    DxForgeUiRegistryExternalReferenceKind::UnknownExternalRegistry
}

fn forge_ui_registry_external_reference_item_name(reference: &str) -> Result<String> {
    let reference_without_ref = reference
        .split_once('#')
        .map_or(reference, |(path, _)| path);
    let reference_without_query = reference_without_ref
        .split_once('?')
        .map_or(reference_without_ref, |(path, _)| path);
    let segment = reference_without_query
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(reference_without_query)
        .trim_end_matches(".json");
    let name = segment.trim();
    if name.is_empty() {
        bail!("Forge UI registry external reference `{reference}` does not name an item");
    }
    Ok(name.to_string())
}

fn validate_forge_ui_registry_external_dependencies(
    item_name: &str,
    field_name: &str,
    dependencies: &[String],
) -> Result<()> {
    let mut seen = BTreeSet::new();
    for dependency in dependencies {
        let trimmed = dependency.trim();
        if trimmed.is_empty() {
            bail!("Forge UI registry item `{item_name}` has an empty {field_name} entry");
        }
        if dependency != trimmed || forge_ui_registry_external_dependency_entry_is_unsafe(trimmed) {
            bail!(
                "Forge UI registry item `{item_name}` has unsafe {field_name} entry `{dependency}`; registry external dependencies must be package names, not package-manager commands"
            );
        }
        if !seen.insert(trimmed.to_string()) {
            bail!(
                "Forge UI registry item `{item_name}` declares duplicate {field_name} entry `{trimmed}`"
            );
        }
    }
    Ok(())
}

fn forge_ui_registry_external_dependency_entry_is_unsafe(entry: &str) -> bool {
    let lower = entry.to_ascii_lowercase();
    lower.contains("://")
        || lower.starts_with('.')
        || lower.starts_with('-')
        || lower.starts_with("npm ")
        || lower.starts_with("pnpm ")
        || lower.starts_with("yarn ")
        || lower.starts_with("bun ")
        || lower.starts_with("npx ")
        || lower.starts_with("cargo ")
        || lower.starts_with("pip ")
        || lower.starts_with("uv ")
        || lower.starts_with("go get ")
        || lower.starts_with("dart ")
        || entry
            .chars()
            .any(|ch| ch.is_whitespace() || matches!(ch, ';' | '&' | '|' | '<' | '>' | '`'))
}

fn forge_ui_registry_item_plan_score(
    has_files: bool,
    missing_inline_content_count: usize,
    dependency_count: usize,
    external_registry_dependency_count: usize,
) -> u8 {
    let mut score = 100u8;
    if !has_files {
        score = score.min(75);
    }
    if missing_inline_content_count > 0 {
        score = score.min(80);
    }
    if dependency_count > 0 {
        score = score.min(85);
    }
    if external_registry_dependency_count > 0 {
        score = score.min(75);
    }
    score
}

pub fn validate_forge_ui_registry_dependency_graphs(
    registry: &DxForgeUiRegistryCatalog,
) -> Result<()> {
    validate_forge_ui_registry_catalog(registry)?;
    for item in &registry.items {
        resolve_forge_ui_registry_dependency_graph(registry, &item.name)?;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DxForgeUiRegistryResolvedDependencyGraph {
    ordered_item_names: Vec<String>,
    edges: Vec<DxForgeUiRegistryDependencyEdge>,
}

fn resolve_forge_ui_registry_dependency_graph(
    registry: &DxForgeUiRegistryCatalog,
    item_name: &str,
) -> Result<DxForgeUiRegistryResolvedDependencyGraph> {
    forge_ui_registry_item_by_name(registry, item_name)?;
    let mut visited = BTreeSet::new();
    let mut visiting = BTreeSet::new();
    let mut stack = Vec::new();
    let mut ordered_item_names = Vec::new();
    let mut edges = Vec::new();

    resolve_forge_ui_registry_dependency_graph_item(
        registry,
        item_name,
        &mut visited,
        &mut visiting,
        &mut stack,
        &mut ordered_item_names,
        &mut edges,
    )?;

    Ok(DxForgeUiRegistryResolvedDependencyGraph {
        ordered_item_names,
        edges,
    })
}

fn resolve_forge_ui_registry_dependency_graph_item(
    registry: &DxForgeUiRegistryCatalog,
    item_name: &str,
    visited: &mut BTreeSet<String>,
    visiting: &mut BTreeSet<String>,
    stack: &mut Vec<String>,
    ordered_item_names: &mut Vec<String>,
    edges: &mut Vec<DxForgeUiRegistryDependencyEdge>,
) -> Result<()> {
    if visited.contains(item_name) {
        return Ok(());
    }
    if visiting.contains(item_name) {
        let cycle = forge_ui_registry_dependency_cycle(stack, item_name);
        bail!("Forge UI registry dependency cycle detected: {cycle}");
    }

    let item = forge_ui_registry_item_by_name(registry, item_name)?;
    visiting.insert(item_name.to_string());
    stack.push(item_name.to_string());

    for dependency in forge_ui_registry_item_graph_dependencies(item) {
        if dependency == item_name {
            bail!("Forge UI registry item `{item_name}` cannot depend on itself");
        }
        if forge_ui_registry_registry_dependency_is_external_reference(dependency) {
            edges.push(DxForgeUiRegistryDependencyEdge {
                from: item_name.to_string(),
                to: dependency.to_string(),
            });
            continue;
        }
        if forge_ui_registry_item_by_name(registry, dependency).is_err() {
            bail!(
                "Forge UI registry item `{item_name}` depends on missing registry item `{dependency}`"
            );
        }
        edges.push(DxForgeUiRegistryDependencyEdge {
            from: item_name.to_string(),
            to: dependency.to_string(),
        });
        resolve_forge_ui_registry_dependency_graph_item(
            registry,
            dependency,
            visited,
            visiting,
            stack,
            ordered_item_names,
            edges,
        )?;
    }

    stack.pop();
    visiting.remove(item_name);
    visited.insert(item_name.to_string());
    ordered_item_names.push(item_name.to_string());
    Ok(())
}

fn forge_ui_registry_item_graph_dependencies(
    item: &DxForgeUiRegistryItem,
) -> impl Iterator<Item = &str> {
    item.extends
        .iter()
        .map(String::as_str)
        .chain(item.registry_dependencies.iter().map(String::as_str))
}

fn forge_ui_registry_dependency_cycle(stack: &[String], item_name: &str) -> String {
    let start = stack
        .iter()
        .position(|entry| entry == item_name)
        .unwrap_or(0);
    let mut cycle = stack[start..].to_vec();
    cycle.push(item_name.to_string());
    cycle.join(" -> ")
}

fn forge_ui_registry_registry_dependency_is_external_reference(dependency: &str) -> bool {
    let lower = dependency.to_ascii_lowercase();
    lower.starts_with("https://")
        || lower.starts_with("http://")
        || dependency.starts_with("./")
        || dependency.starts_with("../")
        || (dependency.starts_with('@') && dependency.contains('/'))
        || forge_ui_registry_registry_dependency_is_github_address(dependency)
}

fn forge_ui_registry_registry_dependency_is_github_address(dependency: &str) -> bool {
    let Some((path, _ref)) = dependency.split_once('#') else {
        return dependency.split('/').count() >= 3;
    };
    path.split('/').count() >= 3
}

fn forge_ui_registry_item_by_name<'a>(
    registry: &'a DxForgeUiRegistryCatalog,
    item_name: &str,
) -> Result<&'a DxForgeUiRegistryItem> {
    registry
        .items
        .iter()
        .find(|item| item.name == item_name)
        .with_context(|| format!("Forge UI registry item `{item_name}` was not found"))
}

fn forge_ui_registry_materialized_target(
    registry: &DxForgeUiRegistryCatalog,
    item: &DxForgeUiRegistryItem,
    file: &DxForgeUiRegistryItemFile,
) -> Result<String> {
    if let Some(target) = &file.target {
        return forge_ui_registry_normalized_target(registry, target);
    }

    let normalized = normalize_forge_ui_registry_path(&file.path)?;
    if forge_ui_registry_path_is_project_target(&normalized) {
        return Ok(normalized);
    }

    let file_name = normalized
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(normalized.as_str());
    forge_ui_registry_normalized_target(
        registry,
        &format!(
            "{}/{}",
            forge_ui_registry_default_target_dir(item.item_type, file.file_type),
            file_name
        ),
    )
}

fn forge_ui_registry_normalized_target(
    registry: &DxForgeUiRegistryCatalog,
    path: &str,
) -> Result<String> {
    validate_forge_ui_registry_target_path(path)?;
    let target = path.trim_start_matches("~/");
    if let Some(resolved) = resolve_forge_ui_registry_target_placeholder(registry, target)? {
        return Ok(resolved);
    }
    normalize_forge_ui_registry_path(target)
}

fn resolve_forge_ui_registry_target_placeholder(
    registry: &DxForgeUiRegistryCatalog,
    target: &str,
) -> Result<Option<String>> {
    let Some(rest) = target.strip_prefix('@') else {
        return Ok(None);
    };
    let Some((placeholder, suffix)) = rest.split_once('/') else {
        return Ok(Some(normalize_forge_ui_registry_path(rest)?));
    };
    if let Some(base) = forge_ui_registry_target_placeholder_base(registry, placeholder)? {
        return Ok(Some(join_forge_ui_registry_path(&base, suffix)?));
    }
    Ok(Some(normalize_forge_ui_registry_path(rest)?))
}

fn forge_ui_registry_target_placeholder_base(
    registry: &DxForgeUiRegistryCatalog,
    placeholder: &str,
) -> Result<Option<String>> {
    for item in &registry.items {
        if item.item_type != DxForgeUiRegistryItemType::Base {
            continue;
        }
        let Some(aliases) = item
            .config
            .as_ref()
            .and_then(|config| config.aliases.as_ref())
        else {
            continue;
        };
        let alias = match placeholder {
            "components" => aliases.components.as_deref(),
            "ui" => aliases.ui.as_deref(),
            "lib" => aliases.lib.as_deref(),
            "hooks" => aliases.hooks.as_deref(),
            _ => None,
        };
        if let Some(alias) = alias {
            return Ok(Some(forge_ui_registry_alias_target_path(alias)?));
        }
    }

    Ok(match placeholder {
        "components" => Some("components".to_string()),
        "ui" => Some("components/ui".to_string()),
        "lib" => Some("lib".to_string()),
        "hooks" => Some("hooks".to_string()),
        _ => None,
    })
}

fn forge_ui_registry_alias_target_path(alias: &str) -> Result<String> {
    let alias = alias.trim();
    let alias = alias
        .strip_prefix("@/")
        .or_else(|| alias.strip_prefix("~/"))
        .unwrap_or(alias);
    normalize_forge_ui_registry_path(alias)
}

fn forge_ui_registry_path_is_project_target(path: &str) -> bool {
    [
        "app/",
        "pages/",
        "components/",
        "hooks/",
        "lib/",
        "styles/",
        "public/",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
}

fn forge_ui_registry_default_target_dir(
    item_type: DxForgeUiRegistryItemType,
    file_type: DxForgeUiRegistryItemType,
) -> &'static str {
    match file_type {
        DxForgeUiRegistryItemType::Ui | DxForgeUiRegistryItemType::Item => "components/ui",
        DxForgeUiRegistryItemType::Block => "components/blocks",
        DxForgeUiRegistryItemType::Component | DxForgeUiRegistryItemType::Example => "components",
        DxForgeUiRegistryItemType::Hook => "hooks",
        DxForgeUiRegistryItemType::Lib => "lib",
        DxForgeUiRegistryItemType::Style | DxForgeUiRegistryItemType::Theme => "styles",
        DxForgeUiRegistryItemType::Font => "styles/fonts",
        DxForgeUiRegistryItemType::Internal => "components/internal",
        DxForgeUiRegistryItemType::Page | DxForgeUiRegistryItemType::File => {
            if item_type == DxForgeUiRegistryItemType::Page {
                "app"
            } else {
                "components"
            }
        }
        DxForgeUiRegistryItemType::Base => "lib/forge",
    }
}

pub fn flatten_forge_ui_registry_catalogs(
    root: &DxForgeUiRegistryCatalog,
    includes: &BTreeMap<String, DxForgeUiRegistryCatalog>,
) -> Result<DxForgeUiRegistryCatalog> {
    validate_forge_ui_registry_catalog(root)?;
    let mut flattened = root.clone();
    flattened.include.clear();
    flattened.items.clear();
    let mut names = BTreeSet::new();
    let mut included_files = BTreeSet::new();
    let mut include_stack = Vec::new();
    append_forge_ui_registry_catalog_items(
        "",
        root,
        includes,
        &mut flattened.items,
        &mut names,
        &mut included_files,
        &mut include_stack,
        0,
    )?;

    validate_forge_ui_registry_catalog(&flattened)?;
    validate_forge_ui_registry_dependency_graphs(&flattened)?;
    Ok(flattened)
}

pub fn load_forge_ui_registry_catalog_from_path(
    registry_file: impl AsRef<Path>,
) -> Result<DxForgeUiRegistryCatalog> {
    let registry_file = registry_file.as_ref();
    let registry_root = registry_file.parent().unwrap_or_else(|| Path::new("."));
    let registry_name = registry_file
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Forge UI registry path must name a JSON file"))?;
    let root_json = fs::read_to_string(registry_file)
        .with_context(|| format!("read Forge UI registry `{}`", registry_file.display()))?;
    let root = parse_forge_ui_registry_json(&root_json)
        .with_context(|| format!("parse Forge UI registry `{}`", registry_file.display()))?;

    if !root.include.is_empty() && registry_name != "registry.json" {
        bail!("Forge UI registries that use include must be named registry.json");
    }

    let mut includes = BTreeMap::new();
    let mut include_stack = Vec::new();
    read_forge_ui_registry_includes(
        registry_root,
        registry_name,
        &root,
        &mut includes,
        &mut include_stack,
        0,
    )?;

    flatten_forge_ui_registry_catalogs(&root, &includes)
}

pub fn resolve_forge_ui_registry_reference(
    registry_file: impl AsRef<Path>,
    item_reference: &str,
) -> Result<DxForgeUiRegistryResolvedReference> {
    let registry_file = registry_file.as_ref();
    let item_reference = item_reference.trim();
    if item_reference.is_empty() {
        bail!("Forge UI registry item reference cannot be empty");
    }

    if let Some((registry_reference, item_name)) = item_reference.split_once('#') {
        let item_name = normalized_forge_ui_registry_item_reference(item_name)?;
        let registry_file =
            resolve_local_forge_ui_registry_file_reference(registry_file, registry_reference)?;
        let catalog = load_forge_ui_registry_catalog_from_path(&registry_file)?;
        forge_ui_registry_item_by_name(&catalog, &item_name)?;
        return Ok(DxForgeUiRegistryResolvedReference {
            catalog,
            registry_file,
            item_name,
            source_kind: "local-registry-file".to_string(),
        });
    }

    let catalog = load_forge_ui_registry_catalog_from_path(registry_file)?;
    if let Some((alias, item_name)) = item_reference.split_once('/') {
        if let Some(configured_registry) =
            configured_forge_ui_registry_reference(&catalog, alias.trim_start_matches('@'))
        {
            let item_name = normalized_forge_ui_registry_item_reference(item_name)?;
            let registry_file = resolve_configured_forge_ui_registry_file_reference(
                registry_file,
                configured_registry,
                &item_name,
            )?;
            let catalog = load_forge_ui_registry_catalog_from_path(&registry_file)?;
            forge_ui_registry_item_by_name(&catalog, &item_name)?;
            return Ok(DxForgeUiRegistryResolvedReference {
                catalog,
                registry_file,
                item_name,
                source_kind: "configured-local-registry".to_string(),
            });
        }
    }

    forge_ui_registry_item_by_name(&catalog, item_reference)?;
    Ok(DxForgeUiRegistryResolvedReference {
        catalog,
        registry_file: registry_file.to_path_buf(),
        item_name: item_reference.to_string(),
        source_kind: "default-registry-file".to_string(),
    })
}

fn normalized_forge_ui_registry_item_reference(item_name: &str) -> Result<String> {
    let item_name = item_name.trim();
    if item_name.is_empty() {
        bail!("Forge UI registry item reference cannot be empty");
    }
    if item_name.contains('#') {
        bail!("Forge UI registry item reference cannot contain `#`");
    }
    Ok(item_name.to_string())
}

fn configured_forge_ui_registry_reference<'a>(
    catalog: &'a DxForgeUiRegistryCatalog,
    alias: &str,
) -> Option<&'a DxForgeUiRegistryConfigItem> {
    let scoped_alias = format!("@{alias}");
    catalog
        .items
        .iter()
        .filter_map(|item| item.config.as_ref())
        .find_map(|config| {
            config
                .registries
                .get(alias)
                .or_else(|| config.registries.get(&scoped_alias))
        })
}

fn configured_forge_ui_registry_reference_url(config: &DxForgeUiRegistryConfigItem) -> &str {
    match config {
        DxForgeUiRegistryConfigItem::Url(url) => url,
        DxForgeUiRegistryConfigItem::Advanced { url, .. } => url,
    }
}

fn resolve_configured_forge_ui_registry_file_reference(
    registry_file: &Path,
    config: &DxForgeUiRegistryConfigItem,
    item_name: &str,
) -> Result<PathBuf> {
    let url = configured_forge_ui_registry_reference_url_for_item(config, item_name);
    resolve_local_forge_ui_registry_file_reference(registry_file, &url)
}

fn configured_forge_ui_registry_reference_url_for_item(
    config: &DxForgeUiRegistryConfigItem,
    item_name: &str,
) -> String {
    configured_forge_ui_registry_reference_url(config).replace("{name}", item_name)
}

fn resolve_local_forge_ui_registry_file_reference(
    registry_file: &Path,
    registry_reference: &str,
) -> Result<PathBuf> {
    let registry_reference = registry_reference.trim();
    if registry_reference.is_empty() {
        bail!("Forge UI registry reference cannot be empty");
    }
    if registry_reference.starts_with("http://") || registry_reference.starts_with("https://") {
        bail!(
            "remote registry resolution is bridge-gated for `{registry_reference}`; no network request was made; create an accepted Forge registry bridge or pull receipt before resolving remote items"
        );
    }

    let normalized = normalize_forge_ui_registry_path(registry_reference)
        .with_context(|| format!("validate Forge UI registry reference `{registry_reference}`"))?;
    validate_project_relative_path(&normalized)
        .with_context(|| format!("validate Forge UI registry reference `{registry_reference}`"))?;
    if !normalized
        .rsplit('/')
        .next()
        .is_some_and(|name| name == "registry.json")
    {
        bail!("Forge UI registry reference `{registry_reference}` must name registry.json");
    }

    let root = registry_file.parent().unwrap_or_else(|| Path::new("."));
    Ok(root.join(normalized))
}

fn append_forge_ui_registry_catalog_items(
    catalog_path: &str,
    catalog: &DxForgeUiRegistryCatalog,
    includes: &BTreeMap<String, DxForgeUiRegistryCatalog>,
    items: &mut Vec<DxForgeUiRegistryItem>,
    names: &mut BTreeSet<String>,
    included_files: &mut BTreeSet<String>,
    include_stack: &mut Vec<String>,
    depth: usize,
) -> Result<()> {
    if depth > FORGE_UI_REGISTRY_MAX_INCLUDE_DEPTH {
        bail!(
            "Forge UI registry include tree is too deep; maximum depth is {}",
            FORGE_UI_REGISTRY_MAX_INCLUDE_DEPTH
        );
    }

    validate_forge_ui_registry_catalog(catalog)
        .with_context(|| format!("validate Forge UI registry catalog `{catalog_path}`"))?;
    let catalog_dir = forge_ui_registry_catalog_dir(catalog_path);

    for include in &catalog.include {
        let resolved = resolve_forge_ui_registry_include_path(&catalog_dir, include)?;
        if include_stack.contains(&resolved) {
            bail!(
                "Forge UI registry include cycle detected: {} -> {resolved}",
                include_stack.join(" -> ")
            );
        }
        if !included_files.insert(resolved.clone()) {
            bail!("Forge UI registry file included more than once: `{resolved}`");
        }
        let included = includes
            .get(&resolved)
            .with_context(|| format!("missing Forge UI registry include `{resolved}`"))?;
        include_stack.push(resolved.clone());
        append_forge_ui_registry_catalog_items(
            &resolved,
            included,
            includes,
            items,
            names,
            included_files,
            include_stack,
            depth + 1,
        )?;
        include_stack.pop();
    }

    for item in &catalog.items {
        push_flattened_forge_ui_registry_item(items, names, &catalog_dir, item)?;
    }

    Ok(())
}

fn read_forge_ui_registry_includes(
    registry_root: &Path,
    catalog_path: &str,
    catalog: &DxForgeUiRegistryCatalog,
    includes: &mut BTreeMap<String, DxForgeUiRegistryCatalog>,
    include_stack: &mut Vec<String>,
    depth: usize,
) -> Result<()> {
    if depth > FORGE_UI_REGISTRY_MAX_INCLUDE_DEPTH {
        bail!(
            "Forge UI registry include tree is too deep; maximum depth is {}",
            FORGE_UI_REGISTRY_MAX_INCLUDE_DEPTH
        );
    }

    let catalog_dir = forge_ui_registry_catalog_dir(catalog_path);
    for include in &catalog.include {
        let resolved = resolve_forge_ui_registry_include_path(&catalog_dir, include)?;
        if include_stack.contains(&resolved) {
            bail!(
                "Forge UI registry include cycle detected: {} -> {resolved}",
                include_stack.join(" -> ")
            );
        }
        if includes.contains_key(&resolved) {
            continue;
        }

        let path = registry_root.join(&resolved);
        let json = fs::read_to_string(&path)
            .with_context(|| format!("read Forge UI registry include `{}`", path.display()))?;
        let included = parse_forge_ui_registry_json(&json)
            .with_context(|| format!("parse Forge UI registry include `{}`", path.display()))?;
        include_stack.push(resolved.clone());
        read_forge_ui_registry_includes(
            registry_root,
            &resolved,
            &included,
            includes,
            include_stack,
            depth + 1,
        )?;
        include_stack.pop();
        includes.insert(resolved, included);
    }

    Ok(())
}

fn push_flattened_forge_ui_registry_item(
    items: &mut Vec<DxForgeUiRegistryItem>,
    names: &mut BTreeSet<String>,
    include_dir: &str,
    item: &DxForgeUiRegistryItem,
) -> Result<()> {
    if !names.insert(item.name.clone()) {
        bail!("duplicate Forge UI registry item `{}`", item.name);
    }
    let mut item = item.clone();
    for file in &mut item.files {
        file.path = join_forge_ui_registry_path(include_dir, &file.path)?;
    }
    items.push(item);
    Ok(())
}

fn resolve_forge_ui_registry_include_path(current_dir: &str, include: &str) -> Result<String> {
    validate_forge_ui_registry_include_path(include)?;
    join_forge_ui_registry_path(current_dir, include)
}

fn forge_ui_registry_catalog_dir(catalog_path: &str) -> String {
    catalog_path
        .strip_suffix("registry.json")
        .unwrap_or_default()
        .trim_end_matches('/')
        .to_string()
}

fn validate_registry_has_items_or_includes(registry: &DxForgeUiRegistryCatalog) -> Result<()> {
    if registry.items.is_empty() && registry.include.is_empty() {
        bail!("Forge UI registry must define at least one of `items` or `include`");
    }
    Ok(())
}

fn validate_forge_ui_registry_file(
    item_name: &str,
    file: &DxForgeUiRegistryItemFile,
) -> Result<()> {
    validate_forge_ui_registry_project_relative_path(&file.path)
        .with_context(|| format!("validate Forge UI registry file `{}`", file.path))?;
    match file.file_type {
        DxForgeUiRegistryItemType::File | DxForgeUiRegistryItemType::Page => {
            let Some(target) = file.target.as_deref() else {
                bail!(
                    "{} file `{}` in item `{item_name}` requires target",
                    forge_ui_registry_item_type_label(file.file_type),
                    file.path
                );
            };
            validate_forge_ui_registry_target_path(target)
                .with_context(|| format!("validate Forge UI registry target `{target}`"))?;
        }
        _ => {
            if let Some(target) = file.target.as_deref() {
                validate_forge_ui_registry_target_path(target)
                    .with_context(|| format!("validate Forge UI registry target `{target}`"))?;
            }
        }
    }
    Ok(())
}

fn validate_forge_ui_registry_include_path(include: &str) -> Result<()> {
    if include.trim().is_empty() {
        bail!("Forge UI registry include path cannot be empty");
    }
    if include.starts_with("http://") || include.starts_with("https://") {
        bail!("remote include `{include}` is not supported");
    }
    validate_forge_ui_registry_project_relative_path(include)
        .with_context(|| format!("validate Forge UI registry include `{include}`"))?;
    if !include
        .trim_start_matches("./")
        .rsplit('/')
        .next()
        .is_some_and(|name| name == "registry.json")
    {
        bail!("Forge UI registry include `{include}` must explicitly reference registry.json");
    }
    Ok(())
}

fn validate_forge_ui_registry_target_path(path: &str) -> Result<()> {
    if let Some(stripped) = path.strip_prefix("~/") {
        return validate_forge_ui_registry_project_relative_path(stripped);
    }
    validate_forge_ui_registry_project_relative_path(path)
}

fn validate_forge_ui_registry_project_relative_path(path: &str) -> Result<()> {
    let normalized = normalize_forge_ui_registry_path(path)?;
    validate_project_relative_path(&normalized)
}

fn join_forge_ui_registry_path(base: &str, path: &str) -> Result<String> {
    let path = normalize_forge_ui_registry_path(path)?;
    if base.is_empty() {
        return Ok(path);
    }
    let base = normalize_forge_ui_registry_path(base)?;
    let joined = format!("{base}/{path}");
    validate_project_relative_path(&joined)?;
    Ok(joined)
}

fn normalize_forge_ui_registry_path(path: &str) -> Result<String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        bail!("path cannot be empty");
    }
    if trimmed.contains('\\') {
        bail!("path must use `/` separators");
    }
    if trimmed.starts_with('/') {
        bail!("path must be project-relative");
    }
    let mut parts = Vec::new();
    for part in trimmed.split('/') {
        match part {
            "" | "." => {}
            ".." => bail!("path cannot escape the project root"),
            other => parts.push(other),
        }
    }
    if parts.is_empty() {
        bail!("path cannot be empty");
    }
    Ok(parts.join("/"))
}

fn forge_ui_registry_item_type_label(item_type: DxForgeUiRegistryItemType) -> &'static str {
    match item_type {
        DxForgeUiRegistryItemType::Lib => "registry:lib",
        DxForgeUiRegistryItemType::Block => "registry:block",
        DxForgeUiRegistryItemType::Component => "registry:component",
        DxForgeUiRegistryItemType::Ui => "registry:ui",
        DxForgeUiRegistryItemType::Hook => "registry:hook",
        DxForgeUiRegistryItemType::Page => "registry:page",
        DxForgeUiRegistryItemType::File => "registry:file",
        DxForgeUiRegistryItemType::Theme => "registry:theme",
        DxForgeUiRegistryItemType::Style => "registry:style",
        DxForgeUiRegistryItemType::Item => "registry:item",
        DxForgeUiRegistryItemType::Base => "registry:base",
        DxForgeUiRegistryItemType::Font => "registry:font",
        DxForgeUiRegistryItemType::Example => "registry:example",
        DxForgeUiRegistryItemType::Internal => "registry:internal",
    }
}
