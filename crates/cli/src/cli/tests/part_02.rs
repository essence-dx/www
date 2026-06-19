#[test]
fn forge_launch_evidence_restart_checklist_writes_fresh_checklist_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-checklist-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-restart-checklist".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart checklist");

    let report =
        read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_restart_checklist"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["checklist"]["checklist_target"],
        "dx-cli-zed-restart-next-actions"
    );
    assert_eq!(
        report["freshness"]["checklist_not_older_than_restart_ledger"],
        true
    );
    assert!(report["checklist"]["lanes"]
        .as_array()
        .expect("restart checklist lanes")
        .iter()
        .any(|lane| lane["id"] == "runtime-approved" && lane["requires_permission"] == true));
    assert!(report["checks"]
        .as_array()
        .expect("restart checklist checks")
        .iter()
        .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true));
}

#[test]
fn forge_launch_evidence_restart_brief_writes_fresh_markdown() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-brief-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-restart-brief".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart brief");

    let markdown =
        fs::read_to_string(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE))
            .expect("restart brief markdown");
    assert!(markdown.contains("# DX Forge Launch Evidence Restart Brief"));
    assert!(markdown.contains("zed-openable-dx-restart-brief"));
    assert!(markdown.contains("runtime-approved"));
}

#[test]
fn forge_launch_evidence_restart_manifest_writes_fresh_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-manifest-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-restart-manifest".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart manifest");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_restart_manifest"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["manifest"]["manifest_target"],
        "dx-cli-zed-indexable-restart-manifest"
    );
    assert_eq!(
        report["freshness"]["manifest_not_older_than_restart_brief"],
        true
    );
    assert!(report["checks"]
        .as_array()
        .expect("restart manifest checks")
        .iter()
        .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true));
}

#[test]
fn forge_launch_evidence_restart_receipt_writes_fresh_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-receipt-app");
    write_launch_evidence_timeline_marker(
        &project,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
    );

    cli.cmd_forge(&[
        "launch-evidence-restart-receipt".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart receipt");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_restart_receipt");
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["receipt"]["receipt_target"],
        "latest-resumable-dx-zed-handoff"
    );
    assert_eq!(
        report["freshness"]["receipt_not_older_than_restart_manifest"],
        true
    );
    assert!(report["checks"]
        .as_array()
        .expect("restart receipt checks")
        .iter()
        .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true));
}

#[test]
fn forge_launch_evidence_restart_summary_writes_fresh_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-summary-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-restart-summary".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart summary");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_restart_summary");
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["summary"]["summary_target"],
        "terminal-friendly-dx-zed-restart-handoff"
    );
    assert_eq!(
        report["freshness"]["summary_not_older_than_restart_receipt"],
        true
    );
    assert!(report["checks"]
        .as_array()
        .expect("restart summary checks")
        .iter()
        .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true));
}

#[test]
fn forge_launch_evidence_restart_snapshot_writes_fresh_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-snapshot-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-restart-snapshot".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart snapshot");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_restart_snapshot"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["snapshot"]["snapshot_target"],
        "latest-openable-dx-zed-restart-file"
    );
    assert_eq!(
        report["freshness"]["snapshot_not_older_than_restart_summary"],
        true
    );
    assert!(report["checks"]
        .as_array()
        .expect("restart snapshot checks")
        .iter()
        .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true));
}

#[test]
fn forge_launch_evidence_restart_dispatch_writes_fresh_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-dispatch-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-restart-dispatch".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart dispatch");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_restart_dispatch"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["dispatch"]["dispatch_target"],
        "one-command-next-worker-dispatch-card"
    );
    assert_eq!(
        report["freshness"]["dispatch_not_older_than_restart_snapshot"],
        true
    );
    assert!(report["checks"]
        .as_array()
        .expect("restart dispatch checks")
        .iter()
        .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true));
}

#[test]
fn forge_launch_evidence_restart_closeout_writes_fresh_markdown() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-closeout-app");
    write_launch_evidence_timeline_marker(
        &project,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE,
    );

    cli.cmd_forge(&[
        "launch-evidence-restart-closeout".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart closeout");

    let closeout_path = project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE);
    let closeout = fs::read_to_string(closeout_path).expect("restart closeout markdown");
    assert!(closeout.contains("# DX Forge Launch Evidence Restart Closeout"));
    assert!(closeout.contains("final-friday-essencefromexistence-closeout-actions"));
    assert!(closeout.contains("Final Operator Actions"));
    assert!(closeout.contains("No execution: `true`"));
}

#[test]
fn forge_launch_evidence_restart_signoff_writes_fresh_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-signoff-app");
    write_launch_evidence_timeline_marker(
        &project,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE,
    );

    cli.cmd_forge(&[
        "launch-evidence-restart-signoff".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart signoff");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_restart_signoff");
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["signoff"]["signoff_target"],
        "friday-essencefromexistence-acceptance-receipt"
    );
    assert_eq!(
        report["freshness"]["signoff_not_older_than_restart_closeout"],
        true
    );
    assert!(report["checks"]
        .as_array()
        .expect("restart signoff checks")
        .iter()
        .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true));
}

#[test]
fn forge_launch_evidence_acceptance_index_writes_fresh_markdown() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("acceptance-index-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-acceptance-index".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write acceptance index");

    let index_path = project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE);
    let index = fs::read_to_string(index_path).expect("acceptance index markdown");
    assert!(index.contains("# DX Forge Launch Evidence Acceptance Index"));
    assert!(index.contains("friday-final-handoff-index"));
    assert!(index.contains("restart-signoff"));
    assert!(index.contains("No execution: `true`"));
}

#[test]
fn forge_launch_evidence_acceptance_digest_writes_fresh_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("acceptance-digest-app");
    write_launch_evidence_timeline_marker(
        &project,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE,
    );

    cli.cmd_forge(&[
        "launch-evidence-acceptance-digest".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write acceptance digest");

    let report =
        read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_DIGEST_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_acceptance_digest"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["digest"]["digest_target"],
        "friday-terminal-final-status-line"
    );
    assert_eq!(
        report["freshness"]["digest_not_older_than_acceptance_index"],
        true
    );
    assert!(report["digest"]["terminal_status_line"]
        .as_str()
        .expect("terminal status line")
        .contains("Friday final handoff"));
}

#[test]
fn forge_launch_evidence_friday_baton_writes_fresh_markdown() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("friday-baton-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_DIGEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE,
        NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-friday-baton".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write Friday baton");

    let baton_path = project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_FRIDAY_BATON_FILE);
    let baton = fs::read_to_string(baton_path).expect("Friday baton markdown");
    assert!(baton.contains("# DX Forge Launch Evidence Friday Baton"));
    assert!(baton.contains("friday-orchestrator-final-handoff"));
    assert!(baton.contains("acceptance-digest"));
    assert!(baton.contains("No execution: `true`"));
}

fn launch_evidence_timeline_step_paths() -> [&'static str; 8] {
    [
        NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
    ]
}

fn write_launch_evidence_timeline_marker(project: &Path, path: &str) {
    let path = project.join(path);
    fs::create_dir_all(path.parent().expect("launch evidence parent"))
        .expect("launch evidence parent");
    fs::write(path, "{}").expect("launch evidence marker");
}

#[test]
fn dx_dev_renders_new_app_template_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("dev-app").expect("dx new");

    let project = dir.path().join("dev-app");
    let translations = HashMap::new();
    let (status, content_type, html) = Cli::handle_request(&project, "/", &translations);

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "text/html; charset=utf-8");
    assert!(html.contains("The web framework for source-owned products."));
    assert!(html.contains("data-dx-route=\"/\""));
    assert!(html.contains("data-dx-hot-reload-target=\"route:/\""));
    assert!(html.contains("data-dx-node-modules=\"forbidden\""));
    assert!(html.contains("<html lang=\"en\" data-theme=\"dark\">"));
    assert!(html.contains("<link rel=\"apple-touch-icon\" href=\"/icon.svg\">"));
    assert!(html.contains("src=\"/logo.svg\""));
    assert!(html.contains("images.unsplash.com/photo-1500530855697-b586d89ba3ee"));
    assert!(html.contains("data-dx-scroll-system=\"native\""));
    assert!(!html.contains("data-template-scrollbar"));
    assert!(html.contains("template-menu-icon"));
    assert!(!html.contains("data-dx-scene-canvas"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_dev_renders_app_route_from_compiled_page_graph() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("compiled-app").expect("dx new");

    let project = dir.path().join("compiled-app");
    let translations = HashMap::new();
    let (_status, _content_type, html) = Cli::handle_request(&project, "/", &translations);

    assert!(html.contains("data-dx-page-graph=\"app/layout\""));
    assert!(html.contains("data-dx-layouts=\"1\""));
    assert!(html.contains("data-dx-boundaries=\"loading,error,not-found\""));
    let packet_section_count = html
        .split("data-dx-packet-sections=\"")
        .nth(1)
        .and_then(|value| value.split('"').next())
        .and_then(|value| value.parse::<usize>().ok())
        .expect("packet section metadata");
    assert!(packet_section_count > 0);
}

#[test]
fn dx_dev_prefers_app_router_page_over_legacy_static_shadow_route() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("tsx-shadow-app").expect("dx new");

    let project = dir.path().join("tsx-shadow-app");
    fs::create_dir_all(project.join("pages")).expect("pages dir");
    fs::write(
        project.join("pages/dashboard.html"),
        r#"<main>stale legacy dashboard fallback</main>"#,
    )
    .expect("legacy static dashboard");

    let translations = HashMap::new();
    let (status, content_type, html) = Cli::handle_request(&project, "/dashboard", &translations);

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "text/html; charset=utf-8");
    assert!(html.contains("Template dashboard"));
    assert!(html.contains("data-dx-route=\"/dashboard\""));
    assert!(!html.contains("stale legacy dashboard fallback"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_dev_reflects_tsx_source_edits_and_updates_reload_token() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("tsx-refresh-app").expect("dx new");

    let project = dir.path().join("tsx-refresh-app");
    let translations = HashMap::new();
    let initial_token = Cli::dev_project_reload_token(&project);
    let (status, content_type, initial_html) =
        Cli::handle_request(&project, "/dashboard", &translations);

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "text/html; charset=utf-8");
    assert!(initial_html.contains("Template dashboard"));

    let dashboard_component = project.join("components/dashboard/TemplateDashboard.tsx");
    let source = fs::read_to_string(&dashboard_component).expect("dashboard component source");
    std::thread::sleep(std::time::Duration::from_millis(20));
    fs::write(
        &dashboard_component,
        source.replace("Template dashboard", "Hot reload dashboard proof"),
    )
    .expect("edit dashboard component");

    let updated_token = Cli::dev_project_reload_token(&project);
    let (status, content_type, updated_html) =
        Cli::handle_request(&project, "/dashboard", &translations);

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "text/html; charset=utf-8");
    assert_ne!(updated_token, initial_token);
    assert!(updated_html.contains("Hot reload dashboard proof"));
    assert!(!updated_html.contains("stale legacy dashboard fallback"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_dev_executes_source_owned_app_route_handler() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("api-app").expect("dx new");

    let project = dir.path().join("api-app");
    fs::write(
        project.join("app/api/health/route.ts"),
        r#"export function GET() {
  return {
    ok: true,
    runtime: "custom-dx-www",
    count: 7,
    status: 202,
  };
}
"#,
    )
    .expect("route handler");

    let translations = HashMap::new();
    let (status, content_type, body) = Cli::handle_request(&project, "/api/health", &translations);
    let response: serde_json::Value = serde_json::from_str(&body).expect("route json");

    assert_eq!(status, "202 Accepted");
    assert_eq!(content_type, "application/json; charset=utf-8");
    assert_eq!(response["ok"], true);
    assert_eq!(response["runtime"], "custom-dx-www");
    assert_eq!(response["count"], 7);
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_dev_route_handler_parses_post_json_body_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("post-api-app").expect("dx new");

    let project = dir.path().join("post-api-app");
    let route_dir = project.join("app/api/echo");
    fs::create_dir_all(&route_dir).expect("route dir");
    fs::write(
        route_dir.join("route.ts"),
        r#"export async function POST(request: Request) {
  const body = await request.json();
  return {
    ok: true,
    method: request.method,
    path: request.path,
    payload: body,
    name: body.name,
    count: body.count,
    status: 201,
  };
}
"#,
    )
    .expect("route handler");

    let translations = HashMap::new();
    let request = concat!(
        "POST /api/echo?source=dev HTTP/1.1\r\n",
        "Host: localhost\r\n",
        "Content-Type: application/json\r\n",
        "Content-Length: 24\r\n",
        "\r\n",
        "{\"name\":\"Ada\",\"count\":2}"
    );
    let (status, content_type, body) = Cli::handle_http_request(&project, request, &translations);
    let response: serde_json::Value = serde_json::from_str(&body).expect("route json");

    assert_eq!(status, "201 Created");
    assert_eq!(content_type, "application/json; charset=utf-8");
    assert_eq!(response["ok"], true);
    assert_eq!(response["method"], "POST");
    assert_eq!(response["path"], "/api/echo");
    assert_eq!(response["payload"]["name"], "Ada");
    assert_eq!(response["name"], "Ada");
    assert_eq!(response["count"], 2);
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_dev_executes_dynamic_app_api_route_handler() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("dynamic-api-app").expect("dx new");

    let project = dir.path().join("dynamic-api-app");
    let route_dir = project.join("app/api/trpc/[trpc]");
    fs::create_dir_all(&route_dir).expect("route dir");
    fs::write(
        route_dir.join("route.ts"),
        r#"export function GET() {
  return {
    ok: true,
    runtime: "dynamic-api-route",
    status: 202,
  };
}
"#,
    )
    .expect("route handler");

    let translations = HashMap::new();
    let (status, content_type, body) =
        Cli::handle_request(&project, "/api/trpc/health", &translations);
    let response: serde_json::Value = serde_json::from_str(&body).expect("route json");

    assert_eq!(status, "202 Accepted");
    assert_eq!(content_type, "application/json; charset=utf-8");
    assert_eq!(response["ok"], true);
    assert_eq!(response["runtime"], "dynamic-api-route");
    assert!(!project.join("node_modules").exists());
}

#[test]
#[cfg(not(feature = "dev-server"))]
fn dx_dev_http_response_frame_separates_headers_from_body() {
    let response = DxCliHttpResponse {
        status: "200 OK".to_string(),
        content_type: "text/html; charset=utf-8".to_string(),
        headers: BTreeMap::new(),
        body: "<!doctype html><main>DX-WWW</main>".to_string(),
    };
    let headers = Cli::dev_response_headers(&response, response.body.len());
    let frame = format!(
        "HTTP/1.1 {}\r\n{}\r\n\r\n{}",
        response.status, headers, response.body
    );

    assert!(frame.contains("\r\n\r\n<!doctype html>"));
    assert!(!headers.contains("<!doctype html>"));
    assert!(headers.contains("Content-Length: 34"));
}

#[test]
fn dx_dev_options_accept_host_port_and_no_hot_reload() {
    let config = DxConfig::default();
    let args = vec![
        "--host".to_string(),
        "localhost".to_string(),
        "--port".to_string(),
        "3042".to_string(),
        "--no-hot-reload".to_string(),
    ];

    let options = parse_dev_options(&args, &config).expect("dev options");

    assert_eq!(options.host, "127.0.0.1");
    assert_eq!(options.port, 3042);
    assert!(!options.hot_reload);
    assert!(options.port_explicit);
}

#[test]
#[cfg(not(feature = "dev-server"))]
fn dx_dev_response_cache_key_only_allows_safe_gets() {
    let get_request = DxCliHttpRequest {
        method: "GET".to_string(),
        path: "/?theme=dark".to_string(),
        headers: BTreeMap::new(),
        body: serde_json::Value::Null,
    };
    let head_request = DxCliHttpRequest {
        method: "HEAD".to_string(),
        path: "/styles/global.css".to_string(),
        headers: BTreeMap::new(),
        body: serde_json::Value::Null,
    };
    let api_request = DxCliHttpRequest {
        method: "GET".to_string(),
        path: "/api/session".to_string(),
        headers: BTreeMap::new(),
        body: serde_json::Value::Null,
    };
    let action_request = DxCliHttpRequest {
        method: "POST".to_string(),
        path: "/_dx/server-action/counter".to_string(),
        headers: BTreeMap::new(),
        body: serde_json::Value::Null,
    };

    assert_eq!(
        Cli::dev_response_cache_key(&get_request),
        Some("GET /?theme=dark".to_string())
    );
    assert_eq!(
        Cli::dev_response_cache_key(&head_request),
        Some("HEAD /styles/global.css".to_string())
    );
    assert_eq!(Cli::dev_response_cache_key(&api_request), None);
    assert_eq!(Cli::dev_response_cache_key(&action_request), None);
}

#[test]
fn dx_dev_auto_selects_next_port_when_default_is_busy() {
    let mut reserved = None;
    for port in 43100..43200 {
        let first = std::net::TcpListener::bind(("127.0.0.1", port));
        let second = std::net::TcpListener::bind(("127.0.0.1", port + 1));
        match (first, second) {
            (Ok(first), Ok(second)) => {
                drop(second);
                reserved = Some((port, first));
                break;
            }
            (first, second) => {
                drop(first);
                drop(second);
            }
        }
    }
    let (busy_port, _busy_listener) = reserved.expect("two adjacent free ports");

    let binding = bind_dev_listener("127.0.0.1", busy_port, false).expect("fallback port");
    let (listener, selected_port) = match binding {
        DxDevServerBinding::Bound { listener, port } => (listener, port),
        DxDevServerBinding::Existing(existing) => {
            panic!(
                "expected fallback port, found existing server at {}",
                existing.url
            )
        }
    };

    assert_eq!(selected_port, busy_port + 1);
    drop(listener);
}

#[test]
fn dx_dev_explicit_busy_port_fails_without_fallback() {
    let busy_listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("reserved port");
    let busy_port = busy_listener.local_addr().expect("local addr").port();

    let error = bind_dev_listener("127.0.0.1", busy_port, true)
        .expect_err("explicit busy port should fail");

    assert!(format!("{error}").contains("already in use"));
}

#[test]
fn dx_dev_hot_reload_script_is_injected_into_html() {
    let dir = tempdir().expect("tempdir");
    let response = DxCliHttpResponse {
        status: "200 OK".to_string(),
        content_type: "text/html; charset=utf-8".to_string(),
        headers: BTreeMap::new(),
        body: "<!doctype html><html><body><main>DX</main></body></html>".to_string(),
    };

    let response = with_dev_hot_reload(dir.path(), response);

    assert!(response.body.contains("data-dx-hot-reload"));
    assert!(response.body.contains("/_dx/hot-reload/version"));
    assert!(response.body.contains("DX-WWW error overlay"));
    assert!(response.body.contains("__DX_SHOW_ERROR__"));
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
}

#[test]
#[cfg(not(feature = "dev-server"))]
fn dx_dev_head_wire_response_keeps_content_length_without_body() {
    let response = DxCliHttpResponse {
        status: "200 OK".to_string(),
        content_type: "text/plain; charset=utf-8".to_string(),
        headers: BTreeMap::new(),
        body: "hello dx".to_string(),
    };

    let frame =
        String::from_utf8(Cli::dev_wire_response_bytes(&response, true)).expect("wire response");

    assert!(frame.contains("HTTP/1.1 200 OK"));
    assert!(frame.contains("Content-Length: 8"));
    assert!(frame.ends_with("\r\n\r\n"));
    assert!(!frame.contains("hello dx"));
}

#[test]
fn dx_dev_route_handler_supports_next_response_cookie_and_redirect_helpers() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("next-response-api-app").expect("dx new");

    let project = dir.path().join("next-response-api-app");
    let route_dir = project.join("app/api/session");
    fs::create_dir_all(&route_dir).expect("route dir");
    fs::write(
        route_dir.join("route.ts"),
        r#"import { NextResponse } from "next/server";

export function GET(request: Request) {
  const theme = request.cookies.get("theme")?.value ?? "light";

  return NextResponse.json(
    {
      ok: true,
      theme,
    },
    {
      status: 201,
      headers: {
        "cache-control": "no-store",
        "x-dx-mode": "safe",
      },
    },
  );
}

export function POST() {
  return NextResponse.redirect("/login", 307);
}
"#,
    )
    .expect("route handler");

    let translations = HashMap::new();
    let get_request = concat!(
        "GET /api/session HTTP/1.1\r\n",
        "Host: localhost\r\n",
        "Cookie: theme=dark; session=abc\r\n",
        "\r\n"
    );
    let get_response = Cli::handle_http_response(&project, get_request, &translations);
    let response: serde_json::Value = serde_json::from_str(&get_response.body).expect("route json");

    assert_eq!(get_response.status, "201 Created");
    assert_eq!(get_response.content_type, "application/json; charset=utf-8");
    assert_eq!(
        get_response
            .headers
            .get("cache-control")
            .map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        get_response.headers.get("x-dx-mode").map(String::as_str),
        Some("safe")
    );
    assert_eq!(response["ok"], true);
    assert_eq!(response["theme"], "dark");

    let post_request = concat!(
        "POST /api/session HTTP/1.1\r\n",
        "Host: localhost\r\n",
        "\r\n"
    );
    let post_response = Cli::handle_http_response(&project, post_request, &translations);
    let redirect: serde_json::Value =
        serde_json::from_str(&post_response.body).expect("redirect json");

    assert_eq!(post_response.status, "307 Temporary Redirect");
    assert_eq!(
        post_response.content_type,
        "application/json; charset=utf-8"
    );
    assert_eq!(
        post_response.headers.get("location").map(String::as_str),
        Some("/login")
    );
    assert_eq!(redirect["redirect"], "/login");
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_dev_resolves_route_groups_dynamic_segments_and_search_params() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("route-app").expect("dx new");

    let project = dir.path().join("route-app");
    let route_dir = project.join("app/(marketing)/blog/[slug]");
    fs::create_dir_all(&route_dir).expect("dynamic route dir");
    fs::write(
        route_dir.join("page.tsx"),
        r#"export const metadata = {
  title: "Blog Detail",
  description: "Dynamic blog article",
};

export default function Page({ params, searchParams }) {
  return (
    <main className="dx-shell">
      <h1>Blog Detail</h1>
      <p>{params.slug}</p>
      <p>{searchParams.ref}</p>
    </main>
  );
}
"#,
    )
    .expect("dynamic page");

    let translations = HashMap::new();
    let (status, content_type, body) =
        Cli::handle_request(&project, "/blog/hello-world?ref=dx", &translations);

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "text/html; charset=utf-8");
    assert!(body.contains("<title>Blog Detail</title>"));
    assert!(body.contains(r#"data-dx-route-params="slug=hello-world""#));
    assert!(body.contains(r#"data-dx-search-params="ref=dx""#));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_dev_strips_query_strings_for_pages_and_static_assets() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().to_path_buf();
    fs::create_dir_all(project.join("pages")).expect("pages dir");
    fs::create_dir_all(project.join("styles")).expect("styles dir");
    fs::write(
        project.join("pages/index.html"),
        r#"<template><main data-test="query-root">Query root route</main></template>"#,
    )
    .expect("page");
    fs::write(
        project.join("styles/dx-landing.css"),
        ".query-root { color: hotpink; }",
    )
    .expect("style");

    let translations = HashMap::new();
    let (status, content_type, body) = Cli::handle_request(&project, "/?qa=1", &translations);

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "text/html; charset=utf-8");
    assert!(body.contains("Query root route"));

    let style_response = Cli::handle_http_response(
        &project,
        "GET /styles/dx-landing.css?v=test HTTP/1.1\r\nHost: localhost\r\n\r\n",
        &translations,
    );

    assert_eq!(style_response.status, "200 OK");
    assert_eq!(style_response.content_type, "text/css; charset=utf-8");
    assert!(style_response.body.contains("hotpink"));
    assert_eq!(
        style_response
            .headers
            .get("cache-control")
            .map(String::as_str),
        Some("no-store")
    );
}

#[test]
fn dx_dev_reload_token_tracks_source_and_forge_manifests() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path();
    fs::create_dir_all(project.join("pages")).expect("pages dir");
    fs::create_dir_all(project.join("styles")).expect("styles dir");
    fs::create_dir_all(project.join(".dx/forge/route-discovery")).expect("forge routes");
    fs::write(project.join("pages/index.html"), "<template>one</template>").expect("page");
    fs::write(
        project.join("styles/dx-landing.css"),
        "body { color: white; }",
    )
    .expect("style");

    let first = Cli::dev_project_reload_token(project);
    std::thread::sleep(std::time::Duration::from_millis(20));
    fs::write(
        project.join("styles/dx-landing.css"),
        "body { color: black; }",
    )
    .expect("style update");
    let after_style = Cli::dev_project_reload_token(project);
    std::thread::sleep(std::time::Duration::from_millis(20));
    fs::write(
        project.join(".dx/forge/route-discovery/conversion-routes.json"),
        r#"{"routes":["/"]}"#,
    )
    .expect("forge manifest");
    let after_forge = Cli::dev_project_reload_token(project);

    assert_ne!(after_style, first);
    assert_ne!(after_forge, after_style);
}

#[test]
fn dx_dev_serves_project_contract_hints_without_source_writes() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("dx"), "project.name = hinted-dev-app\n").expect("config");
    fs::create_dir_all(dir.path().join("app")).expect("app");
    fs::write(
        dir.path().join("app/page.tsx"),
        "export default function Page() { return <main />; }\n",
    )
    .expect("page");

    let translations = HashMap::new();
    let (status, content_type, body) = Cli::handle_request(
        &dir.path().to_path_buf(),
        "/.dx/project-contract-hints.json",
        &translations,
    );

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let artifact: serde_json::Value = serde_json::from_str(&body).expect("hint json");
    assert_eq!(artifact["source_files_written"], false);
    assert_eq!(artifact["auto_write_on_save"], false);
    assert!(artifact["audiences"]
        .as_array()
        .expect("audiences")
        .iter()
        .any(|audience| audience == "dev-server"));
    assert!(artifact["audiences"]
        .as_array()
        .expect("audiences")
        .iter()
        .any(|audience| audience == "lsp"));
    assert!(artifact["hints"]
        .as_array()
        .expect("hints")
        .iter()
        .any(|hint| hint["code"] == "project-contract-missing-components-dir"));
    assert!(!dir.path().join("components").exists());
    assert!(!dir.path().join("server").exists());
    assert!(!dir.path().join("styles").exists());
    assert!(!dir.path().join(".dx/forge/hints").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_lsp_hints_report_import_boundary_and_provenance_diagnostics() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("dx"), "project.name = diagnostic-app\n").expect("config");
    fs::create_dir_all(dir.path().join("app")).expect("app");
    fs::create_dir_all(dir.path().join("server")).expect("server");
    fs::write(
        dir.path().join("app/page.tsx"),
        r#""use client";
import leftPad from "left-pad";
import { recordWelcomeView } from "../server/actions";

export default function Page() {
  return <button onClick={recordWelcomeView}>{leftPad("DX", 3)}</button>;
}
"#,
    )
    .expect("page");
    fs::write(
        dir.path().join("server/actions.ts"),
        r#""use client";

export async function recordWelcomeView() {
  return { ok: true };
}
"#,
    )
    .expect("actions");

    let translations = HashMap::new();
    let (status, content_type, body) = Cli::handle_request(
        &dir.path().to_path_buf(),
        "/.dx/lsp/project-contract-hints.json",
        &translations,
    );

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let artifact: serde_json::Value = serde_json::from_str(&body).expect("hint json");
    assert_eq!(artifact["source_files_written"], false);
    assert_eq!(artifact["auto_write_on_save"], false);
    assert!(artifact["diagnostics"]
        .as_array()
        .expect("diagnostics")
        .iter()
        .any(
            |diagnostic| diagnostic["code"] == "lsp-import-requires-forge-gate"
                && diagnostic["path"] == "app/page.tsx"
        ));
    assert!(artifact["diagnostics"]
        .as_array()
        .expect("diagnostics")
        .iter()
        .any(|diagnostic| diagnostic["code"] == "lsp-client-imports-server-file"));
    assert!(artifact["diagnostics"]
        .as_array()
        .expect("diagnostics")
        .iter()
        .any(
            |diagnostic| diagnostic["code"] == "lsp-server-file-client-boundary"
                && diagnostic["path"] == "server/actions.ts"
        ));
    assert!(artifact["diagnostics"]
        .as_array()
        .expect("diagnostics")
        .iter()
        .any(|diagnostic| diagnostic["code"] == "lsp-forge-provenance-missing"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_build_emits_compiled_app_route_artifacts_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("build-app").expect("dx new");

    let project = dir.path().join("build-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    assert!(project.join(".dx/www/output/app/index.html").exists());
    assert!(project.join(".dx/www/output/index.html").exists());
    assert_eq!(
        fs::read_to_string(project.join(".dx/www/output/index.html")).expect("native entrypoint"),
        fs::read_to_string(project.join(".dx/www/output/app/index.html")).expect("app html"),
        "root index.html should mirror the root app route so native shells can load frontendDist"
    );
    assert!(!project.join(".dx/www/output/app/index.dxpk").exists());
    assert!(project.join(".dx/www/output/app/page-graph.json").exists());
    assert!(!project
        .join(".dx/www/output/app/docs/[[...slug]]/server-data.json")
        .exists());
    assert!(!project
        .join(".dx/www/output/server-contracts.json")
        .exists());
    assert!(!project
        .join(".dx/www/output/server-action-protocols.json")
        .exists());
    assert!(project
        .join(".dx/www/output/import-resolution.json")
        .exists());
    assert!(project.join(".dx/www/output/deploy-adapter.json").exists());
    assert!(project.join(".dx/www/output/rollback.json").exists());
    assert!(!project.join("node_modules").exists());

    let manifest = read_json_value(project.join(".dx/www/output/manifest.json"));
    assert_eq!(manifest["app_routes_compiled"], 1);
    assert_eq!(manifest["server_data_entries_compiled"], 0);
    assert_eq!(manifest["server_contracts_compiled"], 0);
    assert_eq!(manifest["server_action_protocols_compiled"], 0);
    assert_eq!(manifest["deploy_adapter_emitted"], true);
    assert_eq!(manifest["import_resolutions_compiled"], 1);
    assert_eq!(manifest["node_modules_required"], false);

    let import_resolutions = read_json_value(project.join(".dx/www/output/import-resolution.json"));
    assert!(import_resolutions
        .as_array()
        .expect("import resolutions")
        .iter()
        .any(
            |resolution| resolution["specifier"] == "../styles/globals.css"
                && resolution["requires_node_modules"] == false
        ));
    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert_eq!(deploy["adapter"], "dx-www-static-hosting");
    assert_eq!(deploy["no_node_modules_required"], true);
    let root_route = deploy["routes"]
        .as_array()
        .expect("routes")
        .iter()
        .find(|route| route["path"] == "/")
        .expect("root route");
    assert_eq!(root_route["html"], "app/index.html");
    assert_eq!(root_route.get("packet"), None);
    assert_eq!(root_route.get("server_data"), None);
    assert!(deploy["immutable_assets"]
        .as_array()
        .expect("immutable assets")
        .iter()
        .all(|asset| asset["path"] != "app/index.dxpk"));
    assert!(deploy["health_checks"]
        .as_array()
        .expect("health checks")
        .is_empty());
    assert_eq!(deploy["build_manifest"]["path"], "manifest.json");
    assert!(deploy["build_manifest"]["hash"]
        .as_str()
        .expect("manifest hash")
        .starts_with("blake3:"));
    assert_eq!(
        deploy["build_manifest"]["signature_required_for_release"],
        true
    );
    assert_eq!(deploy["rollback"]["metadata_path"], "rollback.json");
    let provider = read_json_value(project.join(".dx/www/output/provider-adapter.dx-cloud.json"));
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .all(|artifact| artifact["path"] != "app/index.dxpk"
            && artifact["path"] != "app/docs/[[...slug]]/server-data.json"));
    let rollback = read_json_value(project.join(".dx/www/output/rollback.json"));
    assert_eq!(rollback["manifest_hash"], deploy["build_manifest"]["hash"]);
}

#[test]
fn dx_build_mirrors_public_root_assets_for_native_static_entrypoints() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("native-public-assets-app").expect("dx new");

    let project = dir.path().join("native-public-assets-app");
    fs::write(
        project.join("public/mobile-companion-runtime.js"),
        "window.DXMobileCompanionRuntime = { nativeStaticReady: true };",
    )
    .expect("mobile runtime");
    fs::write(
        project.join("public/manifest.json"),
        r#"{"name":"DX Mobile public manifest"}"#,
    )
    .expect("public manifest");

    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    assert!(project.join(".dx/www/output/public/logo.svg").is_file());
    assert!(project.join(".dx/www/output/logo.svg").is_file());
    assert_eq!(
        fs::read(project.join(".dx/www/output/logo.svg")).expect("root logo"),
        fs::read(project.join(".dx/www/output/public/logo.svg")).expect("public logo"),
        "root public asset aliases should match their public/ provenance copies"
    );
    assert!(project
        .join(".dx/www/output/public/mobile-companion-runtime.js")
        .is_file());
    assert!(project
        .join(".dx/www/output/mobile-companion-runtime.js")
        .is_file());
    assert_eq!(
        fs::read_to_string(project.join(".dx/www/output/mobile-companion-runtime.js"))
            .expect("root mobile runtime"),
        fs::read_to_string(project.join(".dx/www/output/public/mobile-companion-runtime.js"))
            .expect("public mobile runtime")
    );

    let build_manifest = read_json_value(project.join(".dx/www/output/manifest.json"));
    assert_eq!(build_manifest["app_routes_compiled"], 1);
    assert_eq!(
        fs::read_to_string(project.join(".dx/www/output/public/manifest.json"))
            .expect("public manifest"),
        r#"{"name":"DX Mobile public manifest"}"#
    );
}

#[test]
fn dx_build_emits_true_app_router_execution_fixture_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("app-router-fixture").expect("dx new");

    let project = dir.path().join("app-router-fixture");
    let route_dir = project.join("app/(workspace)/dashboard/[team]/settings");
    let dashboard_dir = project.join("app/(workspace)/dashboard");
    fs::create_dir_all(&route_dir).expect("route dir");
    fs::write(
        project.join("app/template.tsx"),
        r#"export default function RootTemplate({ children }) {
  return <div data-template="root">{children}</div>;
}
"#,
    )
    .expect("root template");
    fs::write(
        dashboard_dir.join("layout.tsx"),
        r#"export const metadata = {
  title: "Dashboard shell",
  description: "Workspace dashboard",
};

export default function DashboardLayout({ children }) {
  return <section data-shell="dashboard">{children}</section>;
}
"#,
    )
    .expect("dashboard layout");
    fs::write(
        dashboard_dir.join("template.tsx"),
        r#"export default function DashboardTemplate({ children }) {
  return <div data-template="dashboard">{children}</div>;
}
"#,
    )
    .expect("dashboard template");
    fs::write(
        dashboard_dir.join("loading.tsx"),
        r#"export default function Loading() {
  return <p>Loading dashboard</p>;
}
"#,
    )
    .expect("dashboard loading");
    fs::write(
        dashboard_dir.join("error.tsx"),
        r#""use client";

export default function Error({ error }) {
  return <p>{error.message}</p>;
}
"#,
    )
    .expect("dashboard error");
    fs::write(
        dashboard_dir.join("not-found.tsx"),
        r#"export default function NotFound() {
  return <p>Dashboard missing</p>;
}
"#,
    )
    .expect("dashboard not found");
    fs::write(
        route_dir.join("page.tsx"),
        r#"export const metadata = {
  title: "Team Settings",
  description: "Team settings page",
  canonical: "/dashboard/acme/settings",
};

export default function Page({ params, searchParams }) {
  return (
    <main>
      <h1>Team Settings</h1>
      <p>{params.team}</p>
      <p>{searchParams.tab}</p>
    </main>
  );
}
"#,
    )
    .expect("nested page");

    let project_cli = Cli::with_cwd(project.clone());
    project_cli
        .cmd_imports(&["sync".to_string()])
        .expect("dx imports sync");
    project_cli.cmd_build().expect("dx build");

    let execution = read_json_value(
        project.join(".dx/www/output/app/dashboard/[team]/settings/app-router-execution.json"),
    );
    assert_eq!(execution["route"], "/dashboard/[team]/settings");
    assert_eq!(
        execution["source_path"],
        "app/(workspace)/dashboard/[team]/settings/page.tsx"
    );
    assert_eq!(execution["runtime"], "source-owned-app-router");
    assert_eq!(execution["node_modules_required"], false);
    assert_eq!(execution["node_modules_present"], false);
    assert!(execution["route_groups"]
        .as_array()
        .expect("route groups")
        .iter()
        .any(|group| group == "(workspace)"));
    assert!(execution["dynamic_segments"]
        .as_array()
        .expect("dynamic segments")
        .iter()
        .any(|segment| segment["name"] == "team" && segment["kind"] == "dynamic"));
    assert!(execution["page_props"]["params"]
        .as_array()
        .expect("params")
        .iter()
        .any(|param| param == "team"));
    assert!(execution["page_props"]["search_params"]
        .as_array()
        .expect("search params")
        .iter()
        .any(|param| param == "tab"));
    assert_eq!(execution["effective_metadata"]["title"], "Team Settings");
    assert_eq!(
        execution["effective_metadata"]["canonical"],
        "/dashboard/acme/settings"
    );
    assert!(execution["layouts"]
        .as_array()
        .expect("layouts")
        .iter()
        .any(|layout| layout["source_path"] == "app/layout.tsx"));
    assert!(execution["layouts"]
        .as_array()
        .expect("layouts")
        .iter()
        .any(|layout| layout["source_path"] == "app/(workspace)/dashboard/layout.tsx"));
    assert! {
        execution["templates"]
            .as_array()
            .expect("templates")
            .iter()
            .any(|template| template["source_path"] == "app/template.tsx"
                && template["depth"] == 0)
    };
    assert! {
        execution["templates"]
            .as_array()
            .expect("templates")
            .iter()
            .any(|template| template["source_path"]
                == "app/(workspace)/dashboard/template.tsx"
                && template["depth"] == 1)
    };
    assert! {
        execution["composition_chain"]
            .as_array()
            .expect("composition chain")
            .iter()
            .any(|entry| entry["kind"] == "template"
                && entry["source_path"] == "app/(workspace)/dashboard/template.tsx")
    };
    let composition_paths = execution["composition_chain"]
        .as_array()
        .expect("composition chain")
        .iter()
        .map(|entry| {
            entry["source_path"]
                .as_str()
                .expect("composition source path")
        })
        .collect::<Vec<_>>();
    assert_eq!(
        composition_paths,
        vec![
            "app/layout.tsx",
            "app/template.tsx",
            "app/(workspace)/dashboard/layout.tsx",
            "app/(workspace)/dashboard/template.tsx",
        ]
    );
    assert!(execution["boundaries"]["loading"]
        .as_array()
        .expect("loading boundaries")
        .iter()
        .any(|path| path == "app/(workspace)/dashboard/loading.tsx"));
    assert!(execution["boundaries"]["error"]
        .as_array()
        .expect("error boundaries")
        .iter()
        .any(|path| path == "app/(workspace)/dashboard/error.tsx"));
    assert!(execution["boundaries"]["not_found"]
        .as_array()
        .expect("not found boundaries")
        .iter()
        .any(|path| path == "app/(workspace)/dashboard/not-found.tsx"));
    assert_eq!(execution["proof"]["page_graph_path"], "page-graph.json");
    assert_eq!(execution["proof"]["packet_path"], "index.dxpk");
    assert_eq!(execution["proof"]["template_count"], 2);
    assert_eq!(execution["proof"]["layout_count"], 2);
    let html =
        fs::read_to_string(project.join(".dx/www/output/app/dashboard/[team]/settings/index.html"))
            .expect("route html");
    assert!(html.contains("data-dx-templates=\"2\""));

    let page_graph = read_json_value(
        project.join(".dx/www/output/app/dashboard/[team]/settings/page-graph.json"),
    );
    assert! {
        page_graph["components"]["nodes"]
            .as_array()
            .expect("graph nodes")
            .iter()
            .any(|node| node["id"] == "app/template" && node["name"] == "Template")
    };
    assert! {
        page_graph["components"]["nodes"]
            .as_array()
            .expect("graph nodes")
            .iter()
            .any(|node| node["id"] == "app/(workspace)/dashboard/template"
                && node["name"] == "Template")
    };
    let graph_edges = page_graph["components"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(graph_edges
        .iter()
        .any(|edge| edge["from"] == "app/layout" && edge["to"] == "app/template"));
    assert!(graph_edges.iter().any(|edge| {
        edge["from"] == "app/template" && edge["to"] == "app/(workspace)/dashboard/layout"
    }));
    assert!(graph_edges.iter().any(|edge| {
        edge["from"] == "app/(workspace)/dashboard/layout"
            && edge["to"] == "app/(workspace)/dashboard/template"
    }));
    assert!(graph_edges.iter().any(
        |edge| edge["from"] == "app/(workspace)/dashboard/template" && edge["to"] == "app/page"
    ));
    assert!(graph_edges
        .iter()
        .any(|edge| edge["from"] == "app/(workspace)/dashboard/template"
            && edge["to"] == "app/(workspace)/dashboard/loading"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_emits_client_island_micro_js_contract_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("client-island-app").expect("dx new");

    let project = dir.path().join("client-island-app");
    fs::write(
        project.join("app/page.tsx"),
        r#"import { CounterList } from "../components/local/CounterList";

export default function Page() {
  return <CounterList />;
}
"#,
    )
    .expect("page");
    fs::create_dir_all(project.join("components/local")).expect("client component dir");
    fs::write(
        project.join("components/local/CounterList.tsx"),
        r#""use client";

import { useState } from "react";

export function CounterList() {
  const [count, setCount] = useState(0);
  const items = [{ id: "first" }, { id: "second" }];

  function increment() {
    setCount((current) => current + 1);
  }

  return (
    <section>
      <button onClick={increment}>Add</button>
      <p>Count: {count}</p>
      <ul>{items.map((item) => <li key={item.id}>{item.id}</li>)}</ul>
    </section>
  );
}
"#,
    )
    .expect("client component");

    let project_cli = Cli::with_cwd(project.clone());
    project_cli
        .cmd_imports(&["sync".to_string()])
        .expect("dx imports sync");
    project_cli.cmd_build().expect("dx build");

    let islands = read_json_value(project.join(".dx/www/output/app/client-islands.json"));
    assert_eq!(islands["route"], "/");
    assert_eq!(islands["runtime"], "js");
    assert_eq!(islands["node_modules_required"], false);
    assert_eq!(islands["deterministic"], true);
    assert!(islands["islands"]
        .as_array()
        .expect("islands")
        .iter()
        .any(|island| {
            island["source_path"] == "components/local/CounterList.tsx"
                && island["delivery_mode"] == "MicroJs"
                && island["state"][0]["name"] == "count"
                && island["events"][0]["event"] == "click"
                && island["events"][0]["operation"] == "add"
                && island["keyed_updates"][0]["expression"] == "item.id"
                && island["micro_js"]["deterministic"] == true
                && island["micro_js"]["script_hash"]
                    .as_str()
                    .expect("script hash")
                    .starts_with("blake3:")
        }));
    let runtime = fs::read_to_string(project.join(".dx/www/output/app/client-islands.js"))
        .expect("client island runtime");
    assert!(runtime.contains("www client islands"));
    assert!(runtime.contains("addEventListener"));
    assert!(runtime.contains("click"));
    let app_html =
        fs::read_to_string(project.join(".dx/www/output/app/index.html")).expect("app html");
    let native_entrypoint =
        fs::read_to_string(project.join(".dx/www/output/index.html")).expect("native entrypoint");
    assert!(app_html.contains("data-dx-client-island-bridge"));
    assert_eq!(
        native_entrypoint, app_html,
        "native root index.html should mirror the final root app route after island stamping"
    );

    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert!(deploy["routes"]
        .as_array()
        .expect("routes")
        .iter()
        .any(|route| {
            route["path"] == "/"
                && route["client_islands"] == "app/client-islands.json"
                && route["client_islands_runtime"] == "app/client-islands.js"
        }));
    let provider = read_json_value(project.join(".dx/www/output/provider-adapter.dx-cloud.json"));
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .any(|artifact| {
            artifact["path"] == "app/client-islands.json" && artifact["kind"] == "client-islands"
        }));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_emits_client_island_hydration_contract_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("hydration-app").expect("dx new");

    let project = dir.path().join("hydration-app");
    fs::write(
        project.join("app/page.tsx"),
        r#"import dynamic from "next/dynamic";
import { SettingsForm } from "../components/local/SettingsForm";

const PreviewPanel = dynamic(() => import("../components/local/PreviewPanel"), { ssr: false });

export default function Page() {
  return (
    <main>
      <SettingsForm initialName="Ada" initialCount={2} />
      <PreviewPanel />
    </main>
  );
}
"#,
    )
    .expect("page");
    fs::write(
        project.join("components/local/SettingsForm.tsx"),
        r#""use client";

import { useState } from "react";

type SettingsFormProps = {
  initialName: string;
  initialCount: number;
};

export function SettingsForm({ initialName, initialCount }: SettingsFormProps) {
  const [name, setName] = useState(initialName);
  const [count, setCount] = useState(initialCount);

  function submit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
  }

  return (
    <form onSubmit={submit}>
      <input name="name" value={name} onChange={(event) => setName(event.target.value)} />
      <button type="button" onClick={() => setCount(count + 1)}>Add</button>
      <p>{count}</p>
    </form>
  );
}
"#,
    )
    .expect("settings form");
    fs::write(
        project.join("components/local/PreviewPanel.tsx"),
        r#"export function PreviewPanel() {
  return <aside>Preview</aside>;
}
"#,
    )
    .expect("preview panel");

    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let islands = read_json_value(project.join(".dx/www/output/app/client-islands.json"));
    let settings = islands["islands"]
        .as_array()
        .expect("islands")
        .iter()
        .find(|island| island["source_path"] == "components/local/SettingsForm.tsx")
        .expect("settings island");
    assert_eq!(settings["hydration"]["deterministic"], true);
    assert_eq!(settings["hydration"]["strategy"], "js-event-replay");
    assert!(settings["hydration"]["props"]
        .as_array()
        .expect("props")
        .iter()
        .any(|prop| prop["name"] == "initialName"
            && prop["source"] == "route-prop"
            && prop["value"] == "Ada"));
    assert!(settings["hydration"]["props"]
        .as_array()
        .expect("props")
        .iter()
        .any(|prop| prop["name"] == "initialCount"
            && prop["source"] == "route-prop"
            && prop["expression"] == "2"));
    assert!(settings["hydration"]["events"]
        .as_array()
        .expect("events")
        .iter()
        .any(|event| event["event"] == "submit"
            && event["form_id"] == "settingsform-form-0"
            && event["prevent_default"] == true));
    assert!(settings["hydration"]["forms"]
        .as_array()
        .expect("forms")
        .iter()
        .any(|form| form["form_id"] == "settingsform-form-0"
            && form["fields"]
                .as_array()
                .expect("fields")
                .iter()
                .any(|field| field["name"] == "name" && field["value_state"] == "name")));
    assert!(islands["dynamic_imports"]
        .as_array()
        .expect("dynamic imports")
        .iter()
        .any(
            |import| import["source"] == "../components/local/PreviewPanel"
                && import["preload"] == true
                && import["ssr"] == false
        ));
    assert_eq!(islands["hydration_runtime"]["source_owned"], true);
    assert_eq!(islands["hydration_runtime"]["dynamic_import_count"], 1);
    assert!(islands["hydration_runtime"]["script_hash"]
        .as_str()
        .expect("script hash")
        .starts_with("blake3:"));

    let island_id = settings["id"].as_str().expect("island id");
    let first_event_id = settings["hydration"]["events"]
        .as_array()
        .expect("events")
        .iter()
        .find_map(|event| event["event_id"].as_str())
        .expect("hydration event id");
    let html = fs::read_to_string(project.join(".dx/www/output/app/index.html")).expect("app html");
    assert!(html.contains(&format!(r#"data-dx-island="{island_id}""#)));
    assert!(html.contains(&format!(r#"data-dx-event-id="{first_event_id}""#)));

    let runtime = fs::read_to_string(project.join(".dx/www/output/app/client-islands.js"))
        .expect("client island runtime");
    assert!(runtime.contains("data-dx-island"));
    assert!(runtime.contains("addEventListener"));
    assert!(runtime.contains("dx:client-island-event"));
    assert!(!runtime.contains("() => {}"));
    assert!(runtime.contains("dx:preload"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_writes_generated_css_module_assets_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("style-module-app").expect("dx new");

    let project = dir.path().join("style-module-app");
    fs::write(
        project.join("app/page.tsx"),
        r#"import { ProductCard } from "../components/local/ProductCard";

export default function Page() {
  return <ProductCard />;
}
"#,
    )
    .expect("page");
    fs::write(
        project.join("components/local/ProductCard.tsx"),
        r#"import styles from "./ProductCard.module.css";

export function ProductCard() {
  return <section className={styles.card}><h1>Products</h1></section>;
}
"#,
    )
    .expect("component");
    fs::write(
        project.join("components/local/ProductCard.module.css"),
        ".card { color: var(--dx-accent); padding: 1rem; }",
    )
    .expect("css module");

    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let graph = read_json_value(project.join(".dx/www/output/app/page-graph.json"));
    let generated = graph["styles"]["classes"]
        .as_array()
        .expect("classes")
        .iter()
        .find(|class| {
            class["name"]
                .as_str()
                .is_some_and(|name| name.starts_with("card__"))
        })
        .expect("scoped class");
    let scoped_class = generated["name"].as_str().expect("class name");
    let html = fs::read_to_string(project.join(".dx/www/output/app/index.html")).expect("html");
    let href = first_generated_css_href(&html).expect("generated css href");
    let css_path = project
        .join(".dx/www/output")
        .join(href.trim_start_matches('/'));
    let css = fs::read_to_string(css_path).expect("generated css");

    assert!(css.contains(scoped_class));
    assert!(css.contains("color:var(--dx-accent)"));
    assert!(
        read_json_value(project.join(".dx/www/output/manifest.json"))
            ["generated_style_assets_compiled"]
            .as_u64()
            .is_some_and(|count| count >= 1)
    );
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_writes_streaming_plan_for_deferred_resumable_routes() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("streaming-app").expect("dx new");

    let project = dir.path().join("streaming-app");
    fs::write(
        project.join("app/page.tsx"),
        r#"import { BuyButton } from "../components/local/BuyButton";

export default async function Page() {
  return <main><h1>Checkout</h1><BuyButton /></main>;
}
"#,
    )
    .expect("page");
    fs::write(
        project.join("app/loading.tsx"),
        r#"export default function Loading() {
  return <p>Preparing checkout</p>;
}
"#,
    )
    .expect("loading");
    fs::write(
        project.join("components/local/BuyButton.tsx"),
        r#""use client";

import { useState } from "react";

export function BuyButton() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>Buy {count}</button>;
}
"#,
    )
    .expect("client component");

    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let streaming = read_json_value(project.join(".dx/www/output/app/streaming-plan.json"));
    assert_eq!(streaming["enabled"], true);
    assert_eq!(streaming["strategy"], "shell-first-deferred-boundaries");
    assert_eq!(streaming["node_modules_required"], false);
    assert_eq!(streaming["deferred_chunks"][0]["boundary"], "loading");
    assert_eq!(
        streaming["resumable_islands"][0]["source_path"],
        "components/local/BuyButton.tsx"
    );
    assert_eq!(streaming["resumable_islands"][0]["runtime"], "js");

    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert!(deploy["routes"]
        .as_array()
        .expect("routes")
        .iter()
        .any(|route| {
            route["path"] == "/" && route["streaming_plan"] == "app/streaming-plan.json"
        }));
    assert_eq!(
        read_json_value(project.join(".dx/www/output/manifest.json"))["streaming_plans_compiled"],
        7
    );
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_emits_next_project_migration_proof_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("next-source-app");
    fs::create_dir_all(project.join("app/api/health")).expect("api dir");
    fs::create_dir_all(project.join("components")).expect("components dir");
    fs::create_dir_all(project.join("server")).expect("server dir");
    fs::create_dir_all(project.join("styles")).expect("styles dir");
    fs::write(
        project.join("package.json"),
        r#"{
  "name": "next-source-app",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build"
  },
  "dependencies": {
    "next": "16.0.0",
    "react": "19.0.0",
    "react-dom": "19.0.0"
  }
}
"#,
    )
    .expect("package json");
    fs::write(project.join("next.config.mjs"), "export default {};\n").expect("next config");
    fs::write(
        project.join("tsconfig.json"),
        r#"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["./*"]
    }
  }
}
"#,
    )
    .expect("tsconfig");
    fs::write(
        project.join("app/layout.tsx"),
        r#"import "../styles/global.css";

export default function RootLayout({ children }) {
  return <html lang="en"><body>{children}</body></html>;
}
"#,
    )
    .expect("layout");
    fs::write(
        project.join("app/page.tsx"),
        r#"import Image from "next/image";
import Link from "next/link";
import { cookies } from "next/cookies";
import { headers } from "next/headers";
import { redirect } from "next/navigation";
import { PricingCta } from "@/components/PricingCta";

export const metadata = {
  title: "Next Source App",
  description: "Small App Router app migrated into DX-WWW output.",
};

export default function Page() {
  return (
    <main>
      <h1>Next Source App</h1>
      <Image src="/hero.png" alt="Hero" width={640} height={360} />
      <Link href="/pricing">Pricing</Link>
      <span>{headers().get("x-plan") ?? cookies().get("plan")?.value ?? ""}</span>
      <PricingCta />
    </main>
  );
}
"#,
    )
    .expect("page");
    fs::write(
        project.join("components/PricingCta.tsx"),
        r#""use client";

import { useState } from "react";

export function PricingCta() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>Join {count}</button>;
}
"#,
    )
    .expect("component");
    fs::write(
        project.join("app/api/health/route.ts"),
        r#"import { NextResponse } from "next/server";

export function GET() {
  return NextResponse.json({ ok: true });
}
"#,
    )
    .expect("route handler");
    fs::write(
        project.join("server/actions.ts"),
        r#"export async function recordWelcomeView(payload: { count: number }) {
  return {
    ok: true,
    source: "next-source-app",
  };
}
"#,
    )
    .expect("server action");
    fs::write(project.join("styles/global.css"), "body { margin: 0; }\n").expect("style");

    let project_cli = Cli::with_cwd(project.clone());
    project_cli
        .cmd_imports(&["sync".to_string()])
        .expect("dx imports sync");
    project_cli.cmd_build().expect("dx build");

    let proof = read_json_value(project.join(".dx/www/output/next-migration-proof.json"));
    assert_eq!(proof["source_framework"], "nextjs-app-router");
    assert_eq!(proof["migration_kind"], "source-owned-runtime-compile");
    assert_eq!(proof["runtime_node_modules_required"], false);
    assert_eq!(proof["node_modules_present"], false);
    assert_eq!(proof["package_installs_executed"], false);
    assert_eq!(proof["lifecycle_scripts_executed"], false);
    assert_eq!(proof["compiled_routes"][0]["html"], "app/index.html");
    assert_eq!(
        proof["blocked_runtime_imports"]
            .as_array()
            .expect("blocked imports")
            .len(),
        0
    );
    assert!(proof["compiler_intrinsics"]
        .as_array()
        .expect("intrinsics")
        .iter()
        .any(|specifier| specifier == "next/link"));
    assert!(proof["compiler_intrinsics"]
        .as_array()
        .expect("intrinsics")
        .iter()
        .any(|specifier| specifier == "next/image"));
    assert!(proof["compiler_intrinsics"]
        .as_array()
        .expect("intrinsics")
        .iter()
        .any(|specifier| specifier == "next/server"));
    assert!(proof["compiler_intrinsics"]
        .as_array()
        .expect("intrinsics")
        .iter()
        .any(|specifier| specifier == "next/headers"));
    assert!(proof["compiler_intrinsics"]
        .as_array()
        .expect("intrinsics")
        .iter()
        .any(|specifier| specifier == "next/cookies"));
    assert!(proof["compiler_intrinsics"]
        .as_array()
        .expect("intrinsics")
        .iter()
        .any(|specifier| specifier == "next/navigation"));
    assert!(project.join(".dx/www/output/app/index.html").is_file());
    assert!(project.join(".dx/www/output/app/index.dxpk").is_file());

    let adapters = read_json_value(project.join(".dx/www/output/next-adapter-fixtures.json"));
    assert_eq!(adapters["adapter_family"], "nextjs-app-router");
    assert_eq!(adapters["forge_owned"], true);
    assert_eq!(adapters["runtime_node_modules_required"], false);
    assert_eq!(adapters["package_installs_executed"], false);
    assert_eq!(adapters["lifecycle_scripts_executed"], false);
    assert_eq!(adapters["strict_runtime_proof"]["score"], 100);
    for (specifier, source_path) in [
        ("next/link", "forge/adapters/next-link.tsx"),
        ("next/image", "forge/adapters/next-image.tsx"),
        ("next/headers", "forge/adapters/next-headers.ts"),
        ("next/cookies", "forge/adapters/next-cookies.ts"),
        ("next/navigation", "forge/adapters/next-navigation.ts"),
    ] {
        assert!(adapters["adapters"]
            .as_array()
            .expect("adapters")
            .iter()
            .any(|adapter| adapter["specifier"] == specifier
                && adapter["source_path"] == source_path
                && adapter["source_owned"] == true
                && adapter["runtime_node_modules_required"] == false));
        assert!(project.join(".dx/www/output").join(source_path).is_file());
    }

    let compatibility =
        read_json_value(project.join(".dx/www/output/next-familiar-compatibility-evidence.json"));
    assert_eq!(
        compatibility["evidence_mode"],
        "next-familiar-source-output-readiness"
    );
    assert_eq!(compatibility["score"], 100);
    assert_eq!(
        compatibility["compatibility_dimensions"]["routes"]["score"],
        100
    );
    assert_eq!(
        compatibility["compatibility_dimensions"]["bytes"]["score"],
        100
    );
    assert_eq!(
        compatibility["compatibility_dimensions"]["hydration"]["score"],
        100
    );
    assert_eq!(
        compatibility["compatibility_dimensions"]["server_actions"]["score"],
        100
    );
    assert_eq!(
        compatibility["compatibility_dimensions"]["security"]["score"],
        100
    );
    assert_eq!(
        compatibility["next_familiar_inventory"]["server_actions"][0],
        "recordWelcomeView"
    );

    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert_eq!(
        deploy["next_migration"]["path"],
        "next-migration-proof.json"
    );
    assert_eq!(
        deploy["next_familiar_compatibility_evidence"]["path"],
        "next-familiar-compatibility-evidence.json"
    );
    assert_eq!(
        deploy["next_adapter_fixtures"]["path"],
        "next-adapter-fixtures.json"
    );
    assert_eq!(
        read_json_value(project.join(".dx/www/output/manifest.json"))
            ["next_migration_proof_emitted"],
        true
    );
    assert_eq!(
        read_json_value(project.join(".dx/www/output/manifest.json"))
            ["next_familiar_compatibility_evidence_emitted"],
        true
    );
    assert_eq!(
        read_json_value(project.join(".dx/www/output/manifest.json"))
            ["next_adapter_fixtures_emitted"],
        true
    );
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_emits_next_familiar_fixtures_for_dynamic_metadata_middleware_and_mixed_output() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("next-familiar-app");
    fs::create_dir_all(project.join("app/(marketing)/about")).expect("about dir");
    fs::create_dir_all(project.join("app/(workspace)/dashboard/[team]")).expect("dynamic dir");
    fs::create_dir_all(project.join("app/api/health")).expect("api dir");
    fs::create_dir_all(project.join("server")).expect("server dir");
    fs::create_dir_all(project.join("styles")).expect("styles dir");

    fs::write(
        project.join("package.json"),
        r#"{
  "name": "next-familiar-app",
  "private": true,
  "dependencies": {
    "next": "16.0.0",
    "react": "19.0.0",
    "react-dom": "19.0.0"
  }
}
"#,
    )
    .expect("package json");
    fs::write(project.join("next.config.mjs"), "export default {};\n").expect("next config");
    fs::write(
        project.join("middleware.ts"),
        r#"import { NextResponse } from "next/server";

export function middleware(request) {
  if (request.nextUrl.pathname === "/old-dashboard") {
    return NextResponse.redirect(new URL("/dashboard/acme", request.url));
  }
  return NextResponse.next();
}

export const config = {
  matcher: ["/old-dashboard"],
};
"#,
    )
    .expect("middleware");
    fs::write(
        project.join("app/layout.tsx"),
        r#"import "../styles/global.css";

export const metadata = {
  title: "Parity Root",
  description: "Root metadata",
};

export default function RootLayout({ children }) {
  return <html lang="en"><body>{children}</body></html>;
}
"#,
    )
    .expect("layout");
    fs::write(
        project.join("app/(marketing)/about/page.tsx"),
        r#"export const metadata = {
  title: "About",
  description: "Static marketing route",
  canonical: "/about",
};

export default function Page() {
  return <main><h1>About</h1><p>Static output</p></main>;
}
"#,
    )
    .expect("about page");
    fs::write(
        project.join("app/(workspace)/dashboard/layout.tsx"),
        r#"export const metadata = {
  title: "Dashboard",
  description: "Dashboard shell",
};

export default function DashboardLayout({ children }) {
  return <section>{children}</section>;
}
"#,
    )
    .expect("dashboard layout");
    fs::write(
        project.join("app/(workspace)/dashboard/[team]/page.tsx"),
        r#"import { loadDashboard } from "../../../../server/loaders";

export const metadata = {
  title: "Team Dashboard",
  description: "Dynamic team dashboard",
  canonical: "/dashboard/acme",
};

export default async function Page({ params }) {
  const dashboard = await loadDashboard();
  return <main><h1>{dashboard.title}</h1><p>{params.team}</p></main>;
}
"#,
    )
    .expect("dynamic page");
    fs::write(
        project.join("app/robots.ts"),
        r#"export default function robots() {
  return { rules: [{ userAgent: "*", allow: "/" }] };
}
"#,
    )
    .expect("robots");
    fs::write(
        project.join("app/sitemap.ts"),
        r#"export default function sitemap() {
  return [{ url: "https://example.com/about" }];
}
"#,
    )
    .expect("sitemap");
    fs::write(
        project.join("app/opengraph-image.tsx"),
        r#"export default function Image() {
  return <div>OG</div>;
}
"#,
    )
    .expect("og image");
    fs::write(
        project.join("app/api/health/route.ts"),
        r#"export function GET() {
  return Response.json({ ok: true });
}
"#,
    )
    .expect("health");
    fs::write(
        project.join("server/loaders.ts"),
        r#"export async function loadDashboard() {
  return { title: "Dashboard" };
}
"#,
    )
    .expect("loader");
    fs::write(project.join("styles/global.css"), "body { margin: 0; }\n").expect("style");

    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let fixtures = read_json_value(project.join(".dx/www/output/next-familiar-fixtures.json"));
    assert_eq!(
        fixtures["fixture_family"],
        "next-familiar-app-router-compatibility"
    );
    assert_eq!(fixtures["node_modules_required"], false);
    assert_eq!(fixtures["package_installs_executed"], false);
    assert_eq!(fixtures["lifecycle_scripts_executed"], false);
    assert_eq!(fixtures["score"], 100);
    assert!(fixtures["dynamic_routes"]
        .as_array()
        .expect("dynamic routes")
        .iter()
        .any(|route| route["route"] == "/dashboard/[team]"
            && route["route_groups"][0] == "(workspace)"
            && route["dynamic_segments"][0]["name"] == "team"
            && route["metadata"]["title"] == "Team Dashboard"));
    assert!(fixtures["route_groups"]
        .as_array()
        .expect("route groups")
        .iter()
        .any(|group| group["segment"] == "(marketing)"
            && group["routes"]
                .as_array()
                .expect("group routes")
                .iter()
                .any(|route| route == "/about")));
    assert!(fixtures["metadata_files"]
        .as_array()
        .expect("metadata files")
        .iter()
        .any(|file| file["kind"] == "robots" && file["source_path"] == "app/robots.ts"));
    assert!(fixtures["metadata_files"]
        .as_array()
        .expect("metadata files")
        .iter()
        .any(|file| file["kind"] == "sitemap" && file["source_path"] == "app/sitemap.ts"));
    assert!(fixtures["metadata_files"]
        .as_array()
        .expect("metadata files")
        .iter()
        .any(|file| file["kind"] == "opengraph-image"
            && file["source_path"] == "app/opengraph-image.tsx"));
    assert!(fixtures["middleware_redirects"]
        .as_array()
        .expect("middleware redirects")
        .iter()
        .any(|redirect| redirect["source_path"] == "middleware.ts"
            && redirect["from"] == "/old-dashboard"
            && redirect["to"] == "/dashboard/acme"
            && redirect["materialized_as"] == "hosting-redirect-rule"));
    assert!(fixtures["mixed_output"]["static_routes"]
        .as_array()
        .expect("static routes")
        .iter()
        .any(|route| route["route"] == "/about"));
    assert!(fixtures["mixed_output"]["server_routes"]
        .as_array()
        .expect("server routes")
        .iter()
        .any(|route| route["route"] == "/dashboard/[team]"
            && route["server_data"] == "app/dashboard/[team]/server-data.json"));
    assert_eq!(fixtures["strict_runtime_proof"]["score"], 100);

    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert_eq!(
        deploy["next_familiar_fixtures"]["path"],
        "next-familiar-fixtures.json"
    );
    let provider = read_json_value(project.join(".dx/www/output/provider-adapter.dx-cloud.json"));
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .any(|artifact| artifact["path"] == "next-familiar-fixtures.json"
            && artifact["kind"] == "next-familiar-fixtures"));
    assert_eq!(
        read_json_value(project.join(".dx/www/output/manifest.json"))
            ["next_familiar_fixtures_emitted"],
        true
    );
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_emits_account_free_provider_adapter_fixture() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("provider-app").expect("dx new");

    let project = dir.path().join("provider-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert_eq!(
        deploy["provider_adapter"]["path"],
        "provider-adapter.dx-cloud.json"
    );
    assert_eq!(
        deploy["provider_adapter_smoke_matrix"]["path"],
        "provider-adapter-smoke-matrix.json"
    );
    assert_eq!(
        deploy["provider_adapter_smoke_matrix"]["hosted_provider_proof"],
        false
    );
    assert_eq!(
        deploy["provider_adapter"]["requires_provider_account"],
        false
    );

    let provider = read_json_value(project.join(".dx/www/output/provider-adapter.dx-cloud.json"));
    let smoke_matrix =
        read_json_value(project.join(".dx/www/output/provider-adapter-smoke-matrix.json"));
    assert_eq!(provider["provider"], "dx-www-cloud-local");
    assert_eq!(provider["adapter_kind"], "account-free-fixture");
    assert_eq!(provider["requires_provider_account"], false);
    assert_eq!(provider["account_bound"], false);
    assert_eq!(provider["network_required"], false);
    assert_eq!(provider["secrets_required"], false);
    assert_eq!(provider["no_node_modules_required"], true);
    assert!(provider["deployment_id"]
        .as_str()
        .expect("deployment id")
        .starts_with("dxlocal-"));
    assert_eq!(
        provider["build_manifest"]["hash"],
        deploy["build_manifest"]["hash"]
    );
    assert!(provider["routes"]
        .as_array()
        .expect("routes")
        .iter()
        .any(|route| route["path"] == "/" && route["html"] == "app/index.html"));
    assert_eq!(provider["server_actions"], deploy["server_actions"]);
    assert_eq!(
        provider["server_action_replay_ledger"],
        deploy["server_action_replay_ledger"]
    );
    assert_eq!(
        provider["server_action_replay_ledger"]["path"],
        "server-action-replay-ledger.json"
    );
    assert_eq!(
        provider["server_action_replay_ledger"]["mode"],
        "local-preview-hash-ledger"
    );
    assert_eq!(
        provider["runtime"]["server_actions"].as_u64(),
        Some(
            provider["server_actions"]
                .as_array()
                .expect("provider server actions")
                .len() as u64
        )
    );
    assert_eq!(provider["health_checks"], deploy["health_checks"]);
    assert_eq!(
        smoke_matrix["coverage"]["server_actions"].as_u64(),
        Some(
            deploy["server_actions"]
                .as_array()
                .expect("deploy server actions")
                .len() as u64
        )
    );
    assert_eq!(
        smoke_matrix["coverage"]["health_checks"].as_u64(),
        Some(
            deploy["health_checks"]
                .as_array()
                .expect("deploy health checks")
                .len() as u64
        )
    );
    assert!(provider["immutable_assets"]
        .as_array()
        .expect("immutable assets")
        .iter()
        .all(|asset| asset["path"] != "app/index.dxpk"));
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .any(
            |artifact| artifact["path"] == "provider-adapter.dx-cloud.json"
                && artifact["account_required"] == false
        ));
    assert_eq!(
        provider["provider_adapter_smoke_matrix"]["path"],
        "provider-adapter-smoke-matrix.json"
    );
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .any(
            |artifact| artifact["path"] == "provider-adapter-smoke-matrix.json"
                && artifact["kind"] == "provider-adapter-smoke-matrix"
                && artifact["bundle"] == "evidence"
                && artifact["cache_control"] == "no-store"
        ));
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .any(
            |artifact| artifact["path"] == "server-action-replay-ledger.json"
                && artifact["kind"] == "server-action-replay-ledger"
                && artifact["bundle"] == "evidence"
                && artifact["cache_control"] == "no-store"
        ));
    assert_eq!(
        smoke_matrix["schema"],
        "dx.www.deploy.provider_adapter_smoke_matrix"
    );
    assert_eq!(smoke_matrix["release_ready"], false);
    assert_eq!(smoke_matrix["hosted_provider_proof"], false);
    assert_eq!(
        smoke_matrix["matrix_status"],
        "local-proof-and-upload-plan-only"
    );
    assert!(smoke_matrix["matrix"]
        .as_array()
        .expect("smoke matrix entries")
        .iter()
        .any(|entry| entry["status"] == "local-replay-passing-foundation"));
    assert!(smoke_matrix["matrix"]
        .as_array()
        .expect("smoke matrix entries")
        .iter()
        .any(|entry| entry["status"] == "upload-plan-only"));
    assert!(smoke_matrix["not_yet_proven"]
        .as_array()
        .expect("smoke matrix backlog")
        .iter()
        .any(|item| item == "multi-provider deployed smoke proof"));
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .all(|artifact| artifact["path"]
            .as_str()
            .expect("artifact path")
            .contains("node_modules")
            == false));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_emits_hosted_preview_bundle_with_forge_receipts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("hosted-preview-app").expect("dx new");

    let project = dir.path().join("hosted-preview-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli
        .cmd_add(&["ui/button"])
        .expect("dx add ui/button");
    project_cli
        .cmd_imports(&["sync".to_string()])
        .expect("dx imports sync");
    project_cli.cmd_build().expect("dx build");

    let hosted = read_json_value(project.join(".dx/www/output/hosted-preview.json"));
    assert_eq!(hosted["deployment_kind"], "dx-www-hosted-preview-bundle");
    assert_eq!(hosted["requires_provider_account"], false);
    assert_eq!(hosted["network_required"], false);
    assert_eq!(hosted["secrets_required"], false);
    assert_eq!(hosted["node_modules_required"], false);
    assert_eq!(hosted["package_installs_executed"], false);
    assert_eq!(hosted["lifecycle_scripts_executed"], false);
    assert_eq!(hosted["deploy_adapter"]["path"], "deploy-adapter.json");
    assert_eq!(
        hosted["provider_adapter"]["path"],
        "provider-adapter.dx-cloud.json"
    );
    assert_eq!(
        hosted["forge"]["source_manifest"]["bundle_path"],
        "forge/source-manifest.json"
    );
    assert_eq!(
        hosted["forge"]["template_manifest"]["bundle_path"],
        "forge/template-manifest.json"
    );
    assert!(hosted["forge"]["receipt_count"].as_u64().unwrap_or(0) >= 2);
    assert!(hosted["forge"]["receipts"]
        .as_array()
        .expect("hosted receipts")
        .iter()
        .any(|receipt| receipt["package_id"] == "shadcn/ui/button"
            && receipt["bundle_path"]
                .as_str()
                .expect("receipt bundle path")
                .starts_with("forge/receipts/")
            && receipt["source_owned"] == true
            && receipt["lifecycle_scripts_executed"] == false));
    let hosted_source_manifest =
        read_json_value(project.join(".dx/www/output/forge/source-manifest.json"));
    assert!(hosted_source_manifest["packages"]
        .as_array()
        .expect("hosted source manifest packages")
        .iter()
        .any(|package| package["package_id"] == "www/minimal-starter"
            && package["variant"] == "default"
            && package["files"]
                .as_array()
                .expect("minimal starter files")
                .iter()
                .any(|file| file["path"] == "app/page.tsx")));
    assert_eq!(hosted["release_gate"]["account_free_preview_ready"], true);
    assert_eq!(hosted["release_gate"]["hosted_release_ready"], false);
    assert_eq!(hosted["release_gate"]["manifest_signature_required"], true);
    assert!(hosted["bundle"]["artifacts"]
        .as_array()
        .expect("bundle artifacts")
        .iter()
        .all(|artifact| artifact["path"]
            .as_str()
            .expect("artifact path")
            .contains("node_modules")
            == false));

    assert!(project
        .join(".dx/www/output/forge/source-manifest.json")
        .is_file());
    assert!(project
        .join(".dx/www/output/forge/template-manifest.json")
        .is_file());
    assert!(project.join(".dx/www/output/forge/receipts").is_dir());

    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert_eq!(deploy["hosted_preview"]["path"], "hosted-preview.json");
    assert_eq!(deploy["hosted_preview"]["requires_provider_account"], false);
    assert_eq!(
        read_json_value(project.join(".dx/www/output/manifest.json"))
            ["hosted_preview_contract_emitted"],
        true
    );

    let provider = read_json_value(project.join(".dx/www/output/provider-adapter.dx-cloud.json"));
    assert_eq!(provider["hosted_preview"]["path"], "hosted-preview.json");
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .any(|artifact| artifact["path"] == "hosted-preview.json"
            && artifact["kind"] == "hosted-preview-contract"));
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .any(|artifact| artifact["path"] == "forge/source-manifest.json"
            && artifact["kind"] == "forge-source-manifest"));
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .any(|artifact| artifact["path"]
            .as_str()
            .expect("artifact path")
            .starts_with("forge/receipts/")
            && artifact["kind"] == "forge-receipt"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_emits_forge_hosting_manifest_release_gate() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("forge-hosting-app").expect("dx new");

    let project = dir.path().join("forge-hosting-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli
        .cmd_add(&["ui/button"])
        .expect("dx add ui/button");
    project_cli.cmd_build().expect("dx build");

    let hosting = read_json_value(project.join(".dx/www/output/forge-hosting-manifest.json"));
    assert_eq!(
        hosting["manifest_kind"],
        "dx-www-forge-hosting-release-gate"
    );
    assert_eq!(hosting["node_modules_required"], false);
    assert_eq!(hosting["package_installs_executed"], false);
    assert_eq!(hosting["lifecycle_scripts_executed"], false);
    assert_eq!(hosting["release_gate"]["coverage_score"], 100);
    assert_eq!(hosting["release_gate"]["account_free_preview_ready"], true);
    assert_eq!(hosting["release_gate"]["hosted_release_ready"], false);
    assert_eq!(
        hosting["release_gate"]["blockers"]
            .as_array()
            .expect("release blockers")
            .iter()
            .any(|blocker| blocker == "build-manifest-signature-required"),
        true
    );
    assert_eq!(hosting["signed_manifest"]["path"], "manifest.json");
    assert_eq!(hosting["signed_manifest"]["signed"], false);
    assert_eq!(
        hosting["signed_manifest"]["signature_required_for_release"],
        true
    );
    assert_eq!(
        hosting["signed_manifest"]["promotion_path"],
        "build-promotion.json"
    );
    assert_eq!(hosting["rollback_inputs"]["metadata_path"], "rollback.json");
    assert!(hosting["rollback_inputs"]["restore_order"]
        .as_array()
        .expect("restore order")
        .iter()
        .any(|entry| entry == "deploy-adapter.json"));
    assert!(hosting["cache_headers"]
        .as_array()
        .expect("cache headers")
        .iter()
        .any(|header| header["glob"] == "**/*.html"
            && header["cache_control"] == "public, max-age=0, must-revalidate"));
    assert_eq!(
        hosting["observability_endpoints"]["ready_path"],
        "/.dx/ready"
    );
    assert_eq!(
        hosting["observability_endpoints"]["metrics_path"],
        "/.dx/observability"
    );
    assert_eq!(
        hosting["observability_endpoints"]["collects_secrets"],
        false
    );
    assert_eq!(hosting["provider_portability"]["portable"], true);
    assert!(hosting["provider_portability"]["providers"]
        .as_array()
        .expect("providers")
        .iter()
        .any(|provider| provider["provider"] == "dx-www-cloud-local"
            && provider["account_required"] == false));
    assert!(hosting["provider_portability"]["providers"]
        .as_array()
        .expect("providers")
        .iter()
        .any(|provider| provider["provider"] == "vercel-static-output"));
    assert!(hosting["release_gate"]["required_artifacts"]
        .as_array()
        .expect("required artifacts")
        .iter()
        .any(|artifact| artifact == "provider-adapter.dx-cloud.json"));

    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert_eq!(
        deploy["forge_hosting_manifest"]["path"],
        "forge-hosting-manifest.json"
    );
    assert_eq!(deploy["forge_hosting_manifest"]["coverage_score"], 100);

    let hosted = read_json_value(project.join(".dx/www/output/hosted-preview.json"));
    assert!(hosted["bundle"]["artifacts"]
        .as_array()
        .expect("hosted artifacts")
        .iter()
        .any(|artifact| artifact["path"] == "forge-hosting-manifest.json"
            && artifact["kind"] == "forge-hosting-manifest"));

    let provider = read_json_value(project.join(".dx/www/output/provider-adapter.dx-cloud.json"));
    assert_eq!(
        provider["forge_hosting_manifest"]["path"],
        "forge-hosting-manifest.json"
    );
    assert!(provider["upload_plan"]
        .as_array()
        .expect("upload plan")
        .iter()
        .any(|artifact| artifact["path"] == "forge-hosting-manifest.json"
            && artifact["kind"] == "forge-hosting-manifest"));
    assert_eq!(
        read_json_value(project.join(".dx/www/output/manifest.json"))
            ["forge_hosting_manifest_emitted"],
        true
    );
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_emits_secret_safe_production_observability_contract() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("observability-app").expect("dx new");

    let project = dir.path().join("observability-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert_eq!(
        deploy["observability"]["metadata_path"],
        "observability.json"
    );
    assert_eq!(deploy["observability"]["ready_path"], "/.dx/ready");
    assert_eq!(
        deploy["observability"]["metrics_path"],
        "/.dx/observability"
    );
    assert_eq!(deploy["observability"]["collects_secrets"], false);

    let observability = read_json_value(project.join(".dx/www/output/observability.json"));
    assert_eq!(observability["secret_fields_collected"], false);
    assert_eq!(observability["privacy"]["collects_request_headers"], false);
    assert_eq!(observability["privacy"]["collects_request_payloads"], false);
    assert_eq!(observability["privacy"]["collects_cookies"], false);
    assert_eq!(
        deploy["route_handlers"]
            .as_array()
            .expect("route handlers")
            .len(),
        0
    );
    assert_eq!(
        observability["health_checks"]
            .as_array()
            .expect("health checks")
            .len(),
        0
    );
    assert_eq!(observability["ready_check"]["path"], "/.dx/ready");
    assert!(observability["ready_check"]["required_artifacts"]
        .as_array()
        .expect("ready artifacts")
        .iter()
        .any(|artifact| artifact == "app/index.html"));
    assert!(observability["route_timings"]
        .as_array()
        .expect("route timings")
        .iter()
        .any(|route| route["path"] == "/"
            && route["metric"] == "route.duration_ms"
            && route["collects_query"] == false));
    assert!(observability["packet_byte_sizes"]
        .as_array()
        .expect("packet byte sizes")
        .iter()
        .all(|packet| packet["path"] != "app/index.dxpk"));
    assert_eq!(
        deploy["server_actions"]
            .as_array()
            .expect("server actions")
            .len(),
        0
    );
    assert_eq!(
        observability["server_action_receipts"]
            .as_array()
            .expect("server action receipts")
            .len(),
        0
    );
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_rollback_verify_compares_build_dirs_and_proves_previous_assets_restorable() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("rollback-app").expect("dx new");

    let project = dir.path().join("rollback-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("initial dx build");

    let previous_build = project.join(".dx/previous-build");
    copy_test_dir(&project.join(".dx/www/output"), &previous_build);

    fs::write(
        project.join("app/page.tsx"),
        r#"import { Button } from "@forge/ui/button";

export default function Page() {
  return <main><Button>Rollback target</Button></main>;
}
"#,
    )
    .expect("update page");
    project_cli.cmd_build().expect("current dx build");

    let output = project.join(".dx/rollback-verify.json");
    project_cli
        .cmd_rollback(&[
            "verify".to_string(),
            "--previous-build-dir".to_string(),
            previous_build.to_string_lossy().into_owned(),
            "--current-build-dir".to_string(),
            project
                .join(".dx/www/output")
                .to_string_lossy()
                .into_owned(),
            "--output".to_string(),
            output.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ])
        .expect("rollback verify");

    let report = read_json_value(output);
    assert_eq!(report["passed"], true);
    assert_eq!(report["strategy"], "manifest-pinned-asset-rollback");
    assert_eq!(
        report["previous_immutable_assets_total"],
        report["previous_immutable_assets_restorable"]
    );
    assert!(
        report["previous_immutable_assets_total"]
            .as_u64()
            .expect("asset count")
            > 0
    );
    assert_eq!(
        report["missing_previous_assets"]
            .as_array()
            .expect("missing assets")
            .len(),
        0
    );
    assert_eq!(
        report["restore_order"]
            .as_array()
            .expect("restore order")
            .first()
            .expect("first restore step"),
        "immutable_assets"
    );
    assert_eq!(report["current_rollback_metadata_present"], true);
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_rollback_verify_fails_when_previous_immutable_asset_is_missing() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("broken-rollback-app").expect("dx new");

    let project = dir.path().join("broken-rollback-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("initial dx build");

    let previous_build = project.join(".dx/previous-build");
    copy_test_dir(&project.join(".dx/www/output"), &previous_build);
    fs::remove_file(previous_build.join("styles/generated.css"))
        .expect("remove previous css asset");

    project_cli.cmd_build().expect("current dx build");

    let error = project_cli
        .cmd_rollback(&[
            "verify".to_string(),
            "--previous-build-dir".to_string(),
            previous_build.to_string_lossy().into_owned(),
            "--current-build-dir".to_string(),
            project
                .join(".dx/www/output")
                .to_string_lossy()
                .into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ])
        .expect_err("rollback verify should fail for missing previous asset");

    let message = error.to_string();
    assert!(
        message.contains("missing previous immutable assets"),
        "rollback verify error should mention missing assets: {message}"
    );
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_preview_production_contract_serves_ready_and_observability_without_secrets() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("observed-preview-app").expect("dx new");

    let project = dir.path().join("observed-preview-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let build_dir = project.join(".dx/www/output");
    let ready = preview_contract::handle_production_contract_request(&build_dir, "/.dx/ready")
        .expect("ready");
    let ready_body: serde_json::Value = serde_json::from_slice(&ready.body).expect("ready json");
    assert_eq!(ready.status, "200 OK");
    assert_eq!(ready.content_type, "application/json; charset=utf-8");
    assert_eq!(ready.cache_control.as_deref(), Some("no-store"));
    assert_eq!(ready_body["ok"], true);
    assert_eq!(ready_body["ready"], true);
    assert_eq!(ready_body["secret_fields_collected"], false);
    assert_eq!(
        ready_body["manifest_hash"]
            .as_str()
            .unwrap_or("")
            .starts_with("blake3:"),
        true
    );

    let observed =
        preview_contract::handle_production_contract_request(&build_dir, "/.dx/observability")
            .expect("observability");
    let observed_body: serde_json::Value =
        serde_json::from_slice(&observed.body).expect("observability json");
    assert_eq!(observed.status, "200 OK");
    assert_eq!(observed.content_type, "application/json; charset=utf-8");
    assert_eq!(observed.cache_control.as_deref(), Some("no-store"));
    assert_eq!(observed_body["secret_fields_collected"], false);
    assert_eq!(observed_body["privacy"]["collects_request_headers"], false);
    assert_eq!(observed_body["privacy"]["collects_request_payloads"], false);
    assert_eq!(
        observed_body
            .get("headers")
            .and_then(|value| value.as_object())
            .map(|headers| headers.is_empty())
            .unwrap_or(true),
        true
    );
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_preview_production_contract_serves_only_deploy_adapter_outputs() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("preview-app").expect("dx new");

    let project = dir.path().join("preview-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let build_dir = project.join(".dx/www/output");
    let home = preview_contract::handle_production_contract_request(&build_dir, "/").expect("home");
    let home_html = String::from_utf8(home.body).expect("home html");

    assert_eq!(home.status, "200 OK");
    assert_eq!(home.content_type, "text/html; charset=utf-8");
    assert_eq!(
        home.cache_control.as_deref(),
        Some("public, max-age=0, must-revalidate")
    );
    let deploy = read_json_value(build_dir.join("deploy-adapter.json"));
    let root_route = deploy["routes"]
        .as_array()
        .expect("deploy routes")
        .iter()
        .find(|route| route["path"] == "/")
        .expect("root deploy route");
    let root_html_path = root_route["html"].as_str().expect("root html path");
    assert_eq!(root_html_path, "app/index.html");
    assert_eq!(root_route.get("packet"), None);
    assert_eq!(
        home_html,
        fs::read_to_string(build_dir.join(root_html_path)).expect("root html output")
    );
    assert!(home_html.contains("data-dx-runtime=\"static\""));
    assert!(home_html.contains("data-dx-output-mode=\"tiny-static\""));
    assert!(home_html.contains("data-dx-js=\"none\""));
    assert!(home_html.contains("Enhanced Development Experience"));
    assert_eq!(home.contract_path, "deploy-adapter.json");

    let packet =
        preview_contract::handle_production_contract_request(&build_dir, "/app/index.dxpk")
            .expect("packet");
    assert_eq!(packet.status, "404 Not Found");
    assert_eq!(packet.cache_control.as_deref(), Some("no-store"));
    assert!(deploy["immutable_assets"]
        .as_array()
        .expect("immutable assets")
        .iter()
        .all(|asset| asset["path"] != "app/index.dxpk"));
    assert!(deploy["immutable_assets"]
        .as_array()
        .expect("immutable assets")
        .iter()
        .all(|asset| !asset["path"]
            .as_str()
            .expect("asset path")
            .starts_with("source-routes/")));

    let ready = preview_contract::handle_production_contract_request(&build_dir, "/.dx/ready")
        .expect("ready");
    let ready_body: serde_json::Value = serde_json::from_slice(&ready.body).expect("ready json");
    assert_eq!(ready.status, "200 OK");
    assert_eq!(ready.content_type, "application/json; charset=utf-8");
    assert_eq!(ready.cache_control.as_deref(), Some("no-store"));
    assert_eq!(ready_body["ok"], true);
    assert_eq!(ready_body["ready"], true);
    assert_eq!(ready_body["production_contract"], true);

    let missing_health =
        preview_contract::handle_production_contract_request(&build_dir, "/api/health")
            .expect("missing health");
    assert_eq!(missing_health.status, "404 Not Found");
    assert!(String::from_utf8(missing_health.body)
        .expect("missing health body")
        .contains("not listed in deploy-adapter.json"));

    let missing =
        preview_contract::handle_production_contract_request(&build_dir, "/pages/index.dxob")
            .expect("missing");
    assert_eq!(missing.status, "404 Not Found");
    assert!(String::from_utf8(missing.body)
        .expect("missing body")
        .contains("not listed in deploy-adapter.json"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_build_emits_route_handler_conformance_matrix_for_app_api_routes() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("route-handler-matrix-app").expect("dx new");

    let project = dir.path().join("route-handler-matrix-app");
    let project_cli = Cli::with_cwd(project.clone());
    fs::create_dir_all(project.join("app/api/health")).expect("api health dir");
    fs::write(
        project.join("app/api/health/route.ts"),
        r#"export function GET() {
  return Response.json({
    ok: true,
    source: "route-handler-matrix",
  });
}
"#,
    )
    .expect("health route handler");
    project_cli.cmd_build().expect("dx build");

    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    assert_eq!(
        deploy["route_handler_conformance_matrix"]["path"],
        "route-handler-conformance-matrix.json"
    );
    assert_eq!(
        deploy["route_handler_conformance_matrix"]["hosted_provider_proof"],
        false
    );

    let matrix =
        read_json_value(project.join(".dx/www/output/route-handler-conformance-matrix.json"));
    assert_eq!(
        matrix["schema"],
        "dx.www.deploy.route_handler_conformance_matrix"
    );
    assert_eq!(
        matrix["matrix_status"],
        "local-route-handler-conformance-foundation"
    );
    assert_eq!(matrix["release_ready"], false);
    assert_eq!(matrix["hosted_provider_proof"], false);
    assert_eq!(matrix["coverage"]["route_handlers"], 1);
    assert_eq!(matrix["coverage"]["get_head_preview_checks"], 2);
    assert_eq!(matrix["coverage"]["automatic_options_cases"], 1);
    assert_eq!(matrix["coverage"]["method_not_allowed_cases"], 1);
    let health_route = matrix["routes"]
        .as_array()
        .expect("matrix routes")
        .iter()
        .find(|route| route["path"] == "/api/health")
        .expect("health route in conformance matrix");
    let replay_cases = health_route["local_replay_cases"]
        .as_array()
        .expect("local replay cases");
    assert!(replay_cases.iter().any(|case| case["method"] == "GET"
        && case["expected_status"] == "200 OK"
        && case["proof"] == "production-preview-health-check"
        && case["execution_scope"] == "local-production-preview"));
    assert!(replay_cases.iter().any(|case| case["method"] == "HEAD"
        && case["expected_status"] == "200 OK"
        && case["proof"] == "production-preview-health-check"
        && case["execution_scope"] == "local-production-preview"));
    assert!(replay_cases.iter().any(|case| case["method"] == "OPTIONS"
        && case["expected_status"] == "204 No Content"
        && case["proof"] == "automatic_route_handler_options_response"
        && case["execution_scope"] == "source-owned-dev-runtime"));
    assert_eq!(
        health_route["method_not_allowed_case"]["expected_status"],
        "405 Method Not Allowed"
    );
    assert_eq!(
        health_route["method_not_allowed_case"]["expectation_source"],
        "local-production-preview-method-guard-contract"
    );
    assert_eq!(
        health_route["method_not_allowed_case"]["proof_status"],
        "expected-contract-not-hosted-replayed"
    );
    assert!(matrix["not_yet_proven"]
        .as_array()
        .expect("not yet proven")
        .iter()
        .any(|item| item == "provider-hosted GET/HEAD/OPTIONS/405 matrix"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_promote_signs_build_manifest_and_rejects_tampered_release() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("promote-app").expect("dx new");

    let project = dir.path().join("promote-app");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let key_dir = project.join(".dx/forge/publisher");
    let key_report = generate_forge_publisher_key(&key_dir, "dx-www-build-publisher", false)
        .expect("publisher key");
    let build_dir = project.join(".dx/www/output");
    let unsigned_error = build_promotion::verify_build_manifest_promotion(&build_dir)
        .expect_err("unsigned build manifest should not verify");
    assert!(
        unsigned_error.contains("build-promotion.json"),
        "unsigned error should mention missing promotion receipt: {unsigned_error}"
    );

    let report_path = project.join(".dx/www/output/promotion-report.json");
    project_cli
        .cmd_promote(&[
            "--build-dir".to_string(),
            build_dir.to_string_lossy().into_owned(),
            "--key".to_string(),
            key_report.private_key_path.to_string_lossy().into_owned(),
            "--output".to_string(),
            report_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ])
        .expect("dx promote");

    let report = read_json_value(report_path.clone());
    assert_eq!(report["passed"], true);
    assert_eq!(report["verification"]["signature_verified"], true);
    assert_eq!(report["verification"]["ready_for_hosted_release"], true);
    assert_eq!(
        report["publisher_identity"]["signer"],
        "dx-www-build-publisher"
    );
    assert!(!fs::read_to_string(&report_path)
        .expect("promotion report text")
        .contains("ed25519-seed:"));

    let deploy = read_json_value(build_dir.join("deploy-adapter.json"));
    assert_eq!(deploy["build_manifest"]["signed"], true);
    assert_eq!(
        deploy["build_manifest"]["publisher"]["signer"],
        "dx-www-build-publisher"
    );
    assert!(deploy["build_manifest"]["signature"]
        .as_str()
        .expect("signature")
        .starts_with("ed25519:"));

    let promotion = read_json_value(build_dir.join("build-promotion.json"));
    assert_eq!(promotion["passed"], true);
    assert_eq!(promotion["build_manifest"]["path"], "manifest.json");
    assert_eq!(
        promotion["publisher_identity"]["signature"],
        deploy["build_manifest"]["signature"]
    );

    let verification =
        build_promotion::verify_build_manifest_promotion(&build_dir).expect("verify signed");
    assert!(verification.ready_for_hosted_release);
    assert!(verification.signature_verified);

    fs::write(build_dir.join("manifest.json"), "{}").expect("tamper manifest");
    let tampered_error = build_promotion::verify_build_manifest_promotion(&build_dir)
        .expect_err("tampered manifest should fail verification");
    assert!(
        tampered_error.contains("manifest hash"),
        "tampered error should mention manifest hash: {tampered_error}"
    );
}

#[test]
fn dx_server_action_post_endpoints_run_in_dev_and_preview_with_receipts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("action-app").expect("dx new");

    let project = dir.path().join("action-app");
    let project_cli = Cli::with_cwd(project.clone());
    fs::create_dir_all(project.join("server")).expect("server dir");
    fs::write(
        project.join("server/actions.ts"),
        r#"export async function recordWelcomeView(payload: { count: number }) {
  return {
    ok: true,
    source: "dx-www-server-action",
  };
}
"#,
    )
    .expect("server action fixture");
    project_cli.cmd_build().expect("dx build");

    let protocols = read_json_value(project.join(".dx/www/output/server-action-protocols.json"));
    let protocol = protocols
        .as_array()
        .expect("protocols")
        .iter()
        .find(|protocol| protocol["action_id"] == "server/actions.ts#recordWelcomeView")
        .expect("recordWelcomeView protocol");
    let endpoint = protocol["endpoint"].as_str().expect("endpoint");
    let action_id = protocol["action_id"].as_str().expect("action id");
    assert_eq!(protocol["request_serialization"], "typed-json-object");
    assert_eq!(protocol["request_schema"]["mode"], "typed-object");
    assert!(protocol["request_schema"]["fields"]
        .as_array()
        .expect("request schema fields")
        .iter()
        .any(|field| field["name"] == "count"
            && field["value_type"] == "number"
            && field["required"] == true));
    assert_eq!(protocol["response_schema"]["mode"], "typed-object");
    let deploy = read_json_value(project.join(".dx/www/output/deploy-adapter.json"));
    let deploy_action = deploy["server_actions"]
        .as_array()
        .expect("deploy server actions")
        .iter()
        .find(|action| action["action_id"] == action_id)
        .expect("deploy server action");
    assert_eq!(deploy_action["method"], "POST");
    assert_eq!(
        deploy_action["request_serialization"],
        protocol["request_serialization"]
    );
    assert_eq!(
        deploy_action["response_serialization"],
        protocol["response_serialization"]
    );
    assert_eq!(deploy_action["csrf_hook"], "required");
    assert_eq!(deploy_action["session_hook"], "required");
    assert_eq!(deploy_action["replay_protection"], "idempotency-key");
    assert_eq!(
        deploy_action["receipt_policy"],
        "hashes-source-session-payload-response"
    );
    assert_eq!(
        deploy_action["runtime_artifacts"]["replay_ledger"],
        "server-action-replay-ledger.json"
    );
    assert_eq!(
        deploy["server_action_replay_ledger"]["path"],
        "server-action-replay-ledger.json"
    );
    assert_eq!(
        deploy["server_action_replay_ledger"]["mode"],
        "local-preview-hash-ledger"
    );
    assert_eq!(deploy["server_action_replay_ledger"]["entry_count"], 0);
    assert_eq!(deploy["server_action_replay_ledger"]["distributed"], false);
    assert_eq!(
        deploy["server_action_replay_ledger"]["provider_hosted"],
        false
    );
    assert_eq!(
        deploy["server_action_replay_ledger"]["release_ready"],
        false
    );
    let ledger_path = project.join(".dx/www/output/server-action-replay-ledger.json");
    let initial_ledger = read_json_value(ledger_path.clone());
    assert_eq!(
        initial_ledger["schema"],
        "dx.www.server_action.replay_ledger"
    );
    assert_eq!(
        initial_ledger["manifest_hash"],
        deploy["build_manifest"]["hash"]
    );
    assert_eq!(initial_ledger["entry_count"], 0);
    assert_eq!(
        initial_ledger["entries"]
            .as_array()
            .expect("initial ledger entries")
            .len(),
        0
    );
    assert!(initial_ledger["actions"]
        .as_array()
        .expect("ledger actions")
        .iter()
        .any(|action| action["action_id"] == action_id
            && action["store_status"]
                == "local-preview-only-distributed-provider-store-not-proven"));
    let body = format!(r#"{{"action_id":"{action_id}","payload":{{"count":4}}}}"#);
    let request = format!(
        "POST {endpoint} HTTP/1.1\r\n\
Host: localhost\r\n\
Content-Type: application/json\r\n\
X-DX-CSRF: csrf-dev\r\n\
X-DX-Session: session-dev\r\n\
Idempotency-Key: action-1\r\n\
Content-Length: {}\r\n\
\r\n\
{}",
        body.len(),
        body
    );
    let translations = HashMap::new();

    let (status, content_type, dev_body) =
        Cli::handle_http_request(&project, &request, &translations);
    let dev_response: serde_json::Value =
        serde_json::from_str(&dev_body).expect("dev action response");

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    assert_eq!(dev_response["body"]["ok"], true);
    assert_eq!(dev_response["protocol"]["method"], "POST");
    assert_eq!(dev_response["protocol"]["endpoint"], endpoint);
    assert_eq!(dev_response["protocol"]["csrf_hook"], "required");
    assert_eq!(dev_response["protocol"]["session_hook"], "required");
    assert_eq!(
        dev_response["protocol"]["replay_protection"],
        "idempotency-key"
    );
    assert_eq!(
        dev_response["protocol"]["request_serialization"],
        "typed-json-object"
    );
    assert_eq!(
        dev_response["protocol"]["response_serialization"],
        "typed-json-object"
    );
    assert_eq!(
        dev_response["protocol"]["receipt_policy"],
        "hashes-source-session-payload-response"
    );
    assert_eq!(
        dev_response["protocol"]["replay_ledger"],
        "server-action-replay-ledger.json"
    );
    assert_eq!(dev_response["replay_ledger"]["mode"], "not-recorded");
    assert_eq!(dev_response["receipt"]["action_id"], action_id);
    assert_eq!(dev_response["receipt"]["replay_safe"], true);
    assert_eq!(dev_response["receipt"]["request_validated"], true);
    assert_eq!(dev_response["receipt"]["response_validated"], true);
    for hash_field in [
        "receipt_id",
        "session_hash",
        "idempotency_key_hash",
        "payload_hash",
        "response_hash",
        "request_schema_hash",
        "response_schema_hash",
    ] {
        assert!(dev_response["receipt"][hash_field]
            .as_str()
            .expect("receipt hash field")
            .starts_with("blake3:"));
    }
    assert_eq!(
        dev_response["receipt"]["validation_errors"],
        serde_json::json!([])
    );
    assert_eq!(
        dev_response["execution_model"],
        "source-owned-safe-interpreter"
    );
    assert_eq!(dev_response["lifecycle_scripts_executed"], false);
    assert_eq!(read_json_value(ledger_path.clone())["entry_count"], 0);
    for raw_secret in ["csrf-dev", "session-dev", "action-1", r#""count":4"#] {
        assert!(!dev_body.contains(raw_secret));
    }

    let invalid_body =
        format!(r#"{{"action_id":"{action_id}","payload":{{"count":"private-value"}}}}"#);
    let invalid_request = format!(
        "POST {endpoint} HTTP/1.1\r\n\
Host: localhost\r\n\
Content-Type: application/json\r\n\
X-DX-CSRF: csrf-dev\r\n\
X-DX-Session: session-dev\r\n\
Idempotency-Key: action-invalid\r\n\
Content-Length: {}\r\n\
\r\n\
{}",
        invalid_body.len(),
        invalid_body
    );
    let (invalid_status, invalid_content_type, invalid_dev_body) =
        Cli::handle_http_request(&project, &invalid_request, &translations);
    let invalid_dev_response: serde_json::Value =
        serde_json::from_str(&invalid_dev_body).expect("invalid dev action response");
    assert_eq!(invalid_status, "400 Bad Request");
    assert_eq!(invalid_content_type, "application/json; charset=utf-8");
    assert_eq!(invalid_dev_response["error"], "server-action-failed");
    assert!(invalid_dev_response["message"]
        .as_str()
        .expect("validation message")
        .contains("payload.count expected number"));
    assert!(!invalid_dev_body.contains("private-value"));

    let invalid_preview = preview_contract::handle_production_contract_http_request(
        &project.join(".dx/www/output"),
        &invalid_request,
        execute_production_contract_server_action,
    )
    .expect("invalid preview action response");
    let invalid_preview_response: serde_json::Value =
        serde_json::from_slice(&invalid_preview.body).expect("invalid preview action json");
    assert_eq!(invalid_preview.status, "400 Bad Request");
    assert_eq!(
        invalid_preview.content_type,
        "application/json; charset=utf-8"
    );
    assert_eq!(invalid_preview.cache_control.as_deref(), Some("no-store"));
    assert_eq!(invalid_preview_response["error"], "server-action-failed");
    assert_eq!(invalid_preview_response["receipt_written"], false);
    assert_eq!(invalid_preview_response["replay_safe"], false);
    assert_eq!(read_json_value(ledger_path.clone())["entry_count"], 0);
    assert!(invalid_preview_response["message"]
        .as_str()
        .expect("preview validation message")
        .contains("payload.count expected number"));
    assert!(!String::from_utf8_lossy(&invalid_preview.body).contains("private-value"));

    let preview = preview_contract::handle_production_contract_http_request(
        &project.join(".dx/www/output"),
        &request,
        execute_production_contract_server_action,
    )
    .expect("preview action response");
    let preview_response: serde_json::Value =
        serde_json::from_slice(&preview.body).expect("preview action json");

    assert_eq!(preview.status, "200 OK");
    assert_eq!(preview.content_type, "application/json; charset=utf-8");
    assert_eq!(preview.cache_control.as_deref(), Some("no-store"));
    assert_eq!(preview_response["body"]["source"], "dx-www-server-action");
    assert_eq!(preview_response["protocol"]["method"], "POST");
    assert_eq!(preview_response["protocol"]["endpoint"], endpoint);
    assert_eq!(preview_response["protocol"]["csrf_hook"], "required");
    assert_eq!(preview_response["protocol"]["session_hook"], "required");
    assert_eq!(
        preview_response["protocol"]["replay_protection"],
        "idempotency-key"
    );
    assert_eq!(
        preview_response["protocol"]["response_serialization"],
        "typed-json-object"
    );
    assert_eq!(preview_response["receipt"]["action_id"], action_id);
    assert_eq!(preview_response["receipt"]["replay_safe"], true);
    assert_eq!(preview_response["receipt"]["request_validated"], true);
    assert_eq!(preview_response["receipt"]["response_validated"], true);
    assert_eq!(
        preview_response["replay_ledger"]["mode"],
        "local-preview-hash-ledger"
    );
    assert_eq!(preview_response["replay_ledger"]["duplicate"], false);
    assert_eq!(preview_response["replay_ledger"]["observed_count"], 1);
    assert_eq!(preview_response["replay_ledger"]["entry_count"], 1);
    assert!(preview_response["replay_ledger"]["replay_key_hash"]
        .as_str()
        .expect("replay key hash")
        .starts_with("blake3:"));
    assert_eq!(
        preview_response["replay_ledger"]["receipt_id"],
        preview_response["receipt"]["receipt_id"]
    );
    let ledger_after_preview = read_json_value(ledger_path.clone());
    assert_eq!(ledger_after_preview["entry_count"], 1);
    let ledger_entry = &ledger_after_preview["entries"]
        .as_array()
        .expect("ledger entries")[0];
    assert_eq!(
        ledger_entry["receipt_id"],
        preview_response["receipt"]["receipt_id"]
    );
    assert_eq!(
        ledger_entry["session_hash"],
        preview_response["receipt"]["session_hash"]
    );
    assert_eq!(
        ledger_entry["idempotency_key_hash"],
        preview_response["receipt"]["idempotency_key_hash"]
    );
    assert_eq!(
        ledger_entry["payload_hash"],
        preview_response["receipt"]["payload_hash"]
    );
    assert_eq!(
        ledger_entry["response_hash"],
        preview_response["receipt"]["response_hash"]
    );
    let ledger_after_preview_text =
        fs::read_to_string(&ledger_path).expect("ledger after preview text");
    for raw_secret in ["csrf-dev", "session-dev", "action-1", r#""count":4"#] {
        assert!(!ledger_after_preview_text.contains(raw_secret));
    }

    let duplicate_preview = preview_contract::handle_production_contract_http_request(
        &project.join(".dx/www/output"),
        &request,
        execute_production_contract_server_action,
    )
    .expect("duplicate preview action response");
    let duplicate_response: serde_json::Value =
        serde_json::from_slice(&duplicate_preview.body).expect("duplicate preview action json");
    assert_eq!(duplicate_response["replay_ledger"]["duplicate"], true);
    assert_eq!(duplicate_response["replay_ledger"]["observed_count"], 2);
    assert_eq!(duplicate_response["replay_ledger"]["entry_count"], 1);
    assert_eq!(
        duplicate_response["replay_ledger"]["replay_key_hash"],
        preview_response["replay_ledger"]["replay_key_hash"]
    );
    assert_eq!(
        duplicate_response["replay_ledger"]["receipt_id"],
        preview_response["receipt"]["receipt_id"]
    );
    let final_ledger = read_json_value(ledger_path);
    let final_entry = &final_ledger["entries"]
        .as_array()
        .expect("final ledger entries")[0];
    assert_eq!(final_entry["duplicate_observed"], true);
    assert_eq!(final_entry["observed_count"], 2);
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_forge_react_starter_benchmark_reports_static_micro_js_and_forge_boundaries() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_new("starter-bench").expect("dx new");

    let project = dir.path().join("starter-bench");
    let project_cli = Cli::with_cwd(project.clone());
    project_cli
        .cmd_forge(&[
            "import".to_string(),
            "npm".to_string(),
            "clsx".to_string(),
            "--write".to_string(),
            "--quiet".to_string(),
        ])
        .expect("forge npm adapter");
    project_cli.cmd_build().expect("dx build");

    let output_path = project.join(".dx/forge/react-starter-benchmark.json");
    project_cli
        .cmd_forge(&[
            "react-starter-benchmark".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            output_path.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect("react starter benchmark");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert!(report["score"].as_u64().expect("score") >= 90);
    assert_eq!(report["no_node_modules"], true);
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["static_output"]["html_exists"], true);
    assert!(report["static_output"]["html_bytes"].as_u64().unwrap_or(0) > 0);
    assert_eq!(report["static_output"]["crawlable_fallback"], true);
    assert_eq!(report["micro_js_interaction"]["runtime"], "js");
    assert_eq!(report["micro_js_interaction"]["wasm_required"], false);
    assert_eq!(
        report["micro_js_interaction"]["event_binding_present"],
        true
    );
    assert_eq!(
        report["nextjs_baseline"]["baseline_kind"],
        "matching-nextjs-static-floor-fixture"
    );
    assert_eq!(report["nextjs_baseline"]["next_build_run"], false);
    assert_eq!(report["nextjs_baseline"]["package_installs_run"], false);
    assert!(
        report["nextjs_baseline"]["nextjs_static_floor_brotli_bytes"]
            .as_u64()
            .unwrap_or(0)
            > 0
    );
    assert_eq!(
        report["architecture_boundaries"]["dx_owned_www_framework"],
        true
    );
    assert_eq!(
        report["architecture_boundaries"]["next_familiar_authoring"],
        true
    );
    assert_eq!(
        report["architecture_boundaries"]["forge_first_no_node_modules_default"],
        true
    );
    assert_eq!(report["forge_boundaries"]["source_manifest_exists"], true);
    assert!(
        report["forge_boundaries"]["package_count"]
            .as_u64()
            .unwrap_or(0)
            >= 1
    );
    assert!(
        report["forge_boundaries"]["source_owned_file_count"]
            .as_u64()
            .unwrap_or(0)
            >= 1
    );
    assert!(report["claim_boundaries"]
        .as_array()
        .expect("claim boundaries")
        .iter()
        .any(|boundary| boundary
            .as_str()
            .unwrap_or_default()
            .contains("not a full Next.js replacement benchmark")));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn forge_add_dry_run_does_not_write_files() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let args = vec![
        "add".to_string(),
        "ui/button".to_string(),
        "--dry-run".to_string(),
    ];

    cli.cmd_forge(&args).expect("forge add dry-run");

    assert!(!dir.path().join("components/ui/button.tsx").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_add_write_creates_manifest_and_receipt() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let args = vec![
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ];

    cli.cmd_forge(&args).expect("forge add write");

    assert!(dir.path().join("components/ui/button.tsx").exists());
    assert!(dir.path().join("components/ui/slot.tsx").exists());
    assert!(dir.path().join("lib/utils.ts").exists());
    assert!(dir.path().join(".dx/forge/source-manifest.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_rollback_write_restores_source_from_receipt() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ])
    .expect("forge add write");
    let receipt_path = first_receipt_path(dir.path());
    let button_path = dir.path().join("components/ui/button.tsx");
    fs::write(&button_path, "broken local edit").expect("local edit");

    cli.cmd_forge(&[
        "rollback".to_string(),
        receipt_path.to_string_lossy().into_owned(),
        "--write".to_string(),
    ])
    .expect("forge rollback write");

    let restored = fs::read_to_string(&button_path).expect("button");
    assert!(restored.contains("buttonVariants"));
    assert!(restored.contains("export { Button, buttonVariants };"));
    assert!(!restored.contains("broken local edit"));
    assert!(!dir.path().join("node_modules").exists());
    let receipts = fs::read_dir(dir.path().join(".dx/forge/receipts"))
        .expect("receipts")
        .count();
    assert!(receipts >= 2);
}

#[test]
fn forge_rollback_dry_run_does_not_restore_files() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ])
    .expect("forge add write");
    let receipt_path = first_receipt_path(dir.path());
    let button_path = dir.path().join("components/ui/button.tsx");
    fs::write(&button_path, "broken local edit").expect("local edit");

    cli.cmd_forge(&[
        "rollback".to_string(),
        receipt_path.to_string_lossy().into_owned(),
        "--dry-run".to_string(),
    ])
    .expect("forge rollback dry-run");

    let current = fs::read_to_string(&button_path).expect("button");
    assert_eq!(current, "broken local edit");
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_add_icon_search_writes_selected_icon_package() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let args = vec![
        "add".to_string(),
        "icon/search".to_string(),
        "--write".to_string(),
    ];

    cli.cmd_forge(&args).expect("forge add icon/search");

    assert!(dir.path().join("components/icons/search.tsx").exists());
    assert!(dir.path().join("components/icons/README.md").exists());
    assert!(dir.path().join("lib/icons.ts").exists());
    assert!(dir.path().join(".dx/forge/source-manifest.json").exists());
    assert!(!dir.path().join("components/ui/button.tsx").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_ui_button_writes_source_owned_package() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["ui/button"]).expect("dx add ui/button");

    assert!(dir.path().join("components/ui/button.tsx").exists());
    assert!(dir.path().join("components/ui/slot.tsx").exists());
    assert!(dir.path().join("lib/utils.ts").exists());
    assert!(dir.path().join(".dx/forge/source-manifest.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_ui_button_does_not_run_scripts_or_create_node_modules() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("package.json"),
        r#"{"scripts":{"prepare":"node -e \"require('fs').writeFileSync('sentinel','bad')\"}}"#,
    )
    .expect("write package");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["ui/button"]).expect("dx add ui/button");

    assert!(dir.path().join("components/ui/button.tsx").exists());
    assert!(!dir.path().join("node_modules").exists());
    assert!(!dir.path().join("sentinel").exists());
}

#[test]
fn dx_add_ui_button_dry_run_writes_nothing() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["ui/button", "--dry-run"])
        .expect("dx add ui/button dry-run");

    assert!(!dir.path().join("components/ui/button.tsx").exists());
    assert!(!dir.path().join(".dx/forge/source-manifest.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_ui_button_variant_writes_isolated_package_fork() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["ui/button", "--variant", "marketing"])
        .expect("dx add ui/button --variant marketing");

    let manifest =
        fs::read_to_string(dir.path().join(".dx/forge/source-manifest.json")).expect("manifest");

    assert!(dir
        .path()
        .join("components/ui/variants/marketing/button.tsx")
        .exists());
    assert!(dir
        .path()
        .join("components/ui/variants/marketing/slot.tsx")
        .exists());
    assert!(dir
        .path()
        .join("lib/forge/variants/marketing/utils.ts")
        .exists());
    assert!(!dir.path().join("components/ui/button.tsx").exists());
    assert!(manifest.contains(r#""variant": "marketing""#));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_icon_search_writes_selected_source_owned_icon() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["icon", "search"])
        .expect("dx add icon search");

    let manifest_path = dir.path().join(".dx/forge/source-manifest.json");
    let manifest = fs::read_to_string(&manifest_path).expect("manifest");

    assert!(dir.path().join("components/icons/search.tsx").exists());
    assert!(dir.path().join("components/icons/README.md").exists());
    assert!(dir.path().join("lib/icons.ts").exists());
    assert!(manifest.contains("dx/icon/search"));
    assert!(!dir.path().join("components/ui/button.tsx").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_icon_search_dry_run_writes_nothing() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["icon", "search", "--dry-run"])
        .expect("dx add icon search dry-run");

    assert!(!dir.path().join("components/icons/search.tsx").exists());
    assert!(!dir.path().join("lib/icons.ts").exists());
    assert!(!dir.path().join(".dx/forge/source-manifest.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_auth_google_writes_source_owned_auth_package() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["auth/better-auth"])
        .expect("dx add auth/better-auth");

    let manifest_path = dir.path().join(".dx/forge/source-manifest.json");
    let manifest = fs::read_to_string(&manifest_path).expect("manifest");

    assert!(dir.path().join("auth/better-auth/options.ts").exists());
    assert!(dir.path().join("auth/better-auth/server.ts").exists());
    assert!(dir.path().join("auth/better-auth/client.ts").exists());
    assert!(dir.path().join("auth/better-auth/route.ts").exists());
    assert!(dir.path().join("auth/better-auth/metadata.ts").exists());
    assert!(dir.path().join("auth/better-auth/.env.example").exists());
    assert!(dir.path().join("auth/better-auth/README.md").exists());
    assert!(manifest.contains("auth/better-auth"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_auth_google_dry_run_writes_nothing() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["auth/better-auth", "--dry-run"])
        .expect("dx add auth/better-auth dry-run");

    assert!(!dir.path().join("auth/better-auth/options.ts").exists());
    assert!(!dir.path().join(".dx/forge/source-manifest.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_better_auth_alias_writes_source_owned_auth_package() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["better-auth"]).expect("dx add better-auth");

    let manifest_path = dir.path().join(".dx/forge/source-manifest.json");
    let manifest = fs::read_to_string(&manifest_path).expect("manifest");

    assert!(dir.path().join("auth/better-auth/options.ts").exists());
    assert!(dir.path().join("auth/better-auth/server.ts").exists());
    assert!(dir.path().join("auth/better-auth/client.ts").exists());
    assert!(dir.path().join("auth/better-auth/route.ts").exists());
    assert!(dir.path().join("auth/better-auth/metadata.ts").exists());
    assert!(dir.path().join("auth/better-auth/.env.example").exists());
    assert!(manifest.contains("auth/better-auth"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_next_intl_alias_writes_source_owned_i18n_package() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["next-intl"]).expect("dx add next-intl");

    let manifest_path = dir.path().join(".dx/forge/source-manifest.json");
    let manifest = fs::read_to_string(&manifest_path).expect("manifest");

    assert!(dir.path().join("i18n/routing.ts").exists());
    assert!(dir.path().join("i18n/navigation.ts").exists());
    assert!(dir.path().join("i18n/request.ts").exists());
    assert!(dir.path().join("i18n/middleware.ts").exists());
    assert!(dir.path().join("i18n/provider.tsx").exists());
    assert!(dir.path().join("i18n/metadata.ts").exists());
    assert!(manifest.contains("i18n/next-intl"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_migration_static_site_writes_honest_source_owned_example() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["migration/static-site"])
        .expect("dx add migration/static-site");

    let manifest_path = dir.path().join(".dx/forge/source-manifest.json");
    let manifest = fs::read_to_string(&manifest_path).expect("manifest");
    let readme_path = dir.path().join("migrations/static-site/README.md");
    let readme = fs::read_to_string(&readme_path).expect("migration readme");

    assert!(dir
        .path()
        .join("migrations/static-site/content.ts")
        .exists());
    assert!(dir.path().join("migrations/static-site/page.tsx").exists());
    assert!(dir
        .path()
        .join("migrations/static-site/sample-wordpress-export.json")
        .exists());
    assert!(manifest.contains("migration/static-site"));
    assert!(readme.contains("not a full WordPress plugin or theme migration"));
    assert!(readme.contains("No package install is required"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migration_audit_reports_static_export_risks_without_installing_packages() {
    let dir = tempdir().expect("tempdir");
    let export_dir = dir.path().join("exports");
    fs::create_dir_all(&export_dir).expect("export dir");
    let input_path = export_dir.join("hello-world.html");
    fs::write(
        &input_path,
        r#"<!doctype html>
<html>
  <head>
    <title>Hello migrated page</title>
    <meta name="description" content="A small WordPress export fixture">
    <link rel="canonical" href="https://example.test/hello-world/">
    <link rel="stylesheet" href="/wp-content/themes/example/style.css">
    <meta http-equiv="refresh" content="0; url=/new-home">
  </head>
  <body>
    <article>
      <h1>Hello migrated page</h1>
      <img src="/wp-content/uploads/hero.jpg" alt="Hero">
      [contact-form-7 id="42"]
      <form action="/wp-json/contact-form-7/v1/contact-forms/42/feedback"></form>
      <section id="comments">Legacy comments</section>
      <a onclick="trackClick()" href="javascript:alert('x')">Unsafe legacy link</a>
      <script src="/wp-content/plugins/gallery/gallery.js"></script>
    </article>
  </body>
</html>"#,
    )
    .expect("html fixture");
    fs::write(
        dir.path().join("package.json"),
        r#"{"scripts":{"prepare":"node -e \"require('fs').writeFileSync('sentinel','bad')\"}}"#,
    )
    .expect("package json");

    let output_path = dir.path().join("migration-audit.json");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "migration-audit".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        input_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "60".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge migration-audit");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_node_modules"], true);
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["page_count"], 1);
    assert!(report["asset_count"].as_u64().expect("asset count") >= 3);
    assert_eq!(report["redirect_count"], 1);
    assert_eq!(report["metadata"]["title"], "Hello migrated page");
    assert_eq!(
        report["metadata"]["description"],
        "A small WordPress export fixture"
    );
    assert_eq!(
        report["metadata"]["canonical_url"],
        "https://example.test/hello-world/"
    );
    assert!(report["dynamic_gaps"]
        .as_array()
        .expect("dynamic gaps")
        .iter()
        .any(|gap| gap["code"] == "wordpress-shortcode"));
    assert!(report["dynamic_gaps"]
        .as_array()
        .expect("dynamic gaps")
        .iter()
        .any(|gap| gap["code"] == "legacy-form"));
    assert!(report["unsafe_html_reviews"]
        .as_array()
        .expect("unsafe reviews")
        .iter()
        .any(|review| review["code"] == "script-tag"));
    assert!(report["unsafe_html_reviews"]
        .as_array()
        .expect("unsafe reviews")
        .iter()
        .any(|review| review["code"] == "inline-event-handler"));
    assert!(report["next_commands"]
        .as_array()
        .expect("next commands")
        .iter()
        .any(|command| command
            .as_str()
            .is_some_and(|value| value.contains("dx add migration/static-site --write"))));
    assert!(!dir.path().join("sentinel").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migrate_static_page_writes_source_owned_route_without_package_installs() {
    let dir = tempdir().expect("tempdir");
    let export_dir = dir.path().join("exports");
    fs::create_dir_all(&export_dir).expect("export dir");
    let input_path = export_dir.join("about.html");
    fs::write(
        &input_path,
        r#"<!doctype html>
<html>
  <head>
    <title>About DX Forge</title>
    <meta name="description" content="A clean static migration fixture">
    <link rel="canonical" href="https://example.test/about/">
  </head>
  <body>
    <article>
      <h1>About DX Forge</h1>
      <p>Source-owned static migration keeps the route editable.</p>
      <img src="/wp-content/uploads/hero.jpg" alt="Migration hero">
    </article>
  </body>
</html>"#,
    )
    .expect("html fixture");
    fs::write(
        dir.path().join("package.json"),
        r#"{"scripts":{"prepare":"node -e \"require('fs').writeFileSync('sentinel','bad')\"}}"#,
    )
    .expect("package json");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let dry_report_path = dir.path().join("migrate-static-page-dry.json");
    cli.cmd_forge(&[
        "migrate-static-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        input_path.to_string_lossy().into_owned(),
        "--route".to_string(),
        "/migrated/about".to_string(),
        "--dry-run".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        dry_report_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge migrate-static-page dry-run");

    let dry_report = read_json_value(dry_report_path);
    assert_eq!(dry_report["passed"], true);
    assert_eq!(dry_report["mode"], "dry-run");
    assert_eq!(dry_report["route"], "/migrated/about");
    assert_eq!(dry_report["slug"], "about");
    assert_eq!(dry_report["wrote_files"], false);
    assert_eq!(dry_report["package_installs_run"], false);
    assert_eq!(dry_report["no_node_modules"], true);
    assert_eq!(
        dry_report["base_package"]["package_id"],
        "migration/static-site"
    );
    assert_eq!(dry_report["base_package"]["wrote_files"], false);
    assert!(dry_report["source_files"]
        .as_array()
        .expect("source files")
        .iter()
        .any(|file| file["kind"] == "generated-content" && file["write_state"] == "planned"));
    assert!(!dir
        .path()
        .join("migrations/static-site/generated/about/content.ts")
        .exists());
    assert!(!dir.path().join(".dx/forge/source-manifest.json").exists());
    assert!(!dir.path().join("sentinel").exists());
    assert!(!dir.path().join("node_modules").exists());

    let write_report_path = dir.path().join("migrate-static-page-write.json");
    cli.cmd_forge(&[
        "migrate-static-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        input_path.to_string_lossy().into_owned(),
        "--route".to_string(),
        "/migrated/about".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        write_report_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge migrate-static-page write");

    let write_report = read_json_value(write_report_path);
    assert_eq!(write_report["passed"], true);
    assert_eq!(write_report["mode"], "write");
    assert_eq!(write_report["wrote_files"], true);
    assert_eq!(write_report["package_installs_run"], false);
    assert_eq!(write_report["no_node_modules"], true);
    assert_eq!(write_report["base_package"]["wrote_files"], true);
    assert!(write_report["source_files"]
        .as_array()
        .expect("source files")
        .iter()
        .any(|file| file["kind"] == "generated-page" && file["write_state"] == "written"));

    let content_path = dir
        .path()
        .join("migrations/static-site/generated/about/content.ts");
    let page_path = dir
        .path()
        .join("migrations/static-site/generated/about/page.tsx");
    let readme_path = dir
        .path()
        .join("migrations/static-site/generated/about/README.md");
    let source_path = dir
        .path()
        .join("migrations/static-site/generated/about/source.html");
    let manifest_path = dir.path().join(".dx/forge/source-manifest.json");

    assert!(dir
        .path()
        .join("migrations/static-site/content.ts")
        .exists());
    assert!(dir.path().join("migrations/static-site/page.tsx").exists());
    assert!(content_path.exists());
    assert!(page_path.exists());
    assert!(readme_path.exists());
    assert!(source_path.exists());
    assert!(manifest_path.exists());

    let generated_content = fs::read_to_string(&content_path).expect("generated content");
    let generated_page = fs::read_to_string(&page_path).expect("generated page");
    let generated_readme = fs::read_to_string(&readme_path).expect("generated readme");
    let source_html = fs::read_to_string(&source_path).expect("source html");
    let manifest = fs::read_to_string(&manifest_path).expect("manifest");

    assert!(generated_content.contains("About DX Forge"));
    assert!(generated_content.contains("/migrated/about"));
    assert!(generated_content.contains("/assets/migrated/about/hero.jpg"));
    assert!(generated_content.contains("manualReviewRequired"));
    assert!(generated_page.contains("StaticSiteMigrationPage"));
    assert!(generated_page.contains("generatedStaticMigrationPage"));
    assert!(generated_readme.contains("No package install is required"));
    assert!(source_html.contains("<article>"));
    assert!(manifest.contains("migration/static-site"));
    assert!(!dir.path().join("sentinel").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migrate_static_page_preserves_existing_local_route_edits() {
    let dir = tempdir().expect("tempdir");
    let export_dir = dir.path().join("exports");
    fs::create_dir_all(&export_dir).expect("export dir");
    let input_path = export_dir.join("about.html");
    fs::write(
        &input_path,
        r#"<!doctype html>
<html>
  <head>
    <title>About DX Forge</title>
    <meta name="description" content="A clean static migration fixture">
    <link rel="canonical" href="https://example.test/about/">
  </head>
  <body>
    <article>
      <h1>About DX Forge</h1>
      <p>Source-owned static migration keeps the route editable.</p>
    </article>
  </body>
</html>"#,
    )
    .expect("html fixture");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "migrate-static-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        input_path.to_string_lossy().into_owned(),
        "--route".to_string(),
        "/migrated/about".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        dir.path()
            .join("migrate-static-page-initial.json")
            .to_string_lossy()
            .into_owned(),
        "--quiet".to_string(),
    ])
    .expect("initial migrate-static-page write");

    let content_path = dir
        .path()
        .join("migrations/static-site/generated/about/content.ts");
    let original_content = fs::read_to_string(&content_path).expect("generated content");
    let edited_content = format!(
        "{}\n// local editor change that Forge must not overwrite\n",
        original_content
    );
    fs::write(&content_path, &edited_content).expect("local edit");

    let rerun_report_path = dir.path().join("migrate-static-page-preserve.json");
    cli.cmd_forge(&[
        "migrate-static-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        input_path.to_string_lossy().into_owned(),
        "--route".to_string(),
        "/migrated/about".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        rerun_report_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("rerun migrate-static-page should preserve local edits");

    let report = read_json_value(rerun_report_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["status"], "needs-review");
    assert_eq!(report["edit_preservation"]["traffic"], "yellow");
    assert_eq!(report["edit_preservation"]["preserved_file_count"], 1);
    assert_eq!(report["edit_preservation"]["overwritten_file_count"], 0);
    assert!(report["source_files"]
        .as_array()
        .expect("source files")
        .iter()
        .any(|file| file["kind"] == "generated-content"
            && file["write_state"] == "preserved-local-edit"));
    assert!(report["edit_preservation"]["files"]
        .as_array()
        .expect("preservation files")
        .iter()
        .any(|file| file["project_relative_path"]
            == "migrations/static-site/generated/about/content.ts"
            && file["traffic"] == "yellow"));
    assert_eq!(
        fs::read_to_string(&content_path).expect("preserved content"),
        edited_content
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migrate_static_page_writes_asset_manifest_with_hashes_and_gaps() {
    let dir = tempdir().expect("tempdir");
    let export_dir = dir.path().join("exports");
    let upload_dir = export_dir.join("wp-content/uploads");
    fs::create_dir_all(&upload_dir).expect("upload dir");
    let input_path = export_dir.join("landing.html");
    let hero_path = upload_dir.join("hero.jpg");
    fs::write(&hero_path, b"local hero bytes").expect("hero asset");
    fs::write(
        &input_path,
        r#"<!doctype html>
<html>
  <head>
    <title>Landing Page</title>
    <meta name="description" content="Asset manifest fixture">
    <link rel="canonical" href="https://example.test/landing/">
  </head>
  <body>
    <article>
      <h1>Landing Page</h1>
      <img src="/wp-content/uploads/hero.jpg" alt="Hero image">
      <img src="/wp-content/uploads/missing.jpg">
    </article>
  </body>
</html>"#,
    )
    .expect("html fixture");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let report_path = dir.path().join("migrate-static-page-assets.json");
    cli.cmd_forge(&[
        "migrate-static-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        input_path.to_string_lossy().into_owned(),
        "--route".to_string(),
        "/migrated/landing".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        report_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge migrate-static-page assets");

    let report = read_json_value(report_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["asset_manifest"]["asset_count"], 2);
    assert_eq!(report["asset_manifest"]["resolved_asset_count"], 1);
    assert_eq!(report["asset_manifest"]["unresolved_media_gap_count"], 1);
    assert_eq!(report["asset_manifest"]["write_state"], "written");
    assert!(report["source_files"]
        .as_array()
        .expect("source files")
        .iter()
        .any(|file| file["kind"] == "asset-manifest" && file["write_state"] == "written"));

    let manifest_path = dir
        .path()
        .join("migrations/static-site/generated/landing/asset-manifest.json");
    assert!(manifest_path.exists());
    let manifest = read_json_value(manifest_path);
    let assets = manifest["assets"].as_array().expect("manifest assets");
    let resolved = assets
        .iter()
        .find(|asset| asset["source_url"] == "/wp-content/uploads/hero.jpg")
        .expect("resolved hero asset");
    assert_eq!(
        resolved["copied_target_path"],
        "/assets/migrated/landing/hero.jpg"
    );
    assert_eq!(resolved["byte_size"], 16);
    assert!(resolved["hash"]
        .as_str()
        .is_some_and(|hash| hash.starts_with("blake3:") && hash.len() > 20));
    assert_eq!(resolved["cache_hint"], "copy-optimize-cache-long");
    assert_eq!(resolved["alt_text_review_state"], "present-review-required");

    let gap = manifest["unresolved_media_gaps"]
        .as_array()
        .expect("gaps")
        .iter()
        .find(|gap| gap["source_url"] == "/wp-content/uploads/missing.jpg")
        .expect("missing media gap");
    assert_eq!(
        gap["reason"],
        "source asset file was not found in the export folder"
    );
    assert_eq!(gap["alt_text_review_state"], "missing");
    assert!(!dir.path().join("node_modules").exists());
}
