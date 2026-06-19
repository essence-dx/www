# DX WWW UI Components Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a WWW-native, source-owned UI Components project inspired by shadcn-ui, with no npm package dependency path and with DX Style, DX Icon, Forge receipts, and DX Check as the first-class toolchain.

**Architecture:** `examples/ui-components` is a real DX WWW App Router project created by `dx new`. Upstream shadcn-ui remains a provenance mirror at `G:\WWW\inspirations\shadcn-ui`; the project consumes Forge-materialized baseline source primitives and represents the full 56-file registry surface as WWW-native TSX without `react`, Radix, lucide, CVA, tailwind-merge, Recharts, Embla, or package-manager runtime imports. Source-owned primitives render now; browser-runtime-heavy pieces use explicit adapter boundaries until DX owns those engines.

**Tech Stack:** DX WWW App Router TSX, DX Style generated CSS, DX Icon, DX Forge UI Components receipts, DX Check, Node source guards.

---

## File Structure

- `examples/ui-components/dx`: project config with `style`, `icons`, `forge`, and `check`.
- `examples/ui-components/app/page.tsx`: launch page for the UI Components website.
- `examples/ui-components/app/docs/components/page.tsx`: component docs route.
- `examples/ui-components/app/docs/components/primitives/page.tsx`: primitive gallery route.
- `examples/ui-components/app/registry/page.tsx`: registry/provenance route.
- `examples/ui-components/components/home/ui-components-home.tsx`: homepage composition.
- `examples/ui-components/components/site/*.tsx`: site shell and source map sections.
- `examples/ui-components/components/gallery/*.tsx`: primitive gallery and previews.
- `examples/ui-components/components/ui/*.tsx`: full shadcn registry surface represented as WWW-native source-owned components or explicit adapter boundaries.
- `examples/ui-components/lib/ui-components/provenance.ts`: upstream commit, source files, implemented components, blocked packages, and package replacement notes.
- `examples/ui-components/lib/ui-components/catalog.ts`: component catalog and implementation status.
- `examples/ui-components/styles/theme.css`: tokens only.
- `examples/ui-components/styles/globals.css`: app CSS that imports generated CSS and uses tokens.
- `benchmarks/ui-components-www-project.test.ts`: source guard for project integrity.

## Task 1: Project Boundary And Guard

**Files:**
- Create: `benchmarks/ui-components-www-project.test.ts`
- Create: `examples/ui-components/lib/ui-components-provenance.ts`
- Modify: `examples/ui-components/README.md`

- [x] **Step 1: Create project through DX**

Run:

```powershell
cd G:\Dx\www\examples
dx new ui-components
```

Expected: `examples/ui-components` exists with `app/`, `components/`, `styles/`, `dx`, `.dx/serializer`, and no `node_modules`.

- [x] **Step 2: Materialize the first Forge primitives**

Run:

```powershell
cd G:\Dx\www
dx forge add shadcn/ui/button --project examples\ui-components --write
dx forge add shadcn/ui/badge --project examples\ui-components --write
dx forge add shadcn/ui/card --project examples\ui-components --write
dx forge add shadcn/ui/label --project examples\ui-components --write
dx forge add shadcn/ui/separator --project examples\ui-components --write
dx forge add shadcn/ui/field --project examples\ui-components --write
dx forge add shadcn/ui/item --project examples\ui-components --write
dx forge add shadcn/ui/input --project examples\ui-components --write
dx forge add shadcn/ui/textarea --project examples\ui-components --write
dx forge add icon/search --project examples\ui-components --write
```

Expected: `.dx/forge/source-manifest.json` lists the UI Components packages and receipts.

- [x] **Step 3: Add a source guard**

The guard must assert:

```js
const forbidden = [
  "from \"react\"",
  "from 'react'",
  "@radix-ui",
  "lucide-react",
  "class-variance-authority",
  "tailwind-merge",
  "clsx",
  "recharts",
  "embla-carousel",
  "cmdk",
  "sonner",
  "next/",
  "node_modules",
]
```

Expected: the guard fails until component source is adapted away from React/npm imports.

## Task 2: WWW-Native Primitive Adaptation

**Files:**
- Create: `examples/ui-components/components/ui/types.ts`
- Modify: `examples/ui-components/components/ui/*.tsx`

- [x] **Step 1: Replace React prop types**

Use this local boundary:

```ts
export type DxElementProps = {
  className?: string;
  children?: any;
  [attribute: string]: any;
};
```

Expected: UI primitive source does not import `react`.

- [x] **Step 2: Remove React-only runtime helpers**

Replace `useMemo`, `forwardRef`, `cloneElement`, and `React.ComponentProps` with WWW-safe source code. `asChild` remains a documented adapter boundary until WWW owns clone semantics.

Expected: primitives render deterministic TSX without hidden runtime packages.

## Task 3: Site Shell And Routes

**Files:**
- Create: `examples/ui-components/components/site/site-header.tsx`
- Create: `examples/ui-components/components/site/site-shell.tsx`
- Create: `examples/ui-components/components/site/registry-map.tsx`
- Create: `examples/ui-components/components/site/dependency-replacement.tsx`
- Create: `examples/ui-components/components/home/ui-components-home.tsx`
- Create: `examples/ui-components/components/gallery/primitive-gallery.tsx`
- Create: `examples/ui-components/components/gallery/primitive-preview.tsx`
- Modify: `examples/ui-components/app/page.tsx`
- Create: `examples/ui-components/app/docs/components/page.tsx`
- Create: `examples/ui-components/app/docs/components/primitives/page.tsx`
- Create: `examples/ui-components/app/registry/page.tsx`

- [x] **Step 1: Build the first UI Components website screen**

Use the materialized primitives:

```tsx
import { Button } from "../components/ui/button";
import { Badge } from "../components/ui/badge";
import { Card } from "../components/ui/card";
```

Expected: the homepage presents full registry coverage, upstream provenance, no package-manager install path, and DX commands.

- [x] **Step 2: Add component/docs/registry routes**

Expected: routes are App Router pages under `app/`, not `pages/` or static `.html`.

## Task 4: DX Style And Theme

**Files:**
- Modify: `examples/ui-components/styles/theme.css`
- Modify: `examples/ui-components/styles/globals.css`
- Generated: `examples/ui-components/styles/generated.css`

- [x] **Step 1: Add tokens**

Use semantic tokens such as `--background`, `--foreground`, `--card`, `--primary`, `--primary-foreground`, `--accent`, `--muted`, `--border`, and `--ring`.

- [x] **Step 2: Use DX Style classes and custom CSS together**

Custom component classes must use tokens. TSX may use DX Style grouped class strings for compact examples.

- [x] **Step 3: Regenerate style output**

Run:

```powershell
cd G:\Dx\www\examples\ui-components
dx style build --json
```

Expected: `styles/generated.css` updates through DX Style, not by hand.

## Task 5: Verification And Commit

**Files:**
- Modify as above.

- [x] **Step 1: Run focused guards**

Run:

```powershell
cd G:\Dx\www
node --test benchmarks\ui-components-www-project.test.ts
git diff --check -- docs\superpowers\plans\2026-06-03-dx-www-ui-components.md benchmarks\ui-components-www-project.test.ts examples\ui-components
```

- [x] **Step 2: Run DX checks**

Run:

```powershell
cd G:\Dx\www\examples\ui-components
dx imports sync
dx icons sync --json
dx style check --json
dx check . --json
```

- [ ] **Step 3: Commit focused work**

Stage only:

```powershell
docs\superpowers\plans\2026-06-03-dx-www-ui-components.md
benchmarks\ui-components-www-project.test.ts
examples\ui-components
```

Commit:

```powershell
git commit -m "Add DX WWW UI Components example"
```

Do not stage unrelated dirty whiteboard files.

## Current Evidence

- `node --test benchmarks\ui-components-www-project.test.ts`: PASS, 5/5 for the first slice; later full-surface guard expects 56 component files.
- `dx imports check --json`: PASS.
- `dx style check --json`: PASS.
- `dx icons check --json`: PASS.
- `dx check . --json`: PASS, score 100, green.
- `dx build`: PASS, 4 routes compiled to `.dx/www/output`.
- `cargo check -p dx-www --no-default-features --features cli --bin dx-www`: PASS.
- `git diff --check` for focused files: PASS.

## Remaining Scope

- The committed checkpoint implemented the first source-owned shadcn/UI Components slice: button, badge, card, label, separator, field, item, input, textarea, and DX Icon search/check usage.
- The follow-up full-surface pass adds all remaining registry component files. Remaining product work is deeper adapter ownership for portals, focus management, command menus, toast, charts, carousel, dialog, popover, and other browser-runtime-heavy surfaces.
