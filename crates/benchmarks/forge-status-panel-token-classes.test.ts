import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const launchSurfacePaths = [
  "examples/template/forge-safety-archive-panel.tsx",
  "examples/template/forge-remote-head-health-panel.tsx",
  "examples/template/forge-golden-path-panel.tsx",
  "examples/template/automations-status.tsx",
  "examples/template/automations/automations-shell.tsx",
  "examples/template/template-dashboard-nav.tsx",
  "examples/template/lib/ai/client-chat.tsx",
  "examples/template/zod-validation-status.tsx",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Forge launch surfaces use DX theme tokens instead of hardcoded color families", () => {
  const forbiddenColorClasses =
    /\b(?:bg|text|border)-(?:black|white|neutral|slate|zinc|stone|gray|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-/;

  for (const surfacePath of launchSurfacePaths) {
    const source = read(surfacePath);

    assert.match(source, /border-border/);
    assert.match(source, /bg-(?:card|muted|background)/);
    assert.match(source, /text-(?:card-foreground|foreground|muted-foreground)/);
    assert.doesNotMatch(source, /#[0-9a-fA-F]{3,8}|rgb\(|hsl\(/);
    assert.doesNotMatch(
      source,
      forbiddenColorClasses,
      `${surfacePath} should use DX theme token classes for launch surface tone`,
    );
  }
});

test("launch shell motion metric controls use DX theme tokens", () => {
  const launchShell = read("examples/template/template-shell.tsx");

  assert.doesNotMatch(
    launchShell,
    /indicatorClassName="[^"]*\bbg-(?:emerald|cyan)-/,
    "motion metric indicators should use semantic DX token backgrounds",
  );
  assert.doesNotMatch(
    launchShell,
    /trackClassName="[^"]*\bbg-neutral-/,
    "motion metric meter tracks should use semantic DX token backgrounds",
  );
  assert.doesNotMatch(
    launchShell,
    /Motion(?:FrameTicker|PageVisibilityBadge)[\s\S]{0,240}className="[^"]*\bborder-neutral-/,
    "motion metric badges should use border-border instead of neutral borders",
  );
});

test("launch shell uses DX theme tokens instead of public color utilities", () => {
  const launchShell = read("examples/template/template-shell.tsx");
  const forbiddenTemplateShellColorClasses =
    /\b(?:bg|text|border)-(?:black|white)(?:\b|\/)|\b(?:bg|text|border)-(?:neutral|slate|zinc|stone|gray|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-/;

  assert.doesNotMatch(
    launchShell,
    forbiddenTemplateShellColorClasses,
    "template-shell.tsx should use DX semantic theme tokens for public color styling",
  );
});

test("Motion package docs show DX token classes in launch examples", () => {
  const motionReadme = read("examples/template/motion/README.md");

  assert.doesNotMatch(
    motionReadme,
    /\b(?:bg|text|border)-(?:black|white|neutral|slate|zinc|stone|gray|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-/,
    "Motion package examples should teach semantic DX token classes",
  );
});
