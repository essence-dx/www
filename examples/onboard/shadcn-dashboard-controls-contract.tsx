export type ShadcnDashboardDensity = "comfortable" | "compact";
export type ShadcnDashboardQueue = "session" | "payment" | "data" | "studio";

export type ShadcnDashboardReceipt =
  | { state: "idle" }
  | {
      state: "ready";
      receiptId: string;
      density: ShadcnDashboardDensity;
      queue: ShadcnDashboardQueue;
      filter: string;
    };

export const shadcnDashboardDensityOptions = [
  {
    id: "comfortable",
    label: "Comfortable",
    description: "Readable review mode for Friday's launch run.",
  },
  {
    id: "compact",
    label: "Compact",
    description: "Dense operator mode for repeated dashboard checks.",
  },
] as const satisfies readonly {
  id: ShadcnDashboardDensity;
  label: string;
  description: string;
}[];

export const shadcnDashboardQueueOptions = [
  {
    id: "session",
    label: "Session",
    controlsId: "mission-session-status",
    packageId: "shadcn/ui/item",
    target: "auth/better-auth",
  },
  {
    id: "payment",
    label: "Payment",
    controlsId: "mission-payment-status",
    packageId: "shadcn/ui/field",
    target: "payments/stripe-js",
  },
  {
    id: "data",
    label: "Data",
    controlsId: "mission-database-status",
    packageId: "shadcn/ui/card",
    target: "tanstack/query",
  },
  {
    id: "studio",
    label: "Studio",
    controlsId: "mission-dashboard-status",
    packageId: "shadcn/ui/button",
    target: "dx-www/template-shell",
  },
] as const satisfies readonly {
  id: ShadcnDashboardQueue;
  label: string;
  controlsId: string;
  packageId: string;
  target: string;
}[];

export const shadcnLaunchDashboardMetadata = {
  packageId: "shadcn/ui/button",
  officialName: "UI Components",
  aliases: [
    "ui/button",
    "ui/badge",
    "ui/card",
    "ui/field",
    "ui/input",
    "ui/label",
    "ui/textarea",
    "ui/item",
    "ui/separator",
  ],
  upstreamPackage: "shadcn-ui",
  upstreamPackages: [
    "shadcn-ui@0.0.1",
    "@radix-ui/react-slot@1.2.4",
    "@radix-ui/react-label@2.1.8",
    "@radix-ui/react-separator@1.1.8",
  ],
  upstreamVersion: "0.0.1",
  sourceMirror: "G:/WWW/inspirations/shadcn-ui",
  sourceMirrors: [
    "G:/WWW/inspirations/shadcn-ui",
    "G:/WWW/inspirations/radix-primitives",
  ],
  docsPath: "docs/packages/ui-components.md",
  provenance: {
    upstreamRegistry: "apps/v4/registry/new-york-v4/ui",
    radixPrimitiveSources: [
      "packages/react/slot/src/slot.tsx",
      "packages/react/label/src/label.tsx",
      "packages/react/separator/src/separator.tsx",
    ],
    publicApi: [
      "Button",
      "buttonVariants",
      "Badge",
      "badgeVariants",
      "Card",
      "CardHeader",
      "CardTitle",
      "CardDescription",
      "CardContent",
      "Field",
      "FieldGroup",
      "FieldLabel",
      "FieldDescription",
      "Input",
      "Label",
      "Textarea",
      "Item",
      "ItemGroup",
      "ItemContent",
      "ItemTitle",
      "ItemDescription",
      "ItemActions",
      "Separator",
    ],
  },
  exportedFiles: [
    "components/template-app/shadcn-dashboard-controls-contract.tsx",
    "components/template-app/shadcn-dashboard-controls.tsx",
    "examples/template/shadcn-dashboard-controls-contract.tsx",
    "examples/template/shadcn-dashboard-controls.tsx",
    "components/ui/button.tsx",
    "components/ui/badge.tsx",
    "components/ui/card.tsx",
    "components/ui/field.tsx",
    "components/ui/input.tsx",
    "components/ui/label.tsx",
    "components/ui/textarea.tsx",
    "components/ui/item.tsx",
    "components/ui/separator.tsx",
  ],
  requiredEnv: [],
  appOwnedBoundaries: [
    "dashboard persistence target",
    "final operator copy",
    "final accessibility review",
    "full shadcn registry synchronization",
  ],
  receiptPaths: [
    "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
    ".dx/forge/receipts/*-shadcn-ui-button.json",
    ".dx/forge/receipts/*-shadcn-ui-card.json",
    ".dx/forge/docs/shadcn-ui-button.md",
    ".dx/forge/template-readiness/launch-route.json",
  ],
  selectedSurfaces: [
    "button",
    "badge",
    "card",
    "label",
    "separator",
    "field",
    "item",
    "input",
    "textarea",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: ["present", "stale", "missing receipt", "blocked", "unsupported surface"],
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
  },
  honestyLabel: "SOURCE-ONLY",
} as const;

export function createShadcnLaunchDashboardReceipt(input: {
  density: ShadcnDashboardDensity;
  queue: ShadcnDashboardQueue;
  filter: string;
}): ShadcnDashboardReceipt {
  const normalizedFilter =
    input.filter.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-") ||
    "all-controls";

  return {
    state: "ready",
    density: input.density,
    queue: input.queue,
    filter: input.filter.trim() || "all controls",
    receiptId: `dx-shadcn-launch-${input.density}-${input.queue}-${normalizedFilter}`,
  };
}
