//! Source-owned Markdown & MDX Content launch package templates.

pub(super) const REACT_MARKDOWN_VERSION: &str = "10.1.0-dx.2";

pub(super) fn react_markdown_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/components/content/markdown.tsx", MARKDOWN_TSX),
        (
            "js/components/content/markdown-components.tsx",
            MARKDOWN_COMPONENTS_TSX,
        ),
        (
            "js/components/content/markdown-metadata.ts",
            MARKDOWN_METADATA_TS,
        ),
        ("js/components/content/mdx-provider.tsx", MDX_PROVIDER_TSX),
        ("js/components/content/README.md", MARKDOWN_README_MD),
        ("js/server/content/mdx.ts", MDX_SERVER_TS),
        ("js/components/markdown.tsx", MARKDOWN_COMPAT_TSX),
        ("js/components/markdown-client.tsx", MARKDOWN_CLIENT_TSX),
        ("js/lib/react-markdown/metadata.ts", MARKDOWN_METADATA_TS),
        ("js/lib/react-markdown/README.md", MARKDOWN_README_MD),
        ("js/lib/mdx/metadata.ts", MARKDOWN_METADATA_TS),
        ("js/lib/mdx/README.md", MARKDOWN_README_MD),
        (
            "js/lib/markdown-mdx-content/receipt.ts",
            MARKDOWN_MDX_RECEIPT_TS,
        ),
    ]
}

const MARKDOWN_TSX: &str = r##"import ReactMarkdown, {
  MarkdownAsync,
  MarkdownHooks,
  defaultUrlTransform,
  type Components,
  type Options,
  type UrlTransform,
} from "react-markdown";
import type * as React from "react";

import { dxMarkdownComponents } from "./markdown-components";

export const dxMarkdownAllowedElements = [
  "a",
  "blockquote",
  "br",
  "code",
  "em",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
  "hr",
  "li",
  "ol",
  "p",
  "pre",
  "strong",
  "ul",
];

export const dxMarkdownSafetyPolicy = {
  rawHtml: "skipHtml = true",
  urlTransform: "dxSafeMarkdownUrl blocks unsafe-url protocols",
  allowedElements: dxMarkdownAllowedElements,
};

export type DxMarkdownSafetyFinding = {
  readonly kind: "raw-html" | "unsafe-url" | "mdx-jsx";
  readonly status: "blocked";
  readonly snippet: string;
  readonly reason: string;
};

const unsafeMarkdownUrlPattern =
  /\]\(\s*(?:javascript|data|vbscript):|href\s*=\s*["']\s*(?:javascript|data|vbscript):/i;

export function auditDxMarkdownSource(source: string) {
  const findings: DxMarkdownSafetyFinding[] = [];

  if (/<\/?[a-z][^>]*>/i.test(source)) {
    findings.push({
      kind: "raw-html",
      status: "blocked",
      snippet: "Raw HTML",
      reason: "Raw HTML stays disabled by the default skipHtml = true policy.",
    });
  }

  if (unsafeMarkdownUrlPattern.test(source)) {
    findings.push({
      kind: "unsafe-url",
      status: "blocked",
      snippet: "unsafe URL",
      reason: "Unsafe URL protocols are rejected before Markdown links render.",
    });
  }

  if (/<[A-Z][\w.:-]*(\s|>|\/>)/.test(source)) {
    findings.push({
      kind: "mdx-jsx",
      status: "blocked",
      snippet: "MDX JSX",
      reason: "MDX component execution remains an app-owned trust boundary.",
    });
  }

  return {
    policy: dxMarkdownSafetyPolicy,
    findings,
    blockedFindingCount: findings.length,
  };
}

export function resolveDxMarkdownAllowedElements(
  options: Pick<Options, "allowedElements" | "disallowedElements">,
) {
  if (options.allowedElements !== undefined) return options.allowedElements;
  if (options.disallowedElements) return undefined;
  return dxMarkdownAllowedElements;
}

export type DxMarkdownProps = Omit<Options, "components" | "children"> & {
  children: string;
  components?: Components;
};

export function DxMarkdown({
  children,
  components,
  skipHtml = true,
  urlTransform = dxSafeMarkdownUrl,
  ...options
}: DxMarkdownProps) {
  const allowedElements = resolveDxMarkdownAllowedElements(options);

  return (
    <ReactMarkdown
      {...options}
      allowedElements={allowedElements}
      components={{ ...dxMarkdownComponents, ...components }}
      skipHtml={skipHtml}
      urlTransform={urlTransform}
    >
      {children}
    </ReactMarkdown>
  );
}

export async function DxMarkdownAsync({
  children,
  components,
  skipHtml = true,
  urlTransform = dxSafeMarkdownUrl,
  ...options
}: DxMarkdownProps) {
  const allowedElements = resolveDxMarkdownAllowedElements(options);

  return (
    <MarkdownAsync
      {...options}
      allowedElements={allowedElements}
      components={{ ...dxMarkdownComponents, ...components }}
      skipHtml={skipHtml}
      urlTransform={urlTransform}
    >
      {children}
    </MarkdownAsync>
  );
}

export function DxMarkdownHooks(props: DxMarkdownProps & { fallback?: React.ReactNode }) {
  const allowedElements = resolveDxMarkdownAllowedElements(props);

  return (
    <MarkdownHooks
      {...props}
      allowedElements={allowedElements}
      components={{ ...dxMarkdownComponents, ...props.components }}
      skipHtml={props.skipHtml ?? true}
      urlTransform={props.urlTransform ?? dxSafeMarkdownUrl}
    />
  );
}

export const dxSafeMarkdownUrl: UrlTransform = (url, key, node) => {
  const transformed = defaultUrlTransform(url, key, node);
  if (!transformed) return transformed;
  if (transformed.startsWith("#") || transformed.startsWith("/")) return transformed;
  if (/^(?:javascript|data|vbscript):/i.test(transformed)) return "";
  return transformed;
};

export type { Components as DxMarkdownComponents, Options as DxMarkdownOptions };
"##;

const MARKDOWN_COMPONENTS_TSX: &str = r#"import type { Components } from "react-markdown";

export const dxMarkdownComponents = {
  h1: ({ children, ...props }) => (
    <h1 className="text-3xl font-semibold tracking-normal" {...props}>
      {children}
    </h1>
  ),
  h2: ({ children, ...props }) => (
    <h2 className="text-xl font-semibold tracking-normal" {...props}>
      {children}
    </h2>
  ),
  p: ({ children, ...props }) => (
    <p className="leading-7 text-muted-foreground" {...props}>
      {children}
    </p>
  ),
  a: ({ children, ...props }) => (
    <a className="font-medium underline underline-offset-4" {...props}>
      {children}
    </a>
  ),
  ul: ({ children, ...props }) => (
    <ul className="grid list-disc gap-2 pl-5" {...props}>
      {children}
    </ul>
  ),
  code: ({ children, ...props }) => (
    <code className="rounded bg-muted px-1 py-0.5 text-sm" {...props}>
      {children}
    </code>
  ),
} satisfies Components;
"#;

const MARKDOWN_COMPAT_TSX: &str = r#"import Markdown, { defaultUrlTransform } from "react-markdown";
import type { Components, Options, UrlTransform } from "react-markdown";

import { dxMarkdownComponents } from "./content/markdown-components";

export const dxMarkdownAllowedElements = [
  "a",
  "blockquote",
  "br",
  "code",
  "em",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
  "hr",
  "li",
  "ol",
  "p",
  "pre",
  "strong",
  "ul",
];

export function resolveDxMarkdownAllowedElements(
  options: Pick<Options, "allowedElements" | "disallowedElements">,
) {
  if (options.allowedElements !== undefined) return options.allowedElements;
  if (options.disallowedElements) return undefined;
  return dxMarkdownAllowedElements;
}

export type DxMarkdownProps = Omit<Options, "children" | "components"> & {
  children: string;
  components?: Components;
};

export function DxMarkdown({
  allowedElements,
  children,
  components,
  skipHtml = true,
  urlTransform = dxMarkdownUrlTransform,
  ...options
}: DxMarkdownProps) {
  const resolvedAllowedElements = resolveDxMarkdownAllowedElements({
    allowedElements,
    disallowedElements: options.disallowedElements,
  });

  return (
    <Markdown
      {...options}
      allowedElements={resolvedAllowedElements}
      components={{ ...dxMarkdownComponents, ...components }}
      skipHtml={skipHtml}
      urlTransform={urlTransform}
    >
      {children}
    </Markdown>
  );
}

export const dxMarkdownUrlTransform: UrlTransform = (url, key, node) =>
  defaultUrlTransform(url, key, node);
"#;

const MARKDOWN_CLIENT_TSX: &str = r#""use client";

import { MarkdownHooks } from "react-markdown";
import type { HooksOptions } from "react-markdown";
import type * as React from "react";

import {
  dxMarkdownAllowedElements,
  dxMarkdownUrlTransform,
  resolveDxMarkdownAllowedElements,
} from "./markdown";
import { dxMarkdownComponents } from "./content/markdown-components";

export type DxMarkdownClientProps = HooksOptions & {
  fallback?: React.ReactNode;
};

export function DxMarkdownClient({
  allowedElements = dxMarkdownAllowedElements,
  components,
  fallback = null,
  skipHtml = true,
  urlTransform = dxMarkdownUrlTransform,
  ...options
}: DxMarkdownClientProps) {
  const resolvedAllowedElements = resolveDxMarkdownAllowedElements({
    allowedElements,
    disallowedElements: options.disallowedElements,
  });

  return (
    <MarkdownHooks
      {...options}
      allowedElements={resolvedAllowedElements}
      components={{ ...dxMarkdownComponents, ...components }}
      fallback={fallback}
      skipHtml={skipHtml}
      urlTransform={urlTransform}
    />
  );
}
"#;

const MDX_PROVIDER_TSX: &str = r#""use client";

import { MDXProvider, useMDXComponents } from "@mdx-js/react";
import type { MDXComponents } from "mdx/types";
import type * as React from "react";

import { dxMarkdownComponents } from "./markdown-components";

export type DxMdxComponents = MDXComponents;

export type DxMdxComponentInput =
  | MDXComponents
  | ((currentComponents: Readonly<MDXComponents>) => MDXComponents);

export type DxMdxProviderProps = {
  children: React.ReactNode;
  components?: DxMdxComponentInput;
  disableParentContext?: boolean;
  surfaceId?: string;
};

export const dxMdxDefaultComponents = dxMarkdownComponents as unknown as MDXComponents;

export function resolveDxMdxComponents(
  components?: DxMdxComponentInput,
  currentComponents: Readonly<MDXComponents> = {},
) {
  const localComponents =
    typeof components === "function" ? components(currentComponents) : components;

  return {
    ...dxMdxDefaultComponents,
    ...currentComponents,
    ...localComponents,
  };
}

export function useDxMdxComponents(components?: DxMdxComponentInput) {
  return useMDXComponents((currentComponents) =>
    resolveDxMdxComponents(components, currentComponents),
  );
}

export function DxMdxProvider({
  children,
  components,
  disableParentContext = false,
  surfaceId = "content-mdx-provider",
}: DxMdxProviderProps) {
  const mergedWithParent = useDxMdxComponents(components);
  const localOnly = resolveDxMdxComponents(components);
  const mergedComponents = disableParentContext ? localOnly : mergedWithParent;

  return (
    <MDXProvider components={mergedComponents} disableParentContext={disableParentContext}>
      <div
        data-dx-package="content/react-markdown"
        data-dx-package-name="Markdown & MDX Content"
        data-dx-component="dx-mdx-provider"
        data-dx-style-surface="markdown-mdx-content"
        data-dx-zed-surface={surfaceId}
      >
        {children}
      </div>
    </MDXProvider>
  );
}
"#;

const MDX_SERVER_TS: &str = r#"import {
  compile,
  compileSync,
  createProcessor,
  nodeTypes,
  type CompileOptions,
  type ProcessorOptions,
} from "@mdx-js/mdx";

export type DxMdxSource = Parameters<typeof compile>[0];
export type DxMdxCompileOptions = CompileOptions;
export type DxMdxProcessorOptions = ProcessorOptions;

export const dxMdxNodeTypes = nodeTypes;

export const dxMdxCompileDefaults = {
  providerImportSource: "@mdx-js/react",
} satisfies CompileOptions;

export function withDxMdxCompileOptions(options: CompileOptions = {}): CompileOptions {
  return {
    ...dxMdxCompileDefaults,
    ...options,
    providerImportSource:
      options.providerImportSource ?? dxMdxCompileDefaults.providerImportSource,
  };
}

export function withDxMdxProcessorOptions(options: ProcessorOptions = {}): ProcessorOptions {
  return {
    ...options,
    providerImportSource:
      options.providerImportSource ?? dxMdxCompileDefaults.providerImportSource,
  };
}

export async function compileDxMdxContent(
  source: DxMdxSource,
  options: CompileOptions = {},
) {
  return compile(source, withDxMdxCompileOptions(options));
}

export function compileDxMdxContentSync(source: DxMdxSource, options: CompileOptions = {}) {
  return compileSync(source, withDxMdxCompileOptions(options));
}

export function createDxMdxProcessor(options: ProcessorOptions = {}) {
  return createProcessor(withDxMdxProcessorOptions(options));
}

export const dxMdxRuntimeBoundary = {
  evaluate: "app-owned",
  run: "app-owned",
  reason:
    "Forge materializes compile/provider source only; executing arbitrary MDX remains an application trust boundary.",
} as const;
"#;

const MARKDOWN_METADATA_TS: &str = r#"export const dxMarkdownMdxContentPackage = {
  officialDxPackageName: "Markdown & MDX Content",
  packageId: "content/react-markdown",
  aliases: ["markdown-mdx-content", "markdown/mdx", "mdx/content", "markdown/react"],
  upstreamPackages: [
    {
      name: "react-markdown",
      version: "10.1.0",
      sourceMirror: "G:/WWW/inspirations/react-markdown",
    },
    {
      name: "@mdx-js/mdx",
      version: "3.1.1",
      sourceMirror: "G:/WWW/inspirations/mdx/packages/mdx",
    },
    {
      name: "@mdx-js/react",
      version: "3.1.1",
      sourceMirror: "G:/WWW/inspirations/mdx/packages/react",
    },
  ],
  upstreamPackage: "react-markdown; @mdx-js/mdx; @mdx-js/react",
  upstreamVersion: "react-markdown@10.1.0; @mdx-js/mdx@3.1.1; @mdx-js/react@3.1.1",
  forgeVersion: "10.1.0-dx.2",
  honestyLabel: "SOURCE-ONLY",
  publicApi: [
    "Markdown default export",
    "MarkdownAsync",
    "MarkdownHooks",
    "defaultUrlTransform",
    "Components",
    "Options",
    "UrlTransform",
    "MDXProvider",
    "useMDXComponents",
    "compile",
    "compileSync",
    "createProcessor",
    "nodeTypes",
    "createMarkdownMdxContentReceipt",
  ],
  materializedFiles: [
    "components/content/markdown.tsx",
    "components/content/markdown-components.tsx",
    "components/content/markdown-metadata.ts",
    "components/content/mdx-provider.tsx",
    "components/content/README.md",
    "server/content/mdx.ts",
    "components/markdown.tsx",
    "components/markdown-client.tsx",
    "lib/react-markdown/metadata.ts",
    "lib/react-markdown/README.md",
    "lib/mdx/metadata.ts",
    "lib/mdx/README.md",
    "lib/markdown-mdx-content/receipt.ts",
  ],
  requiredDependencies: [
    {
      name: "react-markdown",
      version: "^10.1.0",
      reason: "CommonMark rendering through unified, remark-parse, remark-rehype, and React JSX runtime.",
    },
    {
      name: "react",
      version: ">=18",
      reason: "Peer dependency for rendered markdown components and hooks.",
    },
    {
      name: "@mdx-js/mdx",
      version: "^3.1.1",
      reason: "Server-side MDX compilation through compile, compileSync, createProcessor, and nodeTypes.",
    },
    {
      name: "@mdx-js/react",
      version: "^3.1.1",
      reason: "MDX component context through MDXProvider and useMDXComponents.",
    },
  ],
  inspectedSourceFiles: [
    "G:/WWW/inspirations/react-markdown/index.js",
    "G:/WWW/inspirations/react-markdown/lib/index.js",
    "G:/WWW/inspirations/react-markdown/package.json",
    "G:/WWW/inspirations/mdx/packages/mdx/index.js",
    "G:/WWW/inspirations/mdx/packages/mdx/lib/compile.js",
    "G:/WWW/inspirations/mdx/packages/mdx/lib/core.js",
    "G:/WWW/inspirations/mdx/packages/react/index.js",
    "G:/WWW/inspirations/mdx/packages/react/lib/index.js",
    "G:/WWW/inspirations/mdx/packages/react/package.json",
  ],
  surfaces: [
    "safe-markdown-renderer",
    "markdown-component-overrides",
    "mdx-provider",
    "mdx-server-compile",
    "package-metadata",
    "forge-receipt-helper",
  ],
  dxCheckVisibility: [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ],
  dxStyleCompatibility: {
    visibleSurface: "components/content/mdx-provider.tsx",
    marker: 'data-dx-style-surface="markdown-mdx-content"',
    policy: "uses dx-style-compatible utility classes and app-owned typography tokens",
  },
  zedSourceMarkers: [
    'data-dx-component="dx-mdx-provider"',
    'data-dx-zed-surface="content-mdx-provider"',
    'data-dx-package-name="Markdown & MDX Content"',
  ],
  discovery: {
    dxAdd: "dx add markdown-mdx-content --write",
    dxDryRun: "dx add markdown-mdx-content --dry-run --format json",
    zedSurface: "content_mdx_provider",
  },
  boundaries: [
    "Forge materializes editable renderer, provider, compile, and metadata source; it does not install packages or enable raw HTML.",
    "Applications own content moderation, plugin selection, link policy, sanitization review, trusted MDX execution, and final typography.",
  ],
} as const;

export const dxReactMarkdownPackage = dxMarkdownMdxContentPackage;

export type DxMarkdownMdxContentPackage = typeof dxMarkdownMdxContentPackage;
export type DxReactMarkdownPackage = typeof dxReactMarkdownPackage;
"#;

const MARKDOWN_MDX_RECEIPT_TS: &str = r#"export const markdownMdxContentReceiptStatuses = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
] as const;

export type MarkdownMdxContentReceiptStatus =
  (typeof markdownMdxContentReceiptStatuses)[number];

export type MarkdownMdxContentSurfaceId =
  | "safe-markdown-renderer"
  | "markdown-component-overrides"
  | "mdx-provider"
  | "mdx-server-compile"
  | "package-metadata"
  | "forge-receipt-helper";

export type MarkdownMdxContentUpstreamPackage =
  | "react-markdown"
  | "@mdx-js/mdx"
  | "@mdx-js/react"
  | "dx-forge";

export type MarkdownMdxContentReceiptFile = {
  path: string;
  surface: MarkdownMdxContentSurfaceId;
  hashes: {
    sha256?: string;
    blake3?: string;
  };
  provenance: {
    upstreamPackage: MarkdownMdxContentUpstreamPackage;
    sourceMirror: string;
    inspectedSourceFile?: string;
    upstreamApi?: string;
  };
};

export type MarkdownMdxContentReceiptMaterializedSource = {
  schema: "dx.forge.package.materialized_source";
  sourceFile: "lib/markdown-mdx-content/receipt.ts";
  materializedFile: "lib/markdown-mdx-content/receipt.ts";
  surface: "forge-receipt-helper";
  executionGuard: string;
  runtimeProof: false;
  runtimeLimitations: readonly string[];
};

export type MarkdownMdxContentDxStyleCompatibilityStatus =
  | "present"
  | "missing-receipt"
  | "blocked"
  | "unsupported-surface";

export type MarkdownMdxContentDxStyleCompatibility = {
  schema: "dx.forge.package.dx_style_compatibility";
  status: MarkdownMdxContentDxStyleCompatibilityStatus;
  tokenSource: string;
  generatedCss: string;
  visibleSurfaces: readonly string[];
  sourceFiles: readonly string[];
  receiptPath: string;
  runtimeProof: false;
  dataDxMarkers: readonly string[];
  styleBoundaries: readonly string[];
  runtimeLimitations: readonly string[];
};

export type MarkdownMdxContentReceiptInput = {
  selectedSurfaces: readonly MarkdownMdxContentSurfaceId[];
  files: readonly MarkdownMdxContentReceiptFile[];
  status?: MarkdownMdxContentReceiptStatus;
  generatedAt?: string;
  runtimeLimitations?: readonly string[];
  materializedSource?: MarkdownMdxContentReceiptMaterializedSource;
  dxStyleCompatibility?: MarkdownMdxContentDxStyleCompatibility;
};

export const markdownMdxContentUpstreamProvenance = [
  {
    upstreamPackage: "react-markdown",
    upstreamVersion: "10.1.0",
    sourceMirror: "G:/WWW/inspirations/react-markdown",
    inspectedSourceFiles: [
      "G:/WWW/inspirations/react-markdown/index.js",
      "G:/WWW/inspirations/react-markdown/lib/index.js",
      "G:/WWW/inspirations/react-markdown/package.json",
    ],
    publicApis: [
      "Markdown default export",
      "MarkdownAsync",
      "MarkdownHooks",
      "defaultUrlTransform",
      "Components",
      "Options",
      "UrlTransform",
    ],
  },
  {
    upstreamPackage: "@mdx-js/mdx",
    upstreamVersion: "3.1.1",
    sourceMirror: "G:/WWW/inspirations/mdx/packages/mdx",
    inspectedSourceFiles: [
      "G:/WWW/inspirations/mdx/packages/mdx/index.js",
      "G:/WWW/inspirations/mdx/packages/mdx/lib/compile.js",
      "G:/WWW/inspirations/mdx/packages/mdx/lib/core.js",
    ],
    publicApis: ["compile", "compileSync", "createProcessor", "nodeTypes"],
  },
  {
    upstreamPackage: "@mdx-js/react",
    upstreamVersion: "3.1.1",
    sourceMirror: "G:/WWW/inspirations/mdx/packages/react",
    inspectedSourceFiles: [
      "G:/WWW/inspirations/mdx/packages/react/index.js",
      "G:/WWW/inspirations/mdx/packages/react/lib/index.js",
      "G:/WWW/inspirations/mdx/packages/react/package.json",
    ],
    publicApis: ["MDXProvider", "useMDXComponents"],
  },
] as const;

export const markdownMdxContentRequiredEnv = [] as const;

export const markdownMdxContentAppOwnedBoundaries = [
  "Runtime dependency installation for react-markdown, @mdx-js/mdx, @mdx-js/react, React, and selected remark/rehype plugins.",
  "Content moderation, raw HTML policy, sanitization review, link governance, and private-content policy.",
  "Trust decisions for MDX evaluate/run execution and any remote content execution.",
  "Production bundler integration, content cache policy, metadata extraction, and live runtime verification.",
  "Final typography, design tokens, and docs/content information architecture.",
] as const;

export const markdownMdxContentRuntimeLimitations = [
  "SOURCE-ONLY receipt helper; it records materialized files and selected surfaces but does not prove live Markdown or MDX rendering.",
  "Forge does not install runtime dependencies or execute untrusted MDX.",
  "Raw HTML, GFM, math, syntax highlighting, frontmatter, and other plugins remain explicit app-selected surfaces.",
] as const;

export const markdownMdxContentReceiptMaterializedSource = {
  schema: "dx.forge.package.materialized_source",
  sourceFile: "lib/markdown-mdx-content/receipt.ts",
  materializedFile: "lib/markdown-mdx-content/receipt.ts",
  surface: "forge-receipt-helper",
  executionGuard: "dx run --test .\\benchmarks\\markdown-mdx-content-slice.test.ts",
  runtimeProof: false,
  runtimeLimitations: [
    "SOURCE-ONLY: the targeted guard imports and invokes the materialized receipt helper from generated starter source.",
    "It does not prove live Markdown/MDX rendering, runtime dependency installation, browser visual output, or trusted MDX execution.",
  ],
} as const satisfies MarkdownMdxContentReceiptMaterializedSource;

export const markdownMdxContentDxStyleCompatibility = {
  schema: "dx.forge.package.dx_style_compatibility",
  status: "present",
  tokenSource: "styles/theme.css",
  generatedCss: "styles/generated.css",
  visibleSurfaces: ["mdx-provider"],
  sourceFiles: ["components/content/mdx-provider.tsx"],
  receiptPath: ".dx/forge/receipts/packages/content-react-markdown.json",
  runtimeProof: false,
  dataDxMarkers: ['data-dx-style-surface="markdown-mdx-content"'],
  styleBoundaries: [
    "source-owned dx-style marker only",
    "no live Markdown/MDX renderer visual proof",
    "app-owned typography token review",
  ],
  runtimeLimitations: [
    "SOURCE-ONLY: dx-style compatibility is based on visible source markers and token file references, not live Markdown/MDX renderer proof.",
    "ADAPTER-BOUNDARY: final typography tokens, prose theme choices, and generated CSS review stay app-owned.",
  ],
} as const satisfies MarkdownMdxContentDxStyleCompatibility;

export function createMarkdownMdxContentReceipt(input: MarkdownMdxContentReceiptInput) {
  return {
    schema: "dx.forge.markdown_mdx_content_receipt",
    officialDxPackageName: "Markdown & MDX Content",
    package: {
      packageId: "content/react-markdown",
      aliases: ["markdown-mdx-content", "markdown/mdx", "mdx/content", "markdown/react"],
    },
    selectedSurfaces: [...input.selectedSurfaces],
    files: input.files.map((file) => ({
      ...file,
      hashes: { ...file.hashes },
      provenance: { ...file.provenance },
    })),
    upstreamProvenance: markdownMdxContentUpstreamProvenance,
    requiredEnv: markdownMdxContentRequiredEnv,
    appOwnedBoundaries: markdownMdxContentAppOwnedBoundaries,
    materializedSource: input.materializedSource ?? markdownMdxContentReceiptMaterializedSource,
    runtimeLimitations: input.runtimeLimitations ?? markdownMdxContentRuntimeLimitations,
    dxCheckVisibility: {
      status: input.status ?? "present",
      supportedStatuses: markdownMdxContentReceiptStatuses,
    },
    dxStyleCompatibility: input.dxStyleCompatibility ?? markdownMdxContentDxStyleCompatibility,
    honestyLabel: "SOURCE-ONLY",
    generatedAt: input.generatedAt ?? "source-only-not-runtime-generated",
  } as const;
}

export type MarkdownMdxContentReceipt = ReturnType<typeof createMarkdownMdxContentReceipt>;
"#;

const MARKDOWN_README_MD: &str = r#"# Markdown & MDX Content

This Forge slice gives a launch template editable Markdown and MDX content source.

It keeps `skipHtml` enabled by default for Markdown rendering, exposes sync, async, and hook renderers, adds an MDX component provider, and exposes server-side MDX compile helpers. Runtime dependency installation, plugin choice, link policy, moderation, sanitization review, trusted MDX execution, and final typography remain application-owned.
"#;
