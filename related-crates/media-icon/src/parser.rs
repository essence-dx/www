use crate::types::{IconMetadata, IconPack};
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Parse all JSON icon files from data directory
pub fn parse_icon_files(data_dir: &Path) -> Result<Vec<IconMetadata>> {
    let mut all_icons = Vec::new();
    let mut icon_id = 0u32;

    for path in sorted_json_icon_files(data_dir)? {
        let content = fs::read_to_string(&path)?;
        let pack: IconPack = serde_json::from_str(&content)?;

        for icon_name in sorted_icon_names(&pack) {
            all_icons.push(IconMetadata {
                id: icon_id,
                name: icon_name.clone(),
                pack: pack.prefix.clone(),
                category: String::new(),
                tags: vec![],
                popularity: 0,
            });
            icon_id += 1;
        }
    }

    Ok(all_icons)
}

fn sorted_json_icon_files(data_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut json_files = fs::read_dir(data_dir)?
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    json_files.sort_by(|left, right| {
        left.file_name()
            .and_then(|value| value.to_str())
            .cmp(&right.file_name().and_then(|value| value.to_str()))
    });
    Ok(json_files)
}

fn sorted_icon_names(pack: &IconPack) -> Vec<&String> {
    let mut icon_names = pack.icons.keys().collect::<Vec<_>>();
    icon_names.sort();
    icon_names
}

/// Extract icon names for FST building
pub fn extract_icon_names(icons: &[IconMetadata]) -> Vec<(String, u32)> {
    icons
        .iter()
        .map(|icon| (icon.name.clone(), icon.id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parse_icon_files_uses_deterministic_file_and_icon_order() {
        let root = temp_icon_dir("deterministic-order");
        fs::create_dir_all(&root).unwrap();
        fs::write(
            root.join("z-pack.json"),
            pack_json("zpack", &[("zeta", "z"), ("alpha", "a")]),
        )
        .unwrap();
        fs::write(
            root.join("a-pack.json"),
            pack_json("apack", &[("delta", "d"), ("beta", "b")]),
        )
        .unwrap();
        fs::write(root.join("ignore.txt"), "ignored").unwrap();

        let icons = parse_icon_files(&root).unwrap();

        let ordered = icons
            .iter()
            .map(|icon| (icon.id, icon.pack.as_str(), icon.name.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(
            ordered,
            vec![
                (0, "apack", "beta"),
                (1, "apack", "delta"),
                (2, "zpack", "alpha"),
                (3, "zpack", "zeta"),
            ]
        );

        fs::remove_dir_all(root).unwrap();
    }

    fn temp_icon_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("dx-icons-{label}-{unique}"))
    }

    fn pack_json(prefix: &str, icons: &[(&str, &str)]) -> String {
        let icon_entries = icons
            .iter()
            .map(|(name, body)| format!(r#""{name}":{{"body":"<path id=\"{body}\" />"}}"#))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            r#"{{
                "prefix":"{prefix}",
                "info":{{
                    "name":"{prefix}",
                    "total":{},
                    "author":{{"name":"test","url":null}},
                    "license":{{"title":"MIT","spdx":"MIT","url":null}}
                }},
                "icons":{{{icon_entries}}}
            }}"#,
            icons.len()
        )
    }
}
