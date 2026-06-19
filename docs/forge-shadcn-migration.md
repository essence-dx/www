# DX Forge UI Compatibility Migration Guide

This guide is for developers who already understand the normal shadcn/ui workflow and want the closest Forge equivalent for the public beta.

Forge does not claim to replace the full upstream CLI or npm ecosystem today. The current migration lane is intentionally narrow: `ui/button`, `ui/alert`, `ui/avatar`, `ui/badge`, `ui/card`, `ui/label`, `ui/separator`, `ui/field`, `ui/item`, `ui/input`, `ui/skeleton`, and `ui/textarea` can be materialized as source-owned local files with receipts, package docs, update previews, rollback evidence, and no `node_modules` dependency tree. Upstream `shadcn/ui/*` names stay compatibility and provenance metadata.

## Command Mapping

| shadcn/ui habit | Forge command | What changes |
| --- | --- | --- |
| `npx shadcn@latest add button` | `dx add ui/button --dry-run` | Preview the files, hashes, policy decisions, and receipt before writing. |
| `npx shadcn@latest add button` | `dx add ui/button --write` | Materialize editable source files and record Forge ownership evidence. |
| `npx shadcn@latest add alert` | `dx add ui/alert --write` | Materialize the source-owned alert primitive with title and description slots. |
| `npx shadcn@latest add avatar` | `dx add ui/avatar --write` | Materialize avatar image/fallback primitives as project-owned source. |
| `npx shadcn@latest add badge` | `dx add ui/badge --write` | Materialize the source-owned status primitive with `Badge`, `badgeVariants`, and `data-slot="badge"`. |
| `npx shadcn@latest add card` | `dx add ui/card --write` | Materialize the source-owned layout primitive with card subcomponents and `data-slot` ownership. |
| `npx shadcn@latest add label` | `dx add ui/label --write` | Materialize the accessible label primitive used by `FieldLabel`. |
| `npx shadcn@latest add separator` | `dx add ui/separator --write` | Materialize the source-owned section divider primitive with orientation metadata. |
| `npx shadcn@latest add field` | `dx add ui/field --write` | Materialize field grouping, labels, descriptions, separators, and error slots for launch forms. |
| `npx shadcn@latest add item` | `dx add ui/item --write` | Materialize list-row and action primitives for dashboard/package rows. |
| `npx shadcn@latest add input` | `dx add ui/input --dry-run` | Preview the real input component slice from the v4 radix registry shape. |
| `npx shadcn@latest add input` | `dx add ui/input --write` | Materialize `components/ui/input.tsx` plus the local `cn` helper. |
| `npx shadcn@latest add textarea` | `dx add ui/textarea --dry-run` | Preview the real textarea component slice from the v4 radix registry shape. |
| `npx shadcn@latest add textarea` | `dx add ui/textarea --write` | Materialize `components/ui/textarea.tsx` plus the local `cn` helper. |
| `npx shadcn@latest add skeleton` | `dx add ui/skeleton --write` | Materialize the source-owned skeleton loading primitive. |
| Inspect generated code | `dx forge migration-guide --project . --format markdown` | Generate a local migration report with file maps, receipts, docs, and no-`node_modules` status. |
| Check package health | `dx forge verify-package shadcn/ui/textarea --project .` | Verify registry integrity, docs, update preview, rollback state, and scorecard evidence. |
| Update generated code | `dx update ui/button --dry-run` | Review green/yellow/red file traffic before a write can happen. |

## Local Ownership Model

After `dx add ui/button --write`, the app owns the copied source files. Forge owns the review evidence around those files:

- `.dx/forge/source-manifest.json` records tracked package files and hashes.
- `.dx/forge/receipts/*-shadcn-ui-button.json` records the add/update policy decisions.
- `.dx/forge/docs/shadcn-ui-button.md` explains ownership and review boundaries.
- `.dx/forge/docs/shadcn-ui-badge.md`, `.dx/forge/docs/shadcn-ui-card.md`, `.dx/forge/docs/shadcn-ui-field.md`, `.dx/forge/docs/shadcn-ui-item.md`, `.dx/forge/docs/shadcn-ui-label.md`, and `.dx/forge/docs/shadcn-ui-separator.md` document the selected launch primitives and their upstream registry provenance.
- `.dx/forge/docs/shadcn-ui-alert.md`, `.dx/forge/docs/shadcn-ui-avatar.md`, and `.dx/forge/docs/shadcn-ui-skeleton.md` document the added source-owned UI primitives and their upstream registry provenance.
- `.dx/forge/docs/shadcn-ui-input.md` documents the launch form primitive and its upstream registry provenance.
- `.dx/forge/docs/shadcn-ui-textarea.md` documents the launch long-form field primitive and its upstream registry provenance.
- `examples/dashboard/src/components/ShadcnDashboardControls.tsx` proves the package set in a visible starter-dashboard workflow with `data-slot`, `data-variant`, `data-size`, DX icon markers, and a safe local settings receipt.
- `examples/template/shadcn-dashboard-controls-contract.tsx` owns the typed density/queue options, package metadata, and local receipt builder for the `/launch` dashboard controls, and materializes as `components/launch/shadcn-dashboard-controls-contract.tsx`.
- `examples/template/shadcn-dashboard-controls.tsx` proves the package set inside `/launch` as real operator controls for density, queue focus, filtering, notes, direct mission-card focus, and a safe local dashboard receipt. The runtime-safe route keeps selected density/focus controls synchronized with `aria-pressed`, links focused controls to affected dashboard metrics with `aria-controls`, styles live selected controls through `data-dx-shadcn-dashboard-selected`, exposes exact `data-dx-package="shadcn/ui/button"` markers on visible button interactions, exposes `data-dx-shadcn-dashboard-controls-target` for Web Preview mapping, supports arrow/Home/End roving focus through `data-dx-shadcn-dashboard-keyboard="arrow-roving-focus"`, adds `data-dx-shadcn-dashboard-action="focus-target-card"` for dashboard-card focus handoff, and announces receipt changes with `aria-live` while leaving final accessibility review app-owned.
- The earlier primitive source-edit proof has been removed from the www-template source and generated materialization path. Launch-visible shadcn/ui behavior must flow through the dashboard controls workflow.
- `examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json` records the source-owned `/launch` shadcn dashboard workflow, stable selectors, controlled dashboard targets, upstream public APIs including `Label` as the `FieldLabel` dependency, app-owned boundaries, and lightweight guards.
- `dx templates --json` advertises the same `LaunchShadcnDashboardControls` workflow with its `[data-dx-component="shadcn-dashboard-controls"]` selector and receipt path so DX CLI/Zed can discover the dashboard interaction instead of only primitive snippets.
- `dx update ui/button --dry-run` shows whether local files are clean, edited, missing, or blocked.
- `dx forge verify-package shadcn/ui/input --project .` checks the input package before release.
- `dx forge verify-package shadcn/ui/textarea --project .` checks the textarea package before release.

Local edits are allowed. Forge should not silently overwrite them during updates; edited files become reviewable yellow state unless the change is security-sensitive or structurally unsafe.

## Minimal Migration Flow

```powershell
dx add ui/button --dry-run
dx add ui/button --write
dx add ui/alert --write
dx add ui/avatar --write
dx add ui/badge --write
dx add ui/card --write
dx add ui/label --write
dx add ui/separator --write
dx add ui/field --write
dx add ui/item --write
dx add ui/input --dry-run
dx add ui/input --write
dx add ui/textarea --dry-run
dx add ui/textarea --write
dx add ui/skeleton --write
dx forge migration-guide --project . --format markdown --output .\.dx\forge\shadcn-migration.md
dx forge verify-package shadcn/ui/textarea --project . --fail-under 90
dx update ui/button --dry-run
```

The generated migration guide should show:

- `components/ui/button.tsx`, `components/ui/alert.tsx`, `components/ui/avatar.tsx`, `components/ui/badge.tsx`, `components/ui/card.tsx`, `components/ui/label.tsx`, `components/ui/separator.tsx`, `components/ui/field.tsx`, `components/ui/item.tsx`, `components/ui/input.tsx`, `components/ui/skeleton.tsx`, or `components/ui/textarea.tsx` in the file map;
- passing materialized, docs, receipts, verify-package, local-ownership, and no-`node_modules` checks;
- the matching shadcn command, Forge write command, update preview command, and package gallery command;
- the honest scope boundary that this is not a universal npm replacement.

## Beta Boundary

Use this lane to migrate and explain the selected shadcn primitives listed above. The starter dashboard may compose them into app workflows, but it must keep persistence, final copy, accessibility review, and full registry synchronization app-owned. Do not use it to claim full shadcn registry parity, every npm package workflow, or full framework replacement.
