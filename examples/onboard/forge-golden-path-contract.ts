import { launchForgePackageStatus } from "./forge-package-status";
import { forgeSafetyArchiveContract } from "./forge-safety-archive-contract";

export type ForgeGoldenPathStepState =
  | "real"
  | "partial"
  | "blocked"
  | "missing";

export type ForgeGoldenPathStep = {
  readonly id: string;
  readonly label: string;
  readonly state: ForgeGoldenPathStepState;
  readonly command: string;
  readonly evidencePath: string;
  readonly summary: string;
  readonly boundary: string;
};

export type ForgeGoldenPathContract = {
  readonly schema: "dx.forge.golden_path_status";
  readonly component: "forge-golden-path-status";
  readonly statusSurface: "forge-golden-path-status";
  readonly zedSurface: "forge-golden-path-status";
  readonly packageId: "shadcn/ui/button";
  readonly selectedExport: "button";
  readonly state: "real" | "partial";
  readonly realStepCount: number;
  readonly partialStepCount: number;
  readonly blockedStepCount: number;
  readonly totalStepCount: number;
  readonly dxCheckScore: number;
  readonly dxCheckTraffic: "red" | "yellow" | "green" | "score-gated";
  readonly rootManifestPath: string;
  readonly localRegistryManifestPath: string;
  readonly localPublishReceiptPath: string;
  readonly addReceiptPath: string;
  readonly acceptedUpdateReceiptPath: string;
  readonly statusReceiptPath: string;
  readonly safetyArchiveReceiptPath: string;
  readonly steps: readonly ForgeGoldenPathStep[];
  readonly nextAction: string;
  readonly boundary: string;
};

const packageRow = launchForgePackageStatus.packageRows.find(
  (row) => row.name === "shadcn/ui/button",
);

const visibleButtonFiles = [
  "lib/utils.ts",
  "components/ui/slot.tsx",
  "components/ui/button.tsx",
];

const goldenPathSteps = [
  {
    id: "root-dx-manifest",
    label: "Root dx manifest",
    state: "real",
    command: "dx forge publish --registry local --package shadcn/ui/button --write",
    evidencePath: "dx",
    summary:
      "The launch template now declares shadcn/ui/button, front-facing files, exports, defaults, and local/R2 registry boundaries in the root extensionless dx file.",
    boundary:
      "The root manifest records source-owned files and provider config; it is not a live R2 publish.",
  },
  {
    id: "local-publish",
    label: "Local publish",
    state: "real",
    command: "dx forge publish --registry local --package shadcn/ui/button --write --json",
    evidencePath:
      ".dx/forge/registry/local/packages/js/shadcn/ui/button/0.1.0/.dx/build-cache/manifest.json",
    summary:
      "The root dx package was published into the local filesystem registry with a manifest and content-addressed file blobs.",
    boundary:
      "Local filesystem registry is proven; remote registry publish remains dry-run/approval-gated.",
  },
  {
    id: "selective-visible-install",
    label: "Selective visible install",
    state: "real",
    command: "dx forge add shadcn/ui/button#button --registry local --version 0.1.0 --write",
    evidencePath:
      ".dx/forge/receipts/20260522T130626134038800Z-shadcn-ui-button--variant-export-button.json",
    summary: `The selected button export resolves only ${visibleButtonFiles.join(
      ", ",
    )} without a node_modules install.`,
    boundary:
      "The write path was a source-owned no-op because those visible files already matched the local registry package.",
  },
  {
    id: "status-lock",
    label: "Status lock",
    state: "real",
    command: "dx forge status --json && dx check --json",
    evidencePath: ".dx/receipts/forge/status-latest.json",
    summary:
      "Forge status is ready and the package lock is integrity-valid with zero package-lock hash mismatches under the current lightweight dx-check smoke.",
    boundary:
      "Status readiness and package-lock integrity are proven for the local launch wedge; remote status is still provider-boundary evidence.",
  },
  {
    id: "update-dry-run",
    label: "Update dry-run",
    state: "real",
    command: "dx forge update shadcn/ui/button#button --registry local --version 0.1.0 --dry-run --format json",
    evidencePath: "core/src/ecosystem/forge_security.rs",
    summary:
      "The source dry-run path now persists a reviewable UpdateDryRun receipt under .dx/forge/receipts without mutating the source manifest or writing package files.",
    boundary:
      "The governed dx-www binary still needs to be refreshed before the checked-in CLI executable emits this persisted dry-run receipt in www-template smokes.",
  },
  {
    id: "accepted-update",
    label: "Accepted update",
    state: "real",
    command: "dx forge update shadcn/ui/button#button --registry local --version 0.1.0 --write --format json",
    evidencePath:
      ".dx/forge/receipts/20260522T130639403843800Z-shadcn-ui-button--variant-export-button.json",
    summary:
      "The accepted local update wrote a receipt and stayed aligned because every selected file already matched the latest local-registry package.",
    boundary:
      "This proves the accepted no-op update path; it does not prove a non-trivial file rewrite yet.",
  },
  {
    id: "remove-plan",
    label: "Remove write",
    state: "real",
    command: "cargo test -q -p dx-www-compiler forge_local_registry_remove_receipt_rolls_back_from_registry_content --lib",
    evidencePath: "core/src/ecosystem/forge_security.rs",
    summary:
      "The focused local-registry fixture proves remove --write archives and deletes a selected visible package file after local publish/add.",
    boundary:
      "The live launch template still avoids deleting its own UI files; the destructive path is proven in an isolated Rust fixture.",
  },
  {
    id: "archive-restore",
    label: "Archive and restore",
    state: "real",
    command: "cargo test -q -p dx-www-compiler forge_local_registry_remove_receipt_rolls_back_from_registry_content --lib",
    evidencePath: "core/src/ecosystem/forge_security.rs",
    summary:
      "The focused fixture proves rollback --write can restore a removed selected file from the local registry when the source manifest no longer tracks it.",
    boundary:
      "This proves local-registry restore for the launch wedge; remote restore and arbitrary receipt recovery remain separate boundaries.",
  },
  {
    id: "dx-check-score",
    label: "dx-check score",
    state: "real",
    command: "dx check --json",
    evidencePath: "terminal smoke: dx check --json",
    summary:
      "The dx-check source guard is capped at 89/100 until browser or live-provider proof is attached, with forge_package_lock_hash_mismatches=0 and forge_package_lock_integrity_valid=1.",
    boundary:
      "This is source-level package-lock evidence; browser route proof, live provider runs, and governed binary refresh remain separate launch gates.",
  },
  {
    id: "dashboard-row",
    label: "DX-WWW dashboard row",
    state: "real",
    command: "Open / or inspect tools/launch/runtime-template/pages/index.html",
    evidencePath: "examples/template/forge-golden-path-panel.tsx",
    summary:
      "The launch dashboard and static /launch fixture expose this golden-path row before a fresh package-status receipt is generated.",
    boundary:
      "The row is evidence/status UI; it does not replace the command receipts.",
  },
] as const satisfies readonly ForgeGoldenPathStep[];

function countSteps(state: ForgeGoldenPathStepState): number {
  return goldenPathSteps.filter((step) => step.state === state).length;
}

export const forgeGoldenPathContract = {
  schema: "dx.forge.golden_path_status",
  component: "forge-golden-path-status",
  statusSurface: "forge-golden-path-status",
  zedSurface: "forge-golden-path-status",
  packageId: "shadcn/ui/button",
  selectedExport: "button",
  state: "partial",
  realStepCount: countSteps("real"),
  partialStepCount: countSteps("partial"),
  blockedStepCount: countSteps("blocked"),
  totalStepCount: goldenPathSteps.length,
  dxCheckScore: 89,
  dxCheckTraffic: "score-gated",
  rootManifestPath: "dx",
  localRegistryManifestPath:
    ".dx/forge/registry/local/packages/js/shadcn/ui/button/0.1.0/.dx/build-cache/manifest.json",
  localPublishReceiptPath:
    ".dx/forge/registry/local/receipts/20260522T130612Z-shadcn-ui-button.json",
  addReceiptPath:
    ".dx/forge/receipts/20260522T130626134038800Z-shadcn-ui-button--variant-export-button.json",
  acceptedUpdateReceiptPath:
    ".dx/forge/receipts/20260522T130639403843800Z-shadcn-ui-button--variant-export-button.json",
  statusReceiptPath: ".dx/receipts/forge/status-latest.json",
  safetyArchiveReceiptPath:
    packageRow?.safetyArchiveReceiptPath ??
    forgeSafetyArchiveContract.packages[0]?.safetyArchiveReceiptPath ??
    ".dx/forge/receipts/safety/shadcn-ui-button-archive.json",
  steps: goldenPathSteps,
  nextAction:
    "Refresh the governed dx-www binary, then rerun the end-to-end www-template CLI smoke to capture a checked-in UpdateDryRun receipt fixture from the rebuilt command.",
  boundary:
    "This is a launch wedge status contract. It proves root dx local publish/add/update/remove/restore status, dry-run receipt persistence in source tests, score-gated package-lock integrity, and dashboard visibility; browser/provider proof and governed binary refresh remain separate launch gates.",
} as const satisfies ForgeGoldenPathContract;
