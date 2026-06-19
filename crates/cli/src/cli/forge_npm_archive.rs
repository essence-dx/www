use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, bail};
use flate2::read::GzDecoder;

const MAX_NPM_EXTRACTED_FILES: usize = 5_000;
const MAX_NPM_EXTRACTED_BYTES: u64 = 250 * 1024 * 1024;

#[derive(Debug, Clone, Copy)]
pub(super) struct NpmArchiveExtraction {
    pub(super) files: usize,
    pub(super) bytes: u64,
}

pub(super) fn extract_npm_tgz(
    bytes: &[u8],
    destination: &Path,
) -> anyhow::Result<NpmArchiveExtraction> {
    let mut decoder = GzDecoder::new(bytes);
    let mut tar = Vec::new();
    decoder
        .read_to_end(&mut tar)
        .context("decompress npm tarball")?;
    let mut offset = 0usize;
    let mut files = 0usize;
    let mut extracted_bytes = 0u64;

    while offset + 512 <= tar.len() {
        let header = &tar[offset..offset + 512];
        offset += 512;
        if header.iter().all(|byte| *byte == 0) {
            break;
        }
        let path = tar_header_string(&header[0..100], &header[345..500])?;
        let size = parse_tar_octal(&header[124..136])?;
        let typeflag = header[156];
        let data_start = offset;
        let data_end = data_start
            .checked_add(usize::try_from(size).context("tar entry size too large")?)
            .context("tar entry size overflow")?;
        if data_end > tar.len() {
            bail!("npm tarball entry `{path}` extends beyond archive");
        }
        offset = data_end + ((512 - (data_end % 512)) % 512);

        if matches!(typeflag, b'0' | 0) {
            files += 1;
            extracted_bytes = extracted_bytes
                .checked_add(size)
                .context("extracted byte count overflow")?;
            if files > MAX_NPM_EXTRACTED_FILES || extracted_bytes > MAX_NPM_EXTRACTED_BYTES {
                bail!("npm tarball exceeds Forge extraction limits");
            }
            let relative = safe_npm_tar_entry_path(&path)?;
            let target = destination.join(relative);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("create `{}`", parent.display()))?;
            }
            fs::write(&target, &tar[data_start..data_end])
                .with_context(|| format!("write `{}`", target.display()))?;
        } else if typeflag == b'5' {
            let relative = safe_npm_tar_entry_path(&path)?;
            fs::create_dir_all(destination.join(relative))
                .with_context(|| format!("create directory from npm tar entry `{path}`"))?;
        }
    }

    Ok(NpmArchiveExtraction {
        files,
        bytes: extracted_bytes,
    })
}

fn tar_header_string(name: &[u8], prefix: &[u8]) -> anyhow::Result<String> {
    let name = nul_trimmed_string(name)?;
    let prefix = nul_trimmed_string(prefix)?;
    if prefix.is_empty() {
        Ok(name)
    } else {
        Ok(format!("{prefix}/{name}"))
    }
}

fn nul_trimmed_string(bytes: &[u8]) -> anyhow::Result<String> {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    Ok(std::str::from_utf8(&bytes[..end])
        .context("tar header path is not utf-8")?
        .trim()
        .to_string())
}

fn parse_tar_octal(bytes: &[u8]) -> anyhow::Result<u64> {
    let text = std::str::from_utf8(bytes)
        .context("tar octal field is not utf-8")?
        .trim_matches(char::from(0))
        .trim();
    if text.is_empty() {
        return Ok(0);
    }
    u64::from_str_radix(text, 8).context("parse tar octal size")
}

fn safe_npm_tar_entry_path(path: &str) -> anyhow::Result<PathBuf> {
    if path.starts_with('/') || path.contains('\\') || path.contains('\0') {
        bail!("unsafe npm tar entry path `{path}`");
    }
    let Some(path) = path.strip_prefix("package/") else {
        bail!("npm tarball entry `{path}` is outside npm package root");
    };
    if path.is_empty()
        || path.split('/').any(|part| part == "." || part == "..")
        || path
            .split('/')
            .any(|part| matches!(part, "node_modules" | ".git" | ".hg" | ".svn"))
    {
        bail!("unsafe npm tar entry path `{path}`");
    }
    let relative = PathBuf::from(path);
    if relative.is_absolute() {
        bail!("unsafe npm tar entry path `{path}`");
    }
    for component in relative.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => {
                bail!("unsafe npm tar entry path `{path}`");
            }
        }
    }
    Ok(relative)
}
