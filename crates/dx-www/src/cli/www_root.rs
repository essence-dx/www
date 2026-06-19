use std::path::{Path, PathBuf};

pub(super) fn discover_www_repo_root(cwd: &Path) -> PathBuf {
    let mut current = cwd.to_path_buf();
    loop {
        if current.join("dx-www/src/cli/mod.rs").is_file()
            && current.join("docs/getting-started.md").is_file()
        {
            return current;
        }
        if !current.pop() {
            return cwd.to_path_buf();
        }
    }
}
