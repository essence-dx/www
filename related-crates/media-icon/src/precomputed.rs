use crate::bloom::IconBloomFilters;
use crate::machine_catalog::IconPrefixMachineV1;
use crate::perfect_hash::{LowercaseCache, PerfectHashIndex, ValidatedLowercaseCache};
/// Pre-computed indices for instant lookups
/// All expensive computations done once at startup
use crate::types::IconMetadata;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

/// Startup timing breakdown for the pre-computed search structures.
#[derive(Debug, Clone, Copy, Default)]
pub struct PrecomputedBuildTimings {
    pub perfect_hash_build_ns: u64,
    pub lowercase_cache_build_ns: u64,
    pub lowercase_names_build_ns: u64,
    pub bloom_filters_build_ns: u64,
    pub prefix_index_build_ns: u64,
    pub precomputed_total_build_ns: u64,
    pub lowercase_names_from_machine_cache: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixMachineAdoptionSummary {
    pub runtime_prefix_machine_available: bool,
    pub runtime_prefix_machine_adopted: bool,
    pub prefix_machine_id_to_position_validated: bool,
    pub prefix_machine_fallback_reason: Option<String>,
    pub prefix_machine_prefix_count: usize,
    pub prefix_machine_candidate_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerfectHashMachineAdoptionSummary {
    pub runtime_perfect_hash_machine_available: bool,
    pub runtime_perfect_hash_machine_adopted: bool,
    pub perfect_hash_machine_lookup_validated: bool,
    pub perfect_hash_machine_fallback_reason: Option<String>,
    pub perfect_hash_source: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BloomMachineAdoptionSummary {
    pub runtime_bloom_machine_available: bool,
    pub runtime_bloom_machine_adopted: bool,
    pub bloom_machine_no_false_negatives_validated: bool,
    pub bloom_machine_fallback_reason: Option<String>,
    pub bloom_source: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowercaseCacheMachineAdoptionSummary {
    pub runtime_lowercase_cache_machine_available: bool,
    pub runtime_lowercase_cache_machine_adopted: bool,
    pub lowercase_cache_machine_names_validated: bool,
    pub lowercase_cache_machine_fallback_reason: Option<String>,
    pub lowercase_cache_source: &'static str,
}

impl PerfectHashMachineAdoptionSummary {
    pub(crate) fn unavailable() -> Self {
        Self {
            runtime_perfect_hash_machine_available: false,
            runtime_perfect_hash_machine_adopted: false,
            perfect_hash_machine_lookup_validated: false,
            perfect_hash_machine_fallback_reason: Some(
                "perfect_hash_machine_not_available".to_string(),
            ),
            perfect_hash_source: "lowercase_names_rebuild",
        }
    }

    pub(crate) fn adopted() -> Self {
        Self {
            runtime_perfect_hash_machine_available: true,
            runtime_perfect_hash_machine_adopted: true,
            perfect_hash_machine_lookup_validated: true,
            perfect_hash_machine_fallback_reason: None,
            perfect_hash_source: "perfect_hash_machine",
        }
    }

    pub(crate) fn fallback(perfect_hash_machine_fallback_reason: String) -> Self {
        Self {
            runtime_perfect_hash_machine_available: true,
            runtime_perfect_hash_machine_adopted: false,
            perfect_hash_machine_lookup_validated: false,
            perfect_hash_machine_fallback_reason: Some(perfect_hash_machine_fallback_reason),
            perfect_hash_source: "lowercase_names_rebuild",
        }
    }
}

impl BloomMachineAdoptionSummary {
    pub(crate) fn unavailable() -> Self {
        Self {
            runtime_bloom_machine_available: false,
            runtime_bloom_machine_adopted: false,
            bloom_machine_no_false_negatives_validated: false,
            bloom_machine_fallback_reason: Some("bloom_machine_not_available".to_string()),
            bloom_source: "lowercase_names_rebuild",
        }
    }

    pub(crate) fn adopted() -> Self {
        Self {
            runtime_bloom_machine_available: true,
            runtime_bloom_machine_adopted: true,
            bloom_machine_no_false_negatives_validated: true,
            bloom_machine_fallback_reason: None,
            bloom_source: "bloom_machine",
        }
    }

    pub(crate) fn fallback(bloom_machine_fallback_reason: String) -> Self {
        Self {
            runtime_bloom_machine_available: true,
            runtime_bloom_machine_adopted: false,
            bloom_machine_no_false_negatives_validated: false,
            bloom_machine_fallback_reason: Some(bloom_machine_fallback_reason),
            bloom_source: "lowercase_names_rebuild",
        }
    }
}

impl LowercaseCacheMachineAdoptionSummary {
    pub(crate) fn unavailable() -> Self {
        Self {
            runtime_lowercase_cache_machine_available: false,
            runtime_lowercase_cache_machine_adopted: false,
            lowercase_cache_machine_names_validated: false,
            lowercase_cache_machine_fallback_reason: Some(
                "lowercase_cache_machine_not_available".to_string(),
            ),
            lowercase_cache_source: "lowercase_names_rebuild",
        }
    }

    pub(crate) fn adopted() -> Self {
        Self {
            runtime_lowercase_cache_machine_available: true,
            runtime_lowercase_cache_machine_adopted: true,
            lowercase_cache_machine_names_validated: true,
            lowercase_cache_machine_fallback_reason: None,
            lowercase_cache_source: "lowercase_cache_machine",
        }
    }

    pub(crate) fn fallback(lowercase_cache_machine_fallback_reason: String) -> Self {
        Self {
            runtime_lowercase_cache_machine_available: true,
            runtime_lowercase_cache_machine_adopted: false,
            lowercase_cache_machine_names_validated: false,
            lowercase_cache_machine_fallback_reason: Some(lowercase_cache_machine_fallback_reason),
            lowercase_cache_source: "lowercase_names_rebuild",
        }
    }
}

impl PrefixMachineAdoptionSummary {
    fn unavailable() -> Self {
        Self {
            runtime_prefix_machine_available: false,
            runtime_prefix_machine_adopted: false,
            prefix_machine_id_to_position_validated: false,
            prefix_machine_fallback_reason: Some("prefix_machine_not_available".to_string()),
            prefix_machine_prefix_count: 0,
            prefix_machine_candidate_count: 0,
        }
    }

    fn adopted(prefix_machine: &IconPrefixMachineV1) -> Self {
        Self {
            runtime_prefix_machine_available: true,
            runtime_prefix_machine_adopted: true,
            prefix_machine_id_to_position_validated: true,
            prefix_machine_fallback_reason: None,
            prefix_machine_prefix_count: prefix_machine.entries.len(),
            prefix_machine_candidate_count: prefix_machine.icon_ids.len(),
        }
    }

    fn fallback(prefix_machine: &IconPrefixMachineV1, error: PrefixMachineAdoptionError) -> Self {
        Self {
            runtime_prefix_machine_available: true,
            runtime_prefix_machine_adopted: false,
            prefix_machine_id_to_position_validated: false,
            prefix_machine_fallback_reason: Some(error.to_string()),
            prefix_machine_prefix_count: prefix_machine.entries.len(),
            prefix_machine_candidate_count: prefix_machine.icon_ids.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixMachineAdoptionError {
    IconCountMismatch {
        machine_count: usize,
        metadata_count: usize,
    },
    MaxPrefixLenMismatch {
        max_prefix_len: u8,
    },
    DuplicateMetadataIconId(u32),
    PrefixWindowOutOfBounds {
        prefix: String,
        start: u32,
        len: u32,
        icon_ids_len: usize,
    },
    EmptyPrefix,
    PrefixTooLong {
        prefix: String,
        max_prefix_len: u8,
    },
    MissingIconId(u32),
    NonCanonicalPrefixRange {
        prefix: String,
        start: u32,
        expected_start: u32,
    },
    PrefixOutOfOrder {
        previous: String,
        current: String,
    },
    DuplicateMachinePrefix(String),
    PrefixNotLowercase(String),
    PrefixMapMismatch,
}

impl fmt::Display for PrefixMachineAdoptionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IconCountMismatch {
                machine_count,
                metadata_count,
            } => write!(
                formatter,
                "prefix machine icon count {machine_count} does not match metadata count {metadata_count}"
            ),
            Self::MaxPrefixLenMismatch { max_prefix_len } => {
                write!(
                    formatter,
                    "prefix machine max_prefix_len {max_prefix_len} is not 3"
                )
            }
            Self::DuplicateMetadataIconId(id) => {
                write!(formatter, "duplicate metadata icon id {id}")
            }
            Self::PrefixWindowOutOfBounds {
                prefix,
                start,
                len,
                icon_ids_len,
            } => write!(
                formatter,
                "prefix machine entry {prefix:?} range {start}+{len} exceeds icon_ids length {icon_ids_len}"
            ),
            Self::EmptyPrefix => write!(formatter, "prefix machine contains an empty prefix"),
            Self::PrefixTooLong {
                prefix,
                max_prefix_len,
            } => write!(
                formatter,
                "prefix machine entry {prefix:?} exceeds max_prefix_len {max_prefix_len}"
            ),
            Self::MissingIconId(id) => {
                write!(formatter, "prefix machine referenced missing icon id {id}")
            }
            Self::NonCanonicalPrefixRange {
                prefix,
                start,
                expected_start,
            } => write!(
                formatter,
                "prefix machine entry {prefix:?} starts at {start}, expected {expected_start}"
            ),
            Self::PrefixOutOfOrder { previous, current } => write!(
                formatter,
                "prefix machine entry {current:?} is not strictly after {previous:?}"
            ),
            Self::DuplicateMachinePrefix(prefix) => {
                write!(formatter, "duplicate prefix machine entry {prefix:?}")
            }
            Self::PrefixNotLowercase(prefix) => {
                write!(
                    formatter,
                    "prefix machine entry {prefix:?} is not lowercase"
                )
            }
            Self::PrefixMapMismatch => write!(
                formatter,
                "translated prefix machine candidates do not match metadata-derived prefix candidates"
            ),
        }
    }
}

impl std::error::Error for PrefixMachineAdoptionError {}

/// Pre-computed search index (built once, used forever)
pub struct PrecomputedIndex {
    /// Perfect hash for O(1) exact matches
    pub perfect_hash: PerfectHashIndex,

    /// Pre-computed lowercase names (zero allocation)
    pub lowercase_cache: LowercaseCache,

    /// Bloom filters for fast rejection
    pub bloom_filters: IconBloomFilters,

    /// Prefix index for fast prefix matching
    pub prefix_index: PrefixIndex,

    /// Metadata reference
    pub metadata: Vec<IconMetadata>,
}

enum LowercaseNamesSource<'a> {
    Borrowed(&'a [String]),
    Owned(Vec<String>),
}

impl<'a> LowercaseNamesSource<'a> {
    fn as_slice(&self) -> &[String] {
        match self {
            Self::Borrowed(lowercase_names) => lowercase_names,
            Self::Owned(lowercase_names) => lowercase_names,
        }
    }

    fn is_from_machine_cache(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }
}

impl PrecomputedIndex {
    /// Build all indices (done once at startup)
    pub fn build(metadata: Vec<IconMetadata>) -> Self {
        Self::build_with_optional_prefix_machine(metadata, None).0
    }

    pub fn build_with_optional_prefix_machine(
        metadata: Vec<IconMetadata>,
        prefix_machine: Option<&IconPrefixMachineV1>,
    ) -> (Self, PrefixMachineAdoptionSummary) {
        let (
            index,
            _timings,
            prefix_adoption,
            _perfect_hash_adoption,
            _bloom_adoption,
            _lowercase_cache_adoption,
        ) = Self::build_with_timings_and_optional_precomputed_machines(
            metadata,
            prefix_machine,
            None,
            None,
            None,
        );
        (index, prefix_adoption)
    }

    /// Build all indices and return a startup timing breakdown.
    pub fn build_with_timings(metadata: Vec<IconMetadata>) -> (Self, PrecomputedBuildTimings) {
        let (index, timings, _prefix_adoption) =
            Self::build_with_timings_and_optional_prefix_machine(metadata, None);
        (index, timings)
    }

    pub fn build_with_timings_and_optional_prefix_machine(
        metadata: Vec<IconMetadata>,
        prefix_machine: Option<&IconPrefixMachineV1>,
    ) -> (Self, PrecomputedBuildTimings, PrefixMachineAdoptionSummary) {
        let (
            index,
            timings,
            prefix_adoption,
            _perfect_hash_adoption,
            _bloom_adoption,
            _lowercase_cache_adoption,
        ) = Self::build_with_timings_and_optional_precomputed_machines(
            metadata,
            prefix_machine,
            None,
            None,
            None,
        );
        (index, timings, prefix_adoption)
    }

    pub fn build_with_timings_and_optional_precomputed_machines(
        metadata: Vec<IconMetadata>,
        prefix_machine: Option<&IconPrefixMachineV1>,
        perfect_hash_machine: Option<PerfectHashIndex>,
        bloom_machine: Option<IconBloomFilters>,
        lowercase_cache_machine: Option<ValidatedLowercaseCache>,
    ) -> (
        Self,
        PrecomputedBuildTimings,
        PrefixMachineAdoptionSummary,
        PerfectHashMachineAdoptionSummary,
        BloomMachineAdoptionSummary,
        LowercaseCacheMachineAdoptionSummary,
    ) {
        let total_started = Instant::now();

        let mut adopted_lowercase_cache = None;
        let lowercase_cache_adoption = match lowercase_cache_machine {
            Some(lowercase_cache) if lowercase_cache.len() == metadata.len() => {
                adopted_lowercase_cache = Some(lowercase_cache);
                LowercaseCacheMachineAdoptionSummary::adopted()
            }
            Some(lowercase_cache) => {
                let fallback_reason = format!(
                    "lowercase cache length {} does not match metadata count {}",
                    lowercase_cache.len(),
                    metadata.len()
                );
                LowercaseCacheMachineAdoptionSummary::fallback(fallback_reason)
            }
            None => LowercaseCacheMachineAdoptionSummary::unavailable(),
        };

        let lowercase_names_started = Instant::now();
        let lowercase_names = match adopted_lowercase_cache.as_ref() {
            Some(lowercase_cache) => LowercaseNamesSource::Borrowed(lowercase_cache.as_slice()),
            None => LowercaseNamesSource::Owned(
                metadata.iter().map(|m| m.name.to_lowercase()).collect(),
            ),
        };
        let lowercase_names_from_machine_cache = lowercase_names.is_from_machine_cache();
        let lowercase_names_build_ns = duration_nanos_u64(lowercase_names_started.elapsed());

        let perfect_hash_started = Instant::now();
        let (perfect_hash, perfect_hash_adoption) = match perfect_hash_machine {
            Some(perfect_hash) => (perfect_hash, PerfectHashMachineAdoptionSummary::adopted()),
            None => (
                PerfectHashIndex::build_from_lowercase_names(lowercase_names.as_slice()),
                PerfectHashMachineAdoptionSummary::unavailable(),
            ),
        };
        let perfect_hash_build_ns = duration_nanos_u64(perfect_hash_started.elapsed());

        let bloom_filters_started = Instant::now();
        let (bloom_filters, bloom_adoption) = match bloom_machine {
            Some(bloom_filters) => (bloom_filters, BloomMachineAdoptionSummary::adopted()),
            None => (
                IconBloomFilters::build(lowercase_names.as_slice()),
                BloomMachineAdoptionSummary::unavailable(),
            ),
        };
        let bloom_filters_build_ns = duration_nanos_u64(bloom_filters_started.elapsed());

        let prefix_index_started = Instant::now();
        let (prefix_index, prefix_adoption) = match prefix_machine {
            Some(prefix_machine) => {
                match PrefixIndex::from_icon_id_prefix_machine_with_lowercase_names(
                    prefix_machine,
                    &metadata,
                    lowercase_names.as_slice(),
                ) {
                    Ok(prefix_index) => (
                        prefix_index,
                        PrefixMachineAdoptionSummary::adopted(prefix_machine),
                    ),
                    Err(error) => (
                        PrefixIndex::build(lowercase_names.as_slice()),
                        PrefixMachineAdoptionSummary::fallback(prefix_machine, error),
                    ),
                }
            }
            None => (
                PrefixIndex::build(lowercase_names.as_slice()),
                PrefixMachineAdoptionSummary::unavailable(),
            ),
        };
        let prefix_index_build_ns = duration_nanos_u64(prefix_index_started.elapsed());

        let lowercase_cache_started = Instant::now();
        let lowercase_cache = match lowercase_names {
            LowercaseNamesSource::Borrowed(_) => adopted_lowercase_cache
                .expect("borrowed lowercase names require adopted lowercase cache")
                .into_inner(),
            LowercaseNamesSource::Owned(lowercase_names) => {
                LowercaseCache::from_lowercase_names(lowercase_names)
            }
        };
        let lowercase_cache_build_ns = duration_nanos_u64(lowercase_cache_started.elapsed());

        let timings = PrecomputedBuildTimings {
            perfect_hash_build_ns,
            lowercase_cache_build_ns,
            lowercase_names_build_ns,
            bloom_filters_build_ns,
            prefix_index_build_ns,
            precomputed_total_build_ns: duration_nanos_u64(total_started.elapsed()),
            lowercase_names_from_machine_cache,
        };

        (
            Self {
                perfect_hash,
                lowercase_cache,
                bloom_filters,
                prefix_index,
                metadata,
            },
            timings,
            prefix_adoption,
            perfect_hash_adoption,
            bloom_adoption,
            lowercase_cache_adoption,
        )
    }
}

fn duration_nanos_u64(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

/// Prefix trie for fast prefix matching
#[derive(Debug)]
pub struct PrefixIndex {
    /// Map of prefix -> list of icon indices
    /// Pre-computed for all 1-3 char prefixes
    prefix_map: HashMap<String, Vec<u32>>,
}

impl PrefixIndex {
    /// Build prefix index
    pub fn build(lowercase_names: &[String]) -> Self {
        let prefix_map = build_prefix_map(lowercase_names);

        Self { prefix_map }
    }

    pub fn from_icon_id_prefix_machine(
        prefix_machine: &IconPrefixMachineV1,
        metadata: &[IconMetadata],
    ) -> Result<Self, PrefixMachineAdoptionError> {
        let lowercase_names: Vec<String> = metadata
            .iter()
            .map(|icon| icon.name.to_lowercase())
            .collect();
        Self::from_icon_id_prefix_machine_with_lowercase_names(
            prefix_machine,
            metadata,
            &lowercase_names,
        )
    }

    pub fn from_icon_id_prefix_machine_with_lowercase_names(
        prefix_machine: &IconPrefixMachineV1,
        metadata: &[IconMetadata],
        lowercase_names: &[String],
    ) -> Result<Self, PrefixMachineAdoptionError> {
        if prefix_machine.icon_count as usize != metadata.len() {
            return Err(PrefixMachineAdoptionError::IconCountMismatch {
                machine_count: prefix_machine.icon_count as usize,
                metadata_count: metadata.len(),
            });
        }
        if lowercase_names.len() != metadata.len() {
            return Err(PrefixMachineAdoptionError::PrefixMapMismatch);
        }

        if prefix_machine.max_prefix_len != 3 {
            return Err(PrefixMachineAdoptionError::MaxPrefixLenMismatch {
                max_prefix_len: prefix_machine.max_prefix_len,
            });
        }

        let mut id_to_position: HashMap<u32, u32> = HashMap::with_capacity(metadata.len());
        for (position, icon) in metadata.iter().enumerate() {
            if id_to_position.insert(icon.id, position as u32).is_some() {
                return Err(PrefixMachineAdoptionError::DuplicateMetadataIconId(icon.id));
            }
        }

        let expected_prefix_map = build_prefix_map(lowercase_names);
        let mut prefix_map: HashMap<String, Vec<u32>> =
            HashMap::with_capacity(prefix_machine.entries.len());
        let mut expected_start = 0u32;
        let mut previous_prefix: Option<&str> = None;
        for entry in &prefix_machine.entries {
            if entry.prefix.is_empty() {
                return Err(PrefixMachineAdoptionError::EmptyPrefix);
            }
            if entry.prefix != entry.prefix.to_lowercase() {
                return Err(PrefixMachineAdoptionError::PrefixNotLowercase(
                    entry.prefix.clone(),
                ));
            }
            if entry.prefix.len() > prefix_machine.max_prefix_len as usize {
                return Err(PrefixMachineAdoptionError::PrefixTooLong {
                    prefix: entry.prefix.clone(),
                    max_prefix_len: prefix_machine.max_prefix_len,
                });
            }
            if let Some(previous) = previous_prefix
                && entry.prefix.as_str() <= previous
            {
                return Err(PrefixMachineAdoptionError::PrefixOutOfOrder {
                    previous: previous.to_string(),
                    current: entry.prefix.clone(),
                });
            }
            previous_prefix = Some(entry.prefix.as_str());
            if entry.start != expected_start {
                return Err(PrefixMachineAdoptionError::NonCanonicalPrefixRange {
                    prefix: entry.prefix.clone(),
                    start: entry.start,
                    expected_start,
                });
            }

            let end = entry.start.checked_add(entry.len).ok_or_else(|| {
                PrefixMachineAdoptionError::PrefixWindowOutOfBounds {
                    prefix: entry.prefix.clone(),
                    start: entry.start,
                    len: entry.len,
                    icon_ids_len: prefix_machine.icon_ids.len(),
                }
            })?;
            let start = entry.start as usize;
            let end = end as usize;
            if end > prefix_machine.icon_ids.len() {
                return Err(PrefixMachineAdoptionError::PrefixWindowOutOfBounds {
                    prefix: entry.prefix.clone(),
                    start: entry.start,
                    len: entry.len,
                    icon_ids_len: prefix_machine.icon_ids.len(),
                });
            }
            expected_start = end as u32;

            let mut positions = Vec::with_capacity(end.saturating_sub(start));
            for offset in start..end {
                let icon_id = prefix_machine.icon_ids[offset];
                let position = id_to_position
                    .get(&icon_id)
                    .copied()
                    .ok_or(PrefixMachineAdoptionError::MissingIconId(icon_id))?;
                positions.push(position);
            }
            if prefix_map.insert(entry.prefix.clone(), positions).is_some() {
                return Err(PrefixMachineAdoptionError::DuplicateMachinePrefix(
                    entry.prefix.clone(),
                ));
            }
        }
        if expected_start as usize != prefix_machine.icon_ids.len() {
            return Err(PrefixMachineAdoptionError::PrefixWindowOutOfBounds {
                prefix: "<end>".to_string(),
                start: expected_start,
                len: 0,
                icon_ids_len: prefix_machine.icon_ids.len(),
            });
        }
        if prefix_map != expected_prefix_map {
            return Err(PrefixMachineAdoptionError::PrefixMapMismatch);
        }

        Ok(Self { prefix_map })
    }

    /// Get candidates for prefix (O(1) lookup)
    #[inline(always)]
    pub fn get_candidates(&self, prefix: &str) -> Option<&[u32]> {
        if prefix.len() <= 3 {
            self.prefix_map.get(prefix).map(|v| v.as_slice())
        } else {
            // For longer prefixes, use first 3 chars
            self.prefix_map.get(&prefix[..3]).map(|v| v.as_slice())
        }
    }
}

fn build_prefix_map(lowercase_names: &[String]) -> HashMap<String, Vec<u32>> {
    let mut prefix_map: HashMap<String, Vec<u32>> = HashMap::new();

    for (idx, name) in lowercase_names.iter().enumerate() {
        for len in 1..=3.min(name.len()) {
            let prefix = &name[..len];
            prefix_map
                .entry(prefix.to_string())
                .or_default()
                .push(idx as u32);
        }
    }

    prefix_map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_icon(id: u32, name: &str) -> IconMetadata {
        IconMetadata {
            id,
            name: name.to_string(),
            pack: "test".to_string(),
            category: String::new(),
            tags: Vec::new(),
            popularity: 0,
        }
    }

    fn matching_home_prefix_machine() -> IconPrefixMachineV1 {
        IconPrefixMachineV1 {
            selected_data_root: "test".to_string(),
            generated_at_unix_ms: 0,
            source_file_count: 1,
            source_total_bytes: 1,
            source_blake3: [0; 32],
            icon_count: 2,
            max_prefix_len: 3,
            entries: vec![
                crate::machine_catalog::IconPrefixEntryV1 {
                    prefix: "h".to_string(),
                    start: 0,
                    len: 2,
                },
                crate::machine_catalog::IconPrefixEntryV1 {
                    prefix: "ho".to_string(),
                    start: 2,
                    len: 2,
                },
                crate::machine_catalog::IconPrefixEntryV1 {
                    prefix: "hom".to_string(),
                    start: 4,
                    len: 1,
                },
                crate::machine_catalog::IconPrefixEntryV1 {
                    prefix: "hou".to_string(),
                    start: 5,
                    len: 1,
                },
            ],
            icon_ids: vec![100, 200, 100, 200, 100, 200],
        }
    }

    #[test]
    fn test_prefix_index() {
        let names = vec!["home".to_string(), "house".to_string(), "arrow".to_string()];

        let index = PrefixIndex::build(&names);

        let candidates = index.get_candidates("ho").unwrap();
        assert_eq!(candidates.len(), 2); // home, house

        let candidates = index.get_candidates("ar").unwrap();
        assert_eq!(candidates.len(), 1); // arrow
    }

    #[test]
    fn test_prefix_index_from_machine_converts_icon_ids_to_metadata_positions() {
        let metadata = vec![test_icon(100, "home"), test_icon(200, "house")];
        let machine = matching_home_prefix_machine();

        let index = PrefixIndex::from_icon_id_prefix_machine(&machine, &metadata).unwrap();

        assert_eq!(index.get_candidates("ho").unwrap(), &[0, 1]);
        assert_eq!(index.get_candidates("hom").unwrap(), &[0]);
        assert_eq!(index.get_candidates("hou").unwrap(), &[1]);
    }

    #[test]
    fn test_prefix_index_from_machine_accepts_validated_lowercase_names() {
        let metadata = vec![test_icon(100, "Home"), test_icon(200, "House")];
        let lowercase_names = vec!["home".to_string(), "house".to_string()];
        let machine = matching_home_prefix_machine();

        let index = PrefixIndex::from_icon_id_prefix_machine_with_lowercase_names(
            &machine,
            &metadata,
            &lowercase_names,
        )
        .unwrap();

        assert_eq!(index.get_candidates("ho").unwrap(), &[0, 1]);
        assert_eq!(index.get_candidates("hom").unwrap(), &[0]);
        assert_eq!(index.get_candidates("hou").unwrap(), &[1]);
    }

    #[test]
    fn test_prefix_index_from_machine_rejects_missing_icon_id() {
        let metadata = vec![test_icon(100, "home"), test_icon(200, "house")];
        let mut machine = matching_home_prefix_machine();
        machine.icon_ids[1] = 404;

        let error = PrefixIndex::from_icon_id_prefix_machine(&machine, &metadata).unwrap_err();

        assert!(matches!(
            error,
            PrefixMachineAdoptionError::MissingIconId(404)
        ));
    }

    #[test]
    fn test_prefix_index_from_machine_rejects_metadata_mismatch() {
        let metadata = vec![test_icon(100, "home"), test_icon(200, "hotel")];
        let machine = matching_home_prefix_machine();

        let error = PrefixIndex::from_icon_id_prefix_machine(&machine, &metadata).unwrap_err();

        assert!(matches!(
            error,
            PrefixMachineAdoptionError::PrefixMapMismatch
        ));
    }

    #[test]
    fn build_with_optional_lowercase_cache_rejects_length_mismatch() {
        let metadata = vec![test_icon(100, "Home"), test_icon(200, "Arrow")];
        let lowercase_cache = LowercaseCache::from_lowercase_names(vec!["home".to_string()]);

        let (index, timings, _prefix, _perfect_hash, _bloom, lowercase_cache_adoption) =
            PrecomputedIndex::build_with_timings_and_optional_precomputed_machines(
                metadata,
                None,
                None,
                None,
                Some(ValidatedLowercaseCache::from_validated_cache(
                    lowercase_cache,
                )),
            );

        assert!(!lowercase_cache_adoption.runtime_lowercase_cache_machine_adopted);
        assert!(lowercase_cache_adoption.runtime_lowercase_cache_machine_available);
        assert!(!lowercase_cache_adoption.lowercase_cache_machine_names_validated);
        assert_eq!(
            lowercase_cache_adoption.lowercase_cache_source,
            "lowercase_names_rebuild"
        );
        assert!(!timings.lowercase_names_from_machine_cache);
        assert_eq!(
            lowercase_cache_adoption.lowercase_cache_machine_fallback_reason,
            Some("lowercase cache length 1 does not match metadata count 2".to_string())
        );
        assert_eq!(index.lowercase_cache.len(), 2);
        assert_eq!(index.perfect_hash.lookup_exact("home"), Some(0));
        assert_eq!(index.lowercase_cache.get(0), "home");
        assert_eq!(index.lowercase_cache.get(1), "arrow");
    }

    #[test]
    fn build_with_optional_lowercase_cache_adopts_valid_cache() {
        let metadata = vec![test_icon(100, "Home"), test_icon(200, "Arrow")];
        let lowercase_cache =
            LowercaseCache::from_lowercase_names(vec!["home".to_string(), "arrow".to_string()]);

        let (index, timings, _prefix, _perfect_hash, _bloom, lowercase_cache_adoption) =
            PrecomputedIndex::build_with_timings_and_optional_precomputed_machines(
                metadata,
                None,
                None,
                None,
                Some(ValidatedLowercaseCache::from_validated_cache(
                    lowercase_cache,
                )),
            );

        assert!(lowercase_cache_adoption.runtime_lowercase_cache_machine_adopted);
        assert_eq!(
            lowercase_cache_adoption.lowercase_cache_machine_fallback_reason,
            None
        );
        assert_eq!(
            lowercase_cache_adoption.lowercase_cache_source,
            "lowercase_cache_machine"
        );
        assert!(timings.lowercase_names_from_machine_cache);
        assert_eq!(index.lowercase_cache.len(), 2);
        assert_eq!(index.lowercase_cache.get(0), "home");
        assert_eq!(index.perfect_hash.lookup_exact("HOME"), Some(0));
        assert_eq!(index.prefix_index.get_candidates("ar").unwrap(), &[1]);
    }
}
