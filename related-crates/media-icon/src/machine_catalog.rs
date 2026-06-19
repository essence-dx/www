use crate::types::IconMetadata;
use anyhow::{Context, Result, anyhow, bail};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serializer::machine::{
    MachineCacheCodec, MachineCacheKind, MachineCachePaths, MachineCacheSchema, MachineCacheSource,
    MachineCacheWriteOptions, access_typed_machine_cache, open_typed_machine_cache,
    write_typed_machine_cache,
};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const ICON_CATALOG_CACHE_SCHEMA: &str = "dx.icon.catalog.v1";
const ICON_PREFIX_CACHE_SCHEMA: &str = "dx.icon.prefix.v1";

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconCatalogMachineV1 {
    pub selected_data_root: String,
    pub generated_at_unix_ms: u64,
    pub source_file_count: u32,
    pub source_total_bytes: u64,
    pub source_blake3: [u8; 32],
    pub icon_count: u32,
    pub packs: Vec<IconCatalogPackV1>,
    pub pack_count: u32,
    pub entries: Vec<IconCatalogEntryV1>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconCatalogPackV1 {
    pub pack: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconCatalogEntryV1 {
    pub id: u32,
    pub pack_id: u32,
    pub name: String,
    pub category: String,
    pub tags: Vec<String>,
    pub popularity: u32,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconPrefixMachineV1 {
    pub selected_data_root: String,
    pub generated_at_unix_ms: u64,
    pub source_file_count: u32,
    pub source_total_bytes: u64,
    pub source_blake3: [u8; 32],
    pub icon_count: u32,
    pub max_prefix_len: u8,
    pub entries: Vec<IconPrefixEntryV1>,
    pub icon_ids: Vec<u32>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconPrefixEntryV1 {
    pub prefix: String,
    pub start: u32,
    pub len: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconCatalogPrefixReadSummary {
    pub icon_count: usize,
    pub prefix_count: usize,
    pub mode: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconCatalogMachineRuntimeRead {
    pub catalog_machine: IconCatalogMachineV1,
    pub mode: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconPrefixMachineRuntimeRead {
    pub prefix_machine: IconPrefixMachineV1,
    pub mode: &'static str,
}

pub fn write_icon_catalog_prefix_machine_caches_for_index_output(
    data_dir: &Path,
    output_dir: &Path,
    icons: &[IconMetadata],
) -> Result<(PathBuf, PathBuf)> {
    let project_root = project_root_for_index_output(output_dir)?;
    write_icon_catalog_prefix_machine_caches(&project_root, data_dir, icons)
}

pub fn write_icon_catalog_prefix_machine_caches(
    project_root: &Path,
    data_dir: &Path,
    icons: &[IconMetadata],
) -> Result<(PathBuf, PathBuf)> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    let catalog = build_icon_catalog_machine(data_dir, icons, &source);
    let prefix = build_icon_prefix_machine(data_dir, icons, &source);
    let catalog_paths = icon_catalog_machine_paths(project_root, data_dir)?;
    let prefix_paths = icon_prefix_machine_paths(project_root, data_dir)?;
    let options = MachineCacheWriteOptions {
        codec: MachineCacheCodec::None,
    };

    let catalog_receipt = write_typed_machine_cache(
        &catalog,
        &source.source,
        &catalog_paths,
        icon_catalog_schema(),
        options,
    )
    .with_context(|| {
        format!(
            "write icon catalog machine cache {}",
            catalog_paths.machine.display()
        )
    })?;
    let prefix_receipt = write_typed_machine_cache(
        &prefix,
        &source.source,
        &prefix_paths,
        icon_prefix_schema(),
        options,
    )
    .with_context(|| {
        format!(
            "write icon prefix machine cache {}",
            prefix_paths.machine.display()
        )
    })?;

    Ok((catalog_receipt.machine, prefix_receipt.machine))
}

pub fn read_icon_catalog_prefix_machine_cache_summary(
    project_root: &Path,
    data_dir: &Path,
) -> Option<IconCatalogPrefixReadSummary> {
    let source = icon_catalog_source_fingerprint(data_dir).ok()?;
    let catalog_paths = icon_catalog_machine_paths(project_root, data_dir).ok()?;
    let prefix_paths = icon_prefix_machine_paths(project_root, data_dir).ok()?;

    if let (Ok(catalog), Ok(prefix)) = (
        open_typed_machine_cache::<IconCatalogMachineV1>(
            &catalog_paths,
            &source.source,
            icon_catalog_schema(),
        ),
        open_typed_machine_cache::<IconPrefixMachineV1>(
            &prefix_paths,
            &source.source,
            icon_prefix_schema(),
        ),
    ) {
        return Some(summary_from_archives(
            catalog.archived(),
            prefix.archived(),
            "mmap",
        ));
    }

    let catalog_bytes = fs::read(&catalog_paths.machine).ok()?;
    let prefix_bytes = fs::read(&prefix_paths.machine).ok()?;
    let catalog = access_typed_machine_cache::<IconCatalogMachineV1>(
        &catalog_bytes,
        &source.source,
        icon_catalog_schema(),
    )
    .ok()?;
    let prefix = access_typed_machine_cache::<IconPrefixMachineV1>(
        &prefix_bytes,
        &source.source,
        icon_prefix_schema(),
    )
    .ok()?;
    Some(summary_from_archives(catalog, prefix, "bytes"))
}

pub fn read_icon_catalog_machine_cache_for_index_output(
    index_dir: &Path,
    data_dir: &Path,
) -> Result<IconCatalogMachineRuntimeRead> {
    let project_root = project_root_for_index_output(index_dir)?;
    read_icon_catalog_machine_cache(&project_root, data_dir)
}

pub fn read_icon_catalog_machine_cache(
    project_root: &Path,
    data_dir: &Path,
) -> Result<IconCatalogMachineRuntimeRead> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    read_icon_catalog_machine_cache_with_source_fingerprint(project_root, data_dir, &source)
}

pub(crate) fn read_icon_catalog_machine_cache_with_source_fingerprint(
    project_root: &Path,
    data_dir: &Path,
    source: &IconCatalogSourceFingerprint,
) -> Result<IconCatalogMachineRuntimeRead> {
    let catalog_paths = icon_catalog_machine_paths(project_root, data_dir)?;

    if let Ok(catalog) = open_typed_machine_cache::<IconCatalogMachineV1>(
        &catalog_paths,
        &source.source,
        icon_catalog_schema(),
    ) {
        return Ok(IconCatalogMachineRuntimeRead {
            catalog_machine: deserialize_icon_catalog_machine_archive(catalog.archived())?,
            mode: "mmap",
        });
    }

    let catalog_bytes = fs::read(&catalog_paths.machine).with_context(|| {
        format!(
            "read icon catalog machine cache {}",
            catalog_paths.machine.display()
        )
    })?;
    let catalog = access_typed_machine_cache::<IconCatalogMachineV1>(
        &catalog_bytes,
        &source.source,
        icon_catalog_schema(),
    )?;

    Ok(IconCatalogMachineRuntimeRead {
        catalog_machine: deserialize_icon_catalog_machine_archive(catalog)?,
        mode: "bytes",
    })
}

pub fn read_icon_prefix_machine_cache_for_index_output(
    index_dir: &Path,
    data_dir: &Path,
) -> Result<IconPrefixMachineRuntimeRead> {
    let project_root = project_root_for_index_output(index_dir)?;
    read_icon_prefix_machine_cache(&project_root, data_dir)
}

pub fn read_icon_prefix_machine_cache(
    project_root: &Path,
    data_dir: &Path,
) -> Result<IconPrefixMachineRuntimeRead> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    read_icon_prefix_machine_cache_with_source_fingerprint(project_root, data_dir, &source)
}

pub(crate) fn read_icon_prefix_machine_cache_with_source_fingerprint(
    project_root: &Path,
    data_dir: &Path,
    source: &IconCatalogSourceFingerprint,
) -> Result<IconPrefixMachineRuntimeRead> {
    let prefix_paths = icon_prefix_machine_paths(project_root, data_dir)?;

    if let Ok(prefix) = open_typed_machine_cache::<IconPrefixMachineV1>(
        &prefix_paths,
        &source.source,
        icon_prefix_schema(),
    ) {
        return Ok(IconPrefixMachineRuntimeRead {
            prefix_machine: deserialize_icon_prefix_machine_archive(prefix.archived())?,
            mode: "mmap",
        });
    }

    let prefix_bytes = fs::read(&prefix_paths.machine).with_context(|| {
        format!(
            "read icon prefix machine cache {}",
            prefix_paths.machine.display()
        )
    })?;
    let prefix = access_typed_machine_cache::<IconPrefixMachineV1>(
        &prefix_bytes,
        &source.source,
        icon_prefix_schema(),
    )?;

    Ok(IconPrefixMachineRuntimeRead {
        prefix_machine: deserialize_icon_prefix_machine_archive(prefix)?,
        mode: "bytes",
    })
}

pub fn icon_metadata_from_catalog_machine(
    catalog: &IconCatalogMachineV1,
) -> Result<Vec<IconMetadata>> {
    if catalog.icon_count as usize != catalog.entries.len() {
        bail!(
            "catalog machine icon count {} does not match entry count {}",
            catalog.icon_count,
            catalog.entries.len()
        );
    }

    if catalog.pack_count as usize != catalog.packs.len() {
        bail!(
            "catalog machine pack count {} does not match pack table count {}",
            catalog.pack_count,
            catalog.packs.len()
        );
    }

    let mut seen_packs = HashSet::with_capacity(catalog.packs.len());
    for pack in &catalog.packs {
        if !seen_packs.insert(pack.pack.as_str()) {
            bail!("catalog machine duplicate pack {}", pack.pack);
        }
    }

    let mut seen_ids = HashSet::with_capacity(catalog.entries.len());
    let mut metadata = Vec::with_capacity(catalog.entries.len());
    for entry in &catalog.entries {
        if !seen_ids.insert(entry.id) {
            bail!("catalog machine duplicate icon id {}", entry.id);
        }

        let pack_id = entry.pack_id as usize;
        let pack = catalog.packs.get(pack_id).ok_or_else(|| {
            anyhow!(
                "catalog machine entry {} pack_id {} exceeds pack count {}",
                entry.id,
                entry.pack_id,
                catalog.packs.len()
            )
        })?;

        metadata.push(IconMetadata {
            id: entry.id,
            name: entry.name.clone(),
            pack: pack.pack.clone(),
            category: entry.category.clone(),
            tags: entry.tags.clone(),
            popularity: entry.popularity,
        });
    }

    Ok(metadata)
}

pub fn validate_catalog_machine_metadata_parity(
    catalog_metadata: &[IconMetadata],
    runtime_metadata: &[IconMetadata],
) -> Result<()> {
    if catalog_metadata.len() != runtime_metadata.len() {
        bail!(
            "catalog machine metadata parity mismatch: catalog count {} does not match runtime count {}",
            catalog_metadata.len(),
            runtime_metadata.len()
        );
    }

    for (position, (catalog_icon, runtime_icon)) in catalog_metadata
        .iter()
        .zip(runtime_metadata.iter())
        .enumerate()
    {
        if !icon_metadata_matches(catalog_icon, runtime_icon) {
            bail!(
                "catalog machine metadata parity mismatch at position {}: catalog id {} runtime id {}",
                position,
                catalog_icon.id,
                runtime_icon.id
            );
        }
    }

    Ok(())
}

fn icon_metadata_matches(left: &IconMetadata, right: &IconMetadata) -> bool {
    left.id == right.id
        && left.name == right.name
        && left.pack == right.pack
        && left.category == right.category
        && left.tags == right.tags
        && left.popularity == right.popularity
}

fn build_icon_catalog_machine(
    data_dir: &Path,
    icons: &[IconMetadata],
    source: &IconCatalogSourceFingerprint,
) -> IconCatalogMachineV1 {
    let mut pack_ids: BTreeMap<String, u32> = BTreeMap::new();
    let entries = icons
        .iter()
        .map(|icon| {
            let next_pack_id = pack_ids.len() as u32;
            let pack_id = *pack_ids.entry(icon.pack.clone()).or_insert(next_pack_id);
            IconCatalogEntryV1 {
                id: icon.id,
                pack_id,
                name: icon.name.clone(),
                category: icon.category.clone(),
                tags: icon.tags.clone(),
                popularity: icon.popularity,
            }
        })
        .collect();
    let mut packs_by_id = pack_ids
        .into_iter()
        .map(|(pack, pack_id)| (pack_id, pack))
        .collect::<Vec<_>>();
    packs_by_id.sort_by_key(|(pack_id, _pack)| *pack_id);
    let packs = packs_by_id
        .into_iter()
        .map(|(_pack_id, pack)| IconCatalogPackV1 { pack })
        .collect::<Vec<_>>();

    IconCatalogMachineV1 {
        selected_data_root: normalize_path(data_dir),
        generated_at_unix_ms: current_unix_ms(),
        source_file_count: source.file_count,
        source_total_bytes: source.source.bytes,
        source_blake3: source.source.blake3,
        icon_count: u32::try_from(icons.len()).unwrap_or(u32::MAX),
        pack_count: u32::try_from(packs.len()).unwrap_or(u32::MAX),
        packs,
        entries,
    }
}

fn build_icon_prefix_machine(
    data_dir: &Path,
    icons: &[IconMetadata],
    source: &IconCatalogSourceFingerprint,
) -> IconPrefixMachineV1 {
    let mut prefix_map: BTreeMap<String, Vec<u32>> = BTreeMap::new();

    for icon in icons {
        let name = icon.name.to_lowercase();
        for len in 1..=3.min(name.len()) {
            prefix_map
                .entry(name[..len].to_string())
                .or_default()
                .push(icon.id);
        }
    }

    let mut icon_ids = Vec::new();
    let mut entries = Vec::new();
    for (prefix, ids) in prefix_map {
        let start = u32::try_from(icon_ids.len()).unwrap_or(u32::MAX);
        let len = u32::try_from(ids.len()).unwrap_or(u32::MAX);
        icon_ids.extend(ids);
        entries.push(IconPrefixEntryV1 { prefix, start, len });
    }

    IconPrefixMachineV1 {
        selected_data_root: normalize_path(data_dir),
        generated_at_unix_ms: current_unix_ms(),
        source_file_count: source.file_count,
        source_total_bytes: source.source.bytes,
        source_blake3: source.source.blake3,
        icon_count: u32::try_from(icons.len()).unwrap_or(u32::MAX),
        max_prefix_len: 3,
        entries,
        icon_ids,
    }
}

fn icon_catalog_machine_paths(project_root: &Path, data_dir: &Path) -> Result<MachineCachePaths> {
    icon_machine_paths(
        project_root,
        data_dir,
        "catalog.machine",
        "catalog.machine.meta.json",
    )
}

fn icon_prefix_machine_paths(project_root: &Path, data_dir: &Path) -> Result<MachineCachePaths> {
    icon_machine_paths(
        project_root,
        data_dir,
        "prefix.machine",
        "prefix.machine.meta.json",
    )
}

pub(crate) fn icon_machine_paths(
    project_root: &Path,
    data_dir: &Path,
    machine_file: &str,
    metadata_file: &str,
) -> Result<MachineCachePaths> {
    let machine_root = project_root.join(".dx/icon/machine/v1");
    if !machine_root.starts_with(project_root.join(".dx")) {
        bail!(
            "icon catalog cache path escaped .dx root: {}",
            machine_root.display()
        );
    }

    Ok(MachineCachePaths {
        source: data_dir.to_path_buf(),
        machine: machine_root.join(machine_file),
        metadata: machine_root.join(metadata_file),
    })
}

pub(crate) struct IconCatalogSourceFingerprint {
    pub(crate) source: MachineCacheSource,
    pub(crate) file_count: u32,
}

pub(crate) fn icon_catalog_source_fingerprint(
    data_dir: &Path,
) -> Result<IconCatalogSourceFingerprint> {
    let mut json_files = fs::read_dir(data_dir)
        .with_context(|| format!("read icon data directory {}", data_dir.display()))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("list icon data directory {}", data_dir.display()))?;
    json_files.sort_by_key(|entry| entry.file_name());

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
        let modified_unix_ms = modified_unix_ms(&metadata);
        let rel_path = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string();
        let len = metadata.len();
        let file_hash = blake3::hash(&bytes);

        file_count = file_count.saturating_add(1);
        total_bytes = total_bytes.saturating_add(len);
        latest_modified_unix_ms = max_optional(latest_modified_unix_ms, modified_unix_ms);
        source_hasher.update(rel_path.as_bytes());
        source_hasher.update(&len.to_le_bytes());
        source_hasher.update(&modified_unix_ms.unwrap_or_default().to_le_bytes());
        source_hasher.update(file_hash.as_bytes());
    }

    Ok(IconCatalogSourceFingerprint {
        source: MachineCacheSource {
            path: data_dir.to_path_buf(),
            bytes: total_bytes,
            modified_unix_ms: latest_modified_unix_ms,
            blake3: *source_hasher.finalize().as_bytes(),
        },
        file_count,
    })
}

pub(crate) fn project_root_for_index_output(output_dir: &Path) -> Result<PathBuf> {
    if output_dir.file_name().and_then(|value| value.to_str()) != Some("index") {
        bail!(
            "icon catalog machine cache expects an index output directory, got {}",
            output_dir.display()
        );
    }

    if let Some(parent) = output_dir.parent()
        && !parent.as_os_str().is_empty()
    {
        return Ok(parent.to_path_buf());
    }

    std::env::current_dir().context("resolve current directory for relative index output directory")
}

fn icon_catalog_schema() -> MachineCacheSchema {
    MachineCacheSchema {
        name: ICON_CATALOG_CACHE_SCHEMA,
        version: 1,
        kind: MachineCacheKind::Index,
    }
}

fn icon_prefix_schema() -> MachineCacheSchema {
    MachineCacheSchema {
        name: ICON_PREFIX_CACHE_SCHEMA,
        version: 1,
        kind: MachineCacheKind::Index,
    }
}

fn summary_from_archives(
    catalog: &ArchivedIconCatalogMachineV1,
    prefix: &ArchivedIconPrefixMachineV1,
    mode: &'static str,
) -> IconCatalogPrefixReadSummary {
    IconCatalogPrefixReadSummary {
        icon_count: catalog.entries.len(),
        prefix_count: prefix.entries.len(),
        mode,
    }
}

fn deserialize_icon_catalog_machine_archive(
    archived: &ArchivedIconCatalogMachineV1,
) -> Result<IconCatalogMachineV1> {
    let mut deserializer = rkyv::de::Pool::new();
    let catalog_machine: std::result::Result<IconCatalogMachineV1, rkyv::rancor::Error> =
        RkyvDeserialize::deserialize(archived, rkyv::rancor::Strategy::wrap(&mut deserializer));
    catalog_machine.map_err(|error| anyhow!("deserialize icon catalog machine cache: {error}"))
}

fn deserialize_icon_prefix_machine_archive(
    archived: &ArchivedIconPrefixMachineV1,
) -> Result<IconPrefixMachineV1> {
    let mut deserializer = rkyv::de::Pool::new();
    let prefix_machine: std::result::Result<IconPrefixMachineV1, rkyv::rancor::Error> =
        RkyvDeserialize::deserialize(archived, rkyv::rancor::Strategy::wrap(&mut deserializer));
    prefix_machine.map_err(|error| anyhow!("deserialize icon prefix machine cache: {error}"))
}

fn modified_unix_ms(metadata: &fs::Metadata) -> Option<u64> {
    metadata
        .modified()
        .ok()
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
}

fn max_optional(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn current_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or_default()
}

fn normalize_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog_fixture() -> IconCatalogMachineV1 {
        IconCatalogMachineV1 {
            selected_data_root: "test".to_string(),
            generated_at_unix_ms: 0,
            source_file_count: 1,
            source_total_bytes: 1,
            source_blake3: [0; 32],
            icon_count: 2,
            packs: vec![
                IconCatalogPackV1 {
                    pack: "lucide".to_string(),
                },
                IconCatalogPackV1 {
                    pack: "heroicons".to_string(),
                },
            ],
            pack_count: 2,
            entries: vec![
                IconCatalogEntryV1 {
                    id: 100,
                    pack_id: 0,
                    name: "home".to_string(),
                    category: "navigation".to_string(),
                    tags: vec!["house".to_string()],
                    popularity: 90,
                },
                IconCatalogEntryV1 {
                    id: 200,
                    pack_id: 1,
                    name: "arrow-right".to_string(),
                    category: "arrows".to_string(),
                    tags: vec!["next".to_string()],
                    popularity: 80,
                },
            ],
        }
    }

    #[test]
    fn icon_metadata_from_catalog_machine_rebuilds_metadata() {
        let metadata = icon_metadata_from_catalog_machine(&catalog_fixture()).unwrap();

        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata[0].id, 100);
        assert_eq!(metadata[0].pack, "lucide");
        assert_eq!(metadata[1].id, 200);
        assert_eq!(metadata[1].pack, "heroicons");
    }

    #[test]
    fn icon_metadata_from_catalog_machine_rejects_duplicate_ids() {
        let mut catalog = catalog_fixture();
        catalog.entries[1].id = 100;

        let error = icon_metadata_from_catalog_machine(&catalog).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("catalog machine duplicate icon id 100")
        );
    }

    #[test]
    fn validate_catalog_machine_metadata_parity_rejects_drift() {
        let catalog_metadata = icon_metadata_from_catalog_machine(&catalog_fixture()).unwrap();
        let mut runtime_metadata = catalog_metadata.clone();
        runtime_metadata[1].name = "arrow-left".to_string();

        let error = validate_catalog_machine_metadata_parity(&catalog_metadata, &runtime_metadata)
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("catalog machine metadata parity mismatch")
        );
    }
}

const _: &str = "json_source_authoritative";
const _: &str = "catalog_prefix_cache_only";
const _: &str = "full_icon_runtime_baseline_measured";
const _: &str = "faster_than_upstream_claimed";
const _: &str = "upstream_baseline_measured";
const _: &str = "same_machine_benchmark_required";
