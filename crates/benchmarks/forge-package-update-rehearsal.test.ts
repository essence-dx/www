const assert = require("assert");
const fs = require("fs");
const os = require("os");
const path = require("path");
const test = require("node:test");

const {
  buildReport,
  renderMarkdown,
} = require("./measure-forge-package-update-rehearsal.ts");

function writeBaseProject() {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-update-rehearsal-"));
  fs.mkdirSync(path.join(dir, "components", "ui"), { recursive: true });
  fs.mkdirSync(path.join(dir, ".dx", "forge", "receipts"), { recursive: true });
  fs.writeFileSync(path.join(dir, "components", "ui", "button.tsx"), "export function Button() {}\n");
  fs.writeFileSync(path.join(dir, ".dx", "forge", "receipts", "initial-shadcn-ui-button.json"), "{}");
  return dir;
}

function fakeRunner() {
  const calls = [];
  return {
    calls,
    run(args) {
      calls.push(args);
      const command = args.join(" ");
      if (command.includes("update ui/button") && command.includes("--write --accept-yellow")) {
        return { status: 0, stdout: "Traffic: `yellow`\nReceipt: reviewed-yellow.json", stderr: "" };
      }
      if (command.includes("update ui/button") && command.includes("--write")) {
        if (command.includes("yellow")) {
          return { status: 1, stdout: "", stderr: "yellow requires review" };
        }
        if (command.includes("red")) {
          return { status: 1, stdout: "", stderr: "red update blocked" };
        }
        return { status: 0, stdout: "Traffic: `green`\nReceipt: green-update.json", stderr: "" };
      }
      if (command.includes("forge rollback")) {
        const project = args[args.indexOf("--project") + 1];
        fs.writeFileSync(path.join(project, "components", "ui", "button.tsx"), "export function Button() {}\n");
        return { status: 0, stdout: "RollbackDryRun", stderr: "" };
      }
      if (command.includes("check")) {
        return { status: 0, stdout: "{\"score\":100}", stderr: "" };
      }
      return { status: 0, stdout: "", stderr: "" };
    },
  };
}

test("package update rehearsal proves green, yellow, red, and rollback lanes", async () => {
  const runner = fakeRunner();
  const report = await buildReport({
    generatedAt: "2026-05-17T00:00:00.000Z",
    projectDir: writeBaseProject(),
    prepare: false,
    measure: false,
    runDxCommand: runner.run,
  });
  const markdown = renderMarkdown(report);

  assert.equal(report.score, 100);
  assert.equal(report.passed, true);
  assert.equal(report.no_node_modules, true);
  assert.equal(report.scenarios.green_update.passed, true);
  assert.equal(report.scenarios.yellow_default_block.passed, true);
  assert.equal(report.scenarios.yellow_review_accept.passed, true);
  assert.equal(report.scenarios.red_quarantine.passed, true);
  assert.equal(report.scenarios.rollback_coverage.passed, true);
  assert.ok(runner.calls.some((args) => args.includes("--accept-yellow")));
  assert.match(markdown, /green update/i);
  assert.match(markdown, /red quarantine/i);
});
