//! CLI help text and help-argument detection for the DX-WWW command surface.

use dx_compiler::ecosystem::{
    DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP, DX_FORGE_IMPORT_ECOSYSTEMS_HELP,
};

/// Print help message.
pub(super) fn print_help() {
    eprintln!("dx-www: React-familiar Forge-first web framework");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx <COMMAND> [OPTIONS]");
    eprintln!();
    eprintln!("COMMANDS:");
    eprintln!("    new, create <name>      Create a new project");
    eprintln!("    dev                     Start development server");
    eprintln!("    preview --production-contract");
    eprintln!("                            Serve .dx/www/output through deploy-adapter contract");
    eprintln!("    promote --key <private-key.json>");
    eprintln!("                            Sign and verify build manifest before hosted release");
    eprintln!("    rollback verify         Compare two build dirs for rollback readiness");
    eprintln!("    build                   Build web output or target native artifacts");
    eprintln!("    run <file.ts|file.tsx>  OXC-validate and run TypeScript/TSX through DX");
    eprintln!("    style build             Generate dx-style normal CSS");
    eprintln!("    style watch             Write dx-style watch contract and generated CSS");
    eprintln!("    style check             Verify tokens, generated CSS, and Tailwind leakage");
    eprintln!("    imports sync            Generate readable component/import maps");
    eprintln!("    imports check           Verify generated import maps are current");
    eprintln!("    env open|lock|check     Manage sealed typed env values");
    eprintln!("    explain /route          Explain a route and write .dx/routes contract");
    eprintln!("    doctor                  Run one source-only launch readiness doctor");
    eprintln!("    export --analyze        Analyze static export size, routes, and packages");
    eprintln!("    deploy vercel --dry-run Write static Vercel deploy manifest");
    eprintln!("    serializer <path>       Generate .dx/serializer cache files");
    eprintln!("    generate <type> <name>  Generate new files (alias: g)");
    eprintln!("    add <component>         Add component or source-owned package");
    eprintln!("    update <package>        Preview source-owned package updates");
    eprintln!("    templates --json        List DX-WWW launch templates for CLI/Zed");
    eprintln!("    routes --json           List DX Studio preview routes for Zed");
    eprintln!("    preview-manifest --json Export DX Studio Web Preview manifest for Zed");
    eprintln!("    www dev|build|check|new Next-grade DX-WWW app-router command namespace");
    eprintln!("    www native-shell       Materialize DX Native/Tauri shell source for a WWW app");
    eprintln!("    www docs-doctor --json Scan WWW docs for stale proof claims");
    eprintln!("    www readiness --json  Print WWW release-readiness proof graph");
    eprintln!("    www readiness --write-receipts");
    eprintln!("                         Write safe local readiness receipts");
    eprintln!("    next-rust --json       Show vendored Next/Turbopack Rust snapshot");
    eprintln!("    forge audit <path>      Audit package supply-chain risk");
    eprintln!("    forge add ui/button");
    eprintln!("    forge add ui/badge");
    eprintln!("    forge add ui/card");
    eprintln!("    forge add ui/alert");
    eprintln!("    forge add ui/avatar");
    eprintln!("    forge add ui/skeleton");
    eprintln!("    forge add ui/label");
    eprintln!("    forge add ui/separator");
    eprintln!("    forge add ui/field");
    eprintln!("    forge add ui/item");
    eprintln!("    forge add ui/input");
    eprintln!("    forge add ui/textarea");
    eprintln!("    forge add icon/search");
    eprintln!("    forge add authentication");
    eprintln!("    forge add motion-animation");
    eprintln!("    forge add zustand");
    eprintln!("    forge add trpc");
    eprintln!("    forge add db/drizzle");
    eprintln!("    forge add migration/static-site");
    eprintln!("                            Add source-owned package with receipt");
    eprintln!("    forge init-app          Create the public beta Forge adoption app");
    eprintln!("    forge doctor            Run Forge launch-readiness checks");
    eprintln!("    forge docs              Regenerate Forge package docs");
    eprintln!("    forge evidence          Build Forge release proof report");
    eprintln!("    forge review           Review external source without package installs");
    eprintln!("    forge import           Compatibility alias for forge review");
    eprintln!("    migrate next            Plan a Next App Router migration into DX-WWW");
    eprintln!("    forge launch-copy-review Review final public beta launch copy");
    eprintln!("    forge launch-page       Generate the public /forge launch page");
    eprintln!("    forge launch-readiness-bundle Aggregate launch template readiness metadata");
    eprintln!("    forge launch-adoption-report Report no-build launch template adoption status");
    eprintln!("    forge launch-manifest-drift Report source/generated launch manifest drift");
    eprintln!("    forge launch-companion-receipts Report launch companion proof docs");
    eprintln!("    forge migration-audit   Audit static/WordPress exports for Forge migration");
    eprintln!(
        "    forge migrate-static-page Convert one audited static page to source-owned files"
    );
    eprintln!("    forge static-migration-plan Plan a multi-page static migration batch");
    eprintln!("    forge static-migration-smoke Run the static migration conversion smoke");
    eprintln!("    forge materialize-static-assets Copy reviewed migrated assets to public output");
    eprintln!("    forge migration-guide   Map UI registry commands to Forge ownership");
    eprintln!("    forge beta-diagnostics  Report telemetry-free local beta diagnostics");
    eprintln!("    forge public-evidence   Export public Forge evidence links");
    eprintln!("    forge package-gallery   Report launch package files and review state");
    eprintln!("    forge packages          Report launch package discovery metadata");
    eprintln!("    forge release-history   Record public Forge release proof history");
    eprintln!("    forge release-trend     Compare release-readiness evidence trends");
    eprintln!("    forge launch-changelog  Generate public changelog from release history");
    eprintln!("    forge release-notes     Generate public Forge release notes");
    eprintln!("    forge release-dashboard Verify public release artifacts in one gate");
    eprintln!("    forge release-candidate Verify final public beta RC evidence");
    eprintln!("    forge release-bundle    Assemble verified public Forge publish folder");
    eprintln!("    forge trust-policy      Report source-owned package trust policy");
    eprintln!("    forge trust-regression  Prove package trust signals cannot drift silently");
    eprintln!("    forge smoke             Run the source-owned launch smoke");
    eprintln!("    prove vertical          Compile a real .html/.tsx vertical slice proof");
    eprintln!("    check <path>            Score project, Forge, packages, and security");
    eprintln!("    check packages          Report source-owned package maturity and risk");
    eprintln!("    check web-perf --url <url> --device both --json");
    eprintln!("                            Report Rust-owned web performance receipt");
    eprintln!("    help                    Show this help message");
    eprintln!("    version                 Show version information");
    eprintln!();
    eprintln!("GENERATE TYPES:");
    eprintln!("    page       Generate a new page (.html)");
    eprintln!("    component  Generate a new component (.tsx)");
    eprintln!("    api        Generate a new API route");
    eprintln!("    layout     Generate a new layout (.lyt)");
    eprintln!();
    eprintln!("ADD OPTIONS:");
    eprintln!("    dx add button           Add Button component");
    eprintln!("    dx add ui/button        Add editable Forge UI button source");
    eprintln!("    dx add ui/badge         Add editable Forge UI badge source");
    eprintln!("    dx add ui/card          Add editable Forge UI card source");
    eprintln!("    dx add ui/alert         Add editable Forge UI alert source");
    eprintln!("    dx add ui/avatar        Add editable Forge UI avatar source");
    eprintln!("    dx add ui/skeleton      Add editable Forge UI skeleton source");
    eprintln!("    dx add ui/label         Add editable Forge UI label source");
    eprintln!("    dx add ui/separator     Add editable Forge UI separator source");
    eprintln!("    dx add ui/field         Add editable Forge UI field source");
    eprintln!("    dx add ui/item          Add editable Forge UI item source");
    eprintln!("    dx add ui/input         Add editable Forge UI input source");
    eprintln!("    dx add ui/textarea      Add editable Forge UI textarea source");
    eprintln!("    dx add icon search      Add one editable source-owned icon");
    eprintln!("    dx add authentication      Add editable Authentication files");
    eprintln!(
        "    dx forge add auth/better-auth#google-oauth --write Add the Google OAuth provider surface"
    );
    eprintln!("    dx add motion-animation Add editable Motion & Animation files");
    eprintln!("    dx add trpc             Add editable Type-Safe API files");
    eprintln!("    dx add db/drizzle       Add editable Drizzle SQLite source files");
    eprintln!("    dx add webassembly-bridge");
    eprintln!("                            Add editable WebAssembly Bridge loader files");
    eprintln!("    dx add migration/static-site");
    eprintln!("                            Add a scoped static migration example");
    eprintln!("    dx add ui/button --variant marketing");
    eprintln!("                            Add an isolated editable package fork");
    eprintln!("    dx add ui/button --dry-run [--format terminal|json|markdown]");
    eprintln!("                            Preview editable source without writing");
    eprintln!("    dx update ui/button --dry-run");
    eprintln!("    dx update ui/button --write");
    eprintln!("                            Preview Forge update change set");
    eprintln!("    dx add button card      Add multiple components");
    eprintln!("    dx add --all            Add all components");
    eprintln!("    dx add --list           List available components");
    eprintln!();
    eprintln!("FORGE:");
    eprintln!("    dx forge audit . --format terminal --fail-under 80");
    eprintln!("    dx forge add ui/button --project . --dry-run");
    eprintln!("    dx forge add ui/badge --project . --dry-run");
    eprintln!("    dx forge add ui/card --project . --dry-run");
    eprintln!("    dx forge add ui/alert --project . --dry-run");
    eprintln!("    dx forge add ui/avatar --project . --dry-run");
    eprintln!("    dx forge add ui/skeleton --project . --dry-run");
    eprintln!("    dx forge add ui/label --project . --dry-run");
    eprintln!("    dx forge add ui/separator --project . --dry-run");
    eprintln!("    dx forge add ui/field --project . --dry-run");
    eprintln!("    dx forge add ui/item --project . --dry-run");
    eprintln!("    dx forge add ui/input --project . --dry-run");
    eprintln!("    dx forge add ui/textarea --project . --dry-run");
    eprintln!("    dx forge add icon/search --project . --dry-run");
    eprintln!("    dx forge add auth/better-auth --project . --dry-run");
    eprintln!("    dx forge add motion-animation --project . --dry-run");
    eprintln!("    dx forge add trpc --project . --dry-run");
    eprintln!("    dx forge add db/drizzle --project . --dry-run");
    eprintln!("    dx forge add migration/static-site --project . --dry-run");
    eprintln!("    dx forge add ui/button --project . --write");
    eprintln!("    dx forge add ui/card --project . --write");
    eprintln!("    dx forge update ui/button --project . --dry-run");
    eprintln!("    dx forge update ui/button --project . --write");
    eprintln!(
        "    dx forge review <{}> <package> --plan|--write",
        DX_FORGE_IMPORT_ECOSYSTEMS_HELP
    );
    eprintln!("    dx forge review npm lodash --plan --source-dir .dx/cache/npm/lodash/package");
    eprintln!("    dx forge import npm lodash --plan");
    eprintln!("                            Compatibility alias for dx forge review");
    eprintln!("    dx forge registry init --local .dx-registry");
    eprintln!("    dx forge registry validate --file registry.json --format markdown");
    eprintln!(
        "    dx forge registry build --file registry.json --output .dx/forge/registry.json --embed-content --receipt .dx/forge/registry-build-receipt.json"
    );
    eprintln!("    dx forge registry plan --file registry.json --item ui/button --format markdown");
    eprintln!("    dx forge registry docs --file registry.json --item ui/button --format markdown");
    eprintln!(
        "    dx forge registry apply --item ui/button --dry-run --receipt .dx/forge/ui-button-apply-receipt.json"
    );
    eprintln!("    dx forge registry smoke --remote r2 --local .dx-registry-smoke");
    eprintln!("    dx forge registry publish --remote r2 --package ui/button --dry-run");
    eprintln!(
        "    dx forge registry pull --remote r2 --package ui/button --version <version> --dry-run"
    );
    eprintln!("    dx forge registry status --remote r2");
    eprintln!("    dx forge launch-page --project . --out public --format json");
    eprintln!("    dx forge public-evidence --format markdown --output forge-public-evidence.md");
    eprintln!("    dx forge public-evidence --verify public --format markdown");
    eprintln!("    dx forge release-history --format markdown");
    eprintln!("    dx forge release-trend --format markdown --write-history");
    eprintln!("    dx forge launch-changelog --format markdown --output forge-launch-changelog.md");
    eprintln!("    dx forge release-notes --format markdown --output forge-release-notes.md");
    eprintln!("    dx forge release-dashboard --project . --format markdown");
    eprintln!("    dx forge release-bundle --project . --out .dx/forge-release-bundle");
    eprintln!("    dx forge trust-policy --project . --write-policy --format markdown");
    eprintln!("    dx forge trust-regression --project . --format markdown --fail-under 100");
    eprintln!("    dx forge smoke --project . --format markdown --fail-under 90");
    eprintln!(
        "    dx migrate next --plan --format json --output .dx/forge/next-migration-plan.json"
    );
    eprintln!(
        "    dx prove vertical --page pages/index.html --component components/Button.tsx --write"
    );
    eprintln!(
        "    dx prove vertical --fixture forge-site|forge-ci|forge-releases|forge-adoption --out public --write"
    );
    eprintln!("    dx check . --format terminal --fail-under 80");
    eprintln!("    dx check . --latest-receipt --json");
    eprintln!("    dx check . --strict-forge");
    eprintln!("    dx check . --project-contract");
    eprintln!(
        "    dx check . --project-contract --hints-output .dx/forge/hints/project-contract-hints.json"
    );
    eprintln!("    dx check . --strict-project-contract");
    eprintln!("    dx env lock --password-env DX_ENV_PASSWORD");
    eprintln!("    dx env open --password-env DX_ENV_PASSWORD --ttl-seconds 180");
    eprintln!("    dx env reconcile --password-env DX_ENV_PASSWORD");
    eprintln!("    dx env check --json");
    eprintln!("    dx env agent-context --json");
    eprintln!("    dx serializer dx");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    -h, --help     Show help");
    eprintln!("    -v, --version  Show version");
    eprintln!();
    eprintln!("Run `dx <COMMAND> --help` for more information.");
}

pub(super) fn print_new_help() {
    eprintln!("dx new: Create a source-owned DX-WWW App Router project");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx new <name>");
    eprintln!("    dx create <name>");
    eprintln!();
    eprintln!("WHAT IT CREATES:");
    eprintln!("    app/                 TSX routes, layouts, and boundaries");
    eprintln!("    components/local/    Editable client/server UI components");
    eprintln!("    server/              Server actions, loaders, and endpoints");
    eprintln!("    styles/              DX style tokens and generated CSS-facing files");
    eprintln!("    forge/ and .dx/      Source-owned package receipts and manifests");
    eprintln!();
    eprintln!("DEFAULTS:");
    eprintln!("    no npm install");
    eprintln!("    no node_modules");
    eprintln!("    Forge-first package review");
    eprintln!();
    eprintln!("NEXT:");
    eprintln!("    cd <name>");
    eprintln!("    dx dev");
}

pub(super) fn print_dev_help() {
    eprintln!("dx dev: Run the local DX-WWW development server");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx dev");
    eprintln!("    dx dev --host 127.0.0.1 --port 3000 --no-hot-reload");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    --host <host>        Override dev.host from dx");
    eprintln!("    --port <port>        Override dev.port from dx");
    eprintln!("    --no-hot-reload      Disable hot reload for this run");
    eprintln!("    --devtools           Enable devtools for this run");
    eprintln!("    --no-devtools        Disable devtools for this run");
    eprintln!("    --server-mode <mode> Override dev.server_mode: auto, axum, may-minihttp");
    eprintln!();
    eprintln!("DEFAULTS:");
    eprintln!("    reads dx, with legacy dx.config.toml fallback");
    eprintln!("    serves http://127.0.0.1:3000");
    eprintln!("    automatically tries 3001, 3002, ... when the default port is busy");
    eprintln!("    hot reload is enabled unless --no-hot-reload is passed");
    eprintln!("    does not install packages or create node_modules");
}

pub(super) fn print_build_help(command_name: &str) {
    eprintln!("{command_name}: Run the DX source-owned build engine");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    {command_name}");
    eprintln!("    {command_name} --target android");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    --target <target>    Build target: web or android (default: web)");
    eprintln!();
    eprintln!("OUTPUT:");
    eprintln!("    .dx/build/.dx/build-cache/manifest.json");
    eprintln!("    .dx/build/source-build-manifest.json");
    eprintln!("    .dx/build/.dx/build-cache/source-build-receipt.json");
    eprintln!("    .dx/www/output");
    eprintln!("    .dx/native/android-build-receipt.json for --target android");
    eprintln!("    .dx/receipts/build/latest.json");
    eprintln!("    .dx/receipts/build/zed-handoff.json");
    eprintln!("    .dx/receipts/graph/latest.json");
    eprintln!("    .dx/receipts/graph/consumer-snapshot.json");
    eprintln!();
    eprintln!("CONTRACT:");
    eprintln!("    Uses the source-owned build engine and does not install node_modules.");
    eprintln!("    --target android builds a DX Native/Tauri arm64 debug APK.");
}

pub(super) fn print_check_help() {
    eprintln!("dx check: Score DX-WWW project readiness");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx check [path] [--json|--format terminal|json|markdown]");
    eprintln!("    dx check --latest-receipt [--json]");
    eprintln!("    dx check packages [--json]");
    eprintln!("    dx check web-perf --url <url> --device both --json");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    --strict-forge              Enforce Forge launch gate checks");
    eprintln!("    --project-contract          Include DX-WWW project contract checks");
    eprintln!("    --strict-project-contract   Fail on project contract gaps");
    eprintln!("    --latest-receipt            Print the latest dx-check panel receipt");
    eprintln!("    --fail-under <score>        Fail below a project score threshold");
    eprintln!("    --hints-output <path>       Write IDE/project contract hints");
    eprintln!();
    eprintln!("CONTRACT:");
    eprintln!("    Reads the extensionless dx file and generated .dx serializer cache.");
    eprintln!("    Keeps WWW framework checks separate from DX devtools work.");
}

pub(super) fn print_check_web_perf_help() {
    eprintln!("dx check web-perf: Measure WWW performance evidence");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx check web-perf --url http://127.0.0.1:3000 --device both --json");
    eprintln!("    dx check web-perf --url <url> --device desktop --lighthouse --json");
    eprintln!("    dx check web-perf --from-lighthouse <report.json> --device mobile --json");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    --url <url>                 URL to inspect or measure");
    eprintln!("    --device <mode>             mobile, desktop, or both");
    eprintln!("    --lighthouse                Run Lighthouse through npx and Chrome");
    eprintln!("    --from-lighthouse <json>    Import an existing Lighthouse JSON report");
    eprintln!("    --receipt-mode <mode>       dev or static-build receipt lane");
    eprintln!("    --fail-under-total <score>  Fail below the 0-400 Lighthouse total");
    eprintln!("    --json                      Print JSON output");
    eprintln!();
    eprintln!("CONTRACT:");
    eprintln!("    URL-only mode writes a CDP plan without launching Chrome.");
    eprintln!("    Lighthouse mode is the measured browser proof path.");
}

pub(super) fn print_style_help() {
    eprintln!("dx style: Generate and verify dx-style CSS");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx style build [--json|--format terminal|json|markdown]");
    eprintln!("    dx style watch [--json|--format terminal|json|markdown]");
    eprintln!("    dx style check [--json|--format terminal|json|markdown]");
    eprintln!();
    eprintln!("OUTPUT:");
    eprintln!("    styles/generated.css");
    eprintln!("    .dx/style/*.sr");
    eprintln!("    .dx/receipts/style/*.json");
}

pub(super) fn print_icons_help() {
    eprintln!("dx icons: Generate and verify source-owned icon wrappers");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx icons sync [--json|--format terminal|json|markdown]");
    eprintln!("    dx icons check [--json|--format terminal|json|markdown]");
    eprintln!();
    eprintln!("OUTPUT:");
    eprintln!("    components/icons/*.tsx");
    eprintln!("    .dx/icons/*.sr");
    eprintln!("    .dx/receipts/icons/*.json");
}

pub(super) fn print_imports_help() {
    eprintln!("dx imports: Generate and verify readable auto-import maps and IDE types");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx imports sync [--json|--format terminal|json|markdown]");
    eprintln!("    dx imports check [--json|--format terminal|json|markdown]");
    eprintln!();
    eprintln!("OUTPUT:");
    eprintln!("    components/auto-imports.ts");
    eprintln!("    .dx/imports/import-map.json");
    eprintln!("    .dx/imports/imports.d.ts");
    eprintln!("    .dx/imports/*.sr");
    eprintln!("ALIASES:");
    eprintln!("    #imports");
    eprintln!("    #components");
}

pub(super) fn print_serializer_help() {
    eprintln!("dx serializer: Generate DX Serializer machine cache files");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx serializer dx");
    eprintln!("    dx serializer <file-or-directory>");
    eprintln!();
    eprintln!("OUTPUT:");
    eprintln!("    .dx/serializer/*.machine");
}

pub(super) fn print_www_help() {
    eprintln!("dx www: DX-WWW framework commands");
    eprintln!("    www new <name>            Create a DX-WWW app-router template");
    eprintln!("    www dev                   Run the Rust dev server with hot reload");
    eprintln!("    www build [--target web|android]");
    eprintln!("                               Build WWW output or Android debug APK");
    eprintln!("    www check <path>          Run DX-WWW project checks");
    eprintln!("    www routes --json         List DX Studio preview routes");
    eprintln!("    www preview-manifest --json");
    eprintln!("                               Export DX Studio Web Preview manifest");
    eprintln!("    www templates --json      List DX-WWW templates");
    eprintln!("    www native-shell --target tauri --project . --plan");
    eprintln!("                               Plan a DX Native/Tauri shell for a WWW app");
    eprintln!("    www native-shell --target tauri --project . --write");
    eprintln!("                               Materialize src-tauri and native shell receipt");
    eprintln!("    www agent-context --json  Compact agent handoff context");
    eprintln!("    www docs-doctor --json    Scan WWW docs for stale proof claims");
    eprintln!("    www readiness --json      Print release-readiness proof graph");
    eprintln!("    www readiness --write-receipts");
    eprintln!("                               Write safe local readiness receipts");
    eprintln!("    www next-rust [--json]    Show vendored Next/Turbopack Rust snapshot");
}

pub(super) fn print_www_native_shell_help() {
    eprintln!("dx www native-shell: Materialize a DX Native/Tauri WebView shell");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx www native-shell --target tauri --project . --plan");
    eprintln!("    dx www native-shell --target tauri --project . --write");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    --project <path>          DX-WWW project root (default: current directory)");
    eprintln!("    --target tauri            Native shell target; only tauri is supported now");
    eprintln!("    --plan, --dry-run         Print the source materialization plan");
    eprintln!("    --write                   Write src-tauri files and .dx/native receipt");
    eprintln!("    --force                   Allow --write to replace existing native shell files");
    eprintln!(
        "    --native-root <path>      DX Native root with crates/tauri and crates/tauri-build"
    );
    eprintln!("    --product-name <name>     Native app display name");
    eprintln!("    --identifier <id>         Native bundle identifier");
    eprintln!("    --bridge <name>           Native bridge contract name");
    eprintln!("    --dev-port <port>         DX dev port for the WebView host (default: 3000)");
    eprintln!("    --format terminal|json|markdown");
    eprintln!("    --output <path>           Write the rendered plan/receipt report");
    eprintln!("    --quiet                   Suppress stdout when --output is used");
}

pub(super) fn is_help_arg(arg: Option<&String>) -> bool {
    arg.is_some_and(|arg| matches!(arg.as_str(), "--help" | "-h" | "help"))
}

const FORGE_COMMAND_FAMILIES: &[(&str, &str)] = &[
    (
        "install",
        "add, update, remove, rollback, import, init-app, packages",
    ),
    (
        "registry",
        "registry validate, registry list, registry docs, registry plan, registry apply",
    ),
    (
        "proof",
        "doctor, smoke, audit, verify-package, scorecard, evidence, receipts",
    ),
    (
        "release",
        "release-bundle, release-review, release-operations, publish-plan, publisher-key",
    ),
    (
        "migration",
        "migration-audit, migrate-static-page, static-migration-plan, migration-guide",
    ),
];

pub(super) fn forge_unknown_command_message(command: &str) -> String {
    let mut message = format!("Unknown dx forge command `{command}`.\n\nCommon command groups:");
    for (group, commands) in FORGE_COMMAND_FAMILIES {
        message.push_str(&format!("\n  {group}: {commands}"));
    }
    message.push_str("\n\nRun `dx forge --help` for the full Forge command surface.");
    message
}

pub(super) fn print_forge_help() {
    eprintln!("dx forge: source-owned package firewall, materializer, and registry");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx forge add <package> [--project <path>] [--dry-run|--write]");
    eprintln!(
        "    dx forge acquire <{}> <package> [--version <version>] [--registry-url <url>] [--project <path>] [--format terminal|json|markdown] [--json]",
        DX_FORGE_IMPORT_ECOSYSTEMS_HELP
    );
    eprintln!("        npm live acquisition writes Forge cache/evidence, not node_modules");
    eprintln!("        compatibility alias: dx forge add npm <package>");
    eprintln!(
        "    dx forge import <{}> <package> --plan|--write",
        DX_FORGE_IMPORT_ECOSYSTEMS_HELP
    );
    eprintln!(
        "        aliases: {}",
        DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP
    );
    eprintln!(
        "        [--source-dir <path>] [--file <package-path>] [--from-plan <path>] [--project <path>]"
    );
    eprintln!(
        "        [--output <path>] [--format terminal|json|markdown] [--json] [--fail-under <score>] [--quiet]"
    );
    eprintln!(
        "    dx forge update <package> [--project <path>] [--variant <name>] [--registry local] [--local <path>] [--version <version>] [--dry-run|--write] [--format terminal|json|markdown]"
    );
    eprintln!(
        "    dx forge remove <package> [--project <path>] [--variant <name>] [--dry-run|--write] [--format terminal|json|markdown]"
    );
    eprintln!();
    eprintln!("REGISTRY:");
    eprintln!("    dx forge registry validate [--file <path>] [--format terminal|json|markdown]");
    eprintln!(
        "    dx forge registry build [--file <path>] --output <path> [--embed-content] [--source-root <path>] [--receipt <path>]"
    );
    eprintln!(
        "    dx forge registry list [--file <path>] [--type <registry:type>] [--query <text>]"
    );
    eprintln!("    dx forge registry docs --item <name> [--file <path>]");
    eprintln!("    dx forge registry plan --item <name> [--file <path>] [--project <path>]");
    eprintln!(
        "    dx forge registry apply --item <name> [--file <path>] [--project <path>] [--dry-run|--write] [--receipt <path>]"
    );
    eprintln!("    dx forge registry parity [--format terminal|json|markdown]");
    eprintln!("    dx forge registry publish --remote r2 --package <id> --dry-run");
    eprintln!(
        "    dx forge registry pull --remote r2 --package <id> --version <version> [--dry-run]"
    );
    eprintln!("    dx forge registry status --remote r2");
    eprintln!(
        "    dx forge publish --registry local [--package <id>] [--local <path>] [--dry-run|--write]"
    );
    eprintln!();
    eprintln!("PROJECT:");
    eprintln!("    dx forge status [--json]");
    eprintln!("    dx forge remotes [--json]");
    eprintln!("    dx forge remote add r2 [--json]");
    eprintln!("    dx forge receipts [--project <path>] [--json]");
    eprintln!("    dx forge doctor [--project <path>] [--format terminal|json|markdown]");
    eprintln!("    dx forge audit <path> [--format terminal|json|markdown] [--fail-under <score>]");
    eprintln!(
        "    dx forge docs [--project <path>] [--dry-run|--write] [--format terminal|json|markdown]"
    );
    eprintln!(
        "    dx forge evidence [--project <path>] [--history <index.json>] [--output <path>] [--format terminal|json|markdown] [--quiet]"
    );
    eprintln!();
    eprintln!("IMPORT POLICY:");
    eprintln!("    Import is a Forge review gate, not a package-manager install.");
    eprintln!(
        "    Ecosystem support means modeled review surfaces, not universal or live package-manager compatibility."
    );
    eprintln!(
        "    Ecosystem aliases normalize into canonical Forge receipt paths: {}.",
        DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP
    );
    eprintln!(
        "    Package names are validated per ecosystem before they become package ids, receipt paths, or materialized source paths."
    );
    eprintln!(
        "    --plan writes evidence receipts only; no app source, node_modules, or package code is created."
    );
    eprintln!(
        "    --write materializes inspected source directories into source-owned Forge files after --from-plan validates the accepted import plan; reject mode never overwrites different local source."
    );
    eprintln!("    --from-plan is required for reviewed source materialization.");
    eprintln!(
        "    Accepted source snapshots are written under lib/forge/<ecosystem>/<package>/; clean package-name imports require a compatible reviewed adapter or bridge."
    );
    eprintln!(
        "    --file may be repeated or comma-separated to materialize a reviewed package-relative source slice."
    );
    eprintln!(
        "    Bridge requires adapter/manual wrapper evidence before app code depends on that package boundary."
    );
    eprintln!("    Outcomes are materialize, slice, bridge, or reject.");
    eprintln!(
        "    Forge works without package installs and does not run lifecycle/setup/build scripts."
    );
    eprintln!(
        "    Write receipts include rollback-protected source, manifest, receipt, docs, import-plan, .sr, and machine artifacts."
    );
    eprintln!(
        "    Registry apply receipts mirror JSON, .sr, and machine artifacts for humans, LLMs, and tools."
    );
    eprintln!(
        "    Registry list is discovery; registry plan, parity, and apply receipts are the capability and scoring truth."
    );
    eprintln!();
    eprintln!("ADVANCED:");
    eprintln!(
        "    Use dx forge <command> --help for migration, release, CI, and operator evidence commands."
    );
    eprintln!();
    eprintln!("COMMAND GROUPS:");
    for (group, commands) in FORGE_COMMAND_FAMILIES {
        eprintln!("    {group}: {commands}");
    }
}

pub(super) fn print_forge_ui_help() {
    eprintln!("dx forge ui: source-owned Forge UI capability commands");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!(
        "    dx forge ui parity [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
    );
    eprintln!();
    eprintln!("NOTES:");
    eprintln!(
        "    Registry-wide scripts can use dx forge registry parity with the same report shape."
    );
    eprintln!("    Use Forge component ids such as ui/button, ui/card, and ui/item.");
    eprintln!("    Upstream project names appear only as provenance in receipts and reports.");
}

pub(super) fn print_forge_registry_help() {
    eprintln!("dx forge registry: source-owned component and hosted package registry tools");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!(
        "    dx forge registry validate [--file <path>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
    );
    eprintln!(
        "    dx forge registry build [--file <path>] --output <path> [--embed-content] [--source-root <path>] [--receipt <path>] [--format terminal|json|markdown] [--json] [--quiet]"
    );
    eprintln!(
        "    dx forge registry list [--file <path>] [--type <registry:type>] [--query <text>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
    );
    eprintln!(
        "    dx forge registry docs --item <name> [--file <path>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
    );
    eprintln!(
        "    dx forge registry plan --item <name> [--file <path>] [--project <path>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
    );
    eprintln!(
        "    dx forge registry apply --item <name> [--file <path>] [--project <path>] [--dry-run|--write] [--receipt <path>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
    );
    eprintln!(
        "    dx forge registry parity [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
    );
    eprintln!("    dx forge registry init --local <path>");
    eprintln!(
        "    dx forge registry smoke [--remote r2] [--local <path>] [--package <id>] [--output <path>] [--format terminal|json|markdown] [--fail-under <score>] [--quiet]"
    );
    eprintln!("    dx forge registry publish --remote r2 --package <id> --dry-run");
    eprintln!("    dx forge registry publish --remote r2 --package <id> --write --yes");
    eprintln!(
        "    dx forge registry pull --remote r2 --package <id> --version <version> [--dry-run]"
    );
    eprintln!("    dx forge registry status --remote r2");
    eprintln!();
    eprintln!("NOTES:");
    eprintln!(
        "    Registry build flattens authored registry.json include graphs and can embed reviewed local source file contents."
    );
    eprintln!("    List source-owned Forge registry items before planning or materializing them.");
    eprintln!("    Read reviewed registry item docs without writing files.");
    eprintln!(
        "    Registry plan reports source-owned writes, dependency order, Forge bridges, style merges, and review blockers."
    );
    eprintln!(
        "    Apply writes only reviewed inline registry content and refuses package-manager execution."
    );
    eprintln!("    Registry validate is local and does not execute package managers.");
    eprintln!("    Use Forge package ids such as ui/button instead of vendor-branded ids.");
}

#[cfg(test)]
mod tests {
    use super::forge_unknown_command_message;

    #[test]
    fn forge_unknown_command_message_points_to_professional_command_groups() {
        let message = forge_unknown_command_message("ship-now");

        assert!(message.contains("Unknown dx forge command `ship-now`"));
        assert!(message.contains("registry validate"));
        assert!(message.contains("verify-package"));
        assert!(message.contains("release-bundle"));
        assert!(message.contains("dx forge --help"));
    }
}
