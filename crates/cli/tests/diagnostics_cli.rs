use std::fs;
use std::process::Command;

use tempfile::tempdir;

#[test]
fn build_process_emits_dx_code_frame_for_invalid_dx_config() {
    let project = tempdir().expect("temp project");
    fs::write(project.path().join("dx"), "project.name=\n").expect("write invalid dx config");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .arg("build")
        .output()
        .expect("run dx-www build");

    assert!(
        !output.status.success(),
        "build should fail for invalid dx config"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("DX-WWW error: Config parse failed"),
        "{stderr}"
    );
    assert!(stderr.contains("--> dx:"), "{stderr}");
    assert!(stderr.contains("project.name"), "{stderr}");
    assert!(stderr.contains('^'), "{stderr}");
    assert!(
        !project.path().join(".dx").exists(),
        "invalid config should fail before build output is created"
    );
}

#[test]
fn build_process_emits_dx_code_frame_for_invalid_legacy_toml_config() {
    let project = tempdir().expect("temp project");
    fs::write(
        project.path().join("dx.config.toml"),
        "[project\nname = \"demo\"\n",
    )
    .expect("write invalid legacy config");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .arg("build")
        .output()
        .expect("run dx-www build");

    assert!(
        !output.status.success(),
        "build should fail for invalid legacy config"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("DX-WWW error: Config parse failed"),
        "{stderr}"
    );
    assert!(stderr.contains("--> dx.config.toml:"), "{stderr}");
    assert!(stderr.contains("[project"), "{stderr}");
    assert!(stderr.contains('^'), "{stderr}");
    assert!(
        !project.path().join(".dx").exists(),
        "invalid config should fail before build output is created"
    );
}

#[test]
fn build_process_emits_dx_code_frame_for_invalid_app_route_tsx() {
    let project = tempdir().expect("temp project");
    let app_dir = project.path().join("app");
    fs::create_dir_all(&app_dir).expect("create app dir");
    fs::write(
        app_dir.join("page.tsx"),
        "import Broken;\n\nexport default function Page() {\n  return <main>Launch</main>;\n}\n",
    )
    .expect("write invalid app route");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .arg("build")
        .output()
        .expect("run dx-www build");

    assert!(
        !output.status.success(),
        "build should fail for invalid app route TSX"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("DX-WWW error: Parse failed"), "{stderr}");
    assert!(stderr.contains("--> app/page.tsx:"), "{stderr}");
    assert!(stderr.contains("Import declaration is missing"), "{stderr}");
    assert!(stderr.contains("import Broken"), "{stderr}");
    assert!(stderr.contains('^'), "{stderr}");
    assert!(
        !project.path().join(".dx/build/app/index.html").exists(),
        "invalid app route should not emit route output"
    );
}

#[test]
fn build_process_emits_dx_code_frame_for_invalid_dx_style_class() {
    let project = tempdir().expect("temp project");
    let app_dir = project.path().join("app");
    fs::create_dir_all(&app_dir).expect("create app dir");
    fs::write(
        app_dir.join("page.tsx"),
        "export default function Page() {\n  return <main className=\"md:(grid gap-4\">Launch</main>;\n}\n",
    )
    .expect("write app route with invalid dx-style class");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .arg("build")
        .output()
        .expect("run dx-www build");

    assert!(
        !output.status.success(),
        "build should fail for invalid dx-style class"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("DX-WWW error: Unsupported dx-style class"),
        "{stderr}"
    );
    assert!(stderr.contains("--> app/page.tsx:"), "{stderr}");
    assert!(
        stderr.contains("grouped classname syntax is invalid"),
        "{stderr}"
    );
    assert!(stderr.contains("md:(grid gap-4"), "{stderr}");
    assert!(stderr.contains('^'), "{stderr}");
    assert!(
        !project.path().join(".dx/build/app/index.html").exists(),
        "invalid dx-style source should not emit route output"
    );
}

#[test]
fn build_process_emits_dx_code_frame_for_invalid_component_dx_style_class() {
    let project = tempdir().expect("temp project");
    let app_dir = project.path().join("app");
    let components_dir = project.path().join("components");
    fs::create_dir_all(&app_dir).expect("create app dir");
    fs::create_dir_all(&components_dir).expect("create components dir");
    fs::write(
        app_dir.join("page.tsx"),
        "import { BrokenCard } from \"@/components/BrokenCard\";\n\nexport default function Page() {\n  return <BrokenCard />;\n}\n",
    )
    .expect("write app route");
    fs::write(
        components_dir.join("BrokenCard.tsx"),
        "export function BrokenCard() {\n  return <section className=\"md:(grid gap-4\">Broken</section>;\n}\n",
    )
    .expect("write component with invalid dx-style class");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .arg("build")
        .output()
        .expect("run dx-www build");

    assert!(
        !output.status.success(),
        "build should fail for invalid component dx-style class"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("DX-WWW error: Unsupported dx-style class"),
        "{stderr}"
    );
    assert!(
        stderr.contains("--> components/BrokenCard.tsx:"),
        "{stderr}"
    );
    assert!(
        stderr.contains("grouped classname syntax is invalid"),
        "{stderr}"
    );
    assert!(stderr.contains("md:(grid gap-4"), "{stderr}");
    assert!(stderr.contains('^'), "{stderr}");
    assert!(
        !project.path().join(".dx/build/app/index.html").exists(),
        "invalid component dx-style source should not emit route output"
    );
}

#[test]
fn build_process_emits_dx_code_frame_for_unresolved_local_module_import() {
    let project = tempdir().expect("temp project");
    let app_dir = project.path().join("app");
    fs::create_dir_all(&app_dir).expect("create app dir");
    fs::write(
        app_dir.join("page.tsx"),
        "import { MissingWidget } from \"./missing-widget\";\n\nexport default function Page() {\n  return <MissingWidget />;\n}\n",
    )
    .expect("write app route with unresolved import");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .arg("build")
        .output()
        .expect("run dx-www build");

    assert!(
        !output.status.success(),
        "build should fail for unresolved local imports"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("DX-WWW error: Compilation failed"),
        "{stderr}"
    );
    assert!(stderr.contains("--> app/page.tsx:"), "{stderr}");
    assert!(
        stderr.contains("Cannot resolve local source import `./missing-widget` from app/page.tsx."),
        "{stderr}"
    );
    assert!(
        stderr.contains("import { MissingWidget } from \"./missing-widget\";"),
        "{stderr}"
    );
    assert!(stderr.contains("^^^^^^^^^^^^^^^^"), "{stderr}");
    assert!(
        !project.path().join(".dx/build/app/index.html").exists(),
        "unresolved imports should not emit route output"
    );
}

#[test]
fn build_process_emits_dx_code_frame_for_invalid_route_handler_ts() {
    let project = tempdir().expect("temp project");
    let app_dir = project.path().join("app");
    let route_dir = project.path().join("app/api/health");
    fs::create_dir_all(&app_dir).expect("create app dir");
    fs::create_dir_all(&route_dir).expect("create route dir");
    fs::write(
        app_dir.join("page.tsx"),
        "export default function Page() {\n  return <main>Launch</main>;\n}\n",
    )
    .expect("write app route");
    fs::write(
        route_dir.join("route.ts"),
        "export async function GET() {\n  return Response.json({ ok: true });\n",
    )
    .expect("write invalid route handler");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .arg("build")
        .output()
        .expect("run dx-www build");

    assert!(
        !output.status.success(),
        "build should fail for invalid route handler TS"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("DX-WWW error: Parse failed"), "{stderr}");
    assert!(stderr.contains("--> app/api/health/route.ts:"), "{stderr}");
    assert!(stderr.contains("export async function GET()"), "{stderr}");
    assert!(stderr.contains('^'), "{stderr}");
    assert!(
        !project.path().join(".dx/build/app/index.html").exists(),
        "invalid route handler source should not emit route output"
    );
}

#[test]
fn build_process_emits_dx_code_frame_for_invalid_css_source() {
    let project = tempdir().expect("temp project");
    let app_dir = project.path().join("app");
    let styles_dir = project.path().join("styles");
    fs::create_dir_all(&app_dir).expect("create app dir");
    fs::create_dir_all(&styles_dir).expect("create styles dir");
    fs::write(
        app_dir.join("page.tsx"),
        "export default function Page() {\n  return <main>Launch</main>;\n}\n",
    )
    .expect("write app route");
    fs::write(
        styles_dir.join("app.css"),
        ".shell {\n  color: var(--dx-foreground);\n",
    )
    .expect("write invalid CSS source");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .arg("build")
        .output()
        .expect("run dx-www build");

    assert!(
        !output.status.success(),
        "build should fail for invalid CSS source"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("DX-WWW error: Parse failed"), "{stderr}");
    assert!(stderr.contains("--> styles/app.css:"), "{stderr}");
    assert!(
        stderr.contains("CSS block is missing a closing `}`."),
        "{stderr}"
    );
    assert!(stderr.contains(".shell {"), "{stderr}");
    assert!(stderr.contains('^'), "{stderr}");
    assert!(
        !project.path().join(".dx/build/app/index.html").exists(),
        "invalid CSS source should not emit route output"
    );
}
