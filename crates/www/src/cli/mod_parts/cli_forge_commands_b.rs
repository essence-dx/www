impl Cli {
    fn cmd_forge_evidence(&self, args: &[String]) -> DxResult<()> {
        let DxForgeEvidenceCommandOptions {
            project,
            history,
            output,
            format,
            quiet,
        } = parse_forge_evidence_options(&self.cwd, args)?;
        let history = history.unwrap_or_else(|| {
            self.cwd
                .join("benchmarks/reports/vertical-proof-history/index.json")
        });
        let report =
            build_forge_release_evidence_report(&project, &history).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
                forge_release_evidence_markdown(&report)
            }
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
            write_forge_release_evidence_history(&project, &report).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }
        Ok(())
    }

    fn cmd_forge_release_trend(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut release_history: Option<PathBuf> = None;
        let mut medium: Option<PathBuf> = None;
        let mut large: Option<PathBuf> = None;
        let mut history: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under: Option<u8> = None;
        let mut write_history = false;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a path".to_string(),
                                field: Some("forge release-trend".to_string()),
                            })?;
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--release-history" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--release-history requires a JSON file".to_string(),
                                field: Some("forge release-trend".to_string()),
                            })?;
                    release_history = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--medium" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--medium requires a JSON file".to_string(),
                                field: Some("forge release-trend".to_string()),
                            })?;
                    medium = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--large" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--large requires a JSON file".to_string(),
                                field: Some("forge release-trend".to_string()),
                            })?;
                    large = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--history" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--history requires a trend JSON file".to_string(),
                                field: Some("forge release-trend".to_string()),
                            })?;
                    history = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a report path".to_string(),
                                field: Some("forge release-trend".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("forge release-trend".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("forge release-trend".to_string()),
                            })?;
                    fail_under = Some(parse_score_threshold(value)?);
                    index += 2;
                }
                "--write-history" => {
                    write_history = true;
                    index += 1;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge release-trend option: {value}"),
                        field: Some("forge release-trend".to_string()),
                    });
                }
                value => {
                    if output.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge release-trend path: {value}"),
                            field: Some("forge release-trend".to_string()),
                        });
                    }
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let project = std::fs::canonicalize(&project).unwrap_or(project);
        let report_dir = project.join("benchmarks/reports");
        let report =
            build_forge_release_readiness_trend_report(DxForgeReleaseReadinessTrendInput {
                project: project.clone(),
                release_history_path: release_history
                    .unwrap_or_else(|| report_dir.join("forge-public-release-history.json")),
                medium_route_path: medium
                    .unwrap_or_else(|| report_dir.join("forge-medium-route-comparison.json")),
                large_route_path: large
                    .unwrap_or_else(|| report_dir.join("forge-large-content-comparison.json")),
                trend_history_path: history
                    .unwrap_or_else(|| report_dir.join("forge-release-readiness-trend.json")),
                write_history,
            })
            .map_err(forge_error)?;

        let rendered = match format {
            DxOutputFormat::Terminal => forge_release_readiness_trend_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_release_readiness_trend_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, rendered).map_err(forge_error)?;
        } else if !quiet {
            println!("{rendered}");
        }

        if let Some(threshold) = fail_under {
            if report.score < threshold {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "DX Forge release-trend score {} is below fail-under threshold {}",
                        report.score, threshold
                    ),
                    field: Some("forge release-trend".to_string()),
                });
            }
        }

        if !report.passed {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge release-trend needs review: {} finding(s)",
                    report.findings.len()
                ),
                field: Some("forge release-trend".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_launch_changelog(&self, args: &[String]) -> DxResult<()> {
        let mut history: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under: Option<u8> = None;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--history" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--history requires a release-history JSON file"
                                    .to_string(),
                                field: Some("forge launch-changelog".to_string()),
                            })?;
                    history = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a changelog path".to_string(),
                                field: Some("forge launch-changelog".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("forge launch-changelog".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("forge launch-changelog".to_string()),
                            })?;
                    fail_under = Some(parse_score_threshold(value)?);
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge launch-changelog option: {value}"),
                        field: Some("forge launch-changelog".to_string()),
                    });
                }
                value => {
                    if output.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge launch-changelog path: {value}"),
                            field: Some("forge launch-changelog".to_string()),
                        });
                    }
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let history = history.unwrap_or_else(|| {
            self.cwd
                .join("benchmarks/reports/forge-public-release-history.json")
        });
        let report = build_forge_launch_changelog_report(DxForgeLaunchChangelogInput {
            history_path: history,
        })
        .map_err(forge_error)?;

        let rendered = match format {
            DxOutputFormat::Terminal => forge_launch_changelog_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_launch_changelog_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, rendered).map_err(forge_error)?;
        } else if !quiet {
            println!("{rendered}");
        }

        if let Some(threshold) = fail_under {
            if report.score < threshold {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "DX Forge launch-changelog score {} is below fail-under threshold {threshold}",
                        report.score
                    ),
                    field: Some("fail-under".to_string()),
                });
            }
        }

        Ok(())
    }

    fn cmd_forge_release_notes(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut history: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a value".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--history" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--history requires a value".to_string(),
                                field: Some("history".to_string()),
                            })?;
                    history = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a value".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires a value".to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge release-notes option: {value}"),
                        field: Some("forge release-notes".to_string()),
                    });
                }
                value => {
                    if project.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge release-notes path: {value}"),
                            field: Some("project".to_string()),
                        });
                    }
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let history = history.unwrap_or_else(|| {
            self.cwd
                .join("benchmarks/reports/vertical-proof-history/index.json")
        });
        let report = build_forge_release_notes_report(&project, &history, fail_under)
            .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
                forge_release_notes_markdown(&report)
            }
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge release-notes score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_release_bundle(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut out: Option<PathBuf> = None;
        let mut verify: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
        let mut include_adoption = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a path".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--out" | "--output" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--out requires a directory".to_string(),
                                field: Some("out".to_string()),
                            })?;
                    out = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--verify" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--verify requires a directory".to_string(),
                                field: Some("verify".to_string()),
                            })?;
                    verify = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                "--include-adoption" => {
                    include_adoption = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge release-bundle option: {value}"),
                        field: Some("forge release-bundle".to_string()),
                    });
                }
                value => {
                    if project.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge release-bundle path: {value}"),
                            field: Some("project".to_string()),
                        });
                    }
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        if verify.is_some() && out.is_some() {
            return Err(DxError::ConfigValidationError {
                message: "--verify cannot be combined with --out".to_string(),
                field: Some("forge release-bundle".to_string()),
            });
        }

        let report = if let Some(bundle_dir) = verify {
            verify_forge_release_bundle_with_options(&bundle_dir, include_adoption)
                .map_err(forge_error)?
        } else {
            let project = project.unwrap_or_else(|| self.cwd.clone());
            let out = out.unwrap_or_else(|| self.cwd.join(".dx/forge-release-bundle"));
            build_forge_release_bundle(&project, &out, fail_under, include_adoption)
                .map_err(forge_error)?
        };
        let rendered = match format {
            DxOutputFormat::Terminal => forge_release_bundle_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_release_bundle_markdown(&report),
        };

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge release-bundle score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_release_bundle_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_release_bundle_inspect(&self, args: &[String]) -> DxResult<()> {
        let mut bundle: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--bundle" | "--release-bundle" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--bundle requires a release bundle directory".to_string(),
                                field: Some("bundle".to_string()),
                            })?;
                    bundle = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge release-bundle-inspect option: {value}"),
                        field: Some("forge release-bundle-inspect".to_string()),
                    });
                }
                value => {
                    if bundle.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!(
                                "Unexpected forge release-bundle-inspect path: {value}"
                            ),
                            field: Some("bundle".to_string()),
                        });
                    }
                    bundle = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let bundle = bundle.unwrap_or_else(|| self.cwd.join(".dx/forge-release-bundle-adoption"));
        let report =
            build_forge_release_bundle_inspect_report(&bundle, fail_under).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_release_bundle_inspect_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_release_bundle_inspect_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge release-bundle-inspect score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_release_bundle_inspect_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_release_triage(&self, args: &[String]) -> DxResult<()> {
        let mut release_operations: Option<PathBuf> = None;
        let mut publish_plan: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--release-operations" | "--operations" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--release-operations requires a JSON report".to_string(),
                                field: Some("release-operations".to_string()),
                            })?;
                    release_operations = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--publish-plan" | "--plan" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--publish-plan requires a JSON report".to_string(),
                                field: Some("publish-plan".to_string()),
                            })?;
                    publish_plan = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge release-triage option: {value}"),
                        field: Some("forge release-triage".to_string()),
                    });
                }
                value => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unexpected forge release-triage path: {value}"),
                        field: Some("forge release-triage".to_string()),
                    });
                }
            }
        }

        let release_operations = release_operations
            .unwrap_or_else(|| self.cwd.join(".dx/ci/forge-release-operations.json"));
        let publish_plan =
            publish_plan.unwrap_or_else(|| self.cwd.join(".dx/ci/forge-publish-plan.json"));
        let report =
            build_forge_release_triage_report(&release_operations, &publish_plan, fail_under)
                .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_release_triage_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_release_triage_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        Ok(())
    }

    fn cmd_forge_beta_artifact_verify(&self, args: &[String]) -> DxResult<()> {
        let mut release_bundle: Option<PathBuf> = None;
        let mut pages: Option<PathBuf> = None;
        let mut registry_smoke: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--bundle" | "--release-bundle" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--release-bundle requires a directory".to_string(),
                                field: Some("release-bundle".to_string()),
                            })?;
                    release_bundle = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--pages" | "--pages-bundle" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--pages requires a Pages bundle directory".to_string(),
                                field: Some("pages".to_string()),
                            })?;
                    pages = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--registry-smoke" | "--r2-evidence" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--registry-smoke requires a JSON evidence path"
                                    .to_string(),
                                field: Some("registry-smoke".to_string()),
                            })?;
                    registry_smoke = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge beta-artifact-verify option: {value}"),
                        field: Some("forge beta-artifact-verify".to_string()),
                    });
                }
                value => {
                    if release_bundle.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge beta-artifact-verify path: {value}"),
                            field: Some("release-bundle".to_string()),
                        });
                    }
                    release_bundle = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let release_bundle =
            release_bundle.unwrap_or_else(|| self.cwd.join(".dx/forge-release-bundle-adoption"));
        let pages = pages.unwrap_or_else(|| self.cwd.join(".dx/forge-pages"));
        let registry_smoke =
            registry_smoke.unwrap_or_else(|| self.cwd.join(".dx/ci/forge-registry-smoke.json"));
        let report = build_forge_beta_artifact_verify_report(
            &release_bundle,
            &pages,
            &registry_smoke,
            fail_under,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_beta_artifact_verify_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_beta_artifact_verify_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge beta-artifact-verify score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_beta_artifact_verify_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_operator_dashboard(&self, args: &[String]) -> DxResult<()> {
        let mut release_triage: Option<PathBuf> = None;
        let mut beta_artifact_verify: Option<PathBuf> = None;
        let mut ci_snippets: Option<PathBuf> = None;
        let mut installability: Option<PathBuf> = None;
        let mut installability_history: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--release-triage" | "--triage" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--release-triage requires a JSON path".to_string(),
                                field: Some("release-triage".to_string()),
                            })?;
                    release_triage = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--beta-artifact-verify" | "--beta-verify" | "--downloaded-beta-verify" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--beta-artifact-verify requires a JSON path".to_string(),
                                field: Some("beta-artifact-verify".to_string()),
                            })?;
                    beta_artifact_verify = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--ci-snippets" | "--ci" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--ci-snippets requires a JSON path".to_string(),
                                field: Some("ci-snippets".to_string()),
                            })?;
                    ci_snippets = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--installability" | "--installability-snapshot" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--installability requires a JSON path".to_string(),
                                field: Some("installability".to_string()),
                            })?;
                    installability = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--installability-history" | "--history" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--installability-history requires a JSON path"
                                    .to_string(),
                                field: Some("installability-history".to_string()),
                            })?;
                    installability_history = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge operator-dashboard option: {value}"),
                        field: Some("forge operator-dashboard".to_string()),
                    });
                }
                value => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unexpected forge operator-dashboard path: {value}"),
                        field: Some("forge operator-dashboard".to_string()),
                    });
                }
            }
        }

        let release_triage =
            release_triage.unwrap_or_else(|| self.cwd.join(".dx/ci/forge-release-triage.json"));
        let beta_artifact_verify = beta_artifact_verify
            .unwrap_or_else(|| self.cwd.join(".dx/ci/forge-beta-artifact-verify.json"));
        let ci_snippets =
            ci_snippets.unwrap_or_else(|| self.cwd.join(".dx/ci/forge-ci-snippets.json"));
        let installability = installability.unwrap_or_else(|| {
            self.cwd
                .join("benchmarks/reports/forge-installability-snapshot.json")
        });
        let installability_history = installability_history.unwrap_or_else(|| {
            self.cwd
                .join("benchmarks/reports/forge-installability-history/index.json")
        });

        let report = build_forge_operator_dashboard_report(
            &release_triage,
            &beta_artifact_verify,
            &ci_snippets,
            &installability,
            &installability_history,
            fail_under,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_operator_dashboard_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_operator_dashboard_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge operator-dashboard score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_operator_dashboard_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_release_operations(&self, args: &[String]) -> DxResult<()> {
        let DxForgeReleaseOperationsCommandOptions {
            project,
            release_bundle,
            release_manifest,
            trust_regression,
            release_candidate,
            ci_artifacts,
            public_evidence,
            output,
            format,
            fail_under,
            quiet,
        } = parse_forge_release_operations_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let release_manifest = release_manifest
            .or_else(|| {
                release_bundle
                    .as_ref()
                    .map(|bundle| bundle.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON))
            })
            .unwrap_or_else(|| {
                project
                    .join(".dx/forge-release-bundle")
                    .join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON)
            });
        let trust_regression =
            trust_regression.unwrap_or_else(|| project.join(".dx/forge/trust-regression.json"));
        let release_candidate = release_candidate
            .unwrap_or_else(|| project.join(".dx/ci/forge-release-candidate.json"));
        let ci_artifacts = ci_artifacts.unwrap_or_else(|| project.join(".dx/ci"));
        let public_evidence = public_evidence.unwrap_or_else(|| project.join("public"));

        let report = build_forge_release_operations_report(
            &project,
            &release_manifest,
            &trust_regression,
            &release_candidate,
            &ci_artifacts,
            &public_evidence,
            fail_under,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_release_operations_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_release_operations_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge release-operations score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_release_operations_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_publish_plan(&self, args: &[String]) -> DxResult<()> {
        let DxForgePublishPlanCommandOptions {
            project,
            release_bundle,
            pages,
            registry_smoke,
            release_operations,
            output,
            format,
            fail_under,
            quiet,
        } = parse_forge_publish_plan_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let release_bundle =
            release_bundle.unwrap_or_else(|| project.join(".dx/forge-release-bundle-adoption"));
        let pages = pages.unwrap_or_else(|| project.join(".dx/forge-pages"));
        let registry_smoke =
            registry_smoke.unwrap_or_else(|| project.join(".dx/ci/forge-registry-smoke.json"));
        let release_operations = release_operations
            .unwrap_or_else(|| project.join(".dx/ci/forge-release-operations.json"));

        let report = build_forge_publish_plan_report(
            &project,
            &release_bundle,
            &pages,
            &registry_smoke,
            &release_operations,
            fail_under,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_publish_plan_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_publish_plan_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge publish-plan score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_publish_plan_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_release_review(&self, args: &[String]) -> DxResult<()> {
        let DxForgeReleaseReviewCommandOptions {
            project,
            bundle,
            dashboard,
            history,
            route_comparison,
            output,
            format,
            fail_under,
            quiet,
        } = parse_forge_release_review_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let bundle = bundle.unwrap_or_else(|| project.join(".dx/forge-release-bundle"));
        let dashboard =
            dashboard.unwrap_or_else(|| project.join(".dx/ci/forge-release-dashboard.json"));
        let history = history.unwrap_or_else(|| {
            project.join("benchmarks/reports/forge-public-release-history.json")
        });
        let route_comparison = route_comparison.unwrap_or_else(|| {
            project.join("benchmarks/reports/forge-public-route-comparison.json")
        });
        let report = build_forge_release_review_report(
            &project,
            &bundle,
            &dashboard,
            &history,
            &route_comparison,
            fail_under,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_release_review_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_release_review_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge release-review score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_release_review_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_init_app(&self, args: &[String]) -> DxResult<()> {
        let DxForgeInitAppCommandOptions {
            project,
            output,
            format,
            write,
            dry_run: _dry_run,
            quiet,
        } = parse_forge_init_app_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let report = build_forge_init_app_report(&project, write).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_init_app_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_init_app_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_init_app_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_adoption_smoke(&self, args: &[String]) -> DxResult<()> {
        let DxForgeAdoptionSmokeCommandOptions {
            project,
            output,
            format,
            fail_under,
            quiet,
        } = parse_forge_adoption_smoke_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(default_forge_adoption_smoke_project);
        let report =
            build_forge_adoption_smoke_report(&project, fail_under).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_adoption_smoke_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_adoption_smoke_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "Forge adoption smoke score {} is below required threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_adoption_smoke_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_adoption_report(&self, args: &[String]) -> DxResult<()> {
        let DxForgeAdoptionReportCommandOptions {
            project,
            release_bundle,
            output,
            format,
            fail_under,
            quiet,
        } = parse_forge_adoption_report_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let report = build_forge_adoption_report(&project, release_bundle, fail_under)
            .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_adoption_report_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_adoption_report_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "Forge adoption report score {} is below required threshold {fail_under}",
                    report.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_adoption_report_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_beta_install(&self, args: &[String]) -> DxResult<()> {
        let DxForgeBetaInstallCommandOptions {
            project,
            release_bundle,
            artifacts,
            output,
            format,
            fail_under,
            write,
            dry_run: _dry_run,
            quiet,
        } = parse_forge_beta_install_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(|| self.cwd.join(".dx/forge-beta-app"));
        let release_bundle =
            release_bundle.unwrap_or_else(|| self.cwd.join(".dx/forge-release-bundle-adoption"));
        let artifacts = artifacts.unwrap_or_else(|| project.join(".dx/forge/beta-install"));
        let report = build_forge_beta_install_report(
            &project,
            &release_bundle,
            &artifacts,
            write,
            fail_under,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_beta_install_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_beta_install_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge beta-install score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("forge beta-install".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_beta_install_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_smoke(&self, args: &[String]) -> DxResult<()> {
        let DxForgeSmokeCommandOptions {
            project,
            output,
            mut format,
            fail_under,
            ci,
        } = parse_forge_smoke_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(default_forge_smoke_project);
        if ci {
            format = DxOutputFormat::Json;
        }
        let report = build_forge_smoke_report(&project).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_smoke_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_smoke_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        println!("{rendered}");

        if let Some(minimum) = fail_under {
            if report.score < minimum {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Forge smoke score {} is below required threshold {minimum}",
                        report.score
                    ),
                    field: Some("fail-under".to_string()),
                });
            }
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_smoke_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_badge(&self, args: &[String]) -> DxResult<()> {
        let DxForgeBadgeCommandOptions {
            project,
            output,
            fail_under,
            quiet,
        } = parse_forge_badge_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(default_forge_smoke_project);
        let smoke = build_forge_smoke_report(&project).map_err(forge_error)?;
        let evidence = build_forge_release_evidence_report(
            &project,
            &smoke.launch_artifacts.benchmark_history_path,
        )
        .map_err(forge_error)?;
        let badge = build_forge_readiness_badge(&smoke, &evidence, None, fail_under);
        let rendered = serde_json::to_string_pretty(&badge).map_err(forge_error)?;

        if let Some(path) = output {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
                    path: Some(parent.to_path_buf()),
                    message: format!("{}:{}: {}", file!(), line!(), error),
                })?;
            }
            std::fs::write(&path, rendered).map_err(|error| DxError::IoError {
                path: Some(path.clone()),
                message: format!("{}:{}: {}", file!(), line!(), error),
            })?;
            if !quiet {
                println!(
                    "DX Forge readiness badge\nOutput: {}\nStatus: {}\nScore: {}",
                    path.display(),
                    badge.status,
                    badge.score
                );
            }
        } else {
            println!("{rendered}");
        }

        if badge.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "Forge readiness badge score {} is below required threshold {fail_under}",
                    badge.score
                ),
                field: Some("fail-under".to_string()),
            });
        }

        if !badge.passed {
            return Err(DxError::InternalError {
                message: forge_readiness_badge_failure_summary(&badge),
            });
        }

        Ok(())
    }

}
