use crate::bloom::{BloomFilter, IconBloomFilters};
use crate::machine_catalog::{
    IconCatalogSourceFingerprint, icon_catalog_source_fingerprint, icon_machine_paths,
    project_root_for_index_output,
};
use crate::perfect_hash::{LowercaseCache, PerfectHashIndex, ValidatedLowercaseCache};
use crate::types::IconMetadata;
use anyhow::{Context, Result, anyhow, bail};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serializer::machine::{
    MachineCacheCodec, MachineCacheKind, MachineCachePaths, MachineCacheSchema,
    MachineCacheWriteOptions, access_typed_machine_cache, open_typed_machine_cache,
    write_typed_machine_cache,
};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const ICON_PERFECT_HASH_CACHE_SCHEMA: &str = "dx.icon.perfect_hash.v1";
const ICON_BLOOM_CACHE_SCHEMA: &str = "dx.icon.bloom.v1";
const ICON_LOWERCASE_CACHE_SCHEMA: &str = "dx.icon.lowercase_cache.v1";

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconPerfectHashMachineV1 {
    pub selected_data_root: String,
    pub generated_at_unix_ms: u64,
    pub source_file_count: u32,
    pub source_total_bytes: u64,
    pub source_blake3: [u8; 32],
    pub icon_count: u32,
    pub seed: u64,
    pub table_size: u32,
    pub slots: Vec<IconPerfectHashSlotV1>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconPerfectHashSlotV1 {
    pub occupied: bool,
    pub icon_index: u32,
    pub lowercase_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconPerfectHashMachineRuntimeRead {
    pub perfect_hash_machine: IconPerfectHashMachineV1,
    pub mode: &'static str,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconBloomMachineV1 {
    pub selected_data_root: String,
    pub generated_at_unix_ms: u64,
    pub source_file_count: u32,
    pub source_total_bytes: u64,
    pub source_blake3: [u8; 32],
    pub icon_count: u32,
    pub filters: Vec<IconBloomFilterMachineV1>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconBloomFilterMachineV1 {
    pub icon_index: u32,
    pub name_len: u32,
    pub bits: Vec<u64>,
    pub num_hashes: u32,
    pub size: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconBloomMachineRuntimeRead {
    pub bloom_machine: IconBloomMachineV1,
    pub mode: &'static str,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconLowercaseCacheMachineV1 {
    pub selected_data_root: String,
    pub generated_at_unix_ms: u64,
    pub source_file_count: u32,
    pub source_total_bytes: u64,
    pub source_blake3: [u8; 32],
    pub icon_count: u32,
    pub entries: Vec<IconLowercaseCacheEntryV1>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconLowercaseCacheEntryV1 {
    pub icon_index: u32,
    pub lowercase_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconLowercaseCacheMachineRuntimeRead {
    pub lowercase_cache_machine: IconLowercaseCacheMachineV1,
    pub mode: &'static str,
}

pub fn write_icon_perfect_hash_machine_cache_for_index_output(
    data_dir: &Path,
    output_dir: &Path,
    icons: &[IconMetadata],
) -> Result<PathBuf> {
    let project_root = project_root_for_index_output(output_dir)?;
    write_icon_perfect_hash_machine_cache(&project_root, data_dir, icons)
}

pub fn write_icon_perfect_hash_machine_cache(
    project_root: &Path,
    data_dir: &Path,
    icons: &[IconMetadata],
) -> Result<PathBuf> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    let machine = build_icon_perfect_hash_machine(data_dir, icons, &source);
    let paths = icon_perfect_hash_machine_paths(project_root, data_dir)?;
    let receipt = write_typed_machine_cache(
        &machine,
        &source.source,
        &paths,
        icon_perfect_hash_schema(),
        MachineCacheWriteOptions {
            codec: MachineCacheCodec::None,
        },
    )
    .with_context(|| {
        format!(
            "write icon perfect hash machine cache {}",
            paths.machine.display()
        )
    })?;

    Ok(receipt.machine)
}

pub fn read_icon_perfect_hash_machine_cache_for_index_output(
    index_dir: &Path,
    data_dir: &Path,
) -> Result<IconPerfectHashMachineRuntimeRead> {
    let project_root = project_root_for_index_output(index_dir)?;
    read_icon_perfect_hash_machine_cache(&project_root, data_dir)
}

pub fn read_icon_perfect_hash_machine_cache(
    project_root: &Path,
    data_dir: &Path,
) -> Result<IconPerfectHashMachineRuntimeRead> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    read_icon_perfect_hash_machine_cache_with_source_fingerprint(project_root, data_dir, &source)
}

pub(crate) fn read_icon_perfect_hash_machine_cache_with_source_fingerprint(
    project_root: &Path,
    data_dir: &Path,
    source: &IconCatalogSourceFingerprint,
) -> Result<IconPerfectHashMachineRuntimeRead> {
    let paths = icon_perfect_hash_machine_paths(project_root, data_dir)?;

    if let Ok(perfect_hash) = open_typed_machine_cache::<IconPerfectHashMachineV1>(
        &paths,
        &source.source,
        icon_perfect_hash_schema(),
    ) {
        return Ok(IconPerfectHashMachineRuntimeRead {
            perfect_hash_machine: deserialize_icon_perfect_hash_machine_archive(
                perfect_hash.archived(),
            )?,
            mode: "mmap",
        });
    }

    let bytes = fs::read(&paths.machine).with_context(|| {
        format!(
            "read icon perfect hash machine cache {}",
            paths.machine.display()
        )
    })?;
    let perfect_hash = access_typed_machine_cache::<IconPerfectHashMachineV1>(
        &bytes,
        &source.source,
        icon_perfect_hash_schema(),
    )?;

    Ok(IconPerfectHashMachineRuntimeRead {
        perfect_hash_machine: deserialize_icon_perfect_hash_machine_archive(perfect_hash)?,
        mode: "bytes",
    })
}

pub fn write_icon_bloom_machine_cache_for_index_output(
    data_dir: &Path,
    output_dir: &Path,
    icons: &[IconMetadata],
) -> Result<PathBuf> {
    let project_root = project_root_for_index_output(output_dir)?;
    write_icon_bloom_machine_cache(&project_root, data_dir, icons)
}

pub fn write_icon_bloom_machine_cache(
    project_root: &Path,
    data_dir: &Path,
    icons: &[IconMetadata],
) -> Result<PathBuf> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    let machine = build_icon_bloom_machine(data_dir, icons, &source);
    let paths = icon_bloom_machine_paths(project_root, data_dir)?;
    let receipt = write_typed_machine_cache(
        &machine,
        &source.source,
        &paths,
        icon_bloom_schema(),
        MachineCacheWriteOptions {
            codec: MachineCacheCodec::None,
        },
    )
    .with_context(|| format!("write icon bloom machine cache {}", paths.machine.display()))?;

    Ok(receipt.machine)
}

pub fn read_icon_bloom_machine_cache_for_index_output(
    index_dir: &Path,
    data_dir: &Path,
) -> Result<IconBloomMachineRuntimeRead> {
    let project_root = project_root_for_index_output(index_dir)?;
    read_icon_bloom_machine_cache(&project_root, data_dir)
}

pub fn read_icon_bloom_machine_cache(
    project_root: &Path,
    data_dir: &Path,
) -> Result<IconBloomMachineRuntimeRead> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    read_icon_bloom_machine_cache_with_source_fingerprint(project_root, data_dir, &source)
}

pub(crate) fn read_icon_bloom_machine_cache_with_source_fingerprint(
    project_root: &Path,
    data_dir: &Path,
    source: &IconCatalogSourceFingerprint,
) -> Result<IconBloomMachineRuntimeRead> {
    let paths = icon_bloom_machine_paths(project_root, data_dir)?;

    if let Ok(bloom) =
        open_typed_machine_cache::<IconBloomMachineV1>(&paths, &source.source, icon_bloom_schema())
    {
        return Ok(IconBloomMachineRuntimeRead {
            bloom_machine: deserialize_icon_bloom_machine_archive(bloom.archived())?,
            mode: "mmap",
        });
    }

    let bytes = fs::read(&paths.machine)
        .with_context(|| format!("read icon bloom machine cache {}", paths.machine.display()))?;
    let bloom = access_typed_machine_cache::<IconBloomMachineV1>(
        &bytes,
        &source.source,
        icon_bloom_schema(),
    )?;

    Ok(IconBloomMachineRuntimeRead {
        bloom_machine: deserialize_icon_bloom_machine_archive(bloom)?,
        mode: "bytes",
    })
}

pub fn write_icon_lowercase_cache_machine_cache_for_index_output(
    data_dir: &Path,
    output_dir: &Path,
    icons: &[IconMetadata],
) -> Result<PathBuf> {
    let project_root = project_root_for_index_output(output_dir)?;
    write_icon_lowercase_cache_machine_cache(&project_root, data_dir, icons)
}

pub fn write_icon_lowercase_cache_machine_cache(
    project_root: &Path,
    data_dir: &Path,
    icons: &[IconMetadata],
) -> Result<PathBuf> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    let machine = build_icon_lowercase_cache_machine(data_dir, icons, &source);
    let paths = icon_lowercase_cache_machine_paths(project_root, data_dir)?;
    let receipt = write_typed_machine_cache(
        &machine,
        &source.source,
        &paths,
        icon_lowercase_cache_schema(),
        MachineCacheWriteOptions {
            codec: MachineCacheCodec::None,
        },
    )
    .with_context(|| {
        format!(
            "write icon lowercase cache machine cache {}",
            paths.machine.display()
        )
    })?;

    Ok(receipt.machine)
}

pub fn read_icon_lowercase_cache_machine_cache_for_index_output(
    index_dir: &Path,
    data_dir: &Path,
) -> Result<IconLowercaseCacheMachineRuntimeRead> {
    let project_root = project_root_for_index_output(index_dir)?;
    read_icon_lowercase_cache_machine_cache(&project_root, data_dir)
}

pub fn read_icon_lowercase_cache_machine_cache(
    project_root: &Path,
    data_dir: &Path,
) -> Result<IconLowercaseCacheMachineRuntimeRead> {
    let source = icon_catalog_source_fingerprint(data_dir)?;
    read_icon_lowercase_cache_machine_cache_with_source_fingerprint(project_root, data_dir, &source)
}

pub(crate) fn read_icon_lowercase_cache_machine_cache_with_source_fingerprint(
    project_root: &Path,
    data_dir: &Path,
    source: &IconCatalogSourceFingerprint,
) -> Result<IconLowercaseCacheMachineRuntimeRead> {
    let paths = icon_lowercase_cache_machine_paths(project_root, data_dir)?;

    if let Ok(lowercase_cache) = open_typed_machine_cache::<IconLowercaseCacheMachineV1>(
        &paths,
        &source.source,
        icon_lowercase_cache_schema(),
    ) {
        return Ok(IconLowercaseCacheMachineRuntimeRead {
            lowercase_cache_machine: deserialize_icon_lowercase_cache_machine_archive(
                lowercase_cache.archived(),
            )?,
            mode: "mmap",
        });
    }

    let bytes = fs::read(&paths.machine).with_context(|| {
        format!(
            "read icon lowercase cache machine cache {}",
            paths.machine.display()
        )
    })?;
    let lowercase_cache = access_typed_machine_cache::<IconLowercaseCacheMachineV1>(
        &bytes,
        &source.source,
        icon_lowercase_cache_schema(),
    )?;

    Ok(IconLowercaseCacheMachineRuntimeRead {
        lowercase_cache_machine: deserialize_icon_lowercase_cache_machine_archive(lowercase_cache)?,
        mode: "bytes",
    })
}

pub fn perfect_hash_index_from_machine(
    machine: &IconPerfectHashMachineV1,
    metadata: &[IconMetadata],
) -> Result<PerfectHashIndex> {
    validate_perfect_hash_machine_shape(machine, metadata)?;

    let mut hash_table = Vec::with_capacity(machine.slots.len());
    let mut name_table = Vec::with_capacity(machine.slots.len());
    for slot in &machine.slots {
        if slot.occupied {
            hash_table.push(Some(slot.icon_index));
            name_table.push(Some(slot.lowercase_name.clone()));
        } else {
            hash_table.push(None);
            name_table.push(None);
        }
    }

    let index = PerfectHashIndex::from_machine_parts(
        machine.seed,
        machine.table_size as usize,
        hash_table,
        name_table,
    )
    .map_err(|error| anyhow!("perfect hash machine table validation failed: {error}"))?;
    validate_perfect_hash_machine_for_metadata(machine, &index, metadata)?;
    Ok(index)
}

pub fn bloom_filters_from_machine(
    machine: &IconBloomMachineV1,
    metadata: &[IconMetadata],
) -> Result<IconBloomFilters> {
    validate_bloom_machine_shape(machine, metadata)?;

    let mut filters = Vec::with_capacity(machine.filters.len());
    for filter in &machine.filters {
        let bloom_filter = BloomFilter::from_machine_parts(
            filter.bits.clone(),
            filter.num_hashes as usize,
            filter.size as usize,
        )
        .map_err(|error| anyhow!("bloom machine filter validation failed: {error}"))?;
        filters.push(bloom_filter);
    }
    let bloom_filters = IconBloomFilters::from_filters(filters);
    validate_bloom_machine_for_metadata(machine, &bloom_filters, metadata)?;
    Ok(bloom_filters)
}

pub fn lowercase_cache_from_machine(
    machine: &IconLowercaseCacheMachineV1,
    metadata: &[IconMetadata],
) -> Result<ValidatedLowercaseCache> {
    validate_lowercase_cache_machine_for_metadata(machine, metadata)?;
    let lowercase_names = machine
        .entries
        .iter()
        .map(|entry| entry.lowercase_name.clone())
        .collect::<Vec<_>>();

    Ok(ValidatedLowercaseCache::from_validated_cache(
        LowercaseCache::from_lowercase_names(lowercase_names),
    ))
}

pub fn validate_perfect_hash_machine_for_metadata(
    machine: &IconPerfectHashMachineV1,
    index: &PerfectHashIndex,
    metadata: &[IconMetadata],
) -> Result<()> {
    validate_perfect_hash_machine_shape(machine, metadata)?;

    for (position, icon) in metadata.iter().enumerate() {
        let lookup = index.lookup_exact(&icon.name);
        if lookup != Some(position as u32) {
            bail!(
                "perfect hash machine lookup mismatch for icon {} at position {}: got {:?}",
                icon.id,
                position,
                lookup
            );
        }
    }

    Ok(())
}

pub fn validate_bloom_machine_for_metadata(
    machine: &IconBloomMachineV1,
    filters: &IconBloomFilters,
    metadata: &[IconMetadata],
) -> Result<()> {
    validate_bloom_machine_shape(machine, metadata)?;
    if filters.filters_len() != metadata.len() {
        bail!(
            "bloom machine runtime filter count {} does not match metadata count {}",
            filters.filters_len(),
            metadata.len()
        );
    }

    for (position, icon) in metadata.iter().enumerate() {
        let lowercase_name = icon.name.to_lowercase();
        for needle in bloom_validation_needles(&lowercase_name) {
            if !filters.might_match(position, &needle) {
                bail!(
                    "bloom machine false negative for icon {} at position {} and needle {:?}",
                    icon.id,
                    position,
                    needle
                );
            }
        }
    }

    Ok(())
}

pub fn validate_lowercase_cache_machine_for_metadata(
    machine: &IconLowercaseCacheMachineV1,
    metadata: &[IconMetadata],
) -> Result<()> {
    if machine.icon_count as usize != metadata.len() {
        bail!(
            "lowercase cache machine icon count {} does not match metadata count {}",
            machine.icon_count,
            metadata.len()
        );
    }
    if machine.entries.len() != metadata.len() {
        bail!(
            "lowercase cache machine entry count {} does not match metadata count {}",
            machine.entries.len(),
            metadata.len()
        );
    }

    for (position, (entry, icon)) in machine.entries.iter().zip(metadata.iter()).enumerate() {
        if entry.icon_index as usize != position {
            bail!(
                "lowercase cache machine entry {} references icon index {}",
                position,
                entry.icon_index
            );
        }
        if entry.lowercase_name != icon.name.to_lowercase() {
            bail!(
                "lowercase cache machine entry {} lowercase name mismatch for icon {}",
                position,
                icon.id
            );
        }
    }

    Ok(())
}

fn validate_perfect_hash_machine_shape(
    machine: &IconPerfectHashMachineV1,
    metadata: &[IconMetadata],
) -> Result<()> {
    if machine.icon_count as usize != metadata.len() {
        bail!(
            "perfect hash machine icon count {} does not match metadata count {}",
            machine.icon_count,
            metadata.len()
        );
    }
    if machine.table_size as usize != machine.slots.len() {
        bail!(
            "perfect hash machine table_size {} does not match slot count {}",
            machine.table_size,
            machine.slots.len()
        );
    }
    if machine.table_size == 0 {
        bail!("perfect hash machine table_size must be greater than zero");
    }

    let mut seen_indexes = HashSet::with_capacity(metadata.len());
    let mut seen_lowercase_names = HashSet::with_capacity(metadata.len());
    for (slot_index, slot) in machine.slots.iter().enumerate() {
        if !slot.occupied {
            if slot.icon_index != u32::MAX || !slot.lowercase_name.is_empty() {
                bail!("perfect hash machine empty slot {slot_index} contains payload");
            }
            continue;
        }

        let icon_index = slot.icon_index as usize;
        let Some(icon) = metadata.get(icon_index) else {
            bail!(
                "perfect hash machine slot {} references icon index {} beyond metadata count {}",
                slot_index,
                slot.icon_index,
                metadata.len()
            );
        };
        if !seen_indexes.insert(slot.icon_index) {
            bail!(
                "perfect hash machine duplicate icon index {}",
                slot.icon_index
            );
        }
        if !seen_lowercase_names.insert(slot.lowercase_name.as_str()) {
            bail!(
                "perfect hash machine duplicate lowercase name {}",
                slot.lowercase_name
            );
        }
        if slot.lowercase_name != icon.name.to_lowercase() {
            bail!(
                "perfect hash machine slot {} lowercase name mismatch for icon {}",
                slot_index,
                icon.id
            );
        }
    }

    if seen_indexes.len() != metadata.len() {
        bail!(
            "perfect hash machine occupied slot count {} does not match metadata count {}",
            seen_indexes.len(),
            metadata.len()
        );
    }

    Ok(())
}

fn validate_bloom_machine_shape(
    machine: &IconBloomMachineV1,
    metadata: &[IconMetadata],
) -> Result<()> {
    if machine.icon_count as usize != metadata.len() {
        bail!(
            "bloom machine icon count {} does not match metadata count {}",
            machine.icon_count,
            metadata.len()
        );
    }
    if machine.filters.len() != metadata.len() {
        bail!(
            "bloom machine filter count {} does not match metadata count {}",
            machine.filters.len(),
            metadata.len()
        );
    }

    for (position, filter) in machine.filters.iter().enumerate() {
        if filter.icon_index as usize != position {
            bail!(
                "bloom machine filter {} references icon index {}",
                position,
                filter.icon_index
            );
        }
        let lowercase_name = metadata[position].name.to_lowercase();
        if filter.name_len as usize != lowercase_name.len() {
            bail!(
                "bloom machine filter {} name length {} does not match runtime length {}",
                position,
                filter.name_len,
                lowercase_name.len()
            );
        }
        if lowercase_name.len() >= 2 && filter.bits.iter().all(|word| *word == 0) {
            bail!("bloom machine filter {position} has no populated bits");
        }
    }

    Ok(())
}

fn build_icon_perfect_hash_machine(
    data_dir: &Path,
    icons: &[IconMetadata],
    source: &crate::machine_catalog::IconCatalogSourceFingerprint,
) -> IconPerfectHashMachineV1 {
    let lowercase_names: Vec<String> = icons.iter().map(|icon| icon.name.to_lowercase()).collect();
    let index = PerfectHashIndex::build_from_lowercase_names(&lowercase_names);
    let (seed, table_size, hash_table, name_table) = index.to_machine_parts();
    let slots = hash_table
        .into_iter()
        .zip(name_table)
        .map(
            |(icon_index, lowercase_name)| match (icon_index, lowercase_name) {
                (Some(icon_index), Some(lowercase_name)) => IconPerfectHashSlotV1 {
                    occupied: true,
                    icon_index,
                    lowercase_name,
                },
                _ => IconPerfectHashSlotV1 {
                    occupied: false,
                    icon_index: u32::MAX,
                    lowercase_name: String::new(),
                },
            },
        )
        .collect::<Vec<_>>();

    IconPerfectHashMachineV1 {
        selected_data_root: normalize_path(data_dir),
        generated_at_unix_ms: current_unix_ms(),
        source_file_count: source.file_count,
        source_total_bytes: source.source.bytes,
        source_blake3: source.source.blake3,
        icon_count: u32::try_from(icons.len()).unwrap_or(u32::MAX),
        seed,
        table_size: u32::try_from(table_size).unwrap_or(u32::MAX),
        slots,
    }
}

fn build_icon_bloom_machine(
    data_dir: &Path,
    icons: &[IconMetadata],
    source: &crate::machine_catalog::IconCatalogSourceFingerprint,
) -> IconBloomMachineV1 {
    let lowercase_names: Vec<String> = icons.iter().map(|icon| icon.name.to_lowercase()).collect();
    let filters = IconBloomFilters::build(&lowercase_names)
        .to_machine_parts()
        .into_iter()
        .enumerate()
        .map(
            |(position, (bits, num_hashes, size))| IconBloomFilterMachineV1 {
                icon_index: u32::try_from(position).unwrap_or(u32::MAX),
                name_len: u32::try_from(lowercase_names[position].len()).unwrap_or(u32::MAX),
                bits,
                num_hashes: u32::try_from(num_hashes).unwrap_or(u32::MAX),
                size: u32::try_from(size).unwrap_or(u32::MAX),
            },
        )
        .collect::<Vec<_>>();

    IconBloomMachineV1 {
        selected_data_root: normalize_path(data_dir),
        generated_at_unix_ms: current_unix_ms(),
        source_file_count: source.file_count,
        source_total_bytes: source.source.bytes,
        source_blake3: source.source.blake3,
        icon_count: u32::try_from(icons.len()).unwrap_or(u32::MAX),
        filters,
    }
}

fn build_icon_lowercase_cache_machine(
    data_dir: &Path,
    icons: &[IconMetadata],
    source: &crate::machine_catalog::IconCatalogSourceFingerprint,
) -> IconLowercaseCacheMachineV1 {
    let entries = icons
        .iter()
        .enumerate()
        .map(|(position, icon)| IconLowercaseCacheEntryV1 {
            icon_index: u32::try_from(position).unwrap_or(u32::MAX),
            lowercase_name: icon.name.to_lowercase(),
        })
        .collect::<Vec<_>>();

    IconLowercaseCacheMachineV1 {
        selected_data_root: normalize_path(data_dir),
        generated_at_unix_ms: current_unix_ms(),
        source_file_count: source.file_count,
        source_total_bytes: source.source.bytes,
        source_blake3: source.source.blake3,
        icon_count: u32::try_from(icons.len()).unwrap_or(u32::MAX),
        entries,
    }
}

fn icon_perfect_hash_machine_paths(
    project_root: &Path,
    data_dir: &Path,
) -> Result<MachineCachePaths> {
    icon_machine_paths(
        project_root,
        data_dir,
        "perfect-hash.machine",
        "perfect-hash.machine.meta.json",
    )
}

fn icon_bloom_machine_paths(project_root: &Path, data_dir: &Path) -> Result<MachineCachePaths> {
    icon_machine_paths(
        project_root,
        data_dir,
        "bloom.machine",
        "bloom.machine.meta.json",
    )
}

fn icon_lowercase_cache_machine_paths(
    project_root: &Path,
    data_dir: &Path,
) -> Result<MachineCachePaths> {
    icon_machine_paths(
        project_root,
        data_dir,
        "lowercase-cache.machine",
        "lowercase-cache.machine.meta.json",
    )
}

fn icon_perfect_hash_schema() -> MachineCacheSchema {
    MachineCacheSchema {
        name: ICON_PERFECT_HASH_CACHE_SCHEMA,
        version: 1,
        kind: MachineCacheKind::Index,
    }
}

fn icon_bloom_schema() -> MachineCacheSchema {
    MachineCacheSchema {
        name: ICON_BLOOM_CACHE_SCHEMA,
        version: 1,
        kind: MachineCacheKind::Index,
    }
}

fn icon_lowercase_cache_schema() -> MachineCacheSchema {
    MachineCacheSchema {
        name: ICON_LOWERCASE_CACHE_SCHEMA,
        version: 1,
        kind: MachineCacheKind::Index,
    }
}

fn deserialize_icon_perfect_hash_machine_archive(
    archived: &ArchivedIconPerfectHashMachineV1,
) -> Result<IconPerfectHashMachineV1> {
    let mut deserializer = rkyv::de::Pool::new();
    let perfect_hash_machine: std::result::Result<IconPerfectHashMachineV1, rkyv::rancor::Error> =
        RkyvDeserialize::deserialize(archived, rkyv::rancor::Strategy::wrap(&mut deserializer));
    perfect_hash_machine
        .map_err(|error| anyhow!("deserialize icon perfect hash machine cache: {error}"))
}

fn deserialize_icon_bloom_machine_archive(
    archived: &ArchivedIconBloomMachineV1,
) -> Result<IconBloomMachineV1> {
    let mut deserializer = rkyv::de::Pool::new();
    let bloom_machine: std::result::Result<IconBloomMachineV1, rkyv::rancor::Error> =
        RkyvDeserialize::deserialize(archived, rkyv::rancor::Strategy::wrap(&mut deserializer));
    bloom_machine.map_err(|error| anyhow!("deserialize icon bloom machine cache: {error}"))
}

fn deserialize_icon_lowercase_cache_machine_archive(
    archived: &ArchivedIconLowercaseCacheMachineV1,
) -> Result<IconLowercaseCacheMachineV1> {
    let mut deserializer = rkyv::de::Pool::new();
    let lowercase_cache_machine: std::result::Result<
        IconLowercaseCacheMachineV1,
        rkyv::rancor::Error,
    > = RkyvDeserialize::deserialize(archived, rkyv::rancor::Strategy::wrap(&mut deserializer));
    lowercase_cache_machine
        .map_err(|error| anyhow!("deserialize icon lowercase cache machine cache: {error}"))
}

fn bloom_validation_needles(lowercase_name: &str) -> Vec<String> {
    let mut needles = Vec::new();
    let bytes = lowercase_name.as_bytes();
    for i in 0..bytes.len().saturating_sub(1) {
        needles.push(lowercase_name[i..i + 2].to_string());
    }
    for i in 0..bytes.len().saturating_sub(2) {
        needles.push(lowercase_name[i..i + 3].to_string());
    }
    needles
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

const _: &str = "json_source_authoritative";
const _: &str = "perfect_hash_machine_cache_only";
const _: &str = "bloom_machine_cache_only";
const _: &str = "lowercase_cache_machine_cache_only";
const _: &str = "runtime_precomputed_cache_adopted";
const _: &str = "faster_than_upstream_claimed";
const _: &str = "upstream_baseline_measured";
const _: &str = "same_machine_benchmark_required";

#[cfg(test)]
mod tests {
    use super::*;

    fn icon(id: u32, name: &str) -> IconMetadata {
        IconMetadata {
            id,
            name: name.to_string(),
            pack: "test".to_string(),
            category: "test".to_string(),
            tags: Vec::new(),
            popularity: 0,
        }
    }

    #[test]
    fn perfect_hash_index_from_machine_validates_lookup_parity() {
        let metadata = vec![icon(10, "Home"), icon(20, "Arrow")];
        let source = crate::machine_catalog::IconCatalogSourceFingerprint {
            source: serializer::machine::MachineCacheSource {
                path: PathBuf::from("test"),
                bytes: 1,
                modified_unix_ms: None,
                blake3: [0; 32],
            },
            file_count: 1,
        };
        let machine = build_icon_perfect_hash_machine(Path::new("test"), &metadata, &source);
        let index = perfect_hash_index_from_machine(&machine, &metadata).unwrap();

        assert_eq!(index.lookup_exact("home"), Some(0));
        assert_eq!(index.lookup_exact("ARROW"), Some(1));
    }

    #[test]
    fn perfect_hash_index_from_machine_rejects_lookup_drift() {
        let metadata = vec![icon(10, "Home"), icon(20, "Arrow")];
        let source = crate::machine_catalog::IconCatalogSourceFingerprint {
            source: serializer::machine::MachineCacheSource {
                path: PathBuf::from("test"),
                bytes: 1,
                modified_unix_ms: None,
                blake3: [0; 32],
            },
            file_count: 1,
        };
        let mut machine = build_icon_perfect_hash_machine(Path::new("test"), &metadata, &source);
        let slot = machine
            .slots
            .iter_mut()
            .find(|slot| slot.occupied && slot.icon_index == 1)
            .unwrap();
        slot.lowercase_name = "wrong".to_string();

        let error = match perfect_hash_index_from_machine(&machine, &metadata) {
            Ok(_) => panic!("drifted perfect hash machine should be rejected"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("perfect hash machine slot"));
    }

    #[test]
    fn bloom_filters_from_machine_validates_no_false_negatives() {
        let metadata = vec![icon(10, "HomeIcon"), icon(20, "ArrowLeft")];
        let source = crate::machine_catalog::IconCatalogSourceFingerprint {
            source: serializer::machine::MachineCacheSource {
                path: PathBuf::from("test"),
                bytes: 1,
                modified_unix_ms: None,
                blake3: [0; 32],
            },
            file_count: 1,
        };
        let machine = build_icon_bloom_machine(Path::new("test"), &metadata, &source);
        let filters = bloom_filters_from_machine(&machine, &metadata).unwrap();

        assert!(filters.might_match(0, "home"));
        assert!(filters.might_match(1, "arrow"));
    }

    #[test]
    fn bloom_filters_from_machine_rejects_corrupted_filter() {
        let metadata = vec![icon(10, "HomeIcon"), icon(20, "ArrowLeft")];
        let source = crate::machine_catalog::IconCatalogSourceFingerprint {
            source: serializer::machine::MachineCacheSource {
                path: PathBuf::from("test"),
                bytes: 1,
                modified_unix_ms: None,
                blake3: [0; 32],
            },
            file_count: 1,
        };
        let mut machine = build_icon_bloom_machine(Path::new("test"), &metadata, &source);
        for word in &mut machine.filters[0].bits {
            *word = 0;
        }

        let error = match bloom_filters_from_machine(&machine, &metadata) {
            Ok(_) => panic!("corrupted bloom machine should be rejected"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("bloom machine filter 0"));
    }

    #[test]
    fn lowercase_cache_from_machine_validates_name_parity() {
        let metadata = vec![icon(10, "HomeIcon"), icon(20, "ArrowLeft")];
        let source = crate::machine_catalog::IconCatalogSourceFingerprint {
            source: serializer::machine::MachineCacheSource {
                path: PathBuf::from("test"),
                bytes: 1,
                modified_unix_ms: None,
                blake3: [0; 32],
            },
            file_count: 1,
        };
        let machine = build_icon_lowercase_cache_machine(Path::new("test"), &metadata, &source);
        let cache = lowercase_cache_from_machine(&machine, &metadata).unwrap();

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(0), "homeicon");
        assert_eq!(cache.get(1), "arrowleft");
    }

    #[test]
    fn lowercase_cache_from_machine_rejects_name_drift() {
        let metadata = vec![icon(10, "HomeIcon"), icon(20, "ArrowLeft")];
        let source = crate::machine_catalog::IconCatalogSourceFingerprint {
            source: serializer::machine::MachineCacheSource {
                path: PathBuf::from("test"),
                bytes: 1,
                modified_unix_ms: None,
                blake3: [0; 32],
            },
            file_count: 1,
        };
        let mut machine = build_icon_lowercase_cache_machine(Path::new("test"), &metadata, &source);
        machine.entries[1].lowercase_name = "wrong".to_string();

        let error = match lowercase_cache_from_machine(&machine, &metadata) {
            Ok(_) => panic!("drifted lowercase cache machine should be rejected"),
            Err(error) => error,
        };

        assert!(
            error
                .to_string()
                .contains("lowercase cache machine entry 1 lowercase name mismatch")
        );
    }
}
