import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const workspaceRoot = join(dirname(fileURLToPath(import.meta.url)), "..");

function readWorkspaceFile(path: string) {
  return readFileSync(join(workspaceRoot, path), "utf8");
}

function extractOverlayScript() {
  const overlaySource = readWorkspaceFile("dx-www/src/dev/error_overlay.rs");
  const match = overlaySource.match(/const OVERLAY_SCRIPT: &str = r#"\r?\n([\s\S]*?)\r?\n"#;/);
  assert.ok(match, "expected Rust overlay script raw string");
  return match[1];
}

class FakeElement {
  attributes = new Map<string, string>();
  children: FakeElement[] = [];
  className = "";
  hidden = false;
  id = "";
  parentElement: FakeElement | null = null;
  style = { display: "" };
  textContent = "";

  #queryNodes = new Map<string, FakeElement>();
  #registry: Map<string, FakeElement>;
  #tagName: string;

  constructor(tagName: string, registry: Map<string, FakeElement>) {
    this.#tagName = tagName;
    this.#registry = registry;
  }

  appendChild(child: FakeElement) {
    child.parentElement = this;
    this.children.push(child);
    if (child.id) {
      this.#registry.set(child.id, child);
    }
    return child;
  }

  replaceChildren(...children: FakeElement[]) {
    this.children = [];
    for (const child of children) {
      this.appendChild(child);
    }
  }

  querySelector(selector: string) {
    const key = selector.startsWith(".") ? selector.slice(1) : selector;
    let node = this.#queryNodes.get(key);
    if (!node) {
      node = new FakeElement(key, this.#registry);
      node.parentElement = this;
      this.#queryNodes.set(key, node);
    }
    return node;
  }

  setAttribute(name: string, value: string) {
    this.attributes.set(name, String(value));
  }

  set innerHTML(_: string) {
    this.#queryNodes.clear();
  }

  get innerHTML() {
    return `<${this.#tagName}>`;
  }
}

function installOverlayRuntime() {
  const registry = new Map<string, FakeElement>();
  const document = {
    body: new FakeElement("body", registry),
    createElement(tagName: string) {
      return new FakeElement(tagName, registry);
    },
    getElementById(id: string) {
      return registry.get(id) ?? null;
    },
  };
  const window = {
    addEventListener() {},
  } as Record<string, any>;
  const script = extractOverlayScript();
  const fetch = async () => {
    throw new Error("unexpected fetch in overlay recovery unit test");
  };

  Function("window", "document", "fetch", script)(window, document, fetch);

  return { document, window };
}

test("DX dev feedback publishes explicit overlay recovery state", () => {
  const feedbackSource = readWorkspaceFile("dx-www/src/dev/dev_feedback.rs");
  const overlaySource = readWorkspaceFile("dx-www/src/dev/error_overlay.rs");

  assert.match(feedbackSource, /let recovery = diagnostic_issue_recovery_state\(/);
  assert.match(feedbackSource, /"recovery": recovery/);
  assert.match(feedbackSource, /"recovery": errors\["recovery"\]\.clone\(\)/);
  assert.match(feedbackSource, /"status": "recovered"/);
  assert.match(feedbackSource, /"overlay_action": "clear-overlay"/);
  assert.match(feedbackSource, /"clears_overlay": true/);
  assert.match(feedbackSource, /"requires_full_reload": false/);
  assert.match(feedbackSource, /"source_owned_contract": true/);
  assert.match(feedbackSource, /"node_modules_required": false/);
  assert.match(feedbackSource, /"next_runtime": false/);
  assert.match(feedbackSource, /"turbopack_hmr": false/);
  assert.match(
    feedbackSource,
    /errors_snapshot_reports_recovery_state_for_cleared_current_diagnostics/,
  );

  assert.match(overlaySource, /function feedbackSnapshotClearsOverlay\(snapshot\)/);
  assert.match(overlaySource, /function feedbackSnapshotHasAuthoritativeEmptyIssueList\(snapshot\)/);
  assert.match(overlaySource, /snapshot\.recovery\.clears_overlay === true/);
  assert.match(overlaySource, /nextAction\.type === 'clear-overlay'/);
  assert.match(overlaySource, /snapshot\.recovery\.clears_overlay === false/);
  assert.match(overlaySource, /return !status \|\| status === 'current'/);
  assert.match(overlaySource, /value\.errors\.recovery \|\| null/);
  assert.match(
    overlaySource,
    /feedbackSnapshotClearsOverlay\(snapshot\) \|\| feedbackSnapshotHasAuthoritativeEmptyIssueList\(snapshot\)/,
  );
  assert.match(
    overlaySource,
    /overlay_script_uses_explicit_recovery_contract_to_clear/,
  );
});

test("DX overlay keeps existing error visible when diagnostics artifact is missing", () => {
  const { document, window } = installOverlayRuntime();

  window.__DX_SHOW_ERROR__({
    title: "Compile failed",
    severity: "error",
    diagnostic_code: "dx.source.parse_error",
    message: "Unexpected token",
    code_frame: "> 1 | export default",
  });

  const overlay = document.getElementById("dx-error-overlay");
  assert.ok(overlay, "expected overlay to be created");
  assert.equal(overlay.style.display, "flex");
  assert.equal(window.__DX_LAST_ERROR_PAYLOAD__.diagnostic_code, "dx.source.parse_error");

  const result = window.__DX_APPLY_FEEDBACK_ERRORS__({
    schema: "dx.dev_feedback.errors",
    issue_count: 1,
    highest_severity: "info",
    diagnostics_artifact: {
      path: ".dx/diagnostics/latest.json",
      present: false,
      status: "missing",
    },
    recovery: {
      status: "informational",
      overlay_action: "keep-overlay-hidden",
      clears_overlay: false,
      diagnostics_artifact_status: "missing",
    },
    issues: [
      {
        severity: "info",
        diagnostic_code: "dx.dev_feedback.diagnostics_missing",
        message: "DX has not written .dx/diagnostics/latest.json yet.",
      },
    ],
  });

  assert.equal(result, null);
  assert.equal(
    overlay.style.display,
    "flex",
    "missing diagnostics is not an authoritative recovery signal",
  );
  assert.equal(window.__DX_LAST_ERROR_PAYLOAD__.diagnostic_code, "dx.source.parse_error");
});

test("DX overlay clears after explicit recovered diagnostics snapshot", () => {
  const { document, window } = installOverlayRuntime();

  window.__DX_SHOW_ERROR__({
    title: "Compile failed",
    severity: "error",
    diagnostic_code: "dx.source.parse_error",
    message: "Unexpected token",
    code_frame: "> 1 | export default",
  });

  const overlay = document.getElementById("dx-error-overlay");
  assert.ok(overlay, "expected overlay to be created");

  const result = window.__DX_APPLY_FEEDBACK_ERRORS__({
    schema: "dx.dev_feedback.errors",
    issue_count: 0,
    highest_severity: null,
    diagnostics_artifact: {
      path: ".dx/diagnostics/latest.json",
      present: true,
      status: "current",
    },
    recovery: {
      status: "recovered",
      overlay_action: "clear-overlay",
      clears_overlay: true,
      diagnostics_artifact_status: "current",
    },
    issues: [],
  });

  assert.equal(result, null);
  assert.equal(overlay.style.display, "none");
  assert.equal(window.__DX_LAST_ERROR_PAYLOAD__, null);
});
