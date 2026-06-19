const assert = require("assert");
const fs = require("fs");
const os = require("os");
const path = require("path");
const test = require("node:test");

const { budgetForFixture, renderBudgetTriageMarkdown } = require("./measure-vertical-proof.ts");

test("forge-site budget reads dx.config.toml thresholds", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-budget-config-"));
  try {
    const configPath = path.join(dir, "dx.config.toml");
    fs.writeFileSync(
      configPath,
      String.raw`[forge.launch_budget]
profile = "launch-strict"
max_decoded_bytes = 9000
max_brotli_bytes = 1800
max_http_route_median_ms = 3.25
max_chrome_load_event_ms = 40
`
    );

    const budget = budgetForFixture("forge-site", {
      configPath,
      env: {},
    });

    assert.equal(budget.profile, "launch-strict");
    assert.deepEqual(budget.thresholds, {
      decoded_bytes: 9000,
      brotli_bytes: 1800,
      http_route_median_ms: 3.25,
      chrome_load_event_ms: 40,
    });
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("forge-site budget keeps environment overrides above dx.config.toml", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-budget-env-"));
  try {
    const configPath = path.join(dir, "dx.config.toml");
    fs.writeFileSync(
      configPath,
      String.raw`[forge.launch_budget]
max_decoded_bytes = 9000
max_brotli_bytes = 1800
max_http_route_median_ms = 3.25
max_chrome_load_event_ms = 40
`
    );

    const budget = budgetForFixture("forge-site", {
      configPath,
      env: {
        DX_FORGE_SITE_MAX_DECODED_BYTES: "7000",
        DX_FORGE_SITE_MAX_BROTLI_BYTES: "1500",
        DX_FORGE_SITE_MAX_HTTP_MEDIAN_MS: "2.5",
        DX_FORGE_SITE_MAX_CHROME_LOAD_MS: "33",
      },
    });

    assert.deepEqual(budget.thresholds, {
      decoded_bytes: 7000,
      brotli_bytes: 1500,
      http_route_median_ms: 2.5,
      chrome_load_event_ms: 33,
    });
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("public Forge route budgets cover scorecard, CI, changelog, and adoption routes", () => {
  const scorecardBudget = budgetForFixture("forge-scorecard", {
    configPath: path.join(os.tmpdir(), "missing-dx-config.toml"),
    env: {},
  });
  const ciBudget = budgetForFixture("forge-ci", {
    configPath: path.join(os.tmpdir(), "missing-dx-config.toml"),
    env: {},
  });
  const changelogBudget = budgetForFixture("forge-changelog", {
    configPath: path.join(os.tmpdir(), "missing-dx-config.toml"),
    env: {},
  });
  const adoptionBudget = budgetForFixture("forge-adoption", {
    configPath: path.join(os.tmpdir(), "missing-dx-config.toml"),
    env: {},
  });

  assert.equal(scorecardBudget.profile, "compact-forge-scorecard");
  assert.equal(ciBudget.profile, "compact-forge-ci");
  assert.equal(changelogBudget.profile, "compact-forge-changelog");
  assert.equal(adoptionBudget.profile, "compact-forge-adoption");
  assert.ok(scorecardBudget.thresholds.decoded_bytes > 0);
  assert.ok(scorecardBudget.thresholds.brotli_bytes > 0);
  assert.ok(ciBudget.thresholds.decoded_bytes > 0);
  assert.ok(ciBudget.thresholds.brotli_bytes > 0);
  assert.ok(changelogBudget.thresholds.decoded_bytes > 0);
  assert.ok(changelogBudget.thresholds.brotli_bytes > 0);
  assert.ok(adoptionBudget.thresholds.decoded_bytes > 0);
  assert.ok(adoptionBudget.thresholds.brotli_bytes > 0);
});

test("public Forge route budgets read route-specific config and env overrides", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-route-budget-config-"));
  try {
    const configPath = path.join(dir, "dx.config.toml");
    fs.writeFileSync(
      configPath,
      String.raw`[forge.launch_budget.scorecard]
profile = "scorecard-strict"
max_decoded_bytes = 6000
max_brotli_bytes = 1400
max_http_route_median_ms = 3
max_chrome_load_event_ms = 35

[forge.launch_budget.ci]
profile = "ci-strict"
max_decoded_bytes = 5000
max_brotli_bytes = 1200
max_http_route_median_ms = 2.75
max_chrome_load_event_ms = 30
`
    );

    const scorecardBudget = budgetForFixture("forge-scorecard", {
      configPath,
      env: {
        DX_FORGE_SCORECARD_MAX_BROTLI_BYTES: "1100",
      },
    });
    const ciBudget = budgetForFixture("forge-ci", {
      configPath,
      env: {
        DX_FORGE_CI_MAX_DECODED_BYTES: "4500",
      },
    });

    assert.equal(scorecardBudget.profile, "scorecard-strict");
    assert.deepEqual(scorecardBudget.thresholds, {
      decoded_bytes: 6000,
      brotli_bytes: 1100,
      http_route_median_ms: 3,
      chrome_load_event_ms: 35,
    });
    assert.equal(ciBudget.profile, "ci-strict");
    assert.deepEqual(ciBudget.thresholds, {
      decoded_bytes: 4500,
      brotli_bytes: 1200,
      http_route_median_ms: 2.75,
      chrome_load_event_ms: 30,
    });
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("budget triage markdown explains failed launch budget checks", () => {
  const markdown = renderBudgetTriageMarkdown({
    generated_at: "2026-05-16T00:00:00.000Z",
    fixture_mode: "forge-site",
    budget: {
      profile: "launch-strict",
      config_path: "dx.config.toml",
      enforced: true,
      passed: false,
      checks: [
        {
          metric: "decoded bytes",
          value: 12000,
          max: 9000,
          unit: " B",
          passed: false,
        },
        {
          metric: "HTTP route median",
          value: 2.4,
          max: 5,
          unit: " ms",
          passed: true,
        },
      ],
    },
  });

  assert.match(markdown, /DX-WWW Launch Budget Triage/);
  assert.match(markdown, /decoded bytes/);
  assert.match(markdown, /fail/);
  assert.match(markdown, /First Actions/);
});
