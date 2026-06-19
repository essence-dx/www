fn is_localhost(host: &str) -> bool {
    matches!(
        host.to_ascii_lowercase().as_str(),
        "localhost" | "127.0.0.1" | "::1"
    )
}

fn package_section(audit: &DxForgeAuditReport) -> DxCheckSection {
    let findings = audit
        .findings
        .iter()
        .map(check_finding_from_supply_chain)
        .collect::<Vec<_>>();

    DxCheckSection {
        name: "packages".to_string(),
        score: audit.risk_score,
        traffic: audit.traffic,
        metrics: Vec::new(),
        findings,
    }
}

fn security_section(audit: &DxForgeAuditReport) -> DxCheckSection {
    let red = audit
        .findings
        .iter()
        .filter(|finding| {
            matches!(
                finding.severity,
                DxSupplyChainSeverity::Critical | DxSupplyChainSeverity::High
            )
        })
        .count() as u32;
    let yellow = audit
        .findings
        .iter()
        .filter(|finding| matches!(finding.severity, DxSupplyChainSeverity::Medium))
        .count() as u32;
    let scan_findings = ScanFindings {
        supply_chain_red_findings: red,
        supply_chain_yellow_findings: yellow,
        ..Default::default()
    };
    let score = calculate_score(&scan_findings);
    let traffic = if red > 0 {
        DxUpdateTraffic::Red
    } else if yellow > 0 {
        DxUpdateTraffic::Yellow
    } else {
        DxUpdateTraffic::Green
    };
    let findings = audit
        .findings
        .iter()
        .filter(|finding| {
            matches!(
                finding.severity,
                DxSupplyChainSeverity::Critical
                    | DxSupplyChainSeverity::High
                    | DxSupplyChainSeverity::Medium
            )
        })
        .map(check_finding_from_supply_chain)
        .collect::<Vec<_>>();

    DxCheckSection {
        name: "security".to_string(),
        score,
        traffic,
        metrics: Vec::new(),
        findings,
    }
}

fn forge_source_state_metrics(report: &DxForgeSourceStateReport) -> Vec<DxCheckMetric> {
    let mut clean_files = 0u64;
    let mut edited_files = 0u64;
    let mut missing_files = 0u64;
    let mut blocked_files = 0u64;

    for file in report
        .packages
        .iter()
        .flat_map(|package| package.files.iter())
    {
        if file.actual_hash.is_none() {
            missing_files += 1;
            continue;
        }

        match file.traffic {
            DxUpdateTraffic::Green => clean_files += 1,
            DxUpdateTraffic::Yellow => edited_files += 1,
            DxUpdateTraffic::Red => blocked_files += 1,
        }
    }

    vec![
        check_metric("clean_files", clean_files),
        check_metric("edited_files", edited_files),
        check_metric("missing_files", missing_files),
        check_metric("blocked_files", blocked_files),
    ]
}

fn maintainability_section(root: &Path) -> Result<DxCheckSection> {
    let mut findings = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(should_visit_maintainability_entry)
        .filter_map(|entry| entry.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let metadata = fs::metadata(path).with_context(|| format!("stat `{}`", path.display()))?;
        if metadata.len() > 512 * 1024 {
            findings.push(check_finding(
                DxSupplyChainSeverity::Low,
                "large-source-file",
                "source file is larger than 512 KiB",
                Some(relative_path(root, path)),
                "Split large hand-authored files or move generated assets out of source.",
            ));
        }
    }

    Ok(section_from_findings("maintainability", findings))
}

fn section_from_findings(name: &str, findings: Vec<DxCheckFinding>) -> DxCheckSection {
    let score = check_score_from_findings(&findings);
    let traffic = if findings.iter().any(|finding| {
        matches!(
            finding.severity,
            DxSupplyChainSeverity::Critical | DxSupplyChainSeverity::High
        )
    }) {
        DxUpdateTraffic::Red
    } else if findings
        .iter()
        .any(|finding| matches!(finding.severity, DxSupplyChainSeverity::Medium))
    {
        DxUpdateTraffic::Yellow
    } else {
        DxUpdateTraffic::Green
    };

    DxCheckSection {
        name: name.to_string(),
        score,
        traffic,
        metrics: Vec::new(),
        findings,
    }
}

pub(super) fn check_metric(name: impl Into<String>, value: u64) -> DxCheckMetric {
    DxCheckMetric {
        name: name.into(),
        value,
    }
}

fn section_metric(section: &DxCheckSection, name: &str) -> u64 {
    section
        .metrics
        .iter()
        .find(|metric| metric.name == name)
        .map(|metric| metric.value)
        .unwrap_or(0)
}

fn check_score_from_findings(findings: &[DxCheckFinding]) -> u8 {
    let mut score = 100i32;
    let has_blocking_finding = findings.iter().any(|finding| {
        matches!(
            finding.severity,
            DxSupplyChainSeverity::Critical | DxSupplyChainSeverity::High
        )
    });

    for finding in findings {
        score -= match finding.severity {
            DxSupplyChainSeverity::Critical => 60,
            DxSupplyChainSeverity::High => 40,
            DxSupplyChainSeverity::Medium => 15,
            DxSupplyChainSeverity::Low => 5,
            DxSupplyChainSeverity::Info => 0,
        };
    }
    let score = score.clamp(0, 100) as u8;
    if !findings.is_empty() && !has_blocking_finding {
        score.max(90)
    } else {
        score
    }
}

fn combined_score(sections: &[DxCheckSection]) -> u8 {
    if sections.is_empty() {
        return 100;
    }
    let total = sections
        .iter()
        .map(|section| section.score as u32)
        .sum::<u32>();
    (total / sections.len() as u32) as u8
}

pub(super) fn check_finding(
    severity: DxSupplyChainSeverity,
    code: impl Into<String>,
    message: impl Into<String>,
    evidence_path: Option<String>,
    remediation: impl Into<String>,
) -> DxCheckFinding {
    DxCheckFinding {
        severity,
        code: code.into(),
        message: message.into(),
        evidence_path,
        remediation: remediation.into(),
    }
}

fn check_finding_from_supply_chain(finding: &DxSupplyChainFinding) -> DxCheckFinding {
    DxCheckFinding {
        severity: finding.severity,
        code: finding.code.clone(),
        message: finding.message.clone(),
        evidence_path: finding.evidence_path.clone(),
        remediation: finding.remediation.clone(),
    }
}

fn strongest_traffic(left: DxUpdateTraffic, right: DxUpdateTraffic) -> DxUpdateTraffic {
    match (left, right) {
        (DxUpdateTraffic::Red, _) | (_, DxUpdateTraffic::Red) => DxUpdateTraffic::Red,
        (DxUpdateTraffic::Yellow, _) | (_, DxUpdateTraffic::Yellow) => DxUpdateTraffic::Yellow,
        _ => DxUpdateTraffic::Green,
    }
}

fn count_contract_sources(root: &Path, dirs: &[&str], extensions: &[&str]) -> u64 {
    contract_source_paths(root, dirs, extensions).len() as u64
}

fn serializer_cache_stale(root: &Path) -> bool {
    let dx_modified = root
        .join(DX_CONFIG_PATH)
        .metadata()
        .and_then(|metadata| metadata.modified());
    let Ok(dx_modified) = dx_modified else {
        return false;
    };

    root.join(DX_SERIALIZER_MACHINE_PATH)
        .metadata()
        .and_then(|metadata| metadata.modified())
        .is_ok_and(|modified| modified < dx_modified)
}

fn dx_llm_config_value(content: &str, key: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let line = line.trim();
        let value = line.strip_prefix(key)?.strip_prefix('=')?.trim();
        Some(value.trim_matches('"').to_string())
    })
}

fn contract_source_paths(root: &Path, dirs: &[&str], extensions: &[&str]) -> Vec<String> {
    dirs.iter()
        .map(|dir| root.join(dir))
        .filter(|dir| dir.is_dir())
        .flat_map(|dir| WalkDir::new(dir).into_iter().filter_map(|entry| entry.ok()))
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extensions.contains(&extension))
        })
        .map(|entry| relative_path(root, entry.path()))
        .collect()
}

fn count_files_under(root: &Path, dir: &str) -> u64 {
    let path = root.join(dir);
    if !path.is_dir() {
        return 0;
    }
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .count() as u64
}

fn forge_owned_contract_paths(root: &Path) -> HashSet<String> {
    let manifest_path = root.join(SOURCE_MANIFEST_PATH);
    fs::read(&manifest_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<DxSourceManifest>(&bytes).ok())
        .map(|manifest| {
            manifest
                .packages
                .into_iter()
                .flat_map(|package| package.files.into_iter().map(|file| file.path))
                .collect()
        })
        .unwrap_or_default()
}

fn should_visit_maintainability_entry(entry: &DirEntry) -> bool {
    let Some(name) = entry.file_name().to_str() else {
        return true;
    };
    if is_windows_reserved_path_name(name) {
        return false;
    }
    !matches!(
        name,
        "node_modules" | "target" | ".git" | ".next" | "dist" | ".dx"
    )
}

fn is_windows_reserved_path_name(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name);
    matches!(
        stem.to_ascii_uppercase().as_str(),
        "CON"
            | "PRN"
            | "AUX"
            | "NUL"
            | "COM1"
            | "COM2"
            | "COM3"
            | "COM4"
            | "COM5"
            | "COM6"
            | "COM7"
            | "COM8"
            | "COM9"
            | "LPT1"
            | "LPT2"
            | "LPT3"
            | "LPT4"
            | "LPT5"
            | "LPT6"
            | "LPT7"
            | "LPT8"
            | "LPT9"
    )
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn metric_summary(metrics: &[DxCheckMetric]) -> String {
    metrics
        .iter()
        .map(|metric| format!("{}={}", metric.name, metric.value))
        .collect::<Vec<_>>()
        .join(", ")
}
