use std::path::{Path, PathBuf};

use dx_compiler::delivery::{
    DxDeliveryMode, DxVerticalComponentSource, DxVerticalSliceInput, DxVerticalSliceProof,
    compile_vertical_slice,
};
use dx_compiler::ecosystem::{
    DxForgeAddOutcome, DxForgeLocalSourceFile, DxForgeLocalSourcePackage, DxUpdateTraffic,
    canonical_package_id, plan_forge_add, plan_forge_local_source, write_forge_add,
    write_forge_local_source,
};
use serde::Serialize;

use crate::error::{DxError, DxResult};

use super::prove_fixtures::{
    DxForgeLaunchClaimsManifest, DxForgeLaunchEvidenceManifest, DxVerticalFixtureSource,
    DxVerticalProofFixture,
};
use super::prove_runtime::{DXPK_RUNTIME_FIXTURE_JS, inject_dxpk_runtime_fixture};
use super::{Cli, DxOutputFormat, forge_error, resolve_cli_path};

#[derive(Debug, Serialize)]
struct DxVerticalCliSummary {
    route: String,
    page: String,
    components: Vec<String>,
    missing_components: Vec<String>,
    fallback: DxVerticalFallbackSummary,
    packet: DxVerticalPacketSummary,
    browser_packet: DxVerticalBrowserPacketSummary,
    interaction: Option<DxVerticalInteractionSummary>,
    forge_packages: Vec<DxVerticalForgeSummary>,
    forge: Option<DxVerticalForgeSummary>,
    written: Option<DxVerticalOutputSummary>,
}

impl DxVerticalCliSummary {
    fn new(
        proof: &DxVerticalSliceProof,
        written: Option<&DxVerticalOutputSummary>,
        forge_packages: &[DxVerticalForgeSummary],
        forge: Option<&DxVerticalForgeSummary>,
    ) -> Self {
        let profile = proof.fallback.profile();
        Self {
            route: proof.route.clone(),
            page: proof.page.name.clone(),
            components: proof
                .components
                .iter()
                .map(|component| component.name.clone())
                .collect(),
            missing_components: proof.missing_components.clone(),
            fallback: DxVerticalFallbackSummary {
                optimized_bytes: profile.optimized_bytes,
                saved_bytes: profile.saved_bytes,
                script_count: profile.script_count,
                style_count: profile.style_count,
                repeated_node_count: profile.repeated_node_count,
                delivery_mode: profile.delivery_mode.as_str().to_string(),
            },
            packet: DxVerticalPacketSummary::new(&proof.packet),
            browser_packet: DxVerticalBrowserPacketSummary::new(&proof.browser_packet),
            interaction: proof
                .interaction
                .as_ref()
                .map(DxVerticalInteractionSummary::new),
            forge_packages: forge_packages.to_vec(),
            forge: forge.cloned(),
            written: written.cloned(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct DxVerticalFallbackSummary {
    optimized_bytes: usize,
    saved_bytes: usize,
    script_count: usize,
    style_count: usize,
    repeated_node_count: usize,
    delivery_mode: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxVerticalOutputSummary {
    html_path: PathBuf,
    packet_path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    claims_manifest_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence_manifest_path: Option<PathBuf>,
    summary_path: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct DxVerticalPacketSummary {
    format: String,
    bytes: usize,
    decoded_name: String,
    template_count: usize,
    string_count: usize,
    binding_count: usize,
    event_count: usize,
    css_class_count: usize,
    roundtrip_matches: bool,
}

impl DxVerticalPacketSummary {
    fn new(packet: &dx_compiler::delivery::DxVerticalPacketProof) -> Self {
        Self {
            format: packet.format.clone(),
            bytes: packet.bytes,
            decoded_name: packet.decoded_name.clone(),
            template_count: packet.decoded_templates.len(),
            string_count: packet.decoded_strings.len(),
            binding_count: packet.decoded_bindings.len(),
            event_count: packet.decoded_event_count,
            css_class_count: packet.decoded_css_classes.len(),
            roundtrip_matches: packet.roundtrip_matches,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct DxVerticalBrowserPacketSummary {
    format: String,
    bytes: usize,
    decoded_kind: String,
    section_count: usize,
    payload_bytes: u32,
    roundtrip_matches: bool,
    sections: Vec<DxVerticalBrowserPacketSectionSummary>,
}

impl DxVerticalBrowserPacketSummary {
    fn new(packet: &dx_compiler::delivery::DxVerticalBrowserPacketProof) -> Self {
        Self {
            format: packet.format.clone(),
            bytes: packet.bytes,
            decoded_kind: format!("{:?}", packet.decoded_kind),
            section_count: packet.section_count,
            payload_bytes: packet.payload_bytes,
            roundtrip_matches: packet.roundtrip_matches,
            sections: packet
                .decoded_sections
                .iter()
                .map(DxVerticalBrowserPacketSectionSummary::new)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct DxVerticalBrowserPacketSectionSummary {
    kind: String,
    encoding: String,
    bytes: usize,
    content_hash: String,
}

impl DxVerticalBrowserPacketSectionSummary {
    fn new(section: &dx_compiler::delivery::DxVerticalBrowserPacketSectionProof) -> Self {
        Self {
            kind: format!("{:?}", section.kind),
            encoding: format!("{:?}", section.encoding),
            bytes: section.bytes,
            content_hash: section.content_hash.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct DxVerticalInteractionSummary {
    delivery_mode: String,
    state_name: String,
    target_id: String,
    action_count: usize,
    script_bytes: usize,
    warnings: Vec<String>,
}

impl DxVerticalInteractionSummary {
    fn new(interaction: &dx_compiler::delivery::DxVerticalInteractionProof) -> Self {
        Self {
            delivery_mode: interaction.delivery_mode.as_str().to_string(),
            state_name: interaction.state_name.clone(),
            target_id: interaction.target_id.clone(),
            action_count: interaction.program.actions.len(),
            script_bytes: interaction.script_bytes,
            warnings: interaction.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct DxVerticalForgeSummary {
    package_id: String,
    action: String,
    risk_score: u8,
    traffic: String,
    files_tracked: usize,
    manifest_path: Option<PathBuf>,
    receipt_path: Option<PathBuf>,
}

impl DxVerticalForgeSummary {
    fn new(outcome: &DxForgeAddOutcome) -> Self {
        Self {
            package_id: outcome.receipt.package.package_id.clone(),
            action: format!("{:?}", outcome.receipt.action),
            risk_score: outcome.receipt.risk_score,
            traffic: receipt_traffic(outcome).as_str().to_string(),
            files_tracked: outcome.receipt.files_written.len(),
            manifest_path: outcome.manifest_path.clone(),
            receipt_path: outcome.receipt_path.clone(),
        }
    }
}

impl Cli {
    /// Run proof-oriented compiler smoke commands.
    pub fn cmd_prove(&self, args: &[String]) -> DxResult<()> {
        if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
            Self::print_prove_help();
            return Ok(());
        }

        match args[0].as_str() {
            "vertical" => self.cmd_prove_vertical(&args[1..]),
            command => Err(DxError::ConfigValidationError {
                message: format!("Unknown prove command: {command}"),
                field: Some("prove".to_string()),
            }),
        }
    }

    fn print_prove_help() {
        eprintln!("dx prove: compiler-backed product proofs");
        eprintln!();
        eprintln!("USAGE:");
        eprintln!(
            "    dx prove vertical --page <path.html> [--component <path.tsx> ...] [--package <id> ...]"
        );
        eprintln!(
            "    dx prove vertical --fixture forge-site|forge-scorecard|forge-ci|forge-evidence|forge-releases|forge-changelog|forge-quickstart|forge-adoption [--route <route>] [--out <dir>]"
        );
        eprintln!("                      [--route <route>] [--out <dir>] [--dry-run|--write]");
        eprintln!("                      [--format terminal|json|markdown] [--quiet]");
    }

    fn cmd_prove_vertical(&self, args: &[String]) -> DxResult<()> {
        let mut page: Option<PathBuf> = None;
        let mut components = Vec::new();
        let mut forge_packages = Vec::new();
        let mut fixture: Option<DxVerticalProofFixture> = None;
        let mut route: Option<String> = None;
        let mut out_dir = self.cwd.join(".dx").join("vertical");
        let mut write = false;
        let mut format = DxOutputFormat::Terminal;
        let mut claims_manifest = None;
        let mut evidence_manifest = None;
        let mut quiet = false;

        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--page" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| DxError::ConfigValidationError {
                            message: "--page requires a path".to_string(),
                            field: Some("prove vertical".to_string()),
                        })?;
                    page = Some(resolve_cli_path(&self.cwd, value));
                }
                "--component" | "--cp" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| DxError::ConfigValidationError {
                            message: "--component requires a path".to_string(),
                            field: Some("prove vertical".to_string()),
                        })?;
                    components.push(resolve_cli_path(&self.cwd, value));
                }
                "--package" | "--forge-package" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| DxError::ConfigValidationError {
                            message: "--package requires a Forge package id".to_string(),
                            field: Some("prove vertical".to_string()),
                        })?;
                    forge_packages.push(value.clone());
                }
                "--fixture" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| DxError::ConfigValidationError {
                            message: "--fixture requires a fixture name".to_string(),
                            field: Some("prove vertical".to_string()),
                        })?;
                    fixture = Some(DxVerticalProofFixture::parse(value)?);
                }
                "--route" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| DxError::ConfigValidationError {
                            message: "--route requires a route".to_string(),
                            field: Some("prove vertical".to_string()),
                        })?;
                    route = Some(normalize_route(value));
                }
                "--out" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| DxError::ConfigValidationError {
                            message: "--out requires a directory".to_string(),
                            field: Some("prove vertical".to_string()),
                        })?;
                    out_dir = resolve_cli_path(&self.cwd, value);
                }
                "--write" => write = true,
                "--dry-run" => write = false,
                "--format" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| DxError::ConfigValidationError {
                            message: "--format requires terminal, json, or markdown".to_string(),
                            field: Some("prove vertical".to_string()),
                        })?;
                    format = DxOutputFormat::parse(value)?;
                }
                "--quiet" => quiet = true,
                value => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown prove vertical option: {value}"),
                        field: Some("prove vertical".to_string()),
                    });
                }
            }
            index += 1;
        }

        let mut precomputed_package_outcomes = None;
        let (page, page_source, component_sources) = if let Some(fixture) = fixture {
            if page.is_some() || !components.is_empty() || !forge_packages.is_empty() {
                return Err(DxError::ConfigValidationError {
                    message: "--fixture cannot be combined with --page, --component, or --package"
                        .to_string(),
                    field: Some("prove vertical".to_string()),
                });
            }
            let mut source = fixture.source()?;
            let page = self.cwd.join(&source.page_path);
            forge_packages = source.packages.clone();
            route = route.or_else(|| Some(normalize_route(&source.route)));
            if write {
                ensure_fixture_source_writable(&self.cwd, &source)?;
            }
            let package_outcomes = forge_packages
                .iter()
                .map(|package_id| {
                    if write {
                        write_forge_add(package_id, &self.cwd).map_err(forge_error)
                    } else {
                        plan_forge_add(package_id, &self.cwd).map_err(forge_error)
                    }
                })
                .collect::<DxResult<Vec<_>>>()?;
            if write {
                let benchmark_history = self
                    .cwd
                    .join("benchmarks/reports/vertical-proof-history/index.json");
                source = fixture.source_for_project(&self.cwd, &benchmark_history)?;
                write_fixture_source(&self.cwd, &source)?;
            }
            claims_manifest = source.claims_manifest.clone();
            evidence_manifest = source.evidence_manifest.clone();
            let page_source = source.page_source.clone();
            precomputed_package_outcomes = Some(package_outcomes);
            (page, page_source, Vec::new())
        } else {
            let page = page.ok_or_else(|| DxError::ConfigValidationError {
                message: "dx prove vertical requires --page <path.html>".to_string(),
                field: Some("prove vertical".to_string()),
            })?;
            let page_source = read_text_file(&page)?;
            let component_sources = components
                .iter()
                .map(|path| read_vertical_component(path))
                .collect::<DxResult<Vec<_>>>()?;
            (page, page_source, component_sources)
        };
        let mut proof_component_sources = component_sources.clone();
        let package_outcomes = if let Some(package_outcomes) = precomputed_package_outcomes {
            package_outcomes
        } else {
            forge_packages
                .iter()
                .map(|package_id| {
                    if write {
                        write_forge_add(package_id, &self.cwd).map_err(forge_error)
                    } else {
                        plan_forge_add(package_id, &self.cwd).map_err(forge_error)
                    }
                })
                .collect::<DxResult<Vec<_>>>()?
        };

        for package_id in &forge_packages {
            for component in forge_vertical_component_sources(package_id)? {
                if !proof_component_sources
                    .iter()
                    .any(|existing| existing.name == component.name)
                {
                    proof_component_sources.push(component);
                }
            }
        }

        let route = route.unwrap_or_else(|| default_route_for_page(&self.cwd, &page));
        let forge_input = build_vertical_forge_package(
            &self.cwd,
            &route,
            &page,
            &page_source,
            &components,
            &component_sources,
        )?;
        let proof = compile_vertical_slice(DxVerticalSliceInput {
            route,
            page_source,
            components: proof_component_sources,
        })
        .map_err(forge_error)?;

        let forge_outcome = if write {
            write_forge_local_source(forge_input, &self.cwd).map_err(forge_error)?
        } else {
            plan_forge_local_source(forge_input).map_err(forge_error)?
        };
        let forge_package_summaries = package_outcomes
            .iter()
            .map(DxVerticalForgeSummary::new)
            .collect::<Vec<_>>();
        let forge_summary = DxVerticalForgeSummary::new(&forge_outcome);

        let mut output = None;
        if write {
            output = Some(write_vertical_proof(
                &out_dir,
                &proof,
                &forge_package_summaries,
                Some(&forge_summary),
                claims_manifest.as_ref(),
                evidence_manifest.as_ref(),
            )?);
        }

        let summary = DxVerticalCliSummary::new(
            &proof,
            output.as_ref(),
            &forge_package_summaries,
            Some(&forge_summary),
        );
        if !quiet {
            match format {
                DxOutputFormat::Terminal => print_vertical_summary_terminal(&summary),
                DxOutputFormat::Json => println!(
                    "{}",
                    serde_json::to_string_pretty(&summary).map_err(forge_error)?
                ),
                DxOutputFormat::Markdown => println!("{}", vertical_summary_markdown(&summary)),
            }
        }

        Ok(())
    }
}

fn write_fixture_source(project: &Path, source: &DxVerticalFixtureSource) -> DxResult<()> {
    ensure_fixture_source_writable(project, source)?;

    let target = project.join(&source.page_path);
    if target.exists() {
        return Ok(());
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }

    std::fs::write(&target, &source.page_source).map_err(|error| DxError::IoError {
        path: Some(target),
        message: error.to_string(),
    })
}

fn ensure_fixture_source_writable(
    project: &Path,
    source: &DxVerticalFixtureSource,
) -> DxResult<()> {
    let target = project.join(&source.page_path);
    if !target.exists() {
        return Ok(());
    }

    let current = read_text_file(&target)?;
    if current != source.page_source {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Refusing to overwrite existing fixture route `{}` with different content",
                target.display()
            ),
            field: Some("prove vertical fixture".to_string()),
        });
    }

    Ok(())
}

fn read_text_file(path: &Path) -> DxResult<String> {
    std::fs::read_to_string(path).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })
}

fn read_vertical_component(path: &Path) -> DxResult<DxVerticalComponentSource> {
    let name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.trim().is_empty())
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("Component path needs a valid file stem: {}", path.display()),
            field: Some("component".to_string()),
        })?
        .to_string();

    Ok(DxVerticalComponentSource {
        name,
        source: read_text_file(path)?,
    })
}

fn build_vertical_forge_package(
    project: &Path,
    route: &str,
    page_path: &Path,
    page_source: &str,
    component_paths: &[PathBuf],
    component_sources: &[DxVerticalComponentSource],
) -> DxResult<DxForgeLocalSourcePackage> {
    let mut files = vec![DxForgeLocalSourceFile {
        path: project_relative_file(project, page_path)?,
        content: page_source.to_string(),
    }];

    for (path, source) in component_paths.iter().zip(component_sources.iter()) {
        files.push(DxForgeLocalSourceFile {
            path: project_relative_file(project, path)?,
            content: source.source.clone(),
        });
    }

    Ok(DxForgeLocalSourcePackage {
        package_id: vertical_package_id(route),
        variant: "default".to_string(),
        upstream_name: "local:dx-www/vertical".to_string(),
        version: "0.0.0-local".to_string(),
        license: "UNLICENSED".to_string(),
        files,
    })
}

fn forge_vertical_component_sources(package_id: &str) -> DxResult<Vec<DxVerticalComponentSource>> {
    match canonical_package_id(package_id) {
        "shadcn/ui/button" => Ok(vec![DxVerticalComponentSource {
            name: "Button".to_string(),
            source: r#"
<component>
    <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md bg-neutral-950 px-4 py-2 text-sm font-medium text-white shadow-sm">
        <slot />
    </button>
</component>
"#
            .to_string(),
        }]),
        "dx/icon/search" => Ok(vec![DxVerticalComponentSource {
            name: "SearchIcon".to_string(),
            source: r#"
<component>
    <svg class="size-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <circle cx="11" cy="11" r="8" />
        <path d="m21 21-4.35-4.35" />
    </svg>
</component>
"#
            .to_string(),
        }]),
        other => Err(DxError::ConfigValidationError {
            message: format!(
                "dx prove vertical has no compiler proof projection for Forge package `{other}`"
            ),
            field: Some("package".to_string()),
        }),
    }
}

fn project_relative_file(project: &Path, path: &Path) -> DxResult<String> {
    if !path.exists() {
        return project_relative_virtual_file(project, path);
    }

    let project = std::fs::canonicalize(project).map_err(|error| DxError::IoError {
        path: Some(project.to_path_buf()),
        message: error.to_string(),
    })?;
    let path = std::fs::canonicalize(path).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })?;
    let relative = path
        .strip_prefix(&project)
        .map_err(|_| DxError::ConfigValidationError {
            message: format!(
                "Forge vertical proof inputs must live under the project root: {}",
                path.display()
            ),
            field: Some("prove vertical".to_string()),
        })?;

    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/"))
}

fn project_relative_virtual_file(project: &Path, path: &Path) -> DxResult<String> {
    let project = if project.is_absolute() {
        project.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| DxError::IoError {
                path: Some(project.to_path_buf()),
                message: error.to_string(),
            })?
            .join(project)
    };
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        project.join(path)
    };
    let relative = path
        .strip_prefix(&project)
        .map_err(|_| DxError::ConfigValidationError {
            message: format!(
                "Forge vertical proof inputs must live under the project root: {}",
                path.display()
            ),
            field: Some("prove vertical".to_string()),
        })?;

    let mut segments = Vec::new();
    for component in relative.components() {
        match component {
            std::path::Component::Normal(value) => {
                segments.push(value.to_string_lossy().to_string());
            }
            std::path::Component::CurDir => {}
            _ => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Forge vertical proof inputs cannot escape the project root: {}",
                        path.display()
                    ),
                    field: Some("prove vertical".to_string()),
                });
            }
        }
    }

    if segments.is_empty() {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Forge vertical proof input must be a file under the project root: {}",
                path.display()
            ),
            field: Some("prove vertical".to_string()),
        });
    }

    Ok(segments.join("/"))
}

fn normalize_route(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "/" {
        "/".to_string()
    } else {
        format!("/{}", trimmed.trim_matches('/'))
    }
}

fn default_route_for_page(cwd: &Path, page: &Path) -> String {
    let pages_root = cwd.join("pages");
    let relative = page.strip_prefix(&pages_root).unwrap_or(page);
    let without_extension = relative.with_extension("");
    let mut segments = without_extension
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .filter(|segment| *segment != "index")
        .map(str::to_string)
        .collect::<Vec<_>>();

    if segments.is_empty() {
        return "/".to_string();
    }

    for segment in &mut segments {
        *segment = segment.replace('\\', "/");
    }

    normalize_route(&segments.join("/"))
}

fn vertical_package_id(route: &str) -> String {
    let suffix = route
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(sanitize_package_segment)
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if suffix.is_empty() {
        "dx-www/vertical/index".to_string()
    } else {
        format!("dx-www/vertical/{suffix}")
    }
}

fn sanitize_package_segment(segment: &str) -> String {
    segment
        .chars()
        .map(|value| {
            if value.is_ascii_alphanumeric() || matches!(value, '-' | '_') {
                value.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn write_vertical_proof(
    out_dir: &Path,
    proof: &DxVerticalSliceProof,
    forge_packages: &[DxVerticalForgeSummary],
    forge: Option<&DxVerticalForgeSummary>,
    claims_manifest: Option<&DxForgeLaunchClaimsManifest>,
    evidence_manifest: Option<&DxForgeLaunchEvidenceManifest>,
) -> DxResult<DxVerticalOutputSummary> {
    let html_path = out_dir.join(route_output_file(&proof.route));
    let packet_path = route_output_packet_file(&proof.route, out_dir);
    let static_route_delivery = is_static_route_delivery(proof);
    let planned_runtime_path = route_output_runtime_file(&proof.route, out_dir);
    let runtime_path = (!static_route_delivery).then_some(planned_runtime_path.clone());
    let claims_manifest_path =
        claims_manifest.map(|_| route_output_claims_manifest_file(&proof.route, out_dir));
    let evidence_manifest_path =
        evidence_manifest.map(|_| route_output_evidence_manifest_file(&proof.route, out_dir));
    if let Some(parent) = html_path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    if let Some(parent) = runtime_path.as_ref().and_then(|path| path.parent()) {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    let packet_file_name = route_file_name(&packet_path)?;
    let html = if let Some(runtime_path) = &runtime_path {
        let runtime_file_name = route_file_name(runtime_path)?;
        inject_dxpk_runtime_fixture(proof.fallback.html(), &packet_file_name, &runtime_file_name)
    } else {
        remove_stale_runtime_fixture(&planned_runtime_path)?;
        proof.fallback.html().to_string()
    };
    std::fs::write(&html_path, html).map_err(|error| DxError::IoError {
        path: Some(html_path.clone()),
        message: error.to_string(),
    })?;
    std::fs::write(&packet_path, &proof.browser_packet.encoded).map_err(|error| {
        DxError::IoError {
            path: Some(packet_path.clone()),
            message: error.to_string(),
        }
    })?;
    if let Some(runtime_path) = &runtime_path {
        std::fs::write(runtime_path, DXPK_RUNTIME_FIXTURE_JS).map_err(|error| {
            DxError::IoError {
                path: Some(runtime_path.clone()),
                message: error.to_string(),
            }
        })?;
    }
    if let (Some(manifest), Some(path)) = (claims_manifest, claims_manifest_path.as_ref()) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: error.to_string(),
            })?;
        }
        std::fs::write(
            path,
            serde_json::to_string_pretty(manifest).map_err(forge_error)?,
        )
        .map_err(|error| DxError::IoError {
            path: Some(path.clone()),
            message: error.to_string(),
        })?;
    }
    if let (Some(manifest), Some(path)) = (evidence_manifest, evidence_manifest_path.as_ref()) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: error.to_string(),
            })?;
        }
        std::fs::write(
            path,
            serde_json::to_string_pretty(manifest).map_err(forge_error)?,
        )
        .map_err(|error| DxError::IoError {
            path: Some(path.clone()),
            message: error.to_string(),
        })?;
    }

    std::fs::create_dir_all(out_dir).map_err(|error| DxError::IoError {
        path: Some(out_dir.to_path_buf()),
        message: error.to_string(),
    })?;
    let summary_path = out_dir.join("proof.json");
    let output_summary = DxVerticalOutputSummary {
        html_path: html_path.clone(),
        packet_path: packet_path.clone(),
        runtime_path: runtime_path.clone(),
        claims_manifest_path: claims_manifest_path.clone(),
        evidence_manifest_path: evidence_manifest_path.clone(),
        summary_path: summary_path.clone(),
    };
    let summary = DxVerticalCliSummary::new(proof, Some(&output_summary), forge_packages, forge);
    std::fs::write(
        &summary_path,
        serde_json::to_string_pretty(&summary).map_err(forge_error)?,
    )
    .map_err(|error| DxError::IoError {
        path: Some(summary_path.clone()),
        message: error.to_string(),
    })?;

    Ok(output_summary)
}

fn is_static_route_delivery(proof: &DxVerticalSliceProof) -> bool {
    proof.interaction.is_none() && proof.fallback.profile().delivery_mode == DxDeliveryMode::Static
}

fn remove_stale_runtime_fixture(path: &Path) -> DxResult<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(DxError::IoError {
            path: Some(path.to_path_buf()),
            message: error.to_string(),
        }),
    }
}

fn route_output_file(route: &str) -> PathBuf {
    if route == "/" {
        return PathBuf::from("index.html");
    }

    let safe_route = route.trim_matches('/');
    let mut path = PathBuf::new();
    for segment in safe_route.split('/') {
        if !segment.is_empty() {
            path.push(segment);
        }
    }
    path.set_extension("html");
    path
}

fn route_output_packet_file(route: &str, out_dir: &Path) -> PathBuf {
    let mut path = out_dir.join(route_output_file(route));
    path.set_extension("dxp");
    path
}

fn route_output_runtime_file(route: &str, out_dir: &Path) -> PathBuf {
    let mut path = out_dir.join(route_output_file(route));
    path.set_extension("dxp.js");
    path
}

fn route_output_claims_manifest_file(route: &str, out_dir: &Path) -> PathBuf {
    let mut path = out_dir.join(route_output_file(route));
    path.set_extension("claims.json");
    path
}

fn route_output_evidence_manifest_file(route: &str, out_dir: &Path) -> PathBuf {
    let mut path = out_dir.join(route_output_file(route));
    path.set_extension("evidence.json");
    path
}

fn route_file_name(path: &Path) -> DxResult<String> {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .ok_or_else(|| DxError::InternalError {
            message: format!(
                "Could not derive proof asset file name for {}",
                path.display()
            ),
        })
}

fn print_vertical_summary_terminal(summary: &DxVerticalCliSummary) {
    println!("DX-WWW vertical proof");
    println!("Route: {}", summary.route);
    println!("Page: {}", summary.page);
    println!("Components: {}", summary.components.len());
    println!("Fallback bytes: {}", summary.fallback.optimized_bytes);
    println!("Delivery mode: {}", summary.fallback.delivery_mode);
    println!("Packet: {}", summary.packet.format);
    println!("Packet bytes: {}", summary.packet.bytes);
    println!("Decoded templates: {}", summary.packet.template_count);
    println!("Decoded strings: {}", summary.packet.string_count);
    println!("Packet roundtrip: {}", summary.packet.roundtrip_matches);
    println!("Browser packet: {}", summary.browser_packet.format);
    println!("Browser packet bytes: {}", summary.browser_packet.bytes);
    println!(
        "Browser packet sections: {}",
        summary.browser_packet.section_count
    );
    println!(
        "Browser packet roundtrip: {}",
        summary.browser_packet.roundtrip_matches
    );

    if let Some(interaction) = &summary.interaction {
        println!("Interaction: {}", interaction.delivery_mode);
        println!("State: {}", interaction.state_name);
        println!("Actions: {}", interaction.action_count);
        println!("Runtime bytes: {}", interaction.script_bytes);
    }

    if !summary.missing_components.is_empty() {
        println!();
        println!("Missing components:");
        for component in &summary.missing_components {
            println!("  - {component}");
        }
    }

    if let Some(written) = &summary.written {
        println!();
        println!("Written:");
        println!("  HTML: {}", written.html_path.display());
        println!("  Packet: {}", written.packet_path.display());
        if let Some(runtime_path) = &written.runtime_path {
            println!("  Runtime fixture: {}", runtime_path.display());
        } else {
            println!("  Runtime fixture: static route delivery");
        }
        if let Some(path) = &written.claims_manifest_path {
            println!("  Claims manifest: {}", path.display());
        }
        if let Some(path) = &written.evidence_manifest_path {
            println!("  Evidence manifest: {}", path.display());
        }
        println!("  Summary: {}", written.summary_path.display());
    } else {
        println!();
        println!("Dry run: no files written");
    }

    if !summary.forge_packages.is_empty() {
        println!();
        println!("Forge packages:");
        for forge in &summary.forge_packages {
            println!("  - {}", forge.package_id);
            println!("    Action: {}", forge.action);
            println!("    Score: {}", forge.risk_score);
            println!("    Traffic: {}", forge.traffic);
            println!("    Files tracked: {}", forge.files_tracked);
            if let Some(path) = &forge.manifest_path {
                println!("    Manifest: {}", path.display());
            }
            if let Some(path) = &forge.receipt_path {
                println!("    Receipt: {}", path.display());
            }
        }
    }

    if let Some(forge) = &summary.forge {
        println!();
        println!("Forge:");
        println!("  Package: {}", forge.package_id);
        println!("  Action: {}", forge.action);
        println!("  Score: {}", forge.risk_score);
        println!("  Traffic: {}", forge.traffic);
        println!("  Files tracked: {}", forge.files_tracked);
        if let Some(path) = &forge.manifest_path {
            println!("  Manifest: {}", path.display());
        }
        if let Some(path) = &forge.receipt_path {
            println!("  Receipt: {}", path.display());
        }
    }
}

fn vertical_summary_markdown(summary: &DxVerticalCliSummary) -> String {
    let mut markdown = String::new();
    markdown.push_str("# DX-WWW vertical proof\n\n");
    markdown.push_str(&format!("- Route: `{}`\n", summary.route));
    markdown.push_str(&format!("- Page: `{}`\n", summary.page));
    markdown.push_str(&format!("- Components: `{}`\n", summary.components.len()));
    markdown.push_str(&format!(
        "- Fallback bytes: `{}`\n",
        summary.fallback.optimized_bytes
    ));
    markdown.push_str(&format!(
        "- Delivery mode: `{}`\n",
        summary.fallback.delivery_mode
    ));
    markdown.push_str(&format!("- Packet: `{}`\n", summary.packet.format));
    markdown.push_str(&format!("- Packet bytes: `{}`\n", summary.packet.bytes));
    markdown.push_str(&format!(
        "- Decoded templates: `{}`\n",
        summary.packet.template_count
    ));
    markdown.push_str(&format!(
        "- Decoded strings: `{}`\n",
        summary.packet.string_count
    ));
    markdown.push_str(&format!(
        "- Packet roundtrip: `{}`\n",
        summary.packet.roundtrip_matches
    ));
    markdown.push_str(&format!(
        "- Browser packet: `{}`\n",
        summary.browser_packet.format
    ));
    markdown.push_str(&format!(
        "- Browser packet bytes: `{}`\n",
        summary.browser_packet.bytes
    ));
    markdown.push_str(&format!(
        "- Browser packet sections: `{}`\n",
        summary.browser_packet.section_count
    ));
    markdown.push_str(&format!(
        "- Browser packet roundtrip: `{}`\n",
        summary.browser_packet.roundtrip_matches
    ));

    if let Some(interaction) = &summary.interaction {
        markdown.push_str(&format!("- Interaction: `{}`\n", interaction.delivery_mode));
        markdown.push_str(&format!("- State: `{}`\n", interaction.state_name));
        markdown.push_str(&format!("- Actions: `{}`\n", interaction.action_count));
        markdown.push_str(&format!(
            "- Runtime bytes: `{}`\n",
            interaction.script_bytes
        ));
    }

    if !summary.missing_components.is_empty() {
        markdown.push_str("\n## Missing Components\n\n");
        for component in &summary.missing_components {
            markdown.push_str(&format!("- `{component}`\n"));
        }
    }

    if let Some(written) = &summary.written {
        markdown.push_str("\n## Written\n\n");
        markdown.push_str(&format!("- HTML: `{}`\n", written.html_path.display()));
        markdown.push_str(&format!("- Packet: `{}`\n", written.packet_path.display()));
        if let Some(runtime_path) = &written.runtime_path {
            markdown.push_str(&format!(
                "- Runtime fixture: `{}`\n",
                runtime_path.display()
            ));
        } else {
            markdown.push_str("- Runtime fixture: `static route delivery`\n");
        }
        if let Some(path) = &written.claims_manifest_path {
            markdown.push_str(&format!("- Claims manifest: `{}`\n", path.display()));
        }
        if let Some(path) = &written.evidence_manifest_path {
            markdown.push_str(&format!("- Evidence manifest: `{}`\n", path.display()));
        }
        markdown.push_str(&format!(
            "- Summary: `{}`\n",
            written.summary_path.display()
        ));
    }

    if !summary.forge_packages.is_empty() {
        markdown.push_str("\n## Forge Packages\n\n");
        for forge in &summary.forge_packages {
            markdown.push_str(&format!("- Package: `{}`\n", forge.package_id));
            markdown.push_str(&format!("  - Action: `{}`\n", forge.action));
            markdown.push_str(&format!("  - Score: `{}`\n", forge.risk_score));
            markdown.push_str(&format!("  - Traffic: `{}`\n", forge.traffic));
            markdown.push_str(&format!("  - Files tracked: `{}`\n", forge.files_tracked));
            if let Some(path) = &forge.manifest_path {
                markdown.push_str(&format!("  - Manifest: `{}`\n", path.display()));
            }
            if let Some(path) = &forge.receipt_path {
                markdown.push_str(&format!("  - Receipt: `{}`\n", path.display()));
            }
        }
    }

    if let Some(forge) = &summary.forge {
        markdown.push_str("\n## Forge\n\n");
        markdown.push_str(&format!("- Package: `{}`\n", forge.package_id));
        markdown.push_str(&format!("- Action: `{}`\n", forge.action));
        markdown.push_str(&format!("- Score: `{}`\n", forge.risk_score));
        markdown.push_str(&format!("- Traffic: `{}`\n", forge.traffic));
        markdown.push_str(&format!("- Files tracked: `{}`\n", forge.files_tracked));
        if let Some(path) = &forge.manifest_path {
            markdown.push_str(&format!("- Manifest: `{}`\n", path.display()));
        }
        if let Some(path) = &forge.receipt_path {
            markdown.push_str(&format!("- Receipt: `{}`\n", path.display()));
        }
    }

    markdown
}

fn receipt_traffic(outcome: &DxForgeAddOutcome) -> DxUpdateTraffic {
    if outcome
        .receipt
        .policy_decisions
        .iter()
        .any(|decision| decision.traffic == DxUpdateTraffic::Red)
    {
        return DxUpdateTraffic::Red;
    }

    if outcome
        .receipt
        .policy_decisions
        .iter()
        .any(|decision| decision.traffic == DxUpdateTraffic::Yellow)
    {
        return DxUpdateTraffic::Yellow;
    }

    DxUpdateTraffic::Green
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn prove_vertical_dry_run_writes_nothing() {
        let dir = tempdir().expect("tempdir");
        write_vertical_fixture(dir.path());
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--page".to_string(),
            "pages/index.html".to_string(),
            "--component".to_string(),
            "components/Button.tsx".to_string(),
            "--dry-run".to_string(),
        ])
        .expect("prove vertical dry run");

        assert!(!dir.path().join(".dx/vertical/index.html").exists());
        assert!(!dir.path().join(".dx/forge/source-manifest.json").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_write_emits_html_and_summary() {
        let dir = tempdir().expect("tempdir");
        write_vertical_fixture(dir.path());
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--page".to_string(),
            "pages/index.html".to_string(),
            "--component".to_string(),
            "components/Button.tsx".to_string(),
            "--out".to_string(),
            "proof".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove vertical write");

        let html = fs::read_to_string(dir.path().join("proof/index.html")).expect("html");
        let summary = fs::read_to_string(dir.path().join("proof/proof.json")).expect("summary");
        let packet = fs::read(dir.path().join("proof/index.dxp")).expect("packet");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");
        assert!(html.contains("DX-WWW CLI vertical proof"));
        assert!(html.contains("<button"));
        assert!(!html.contains(r#"href="index.dxp""#));
        assert!(!html.contains(r#"src="index.dxp.js" data-dx-packet="index.dxp""#));
        assert!(packet.starts_with(b"DXPK"));
        assert!(!dir.path().join("proof/index.dxp.js").exists());
        assert!(summary.contains("\"delivery_mode\""));
        assert!(summary.contains("\"static\""));
        assert!(summary.contains("\"browser_packet\""));
        assert!(summary.contains("\"dxp-v1\""));
        assert!(summary.contains("\"packet_path\""));
        assert!(!summary.contains("\"runtime_path\""));
        assert!(summary.contains("\"forge\""));
        assert!(manifest.contains("dx-www/vertical/index"));
        assert!(manifest.contains("components/Button.tsx"));
        assert!(
            fs::read_dir(dir.path().join(".dx/forge/receipts"))
                .expect("receipts")
                .next()
                .is_some()
        );
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_uses_forge_materialized_button_package() {
        let dir = tempdir().expect("tempdir");
        write_forge_button_page_fixture(dir.path());
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--page".to_string(),
            "pages/index.html".to_string(),
            "--package".to_string(),
            "ui/button".to_string(),
            "--out".to_string(),
            "proof".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove vertical with forge package");

        let html = fs::read_to_string(dir.path().join("proof/index.html")).expect("html");
        let summary = fs::read_to_string(dir.path().join("proof/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");
        assert!(html.contains("Launch with Forge"));
        assert!(html.contains("<button"));
        assert!(!html.contains("data-component=\"Button\""));
        assert!(summary.contains("\"forge_packages\""));
        assert!(summary.contains("shadcn/ui/button"));
        assert!(manifest.contains("shadcn/ui/button"));
        assert!(manifest.contains("dx-www/vertical/index"));
        assert!(dir.path().join("components/ui/button.tsx").exists());
        assert!(dir.path().join("components/ui/slot.tsx").exists());
        assert!(dir.path().join("lib/utils.ts").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_uses_forge_materialized_selected_icon_package() {
        let dir = tempdir().expect("tempdir");
        write_forge_icon_page_fixture(dir.path());
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--page".to_string(),
            "pages/index.html".to_string(),
            "--package".to_string(),
            "icon/search".to_string(),
            "--out".to_string(),
            "proof".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove vertical with selected icon package");

        let html = fs::read_to_string(dir.path().join("proof/index.html")).expect("html");
        let summary = fs::read_to_string(dir.path().join("proof/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");
        assert!(html.contains("Search the registry"));
        assert!(html.contains("<svg"));
        assert!(!html.contains("data-component=\"SearchIcon\""));
        assert!(summary.contains("dx/icon/search"));
        assert!(manifest.contains("dx/icon/search"));
        assert!(dir.path().join("components/icons/search.tsx").exists());
        assert!(dir.path().join("lib/icons.ts").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_site_fixture_writes_public_route() {
        let dir = tempdir().expect("tempdir");
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-site".to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove forge site fixture");

        let source = fs::read_to_string(dir.path().join("pages/forge.html")).expect("source");
        let html = fs::read_to_string(dir.path().join("public/forge.html")).expect("html");
        let claims =
            fs::read_to_string(dir.path().join("public/forge.claims.json")).expect("claims");
        let evidence =
            fs::read_to_string(dir.path().join("public/forge.evidence.json")).expect("evidence");
        let summary = fs::read_to_string(dir.path().join("public/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");
        assert!(source.contains("Not universal npm yet"));
        assert!(source.contains("release-proof and package-scorecard models"));
        assert!(source.contains("Package scorecard"));
        assert!(source.contains("Benchmark evidence"));
        assert!(source.contains("Evidence model"));
        assert!(source.contains("forge.evidence.json"));
        assert!(!source.contains("Package detail matrix"));
        assert!(!source.contains("Advisory coverage"));
        assert!(!source.contains("License review"));
        assert!(html.contains("DX Forge launch evidence"));
        assert!(html.contains("Package scorecard"));
        assert!(html.contains("Benchmark evidence"));
        assert!(html.contains("Evidence model"));
        assert!(html.contains("forge.evidence.json"));
        assert!(!html.contains("Package detail matrix"));
        assert!(!html.contains("Advisory coverage"));
        assert!(!html.contains("License review"));
        assert!(html.contains("Not universal npm yet"));
        assert!(html.contains("<button"));
        assert!(html.contains("<svg"));
        assert!(!html.contains(r#"href="forge.dxp""#));
        assert!(!html.contains(r#"src="forge.dxp.js""#));
        assert!(claims.contains("\"route\": \"/forge\""));
        assert!(claims.contains("\"source_field\": \"package_scorecard.packages[].source_owned\""));
        assert!(claims.contains("\"verification_status\": \"verified\""));
        assert!(claims.contains("auth/better-auth"));
        assert!(evidence.contains("\"route\": \"/forge\""));
        assert!(evidence.contains("\"packages\""));
        assert!(evidence.contains("\"provenance_source\""));
        assert!(evidence.contains("\"license_reviewed\""));
        assert!(evidence.contains("auth/better-auth"));
        assert!(summary.contains("\"route\": \"/forge\""));
        assert!(summary.contains("forge.claims.json"));
        assert!(summary.contains("forge.evidence.json"));
        assert!(summary.contains("shadcn/ui/button"));
        assert!(summary.contains("dx/icon/search"));
        assert!(summary.contains("dx-www/vertical/forge"));
        assert!(manifest.contains("pages/forge.html"));
        assert!(manifest.contains("shadcn/ui/button"));
        assert!(manifest.contains("dx/icon/search"));
        assert!(dir.path().join("components/ui/button.tsx").exists());
        assert!(dir.path().join("components/icons/search.tsx").exists());
        assert!(dir.path().join("public/forge.dxp").exists());
        assert!(!dir.path().join("public/forge.dxp.js").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_site_snapshots_match_launch_copy() {
        let dir = tempdir().expect("tempdir");
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-site".to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove forge site fixture");

        let source = fs::read_to_string(dir.path().join("pages/forge.html")).expect("source");
        let html = fs::read_to_string(dir.path().join("public/forge.html")).expect("html");

        assert_forge_page_snapshot("forge-site-source.html", &source, dir.path());
        assert_forge_page_snapshot("forge-site.html", &html, dir.path());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_scorecard_fixture_writes_public_route() {
        let dir = tempdir().expect("tempdir");
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-scorecard".to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove forge scorecard fixture");

        let source =
            fs::read_to_string(dir.path().join("pages/forge/scorecard.html")).expect("source");
        let html =
            fs::read_to_string(dir.path().join("public/forge/scorecard.html")).expect("html");
        let summary = fs::read_to_string(dir.path().join("public/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");
        assert!(source.contains("DX Forge Package Scorecard"));
        assert!(source.contains("shadcn/ui/button"));
        assert!(source.contains("dx/icon/search"));
        assert!(source.contains("auth/better-auth"));
        assert!(html.contains("DX Forge Package Scorecard"));
        assert!(html.contains("not a universal npm replacement"));
        assert!(html.contains("shadcn/ui/button"));
        assert!(html.contains("dx/icon/search"));
        assert!(html.contains("auth/better-auth"));
        assert!(summary.contains("\"route\": \"/forge/scorecard\""));
        assert!(summary.contains("dx-www/vertical/forge-scorecard"));
        assert!(manifest.contains("pages/forge/scorecard.html"));
        assert!(dir.path().join("public/forge/scorecard.dxp").exists());
        assert!(!dir.path().join("public/forge/scorecard.dxp.js").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_ci_fixture_writes_public_route() {
        let dir = tempdir().expect("tempdir");
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-ci".to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove forge ci fixture");

        let source = fs::read_to_string(dir.path().join("pages/forge/ci.html")).expect("source");
        let html = fs::read_to_string(dir.path().join("public/forge/ci.html")).expect("html");
        let claims =
            fs::read_to_string(dir.path().join("public/forge/ci.claims.json")).expect("claims");
        let summary = fs::read_to_string(dir.path().join("public/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");

        assert!(source.contains("DX Forge CI evidence"));
        assert!(source.contains("DxForgeSmokeReport"));
        assert!(source.contains("forge-triage.md"));
        assert!(source.contains("No node_modules"));
        assert!(html.contains("DX Forge CI evidence"));
        assert!(html.contains("forge-readiness-badge.json"));
        assert!(html.contains("forge-triage.md"));
        assert!(html.contains("No node_modules"));
        assert!(claims.contains("\"route\": \"/forge/ci\""));
        assert!(claims.contains("\"source_model\": \"DxForgeSmokeReport\""));
        assert!(claims.contains("\"source_model\": \"DxForgeReadinessBadge\""));
        assert!(summary.contains("\"route\": \"/forge/ci\""));
        assert!(summary.contains("dx-www/vertical/forge-ci"));
        assert!(manifest.contains("pages/forge/ci.html"));
        assert!(dir.path().join("public/forge/ci.dxp").exists());
        assert!(!dir.path().join("public/forge/ci.dxp.js").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_evidence_fixture_writes_public_route() {
        let dir = tempdir().expect("tempdir");
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-evidence".to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove forge evidence fixture");

        let source =
            fs::read_to_string(dir.path().join("pages/forge/evidence.html")).expect("source");
        let html = fs::read_to_string(dir.path().join("public/forge/evidence.html")).expect("html");
        let claims = fs::read_to_string(dir.path().join("public/forge/evidence.claims.json"))
            .expect("claims");
        let summary = fs::read_to_string(dir.path().join("public/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");

        assert!(source.contains("DX Forge Public Evidence"));
        assert!(source.contains("forge-readiness-badge.json"));
        assert!(source.contains("forge-public-route-comparison.md"));
        assert!(source.contains("forge.claims.json"));
        assert!(source.contains("forge.evidence.json"));
        assert!(source.contains("forge/scorecard.html"));
        assert!(source.contains("forge/ci.html"));
        assert!(html.contains("DX Forge Public Evidence"));
        assert!(html.contains("forge-readiness-badge.json"));
        assert!(html.contains("forge-public-route-comparison.md"));
        assert!(html.contains("forge/ci.html"));
        assert!(claims.contains("\"route\": \"/forge/evidence\""));
        assert!(claims.contains("\"public-evidence-index\""));
        assert!(summary.contains("\"route\": \"/forge/evidence\""));
        assert!(summary.contains("dx-www/vertical/forge-evidence"));
        assert!(manifest.contains("pages/forge/evidence.html"));
        assert!(dir.path().join("public/forge/evidence.dxp").exists());
        assert!(!dir.path().join("public/forge/evidence.dxp.js").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_releases_fixture_writes_public_route() {
        let dir = tempdir().expect("tempdir");
        write_public_release_history_fixture(dir.path());
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-releases".to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove forge releases fixture");

        let source =
            fs::read_to_string(dir.path().join("pages/forge/releases.html")).expect("source");
        let html = fs::read_to_string(dir.path().join("public/forge/releases.html")).expect("html");
        let claims = fs::read_to_string(dir.path().join("public/forge/releases.claims.json"))
            .expect("claims");
        let summary = fs::read_to_string(dir.path().join("public/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");

        assert!(source.contains("DX Forge Release History"));
        assert!(source.contains("93 / 100"));
        assert!(source.contains("/forge/evidence"));
        assert!(source.contains("No release regressions detected"));
        assert!(html.contains("DX Forge Release History"));
        assert!(html.contains("forge-public-release-history.json"));
        assert!(html.contains("/forge/releases"));
        assert!(html.contains("/forge/ci"));
        assert!(claims.contains("\"route\": \"/forge/releases\""));
        assert!(claims.contains("\"source_model\": \"DxForgePublicReleaseHistory\""));
        assert!(claims.contains("\"latest-release-regressions\""));
        assert!(summary.contains("\"route\": \"/forge/releases\""));
        assert!(summary.contains("dx-www/vertical/forge-releases"));
        assert!(manifest.contains("pages/forge/releases.html"));
        assert!(dir.path().join("public/forge/releases.dxp").exists());
        assert!(!dir.path().join("public/forge/releases.dxp.js").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_changelog_fixture_writes_public_route() {
        let dir = tempdir().expect("tempdir");
        write_public_release_history_fixture(dir.path());
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-changelog".to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove forge changelog fixture");

        let source =
            fs::read_to_string(dir.path().join("pages/forge/changelog.html")).expect("source");
        let html =
            fs::read_to_string(dir.path().join("public/forge/changelog.html")).expect("html");
        let claims = fs::read_to_string(dir.path().join("public/forge/changelog.claims.json"))
            .expect("claims");
        let summary = fs::read_to_string(dir.path().join("public/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");

        assert!(source.contains("DX Forge Public Launch Changelog"));
        assert!(source.contains("93 / 100"));
        assert!(source.contains("Added public routes"));
        assert!(source.contains("does not claim live production traffic"));
        assert!(html.contains("DX Forge Public Launch Changelog"));
        assert!(html.contains("forge-public-launch-changelog.json"));
        assert!(html.contains("/forge/changelog"));
        assert!(claims.contains("\"route\": \"/forge/changelog\""));
        assert!(claims.contains("\"source_model\": \"DxForgeLaunchChangelogReport\""));
        assert!(claims.contains("\"launch-changelog-honest-scope\""));
        assert!(summary.contains("\"route\": \"/forge/changelog\""));
        assert!(summary.contains("dx-www/vertical/forge-changelog"));
        assert!(manifest.contains("pages/forge/changelog.html"));
        assert!(dir.path().join("public/forge/changelog.dxp").exists());
        assert!(!dir.path().join("public/forge/changelog.dxp.js").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_adoption_fixture_writes_public_route_from_report() {
        let dir = tempdir().expect("tempdir");
        let cli = Cli::with_cwd(dir.path().to_path_buf());
        cli.cmd_forge(&[
            "adoption-smoke".to_string(),
            "--project".to_string(),
            dir.path().display().to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            dir.path()
                .join(".dx/forge/adoption-smoke/forge-smoke.json")
                .display()
                .to_string(),
            "--fail-under".to_string(),
            "90".to_string(),
            "--quiet".to_string(),
        ])
        .expect("adoption smoke");

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-adoption".to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove forge adoption fixture");

        let source =
            fs::read_to_string(dir.path().join("pages/forge/adoption.html")).expect("source");
        let html = fs::read_to_string(dir.path().join("public/forge/adoption.html")).expect("html");
        let claims = fs::read_to_string(dir.path().join("public/forge/adoption.claims.json"))
            .expect("claims");
        let summary = fs::read_to_string(dir.path().join("public/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");

        assert!(source.contains("DX Forge Adoption Report"));
        assert!(source.contains("pages/forge-adoption.html"));
        assert!(source.contains("shadcn/ui/button"));
        assert!(source.contains("No node_modules"));
        assert!(html.contains("DX Forge Adoption Report"));
        assert!(html.contains("source-owned packages"));
        assert!(html.contains("release bundle"));
        assert!(claims.contains("\"route\": \"/forge/adoption\""));
        assert!(claims.contains("\"source_model\": \"DxForgeAdoptionReport\""));
        assert!(claims.contains("\"adoption-no-node-modules\""));
        assert!(summary.contains("\"route\": \"/forge/adoption\""));
        assert!(summary.contains("dx-www/vertical/forge-adoption"));
        assert!(manifest.contains("pages/forge/adoption.html"));
        assert!(dir.path().join("public/forge/adoption.dxp").exists());
        assert!(!dir.path().join("public/forge/adoption.dxp.js").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_quickstart_fixture_writes_public_beta_route() {
        let dir = tempdir().expect("tempdir");
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-quickstart".to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove forge quickstart fixture");

        let source =
            fs::read_to_string(dir.path().join("pages/forge/quickstart.html")).expect("source");
        let html =
            fs::read_to_string(dir.path().join("public/forge/quickstart.html")).expect("html");
        let claims = fs::read_to_string(dir.path().join("public/forge/quickstart.claims.json"))
            .expect("claims");
        let summary = fs::read_to_string(dir.path().join("public/proof.json")).expect("summary");
        let manifest = fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json"))
            .expect("manifest");

        assert!(source.contains("DX Forge Public Beta Quickstart"));
        assert!(source.contains("dx forge init-app --write"));
        assert!(source.contains("dx forge ci"));
        assert!(source.contains("measure-forge-source-owned-package-review.ts"));
        assert!(source.contains("no node_modules"));
        assert!(source.contains("not a universal npm replacement"));
        assert!(html.contains("DX Forge Public Beta Quickstart"));
        assert!(html.contains("dx forge init-app --write"));
        assert!(html.contains("forge-public-beta-quickstart.md"));
        assert!(html.contains("No node_modules"));
        assert!(claims.contains("\"route\": \"/forge/quickstart\""));
        assert!(claims.contains("\"source_model\": \"DxForgePublicBetaQuickstart\""));
        assert!(claims.contains("\"quickstart-starts-with-init-app\""));
        assert!(claims.contains("\"quickstart-honest-scope\""));
        assert!(summary.contains("\"route\": \"/forge/quickstart\""));
        assert!(summary.contains("dx-www/vertical/forge-quickstart"));
        assert!(manifest.contains("pages/forge/quickstart.html"));
        assert!(dir.path().join("public/forge/quickstart.dxp").exists());
        assert!(!dir.path().join("public/forge/quickstart.dxp.js").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn prove_vertical_forge_site_fixture_refuses_to_overwrite_local_route() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        fs::write(
            dir.path().join("pages/forge.html"),
            "<page>local edits</page>",
        )
        .expect("local route");
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        let error = cli
            .cmd_prove(&[
                "vertical".to_string(),
                "--fixture".to_string(),
                "forge-site".to_string(),
                "--write".to_string(),
            ])
            .expect_err("fixture should not overwrite local edits");

        assert!(error.to_string().contains("Refusing to overwrite"));
    }

    #[test]
    fn prove_vertical_write_emits_counter_interaction_runtime() {
        let dir = tempdir().expect("tempdir");
        write_counter_fixture(dir.path());
        let cli = Cli::with_cwd(dir.path().to_path_buf());

        cli.cmd_prove(&[
            "vertical".to_string(),
            "--page".to_string(),
            "pages/counter.html".to_string(),
            "--out".to_string(),
            "proof".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("prove counter write");

        let html = fs::read_to_string(dir.path().join("proof/counter.html")).expect("html");
        let packet = fs::read(dir.path().join("proof/counter.dxp")).expect("packet");
        let runtime = fs::read_to_string(dir.path().join("proof/counter.dxp.js")).expect("runtime");
        let summary = fs::read_to_string(dir.path().join("proof/proof.json")).expect("summary");
        assert!(html.contains(r#"id="dx-state-count">0</span>"#));
        assert!(html.contains("dx-action-increment"));
        assert!(html.contains("<script>"));
        assert!(html.contains(r#"href="counter.dxp""#));
        assert!(html.contains(r#"src="counter.dxp.js" data-dx-packet="counter.dxp""#));
        assert!(packet.starts_with(b"DXPK"));
        assert!(runtime.contains("window.__DX_PACKET_APPLIED__"));
        let summary_json: serde_json::Value = serde_json::from_str(&summary).expect("summary json");
        assert_eq!(summary_json["interaction"]["delivery_mode"], "js");
        assert!(summary.contains("\"packet\""));
        assert!(summary.contains("\"browser_packet\""));
        assert!(summary.contains("\"dxob-v1\""));
        assert!(summary.contains("\"dxp-v1\""));
        assert!(summary.contains("\"roundtrip_matches\": true"));
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn vertical_package_id_is_stable_and_safe() {
        assert_eq!(vertical_package_id("/"), "dx-www/vertical/index");
        assert_eq!(
            vertical_package_id("/Docs/Launch Notes"),
            "dx-www/vertical/docs-launch-notes"
        );
        assert_eq!(
            vertical_package_id("/../outside"),
            "dx-www/vertical/outside"
        );
    }

    fn write_vertical_fixture(root: &Path) {
        fs::create_dir_all(root.join("pages")).expect("pages");
        fs::create_dir_all(root.join("components")).expect("components");
        fs::write(
            root.join("components/Button.tsx"),
            r#"
<component>
    <button class="inline-flex items-center rounded-md px-3 py-2 text-sm font-medium">
        Increase
    </button>
</component>
"#,
        )
        .expect("component");
        fs::write(
            root.join("pages/index.html"),
            r#"
<page>
    <main class="mx-auto max-w-2xl p-6">
        <h1 class="text-3xl font-semibold">DX-WWW CLI vertical proof</h1>
        <Button />
    </main>
</page>
"#,
        )
        .expect("page");
    }

    fn write_counter_fixture(root: &Path) {
        fs::create_dir_all(root.join("pages")).expect("pages");
        fs::write(
            root.join("pages/counter.html"),
            r#"
<script lang="rust">
let mut count = 0;

fn increment() {
    count += 1;
}
</script>

<page>
    <main>
        <p>Count: {count}</p>
        <button on:click={increment}>Increase</button>
    </main>
</page>
"#,
        )
        .expect("page");
    }

    fn write_forge_button_page_fixture(root: &Path) {
        fs::create_dir_all(root.join("pages")).expect("pages");
        fs::write(
            root.join("pages/index.html"),
            r#"
<page>
    <main class="mx-auto max-w-2xl p-6">
        <h1>DX Forge package proof</h1>
        <Button>Launch with Forge</Button>
    </main>
</page>
"#,
        )
        .expect("page");
    }

    fn write_forge_icon_page_fixture(root: &Path) {
        fs::create_dir_all(root.join("pages")).expect("pages");
        fs::write(
            root.join("pages/index.html"),
            r#"
<page>
    <main class="mx-auto max-w-2xl p-6">
        <h1>DX Forge selected icon proof</h1>
        <button class="inline-flex items-center gap-2 rounded-md border px-3 py-2">
            <SearchIcon />
            <span>Search the registry</span>
        </button>
    </main>
</page>
"#,
        )
        .expect("page");
    }

    fn write_public_release_history_fixture(root: &Path) {
        let report_dir = root.join("benchmarks/reports");
        fs::create_dir_all(&report_dir).expect("release history dir");
        fs::write(
            report_dir.join("forge-public-release-history.json"),
            r#"{
  "updated_at": "2026-05-16T18:28:57.583123600+00:00",
  "records": [
    {
      "generated_at": "2026-05-16T18:28:57.583123600+00:00",
      "source_dashboard": ".dx/ci/forge-release-dashboard.json",
      "source_route_comparison": "benchmarks/reports/forge-public-route-comparison.json",
      "dashboard": {
        "generated_at": "2026-05-16T18:12:44.075541500+00:00",
        "score": 93,
        "fail_under": 90,
        "passed": true,
        "no_node_modules": true,
        "public_evidence_links": 9,
        "findings": [],
        "checks": {
          "route_comparison": {
            "passed": true,
            "score": 100,
            "message": "4 public routes measured, 4248 Brotli bytes total."
          }
        }
      },
      "route_comparison": {
        "generated_at": "2026-05-16T18:12:44.075541500+00:00",
        "route_count": 4,
        "total_decoded_bytes": 19629,
        "total_brotli_bytes": 4248,
        "lowest_brotli_route": "/forge/ci",
        "routes": [
          {
            "route": "/forge",
            "fixture_mode": "forge-site",
            "delivery": "static",
            "decoded_bytes": 5141,
            "brotli_bytes": 1240,
            "http_route_median_ms": 1.565,
            "chrome_load_event_ms": 9.5,
            "budget_passed": true
          },
          {
            "route": "/forge/scorecard",
            "fixture_mode": "forge-scorecard",
            "delivery": "static",
            "decoded_bytes": 4189,
            "brotli_bytes": 1040,
            "http_route_median_ms": 1.523,
            "chrome_load_event_ms": 8.8,
            "budget_passed": true
          },
          {
            "route": "/forge/ci",
            "fixture_mode": "forge-ci",
            "delivery": "static",
            "decoded_bytes": 3309,
            "brotli_bytes": 862,
            "http_route_median_ms": 1.767,
            "chrome_load_event_ms": 8.9,
            "budget_passed": true
          },
          {
            "route": "/forge/evidence",
            "fixture_mode": "forge-evidence",
            "delivery": "static",
            "decoded_bytes": 6988,
            "brotli_bytes": 1106,
            "http_route_median_ms": 0.939,
            "chrome_load_event_ms": 8.3,
            "budget_passed": true
          }
        ]
      },
      "regression_findings": []
    }
  ]
}
"#,
        )
        .expect("release history fixture");
    }

    fn assert_forge_page_snapshot(name: &str, actual: &str, project: &Path) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/forge-pages")
            .join(name);
        let expected = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read Forge page snapshot `{name}`: {error}"));
        let actual = normalize_forge_page_snapshot(actual, project);
        assert_eq!(
            normalize_snapshot_line_endings(&expected),
            actual,
            "Forge page snapshot `{name}` drifted"
        );
    }

    fn normalize_forge_page_snapshot(actual: &str, project: &Path) -> String {
        let mut normalized = normalize_snapshot_line_endings(actual);
        normalized = normalized.replace(&project.display().to_string(), "<project>");
        normalized = normalized.replace('\\', "/");
        normalize_iso_timestamps(&normalized)
    }

    fn normalize_iso_timestamps(value: &str) -> String {
        let mut normalized = String::with_capacity(value.len());
        let bytes = value.as_bytes();
        let mut index = 0usize;
        while index < bytes.len() {
            if let Some(end) = iso_timestamp_end(bytes, index) {
                normalized.push_str("<timestamp>");
                index = end;
            } else {
                normalized.push(bytes[index] as char);
                index += 1;
            }
        }
        normalized
    }

    fn iso_timestamp_end(bytes: &[u8], index: usize) -> Option<usize> {
        if !(index + 19 <= bytes.len()
            && bytes[index..index + 4]
                .iter()
                .all(|byte| byte.is_ascii_digit())
            && bytes[index + 4] == b'-'
            && bytes[index + 5..index + 7]
                .iter()
                .all(|byte| byte.is_ascii_digit())
            && bytes[index + 7] == b'-'
            && bytes[index + 8..index + 10]
                .iter()
                .all(|byte| byte.is_ascii_digit())
            && bytes[index + 10] == b'T'
            && bytes[index + 11..index + 13]
                .iter()
                .all(|byte| byte.is_ascii_digit())
            && bytes[index + 13] == b':'
            && bytes[index + 14..index + 16]
                .iter()
                .all(|byte| byte.is_ascii_digit())
            && bytes[index + 16] == b':'
            && bytes[index + 17..index + 19]
                .iter()
                .all(|byte| byte.is_ascii_digit()))
        {
            return None;
        }

        let mut end = index + 19;
        if bytes.get(end) == Some(&b'.') {
            end += 1;
            let fraction_start = end;
            while bytes.get(end).is_some_and(|byte| byte.is_ascii_digit()) {
                end += 1;
            }
            if end == fraction_start {
                return None;
            }
        }

        match bytes.get(end).copied() {
            Some(b'Z') => Some(end + 1),
            Some(b'+') | Some(b'-')
                if end + 6 <= bytes.len()
                    && bytes[end + 1..end + 3]
                        .iter()
                        .all(|byte| byte.is_ascii_digit())
                    && bytes[end + 3] == b':'
                    && bytes[end + 4..end + 6]
                        .iter()
                        .all(|byte| byte.is_ascii_digit()) =>
            {
                Some(end + 6)
            }
            _ => None,
        }
    }

    fn normalize_snapshot_line_endings(value: &str) -> String {
        let mut normalized = value
            .replace("\r\n", "\n")
            .lines()
            .map(str::trim_end)
            .collect::<Vec<_>>()
            .join("\n");
        if !normalized.ends_with('\n') {
            normalized.push('\n');
        }
        normalized
    }
}
