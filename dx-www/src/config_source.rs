//! Project configuration sources.
//!
//! New DX-WWW projects use the extensionless `dx` file as their LLM-format
//! source of truth. Legacy `dx.config.toml` projects remain readable through
//! `DxConfig::load_project`.

use std::path::PathBuf;

use serializer::llm::{DxDocument, DxLlmValue, llm_to_document};

use crate::config::{
    BuildTarget, ConfigError, ConfigResult, CssCompiler, DxConfig, DxDevServerMode,
    OptimizationLevel, ScriptLanguage,
};

/// Parse an LLM-format extensionless `dx` config file.
pub(crate) fn parse_dx_config_source(input: &str) -> ConfigResult<DxConfig> {
    let document =
        llm_to_document(input).map_err(|error| ConfigError::DxParseError(error.to_string()))?;
    let mut config = DxConfig::default();

    apply_string(&document, "project.name", &mut config.project.name);
    apply_string(&document, "project.version", &mut config.project.version);
    config.project.description = optional_string(&document, "project.description");
    config.project.license = optional_string(&document, "project.license");
    config.project.repository = optional_string(&document, "project.repository");
    if let Some(authors) = optional_string_array(&document, "project.authors") {
        config.project.authors = authors;
    }

    let explicit_build_output_dir = optional_path(&document, "build.output_dir");
    if let Some(value) = explicit_build_output_dir.clone() {
        config.build.output_dir = value;
    }
    if let Some(value) = optional_path(&document, "build.cache_dir") {
        config.build.cache_dir = value;
    }
    if let Some(value) = optional_string(&document, "build.optimization_level") {
        config.build.optimization_level = parse_optimization_level(&value)?;
    }
    if let Some(value) = optional_string(&document, "build.target") {
        config.build.target = parse_build_target(&value)?;
    }
    if let Some(value) = optional_bool(&document, "build.source_maps") {
        config.build.source_maps = value;
    }
    if let Some(value) = optional_bool(&document, "build.minify") {
        config.build.minify = value;
    }
    if let Some(value) = optional_bool(&document, "build.tree_shake") {
        config.build.tree_shake = value;
    }
    if let Some(value) = optional_usize(&document, "build.parallel_jobs")? {
        config.build.parallel_jobs = Some(value);
    }

    apply_string(
        &document,
        "routing.pages_dir",
        &mut config.routing.pages_dir,
    );
    apply_string(&document, "routing.api_dir", &mut config.routing.api_dir);
    apply_string(
        &document,
        "routing.components_dir",
        &mut config.routing.components_dir,
    );
    apply_string(
        &document,
        "routing.layouts_dir",
        &mut config.routing.layouts_dir,
    );
    apply_string(
        &document,
        "routing.styles_dir",
        &mut config.routing.styles_dir,
    );
    apply_string(&document, "routing.lib_dir", &mut config.routing.lib_dir);
    if let Some(value) = optional_bool(&document, "routing.trailing_slash") {
        config.routing.trailing_slash = value;
    }
    if let Some(value) = optional_bool(&document, "routing.case_sensitive") {
        config.routing.case_sensitive = value;
    }
    if let Some(value) = optional_bool(&document, "routing.auto_index") {
        config.routing.auto_index = value;
    }

    apply_string(&document, "dev.host", &mut config.dev.host);
    if let Some(value) = optional_u16(&document, "dev.port")? {
        config.dev.port = value;
    }
    if let Some(value) = optional_bool(&document, "dev.hot_reload") {
        config.dev.hot_reload = value;
    }
    if let Some(value) = optional_bool(&document, "dev.devtools") {
        config.dev.devtools = value;
    }
    if let Some(value) = optional_string_any(&document, &["dev.server_mode", "dev.server"]) {
        config.dev.server_mode = parse_dev_server_mode(&value)?;
    }
    if let Some(value) = optional_bool(&document, "dev.open_browser") {
        config.dev.open_browser = value;
    }
    if let Some(value) = optional_path_array(&document, "dev.watch_dirs") {
        config.dev.watch_dirs = value;
    }
    if let Some(value) = optional_string_array(&document, "dev.ignore_patterns") {
        config.dev.ignore_patterns = value;
    }
    if let Some(value) = optional_u16(&document, "dev.ws_port")? {
        config.dev.ws_port = Some(value);
    }
    if let Some(value) = optional_bool(&document, "dev.https") {
        config.dev.https = value;
    }

    if let Some(value) = optional_string(&document, "languages.default") {
        config.languages.default = parse_script_language(&value)?;
    }
    if let Some(values) = optional_string_array(&document, "languages.enabled") {
        config.languages.enabled = values
            .iter()
            .map(|value| parse_script_language(value))
            .collect::<ConfigResult<Vec<_>>>()?;
    }

    if let Some(value) = optional_string(&document, "css.compiler") {
        config.css.compiler = parse_css_compiler(&value)?;
    }
    if let Some(value) = optional_bool(&document, "css.atomic_classes") {
        config.css.atomic_classes = value;
    }
    if let Some(value) = optional_bool(&document, "css.purge_unused") {
        config.css.purge_unused = value;
    }
    if let Some(value) = optional_bool(&document, "css.modules") {
        config.css.modules = value;
    }
    if let Some(value) = optional_bool(&document, "css.autoprefixer") {
        config.css.autoprefixer = value;
    }
    if let Some(value) = optional_bool(&document, "css.nesting") {
        config.css.nesting = value;
    }

    apply_string(
        &document,
        "assets.public_dir",
        &mut config.assets.public_dir,
    );
    if let Some(value) = optional_bool(&document, "assets.optimize_images") {
        config.assets.optimize_images = value;
    }
    if let Some(value) = optional_bool(&document, "assets.content_hash") {
        config.assets.content_hash = value;
    }
    if let Some(value) = optional_u32(&document, "assets.max_image_width")? {
        config.assets.max_image_width = value;
    }
    if let Some(value) = optional_u8(&document, "assets.image_quality")? {
        config.assets.image_quality = value;
    }
    if let Some(value) = optional_bool(&document, "assets.webp") {
        config.assets.webp = value;
    }
    if let Some(value) = optional_bool(&document, "assets.avif") {
        config.assets.avif = value;
    }

    apply_string_any(
        &document,
        &["tooling.biome.version", "biome.version"],
        &mut config.tooling.biome.version,
    );
    if let Some(value) = optional_bool_any(
        &document,
        &["tooling.biome.formatter.enabled", "biome.formatter"],
    ) {
        config.tooling.biome.formatter_enabled = value;
    }
    apply_string_any(
        &document,
        &["tooling.biome.formatter.indent_style", "biome.indent_style"],
        &mut config.tooling.biome.indent_style,
    );
    if let Some(value) = optional_u8_any(
        &document,
        &["tooling.biome.formatter.indent_width", "biome.indent_width"],
    )? {
        config.tooling.biome.indent_width = value;
    }
    if let Some(value) = optional_u16_any(
        &document,
        &["tooling.biome.formatter.line_width", "biome.line_width"],
    )? {
        config.tooling.biome.line_width = value;
    }
    if let Some(value) = optional_bool(&document, "tooling.biome.organize_imports.enabled") {
        config.tooling.biome.organize_imports_enabled = value;
    }
    if let Some(value) =
        optional_bool_any(&document, &["tooling.biome.linter.enabled", "biome.linter"])
    {
        config.tooling.biome.linter_enabled = value;
    }
    if let Some(value) = optional_bool_any(
        &document,
        &["tooling.biome.linter.recommended", "biome.recommended"],
    ) {
        config.tooling.biome.recommended = value;
    }
    apply_string(
        &document,
        "tooling.biome.rules.correctness.no_unused_imports",
        &mut config.tooling.biome.no_unused_imports,
    );
    apply_string(
        &document,
        "tooling.biome.rules.correctness.no_unused_variables",
        &mut config.tooling.biome.no_unused_variables,
    );
    apply_string(
        &document,
        "tooling.biome.rules.style.use_const",
        &mut config.tooling.biome.use_const,
    );
    if let Some(value) = table_value_by_key(
        &document,
        "biome_rules",
        "name",
        "no_unused_imports",
        "level",
    ) {
        config.tooling.biome.no_unused_imports = value;
    }
    if let Some(value) = table_value_by_key(
        &document,
        "biome_rules",
        "name",
        "no_unused_variables",
        "level",
    ) {
        config.tooling.biome.no_unused_variables = value;
    }
    if let Some(value) = table_value_by_key(&document, "biome_rules", "name", "use_const", "level")
    {
        config.tooling.biome.use_const = value;
    }
    if let Some(values) = optional_string_array(&document, "tooling.biome.files.ignore")
        .or_else(|| table_column_values(&document, "ignore", "path"))
    {
        config.tooling.biome.ignore = values;
    }

    apply_string_any(
        &document,
        &["tooling.dx_style.mode", "style.mode"],
        &mut config.tooling.dx_style.mode,
    );
    apply_string_any(
        &document,
        &["tooling.dx_style.dark_mode", "style.dark_mode"],
        &mut config.tooling.dx_style.dark_mode,
    );
    apply_string_any(
        &document,
        &["tooling.dx_style.tokens", "style.tokens"],
        &mut config.tooling.dx_style.tokens,
    );
    apply_string_any(
        &document,
        &["tooling.dx_style.generated_css", "style.generated_css"],
        &mut config.tooling.dx_style.generated_css,
    );
    apply_string_any(
        &document,
        &["tooling.dx_style.base_color", "style.base_color"],
        &mut config.tooling.dx_style.base_color,
    );
    if let Some(value) = optional_bool_any(
        &document,
        &["tooling.dx_style.css_variables", "style.css_variables"],
    ) {
        config.tooling.dx_style.css_variables = value;
    }

    apply_string_any(
        &document,
        &["tooling.imports.map", "imports.map"],
        &mut config.tooling.imports.map,
    );
    apply_string_any(
        &document,
        &["tooling.imports.barrel", "imports.barrel"],
        &mut config.tooling.imports.barrel,
    );
    apply_string_any(
        &document,
        &["tooling.imports.declarations", "imports.declarations"],
        &mut config.tooling.imports.declarations,
    );
    if let Some(values) = optional_string_array_any(
        &document,
        &["tooling.imports.scan_roots", "imports.scan_roots"],
    ) {
        config.tooling.imports.scan_roots = values;
    }
    if let Some(values) = optional_string_array_any(
        &document,
        &["tooling.imports.used_roots", "imports.used_roots"],
    ) {
        config.tooling.imports.used_roots = values;
    }
    if let Some(values) =
        optional_string_array_any(&document, &["tooling.imports.aliases", "imports.aliases"])
    {
        config.tooling.imports.aliases = values;
    }
    if let Some(value) = optional_bool_any(
        &document,
        &["tooling.imports.used_only", "imports.used_only"],
    ) {
        config.tooling.imports.used_only = value;
    }

    apply_string_any(
        &document,
        &["tooling.icons.component", "icons.component"],
        &mut config.tooling.icons.component,
    );
    apply_string_any(
        &document,
        &["tooling.icons.source_tag", "icons.source_tag"],
        &mut config.tooling.icons.source_tag,
    );
    apply_string_any(
        &document,
        &["tooling.icons.runtime_tag", "icons.runtime_tag"],
        &mut config.tooling.icons.runtime_tag,
    );
    apply_string_any(
        &document,
        &["tooling.icons.source", "icons.source"],
        &mut config.tooling.icons.source,
    );
    apply_string_any(
        &document,
        &["tooling.icons.generated_dir", "icons.generated_dir"],
        &mut config.tooling.icons.generated_dir,
    );

    if let Some(value) = optional_bool_any(&document, &["tooling.ui.enabled", "ui.enabled"]) {
        config.tooling.ui.enabled = value;
    }
    apply_string_any(
        &document,
        &["tooling.ui.components_dir", "ui.components_dir"],
        &mut config.tooling.ui.components_dir,
    );
    apply_string_any(
        &document,
        &["tooling.ui.source_package", "ui.source_package"],
        &mut config.tooling.ui.source_package,
    );
    apply_string_any(
        &document,
        &["tooling.ui.channel", "ui.channel"],
        &mut config.tooling.ui.channel,
    );

    apply_string_any(
        &document,
        &["tooling.classnames.mode", "classnames.mode"],
        &mut config.tooling.classnames.mode,
    );
    apply_string_any(
        &document,
        &["tooling.classnames.helper", "classnames.helper"],
        &mut config.tooling.classnames.helper,
    );
    if let Some(values) = optional_string_array(&document, "tooling.classnames.compat_helpers")
        .or_else(|| table_column_values(&document, "classnames_compat", "name"))
    {
        config.tooling.classnames.compat_helpers = values;
    }
    apply_string_any(
        &document,
        &["tooling.classnames.runtime", "classnames.runtime"],
        &mut config.tooling.classnames.runtime,
    );
    if let Some(value) = optional_bool_any(
        &document,
        &[
            "tooling.classnames.import_required",
            "classnames.import_required",
        ],
    ) {
        config.tooling.classnames.import_required = value;
    }
    if let Some(value) = optional_bool_any(
        &document,
        &[
            "tooling.classnames.scan_static_strings",
            "classnames.scan_static_strings",
        ],
    ) {
        config.tooling.classnames.scan_static_strings = value;
    }
    if let Some(value) = optional_bool_any(
        &document,
        &[
            "tooling.classnames.object_array_payloads",
            "classnames.object_array_payloads",
        ],
    ) {
        config.tooling.classnames.object_array_payloads = value;
    }

    apply_string_any(
        &document,
        &[
            "tooling.forge_ui.style",
            "forge_ui.style",
            "tooling.shadcn.style",
            "shadcn.style",
        ],
        &mut config.tooling.forge_ui.style,
    );
    if let Some(value) = optional_bool_any(
        &document,
        &[
            "tooling.forge_ui.rsc",
            "forge_ui.rsc",
            "tooling.shadcn.rsc",
            "shadcn.rsc",
        ],
    ) {
        config.tooling.forge_ui.rsc = value;
    }
    if let Some(value) = optional_bool_any(
        &document,
        &[
            "tooling.forge_ui.tsx",
            "forge_ui.tsx",
            "tooling.shadcn.tsx",
            "shadcn.tsx",
        ],
    ) {
        config.tooling.forge_ui.tsx = value;
    }
    apply_string_any(
        &document,
        &[
            "tooling.forge_ui.icon_library",
            "forge_ui.icon_library",
            "tooling.shadcn.icon_library",
            "shadcn.icon_library",
        ],
        &mut config.tooling.forge_ui.icon_library,
    );
    apply_string_any(
        &document,
        &[
            "tooling.forge_ui.aliases.components",
            "forge_ui.components_alias",
            "tooling.shadcn.aliases.components",
            "shadcn.components_alias",
        ],
        &mut config.tooling.forge_ui.components_alias,
    );
    apply_string_any(
        &document,
        &[
            "tooling.forge_ui.aliases.ui",
            "forge_ui.ui_alias",
            "tooling.shadcn.aliases.ui",
            "shadcn.ui_alias",
        ],
        &mut config.tooling.forge_ui.ui_alias,
    );
    apply_string_any(
        &document,
        &[
            "tooling.forge_ui.aliases.lib",
            "forge_ui.lib_alias",
            "tooling.shadcn.aliases.lib",
            "shadcn.lib_alias",
        ],
        &mut config.tooling.forge_ui.lib_alias,
    );
    apply_string_any(
        &document,
        &[
            "tooling.forge_ui.aliases.styles",
            "forge_ui.styles_alias",
            "tooling.shadcn.aliases.styles",
            "shadcn.styles_alias",
        ],
        &mut config.tooling.forge_ui.styles_alias,
    );

    if let Some(value) = optional_bool_any(
        &document,
        &[
            "framework.www.app_router",
            "framework.next.app_router",
            "www.app_router",
        ],
    ) {
        config.framework.www.app_router = value;
    }
    apply_string_any(
        &document,
        &[
            "framework.www.config_owner_file",
            "framework.next.config_owner_file",
            "www.config_owner_file",
        ],
        &mut config.framework.www.config_owner_file,
    );
    if let Some(values) = optional_string_array_any(
        &document,
        &[
            "framework.www.config_files",
            "framework.next.config_files",
            "www.config_files",
        ],
    ) {
        config.framework.www.config_files = values;
    }
    apply_string_any(
        &document,
        &[
            "framework.www.app_dir",
            "framework.next.app_dir",
            "www.app_dir",
        ],
        &mut config.framework.www.app_dir,
    );
    apply_string_any(
        &document,
        &[
            "framework.www.route_root",
            "framework.next.route_root",
            "www.route_root",
        ],
        &mut config.framework.www.route_root,
    );
    apply_string_any(
        &document,
        &[
            "framework.www.server_components",
            "framework.next.server_components",
            "www.server_components",
        ],
        &mut config.framework.www.server_components,
    );
    if let Some(value) = optional_bool_any(
        &document,
        &[
            "framework.www.turbopack_runtime",
            "framework.next.turbopack_runtime",
            "www.turbopack_runtime",
        ],
    ) {
        config.framework.www.turbopack_runtime = value;
    }
    if explicit_build_output_dir.is_none()
        && let Some(value) = optional_path_any(
            &document,
            &[
                "framework.www.output_dir",
                "framework.next.output_dir",
                "www.output_dir",
            ],
        )
    {
        config.build.output_dir = value;
    }

    if let Some(value) = optional_bool_any(
        &document,
        &[
            "framework.fumadocs.enabled",
            "fumadocs.enabled",
            "docs.enabled",
        ],
    ) {
        config.framework.fumadocs.enabled = value;
    } else if optional_value(&document, "docs").is_some() {
        config.framework.fumadocs.enabled = true;
    }
    apply_string_any(
        &document,
        &["framework.fumadocs.package_id", "fumadocs.package_id"],
        &mut config.framework.fumadocs.package_id,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.config_owner_file",
            "fumadocs.config_owner_file",
        ],
        &mut config.framework.fumadocs.config_owner_file,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.docs_route",
            "fumadocs.docs_route",
            "docs.route",
        ],
        &mut config.framework.fumadocs.docs_route,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.readiness_route",
            "fumadocs.readiness_route",
        ],
        &mut config.framework.fumadocs.readiness_route,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.content_dir",
            "fumadocs.content_dir",
            "docs.content",
        ],
        &mut config.framework.fumadocs.content_dir,
    );
    apply_string_any(
        &document,
        &["framework.fumadocs.source_file", "fumadocs.source_file"],
        &mut config.framework.fumadocs.source_file,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.source_plugin_file",
            "fumadocs.source_plugin_file",
        ],
        &mut config.framework.fumadocs.source_plugin_file,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.source_plugin_icon_surface",
            "fumadocs.source_plugin_icon_surface",
        ],
        &mut config.framework.fumadocs.source_plugin_icon_surface,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.icon_component_file",
            "fumadocs.icon_component_file",
        ],
        &mut config.framework.fumadocs.icon_component_file,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.layout_options_file",
            "fumadocs.layout_options_file",
        ],
        &mut config.framework.fumadocs.layout_options_file,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.openapi_schema_file",
            "fumadocs.openapi_schema_file",
            "docs.openapi",
        ],
        &mut config.framework.fumadocs.openapi_schema_file,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.openapi_proxy_route",
            "fumadocs.openapi_proxy_route",
        ],
        &mut config.framework.fumadocs.openapi_proxy_route,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.openapi_allowed_origins_env",
            "fumadocs.openapi_allowed_origins_env",
        ],
        &mut config.framework.fumadocs.openapi_allowed_origins_env,
    );
    apply_string_any(
        &document,
        &["framework.fumadocs.search_route", "fumadocs.search_route"],
        &mut config.framework.fumadocs.search_route,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.static_search_route",
            "fumadocs.static_search_route",
        ],
        &mut config.framework.fumadocs.static_search_route,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.llms_index_route",
            "fumadocs.llms_index_route",
        ],
        &mut config.framework.fumadocs.llms_index_route,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.llms_full_route",
            "fumadocs.llms_full_route",
        ],
        &mut config.framework.fumadocs.llms_full_route,
    );
    apply_string_any(
        &document,
        &[
            "framework.fumadocs.llms_page_markdown_route",
            "fumadocs.llms_page_markdown_route",
        ],
        &mut config.framework.fumadocs.llms_page_markdown_route,
    );
    if let Some(values) = optional_string_array(&document, "framework.fumadocs.generated_routes")
        .or_else(|| table_column_values(&document, "fumadocs_routes", "route"))
    {
        config.framework.fumadocs.generated_routes = values;
    }
    if config.framework.fumadocs.enabled
        && config.framework.fumadocs.generated_routes.is_empty()
        && optional_value(&document, "docs").is_some()
    {
        config.framework.fumadocs.generated_routes = vec![
            config.framework.fumadocs.docs_route.clone(),
            config.framework.fumadocs.readiness_route.clone(),
            config.framework.fumadocs.search_route.clone(),
        ];
    }
    if let Some(values) =
        optional_string_array(&document, "framework.fumadocs.required_runtime_packages")
            .or_else(|| table_column_values(&document, "fumadocs_runtime_packages", "name"))
    {
        config.framework.fumadocs.required_runtime_packages = values;
    }

    if let Some(value) = optional_bool(&document, "server.ssr") {
        config.server.ssr = value;
    }
    apply_string(
        &document,
        "server.api_prefix",
        &mut config.server.api_prefix,
    );
    if let Some(value) = optional_bool(&document, "server.cors_enabled") {
        config.server.cors_enabled = value;
    }
    if let Some(value) = optional_string_array(&document, "server.cors_origins") {
        config.server.cors_origins = value;
    }
    if let Some(value) = optional_bool(&document, "server.compression") {
        config.server.compression = value;
    }
    if let Some(value) = optional_bool(&document, "server.request_logging") {
        config.server.request_logging = value;
    }
    if let Some(value) = optional_u64(&document, "server.timeout")? {
        config.server.timeout = value;
    }

    config.validate()?;
    Ok(config)
}

fn apply_string(document: &DxDocument, key: &str, target: &mut String) {
    if let Some(value) = optional_string(document, key) {
        *target = value;
    }
}

fn apply_string_any(document: &DxDocument, keys: &[&str], target: &mut String) {
    if let Some(value) = keys.iter().find_map(|key| optional_string(document, key)) {
        *target = value;
    }
}

fn optional_string_any(document: &DxDocument, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| optional_string(document, key))
}

fn optional_value<'a>(document: &'a DxDocument, key: &str) -> Option<&'a DxLlmValue> {
    document.get_path(key)
}

fn optional_string(document: &DxDocument, key: &str) -> Option<String> {
    optional_value(document, key).map(value_to_string)
}

fn optional_string_array(document: &DxDocument, key: &str) -> Option<Vec<String>> {
    optional_value(document, key).map(|value| match value {
        DxLlmValue::Arr(items) => items.iter().map(value_to_string).collect(),
        value => vec![value_to_string(value)],
    })
}

fn optional_string_array_any(document: &DxDocument, keys: &[&str]) -> Option<Vec<String>> {
    keys.iter()
        .find_map(|key| optional_string_array(document, key))
}

fn optional_path(document: &DxDocument, key: &str) -> Option<PathBuf> {
    optional_string(document, key).map(PathBuf::from)
}

fn optional_path_any(document: &DxDocument, keys: &[&str]) -> Option<PathBuf> {
    keys.iter().find_map(|key| optional_path(document, key))
}

fn optional_path_array(document: &DxDocument, key: &str) -> Option<Vec<PathBuf>> {
    optional_string_array(document, key).map(|items| items.into_iter().map(PathBuf::from).collect())
}

fn optional_bool(document: &DxDocument, key: &str) -> Option<bool> {
    optional_value(document, key).and_then(|value| match value {
        DxLlmValue::Bool(value) => Some(*value),
        DxLlmValue::Str(value) if value.eq_ignore_ascii_case("true") => Some(true),
        DxLlmValue::Str(value) if value.eq_ignore_ascii_case("false") => Some(false),
        _ => None,
    })
}

fn optional_bool_any(document: &DxDocument, keys: &[&str]) -> Option<bool> {
    keys.iter().find_map(|key| optional_bool(document, key))
}

fn optional_u8(document: &DxDocument, key: &str) -> ConfigResult<Option<u8>> {
    optional_integer(document, key, "u8").map(|value| value.map(|value| value as u8))
}

fn optional_u16(document: &DxDocument, key: &str) -> ConfigResult<Option<u16>> {
    optional_integer(document, key, "u16").map(|value| value.map(|value| value as u16))
}

fn optional_u8_any(document: &DxDocument, keys: &[&str]) -> ConfigResult<Option<u8>> {
    for key in keys {
        if let Some(value) = optional_u8(document, key)? {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn optional_u16_any(document: &DxDocument, keys: &[&str]) -> ConfigResult<Option<u16>> {
    for key in keys {
        if let Some(value) = optional_u16(document, key)? {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn optional_u32(document: &DxDocument, key: &str) -> ConfigResult<Option<u32>> {
    optional_integer(document, key, "u32").map(|value| value.map(|value| value as u32))
}

fn optional_u64(document: &DxDocument, key: &str) -> ConfigResult<Option<u64>> {
    optional_integer(document, key, "u64")
}

fn optional_usize(document: &DxDocument, key: &str) -> ConfigResult<Option<usize>> {
    optional_integer(document, key, "usize").map(|value| value.map(|value| value as usize))
}

fn optional_integer(
    document: &DxDocument,
    key: &str,
    type_name: &str,
) -> ConfigResult<Option<u64>> {
    let Some(value) = optional_value(document, key) else {
        return Ok(None);
    };
    match value {
        DxLlmValue::Num(value) if *value >= 0.0 && value.fract() == 0.0 => Ok(Some(*value as u64)),
        DxLlmValue::Str(value) => value
            .parse::<u64>()
            .map(Some)
            .map_err(|_| invalid_value(key, format!("expected {type_name}, got `{value}`"))),
        _ => Err(invalid_value(
            key,
            format!("expected {type_name}, got {}", value.type_name()),
        )),
    }
}

fn value_to_string(value: &DxLlmValue) -> String {
    match value {
        DxLlmValue::Str(value) | DxLlmValue::Ref(value) => value.clone(),
        DxLlmValue::Num(value) if value.fract() == 0.0 => (*value as i64).to_string(),
        DxLlmValue::Num(value) => value.to_string(),
        DxLlmValue::Bool(value) => value.to_string(),
        DxLlmValue::Null => "null".to_string(),
        DxLlmValue::Arr(_) | DxLlmValue::Obj(_) => value.to_string(),
    }
}

fn table_column_values(
    document: &DxDocument,
    section_name: &str,
    column: &str,
) -> Option<Vec<String>> {
    document
        .section_by_name(section_name)?
        .column_values(column)
        .map(|values| values.into_iter().map(value_to_string).collect())
}

fn table_value_by_key(
    document: &DxDocument,
    section_name: &str,
    key_column: &str,
    key: &str,
    value_column: &str,
) -> Option<String> {
    document
        .section_by_name(section_name)?
        .value_by_key(key_column, key, value_column)
        .map(value_to_string)
}

fn parse_optimization_level(value: &str) -> ConfigResult<OptimizationLevel> {
    match value.to_ascii_lowercase().as_str() {
        "debug" => Ok(OptimizationLevel::Debug),
        "release" => Ok(OptimizationLevel::Release),
        "size" => Ok(OptimizationLevel::Size),
        _ => Err(invalid_value(
            "build.optimization_level",
            format!("unsupported optimization level `{value}`"),
        )),
    }
}

fn parse_build_target(value: &str) -> ConfigResult<BuildTarget> {
    match value.to_ascii_lowercase().as_str() {
        "web" => Ok(BuildTarget::Web),
        "server" => Ok(BuildTarget::Server),
        "edge" => Ok(BuildTarget::Edge),
        "static" => Ok(BuildTarget::Static),
        _ => Err(invalid_value(
            "build.target",
            format!("unsupported build target `{value}`"),
        )),
    }
}

fn parse_script_language(value: &str) -> ConfigResult<ScriptLanguage> {
    ScriptLanguage::parse(value).ok_or_else(|| {
        invalid_value(
            "languages",
            format!("unsupported script language `{value}`"),
        )
    })
}

fn parse_css_compiler(value: &str) -> ConfigResult<CssCompiler> {
    match value.to_ascii_lowercase().as_str() {
        "dxstyle" | "dx-style" | "dx_style" => Ok(CssCompiler::DxStyle),
        "lightning" | "lightningcss" | "lightning-css" => Ok(CssCompiler::Lightning),
        "none" => Ok(CssCompiler::None),
        _ => Err(invalid_value(
            "css.compiler",
            format!("unsupported css compiler `{value}`"),
        )),
    }
}

fn parse_dev_server_mode(value: &str) -> ConfigResult<DxDevServerMode> {
    DxDevServerMode::from_config_value(value).ok_or_else(|| {
        invalid_value(
            "dev.server_mode",
            format!("unsupported dev server mode `{value}`"),
        )
    })
}

fn invalid_value(field: &str, message: String) -> ConfigError {
    ConfigError::InvalidValue {
        field: field.to_string(),
        message,
    }
}
