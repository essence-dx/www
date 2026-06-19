import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const collectorPath = path.join(root, "benchmarks", "dx-www-no-js-browser-receipt.ts");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function writeFixture(filePath: string, scriptExecutionDisabled: boolean): void {
  const fixture = {
    schema: "dx.www.readiness.no_js_browser_dom_snapshot_fixture.v1",
    url: "file:///G:/Dx/www/examples/template/.dx/www/output/app/index.html",
    user_agent: "fixture-no-js-browser",
    script_execution_disabled_before_navigation: scriptExecutionDisabled,
    document: {
      nodeName: "#document",
      children: [
        {
          nodeName: "html",
          children: [
            {
              nodeName: "head",
              children: [
                {
                  nodeName: "title",
                  children: [{ nodeName: "#text", nodeValue: "DX WWW" }],
                },
              ],
            },
            {
              nodeName: "body",
              children: [
                {
                  nodeName: "div",
                  attributes: ["data-dx-output-mode", "tiny-static", "data-dx-js", "none"],
                  children: [
                    {
                      nodeName: "main",
                      attributes: ["aria-label", "Starter"],
                      children: [
                        { nodeName: "h1", children: [{ nodeName: "#text", nodeValue: "DX WWW" }] },
                        {
                          nodeName: "a",
                          attributes: ["href", "/state-runtime"],
                          children: [{ nodeName: "#text", nodeValue: "State runtime" }],
                        },
                        {
                          nodeName: "form",
                          children: [
                            {
                              nodeName: "label",
                              children: [{ nodeName: "#text", nodeValue: "Note" }],
                            },
                          ],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    },
  };
  fs.writeFileSync(filePath, `${JSON.stringify(fixture, null, 2)}\n`);
}

test("no-JS browser receipt collector is TypeScript, source-owned, and disables scripts before navigation", () => {
  const collector = read("benchmarks/dx-www-no-js-browser-receipt.ts");
  const readiness = read("dx-www/src/cli/readiness.rs");

  for (const marker of [
    "NO_JS_BROWSER_SCHEMA",
    "NO_JS_ARTIFACT_RECEIPT",
    "dx.www.readiness.no_js_browser_receipt_contract",
    "dx.www.readiness.no_js_browser_dom_snapshot_fixture.v1",
    "Emulation.setScriptExecutionDisabled",
    "Page.navigate",
    "DOM.getDocument",
    "local-js-disabled-browser-no-js-route-replay",
    "script_execution_disabled_before_navigation",
    "script_execution_disabled_cdp",
    "artifact_html_blake3",
    "release_ready: false",
    "fastest_world_claim: false",
    "Chrome or Edge was not found. Set DX_BROWSER",
  ]) {
    assert.match(collector, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
  assert.doesNotMatch(collector, /from\s+["'](?:playwright|puppeteer)["']/);
  assert.doesNotMatch(collector, /require\(["'](?:playwright|puppeteer)["']\)/);
  assert.match(readiness, /dx-www-no-js-browser-receipt\.test\.ts/);
  assert.match(readiness, /dx-www-no-js-browser-receipt\.ts --html-path/);
});

test("no-JS browser receipt collector writes an importable receipt from a DOM fixture", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-no-js-browser-"));
  const fixturePath = path.join(tempDir, "fixture.json");
  const outPath = path.join(tempDir, "no-js-browser-latest.json");
  writeFixture(fixturePath, true);

  const output = execFileSync(
    process.execPath,
    [collectorPath, "--from-dom-json", fixturePath, "--out", outPath],
    { cwd: root, encoding: "utf8" },
  );
  const report = JSON.parse(output);
  const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));
  const artifactReceipt = JSON.parse(
    read(".dx/receipts/readiness/no-js-artifact-latest.json"),
  );

  assert.equal(report.passed, true);
  assert.equal(receipt.schema, "dx.www.readiness.no_js_browser_receipt_contract");
  assert.equal(receipt.passed, true);
  assert.equal(receipt.status, "current-local-js-disabled-browser-proof");
  assert.equal(receipt.live_browser_executed, true);
  assert.equal(receipt.javascript_disabled_browser, true);
  assert.equal(receipt.page_javascript_enabled, false);
  assert.equal(receipt.script_tag_count, 0);
  assert.equal(receipt.data_dx_output_mode_tiny_static, true);
  assert.equal(receipt.data_dx_js_none, true);
  assert.equal(receipt.semantic_landmark_present, true);
  assert.equal(receipt.visible_text_present, true);
  assert.equal(receipt.link_count, 1);
  assert.equal(receipt.form_count, 1);
  assert.equal(receipt.seo_title_present, true);
  assert.ok(receipt.accessibility_signal_count > 0);
  assert.equal(receipt.html_path, artifactReceipt.html_path);
  assert.equal(receipt.artifact_html_blake3, artifactReceipt.artifact_html_blake3);
  assert.match(receipt.artifact_html_blake3, /^blake3:[a-f0-9]{64}$/);
  assert.equal(receipt.release_ready, false);
  assert.equal(receipt.fastest_world_claim, false);
});

test("no-JS browser receipt collector refuses to pass fixture proof when script disabling is not proven", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-no-js-browser-stale-"));
  const fixturePath = path.join(tempDir, "fixture.json");
  const outPath = path.join(tempDir, "no-js-browser-latest.json");
  writeFixture(fixturePath, false);

  execFileSync(process.execPath, [collectorPath, "--from-dom-json", fixturePath, "--out", outPath], {
    cwd: root,
    encoding: "utf8",
  });
  const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));

  assert.equal(receipt.passed, false);
  assert.equal(receipt.status, "candidate-not-current");
  assert.equal(receipt.javascript_disabled_browser, false);
  assert.equal(receipt.script_execution_disabled_before_navigation, false);
});
