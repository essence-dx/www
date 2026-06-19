import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string): string {
  const fullPath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function runtimeSource(): string {
  return [
    "dx-www/src/cli/devtools/assets/runtime/part-01-boot.ts",
    "dx-www/src/cli/devtools/assets/runtime/part-02-protocol.ts",
    "dx-www/src/cli/devtools/assets/runtime/part-03-controls.ts",
    "dx-www/src/cli/devtools/assets/runtime/part-04-render.ts",
    "dx-www/src/cli/devtools/assets/runtime/part-05-events.ts",
  ]
    .map(read)
    .join("\n");
}

test("app router shell preview stays available for tools without duplicating visible content", () => {
  const source = read("dx-www/src/cli/app_router_execution.rs");
  const islandRuntime = read("dx-www/src/cli/app_router_execution/client_island_runtime.rs");

  assert.match(source, /fn app_router_runtime_shell_preview/);
  assert.match(source, /replace_visible_body_with_app_router_shell/);
  assert.match(source, /data-dx-app-router-shell-visible="true"/);
  assert.match(source, /data-dx-no-js-fallback-preview="true"/);
  assert.match(source, /data-dx-tsx-app-router-shell="layout-template-page-composition"/);
  assert.match(islandRuntime, /function isEvidencePreview/);
  assert.match(islandRuntime, /\[data-dx-static-dom-preview-hidden='true'\]/);
});

test("devtools CSS value controls are real custom editable controls", () => {
  const source = runtimeSource();
  const css = read("dx-www/src/cli/devtools/assets/devtools.css");

  assert.match(source, /function editableControlValue/);
  assert.match(source, /function activeStyleDraftEditor/);
  assert.match(source, /editableControlValue\(propertyNode\)\.trim\(\)/);
  assert.match(source, /editableControlValue\(valueNode\)\.trim\(\)/);
  assert.match(source, /contenteditable:\s*"true"/);
  assert.match(source, /onBlur:\s*\(event\)\s*=>\s*commitEditableText\(event,\s*onInput\)/);
  assert.match(source, /onPaste:\s*onPlainTextPaste/);
  assert.match(source, /function nudgeCssValue/);
  assert.match(source, /data-dx-devtools-value-decrement/);
  assert.match(source, /data-dx-devtools-value-increment/);
  assert.match(css, /\.dx-devtools-stepper\s*\{/);
  assert.match(css, /\.dx-devtools-stepper-button\s*\{/);
});

test("devtools color picker has an actual source-owned color surface", () => {
  const source = runtimeSource();
  const css = read("dx-www/src/cli/devtools/assets/devtools.css");

  assert.match(source, /function colorSpectrumControl/);
  assert.match(source, /function startSpectrumDrag/);
  assert.match(source, /function onSpectrumKeydown/);
  assert.match(source, /function setSpectrumPoint/);
  assert.match(source, /data-dx-devtools-spectrum/);
  assert.match(source, /onPointerdown:\s*startSpectrumDrag/);
  assert.match(source, /onKeydown:\s*onSpectrumKeydown/);
  assert.match(source, /pickerApplyButton\("Use picked color",\s*targetProperty,\s*hslPickerValue\)/);
  assert.match(source, /function hslToRgb/);
  assert.match(source, /function syncRgbFromHsl/);
  assert.match(css, /\.dx-devtools-spectrum\s*\{/);
  assert.match(css, /--dx-devtools-spectrum-hue/);
  assert.match(css, /\.dx-devtools-spectrum-thumb\s*\{/);
  assert.match(css, /\.dx-devtools-spectrum-readout\s*\{/);
});

test("devtools inspector rows and style actions are functional controls", () => {
  const source = runtimeSource();

  assert.match(source, /function selectParentChainItem/);
  assert.match(source, /document\.querySelector\(item\.selector\)/);
  assert.match(source, /captureElement\(target\)/);
  assert.match(source, /data-dx-devtools-parent-chain-item/);
  assert.match(source, /onClick:\s*\(\)\s*=>\s*selectParentChainItem\(item\)/);
  assert.match(source, /data-dx-devtools-computed-css-property/);
  assert.match(source, /onClick:\s*\(\)\s*=>\s*setStyleDraft\(property,\s*value\s*\|\|\s*""\)/);
  assert.match(source, /data-dx-devtools-style-preview/);
  assert.match(source, /data-dx-devtools-style-apply/);
  assert.match(source, /data-dx-devtools-style-undo/);
});
