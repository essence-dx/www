use std::path::Path;

const DX_WWW_OUTPUT_SENTINELS: &[&str] = &[
    "index.html",
    "app/index.html",
    "source-routes/root/index.html",
    "manifest.json",
    "deploy-adapter.json",
];

pub(super) fn dx_www_output_present(project_root: &Path) -> bool {
    let output_root = project_root.join(".dx/www/output");
    DX_WWW_OUTPUT_SENTINELS
        .iter()
        .any(|relative_path| output_root.join(relative_path).is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_output(project_root: &Path, relative_path: &str) {
        let path = project_root.join(".dx/www/output").join(relative_path);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("output dir");
        std::fs::write(path, "<!doctype html>").expect("output file");
    }

    #[test]
    fn detects_legacy_root_output() {
        let project = tempfile::tempdir().expect("tempdir");
        write_output(project.path(), "index.html");

        assert!(dx_www_output_present(project.path()));
    }

    #[test]
    fn detects_app_router_output() {
        let project = tempfile::tempdir().expect("tempdir");
        write_output(project.path(), "app/index.html");

        assert!(dx_www_output_present(project.path()));
    }

    #[test]
    fn detects_source_route_output() {
        let project = tempfile::tempdir().expect("tempdir");
        write_output(project.path(), "source-routes/root/index.html");

        assert!(dx_www_output_present(project.path()));
    }

    #[test]
    fn detects_manifest_only_output() {
        let project = tempfile::tempdir().expect("tempdir");
        write_output(project.path(), "manifest.json");

        assert!(dx_www_output_present(project.path()));
    }

    #[test]
    fn rejects_missing_or_empty_output() {
        let missing = tempfile::tempdir().expect("missing");
        let empty = tempfile::tempdir().expect("empty");
        std::fs::create_dir_all(empty.path().join(".dx/www/output")).expect("empty output");

        assert!(!dx_www_output_present(missing.path()));
        assert!(!dx_www_output_present(empty.path()));
    }
}
