#[test]
fn forge_golden_schema_fixture_covers_release_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    cli.cmd_add(&["icon", "search", "--write"])
        .expect("dx add icon search");
    cli.cmd_add(&["auth/better-auth", "--write"])
        .expect("dx add auth/better-auth");
    let history_path = write_passing_forge_benchmark_history(dir.path());
    let artifacts = forge_golden_artifact_values(dir.path(), &history_path);
    let fixture = read_forge_golden_schema_fixture();

    assert_eq!(fixture.version, 1);
    assert!(!fixture.artifacts.is_empty());
    for artifact in &fixture.artifacts {
        let value = artifacts
            .get(&artifact.name)
            .unwrap_or_else(|| panic!("unknown golden artifact `{}`", artifact.name));
        assert!(
            !artifact.required_paths.is_empty(),
            "golden artifact `{}` has no required paths",
            artifact.name
        );
        for path in &artifact.required_paths {
            assert!(
                json_fixture_path_exists(value, path),
                "golden artifact `{}` is missing required JSON path `{path}`",
                artifact.name
            );
        }
    }
    for name in artifacts.keys() {
        assert!(
            fixture
                .artifacts
                .iter()
                .any(|artifact| artifact.name == *name),
            "golden fixture does not cover `{name}`"
        );
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_golden_schema_fixture_covers_ci_evidence_artifacts() {
    let fixture = read_forge_golden_schema_fixture();
    let artifact_names = fixture
        .artifacts
        .iter()
        .map(|artifact| artifact.name.as_str())
        .collect::<BTreeSet<_>>();

    for artifact in [
        "forge_smoke",
        "forge_triage",
        "forge_readiness_badge",
        "forge_ci_claims",
    ] {
        assert!(
            artifact_names.contains(artifact),
            "golden fixture does not cover CI evidence artifact `{artifact}`"
        );
    }
}

#[test]
fn forge_golden_schema_fixture_covers_public_release_json_contracts() {
    let fixture = read_forge_golden_schema_fixture();
    let artifact_names = fixture
        .artifacts
        .iter()
        .map(|artifact| artifact.name.as_str())
        .collect::<BTreeSet<_>>();

    for artifact in [
        "pages_bundle_verification",
        "release_bundle_manifest",
        "release_notes",
        "public_evidence",
        "public_route_comparison",
        "public_release_history",
        "public_launch_changelog",
    ] {
        assert!(
            artifact_names.contains(artifact),
            "golden fixture does not cover public release JSON artifact `{artifact}`"
        );
    }
}

#[test]
fn forge_golden_schema_fixture_covers_adoption_artifacts() {
    let fixture = read_forge_golden_schema_fixture();
    let artifact_names = fixture
        .artifacts
        .iter()
        .map(|artifact| artifact.name.as_str())
        .collect::<BTreeSet<_>>();

    for artifact in [
        "forge_adoption_report",
        "forge_adoption_claims",
        "forge_adoption_proof",
    ] {
        assert!(
            artifact_names.contains(artifact),
            "golden fixture does not cover adoption artifact `{artifact}`"
        );
    }
}

fn first_receipt_path(root: &Path) -> PathBuf {
    fs::read_dir(root.join(".dx/forge/receipts"))
        .expect("receipts")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .next()
        .expect("receipt path")
}

#[derive(Debug, serde::Deserialize)]
struct ForgeGoldenSchemaFixture {
    version: u32,
    artifacts: Vec<ForgeGoldenArtifactFixture>,
}

#[derive(Debug, serde::Deserialize)]
struct ForgeGoldenArtifactFixture {
    name: String,
    required_paths: Vec<String>,
}

fn read_forge_golden_schema_fixture() -> ForgeGoldenSchemaFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/forge-golden/schema-required-fields.json");
    let text = fs::read_to_string(&path).expect("golden schema fixture");
    serde_json::from_str(&text).expect("golden schema fixture json")
}

fn forge_golden_artifact_values(
    project: &Path,
    history_path: &Path,
) -> BTreeMap<String, serde_json::Value> {
    let mut artifacts = BTreeMap::new();
    artifacts.insert(
        "source_manifest".to_string(),
        read_json_value(project.join(".dx/forge/source-manifest.json")),
    );
    artifacts.insert(
        "receipt".to_string(),
        read_json_value(first_receipt_path(project)),
    );
    artifacts.insert(
        "doctor_report".to_string(),
        serde_json::to_value(build_forge_doctor_report(project).expect("doctor report"))
            .expect("doctor json"),
    );
    artifacts.insert(
        "scorecard".to_string(),
        serde_json::to_value(
            build_forge_package_scorecard_for_project(project).expect("scorecard report"),
        )
        .expect("scorecard json"),
    );
    artifacts.insert(
        "public_evidence".to_string(),
        serde_json::to_value(build_forge_public_evidence_report(
            &build_forge_package_scorecard_for_project(project).expect("scorecard report"),
        ))
        .expect("public evidence json"),
    );
    artifacts.insert(
        "release_evidence".to_string(),
        serde_json::to_value(
            build_forge_release_evidence_report(project, history_path).expect("release proof"),
        )
        .expect("release proof json"),
    );
    let release_project = tempdir().expect("release notes project");
    let release_history_path = write_passing_forge_benchmark_history(release_project.path());
    artifacts.insert(
        "release_notes".to_string(),
        serde_json::to_value(
            build_forge_release_notes_report(release_project.path(), &release_history_path, 90)
                .expect("release notes"),
        )
        .expect("release notes json"),
    );
    let route_comparison_path = write_passing_public_route_comparison(project);
    artifacts.insert(
        "public_route_comparison".to_string(),
        read_json_value(route_comparison_path),
    );
    let dashboard_path = write_passing_release_dashboard(project);
    let route_comparison_path = write_passing_public_route_comparison(project);
    let release_history_path = project.join("benchmarks/reports/forge-public-release-history.json");
    record_forge_public_release_history(DxForgePublicReleaseHistoryInput {
        root: project.to_path_buf(),
        dashboard_path,
        route_comparison_path,
        history_path: release_history_path.clone(),
    })
    .expect("public release history");
    artifacts.insert(
        "public_release_history".to_string(),
        read_json_value(release_history_path.clone()),
    );
    artifacts.insert(
        "public_launch_changelog".to_string(),
        serde_json::to_value(
            build_forge_launch_changelog_report(DxForgeLaunchChangelogInput {
                history_path: release_history_path,
            })
            .expect("public launch changelog"),
        )
        .expect("public launch changelog json"),
    );
    let release_bundle_project = tempdir().expect("golden release bundle project");
    let release_bundle_dashboard = write_passing_release_dashboard(release_bundle_project.path());
    let release_bundle_routes =
        write_passing_public_route_comparison(release_bundle_project.path());
    let release_bundle_history = release_bundle_project
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    record_forge_public_release_history(DxForgePublicReleaseHistoryInput {
        root: release_bundle_project.path().to_path_buf(),
        dashboard_path: release_bundle_dashboard,
        route_comparison_path: release_bundle_routes,
        history_path: release_bundle_history,
    })
    .expect("golden release bundle history");
    let release_bundle_dir = release_bundle_project.path().join("release-bundle");
    build_forge_release_bundle(
        release_bundle_project.path(),
        &release_bundle_dir,
        90,
        false,
    )
    .expect("golden release bundle");
    artifacts.insert(
        "release_bundle_manifest".to_string(),
        read_json_value(release_bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON)),
    );
    let smoke = build_forge_smoke_report(project).expect("forge smoke report");
    let ci_artifact_dir = project.join(".dx/forge/golden-ci-artifacts");
    write_forge_ci_artifacts(&smoke, &ci_artifact_dir, 90).expect("write forge ci artifacts");
    let ci_evidence = build_forge_release_evidence_report(
        project,
        &smoke.launch_artifacts.benchmark_history_path,
    )
    .expect("forge ci evidence");
    let ci_badge = build_forge_readiness_badge(
        &smoke,
        &ci_evidence,
        Some(ci_artifact_dir.join("forge-smoke.json")),
        90,
    );
    artifacts.insert(
        "forge_smoke".to_string(),
        read_json_value(ci_artifact_dir.join("forge-smoke.json")),
    );
    artifacts.insert(
        "forge_triage".to_string(),
        forge_triage_markdown_contract(
            &fs::read_to_string(ci_artifact_dir.join("forge-triage.md"))
                .expect("forge triage markdown"),
        ),
    );
    artifacts.insert(
        "forge_readiness_badge".to_string(),
        read_json_value(ci_artifact_dir.join("forge-readiness-badge.json")),
    );
    artifacts.insert(
        "forge_ci_claims".to_string(),
        serde_json::to_value(prove_fixtures::forge_ci_claims_manifest(&smoke, &ci_badge))
            .expect("forge ci claims json"),
    );
    let adoption_project = tempdir().expect("golden adoption project");
    let adoption_report = write_forge_adoption_contract_artifacts(adoption_project.path());
    artifacts.insert(
        "forge_adoption_report".to_string(),
        serde_json::to_value(&adoption_report).expect("forge adoption report json"),
    );
    artifacts.insert(
        "forge_adoption_claims".to_string(),
        read_json_value(
            adoption_project
                .path()
                .join("public/forge/adoption.claims.json"),
        ),
    );
    artifacts.insert(
        "forge_adoption_proof".to_string(),
        read_json_value(adoption_project.path().join("public/proof.json")),
    );
    let pages_project = tempdir().expect("pages bundle project");
    let pages_cli = Cli::with_cwd(pages_project.path().to_path_buf());
    let pages_dir = pages_project.path().join("forge-pages");
    write_forge_pages_publish_bundle(
        &pages_cli,
        pages_project.path(),
        &pages_dir,
        &ci_artifact_dir,
    );
    artifacts.insert(
        "pages_bundle_verification".to_string(),
        serde_json::to_value(
            verify_forge_pages_bundle(&pages_dir).expect("pages bundle verification"),
        )
        .expect("pages bundle verification json"),
    );
    artifacts
}

fn write_forge_adoption_contract_artifacts(project: &Path) -> DxForgeAdoptionReport {
    let cli = Cli::with_cwd(project.to_path_buf());
    cli.cmd_forge(&[
        "adoption-smoke".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge adoption smoke");

    let report = build_forge_adoption_report(project, None, 90).expect("forge adoption report");

    cli.cmd_prove(&[
        "vertical".to_string(),
        "--fixture".to_string(),
        "forge-adoption".to_string(),
        "--out".to_string(),
        "public".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge adoption route proof");

    assert!(project.join("public/forge/adoption.claims.json").is_file());
    assert!(project.join("public/proof.json").is_file());
    assert!(!project.join("node_modules").exists());
    report
}

fn forge_triage_markdown_contract(markdown: &str) -> serde_json::Value {
    let title = markdown
        .lines()
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
        .unwrap_or("untitled");
    let mut sections = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut current_body = String::new();

    for line in markdown.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(previous_heading) = current_heading.replace(heading.trim().to_string()) {
                sections.push(serde_json::json!({
                    "heading": previous_heading,
                    "body": current_body.trim()
                }));
                current_body.clear();
            }
        } else if current_heading.is_some() {
            current_body.push_str(line);
            current_body.push('\n');
        }
    }
    if let Some(heading) = current_heading {
        sections.push(serde_json::json!({
            "heading": heading,
            "body": current_body.trim()
        }));
    }

    let code_mentions = markdown_code_spans(markdown)
        .into_iter()
        .map(|value| serde_json::json!({ "value": value }))
        .collect::<Vec<_>>();

    serde_json::json!({
        "title": title,
        "sections": sections,
        "code_mentions": code_mentions
    })
}

fn markdown_code_spans(markdown: &str) -> BTreeSet<String> {
    let mut spans = BTreeSet::new();
    for segment in markdown.split('`').skip(1).step_by(2) {
        if !segment.trim().is_empty() {
            spans.insert(segment.trim().to_string());
        }
    }
    spans
}

fn read_json_value(path: PathBuf) -> serde_json::Value {
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read json {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("parse json {}: {error}", path.display()))
}

fn first_generated_css_href(html: &str) -> Option<String> {
    let marker = r#"<link rel="stylesheet" href=""#;
    html.split(marker).skip(1).find_map(|part| {
        let (href, rest) = part.split_once('"')?;
        rest.contains(r#"data-dx-generated="true""#)
            .then(|| href.to_string())
    })
}

fn copy_test_dir(source: &Path, destination: &Path) {
    if destination.exists() {
        fs::remove_dir_all(destination)
            .unwrap_or_else(|error| panic!("remove {}: {error}", destination.display()));
    }
    for entry in walkdir::WalkDir::new(source)
        .into_iter()
        .filter_map(Result::ok)
    {
        let relative = entry
            .path()
            .strip_prefix(source)
            .unwrap_or_else(|error| panic!("relative copy path: {error}"));
        let target = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)
                .unwrap_or_else(|error| panic!("create {}: {error}", target.display()));
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)
                    .unwrap_or_else(|error| panic!("create {}: {error}", parent.display()));
            }
            fs::copy(entry.path(), &target)
                .unwrap_or_else(|error| panic!("copy {}: {error}", target.display()));
        }
    }
}

fn trust_regression_case<'a>(cases: &'a [serde_json::Value], name: &str) -> &'a serde_json::Value {
    cases
        .iter()
        .find(|case| case["name"] == name)
        .unwrap_or_else(|| panic!("missing trust-regression case {name}"))
}

fn registry_smoke_operation<'a>(
    operations: &'a [serde_json::Value],
    action: &str,
) -> &'a serde_json::Value {
    operations
        .iter()
        .find(|operation| operation["action"] == action)
        .unwrap_or_else(|| panic!("missing registry-smoke operation {action}"))
}

fn artifact_names(path: &Path) -> BTreeSet<String> {
    fs::read_dir(path)
        .expect("artifact dir")
        .map(|entry| {
            entry
                .expect("artifact entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

fn write_public_evidence_export_fixture(project: &Path) -> PathBuf {
    let public_dir = project.join("public");
    let ci_project = project.join("ci-project");
    fs::create_dir_all(&ci_project).expect("ci project");
    let ci_cli = Cli::with_cwd(ci_project.clone());
    let ci_artifact_dir = ci_project.join("ci-artifacts");

    ci_cli
        .cmd_forge(&[
            "ci".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            ci_artifact_dir.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect("forge ci public artifacts");

    let cli = Cli::with_cwd(project.to_path_buf());
    for fixture in ["forge-scorecard", "forge-ci", "forge-evidence"] {
        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            fixture.to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .unwrap_or_else(|error| panic!("prove fixture {fixture}: {error}"));
    }

    fs::copy(
        ci_artifact_dir.join("forge-readiness-badge.json"),
        public_dir.join("forge-readiness-badge.json"),
    )
    .expect("copy readiness badge");
    copy_public_evidence_benchmark_reports(&public_dir);

    public_dir
}

fn copy_public_evidence_benchmark_reports(public_dir: &Path) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .expect("repo root");
    for artifact in [
        "forge-public-route-comparison.md",
        "forge-launch-delivery-comparison.md",
    ] {
        let source = repo_root.join("benchmarks/reports").join(artifact);
        fs::copy(&source, public_dir.join(artifact))
            .unwrap_or_else(|error| panic!("copy benchmark report {}: {error}", source.display()));
    }
}

fn write_forge_pages_publish_bundle(
    cli: &Cli,
    pages_project: &Path,
    pages_dir: &Path,
    ci_artifact_dir: &Path,
) {
    fs::create_dir_all(pages_dir).expect("pages dir");
    fs::copy(
        ci_artifact_dir.join("forge-readiness-badge.json"),
        pages_dir.join("forge-readiness-badge.json"),
    )
    .expect("copy readiness badge into pages bundle");
    write_passing_public_release_history(pages_project);
    write_forge_pages_fixture(cli, pages_dir, "forge-ci");
    write_forge_pages_fixture(cli, pages_dir, "forge-releases");
    write_forge_pages_fixture(cli, pages_dir, "forge-changelog");
    let adoption_project = pages_project.join(".dx/pages-adoption-project");
    let adoption_cli = Cli::with_cwd(adoption_project.clone());
    adoption_cli
        .cmd_forge(&[
            "adoption-smoke".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--fail-under".to_string(),
            "90".to_string(),
            "--quiet".to_string(),
        ])
        .expect("pages adoption smoke");
    write_forge_pages_fixture(&adoption_cli, pages_dir, "forge-adoption");
    fs::copy(
        pages_dir.join("forge/ci.proof.json"),
        pages_dir.join("proof.json"),
    )
    .expect("preserve legacy /forge/ci proof.json");
}

fn write_forge_pages_fixture(cli: &Cli, pages_dir: &Path, fixture: &str) {
    let (html, clean_index, proof) = match fixture {
        "forge-ci" => (
            "forge/ci.html",
            "forge/ci/index.html",
            "forge/ci.proof.json",
        ),
        "forge-releases" => (
            "forge/releases.html",
            "forge/releases/index.html",
            "forge/releases.proof.json",
        ),
        "forge-changelog" => (
            "forge/changelog.html",
            "forge/changelog/index.html",
            "forge/changelog.proof.json",
        ),
        "forge-adoption" => (
            "forge/adoption.html",
            "forge/adoption/index.html",
            "forge/adoption.proof.json",
        ),
        other => panic!("unsupported pages fixture {other}"),
    };

    cli.cmd_prove(&[
        "vertical".to_string(),
        "--fixture".to_string(),
        fixture.to_string(),
        "--out".to_string(),
        pages_dir.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .unwrap_or_else(|error| panic!("prove pages fixture {fixture}: {error}"));

    fs::copy(pages_dir.join("proof.json"), pages_dir.join(proof))
        .unwrap_or_else(|error| panic!("copy pages proof {proof}: {error}"));
    let clean_index_path = pages_dir.join(clean_index);
    if let Some(parent) = clean_index_path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|error| panic!("create pages clean route {clean_index}: {error}"));
    }
    fs::copy(pages_dir.join(html), clean_index_path)
        .unwrap_or_else(|error| panic!("copy pages clean route {clean_index}: {error}"));
}

fn write_passing_public_release_history(root: &Path) -> PathBuf {
    let dashboard_path = write_passing_release_dashboard(root);
    let route_comparison_path = write_passing_public_route_comparison(root);
    let history_path = root.join("benchmarks/reports/forge-public-release-history.json");
    record_forge_public_release_history(DxForgePublicReleaseHistoryInput {
        root: root.to_path_buf(),
        dashboard_path,
        route_comparison_path,
        history_path: history_path.clone(),
    })
    .expect("public release history fixture");
    history_path
}

fn assert_secret_markers_absent(path: &Path) {
    let mut pending = vec![path.to_path_buf()];
    while let Some(current) = pending.pop() {
        for entry in fs::read_dir(&current)
            .unwrap_or_else(|error| panic!("read dir {}: {error}", current.display()))
        {
            let entry = entry.expect("public artifact entry");
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            let text = String::from_utf8_lossy(
                &fs::read(&path)
                    .unwrap_or_else(|error| panic!("read artifact {}: {error}", path.display())),
            )
            .into_owned();
            for marker in [
                "CLOUDFLARE_R2_",
                "DX_FORGE_R2_LIVE",
                "R2_SECRET",
                "SECRET_ACCESS_KEY",
            ] {
                assert!(
                    !text.contains(marker),
                    "public artifact {} leaked marker {marker}",
                    path.display()
                );
            }
        }
    }
}

fn json_fixture_path_exists(value: &serde_json::Value, path: &str) -> bool {
    let mut current = vec![value];
    for segment in path.split('.') {
        let (key, wants_array) = segment
            .strip_suffix("[]")
            .map(|key| (key, true))
            .unwrap_or((segment, false));
        let mut next = Vec::new();
        for node in current {
            let Some(child) = node.get(key) else {
                continue;
            };
            if wants_array {
                let Some(items) = child.as_array() else {
                    continue;
                };
                if items.is_empty() {
                    continue;
                }
                next.extend(items);
            } else {
                next.push(child);
            }
        }
        if next.is_empty() {
            return false;
        }
        current = next;
    }
    true
}

fn write_passing_forge_benchmark_history(root: &Path) -> PathBuf {
    let history_dir = root.join("benchmarks/reports/vertical-proof-history");
    fs::create_dir_all(&history_dir).expect("history dir");
    let history_path = history_dir.join("index.json");
    fs::write(
        &history_path,
        r#"{
  "updated_at": "2026-05-16T00:00:00Z",
  "snapshots": [
    {
      "generated_at": "2026-05-16T00:00:00Z",
      "fixture_mode": "forge-site",
      "route_delivery": "static",
      "markdown": "vertical-proof-history/forge-site.md",
      "forge_packages": 2,
      "forge_files_tracked": 7,
      "decoded_bytes": 5200,
      "brotli_bytes": 1250,
      "http_route_median_ms": 2.1,
      "chrome_load_event_ms": 10.0,
      "dx_packet_applied": false,
      "interaction_works": false
    },
    {
      "generated_at": "2026-05-16T00:00:00Z",
      "fixture_mode": "forge-combo",
      "markdown": "vertical-proof-history/snapshot.md",
      "forge_packages": 4,
      "forge_files_tracked": 13,
      "decoded_bytes": 12000,
      "brotli_bytes": 3200,
      "http_route_median_ms": 2.5,
      "chrome_load_event_ms": 35.0,
      "dx_packet_applied": true,
      "interaction_works": true
    }
  ]
}"#,
    )
    .expect("history index");
    history_path
}

struct ForgeReleaseCandidateFixture {
    ci_artifact_dir: PathBuf,
    pages_dir: PathBuf,
    route_comparison_path: PathBuf,
    source_review_path: PathBuf,
    static_evidence_path: PathBuf,
}

fn write_release_candidate_fixture(root: &Path) -> ForgeReleaseCandidateFixture {
    let ci_project = root.join(".dx/rc-ci-project");
    let pages_project = root.join(".dx/rc-pages-project");
    fs::create_dir_all(&ci_project).expect("ci project");
    fs::create_dir_all(&pages_project).expect("pages project");
    let ci_cli = Cli::with_cwd(ci_project.clone());
    let pages_cli = Cli::with_cwd(pages_project.clone());
    let ci_artifact_dir = root.join(".dx/rc-ci-artifacts");
    let pages_dir = root.join(".dx/rc-forge-pages");

    ci_cli
        .cmd_forge(&[
            "ci".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            ci_artifact_dir.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect("release candidate CI artifacts");
    write_forge_pages_publish_bundle(&pages_cli, &pages_project, &pages_dir, &ci_artifact_dir);

    ForgeReleaseCandidateFixture {
        ci_artifact_dir,
        pages_dir,
        route_comparison_path: write_passing_public_route_comparison_with_adoption(root),
        source_review_path: write_passing_source_owned_review_report(root),
        static_evidence_path: write_passing_static_competitor_evidence_report(root),
    }
}

fn write_passing_source_owned_review_report(root: &Path) -> PathBuf {
    let report_dir = root.join("benchmarks/reports");
    fs::create_dir_all(&report_dir).expect("source review dir");
    let report_path = report_dir.join("forge-source-owned-package-review.json");
    let packages = FORGE_WWW_TEMPLATE_PACKAGE_IDS
        .iter()
        .map(|package_id| {
            serde_json::json!({
                "package_id": package_id,
                "source_kind": "curated-registry",
                "docs": { "exists": true },
                "receipts": [format!("receipt-{}.json", package_id.replace('/', "-"))],
                "rollback": { "exists": true },
                "advisory": {
                    "placeholder": true,
                    "provider": "dx-forge-curated-advisory-fixture"
                }
            })
        })
        .collect::<Vec<_>>();
    let report = serde_json::json!({
        "generated_at": "2026-05-18T00:00:00Z",
        "report_id": "forge-source-owned-package-review-v1",
        "score": 100,
        "passed": true,
        "no_node_modules": true,
        "package_count": packages.len(),
        "packages": packages,
        "checks": {
            "docs": { "passed": true },
            "receipts": { "passed": true },
            "rollback": { "passed": true },
            "advisory": { "passed": true },
            "yellow_review": { "passed": true }
        },
        "findings": []
    });
    fs::write(
        &report_path,
        serde_json::to_string_pretty(&report).expect("source-owned review report json"),
    )
    .expect("source-owned review report");
    report_path
}

fn write_passing_static_competitor_evidence_report(root: &Path) -> PathBuf {
    let report_dir = root.join("benchmarks/reports");
    fs::create_dir_all(&report_dir).expect("static evidence dir");
    let report_path = report_dir.join("forge-static-competitor-evidence.json");
    fs::write(
        &report_path,
        r#"{
  "generated_at": "2026-05-18T00:00:00Z",
  "source_route_comparison": "benchmarks/reports/forge-public-route-comparison.json",
  "score": 100,
  "passed": true,
  "scope": {
    "community_excluded": true,
    "not_full_framework_benchmark": true,
    "competitor_builds_not_run": true,
    "no_package_install": true,
    "no_node_modules_created": true,
    "safe_public_claim": "This static-floor fixture does not prove broad framework replacement."
  },
  "required_routes": [
    "/forge",
    "/forge/scorecard",
    "/forge/ci",
    "/forge/evidence",
    "/forge/releases",
    "/forge/changelog",
    "/forge/adoption"
  ],
  "frameworks": [
    { "framework": "DX-WWW", "baseline_kind": "measured-static-routes", "route_count": 7 },
    { "framework": "Astro", "baseline_kind": "static-floor", "route_count": 7 },
    { "framework": "SvelteKit", "baseline_kind": "static-floor", "route_count": 7 },
    { "framework": "Next.js", "baseline_kind": "static-floor", "route_count": 7 }
  ],
  "findings": []
}"#,
    )
    .expect("static competitor evidence report");
    report_path
}

fn write_passing_release_dashboard(root: &Path) -> PathBuf {
    let report_dir = root.join(".dx/ci");
    fs::create_dir_all(&report_dir).expect("dashboard dir");
    let report_path = report_dir.join("forge-release-dashboard.json");
    let package_count = FORGE_WWW_TEMPLATE_PACKAGE_IDS.len();
    let report = serde_json::json!({
        "version": 1,
        "project": "project",
        "generated_at": "2026-05-16T00:00:00Z",
        "passed": true,
        "score": 93,
        "fail_under": 90,
        "checks": {
            "ci_artifacts": {
                "passed": true,
                "score": 100,
                "message": "13 CI artifacts and 6 route checks verified."
            },
            "pages_bundle": {
                "passed": true,
                "score": 100,
                "message": "12 Pages artifacts and 9 publish checks verified."
            },
            "release_notes": {
                "passed": true,
                "score": 93,
                "message": format!("Release notes status `passing` with {package_count} source-owned packages.")
            },
            "public_evidence": {
                "passed": true,
                "score": 100,
                "message": "9 public evidence links checked for route `/forge/evidence`."
            },
            "route_comparison": {
                "passed": true,
                "score": 100,
                "message": "6 public routes measured, 6014 Brotli bytes total."
            }
        },
        "release_notes": {
            "passed": true,
            "score": 93,
            "status": "passing",
            "no_node_modules": true,
            "package_count": package_count,
            "findings": []
        },
        "public_evidence": {
            "route": "/forge/evidence",
            "passed": true,
            "score": 100,
            "package_count": package_count,
            "links": 9,
            "findings": []
        },
        "findings": []
    });
    fs::write(
        &report_path,
        serde_json::to_string_pretty(&report).expect("release dashboard report json"),
    )
    .expect("release dashboard report");
    report_path
}

fn write_regressing_release_dashboard(root: &Path) -> PathBuf {
    let report_dir = root.join(".dx/ci");
    fs::create_dir_all(&report_dir).expect("dashboard dir");
    let report_path = report_dir.join("forge-release-dashboard-regression.json");
    let package_count = FORGE_WWW_TEMPLATE_PACKAGE_IDS.len();
    let report = serde_json::json!({
        "version": 1,
        "project": "project",
        "generated_at": "2026-05-16T00:05:00Z",
        "passed": false,
        "score": 88,
        "fail_under": 90,
        "checks": {
            "ci_artifacts": {
                "passed": true,
                "score": 100,
                "message": "13 CI artifacts and 6 route checks verified."
            },
            "pages_bundle": {
                "passed": true,
                "score": 100,
                "message": "6 Pages artifacts and 6 publish checks verified."
            },
            "release_notes": {
                "passed": false,
                "score": 88,
                "message": "Release notes need review after route regression."
            },
            "public_evidence": {
                "passed": true,
                "score": 100,
                "message": "9 public evidence links checked for route `/forge/evidence`."
            },
            "route_comparison": {
                "passed": false,
                "score": 80,
                "message": "3 public routes measured, 7000 Brotli bytes total."
            }
        },
        "release_notes": {
            "passed": false,
            "score": 88,
            "status": "needs-review",
            "no_node_modules": true,
            "package_count": package_count,
            "findings": ["Route budget failed."]
        },
        "public_evidence": {
            "route": "/forge/evidence",
            "passed": true,
            "score": 100,
            "package_count": package_count,
            "links": 9,
            "findings": []
        },
        "findings": ["Route budget failed."]
    });
    fs::write(
        &report_path,
        serde_json::to_string_pretty(&report).expect("regressing release dashboard report json"),
    )
    .expect("regressing release dashboard report");
    report_path
}

fn write_passing_public_route_comparison(root: &Path) -> PathBuf {
    let report_dir = root.join("benchmarks/reports");
    fs::create_dir_all(&report_dir).expect("route comparison dir");
    let report_path = report_dir.join("forge-public-route-comparison.json");
    fs::write(
        &report_path,
        r#"{
  "generated_at": "2026-05-16T00:00:00Z",
  "source_history_index": "vertical-proof-history/index.json",
  "route_count": 6,
  "total_decoded_bytes": 28731,
  "total_brotli_bytes": 6014,
  "lowest_brotli_route": "/forge/releases",
  "routes": [
    {
      "route": "/forge",
      "fixture_mode": "forge-site",
      "role": "Launch evidence",
      "status": "measured",
      "generated_at": "2026-05-16T00:00:00Z",
      "route_delivery": "static",
      "runtime_asset_written": false,
      "packet_artifact_written": true,
      "http_resources": 1,
      "forge_packages": 2,
      "forge_files_tracked": 7,
      "decoded_bytes": 5143,
      "brotli_bytes": 1240,
      "http_route_median_ms": 1.914,
      "chrome_load_event_ms": 10.2,
      "dx_packet_applied": false,
      "interaction_works": false,
      "markdown": "vertical-proof-history/forge-site.md",
      "json": "vertical-proof-history/forge-site.json",
      "budget_passed": true
    },
    {
      "route": "/forge/scorecard",
      "fixture_mode": "forge-scorecard",
      "role": "Package scorecard",
      "status": "measured",
      "generated_at": "2026-05-16T00:00:00Z",
      "route_delivery": "static",
      "runtime_asset_written": false,
      "packet_artifact_written": true,
      "http_resources": 1,
      "forge_packages": 0,
      "forge_files_tracked": 0,
      "decoded_bytes": 4189,
      "brotli_bytes": 1040,
      "http_route_median_ms": 2.195,
      "chrome_load_event_ms": 8.4,
      "dx_packet_applied": false,
      "interaction_works": false,
      "markdown": "vertical-proof-history/forge-scorecard.md",
      "json": "vertical-proof-history/forge-scorecard.json",
      "budget_passed": true
    },
    {
      "route": "/forge/ci",
      "fixture_mode": "forge-ci",
      "role": "CI evidence",
      "status": "measured",
      "generated_at": "2026-05-16T00:00:00Z",
      "route_delivery": "static",
      "runtime_asset_written": false,
      "packet_artifact_written": true,
      "http_resources": 1,
      "forge_packages": 0,
      "forge_files_tracked": 0,
      "decoded_bytes": 3309,
      "brotli_bytes": 862,
      "http_route_median_ms": 1.767,
      "chrome_load_event_ms": 8.9,
      "dx_packet_applied": false,
      "interaction_works": false,
      "markdown": "vertical-proof-history/forge-ci.md",
      "json": "vertical-proof-history/forge-ci.json",
      "budget_passed": true
    },
    {
      "route": "/forge/evidence",
      "fixture_mode": "forge-evidence",
      "role": "Evidence index",
      "status": "measured",
      "generated_at": "2026-05-16T00:00:00Z",
      "route_delivery": "static",
      "runtime_asset_written": false,
      "packet_artifact_written": true,
      "http_resources": 10,
      "forge_packages": 0,
      "forge_files_tracked": 0,
      "decoded_bytes": 6988,
      "brotli_bytes": 1106,
      "http_route_median_ms": 0.939,
      "chrome_load_event_ms": 8.3,
      "dx_packet_applied": false,
      "interaction_works": false,
      "markdown": "vertical-proof-history/forge-evidence.md",
      "json": "vertical-proof-history/forge-evidence.json",
      "budget_passed": true
    },
    {
      "route": "/forge/releases",
      "fixture_mode": "forge-releases",
      "role": "Release history",
      "status": "measured",
      "generated_at": "2026-05-16T00:00:00Z",
      "route_delivery": "static",
      "runtime_asset_written": false,
      "packet_artifact_written": true,
      "http_resources": 1,
      "forge_packages": 0,
      "forge_files_tracked": 0,
      "decoded_bytes": 4563,
      "brotli_bytes": 855,
      "http_route_median_ms": 1.763,
      "chrome_load_event_ms": 11.3,
      "dx_packet_applied": false,
      "interaction_works": false,
      "markdown": "vertical-proof-history/forge-releases.md",
      "json": "vertical-proof-history/forge-releases.json",
      "budget_passed": true
    },
    {
      "route": "/forge/changelog",
      "fixture_mode": "forge-changelog",
      "role": "Launch changelog",
      "status": "measured",
      "generated_at": "2026-05-16T00:00:00Z",
      "route_delivery": "static",
      "runtime_asset_written": false,
      "packet_artifact_written": true,
      "http_resources": 1,
      "forge_packages": 0,
      "forge_files_tracked": 0,
      "decoded_bytes": 4539,
      "brotli_bytes": 911,
      "http_route_median_ms": 2.993,
      "chrome_load_event_ms": 11.6,
      "dx_packet_applied": false,
      "interaction_works": false,
      "markdown": "vertical-proof-history/forge-changelog.md",
      "json": "vertical-proof-history/forge-changelog.json",
      "budget_passed": true
    }
  ]
}"#,
    )
    .expect("route comparison report");
    report_path
}

fn write_passing_public_route_comparison_with_adoption(root: &Path) -> PathBuf {
    let report_path = write_passing_public_route_comparison(root);
    let mut value = read_json_value(report_path.clone());
    let routes = value
        .get_mut("routes")
        .and_then(|routes| routes.as_array_mut())
        .expect("routes array");
    routes.push(serde_json::json!({
        "route": "/forge/adoption",
        "fixture_mode": "forge-adoption",
        "role": "Adoption evidence",
        "status": "measured",
        "generated_at": "2026-05-17T00:00:00Z",
        "route_delivery": "static",
        "runtime_asset_written": false,
        "packet_artifact_written": true,
        "http_resources": 1,
        "forge_packages": 4,
        "forge_files_tracked": 14,
        "decoded_bytes": 8465,
        "brotli_bytes": 1270,
        "http_route_median_ms": 2.815,
        "chrome_load_event_ms": 21.1,
        "dx_packet_applied": false,
        "interaction_works": false,
        "markdown": "vertical-proof-history/forge-adoption.md",
        "json": "vertical-proof-history/forge-adoption.json",
        "budget_passed": true
    }));
    value["route_count"] = serde_json::json!(routes.len());
    value["total_decoded_bytes"] = serde_json::json!(37196);
    value["total_brotli_bytes"] = serde_json::json!(7284);
    fs::write(
        &report_path,
        serde_json::to_string_pretty(&value).expect("route comparison json"),
    )
    .expect("route comparison with adoption");
    report_path
}

fn write_signed_adoption_release_bundle_fixture(root: &Path, bundle_name: &str) -> PathBuf {
    let cli = Cli::with_cwd(root.to_path_buf());
    let dashboard_path = write_passing_release_dashboard(root);
    let route_comparison_path = write_passing_public_route_comparison_with_adoption(root);
    let release_history_path = root.join("benchmarks/reports/forge-public-release-history.json");
    let bundle_dir = root.join(bundle_name);

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        release_history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release history fixture");
    cli.cmd_forge(&[
        "release-bundle".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--out".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--include-adoption".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("adoption release bundle fixture");

    let manifest_path = bundle_dir.join("forge-release-manifest.json");
    let manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
            .expect("manifest json");
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&signed_release_manifest_value_for_test(&manifest))
            .expect("signed manifest json"),
    )
    .expect("write signed manifest");

    bundle_dir
}

fn write_public_route_comparison_without_changelog(root: &Path) -> PathBuf {
    let report_path = write_passing_public_route_comparison(root);
    let mut value = read_json_value(report_path.clone());
    let route_count = {
        let routes = value
            .get_mut("routes")
            .and_then(|routes| routes.as_array_mut())
            .expect("routes array");
        routes.retain(|route| {
            route.get("route").and_then(|route| route.as_str()) != Some("/forge/changelog")
        });
        routes.len()
    };
    value["route_count"] = serde_json::json!(route_count);
    value["total_decoded_bytes"] = serde_json::json!(24192);
    value["total_brotli_bytes"] = serde_json::json!(5103);
    fs::write(
        &report_path,
        serde_json::to_string_pretty(&value).expect("route comparison json"),
    )
    .expect("route comparison without changelog");
    report_path
}

fn write_four_route_public_route_comparison(root: &Path) -> PathBuf {
    let report_dir = root.join("benchmarks/reports");
    fs::create_dir_all(&report_dir).expect("route comparison dir");
    let report_path = report_dir.join("forge-public-route-comparison-four-routes.json");
    fs::write(
        &report_path,
        r#"{
  "generated_at": "2026-05-15T23:55:00Z",
  "source_history_index": "vertical-proof-history/index.json",
  "route_count": 4,
  "total_decoded_bytes": 19629,
  "total_brotli_bytes": 4248,
  "lowest_brotli_route": "/forge/ci",
  "routes": [
    {
      "route": "/forge",
      "fixture_mode": "forge-site",
      "status": "measured",
      "route_delivery": "static",
      "decoded_bytes": 5143,
      "brotli_bytes": 1240,
      "http_route_median_ms": 1.914,
      "chrome_load_event_ms": 10.2,
      "budget_passed": true
    },
    {
      "route": "/forge/scorecard",
      "fixture_mode": "forge-scorecard",
      "status": "measured",
      "route_delivery": "static",
      "decoded_bytes": 4189,
      "brotli_bytes": 1040,
      "http_route_median_ms": 2.195,
      "chrome_load_event_ms": 8.4,
      "budget_passed": true
    },
    {
      "route": "/forge/ci",
      "fixture_mode": "forge-ci",
      "status": "measured",
      "route_delivery": "static",
      "decoded_bytes": 3309,
      "brotli_bytes": 862,
      "http_route_median_ms": 1.767,
      "chrome_load_event_ms": 8.9,
      "budget_passed": true
    },
    {
      "route": "/forge/evidence",
      "fixture_mode": "forge-evidence",
      "status": "measured",
      "route_delivery": "static",
      "decoded_bytes": 6988,
      "brotli_bytes": 1106,
      "http_route_median_ms": 0.939,
      "chrome_load_event_ms": 8.3,
      "budget_passed": true
    }
  ]
}"#,
    )
    .expect("four-route comparison report");
    report_path
}

fn write_regressing_public_route_comparison(root: &Path) -> PathBuf {
    let report_dir = root.join("benchmarks/reports");
    fs::create_dir_all(&report_dir).expect("route comparison dir");
    let report_path = report_dir.join("forge-public-route-comparison-regression.json");
    fs::write(
        &report_path,
        r#"{
  "generated_at": "2026-05-16T00:05:00Z",
  "source_history_index": "vertical-proof-history/index.json",
  "route_count": 3,
  "total_decoded_bytes": 20000,
  "total_brotli_bytes": 7000,
  "lowest_brotli_route": "/forge/ci",
  "routes": [
    {
      "route": "/forge",
      "fixture_mode": "forge-site",
      "status": "measured",
      "route_delivery": "static",
      "decoded_bytes": 6000,
      "brotli_bytes": 1500,
      "http_route_median_ms": 3.5,
      "chrome_load_event_ms": 18.0,
      "budget_passed": true
    },
    {
      "route": "/forge/scorecard",
      "fixture_mode": "forge-scorecard",
      "status": "measured",
      "route_delivery": "static",
      "decoded_bytes": 4189,
      "brotli_bytes": 1036,
      "http_route_median_ms": 2.3,
      "chrome_load_event_ms": 11.6,
      "budget_passed": true
    },
    {
      "route": "/forge/ci",
      "fixture_mode": "forge-ci",
      "status": "measured",
      "route_delivery": "static",
      "decoded_bytes": 3309,
      "brotli_bytes": 865,
      "http_route_median_ms": 4.162,
      "chrome_load_event_ms": 13.0,
      "budget_passed": false
    }
  ]
}"#,
    )
    .expect("regressing route comparison report");
    report_path
}

fn assert_forge_json_snapshot(name: &str, actual: serde_json::Value) {
    let expected_text = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/forge-golden")
            .join(name),
    )
    .unwrap_or_else(|error| panic!("read Forge JSON snapshot `{name}`: {error}"));
    let expected: serde_json::Value =
        serde_json::from_str(&expected_text).expect("Forge JSON snapshot");
    assert_eq!(expected, actual, "Forge JSON snapshot `{name}` drifted");
}

fn forge_release_manifest_shape_contract(
    manifest: &DxForgeReleaseBundleManifest,
) -> serde_json::Value {
    serde_json::json!({
        "version": manifest.version,
        "artifact_count": manifest.artifact_count,
        "hash_algorithm": &manifest.hash_algorithm,
        "integrity": {
            "scheme": &manifest.integrity.scheme,
            "signed": manifest.integrity.signed,
            "signature": manifest.integrity.signature.as_deref(),
            "message": &manifest.integrity.message,
        },
        "artifact_integrity": {
            "scheme": &manifest.artifact_integrity.scheme,
            "hash_algorithm": &manifest.artifact_integrity.hash_algorithm,
            "artifact_count": manifest.artifact_integrity.artifact_count,
            "verified_locally": manifest.artifact_integrity.verified_locally,
            "message": &manifest.artifact_integrity.message,
        },
        "publisher_identity": {
            "scheme": &manifest.publisher_identity.scheme,
            "status": &manifest.publisher_identity.status,
            "signer": manifest.publisher_identity.signer.as_deref(),
            "key_id": manifest.publisher_identity.key_id.as_deref(),
            "algorithm": manifest.publisher_identity.algorithm.as_deref(),
            "public_key": manifest.publisher_identity.public_key.as_deref(),
            "signature": manifest.publisher_identity.signature.as_deref(),
            "signed_at": manifest.publisher_identity.signed_at.as_deref(),
            "message": &manifest.publisher_identity.message,
        },
        "artifacts": manifest
            .artifacts
            .iter()
            .map(|artifact| {
                serde_json::json!({
                    "path": &artifact.path,
                    "artifact_type": &artifact.artifact_type,
                    "route": artifact.route.as_deref(),
                })
            })
            .collect::<Vec<_>>()
    })
}

fn signed_release_manifest_value_for_test(
    manifest: &DxForgeReleaseBundleManifest,
) -> serde_json::Value {
    use ed25519_dalek::{Signer, SigningKey};

    let signing_key = SigningKey::from_bytes(&[7u8; 32]);
    let public_key_bytes = signing_key.verifying_key().to_bytes();
    let public_key = format!("ed25519:{}", hex_encode_for_test(&public_key_bytes));
    let key_id = format!(
        "ed25519-blake3:{}",
        blake3::hash(&public_key_bytes).to_hex()
    );
    let signer = "dx-forge-test-publisher";
    let signed_at = "2026-05-18T00:00:00Z";
    let payload = release_manifest_signing_payload_for_test(
        manifest,
        signer,
        &key_id,
        &public_key,
        signed_at,
    );
    let signature = format!(
        "ed25519:{}",
        hex_encode_for_test(&signing_key.sign(payload.as_bytes()).to_bytes())
    );

    let mut value = serde_json::to_value(manifest).expect("manifest json");
    value["integrity"]["signed"] = serde_json::Value::Bool(true);
    value["integrity"]["signature"] = serde_json::Value::String(signature.clone());
    value["integrity"]["message"] = serde_json::Value::String(
        "Signed release manifest: Ed25519 publisher identity verified against the artifact digest."
            .to_string(),
    );
    value["publisher_identity"]["status"] = serde_json::Value::String("signed".to_string());
    value["publisher_identity"]["signer"] = serde_json::Value::String(signer.to_string());
    value["publisher_identity"]["key_id"] = serde_json::Value::String(key_id);
    value["publisher_identity"]["algorithm"] = serde_json::Value::String("ed25519".to_string());
    value["publisher_identity"]["public_key"] = serde_json::Value::String(public_key);
    value["publisher_identity"]["signature"] = serde_json::Value::String(signature);
    value["publisher_identity"]["signed_at"] = serde_json::Value::String(signed_at.to_string());
    value["publisher_identity"]["message"] = serde_json::Value::String(
            "Publisher identity is attached with an Ed25519 signature over the release manifest digest."
                .to_string(),
        );
    value
}

fn release_manifest_signing_payload_for_test(
    manifest: &DxForgeReleaseBundleManifest,
    signer: &str,
    key_id: &str,
    public_key: &str,
    signed_at: &str,
) -> String {
    format!(
        "dx-forge-release-publisher-identity-v1\nversion={}\nhash_algorithm={}\nintegrity_scheme={}\nartifact_integrity_scheme={}\nmanifest_digest={}\nartifact_integrity_digest={}\nartifact_count={}\nsigner={}\nkey_id={}\npublic_key={}\nsigned_at={}\n",
        manifest.version,
        manifest.hash_algorithm,
        manifest.integrity.scheme,
        manifest.artifact_integrity.scheme,
        manifest.integrity.digest,
        manifest.artifact_integrity.digest,
        manifest.artifact_count,
        signer,
        key_id,
        public_key,
        signed_at
    )
}

fn hex_encode_for_test(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn forge_pages_bundle_shape_contract(
    report: &DxForgePagesBundleVerificationReport,
) -> serde_json::Value {
    serde_json::json!({
        "artifact_count": report.artifacts.len(),
        "check_count": report.checks.len(),
        "passed": report.passed,
        "score": report.score,
        "artifacts": report
            .artifacts
            .iter()
            .map(|artifact| {
                serde_json::json!({
                    "name": &artifact.name,
                    "valid_json": artifact.valid_json,
                    "passed": artifact.passed,
                })
            })
            .collect::<Vec<_>>(),
        "checks": report
            .checks
            .iter()
            .map(|check| {
                serde_json::json!({
                    "route": &check.route,
                    "artifacts": &check.artifacts,
                    "passed": check.passed,
                    "message": &check.message,
                })
            })
            .collect::<Vec<_>>(),
        "findings": &report.findings,
    })
}

fn forge_launch_changelog_shape_contract(value: &serde_json::Value) -> serde_json::Value {
    let entries = value
        .get("entries")
        .and_then(|entries| entries.as_array())
        .cloned()
        .unwrap_or_default();
    let entry_shapes = entries
        .iter()
        .map(|entry| {
            serde_json::json!({
                "dashboard_score": entry.get("dashboard_score"),
                "dashboard_passed": entry.get("dashboard_passed"),
                "route_count": entry.get("route_count"),
                "total_decoded_bytes": entry.get("total_decoded_bytes"),
                "total_brotli_bytes": entry.get("total_brotli_bytes"),
                "total_decoded_delta_bytes": entry.get("total_decoded_delta_bytes"),
                "total_brotli_delta_bytes": entry.get("total_brotli_delta_bytes"),
                "added_routes": entry.get("added_routes"),
                "removed_routes": entry.get("removed_routes"),
                "changed_route_count": entry
                    .get("changed_routes")
                    .and_then(|routes| routes.as_array())
                    .map(|routes| routes.len())
                    .unwrap_or_default(),
                "regression_finding_count": entry
                    .get("regression_findings")
                    .and_then(|findings| findings.as_array())
                    .map(|findings| findings.len())
                    .unwrap_or_default(),
                "summary": entry.get("summary"),
            })
        })
        .collect::<Vec<_>>();
    let latest = value
        .get("latest")
        .cloned()
        .unwrap_or(serde_json::Value::Null);

    serde_json::json!({
        "version": value.get("version"),
        "passed": value.get("passed"),
        "score": value.get("score"),
        "status": value.get("status"),
        "record_count": value.get("record_count"),
        "latest": {
            "dashboard_score": latest.get("dashboard_score"),
            "dashboard_passed": latest.get("dashboard_passed"),
            "route_count": latest.get("route_count"),
            "total_decoded_delta_bytes": latest.get("total_decoded_delta_bytes"),
            "total_brotli_delta_bytes": latest.get("total_brotli_delta_bytes"),
            "added_routes": latest.get("added_routes"),
            "removed_routes": latest.get("removed_routes"),
            "changed_route_count": latest
                .get("changed_routes")
                .and_then(|routes| routes.as_array())
                .map(|routes| routes.len())
                .unwrap_or_default(),
            "regression_finding_count": latest
                .get("regression_findings")
                .and_then(|findings| findings.as_array())
                .map(|findings| findings.len())
                .unwrap_or_default(),
            "summary": latest.get("summary"),
        },
        "entry_count": entries.len(),
        "entries": entry_shapes,
        "honest_scope_count": value
            .get("honest_scope")
            .and_then(|scope| scope.as_array())
            .map(|scope| scope.len())
            .unwrap_or_default(),
        "findings": value.get("findings"),
    })
}

fn forge_public_route_comparison_shape_contract(value: &serde_json::Value) -> serde_json::Value {
    let routes = value
        .get("routes")
        .and_then(|routes| routes.as_array())
        .cloned()
        .unwrap_or_default();

    serde_json::json!({
        "route_count": value.get("route_count"),
        "total_decoded_bytes": value.get("total_decoded_bytes"),
        "total_brotli_bytes": value.get("total_brotli_bytes"),
        "lowest_brotli_route": value.get("lowest_brotli_route"),
        "routes": routes
            .iter()
            .map(|route| {
                serde_json::json!({
                    "route": route.get("route"),
                    "role": route.get("role"),
                    "fixture_mode": route.get("fixture_mode"),
                    "status": route.get("status"),
                    "route_delivery": route.get("route_delivery"),
                    "runtime_asset_written": route.get("runtime_asset_written"),
                    "packet_artifact_written": route.get("packet_artifact_written"),
                    "http_resources": route.get("http_resources"),
                    "forge_packages": route.get("forge_packages"),
                    "forge_files_tracked": route.get("forge_files_tracked"),
                    "decoded_bytes": route.get("decoded_bytes"),
                    "brotli_bytes": route.get("brotli_bytes"),
                    "dx_packet_applied": route.get("dx_packet_applied"),
                    "interaction_works": route.get("interaction_works"),
                    "budget_passed": route.get("budget_passed"),
                    "has_markdown": route.get("markdown").and_then(|value| value.as_str()).is_some(),
                    "has_json": route.get("json").and_then(|value| value.as_str()).is_some(),
                })
            })
            .collect::<Vec<_>>(),
    })
}

fn forge_adoption_report_shape_contract(value: &serde_json::Value) -> serde_json::Value {
    let empty = Vec::new();
    let packages = value
        .get("packages")
        .and_then(|packages| packages.as_array())
        .unwrap_or(&empty);
    let routes = value
        .get("public_routes")
        .and_then(|routes| routes.as_array())
        .unwrap_or(&empty);

    serde_json::json!({
        "version": value.get("version"),
        "passed": value.get("passed"),
        "score": value.get("score"),
        "fail_under": value.get("fail_under"),
        "no_node_modules": value.get("no_node_modules"),
        "package_count": value.get("package_count"),
        "receipt_count_covers_packages": value
            .get("receipt_count")
            .and_then(|receipt_count| receipt_count.as_u64())
            .zip(value.get("package_count").and_then(|package_count| package_count.as_u64()))
            .map(|(receipt_count, package_count)| receipt_count >= package_count)
            .unwrap_or(false),
        "package_docs_present": value.get("package_docs_present"),
        "package_docs_missing": value.get("package_docs_missing"),
        "packages": packages
            .iter()
            .map(|package| {
                serde_json::json!({
                    "package_id": package.get("package_id"),
                    "variant": package.get("variant"),
                    "version": package.get("version"),
                    "file_count": package.get("file_count"),
                    "docs_exists": package.get("docs_exists"),
                    "has_rollback_receipt": package
                        .get("rollback_receipt")
                        .and_then(|receipt| receipt.as_str())
                        .is_some(),
                })
            })
            .collect::<Vec<_>>(),
        "project_structure": {
            "dx_config_exists": value.pointer("/project_structure/dx_config_exists"),
            "pages_dir_exists": value.pointer("/project_structure/pages_dir_exists"),
            "components_dir_exists": value.pointer("/project_structure/components_dir_exists"),
            "app_route_exists": value.pointer("/project_structure/app_route_exists"),
            "app_route_path_is_adoption_fixture": json_path_ends_with(
                value.pointer("/project_structure/app_route_path"),
                "pages/forge-adoption.html",
            ),
        },
        "public_routes": routes
            .iter()
            .map(|route| {
                serde_json::json!({
                    "route": route.get("route"),
                    "html_exists": route.get("html_exists"),
                    "clean_index_exists": route.get("clean_index_exists"),
                    "packet_exists": route.get("packet_exists"),
                    "proof_exists": route.get("proof_exists"),
                    "has_claims_path": route
                        .get("claims_path")
                        .and_then(|claims_path| claims_path.as_str())
                        .is_some(),
                    "passed": route.get("passed"),
                })
            })
            .collect::<Vec<_>>(),
        "release_bundle": {
            "exists": value.pointer("/release_bundle/exists"),
            "passed": value.pointer("/release_bundle/passed"),
            "score": value.pointer("/release_bundle/score"),
            "route_count": value.pointer("/release_bundle/route_count"),
            "no_node_modules": value.pointer("/release_bundle/no_node_modules"),
            "finding_count": value
                .pointer("/release_bundle/findings")
                .and_then(|findings| findings.as_array())
                .map(|findings| findings.len())
                .unwrap_or_default(),
        },
        "dx_check": {
            "score": value.pointer("/dx_check/score"),
            "release_gate_score": value.pointer("/dx_check/release_gate_score"),
            "traffic": value.pointer("/dx_check/traffic"),
            "strict_forge_passed": value.pointer("/dx_check/strict_forge_passed"),
            "section_count": value.pointer("/dx_check/section_count"),
            "finding_count": value.pointer("/dx_check/finding_count"),
        },
        "finding_count": value
            .get("findings")
            .and_then(|findings| findings.as_array())
            .map(|findings| findings.len())
            .unwrap_or_default(),
        "honest_scope_count": value
            .get("honest_scope")
            .and_then(|scope| scope.as_array())
            .map(|scope| scope.len())
            .unwrap_or_default(),
    })
}

fn forge_adoption_claims_shape_contract(value: &serde_json::Value) -> serde_json::Value {
    let empty = Vec::new();
    let claims = value
        .get("claims")
        .and_then(|claims| claims.as_array())
        .unwrap_or(&empty);

    serde_json::json!({
        "version": value.get("version"),
        "route": value.get("route"),
        "claim_count": claims.len(),
        "claims": claims
            .iter()
            .map(|claim| {
                serde_json::json!({
                    "id": claim.get("id"),
                    "source_model": claim.get("source_model"),
                    "source_field": claim.get("source_field"),
                    "verification_status": claim.get("verification_status"),
                    "has_evidence": claim
                        .get("evidence")
                        .and_then(|evidence| evidence.as_str())
                        .map(|evidence| !evidence.trim().is_empty())
                        .unwrap_or(false),
                })
            })
            .collect::<Vec<_>>(),
    })
}

fn forge_adoption_proof_shape_contract(value: &serde_json::Value) -> serde_json::Value {
    let empty = Vec::new();
    let sections = value
        .pointer("/browser_packet/sections")
        .and_then(|sections| sections.as_array())
        .unwrap_or(&empty);

    serde_json::json!({
        "route": value.get("route"),
        "page": value.get("page"),
        "component_count": value
            .get("components")
            .and_then(|components| components.as_array())
            .map(|components| components.len())
            .unwrap_or_default(),
        "missing_component_count": value
            .get("missing_components")
            .and_then(|components| components.as_array())
            .map(|components| components.len())
            .unwrap_or_default(),
        "fallback": {
            "delivery_mode": value.pointer("/fallback/delivery_mode"),
            "script_count": value.pointer("/fallback/script_count"),
            "style_count": value.pointer("/fallback/style_count"),
            "optimized_bytes_positive": value
                .pointer("/fallback/optimized_bytes")
                .and_then(|bytes| bytes.as_u64())
                .map(|bytes| bytes > 0)
                .unwrap_or(false),
        },
        "packet": {
            "format": value.pointer("/packet/format"),
            "template_count": value.pointer("/packet/template_count"),
            "string_count": value.pointer("/packet/string_count"),
            "event_count": value.pointer("/packet/event_count"),
            "roundtrip_matches": value.pointer("/packet/roundtrip_matches"),
            "bytes_positive": value
                .pointer("/packet/bytes")
                .and_then(|bytes| bytes.as_u64())
                .map(|bytes| bytes > 0)
                .unwrap_or(false),
        },
        "browser_packet": {
            "format": value.pointer("/browser_packet/format"),
            "decoded_kind": value.pointer("/browser_packet/decoded_kind"),
            "section_count": value.pointer("/browser_packet/section_count"),
            "roundtrip_matches": value.pointer("/browser_packet/roundtrip_matches"),
            "sections": sections
                .iter()
                .map(|section| {
                    serde_json::json!({
                        "kind": section.get("kind"),
                        "encoding": section.get("encoding"),
                        "bytes_positive": section
                            .get("bytes")
                            .and_then(|bytes| bytes.as_u64())
                            .map(|bytes| bytes > 0)
                            .unwrap_or(false),
                    })
                })
                .collect::<Vec<_>>(),
        },
        "interaction_present": !value
            .get("interaction")
            .is_none_or(|interaction| interaction.is_null()),
        "forge_package_count": value
            .get("forge_packages")
            .and_then(|packages| packages.as_array())
            .map(|packages| packages.len())
            .unwrap_or_default(),
        "written": {
            "has_html_path": value.pointer("/written/html_path").and_then(|path| path.as_str()).is_some(),
            "has_packet_path": value.pointer("/written/packet_path").and_then(|path| path.as_str()).is_some(),
            "has_runtime_path": value.pointer("/written/runtime_path").and_then(|path| path.as_str()).is_some(),
            "has_claims_manifest_path": value.pointer("/written/claims_manifest_path").and_then(|path| path.as_str()).is_some(),
            "has_summary_path": value.pointer("/written/summary_path").and_then(|path| path.as_str()).is_some(),
        },
    })
}

fn json_path_ends_with(value: Option<&serde_json::Value>, suffix: &str) -> bool {
    value
        .and_then(|value| value.as_str())
        .map(|path| path.replace('\\', "/").ends_with(suffix))
        .unwrap_or(false)
}

fn read_forge_changelog_budget_evidence(
    repo_root: &Path,
    route_comparison: &serde_json::Value,
) -> serde_json::Value {
    let changelog_json = route_comparison
        .get("routes")
        .and_then(|routes| routes.as_array())
        .and_then(|routes| {
            routes.iter().find(|route| {
                route.get("route").and_then(|value| value.as_str()) == Some("/forge/changelog")
            })
        })
        .and_then(|route| route.get("json"))
        .and_then(|value| value.as_str())
        .expect("/forge/changelog route comparison JSON path");
    read_json_value(repo_root.join("benchmarks/reports").join(changelog_json))
}

fn forge_changelog_budget_shape_contract(value: &serde_json::Value) -> serde_json::Value {
    let budget = value
        .get("budget")
        .cloned()
        .unwrap_or(serde_json::Value::Null);

    serde_json::json!({
        "fixture_mode": value.get("fixture_mode"),
        "route": value.get("proof").and_then(|proof| proof.get("route")),
        "delivery": {
            "route_mode": value.get("delivery").and_then(|delivery| delivery.get("route_mode")),
            "packet_artifact_written": value
                .get("delivery")
                .and_then(|delivery| delivery.get("packet_artifact_written")),
            "runtime_asset_written": value
                .get("delivery")
                .and_then(|delivery| delivery.get("runtime_asset_written")),
        },
        "fallback": {
            "optimized_bytes": value
                .get("proof")
                .and_then(|proof| proof.get("fallback"))
                .and_then(|fallback| fallback.get("optimized_bytes")),
            "script_count": value
                .get("proof")
                .and_then(|proof| proof.get("fallback"))
                .and_then(|fallback| fallback.get("script_count")),
            "style_count": value
                .get("proof")
                .and_then(|proof| proof.get("fallback"))
                .and_then(|fallback| fallback.get("style_count")),
            "delivery_mode": value
                .get("proof")
                .and_then(|proof| proof.get("fallback"))
                .and_then(|fallback| fallback.get("delivery_mode")),
        },
        "http": {
            "resource_count": value.get("http").and_then(|http| http.get("resource_count")),
            "total_decoded_bytes": value
                .get("http")
                .and_then(|http| http.get("total_decoded_bytes")),
            "brotli_bytes": value
                .get("http")
                .and_then(|http| http.get("compression_estimate"))
                .and_then(|estimate| estimate.get("brotli_bytes")),
            "median_ms": value
                .get("http")
                .and_then(|http| http.get("route_timing_ms"))
                .and_then(|timing| timing.get("median")),
        },
        "chrome": {
            "enabled": value.get("chrome").and_then(|chrome| chrome.get("enabled")),
            "scripts": value.get("chrome").and_then(|chrome| chrome.get("scripts")),
            "load_event_ms": value
                .get("chrome")
                .and_then(|chrome| chrome.get("load_event_ms")),
            "dx_packet_applied": value
                .get("chrome")
                .and_then(|chrome| chrome.get("dx_packet_applied")),
            "interaction_works": value
                .get("chrome")
                .and_then(|chrome| chrome.get("interaction_works")),
        },
        "budget": {
            "profile": budget.get("profile"),
            "enforced": budget.get("enforced"),
            "passed": budget.get("passed"),
            "checks": budget
                .get("checks")
                .and_then(|checks| checks.as_array())
                .cloned()
                .unwrap_or_default()
                .iter()
                .map(|check| {
                    serde_json::json!({
                        "metric": check.get("metric"),
                        "value": check.get("value"),
                        "max": check.get("max"),
                        "unit": check.get("unit"),
                        "passed": check.get("passed"),
                    })
                })
                .collect::<Vec<_>>(),
        },
    })
}

fn forge_pages_bundle_shape_markdown(report: &DxForgePagesBundleVerificationReport) -> String {
    let mut output = format!(
        "# Forge Pages Bundle Shape\n\n- Artifacts: `{}`\n- Checks: `{}`\n- Score: `{}`\n- Passed: `{}`\n\n",
        report.artifacts.len(),
        report.checks.len(),
        report.score,
        report.passed
    );
    output.push_str("## Artifacts\n\n");
    output.push_str("| Artifact | JSON | Passed |\n");
    output.push_str("| --- | --- | --- |\n");
    for artifact in &report.artifacts {
        let valid_json = artifact
            .valid_json
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string());
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            artifact.name, valid_json, artifact.passed
        ));
    }
    output.push_str("\n## Checks\n\n");
    output.push_str("| Check | Artifacts | Passed | Message |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for check in &report.checks {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            check.route,
            check.artifacts.join(", "),
            check.passed,
            check.message
        ));
    }
    output
}

fn assert_forge_markdown_snapshot(name: &str, actual: &str, project: Option<&Path>) {
    let expected = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/forge-markdown")
            .join(name),
    )
    .unwrap_or_else(|error| panic!("read Forge Markdown snapshot `{name}`: {error}"));
    let actual = normalize_forge_markdown_snapshot(actual, project);
    assert_eq!(
        normalize_snapshot_line_endings(&expected),
        actual,
        "Forge Markdown snapshot `{name}` drifted"
    );
}

fn normalize_forge_markdown_snapshot(actual: &str, project: Option<&Path>) -> String {
    let mut normalized = normalize_snapshot_line_endings(actual);
    if let Some(project) = project {
        normalized = normalized.replace(&project.display().to_string(), "<project>");
    }
    normalized = normalized.replace('\\', "/");

    let mut replaced_generated_at = false;
    normalized
        .lines()
        .map(|line| {
            if !replaced_generated_at && line.starts_with("- Generated: `") {
                replaced_generated_at = true;
                "- Generated: `<generated-at>`"
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn assert_forge_release_manifest_markdown_snapshot(name: &str, actual: &str) {
    let expected = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/forge-markdown")
            .join(name),
    )
    .unwrap_or_else(|error| panic!("read Forge release manifest snapshot `{name}`: {error}"));
    assert_eq!(
        normalize_snapshot_line_endings(&expected),
        normalize_forge_release_manifest_markdown(actual),
        "Forge release manifest Markdown snapshot `{name}` drifted"
    );
}

fn assert_forge_pages_shape_markdown_snapshot(name: &str, actual: &str) {
    let expected = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/forge-markdown")
            .join(name),
    )
    .unwrap_or_else(|error| panic!("read Forge Pages shape snapshot `{name}`: {error}"));
    assert_eq!(
        normalize_snapshot_line_endings(&expected),
        normalize_snapshot_line_endings(actual),
        "Forge Pages shape Markdown snapshot `{name}` drifted"
    );
}

fn assert_forge_launch_changelog_markdown_snapshot(name: &str, actual: &str, project: &Path) {
    let expected = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/forge-markdown")
            .join(name),
    )
    .unwrap_or_else(|error| panic!("read Forge launch changelog snapshot `{name}`: {error}"));
    assert_eq!(
        normalize_snapshot_line_endings(&expected),
        normalize_forge_launch_changelog_markdown(actual, project),
        "Forge launch changelog Markdown snapshot `{name}` drifted"
    );
}

fn normalize_forge_release_manifest_markdown(actual: &str) -> String {
    normalize_snapshot_line_endings(actual)
        .lines()
        .map(|line| {
            if line.starts_with("Generated: `") {
                return "Generated: `<generated-at>`".to_string();
            }
            if line.starts_with("- Manifest digest: `") {
                return "- Manifest digest: `<digest>`".to_string();
            }
            if line.starts_with("- Artifact integrity digest: `") {
                return "- Artifact integrity digest: `<digest>`".to_string();
            }

            let mut cells = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect::<Vec<_>>();
            if cells.len() == 5
                && cells
                    .first()
                    .is_some_and(|artifact| artifact.starts_with('`'))
                && cells.last().is_some_and(|hash| hash.starts_with('`'))
            {
                cells[3] = "<bytes>".to_string();
                cells[4] = "`<blake3>`".to_string();
                return format!(
                    "| {} | {} | {} | {} | {} |",
                    cells[0], cells[1], cells[2], cells[3], cells[4]
                );
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn normalize_forge_launch_changelog_markdown(actual: &str, project: &Path) -> String {
    let project_path = project.display().to_string();
    normalize_snapshot_line_endings(actual)
        .replace(&project_path, "<project>")
        .replace('\\', "/")
        .lines()
        .map(|line| {
            if line.starts_with("- Generated: `") {
                return "- Generated: `<generated-at>`".to_string();
            }

            let mut cells = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect::<Vec<_>>();
            if cells.len() == 8
                && cells.first().is_some_and(|recorded| {
                    recorded.contains('T')
                        && (recorded.ends_with('Z') || recorded.contains("+00:00"))
                })
            {
                cells[0] = "<recorded-at>".to_string();
                return format!("| {} |", cells.join(" | "));
            }

            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn assert_forge_route_comparison_markdown_snapshot(name: &str, actual: &str) {
    let expected = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/forge-markdown")
            .join(name),
    )
    .unwrap_or_else(|error| {
        panic!("read Forge route comparison Markdown snapshot `{name}`: {error}")
    });
    assert_eq!(
        normalize_snapshot_line_endings(&expected),
        normalize_forge_route_comparison_markdown(actual),
        "Forge route comparison Markdown snapshot `{name}` drifted"
    );
}

fn assert_forge_release_history_markdown_snapshot(name: &str, actual: &str) {
    let expected = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/forge-markdown")
            .join(name),
    )
    .unwrap_or_else(|error| panic!("read Forge release history snapshot `{name}`: {error}"));
    assert_eq!(
        normalize_snapshot_line_endings(&expected),
        normalize_forge_release_history_markdown(actual),
        "Forge release history Markdown snapshot `{name}` drifted"
    );
}

fn normalize_forge_route_comparison_markdown(actual: &str) -> String {
    normalize_snapshot_line_endings(actual)
        .lines()
        .map(|line| {
            if line.starts_with("Generated:") {
                return "Generated: <generated-at>".to_string();
            }

            let mut cells = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect::<Vec<_>>();
            if cells.len() >= 16
                && cells
                    .first()
                    .is_some_and(|route| route.starts_with("/forge"))
            {
                let fixture = cells[2].clone();
                cells[3] = "<generated-at>".to_string();
                cells[15] = format!("[md](vertical-proof-history/<{fixture}>.md)");
                return format!("| {} |", cells.join(" | "));
            }

            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn normalize_forge_release_history_markdown(actual: &str) -> String {
    let mut normalized = normalize_snapshot_line_endings(actual)
        .lines()
        .map(|line| {
            if line.starts_with("Updated:") {
                return "Updated: <generated-at>".to_string();
            }
            if line.starts_with("| 20") && line.contains(" | 93 | ") {
                let cells = line
                    .trim()
                    .trim_matches('|')
                    .split('|')
                    .map(str::trim)
                    .collect::<Vec<_>>();
                if cells.len() >= 6 {
                    return format!("| <recorded-at> | {} |", cells[1..].join(" | "));
                }
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    while normalized.ends_with('\n') {
        normalized.pop();
    }
    normalized.push('\n');
    normalized
}

fn normalize_snapshot_line_endings(value: &str) -> String {
    let mut normalized = value.replace("\r\n", "\n");
    if !normalized.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}
