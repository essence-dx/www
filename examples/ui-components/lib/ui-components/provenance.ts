export type UiComponentStatus = "implemented" | "planned" | "adapter-boundary";

export type DependencyReplacement = {
  upstreamPackage: string;
  dxReplacement: string;
  status: UiComponentStatus;
};

export const upstreamShadcnSource = {
  schema: "dx.www.ui_components.provenance",
  name: "shadcn-ui",
  repository: "https://github.com/shadcn-ui/ui.git",
  localMirror: "G:\\WWW\\inspirations\\shadcn-ui",
  commit: "cd54e0927f3853a777f700a0bbf34507cf697b9c",
  license: "MIT",
  registryRoot: "apps/v4/registry/new-york-v4/ui",
  registryComponentCount: 56,
  copiedLicenseNotice: "notices/shadcn-ui/LICENSE.md",
} as const;

export const sourceRoutes = [
  "apps/v4/app/(app)/(root)/page.tsx",
  "apps/v4/app/(app)/docs/[[...slug]]/page.tsx",
  "apps/v4/app/(app)/blocks/page.tsx",
  "apps/v4/app/(app)/charts/[chart]/page.tsx",
  "apps/v4/registry/new-york-v4/ui",
] as const;

export const dependencyReplacements: DependencyReplacement[] = [
  {
    upstreamPackage: "class-variance-authority",
    dxReplacement: "Local variant functions plus DX Style generated classes.",
    status: "implemented",
  },
  {
    upstreamPackage: "tailwind-merge and clsx",
    dxReplacement: "Project-local cn/classes helper and DX Style grouping.",
    status: "implemented",
  },
  {
    upstreamPackage: "lucide-react",
    dxReplacement: "DX Icon through <Icon /> and source-owned icon components.",
    status: "implemented",
  },
  {
    upstreamPackage: "radix-ui",
    dxReplacement: "Source-owned primitives plus explicit focus/state-machine adapter boundaries.",
    status: "planned",
  },
  {
    upstreamPackage: "recharts",
    dxReplacement: "DX Charts package, not npm Recharts.",
    status: "planned",
  },
  {
    upstreamPackage: "embla-carousel-react",
    dxReplacement: "DX Carousel package, not npm Embla.",
    status: "planned",
  },
  {
    upstreamPackage: "cmdk and sonner",
    dxReplacement: "DX Command and DX Toast packages with source-owned keyboard and announcement behavior.",
    status: "adapter-boundary",
  },
] as const;

export const runtimeBoundaries = [
  "React hydration previews are not used by this WWW project.",
  "Radix and Base UI focus/state machines are adapter boundaries until DX owns them.",
  "Recharts, Embla, cmdk, sonner, and package-manager registry generation are not imported.",
  "Every implemented primitive is editable source under components/ui and tracked by Forge receipts.",
] as const;
