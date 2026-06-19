const assert = require("assert");
const fs = require("fs");
const os = require("os");
const path = require("path");
const test = require("node:test");

const {
  buildReview,
  renderMarkdown,
} = require("./measure-forge-source-owned-package-review.ts");

function writeReviewProject() {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-source-review-"));
  fs.mkdirSync(path.join(dir, "components", "ui"), { recursive: true });
  fs.mkdirSync(path.join(dir, "lib", "icons"), { recursive: true });
  fs.mkdirSync(path.join(dir, "lib", "auth", "better-auth"), { recursive: true });
  fs.mkdirSync(path.join(dir, ".dx", "forge", "docs"), { recursive: true });
  fs.mkdirSync(path.join(dir, ".dx", "forge", "receipts"), { recursive: true });
  fs.writeFileSync(path.join(dir, "components", "ui", "button.tsx"), "export function Button() {}\n");
  fs.writeFileSync(path.join(dir, "lib", "icons", "search.tsx"), "export function SearchIcon() {}\n");
  fs.writeFileSync(path.join(dir, "lib", "auth", "better-auth", "route.ts"), "export const route = '/auth/better-auth';\n");

  const packages = [
    ["shadcn/ui/button", "shadcn-ui-button", "components/ui/button.tsx"],
    ["dx/icon/search", "dx-icon-search", "lib/icons/search.tsx"],
    ["auth/better-auth", "auth-better-auth", "lib/auth/better-auth/route.ts"],
  ];

  for (const [, slug] of packages) {
    fs.writeFileSync(path.join(dir, ".dx", "forge", "docs", `${slug}.md`), `# ${slug}\n`);
    fs.writeFileSync(path.join(dir, ".dx", "forge", "receipts", `20260517T000000000000000Z-${slug}.json`), "{}\n");
  }

  fs.writeFileSync(
    path.join(dir, ".dx", "forge", "source-manifest.json"),
    `${JSON.stringify(
      {
        version: 1,
        packages: packages.map(([packageId, slug, filePath]) => ({
          package_id: packageId,
          upstream_name: `@dx/${slug}`,
          version: "0.1.0",
          generator: "dx-forge/test",
          variant: "default",
          source_kind: "curated-registry",
          integrity_hash: `${slug}-hash`,
          license: "MIT OR Apache-2.0",
          provenance: {
            source: "dx-forge-curated-registry",
            upstream_reference: `dx-forge://packages/${packageId}`,
            verified: false,
            note: "Curated test provenance only.",
          },
          advisory_review: {
            coverage_kind: "curated-fixture",
            provider: "dx-forge-curated-advisory-fixture",
            live_coverage: false,
            finding_count: 0,
            reviewed_at: "2026-05-17T00:00:00Z",
            note: "Curated fixture, not a live advisory feed.",
          },
          license_review: {
            declared_license: "MIT OR Apache-2.0",
            reviewed: false,
            note: "Declared package license only.",
          },
          files: [
            {
              path: filePath,
              logical_path: `js/${filePath}`,
              hash: `${slug}-file-hash`,
              bytes: 42,
            },
          ],
        })),
      },
      null,
      2
    )}\n`
  );

  return dir;
}

function fakeRunner() {
  const calls = [];
  return {
    calls,
    run(args) {
      calls.push(args);
      const command = args.join(" ");
      if (command.includes("forge adoption-report")) {
        const output = args[args.indexOf("--output") + 1];
        fs.mkdirSync(path.dirname(output), { recursive: true });
        fs.writeFileSync(
          output,
          JSON.stringify(
            {
              passed: true,
              score: 100,
              no_node_modules: true,
              package_count: 3,
              receipt_count: 3,
              package_docs_present: 3,
              package_docs_missing: 0,
            },
            null,
            2
          )
        );
        return { status: 0, stdout: JSON.stringify({ passed: true, score: 100 }), stderr: "" };
      }
      if (command.includes("forge verify-package --all")) {
        return {
          status: 0,
          stdout: JSON.stringify({
            passed: true,
            score: 100,
            packages: [
              { package_id: "shadcn/ui/button", rollback: { passed: true }, docs: { passed: true } },
              { package_id: "dx/icon/search", rollback: { passed: true }, docs: { passed: true } },
              { package_id: "auth/better-auth", rollback: { passed: true }, docs: { passed: true } },
            ],
            missing_packages: [],
          }),
          stderr: "",
        };
      }
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
        return { status: 0, stdout: "RollbackWrite", stderr: "" };
      }
      if (command.includes("check")) {
        return { status: 0, stdout: "{\"score\":100}", stderr: "" };
      }
      return { status: 0, stdout: "", stderr: "" };
    },
  };
}

test("source-owned package review joins docs receipts rollback advisory and yellow edit evidence", async () => {
  const runner = fakeRunner();
  const report = await buildReview({
    generatedAt: "2026-05-17T00:00:00.000Z",
    projectDir: writeReviewProject(),
    prepare: false,
    runDxCommand: runner.run,
  });
  const markdown = renderMarkdown(report);

  assert.equal(report.score, 100);
  assert.equal(report.passed, true);
  assert.equal(report.no_node_modules, true);
  assert.equal(report.review_gates.docs.passed, true);
  assert.equal(report.review_gates.receipts.passed, true);
  assert.equal(report.review_gates.rollback.passed, true);
  assert.equal(report.review_gates.advisory_placeholders.passed, true);
  assert.equal(report.review_gates.local_edit_yellow.passed, true);
  assert.equal(report.packages.length, 3);
  assert.ok(report.packages.every((packageReview) => packageReview.advisory.placeholder === true));
  assert.ok(runner.calls.some((args) => args.join(" ").includes("forge verify-package --all")));
  assert.match(markdown, /Source-Owned Package Fixture Review/);
  assert.match(markdown, /curated-fixture/);
  assert.match(markdown, /yellow review accept/i);
});
