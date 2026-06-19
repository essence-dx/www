use crate::index::IconIndex;
use crate::machine_manifest::write_icon_manifest_machine_cache;
use crate::machine_readiness::ensure_icon_machine_caches_for_index_output;
use crate::parser::parse_icon_files;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Build icon search index from JSON data
pub struct IndexBuilder;

impl IndexBuilder {
    /// Build index from data directory
    pub fn build_from_dir(data_dir: &Path, output_dir: &Path) -> Result<()> {
        println!("Parsing icon files from {:?}...", data_dir);
        let icons = parse_icon_files(data_dir)?;
        println!("Parsed {} icons", icons.len());
        let _ = ensure_icon_machine_caches_for_index_output(data_dir, output_dir, &icons);

        println!("Building FST and rkyv index...");
        let index = IconIndex::build(icons)?;

        println!("Saving index to {:?}...", output_dir);
        std::fs::create_dir_all(output_dir)?;
        index.save_all(output_dir)?;

        println!("Index built successfully!");
        Ok(())
    }

    /// Build only the generated machine manifest for an icon data directory.
    pub fn build_manifest_from_dir(project_root: &Path, data_dir: &Path) -> Result<PathBuf> {
        write_icon_manifest_machine_cache(project_root, data_dir)
    }
}
