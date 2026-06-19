use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use dx_compiler::ecosystem::DxForgeImportEcosystem;

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

const COMMAND_FIELD: &str = "forge import";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeImportCommandOptions {
    pub(super) ecosystem: String,
    pub(super) package_name: String,
    pub(super) project: PathBuf,
    pub(super) source_dir: Option<PathBuf>,
    pub(super) selected_files: Vec<String>,
    pub(super) accepted_plan: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) fail_under_explicit: bool,
    pub(super) write: bool,
    pub(super) plan: bool,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_import_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeImportCommandOptions> {
    let ecosystem = args.first().map(String::as_str).unwrap_or_default();
    let ecosystem = DxForgeImportEcosystem::from_segment(ecosystem).ok_or_else(|| {
        forge_import_options_error(
            format!("Unsupported Forge import source: {ecosystem}"),
            "forge import",
        )
    })?;
    let ecosystem = ecosystem.as_segment();

    let mut package_name: Option<String> = None;
    let mut project: Option<PathBuf> = None;
    let mut source_dir: Option<PathBuf> = None;
    let mut selected_files = Vec::new();
    let mut accepted_plan: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 80u8;
    let mut fail_under_explicit = false;
    let mut plan = false;
    let mut write = false;
    let mut quiet = false;
    let mut index = 1usize;

    while index < args.len() {
        match args[index].as_str() {
            "--plan" | "--dry-run" => {
                plan = true;
                index += 1;
            }
            "--write" | "--materialize" => {
                write = true;
                index += 1;
            }
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_import_options_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--source-dir" | "--source" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_import_options_error("--source-dir requires a path", "source-dir")
                })?;
                source_dir = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--file" | "--files" | "--source-file" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_import_options_error("--file requires a package-relative path", "file")
                })?;
                for file in value
                    .split(',')
                    .map(str::trim)
                    .filter(|file| !file.is_empty())
                {
                    selected_files.push(file.replace('\\', "/"));
                }
                index += 2;
            }
            "--from-plan" | "--accepted-plan" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_import_options_error(
                        "--from-plan requires an accepted import plan path",
                        "from-plan",
                    )
                })?;
                accepted_plan = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_import_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_import_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_import_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                fail_under_explicit = true;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(forge_import_options_error(
                    format!("Unknown forge import option: {value}"),
                    COMMAND_FIELD,
                ));
            }
            value => {
                if package_name.replace(value.to_string()).is_some() {
                    return Err(forge_import_options_error(
                        "dx forge import accepts one package at a time",
                        "package",
                    ));
                }
                index += 1;
            }
        }
    }

    if plan && write {
        return Err(forge_import_options_error(
            "choose either --plan or --write for dx forge import",
            COMMAND_FIELD,
        ));
    }

    if !plan && !write {
        return Err(forge_import_options_error(
            "dx forge import requires --plan or --write",
            COMMAND_FIELD,
        ));
    }

    let package_name = package_name.ok_or_else(|| {
        forge_import_options_error("dx forge import requires a package name", "package")
    })?;

    Ok(DxForgeImportCommandOptions {
        ecosystem: ecosystem.to_string(),
        package_name,
        project: project.unwrap_or_else(|| cwd.to_path_buf()),
        source_dir,
        selected_files,
        accepted_plan,
        output,
        format,
        fail_under,
        fail_under_explicit,
        write,
        plan,
        quiet,
    })
}

fn forge_import_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn parse_forge_import_options_accepts_plan_output_and_json_alias() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_forge_import_options(
            &cwd,
            &args(&[
                "npm",
                "lodash",
                "--plan",
                "--project",
                "apps/site",
                "--source-dir",
                ".dx/cache/npm/lodash/package",
                "--out",
                ".dx/forge/import-plans/npm-lodash.json",
                "--json",
                "--fail-under",
                "90",
                "--quiet",
            ]),
        )
        .expect("options");

        assert_eq!(options.ecosystem, "npm");
        assert_eq!(options.package_name, "lodash");
        assert!(options.project.ends_with("apps/site"));
        assert_eq!(
            options.source_dir,
            Some(cwd.join(".dx/cache/npm/lodash/package"))
        );
        assert!(options.selected_files.is_empty());
        assert_eq!(options.accepted_plan, None);
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/forge/import-plans/npm-lodash.json"))
        );
        assert_eq!(options.format, DxOutputFormat::Json);
        assert_eq!(options.fail_under, 90);
        assert!(options.fail_under_explicit);
        assert!(options.plan);
        assert!(!options.write);
        assert!(options.quiet);
    }

    #[test]
    fn parse_forge_import_options_accepts_write_mode() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_forge_import_options(
            &cwd,
            &args(&[
                "npm",
                "lodash",
                "--write",
                "--from-plan",
                ".dx/forge/import-plans/npm-lodash.json",
            ]),
        )
        .expect("options");

        assert_eq!(options.ecosystem, "npm");
        assert_eq!(options.package_name, "lodash");
        assert_eq!(options.project, cwd);
        assert_eq!(
            options.accepted_plan,
            Some(PathBuf::from(
                "G:/workspace/.dx/forge/import-plans/npm-lodash.json"
            ))
        );
        assert_eq!(options.fail_under, 80);
        assert!(!options.fail_under_explicit);
        assert!(options.write);
        assert!(!options.plan);
        assert!(options.selected_files.is_empty());
    }

    #[test]
    fn parse_forge_import_options_accepts_selected_source_files() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_forge_import_options(
            &cwd,
            &args(&[
                "npm",
                "lodash",
                "--write",
                "--file",
                "src/index.ts,src/array.ts",
                "--source-file",
                "styles\\tokens.css",
            ]),
        )
        .expect("options");

        assert_eq!(
            options.selected_files,
            vec![
                "src/index.ts".to_string(),
                "src/array.ts".to_string(),
                "styles/tokens.css".to_string(),
            ]
        );
    }

    #[test]
    fn parse_forge_import_options_rejects_missing_mode() {
        let error =
            parse_forge_import_options(&PathBuf::from("G:/workspace"), &args(&["npm", "lodash"]))
                .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "dx forge import requires --plan or --write");
                assert_eq!(field.as_deref(), Some(COMMAND_FIELD));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_import_options_rejects_unknown_import_source() {
        let error = parse_forge_import_options(
            &PathBuf::from("G:/workspace"),
            &args(&["apt", "curl", "--plan"]),
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unsupported Forge import source: apt");
                assert_eq!(field.as_deref(), Some("forge import"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_import_options_accepts_external_ecosystem_plans() {
        let cwd = PathBuf::from("G:/workspace");
        for ecosystem in [
            "pip", "cargo", "go", "pub", "maven", "nuget", "composer", "gem", "swift", "jsr",
            "hex", "cran",
        ] {
            let options =
                parse_forge_import_options(&cwd, &args(&[ecosystem, "sample-package", "--plan"]))
                    .expect("options");

            assert_eq!(options.ecosystem, ecosystem);
            assert_eq!(options.package_name, "sample-package");
            assert!(options.plan);
            assert!(!options.write);
        }
    }
}
