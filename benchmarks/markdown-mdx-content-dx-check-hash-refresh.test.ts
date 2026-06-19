const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function markdownMdxVisibility(packageStatus) {
  return packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "content/react-markdown",
  );
}

function readModelBlock(readModel) {
  const start = readModel.indexOf(
    "export const markdownMdxContentPackageVisibility = {",
  );
  assert.notEqual(start, -1, "Markdown & MDX Content read-model export missing");
  const nextExport = readModel.indexOf("\n\nexport const ", start + 1);
  return readModel.slice(start, nextExport === -1 ? readModel.length : nextExport);
}

test("Markdown & MDX Content exposes receipt-hash helper freshness through dx-check", () => {
  const metrics = [
    "markdown_mdx_content_receipt_hash_refresh_current",
    "markdown_mdx_content_receipt_hash_refresh_stale",
    "markdown_mdx_content_receipt_hash_refresh_missing",
  ];
  const projectChecker = read(
    "core/src/ecosystem/project_check/markdown_mdx_content_dx_check.rs",
  );
  const checkPanelReader = read("core/src/ecosystem/dx_check_receipt.rs");
  const helper = read("examples/template/markdown-mdx-content-receipt-hashes.ts");
  const packageStatus = JSON.parse(
    read("examples/template/.dx/forge/package-status.json"),
  );
  const visibility = markdownMdxVisibility(packageStatus);
  assert.ok(visibility, "Markdown & MDX Content package-status row missing");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const readModelPackageBlock = readModelBlock(readModel);
  const docs = read("docs/packages/content-react-markdown.md");
  const frameworkDocs = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");

  assert.match(projectChecker, /receipt_hash_refresh_counts\(visibility\)/);
  assert.match(
    projectChecker,
    /hash_mismatches > 0[\s\S]*receipt_hash_refresh_stale > 0[\s\S]*receipt_hash_refresh_missing > 0/,
  );
  assert.match(checkPanelReader, /const MARKDOWN_MDX_CONTENT_METRICS: \[&str; 15\]/);
  assert.match(
    checkPanelReader,
    /let \(refresh_current, refresh_stale, refresh_missing\) =\s*receipt_hash_refresh_counts\(package\);/,
  );
  assert.match(checkPanelReader, /dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_hash_refresh_row/);
  assert.match(helper, /const DX_CHECK_METRICS = \[/);
  assert.match(
    helper,
    /ensureStringArrayIncludes\(visibility, "dx_check_metrics", DX_CHECK_METRICS\)/,
  );
  assert.match(helper, /replaceReadModelDxCheckMetricsBlock/);

  for (const metric of metrics) {
    assert.match(projectChecker, new RegExp(metric));
    assert.match(checkPanelReader, new RegExp(metric));
    assert.match(docs, new RegExp(metric));
    assert.match(frameworkDocs, new RegExp(metric));
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Markdown & MDX Content visibility metrics`,
    );
    assert.ok(
      packageStatus.dx_check_metrics.includes(metric),
      `${metric} missing from package-status global dx_check_metrics`,
    );
    assert.match(readModelPackageBlock, new RegExp(metric));
    assert.match(readModel, new RegExp(metric));
  }
});
