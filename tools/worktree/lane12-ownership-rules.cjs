"use strict";

const REMOVED_SCOPE_DELETION_RULES = Object.freeze([
  {
    owner: "scope-removal-deletion",
    reason:
      "deleted external debugging tool clone surface belongs to Lane 12 scope cleanup",
    patterns: [/^core\/src\/devtools\.rs$/],
  },
  {
    owner: "scope-removal-deletion",
    reason:
      "deleted external framework parity fixture belongs to Lane 12 scope cleanup",
    patterns: [/^dx-www\/src\/cli\/next_parity_fixtures\.rs$/],
  },
  {
    owner: "scope-removal-deletion",
    reason:
      "deleted Turbopack runtime/build execution target belongs to Lane 12 scope cleanup",
    patterns: [
      /^tools\/build-graph\/turbo-tasks-executor\.ts$/,
      /^tools\/build-graph\/turbo-tasks-execution-/,
      /^tools\/build-graph\/turbo-tasks-zed-/,
    ],
  },
]);

const GENERATED_ARTIFACT_RULES = Object.freeze([
  {
    owner: "generated-build-artifact",
    artifactKind: "dx-build-output",
    reason:
      "generated .dx build outputs and receipts are proof artifacts, not source lane changes",
    patterns: [
      /^\.dx\/(?:build|receipts)(?:\/|$)/,
      /^examples\/template\/\.dx\/(?:build|receipts)(?:\/|$)/,
    ],
  },
  {
    owner: "generated-forge-artifact",
    artifactKind: "forge-cache-or-receipt",
    reason:
      "Forge cache, status, and receipt outputs are generated evidence that need explicit artifact-owner review",
    patterns: [
      /^examples\/template\/\.dx\/forge\/(?:cache|receipts)(?:\/|$)/,
      /^examples\/template\/\.dx\/forge\/package-status\.json$/,
    ],
  },
  {
    owner: "generated-scratch-artifact",
    artifactKind: "compiler-scratch",
    reason: "generated compiler scratch artifacts must not be bundled into source commits",
    patterns: [/^librust_out\.rlib$/],
  },
]);

const OWNER_RULES = Object.freeze([
  {
    lane: 12,
    owner: "scope-status-coordination",
    reason: "Lane 12 framework-scope status, audit, and worker coordination surfaces",
    patterns: [
      /^dx-www\/README\.md$/,
      /^benchmarks\/measure-(?:current-status|forge-package-update-rehearsal|real-routes)\.ts$/,
      /^tools\/next-rust-merge\//,
      /^dx-www\/src\/cli\/(?:launch_readiness_bundle|public_framework_tools)\.rs$/,
    ],
  },
  {
    lane: null,
    owner: "shared-launch-coordinator",
    reason: "shared launch coordinator output is dirty outside this Lane 12 slice",
    patterns: [/^tools\/launch-stabilize\/coordinator\.cjs$/],
  },
  {
    lane: null,
    owner: "shared-public-api",
    reason: "shared public API exports need coordinator review before staging",
    patterns: [/^core\/src\/lib\.rs$/, /^dx-www\/src\/lib\.rs$/],
  },
  {
    lane: null,
    owner: "shared-rust-manifest",
    reason: "Cargo manifest changes affect compile proof and need cross-lane review",
    patterns: [/^dx-www\/Cargo\.toml$/],
  },
  {
    lane: null,
    owner: "shared-tsx-compiler-core",
    reason: "shared TSX parser and lowering code feeds App Router, route, and diagnostics lanes",
    patterns: [/^core\/src\/delivery\/tsx_ast\.rs$/],
  },
  {
    lane: null,
    owner: "next-rust-reference-provenance",
    reason: "Next/Turbopack reference provenance must remain non-runtime and not be bundled blindly",
    patterns: [
      /^dx-www\/src\/next_rust\.rs$/,
      /^dx-www\/src\/next_rust_(?:source_map|task)_adapter\.rs$/,
    ],
  },
  {
    lane: 12,
    owner: "scope-cleanup-docs-truth",
    reason: "Lane 12 active scope docs, status truth, and coordination checks",
    patterns: [
      /^(README|DX|TODO|CHANGELOG)\.md$/,
      /^docs\//,
      /^benchmarks\/(?!dx-build-|installed-smoke-|react-starter-|next-rust-(?:merge|vendor|reference)).*(?:scope|scorecard|coordinator|ownership).*\.([cm]?js|ts|tsx|mjs)$/,
      /^benchmarks\/reports\//,
      /^tools\/worktree\//,
      /^tools\/vendor\/next-rust-boundary\//,
      /^tools\/vendor\/next-rust-boundary-check\.js$/,
    ],
  },
  {
    lane: null,
    owner: "dx-style",
    reason: "dx-style/Tailwind is a separate active lane and must not be bundled into Lane 12",
    patterns: [
      /^related-crates\/style\//,
      /^tools\/dx-style(?:\/|$)/,
      /^tools\/style(?:\/|$)/,
      /^benchmarks\/dx-style/,
      /^dx-www\/src\/cli\/dx_style_support\.rs$/,
    ],
  },
  {
    lane: 1,
    owner: "dx-build-command-contract",
    reason: "dx build command contract belongs to Lane 1",
    patterns: [
      /^tools\/build\/installed-smoke\//,
      /^tools\/build\/readiness-gate\//,
      /^dx-www\/src\/cli\/preview_(?:command|contract)\.rs$/,
      /^benchmarks\/dx-build-/,
      /^benchmarks\/installed-smoke-/,
      /^dx-www\/tests\/dx_build/,
    ],
  },
  {
    lane: 2,
    owner: "source-build-graph",
    reason: "source graph, manifests, server-data, CSS, and assets belong to Lane 2",
    patterns: [
      /^dx-www\/src\/build\//,
      /^tools\/build-graph\//,
      /^benchmarks\/(?:source-build|dx-build-graph|installed-smoke-manifest)/,
      /^benchmarks\/app-router-server-data-build-contract\.test\.ts$/,
      /^dx-www\/src\/cli\/app_(?:router_server_data|router_style_assets|server_data_manifest)\.rs$/,
      /^dx-www\/tests\/app_router_server_data\.rs$/,
      /^dx-www\/tests\/source_build/,
    ],
  },
  {
    lane: 3,
    owner: "app-router-filesystem-routing",
    reason: "App Router filesystem route discovery belongs to Lane 3",
    patterns: [
      /^dx-www\/src\/router\//,
      /^router\/src\//,
      /^dx-www\/src\/project\.rs$/,
      /^dx-www\/src\/app_router_segments\.rs$/,
      /^dx-www\/src\/cli\/app_(?:page_route_diagnostics|page_routes|router_paths|segment_files)\.rs$/,
      /^benchmarks\/(?:app-router-filesystem|app-router-page|app-router-segment|app-router-discovery|app-router-dynamic|app-router-invalid-param|app-router-invalid-non-path|app-router-invalid-segment|app-router-src-app|project-scan-src-app|app-router-route-root|app-router-route-precedence|app-router-duplicate-param|app-router-shape-collision|app-router-shared-segment|app-router-static-segment|app-router-terminal-catch-all|dx-app-router-catch-all)/,
    ],
  },
  {
    lane: 4,
    owner: "app-router-render-semantics",
    reason: "App Router render semantics belongs to Lane 4",
    patterns: [
      /^dx-www\/src\/cli\/app_router_execution/,
      /^dx-www\/src\/cli\/app_router_build_output\.rs$/,
      /^dx-www\/src\/cli\/app_router_(?:build|runtime)_command\.rs$/,
      /^dx-www\/src\/cli\/app_router_semantics\.rs$/,
      /^dx-www\/src\/cli\/server_action_runtime\.rs$/,
      /^benchmarks\/app-router-(?:build-output|execution)-/,
      /^benchmarks\/app-router-source-owned-vocabulary\.test\.cjs$/,
      /^benchmarks\/tsx-app-router-/,
    ],
  },
  {
    lane: 5,
    owner: "route-handlers",
    reason: "route handlers and request behavior belong to Lane 5",
    patterns: [
      /^core\/src\/delivery\/route_handler/,
      /^core\/src\/delivery\/(?:mod|server_contract|tests)\.rs$/,
      /^dx-www\/src\/api\//,
      /^dx-www\/src\/cli\/app_api_routes\.rs$/,
      /^dx-www\/src\/cli\/app_route_handler/,
      /^dx-www\/src\/build\/source_engine\/route_handler/,
      /^benchmarks\/(?:app-api-route-handler|route-handler|provider-route-handler|ai-route-handler|automation-route-handler|fumadocs-.*route-handler|dx-api-router|dx-router-request-normalization)/,
    ],
  },
  {
    lane: 6,
    owner: "resolver-module-linker",
    reason: "resolver, linker, and no-node_modules diagnostics belong to Lane 6",
    patterns: [
      /^dx-www\/src\/build\/source_engine\/module_(?:linker|resolver)/,
      /^benchmarks\/source-resolver-/,
      /^dx-www\/tests\/source_resolver/,
    ],
  },
  {
    lane: 7,
    owner: "hot-reload-dev-server",
    reason: "hot reload and dev server loop belong to Lane 7",
    patterns: [
      /^dx-www\/src\/dev\//,
      /^dx-www\/src\/hot_reload_protocol\.rs$/,
      /^dx-www\/src\/cli\/dev_bridge\.rs$/,
      /^dx-www\/src\/cli\/dev_(?:hot_reload_client|http|response|wire)\.rs$/,
      /^benchmarks\/dx-(?:dev|hot-reload)/,
    ],
  },
  {
    lane: 8,
    owner: "diagnostics-overlay",
    reason: "diagnostics and basic DX feedback overlay belong to Lane 8",
    patterns: [
      /^dx-www\/src\/diagnostics/,
      /^dx-www\/src\/error\.rs$/,
      /^dx-www\/src\/cli\/app_route_diagnostics\.rs$/,
      /^benchmarks\/diagnostics-/,
      /^dx-www\/tests\/diagnostics/,
    ],
  },
  {
    lane: 9,
    owner: "cli-architecture-split",
    reason: "CLI structure and mod.rs risk belong to Lane 9",
    patterns: [
      /^dx-www\/src\/cli\/(?:mod|help_text|tests|build_command|build_options|command_output|dev_command|dev_options|generate_command|migrate_command|new_command|promote_command|rollback_command|templates_command|next_rust_status|update_options|update_command|forge_.*options)\.rs$/,
      /^dx-www\/src\/cli\/tests\//,
      /^dx-www\/src\/cli\/forge_(?:doctor|hosted_registry_smoke|launch_page|packages_command|provenance_command|public_status|publisher_key_command|release_candidate|release_candidate_command|release_dashboard|release_dashboard_command|release_proof|release_history|remote_lifecycle|trust_policy_command|trust_regression_command)\.rs$/,
      /^benchmarks\/cli-/,
    ],
  },
  {
    lane: 10,
    owner: "template-product-path",
    reason: "template product path and artifact discipline belong to Lane 10",
    patterns: [
      /^examples\/template\//,
      /^examples\/conversion-proof\//,
      /^core\/src\/ecosystem\/forge_/,
      /^dx-www\/src\/cli\/default_template/,
      /^dx-www\/src\/cli\/(?:deploy_adapter_contract|forge_hosting_manifest|forge_launch_copy_review|forge_public_add|forge_public_evidence|forge_react_starter_benchmark|forge_static_asset_materialization|hosted_preview_contract|next_adapter_fixtures|next_migration|next_migration_plan|next_familiar_fixtures)\.rs$/,
      /^dx-www\/src\/cli\/studio_command\.rs$/,
      /^dx-www\/src\/cli\/studio_json_surface\.rs$/,
      /^dx-www\/src\/cli\/studio_manifest(?:\/.*)?\.rs$/,
      /^dx-www\/src\/cli\/template_options\.rs$/,
      /^benchmarks\/(?:default-www-template|www-template|react-starter)/,
    ],
  },
  {
    lane: 11,
    owner: "behavioral-test-evidence",
    reason: "cross-lane behavioral guard tests belong to Lane 11",
    patterns: [
      /^benchmarks\/public-framework/,
      /^benchmarks\/nextjs-compatibility-map/,
      /^benchmarks\/next-rust-(?:merge|vendor|reference|source-map|task-input)/,
    ],
  },
]);

function normalizeStatusPath(rawPath) {
  const trimmed = String(rawPath || "").trim().replace(/^"|"$/g, "");
  const renameMarker = " -> ";
  const renameIndex = trimmed.lastIndexOf(renameMarker);
  const activePath =
    renameIndex === -1 ? trimmed : trimmed.slice(renameIndex + renameMarker.length);
  return normalizePath(activePath);
}

function originalRenamePath(rawPath) {
  const trimmed = String(rawPath || "").trim().replace(/^"|"$/g, "");
  const renameMarker = " -> ";
  const renameIndex = trimmed.lastIndexOf(renameMarker);
  if (renameIndex === -1) {
    return null;
  }
  return normalizePath(trimmed.slice(0, renameIndex));
}

function normalizePath(value) {
  return String(value || "").replaceAll("\\", "/");
}

function parseStatusLine(line) {
  const text = String(line || "");
  if (text.trim() === "") {
    return null;
  }

  const status = text.slice(0, 2);
  const rawPath = text.length > 3 ? text.slice(3) : text.trim();
  return {
    status,
    path: normalizeStatusPath(rawPath),
    originalPath: originalRenamePath(rawPath),
    deleted: status.includes("D"),
    untracked: status === "??",
  };
}

function classifyStatusLines(input) {
  const lines = Array.isArray(input) ? input : String(input || "").split(/\r?\n/);
  return lines
    .map(parseStatusLine)
    .filter(Boolean)
    .map((entry) => classifyEntry(entry));
}

function stagedNameStatusToShortStatus(input) {
  const lines = Array.isArray(input) ? input : String(input || "").split(/\r?\n/);
  return lines
    .map((line) => stagedNameStatusLineToShortStatus(line))
    .filter(Boolean)
    .join("\n");
}

function stagedNameStatusLineToShortStatus(line) {
  const text = String(line || "");
  if (text.trim() === "") {
    return null;
  }

  const parts = text.split("\t");
  const statusCode = parts[0]?.trim() || "";
  const status = statusCode.slice(0, 1);
  if ((status === "R" || status === "C") && parts.length >= 3) {
    return `${status}  ${normalizePath(parts[1])} -> ${normalizePath(parts[2])}`;
  }

  if (parts.length >= 2) {
    return `${status || "M"}  ${normalizePath(parts[1])}`;
  }

  return null;
}

function classifyEntry(entry) {
  if (entry.path === "-") {
    return {
      ...entry,
      lane: 12,
      owner: "workspace-junk-deletion",
      reason: "root junk file deletion is intentional and must be preserved, not restored",
      sourceBoundary: "workspace-preserved-deletion",
      artifactKind: null,
      commitPolicy: "preserve-deletion",
      lane12Stageable: false,
    };
  }

  const deletionRule = entry.deleted
    ? REMOVED_SCOPE_DELETION_RULES.find((candidate) =>
        candidate.patterns.some((pattern) => pattern.test(entry.path)),
      )
    : null;
  if (deletionRule) {
    return {
      ...entry,
      lane: 12,
      owner: deletionRule.owner,
      reason: deletionRule.reason,
      sourceBoundary: "source-scope-removal",
      artifactKind: null,
      commitPolicy: "lane-12-scope-removal",
      lane12Stageable: true,
    };
  }

  const sourceBoundary = classifySourceBoundary(entry.path);
  if (sourceBoundary.commitPolicy !== null) {
    return {
      ...entry,
      lane: null,
      owner: sourceBoundary.owner,
      reason: sourceBoundary.reason,
      sourceBoundary: sourceBoundary.sourceBoundary,
      artifactKind: sourceBoundary.artifactKind,
      commitPolicy: sourceBoundary.commitPolicy,
      lane12Stageable: false,
    };
  }

  const rule = OWNER_RULES.find((candidate) =>
    candidate.patterns.some((pattern) => pattern.test(entry.path)),
  );
  const lane = rule?.lane ?? null;
  const owner = rule?.owner ?? "unclassified";
  return {
    ...entry,
    lane,
    owner,
    reason: rule?.reason ?? "no Lane 12 owner rule matched this path",
    sourceBoundary: "source-or-coordination",
    artifactKind: null,
    commitPolicy: lane === 12 ? "lane-12-only" : "leave-for-owner",
    lane12Stageable: lane === 12,
  };
}

function classifySourceBoundary(path) {
  if (/(^|\/)node_modules(?:\/|$)/.test(path)) {
    return {
      owner: "forbidden-node-modules",
      reason:
        "node_modules output is forbidden in DX-WWW source commits; keep dependency trees out of release-control staging",
      sourceBoundary: "node-modules-boundary",
      artifactKind: "node-modules",
      commitPolicy: "forbidden-node-modules",
    };
  }

  const artifactRule = GENERATED_ARTIFACT_RULES.find((candidate) =>
    candidate.patterns.some((pattern) => pattern.test(path)),
  );
  if (artifactRule) {
    return {
      owner: artifactRule.owner,
      reason: artifactRule.reason,
      sourceBoundary: "generated-artifact",
      artifactKind: artifactRule.artifactKind,
      commitPolicy: "generated-artifact",
    };
  }

  return {
    owner: null,
    reason: null,
    sourceBoundary: "source-or-coordination",
    artifactKind: null,
    commitPolicy: null,
  };
}

module.exports = {
  classifyStatusLines,
  classifySourceBoundary,
  stagedNameStatusToShortStatus,
};
