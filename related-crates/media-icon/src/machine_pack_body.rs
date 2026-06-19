use crate::machine_catalog::{
    IconCatalogSourceFingerprint, icon_catalog_source_fingerprint, icon_machine_paths,
    project_root_for_index_output,
};
use crate::types::{IconMetadata, IconPack};
use anyhow::{Context, Result, anyhow, bail};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serializer::machine::{
    MachineCacheCodec, MachineCacheKind, MachineCacheSchema, MachineCacheSource,
    MachineCacheWriteOptions, access_typed_machine_cache, open_typed_machine_cache,
    write_typed_machine_cache,
};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const ICON_PACK_BODY_CACHE_SCHEMA: &str = "dx.icon.pack_body.v1";

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconPackBodyMachineV1 {
    pub selected_data_root: String,
    pub generated_at_unix_ms: u64,
    pub source_file_count: u32,
    pub source_total_bytes: u64,
    pub source_blake3: [u8; 32],
    pub pack_count: u32,
    pub icon_count: u32,
    pub packs: Vec<IconPackBodyPackV1>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconPackBodyPackV1 {
    pub pack: String,
    pub rel_path: String,
    pub source_bytes: u64,
    pub source_blake3: [u8; 32],
    pub icon_count: u32,
    pub icons: Vec<IconPackBodyEntryV1>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconPackBodyEntryV1 {
    pub name: String,
    pub body: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconPackBodyReadSummary {
    pub pack_count: usize,
    pub icon_count: usize,
    pub source_total_bytes: u64,
    pub mode: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconPackBodyMachineRuntimeRead {
    pub pack_body_machine: IconPackBodyMachineV1,
    pub mode: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedIconPackBody {
    pub body: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

pub fn write_icon_pack_body_machine_cache_for_index_output(
    data_dir: &Path,
    output_dir: &Path,
) -> Result<PathBuf> {
    let project_root = project_root_for_index_output(output_dir)?;
    write_icon_pack_body_machine_cache(&project_root, data_dir)
}

pub fn write_icon_pack_body_machine_cache(project_root: &Path, data_dir: &Path) -> Result<PathBuf> {
    let build = build_icon_pack_body_machine_with_source(data_dir)?;
    validate_icon_pack_body_machine(&build.machine)?;
    let paths = icon_pack_body_machine_paths(project_root, data_dir)?;
    let receipt = write_typed_machine_cache(
        &build.machine,
        &build.source,
        &paths,
        icon_pack_body_schema(),
        MachineCacheWriteOptions {
            codec: MachineCacheCodec::None,
        },
    )
    .with_context(|| {
        format!(
            "write icon pack body machine cache {}",
            paths.machine.display()
        )
    })?;

    Ok(receipt.machine)
}

pub fn read_icon_pack_body_machine_cache_summary(
    project_root: &Path,
    data_dir: &Path,
) -> Option<IconPackBodyReadSummary> {
    let source = icon_catalog_source_fingerprint(data_dir).ok()?;
    let paths = icon_pack_body_machine_paths(project_root, data_dir).ok()?;

    if let Ok(machine) = open_typed_machine_cache::<IconPackBodyMachineV1>(
        &paths,
        &source.source,
        icon_pack_body_schema(),
    ) {
        let pack_body_machine =
            deserialize_icon_pack_body_machine_archive(machine.archived()).ok()?;
        validate_icon_pack_body_machine(&pack_body_machine).ok()?;
        return Some(pack_body_summary(&pack_body_machine, "mmap"));
    }

    let bytes = fs::read(&paths.machine).ok()?;
    let machine = access_typed_machine_cache::<IconPackBodyMachineV1>(
        &bytes,
        &source.source,
        icon_pack_body_schema(),
    )
    .ok()?;
    let pack_body_machine = deserialize_icon_pack_body_machine_archive(machine).ok()?;
    validate_icon_pack_body_machine(&pack_body_machine).ok()?;
    Some(pack_body_summary(&pack_body_machine, "bytes"))
}

pub fn read_icon_pack_body_machine_cache_for_index_output(
    index_dir: &Path,
    data_dir: &Path,
) -> Result<IconPackBodyMachineRuntimeRead> {
    let project_root = project_root_for_index_output(index_dir)?;
    read_icon_pack_body_machine_cache(&project_root, data_dir)
}

pub fn read_icon_pack_body_machine_cache(
    project_root: &Path,
    data_dir: &Path,
) -> Result<IconPackBodyMachineRuntimeRead> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    read_icon_pack_body_machine_cache_with_source_fingerprint(project_root, data_dir, &source)
}

pub(crate) fn read_icon_pack_body_machine_cache_with_source_fingerprint(
    project_root: &Path,
    data_dir: &Path,
    source: &IconCatalogSourceFingerprint,
) -> Result<IconPackBodyMachineRuntimeRead> {
    let paths = icon_pack_body_machine_paths(project_root, data_dir)?;

    if let Ok(machine) = open_typed_machine_cache::<IconPackBodyMachineV1>(
        &paths,
        &source.source,
        icon_pack_body_schema(),
    ) {
        let pack_body_machine = deserialize_icon_pack_body_machine_archive(machine.archived())?;
        validate_icon_pack_body_machine(&pack_body_machine)?;
        return Ok(IconPackBodyMachineRuntimeRead {
            pack_body_machine,
            mode: "mmap",
        });
    }

    let bytes = fs::read(&paths.machine).with_context(|| {
        format!(
            "read icon pack body machine cache {}",
            paths.machine.display()
        )
    })?;
    let machine = access_typed_machine_cache::<IconPackBodyMachineV1>(
        &bytes,
        &source.source,
        icon_pack_body_schema(),
    )?;
    let pack_body_machine = deserialize_icon_pack_body_machine_archive(machine)?;
    validate_icon_pack_body_machine(&pack_body_machine)?;

    Ok(IconPackBodyMachineRuntimeRead {
        pack_body_machine,
        mode: "bytes",
    })
}

pub fn read_icon_pack_body_machine_cache_with_source_audit(
    project_root: &Path,
    data_dir: &Path,
) -> Result<IconPackBodyMachineRuntimeRead> {
    let read = read_icon_pack_body_machine_cache(project_root, data_dir)?;
    validate_icon_pack_body_source_files(&read.pack_body_machine, data_dir)?;
    Ok(read)
}

pub fn resolve_icon_pack_body(
    machine: &IconPackBodyMachineV1,
    pack: &str,
    name: &str,
) -> Option<ResolvedIconPackBody> {
    let pack = machine.packs.iter().find(|entry| entry.pack == pack)?;
    let icon = pack.icons.iter().find(|entry| entry.name == name)?;
    Some(ResolvedIconPackBody {
        body: icon.body.clone(),
        width: icon.width,
        height: icon.height,
    })
}

pub fn validate_icon_pack_body_machine(machine: &IconPackBodyMachineV1) -> Result<()> {
    if machine.pack_count as usize != machine.packs.len() {
        bail!(
            "pack body machine pack count {} does not match pack table count {}",
            machine.pack_count,
            machine.packs.len()
        );
    }
    if machine.source_file_count as usize != machine.packs.len() {
        bail!(
            "pack body machine source file count {} does not match pack count {}",
            machine.source_file_count,
            machine.packs.len()
        );
    }

    let mut seen_packs = HashSet::with_capacity(machine.packs.len());
    let mut seen_rel_paths = HashSet::with_capacity(machine.packs.len());
    let mut computed_icon_count = 0usize;
    let mut computed_source_bytes = 0u64;

    for pack in &machine.packs {
        if pack.pack.trim().is_empty() {
            bail!("pack body machine contains empty pack prefix");
        }
        if pack.rel_path.trim().is_empty()
            || pack.rel_path.contains('/')
            || pack.rel_path.contains('\\')
            || pack.rel_path.contains(':')
            || Path::new(&pack.rel_path).is_absolute()
            || Path::new(&pack.rel_path).components().count() != 1
            || pack.rel_path == "."
            || pack.rel_path == ".."
        {
            bail!(
                "pack body machine pack {} has non-local rel_path {}",
                pack.pack,
                pack.rel_path
            );
        }
        if !pack.rel_path.ends_with(".json") {
            bail!(
                "pack body machine pack {} rel_path {} is not a JSON file",
                pack.pack,
                pack.rel_path
            );
        }
        if !seen_packs.insert(pack.pack.as_str()) {
            bail!("pack body machine duplicate pack {}", pack.pack);
        }
        if !seen_rel_paths.insert(pack.rel_path.as_str()) {
            bail!("pack body machine duplicate rel_path {}", pack.rel_path);
        }
        if pack.icon_count as usize != pack.icons.len() {
            bail!(
                "pack body machine pack {} icon count {} does not match icon entries {}",
                pack.pack,
                pack.icon_count,
                pack.icons.len()
            );
        }

        let mut seen_icon_names = HashSet::with_capacity(pack.icons.len());
        for icon in &pack.icons {
            if icon.name.is_empty() {
                bail!(
                    "pack body machine pack {} contains empty icon name",
                    pack.pack
                );
            }
            if icon.body.is_empty() {
                bail!(
                    "pack body machine pack {} icon {} contains empty body",
                    pack.pack,
                    icon.name
                );
            }
            validate_dimension(icon.width, &pack.pack, &icon.name, "width")?;
            validate_dimension(icon.height, &pack.pack, &icon.name, "height")?;
            if !seen_icon_names.insert(icon.name.as_str()) {
                bail!(
                    "pack body machine pack {} duplicate icon {}",
                    pack.pack,
                    icon.name
                );
            }
        }

        computed_icon_count = computed_icon_count.saturating_add(pack.icons.len());
        computed_source_bytes = computed_source_bytes.saturating_add(pack.source_bytes);
    }

    if computed_icon_count != machine.icon_count as usize {
        bail!(
            "pack body machine icon count {} does not match computed icon count {}",
            machine.icon_count,
            computed_icon_count
        );
    }
    if computed_source_bytes != machine.source_total_bytes {
        bail!(
            "pack body machine source total bytes {} does not match computed source bytes {}",
            machine.source_total_bytes,
            computed_source_bytes
        );
    }

    Ok(())
}

pub fn validate_icon_pack_body_parser_metadata(
    machine: &IconPackBodyMachineV1,
    metadata: &[IconMetadata],
) -> Result<()> {
    validate_icon_pack_body_machine(machine)?;
    if metadata.len() != machine.icon_count as usize {
        bail!(
            "pack body parser metadata count {} does not match machine icon count {}",
            metadata.len(),
            machine.icon_count
        );
    }

    let mut expected_id = 0u32;
    for pack in &machine.packs {
        for icon in &pack.icons {
            let icon_metadata = metadata.get(expected_id as usize).ok_or_else(|| {
                anyhow!("pack body parser metadata missing expected id {expected_id}")
            })?;
            if icon_metadata.id != expected_id {
                bail!(
                    "pack body parser metadata id {} does not match expected id {}",
                    icon_metadata.id,
                    expected_id
                );
            }
            if icon_metadata.pack != pack.pack {
                bail!(
                    "pack body parser metadata id {} pack {} does not match machine pack {}",
                    expected_id,
                    icon_metadata.pack,
                    pack.pack
                );
            }
            if icon_metadata.name != icon.name {
                bail!(
                    "pack body parser metadata id {} name {} does not match machine icon {}",
                    expected_id,
                    icon_metadata.name,
                    icon.name
                );
            }
            expected_id = expected_id.saturating_add(1);
        }
    }

    Ok(())
}

pub fn validate_icon_pack_body_runtime_metadata(
    machine: &IconPackBodyMachineV1,
    metadata: &[IconMetadata],
) -> Result<()> {
    validate_icon_pack_body_parser_metadata(machine, metadata)?;
    Ok(())
}

fn validate_icon_pack_body_source_files(
    machine: &IconPackBodyMachineV1,
    data_dir: &Path,
) -> Result<()> {
    let mut source_hasher = blake3::Hasher::new();
    let mut source_file_count = 0u32;
    let mut source_total_bytes = 0u64;
    let mut latest_modified_unix_ms = None;

    for pack in &machine.packs {
        let path = data_dir.join(&pack.rel_path);
        let bytes =
            fs::read(&path).with_context(|| format!("read icon pack source {}", path.display()))?;
        let metadata = fs::metadata(&path)
            .with_context(|| format!("stat icon pack source {}", path.display()))?;
        let source_blake3 = *blake3::hash(&bytes).as_bytes();
        if pack.source_bytes != metadata.len() {
            bail!(
                "pack body machine pack {} source bytes {} do not match current bytes {}",
                pack.pack,
                pack.source_bytes,
                metadata.len()
            );
        }
        if pack.source_blake3 != source_blake3 {
            bail!(
                "pack body machine pack {} source hash does not match current JSON",
                pack.pack
            );
        }

        let json = std::str::from_utf8(&bytes)
            .with_context(|| format!("decode icon pack source {}", path.display()))?;
        let source_pack = serde_json::from_str::<IconPack>(json)
            .with_context(|| format!("parse icon pack source {}", path.display()))?;
        if source_pack.prefix != pack.pack {
            bail!(
                "pack body machine pack {} source prefix changed to {}",
                pack.pack,
                source_pack.prefix
            );
        }
        if source_pack.icons.len() != pack.icons.len() {
            bail!(
                "pack body machine pack {} icon entries {} do not match current JSON icons {}",
                pack.pack,
                pack.icons.len(),
                source_pack.icons.len()
            );
        }
        for icon in &pack.icons {
            let source_icon = source_pack.icons.get(&icon.name).ok_or_else(|| {
                anyhow!(
                    "pack body machine pack {} icon {} is missing from current JSON",
                    pack.pack,
                    icon.name
                )
            })?;
            if source_icon.body != icon.body {
                bail!(
                    "pack body machine pack {} icon {} body does not match current JSON",
                    pack.pack,
                    icon.name
                );
            }
            if source_icon.width != icon.width || source_icon.height != icon.height {
                bail!(
                    "pack body machine pack {} icon {} dimensions do not match current JSON",
                    pack.pack,
                    icon.name
                );
            }
        }

        let modified_unix_ms = file_modified_unix_ms(&metadata);
        source_file_count = source_file_count.saturating_add(1);
        source_total_bytes = source_total_bytes.saturating_add(metadata.len());
        latest_modified_unix_ms = max_optional(latest_modified_unix_ms, modified_unix_ms);
        source_hasher.update(pack.rel_path.as_bytes());
        source_hasher.update(&metadata.len().to_le_bytes());
        source_hasher.update(&modified_unix_ms.unwrap_or_default().to_le_bytes());
        source_hasher.update(&source_blake3);
    }

    if source_file_count != machine.source_file_count {
        bail!(
            "pack body machine source file count {} does not match current JSON file count {}",
            machine.source_file_count,
            source_file_count
        );
    }
    if source_total_bytes != machine.source_total_bytes {
        bail!(
            "pack body machine source bytes {} do not match current JSON bytes {}",
            machine.source_total_bytes,
            source_total_bytes
        );
    }
    let source_blake3 = *source_hasher.finalize().as_bytes();
    if machine.source_blake3 != source_blake3 {
        bail!("pack body machine source hash does not match current JSON directory");
    }

    Ok(())
}

fn validate_dimension(value: Option<f32>, pack: &str, icon: &str, dimension: &str) -> Result<()> {
    let Some(value) = value else {
        return Ok(());
    };
    if !value.is_finite() || value <= 0.0 {
        bail!("pack body machine pack {pack} icon {icon} has invalid {dimension} {value}");
    }
    Ok(())
}

struct IconPackBodyMachineBuild {
    source: MachineCacheSource,
    machine: IconPackBodyMachineV1,
}

fn build_icon_pack_body_machine_with_source(data_dir: &Path) -> Result<IconPackBodyMachineBuild> {
    let mut json_files = fs::read_dir(data_dir)
        .with_context(|| format!("read icon data directory {}", data_dir.display()))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("list icon data directory {}", data_dir.display()))?;
    json_files.sort_by_key(|entry| entry.file_name());

    let mut packs = Vec::new();
    let mut icon_count = 0u32;
    let mut file_count = 0u32;
    let mut total_bytes = 0u64;
    let mut latest_modified_unix_ms = None;
    let mut source_hasher = blake3::Hasher::new();

    for entry in json_files {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }

        let bytes =
            fs::read(&path).with_context(|| format!("read icon pack {}", path.display()))?;
        let metadata =
            fs::metadata(&path).with_context(|| format!("stat icon pack {}", path.display()))?;
        let modified_unix_ms = file_modified_unix_ms(&metadata);
        let rel_path = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string();
        let source_bytes = metadata.len();
        let source_blake3 = *blake3::hash(&bytes).as_bytes();
        let pack = serde_json::from_str::<IconPack>(
            std::str::from_utf8(&bytes)
                .with_context(|| format!("decode icon pack {}", path.display()))?,
        )
        .with_context(|| format!("parse icon pack {}", path.display()))?;
        let mut icons = pack
            .icons
            .into_iter()
            .map(|(name, icon)| IconPackBodyEntryV1 {
                name,
                body: icon.body,
                width: icon.width,
                height: icon.height,
            })
            .collect::<Vec<_>>();
        icons.sort_by(|left, right| left.name.cmp(&right.name));
        icon_count = icon_count.saturating_add(u32::try_from(icons.len()).unwrap_or(u32::MAX));
        file_count = file_count.saturating_add(1);
        total_bytes = total_bytes.saturating_add(source_bytes);
        latest_modified_unix_ms = max_optional(latest_modified_unix_ms, modified_unix_ms);
        source_hasher.update(rel_path.as_bytes());
        source_hasher.update(&source_bytes.to_le_bytes());
        source_hasher.update(&modified_unix_ms.unwrap_or_default().to_le_bytes());
        source_hasher.update(&source_blake3);

        packs.push(IconPackBodyPackV1 {
            pack: pack.prefix,
            rel_path,
            source_bytes,
            source_blake3,
            icon_count: u32::try_from(icons.len()).unwrap_or(u32::MAX),
            icons,
        });
    }
    packs.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));

    let source_blake3 = *source_hasher.finalize().as_bytes();
    let machine = IconPackBodyMachineV1 {
        selected_data_root: normalize_path(data_dir),
        generated_at_unix_ms: current_unix_ms(),
        source_file_count: file_count,
        source_total_bytes: total_bytes,
        source_blake3,
        pack_count: u32::try_from(packs.len()).unwrap_or(u32::MAX),
        icon_count,
        packs,
    };

    Ok(IconPackBodyMachineBuild {
        source: MachineCacheSource {
            path: data_dir.to_path_buf(),
            bytes: total_bytes,
            modified_unix_ms: latest_modified_unix_ms,
            blake3: source_blake3,
        },
        machine,
    })
}

fn icon_pack_body_machine_paths(
    project_root: &Path,
    data_dir: &Path,
) -> Result<serializer::machine::MachineCachePaths> {
    icon_machine_paths(
        project_root,
        data_dir,
        "pack-body.machine",
        "pack-body.machine.meta.json",
    )
}

fn icon_pack_body_schema() -> MachineCacheSchema {
    MachineCacheSchema {
        name: ICON_PACK_BODY_CACHE_SCHEMA,
        version: 1,
        kind: MachineCacheKind::Index,
    }
}

fn deserialize_icon_pack_body_machine_archive(
    archived: &ArchivedIconPackBodyMachineV1,
) -> Result<IconPackBodyMachineV1> {
    let mut deserializer = rkyv::de::Pool::new();
    let pack_body_machine: std::result::Result<IconPackBodyMachineV1, rkyv::rancor::Error> =
        RkyvDeserialize::deserialize(archived, rkyv::rancor::Strategy::wrap(&mut deserializer));
    pack_body_machine.map_err(|error| anyhow!("deserialize icon pack body machine cache: {error}"))
}

fn pack_body_summary(
    machine: &IconPackBodyMachineV1,
    mode: &'static str,
) -> IconPackBodyReadSummary {
    IconPackBodyReadSummary {
        pack_count: machine.packs.len(),
        icon_count: machine.icon_count as usize,
        source_total_bytes: machine.source_total_bytes,
        mode,
    }
}

fn current_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or_default()
}

fn file_modified_unix_ms(metadata: &fs::Metadata) -> Option<u64> {
    metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
}

fn max_optional(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}

fn normalize_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

pub const PACK_BODY_MACHINE_JSON_SOURCE_AUTHORITATIVE: bool = true;
pub const PACK_BODY_MACHINE_CACHE_ONLY: bool = true;
pub const PACK_BODY_MACHINE_PARSER_ADOPTION_DEFERRED: bool = true;
pub const PACK_BODY_MACHINE_RUNTIME_ADOPTED: bool = false;
pub const PACK_BODY_MACHINE_FULL_ICON_SEARCH_SPEED_CLAIMED: bool = false;
pub const PACK_BODY_MACHINE_FASTER_THAN_UPSTREAM_CLAIMED: bool = false;
pub const PACK_BODY_MACHINE_UPSTREAM_BASELINE_MEASURED: bool = false;
pub const PACK_BODY_MACHINE_SAME_MACHINE_BENCHMARK_REQUIRED: bool = true;
pub const PACK_BODY_MACHINE_PARSER_PARITY_GATE: bool = true;
pub const PACK_BODY_MACHINE_RUNTIME_ADOPTION_DEFERRED: bool = true;
pub const PACK_BODY_MACHINE_CLI_EXPORT_ADOPTED: bool = true;
pub const PACK_BODY_MACHINE_STARTUP_RECEIPT_EVIDENCE_ADOPTED: bool = true;
pub const PACK_BODY_MACHINE_FAST_CACHE_HIT_READ_ADOPTED: bool = true;
pub const PACK_BODY_MACHINE_DEEP_SOURCE_AUDIT_AVAILABLE: bool = true;
pub const PACK_BODY_MACHINE_FAST_READ_STILL_USES_SOURCE_FINGERPRINT: bool = true;
pub const PACK_BODY_MACHINE_ENGINE_BODY_RESOLUTION_ADOPTED: bool = true;
pub const PACK_BODY_MACHINE_ENGINE_SEARCH_ADOPTED: bool = false;

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> IconPackBodyMachineV1 {
        IconPackBodyMachineV1 {
            selected_data_root: "test".to_string(),
            generated_at_unix_ms: 0,
            source_file_count: 1,
            source_total_bytes: 10,
            source_blake3: [0; 32],
            pack_count: 1,
            icon_count: 2,
            packs: vec![IconPackBodyPackV1 {
                pack: "test".to_string(),
                rel_path: "test.json".to_string(),
                source_bytes: 10,
                source_blake3: [1; 32],
                icon_count: 2,
                icons: vec![
                    IconPackBodyEntryV1 {
                        name: "home".to_string(),
                        body: "<path />".to_string(),
                        width: Some(24.0),
                        height: Some(24.0),
                    },
                    IconPackBodyEntryV1 {
                        name: "arrow".to_string(),
                        body: "<path />".to_string(),
                        width: None,
                        height: None,
                    },
                ],
            }],
        }
    }

    #[test]
    fn pack_body_machine_validation_accepts_valid_shape() {
        validate_icon_pack_body_machine(&fixture()).unwrap();
    }

    #[test]
    fn pack_body_machine_validation_rejects_icon_count_drift() {
        let mut machine = fixture();
        machine.packs[0].icon_count = 3;

        let error = validate_icon_pack_body_machine(&machine).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("pack body machine pack test icon count 3")
        );
    }

    #[test]
    fn pack_body_machine_validation_rejects_path_escape() {
        let mut machine = fixture();
        machine.packs[0].rel_path = "../test.json".to_string();

        let error = validate_icon_pack_body_machine(&machine).unwrap_err();

        assert!(error.to_string().contains("non-local rel_path"));
    }

    #[test]
    fn pack_body_machine_validation_rejects_drive_relative_path() {
        let mut machine = fixture();
        machine.packs[0].rel_path = "C:test.json".to_string();

        let error = validate_icon_pack_body_machine(&machine).unwrap_err();

        assert!(error.to_string().contains("non-local rel_path"));
    }

    #[test]
    fn pack_body_machine_validation_rejects_invalid_dimensions() {
        let mut machine = fixture();
        machine.packs[0].icons[0].width = Some(f32::NAN);

        let error = validate_icon_pack_body_machine(&machine).unwrap_err();

        assert!(error.to_string().contains("invalid width"));
    }

    #[test]
    fn pack_body_parser_metadata_parity_accepts_matching_order() {
        let metadata = vec![
            IconMetadata {
                id: 0,
                name: "home".to_string(),
                pack: "test".to_string(),
                category: String::new(),
                tags: Vec::new(),
                popularity: 0,
            },
            IconMetadata {
                id: 1,
                name: "arrow".to_string(),
                pack: "test".to_string(),
                category: String::new(),
                tags: Vec::new(),
                popularity: 0,
            },
        ];

        validate_icon_pack_body_parser_metadata(&fixture(), &metadata).unwrap();
    }

    #[test]
    fn pack_body_parser_metadata_parity_rejects_order_drift() {
        let metadata = vec![
            IconMetadata {
                id: 0,
                name: "arrow".to_string(),
                pack: "test".to_string(),
                category: String::new(),
                tags: Vec::new(),
                popularity: 0,
            },
            IconMetadata {
                id: 1,
                name: "home".to_string(),
                pack: "test".to_string(),
                category: String::new(),
                tags: Vec::new(),
                popularity: 0,
            },
        ];

        let error = validate_icon_pack_body_parser_metadata(&fixture(), &metadata).unwrap_err();

        assert!(error.to_string().contains("does not match machine icon"));
    }

    #[test]
    fn pack_body_parser_metadata_parity_rejects_id_drift() {
        let metadata = vec![
            IconMetadata {
                id: 0,
                name: "home".to_string(),
                pack: "test".to_string(),
                category: String::new(),
                tags: Vec::new(),
                popularity: 0,
            },
            IconMetadata {
                id: 7,
                name: "arrow".to_string(),
                pack: "test".to_string(),
                category: String::new(),
                tags: Vec::new(),
                popularity: 0,
            },
        ];

        let error = validate_icon_pack_body_parser_metadata(&fixture(), &metadata).unwrap_err();

        assert!(error.to_string().contains("does not match expected id"));
    }

    #[test]
    fn pack_body_runtime_metadata_parity_rejects_pack_or_name_drift() {
        let metadata = vec![
            IconMetadata {
                id: 0,
                name: "home".to_string(),
                pack: "wrong".to_string(),
                category: String::new(),
                tags: Vec::new(),
                popularity: 0,
            },
            IconMetadata {
                id: 1,
                name: "arrow".to_string(),
                pack: "test".to_string(),
                category: String::new(),
                tags: Vec::new(),
                popularity: 0,
            },
        ];

        let error = validate_icon_pack_body_runtime_metadata(&fixture(), &metadata).unwrap_err();

        assert!(error.to_string().contains("does not match machine pack"));
    }

    #[test]
    fn pack_body_resolver_returns_body_and_dimensions() {
        let resolved = resolve_icon_pack_body(&fixture(), "test", "home").unwrap();

        assert_eq!(resolved.body, "<path />");
        assert_eq!(resolved.width, Some(24.0));
        assert_eq!(resolved.height, Some(24.0));
    }

    #[test]
    fn pack_body_resolver_returns_none_for_missing_icon() {
        assert!(resolve_icon_pack_body(&fixture(), "test", "missing").is_none());
        assert!(resolve_icon_pack_body(&fixture(), "missing", "home").is_none());
    }

    fn write_mismatched_body_cache_fixture(test_name: &str) -> (PathBuf, PathBuf) {
        let project_root = std::env::temp_dir().join(format!(
            "dx-icons-pack-body-{test_name}-{}-{}",
            std::process::id(),
            current_unix_ms()
        ));
        let data_dir = project_root.join("data");
        fs::create_dir_all(&data_dir).unwrap();
        fs::write(
            data_dir.join("test.json"),
            r#"{
  "prefix": "test",
  "info": {
    "name": "Test",
    "total": 1,
    "author": { "name": "DX", "url": null },
    "license": { "title": "MIT", "spdx": null, "url": null }
  },
  "icons": {
    "home": { "body": "<path id=\"json\" />", "width": 24, "height": 24 }
  }
}"#,
        )
        .unwrap();

        let mut build = build_icon_pack_body_machine_with_source(&data_dir).unwrap();
        build.machine.packs[0].icons[0].body = "<path id=\"machine\" />".to_string();
        validate_icon_pack_body_machine(&build.machine).unwrap();
        let paths = icon_pack_body_machine_paths(&project_root, &data_dir).unwrap();
        write_typed_machine_cache(
            &build.machine,
            &build.source,
            &paths,
            icon_pack_body_schema(),
            MachineCacheWriteOptions {
                codec: MachineCacheCodec::None,
            },
        )
        .unwrap();

        (project_root, data_dir)
    }

    #[test]
    fn pack_body_machine_fast_cache_hit_skips_deep_source_audit() {
        let (project_root, data_dir) =
            write_mismatched_body_cache_fixture("fast-cache-hit-skips-audit");

        let read = read_icon_pack_body_machine_cache(&project_root, &data_dir).unwrap();
        let resolved = resolve_icon_pack_body(&read.pack_body_machine, "test", "home").unwrap();

        assert_eq!(read.mode, "mmap");
        assert_eq!(resolved.body, "<path id=\"machine\" />");

        let _ = fs::remove_dir_all(project_root);
    }

    #[test]
    fn pack_body_machine_source_audit_rejects_cached_body_drift() {
        let (project_root, data_dir) =
            write_mismatched_body_cache_fixture("source-audit-rejects-drift");

        let error = read_icon_pack_body_machine_cache_with_source_audit(&project_root, &data_dir)
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("body does not match current JSON")
        );

        let _ = fs::remove_dir_all(project_root);
    }
}
