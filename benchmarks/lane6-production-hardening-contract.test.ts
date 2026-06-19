import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

type StatusEntry = {
  status: string;
  filePath: string;
};

function read(relativePath: string): string {
  const fullPath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function expectMarkers(source: string, markers: string[], label: string): void {
  for (const marker of markers) {
    assert.ok(source.includes(marker), `${label} should include ${marker}`);
  }
}

function expectMatches(source: string, patterns: RegExp[], label: string): void {
  for (const pattern of patterns) {
    assert.match(source, pattern, `${label} should match ${pattern}`);
  }
}

function git(args: string[]): string {
  return execFileSync("git", args, {
    cwd: repoRoot,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
}

function statusEntries(): StatusEntry[] {
  const fields = git(["status", "--porcelain=v1", "-z", "--untracked-files=all"]).split("\0");
  const entries: StatusEntry[] = [];

  for (let index = 0; index < fields.length; index += 1) {
    const field = fields[index];
    if (!field) {
      continue;
    }

    const status = field.slice(0, 2);
    const filePath = field.slice(3);
    entries.push({ status, filePath });

    if (/[CR]/.test(status)) {
      index += 1;
    }
  }

  return entries;
}

function isIntroduced(status: string): boolean {
  return status === "??" || /[ACR]/.test(status);
}

function extractFunction(source: string, signaturePattern: RegExp, label: string): string {
  const match = signaturePattern.exec(source);
  assert.ok(match, `expected ${label} to exist`);

  const start = match.index;
  const openBrace = source.indexOf("{", start);
  assert.notEqual(openBrace, -1, `expected ${label} to have a body`);

  let depth = 0;
  for (let index = openBrace; index < source.length; index += 1) {
    const current = source[index];
    if (current === "{") {
      depth += 1;
    } else if (current === "}") {
      depth -= 1;
      if (depth === 0) {
        return source.slice(start, index + 1);
      }
    }
  }

  assert.fail(`expected to find the end of ${label}`);
}

test("strict clippy target is the production-hardening gate", () => {
  const agents = read("AGENTS.md");

  expectMarkers(
    agents,
    [
      "Production hardening, warning-clean, and 100/100 claims are gated by strict clippy:",
      "cargo clippy -p dx-www --no-default-features --features cli --bin dx-www -j6 -- -D warnings",
      "Do not call a production-hardening lane green until this exact target passes",
    ],
    "AGENTS.md",
  );
});

test("React hooks stay bounded compatibility syntax, not public WWW runtime APIs", () => {
  const agents = read("AGENTS.md");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const stateRuntime = read("dx-www/src/cli/app_router_execution/state_runtime.rs");
  const semantics = read("dx-www/src/cli/app_router_semantics.rs");

  expectMarkers(
    agents,
    [
      "WWW is DX-native here, not React-runtime dependent.",
      "`useState` may remain as compatibility sugar only when WWW can lower it exactly",
      "`useEffect`, `useReducer`, `useContext`, transitions, and full React scheduling",
    ],
    "agent state contract",
  );

  expectMarkers(
    readiness,
    [
      "\"public_runtime\": \"DX-native fine-grained state\"",
      "\"dx_native_api\": [\"state()\", \"derived()\", \"effect()\", \"action()\"]",
      "\"compatibility_lowering\": [\"React useState exact DX-state-slot adapter syntax only\"]",
      "\"react_hook_inventory_api\": [\"useState\"]",
      "\"unsupported_react_hooks\": [\"useEffect\", \"useReducer\", \"useContext\", \"useTransition\"]",
      "\"react_api_shim_executed\": false",
      "\"full_react_hook_runtime\": false",
    ],
    "readiness reactivity receipt",
  );

  expectMarkers(
    stateRuntime,
    [
      "react_api_shim_executed: false",
      "full_react_hook_runtime: false",
      "dx.state-runtime.operation.unsupported-react-like-operation",
    ],
    "state runtime",
  );

  expectMarkers(
    semantics,
    [
      "dx.react-hook.useState.exact-dx-state-slot-lowering",
      "dx.react-hook.useState.missing-exact-state-slot",
      "callback bodies and cleanup are not executed with hidden React semantics",
    ],
    "app router semantics",
  );
});

test("DX-native state is the source of truth for runtime state behavior", () => {
  const agents = read("AGENTS.md");
  const stateRuntime = read("dx-www/src/cli/app_router_execution/state_runtime.rs");
  const staticMarkup = read("dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs");

  expectMarkers(
    agents,
    [
      "`state()` for local reactive values.",
      "`derived()` for computed values.",
      "`effect()` for supported side-effect boundaries.",
      "`action()` for event and mutation semantics.",
      "`store()` for framework-owned global stores.",
      "This is the default",
      "framework state path; do not replace it with template-only Zustand-style",
    ],
    "AGENTS.md state source-of-truth section",
  );

  expectMarkers(
    stateRuntime,
    [
      "dx.tsx.dxNativeReactivityCapabilities",
      "__DX_STATE_GRAPH_RUNTIME__",
      "reflectStateSlotToDom",
      "setRuntimeSlot",
      "refreshDerivedSlots",
      "scheduleEffectsForState",
      "DX-native state() slots, app-global store slots, and explicit state graph slots",
    ],
    "DX-native state runtime",
  );

  expectMarkers(
    staticMarkup,
    [
      "data-dx-state-read",
      "data-dx-state-value",
      "data-dx-state-checked",
      "The generated runtime hook updates only elements carrying compiler-owned data-dx-state-* markers.",
    ],
    "state DOM markers",
  );
});

test("Devtools endpoints and HTML injection remain dev-only", () => {
  const agents = read("AGENTS.md");
  const protocol = read("dx-www/src/cli/devtools/protocol.rs");
  const devHttp = read("dx-www/src/cli/dev_http.rs");
  const devWire = read("dx-www/src/cli/dev_wire.rs");
  const cliCoreImpl = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const cmdBuild = extractFunction(
    cliCoreImpl,
    /pub\s+fn\s+cmd_build\s*\(&self\)\s*->\s*DxResult<\(\)>/,
    "DxCli::cmd_build",
  );

  expectMarkers(
    agents,
    [
      "Devtools and hot reload are development-only. They must not appear in",
      "Devtools endpoints live under `/_dx/devtools/*` and must remain dev-only.",
      "Devtools, hot reload clients, dev endpoints, and hidden production file",
    ],
    "AGENTS.md Devtools contract",
  );

  expectMatches(
    protocol,
    [
      /pub\(super\)\s+fn\s+devtools_protocol_response\s*\([\s\S]*devtools_enabled:\s*bool/,
      /if\s+!devtools_enabled\s*\{\s*return Some\(json_response\(404,\s*disabled_payload\(path\),\s*include_body\)\);/,
    ],
    "Devtools protocol",
  );
  expectMatches(
    devHttp,
    [/if\s+devtools\s+&&\s+!response\.body\.contains\("data-dx-devtools-runtime"\)/],
    "dev HTTP injection path",
  );
  expectMatches(
    devWire,
    [/if\s+hot_reload\s+\|\|\s+devtools_enabled[\s\S]*with_dev_html_injections_token\(response,\s*token\.as_deref\(\),\s*devtools_enabled,\s*false\)/],
    "dev wire injection path",
  );

  assert.doesNotMatch(cmdBuild, /devtools_injection_tags|data-dx-devtools-runtime|\/_dx\/devtools/);
});

test("Devtools preview, apply, and undo contracts are implemented as real protocol receipts", () => {
  const protocol = read("dx-www/src/cli/devtools/protocol.rs");
  const styleOps = read("dx-www/src/cli/devtools/style_ops.rs");
  const runtimeProtocol = read("dx-www/src/cli/devtools/assets/runtime/part-02-protocol.ts");
  const readiness = read("dx-www/src/cli/readiness.rs");

  expectMarkers(
    protocol,
    [
      "const STYLE_PREVIEW_ENDPOINT: &str = \"/_dx/devtools/style-preview\";",
      "const STYLE_APPLY_ENDPOINT: &str = \"/_dx/devtools/style-apply\";",
      "const STYLE_UNDO_ENDPOINT: &str = \"/_dx/devtools/style-undo\";",
      "fn style_preview_payload(project_root: &Path, request_path: &str, body: &Value) -> Value",
      "fn style_apply_payload(project_root: &Path, request_path: &str, body: &Value) -> (u16, Value)",
      "fn style_undo_payload(project_root: &Path) -> (u16, Value)",
      "fn visual_edit_receipt_contract(endpoint: &str) -> Value",
      "fn visual_edit_receipt_candidate(",
      "fn visual_edit_undo_receipt_candidate(",
      "fn write_visual_edit_receipt_artifacts(",
      "write_sr_artifact(",
      "json-sr-machine-written",
      "visual_edit_apply_receipt_formats\": [\"json\", \"sr\", \"machine\"]",
      "visual_edit_undo_receipts\": true",
      "write_readiness_visual_edit_replay_receipt",
    ],
    "Devtools protocol contract",
  );

  expectMarkers(
    styleOps,
    [
      "pub(crate) fn preview_style_change(request: &DxStyleChangeRequest) -> DxStyleChangeOutcome",
      "reason: \"preview-does-not-mutate-source\".to_string()",
      "pub(crate) fn apply_style_change(",
      "pub(crate) fn undo_style_change_json(project_root: &Path, receipt: &Value) -> Value",
      "missing-undo-patch",
      "undo-source-range-mismatch",
      "reason\": \"undone-exact-source-range\"",
    ],
    "Devtools style operations",
  );

  expectMarkers(
    runtimeProtocol,
    [
      "proof.previewProtocolOk = result?.ok === true && result?.body?.schema === \"dx.devtools.style_preview\"",
      "proof.applyProtocolOk = result?.ok === true && result?.body?.schema === \"dx.devtools.style_apply\"",
      "proof.undoProtocolOk = result?.ok === true && result?.body?.schema === \"dx.devtools.style_undo\"",
      "schema: \"dx.visual_edit.style_operation\"",
    ],
    "Devtools browser runtime protocol",
  );

  expectMarkers(
    readiness,
    [
      "--write-visual-edit-replay",
      "safe-preview-apply-undo-receipts-current-browser-workbench-missing",
      "interactive browser workbench replay for inspect, cascade, preview, apply, undo, and receipt",
      "Inspect, cascade, preview, safe apply, and safe undo receipt foundations exist; browser workbench replay remains the release readiness gate.",
    ],
    "readiness visual-edit gate",
  );
});

test("this lane does not introduce JS, CJS, or MJS scripts", () => {
  const introducedJsFamilyFiles = statusEntries()
    .filter(({ status }) => isIntroduced(status))
    .map(({ filePath }) => filePath.replaceAll("\\", "/"))
    .filter((filePath) => /\.(?:js|cjs|mjs)$/i.test(filePath));

  assert.deepEqual(
    introducedJsFamilyFiles,
    [],
    `new Lane 6 guard files must stay TypeScript or docs-only: ${introducedJsFamilyFiles.join(", ")}`,
  );
});
