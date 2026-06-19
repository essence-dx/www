/// Minimal Perfect Hash Function (MPHF) for O(1) icon lookup
/// Pre-computed at build time for zero-cost runtime lookups
use crate::types::IconMetadata;

/// Perfect hash index for instant O(1) lookups
pub struct PerfectHashIndex {
    /// Pre-computed hash -> icon index mapping
    hash_table: Vec<Option<u32>>,
    /// Pre-computed hash -> lowercase icon name mapping
    name_table: Vec<Option<String>>,
    /// Hash function parameters (computed at build time)
    seed: u64,
    table_size: usize,
}

impl PerfectHashIndex {
    /// Build perfect hash index from icon names (done once at startup)
    pub fn build(metadata: &[IconMetadata]) -> Self {
        let lowercase_names: Vec<String> = metadata
            .iter()
            .map(|icon| icon.name.to_lowercase())
            .collect();
        Self::build_from_lowercase_names(&lowercase_names)
    }

    /// Build perfect hash index from already lowercased icon names.
    pub fn build_from_lowercase_names(lowercase_names: &[String]) -> Self {
        let mut table_size = lowercase_names.len().saturating_mul(2).max(1);

        loop {
            if let Some(seed) = Self::find_perfect_seed_for_names(lowercase_names, table_size) {
                let mut hash_table = vec![None; table_size];
                let mut name_table = vec![None; table_size];

                for (idx, name) in lowercase_names.iter().enumerate() {
                    let hash = Self::hash_name(name, seed, table_size);
                    hash_table[hash] = Some(idx as u32);
                    name_table[hash] = Some(name.clone());
                }

                return Self {
                    hash_table,
                    name_table,
                    seed,
                    table_size,
                };
            }

            table_size = table_size.saturating_mul(2);
        }
    }

    pub fn to_machine_parts(&self) -> (u64, usize, Vec<Option<u32>>, Vec<Option<String>>) {
        (
            self.seed,
            self.table_size,
            self.hash_table.clone(),
            self.name_table.clone(),
        )
    }

    pub fn from_machine_parts(
        seed: u64,
        table_size: usize,
        hash_table: Vec<Option<u32>>,
        name_table: Vec<Option<String>>,
    ) -> Result<Self, String> {
        if table_size == 0 {
            return Err("perfect hash machine table_size must be greater than zero".to_string());
        }
        if hash_table.len() != table_size {
            return Err(format!(
                "perfect hash machine hash_table length {} does not match table_size {}",
                hash_table.len(),
                table_size
            ));
        }
        if name_table.len() != table_size {
            return Err(format!(
                "perfect hash machine name_table length {} does not match table_size {}",
                name_table.len(),
                table_size
            ));
        }
        for (slot, (hash_entry, name_entry)) in hash_table.iter().zip(name_table.iter()).enumerate()
        {
            if hash_entry.is_some() != name_entry.is_some() {
                return Err(format!(
                    "perfect hash machine slot {slot} has mismatched hash/name occupancy"
                ));
            }
        }

        Ok(Self {
            hash_table,
            name_table,
            seed,
            table_size,
        })
    }

    /// Find a seed that produces no collisions
    fn find_perfect_seed_for_names(lowercase_names: &[String], table_size: usize) -> Option<u64> {
        for seed in 0..10000 {
            let mut used = vec![false; table_size];
            let mut collision = false;

            for name in lowercase_names {
                let hash = Self::hash_name(name, seed, table_size);
                if used[hash] {
                    collision = true;
                    break;
                }
                used[hash] = true;
            }

            if !collision {
                return Some(seed);
            }
        }

        None
    }

    /// Hash function (FNV-1a variant)
    #[inline(always)]
    fn hash_name(name: &str, seed: u64, table_size: usize) -> usize {
        let mut hash = seed.wrapping_add(0xcbf29ce484222325);
        for byte in name.as_bytes() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        (hash as usize) % table_size
    }

    /// O(1) exact match lookup
    #[inline(always)]
    pub fn lookup_exact(&self, query: &str) -> Option<u32> {
        let query = query.to_lowercase();
        let hash = Self::hash_name(&query, self.seed, self.table_size);
        match (self.hash_table.get(hash), self.name_table.get(hash)) {
            (Some(Some(idx)), Some(Some(name))) if name == &query => Some(*idx),
            _ => None,
        }
    }
}

/// Pre-computed lowercase name cache (eliminates runtime allocations)
pub struct LowercaseCache {
    /// Pre-computed lowercase names (computed once at startup)
    lowercase_names: Vec<String>,
}

pub struct ValidatedLowercaseCache {
    inner: LowercaseCache,
}

impl ValidatedLowercaseCache {
    pub(crate) fn from_validated_cache(inner: LowercaseCache) -> Self {
        Self { inner }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[String] {
        self.inner.as_slice()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> &str {
        self.inner.get(idx)
    }

    pub fn into_inner(self) -> LowercaseCache {
        self.inner
    }
}

impl LowercaseCache {
    /// Build cache from metadata (done once)
    pub fn build(metadata: &[IconMetadata]) -> Self {
        let lowercase_names = metadata
            .iter()
            .map(|icon| icon.name.to_lowercase())
            .collect();

        Self::from_lowercase_names(lowercase_names)
    }

    /// Build cache from already lowercased icon names.
    pub fn from_lowercase_names(lowercase_names: Vec<String>) -> Self {
        Self { lowercase_names }
    }

    /// Get pre-computed lowercase name (zero allocation)
    #[inline(always)]
    pub fn get(&self, idx: usize) -> &str {
        &self.lowercase_names[idx]
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.lowercase_names.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.lowercase_names.is_empty()
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[String] {
        &self.lowercase_names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_hash() {
        let icons = vec![
            IconMetadata {
                id: 0,
                name: "home".to_string(),
                pack: "test".to_string(),
                category: "test".to_string(),
                tags: vec![],
                popularity: 1,
            },
            IconMetadata {
                id: 1,
                name: "arrow".to_string(),
                pack: "test".to_string(),
                category: "test".to_string(),
                tags: vec![],
                popularity: 1,
            },
        ];

        let index = PerfectHashIndex::build(&icons);

        assert_eq!(index.lookup_exact("home"), Some(0));
        assert_eq!(index.lookup_exact("arrow"), Some(1));
        assert_eq!(index.lookup_exact("notfound"), None);
    }

    #[test]
    fn test_perfect_hash_from_lowercase_names_matches_metadata_build() {
        let icons = vec![
            IconMetadata {
                id: 0,
                name: "Home".to_string(),
                pack: "test".to_string(),
                category: "test".to_string(),
                tags: vec![],
                popularity: 1,
            },
            IconMetadata {
                id: 1,
                name: "Arrow".to_string(),
                pack: "test".to_string(),
                category: "test".to_string(),
                tags: vec![],
                popularity: 1,
            },
        ];
        let lowercase_names: Vec<String> =
            icons.iter().map(|icon| icon.name.to_lowercase()).collect();

        let metadata_index = PerfectHashIndex::build(&icons);
        let lowercase_index = PerfectHashIndex::build_from_lowercase_names(&lowercase_names);

        for index in [&metadata_index, &lowercase_index] {
            assert_eq!(index.lookup_exact("home"), Some(0));
            assert_eq!(index.lookup_exact("HOME"), Some(0));
            assert_eq!(index.lookup_exact("arrow"), Some(1));
            assert_eq!(index.lookup_exact("ARROW"), Some(1));
            assert_eq!(index.lookup_exact("notfound"), None);
        }
    }
}
