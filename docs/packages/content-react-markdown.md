# Markdown & MDX Content

Official DX package name: `Markdown & MDX Content`

Package id: `content/react-markdown`

Upstream package: `react-markdown`; `@mdx-js/mdx`; `@mdx-js/react`

`content/react-markdown` is the Forge-owned Markdown & MDX Content package slice in DX-WWW. The upstream packages stay provenance metadata; the front-facing lane name is Markdown & MDX Content.

The slice materializes source files that import public upstream APIs. It does not install dependencies, execute untrusted MDX, enable raw HTML, or create a template-local `node_modules` workflow.

## Real Upstream Surface

- `Markdown`, `MarkdownAsync`, `MarkdownHooks`, `defaultUrlTransform`, `Components`, `Options`, and `UrlTransform` from `react-markdown`.
- `MDXProvider` and `useMDXComponents` from `@mdx-js/react`.
- `compile`, `compileSync`, `createProcessor`, and `nodeTypes` from `@mdx-js/mdx`.
- `providerImportSource: "@mdx-js/react"` is the default MDX compile handoff so app-owned MDX components can flow through the provider surface.

## Exported Files

- `components/content/markdown.tsx`
- `components/content/markdown-components.tsx`
- `components/content/markdown-metadata.ts`
- `components/content/mdx-provider.tsx`
- `components/content/README.md`
- `server/content/mdx.ts`
- `components/markdown.tsx`
- `components/markdown-client.tsx`
- `lib/react-markdown/metadata.ts`
- `lib/react-markdown/README.md`
- `lib/mdx/metadata.ts`
- `lib/mdx/README.md`
- `lib/markdown-mdx-content/receipt.ts`

## Forge Metadata

- Official DX package name: `Markdown & MDX Content`
- Package id: `content/react-markdown`
- Aliases: `markdown-mdx-content`, `markdown/mdx`, `mdx/content`, `markdown/react`
- Source mirror: `G:\WWW\inspirations\react-markdown`; `G:\WWW\inspirations\mdx`
- Upstream versions: `react-markdown@10.1.0`; `@mdx-js/mdx@3.1.1`; `@mdx-js/react@3.1.1`
- Surfaces: safe Markdown renderer, component overrides, MDX provider, server MDX compile helper, package metadata, Forge receipt helper
- Required env: none
- Receipt paths: `.dx/forge/docs/content-react-markdown.md`, `.dx/forge/receipts/*-content-react-markdown.json`, `examples/template/.dx/forge/receipts/2026-05-22-content-react-markdown-source-guard.json`, `docs/packages/content-react-markdown.md`
- Honesty label: `SOURCE-ONLY`

## Forge Receipt Shape

`lib/markdown-mdx-content/receipt.ts` exports `createMarkdownMdxContentReceipt` and the schema `dx.forge.markdown_mdx_content_receipt`.

The receipt helper records the official package name, package id, aliases, selected surfaces, materialized files, file hashes (`sha256` and `blake3` slots), upstream provenance, required env, app-owned boundaries, runtime limitations, dx-check visibility status, `materializedSource`, `dxStyleCompatibility`, and `SOURCE-ONLY` honesty label. Its default materialized-source payload uses `dx.forge.package.materialized_source` to show that `lib/markdown-mdx-content/receipt.ts` is the editable generated-starter helper covered by the lane guard. Its default dx-style payload uses `dx.forge.package.dx_style_compatibility` for the visible MDX provider marker, token source, generated CSS boundary, and runtime limitations. It is editable source for generated starters and dx-check/Zed consumers; it does not claim live Markdown or MDX runtime proof.

## dx-check Visibility

The package metadata exposes dx-check visibility for present, stale, missing receipt, blocked, and unsupported surface states. These labels are source-owned and receipt-facing; they do not claim runtime rendering proof by themselves.

## Package Status Read Model

The www-template package-status read model exposes `Markdown & MDX Content` with package id `content/react-markdown`, selected surfaces for the safe Markdown renderer, MDX provider, server compile helper, and Forge receipt helper, and status vocabulary for `present`, `stale`, `missing-receipt`, `blocked`, and `unsupported-surface`.

The `materializedSource` package-status and read-model mirror records `dx.forge.package.materialized_source`, `lib/markdown-mdx-content/receipt.ts`, the `forge-receipt-helper` surface, and the targeted no-install execution guard. This is `SOURCE-ONLY` evidence that the editable receipt helper is materialized and covered by the lane guard; it is not live Markdown/MDX renderer proof.

The package-status row also exposes a package-owned `receipt_hash_refresh` manifest through `examples/template/markdown-mdx-content-receipt-hashes.ts`. That helper tracks `docs/packages/content-react-markdown.source-guard-runbook.json`, the package docs, the focused lane guard, and the Studio manifest in `examples/template/.dx/forge/receipts/2026-05-22-content-react-markdown-source-guard.json`, then mirrors the current hashes into the `source-guard-runbook-fixture` selected surface, the typed read model, and the Zed receipt surface `markdown-mdx-content:receipt-hash-refresh`. This makes fixture-path drift stale-detectable while the www-template package receipt stays checked in and source-owned.

The launch template now reports `present` because `.dx/forge/receipts/packages/content-react-markdown.json` is checked in for the default starter. That is still source-only honesty: Forge source is receipt-backed for dx-check/Zed, but no live Markdown renderer, MDX execution, dependency install, browser render, or sanitization proof is claimed.

## Rust dx-check Emitter

The Rust `dx check` Forge section reads `.dx/forge/package-status.json` for package id `content/react-markdown` and the package receipt `.dx/forge/receipts/packages/content-react-markdown.json`.

It emits `markdown_mdx_content_package_present`, `markdown_mdx_content_receipt_present`, `markdown_mdx_content_receipt_stale`, `markdown_mdx_content_missing_receipt`, `markdown_mdx_content_blocked_surface`, `markdown_mdx_content_unsupported_surface`, `markdown_mdx_content_hash_manifest_present`, `markdown_mdx_content_hash_mismatch`, `markdown_mdx_content_receipt_hash_refresh_current`, `markdown_mdx_content_receipt_hash_refresh_stale`, `markdown_mdx_content_receipt_hash_refresh_missing`, `markdown_mdx_content_materialized_source_present`, and `markdown_mdx_content_materialized_source_missing`. Missing receipts produce `markdown-mdx-content-missing-receipt`; missing materialized receipt-helper evidence produces `markdown-mdx-content-missing-materialized-source`; stale, blocked, unsupported, hash-mismatched, and stale-helper package-status rows produce matching Markdown & MDX Content findings, including `markdown-mdx-content-hash-mismatch`, without claiming live Markdown/MDX runtime proof.

Hash freshness is byte-derived when either selected package-status surfaces expose `hash_algorithm: "sha256"` with `file_hashes`, or the generated Markdown & MDX Content receipt contains `files[].hashes.sha256`. The `source-guard-runbook-fixture` selected surface covers the package-owned runbook fixture and its focused proof files, while the www-template package receipt covers editable renderer, provider, server compile, and receipt-helper files for the default starter. The helper-freshness metrics are separate: `markdown_mdx_content_receipt_hash_refresh_stale` or `markdown_mdx_content_receipt_hash_refresh_missing` can mark the package row stale while `markdown_mdx_content_hash_mismatch` remains reserved for selected source-file byte drift.

The focused fixture `markdown_mdx_content_package_metrics_reports_hash_mismatch_metric_and_finding` writes a temporary generated receipt plus package-status row, mutates `components/content/markdown.tsx`, and proves `markdown_mdx_content_receipt_stale`, `markdown_mdx_content_hash_manifest_present`, `markdown_mdx_content_hash_mismatch`, and `markdown-mdx-content-hash-mismatch` flip together without claiming live renderer proof.

The generated-starter receipt fixture imports the materialized `lib/markdown-mdx-content/receipt.ts` source from the Forge template, invokes `createMarkdownMdxContentReceipt`, and asserts the `materializedSource` and `dxStyleCompatibility` payloads survive execution from generated source. This is source-only proof for the receipt helper, not live Markdown/MDX rendering proof.

The DX Studio/check-panel package row now reads the same package-status row for `content/react-markdown` and surfaces `markdown_mdx_content_receipt_hash_refresh_current` / `markdown_mdx_content_receipt_hash_refresh_stale` / `markdown_mdx_content_receipt_hash_refresh_missing` plus `markdown_mdx_content_materialized_source_present` / `markdown_mdx_content_materialized_source_missing` beside receipt, hash, blocked, unsupported, and dx-style evidence. A missing helper-freshness row prompts `markdown-mdx-content-receipt-hashes.ts --write`; a missing materialized-source row prompts regeneration of the package-status `materializedSource` evidence. Neither path claims live Markdown/MDX renderer proof.

The focused Rust fixture `dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row` writes a temporary Markdown & MDX Content package-status row and package receipt, proves the check-panel row reports `markdown_mdx_content_materialized_source_present = 1`, removes only the materialized-source payload, and then proves `markdown_mdx_content_materialized_source_missing = 1` while the package stays receipt-backed and source-only. Target it with `cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib`.

The focused Rust fixture `dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_hash_refresh_row` flips only the `receipt_hash_refresh.status` and `stale_file_count` payload in a temporary package-status row, proving `markdown_mdx_content_receipt_hash_refresh_current` drops, `markdown_mdx_content_receipt_hash_refresh_stale` rises, and `markdown_mdx_content_hash_mismatch` stays byte-clean. Target it with `cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_hash_refresh_row --lib`.

## DX Style And Zed Markers

The visible MDX provider surface carries:

- `data-dx-package="content/react-markdown"`
- `data-dx-package-name="Markdown & MDX Content"`
- `data-dx-component="dx-mdx-provider"`
- `data-dx-style-surface="markdown-mdx-content"`
- `data-dx-zed-surface="content-mdx-provider"`

The Markdown renderer keeps typography classes in front-facing source so dx-style can scan and report unsupported project utilities. Final prose typography remains app-owned.

Package-status and Rust dx-check now expose `markdown_mdx_content_dx_style_compatibility_present` and `markdown_mdx_content_dx_style_compatibility_missing` from the `dx.forge.package.dx_style_compatibility` row for the `mdx-provider` surface. Missing evidence raises `markdown-mdx-content-missing-dx-style-compatibility`; this is source-marker and token-reference evidence only, not live renderer proof.

Generated receipts from `createMarkdownMdxContentReceipt` now carry the same dx-style compatibility metadata by default, so Zed/DX Studio can read the source-marker and token-boundary evidence from the package receipt even before opening raw package-status JSON.

## App-owned Boundaries

- Installing `react-markdown`, `@mdx-js/mdx`, `@mdx-js/react`, React, and any remark/rehype plugins.
- Content moderation, raw HTML policy, sanitization review, link governance, and private-content policy.
- Trust decisions for MDX evaluation or `run`/`evaluate` style execution.
- Production bundler integration, cache policy, metadata extraction, and runtime verification.
- Final typography, design tokens, and docs/content information architecture.

## No Node Modules Path

The DX/Forge starter path remains source-owned and no-install. Forge writes editable renderer/provider/compile source and metadata; the consuming app decides when to install runtime dependencies and run live Markdown/MDX verification.

## Source Guard

```powershell
dx run --test .\benchmarks\markdown-mdx-content-slice.test.ts
```

The guard fails if the assigned upstream mirrors, official package naming, MDX provider, MDX compile helpers, generated-starter receipt execution, dx-check visibility states, byte-derived hash fixture, dx-style/Zed markers, launch catalog metadata, or this package document disappear.

```powershell
node tools/launch/run-template-receipt-helper.js examples/template/markdown-mdx-content-receipt-hashes.ts --check --json
```

The receipt-hash helper checks and reports the source-guard runbook fixture receipt without installing dependencies, starting a server, reading secrets, or claiming live renderer proof. Use `--write` only after reviewing source changes so the receipt, package-status row, and typed read model stay synchronized.

Zed/DX Studio exposes the targetable source-only runbook entry `markdown-mdx-content-materialized-source-fixture` for:

```powershell
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib
```

`docs/packages/content-react-markdown.source-guard-runbook.json` is the package-owned generated-manifest fixture for that `/launch` `source_guard_runbook_index` entry, including the command, contract, source-only execution flags, upstream provenance, Zed/DX Studio markers, and app-owned boundaries. Studio metadata now exposes the same path through structured `fixture_path` fields on the source guard, runbook contract, and runbook command, plus a route-level `fixture_paths` index so tools do not need to parse proof strings.

That command proves the check-panel materialized-source present/missing metrics from local package-status evidence only; it does not claim live Markdown or MDX renderer proof.

Zed/DX Studio also publishes the helper-freshness check-panel runbook entry `markdown-mdx-content-check-panel-helper-freshness` for:

```powershell
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_hash_refresh_row --lib
```

That command proves the package row distinguishes stale receipt-helper freshness from selected source-file hash drift. The package-owned runbook fixture now records the same structured `fixture_path`, `contract`, and `command` metadata for `markdown_mdx_content_receipt_hash_refresh_current`, `markdown_mdx_content_receipt_hash_refresh_stale`, `markdown_mdx_content_receipt_hash_refresh_missing`, and byte-derived `markdown_mdx_content_hash_mismatch`, so Studio can rerun the proof without parsing Rust or claiming live Markdown/MDX renderer proof.

## Intentionally Deferred

- Live Markdown/MDX rendering proof before app-owned dependency installation.
- Any automatic plugin policy for GFM, math, syntax highlighting, frontmatter, or raw HTML.
- Runtime MDX evaluation of untrusted content.
- Full Tailwind Typography parity; dx-style compatibility remains visible through scanned source and receipts.
