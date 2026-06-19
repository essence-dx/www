# Examples World Integration Lab Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create `examples/world` as a template-level integration lab that maps the top three targets from every `WORLD.md` category into WWW/DX provider contracts without hardcoding provider support into Rust.

**Architecture:** `examples/world` is a normal WWW app using public framework surfaces: `app/`, `lib/`, `server/`, `styles/`, `public/`, `dx`, DX Env Firewall, typed provider registries, source-owned receipts, and status routes. Integrations are provider cards and adapters with `preview-only`, `live-if-env-present`, or `live-validated` states. Framework upgrades discovered during implementation are recorded in root `PLAN.md`, not implemented directly.

**Tech Stack:** DX WWW TSX app template, TypeScript provider registry, route-handler status adapters, `.sr`/`.machine` evidence expectations, DX Env Firewall, focused TypeScript source tests.

---

### Task 1: World Project Skeleton

**Files:**
- Create: `examples/world/dx`
- Create: `examples/world/README.md`
- Create: `examples/world/package.json`
- Create: `examples/world/tsconfig.json`
- Create: `examples/world/.gitignore`
- Create: `examples/world/app/layout.tsx`
- Create: `examples/world/app/page.tsx`
- Create: `examples/world/styles/globals.css`
- Create: `examples/world/styles/theme.css`
- Create: `examples/world/styles/generated.css`

- [ ] Create a normal WWW project shape with no framework Rust changes.
- [ ] Keep all app code `.ts` / `.tsx`; do not add `.js`, `.cjs`, or `.mjs`.
- [ ] Render the root dashboard from registry data, not hardcoded card markup.

### Task 2: Provider Registry And Contracts

**Files:**
- Create: `examples/world/lib/world/contracts.ts`
- Create: `examples/world/lib/world/categories.ts`
- Create: `examples/world/lib/world/registry.ts`
- Create: `examples/world/lib/world/status.ts`
- Create: `examples/world/lib/world/env.ts`
- Create: `examples/world/lib/world/receipts.ts`

- [ ] Encode all 20 `WORLD.md` categories.
- [ ] Encode the top three targets for each category.
- [ ] Store provider env requirements, runtime boundary, validation mode, adapter kind, and receipt expectations.
- [ ] Mark providers as `preview-only` unless live env variables are present.
- [ ] Include Vercel and Turso as live-capable providers because the local machine may already have their CLIs/accounts.

### Task 3: Status Routes And Example Pages

**Files:**
- Create: `examples/world/app/integrations/page.tsx`
- Create: `examples/world/app/readiness/page.tsx`
- Create: `examples/world/app/api/world/status/route.ts`
- Create: `examples/world/server/world/status.ts`

- [ ] Add a read-only `/integrations` page with category cards.
- [ ] Add a `/readiness` page that summarizes preview/live status and framework suggestions.
- [ ] Add `GET /api/world/status` returning redacted provider readiness.
- [ ] Ensure no raw env value is returned to the browser or status route.

### Task 4: Framework Suggestions In PLAN.md

**Files:**
- Modify: `PLAN.md`

- [ ] Add a professional `World Integration Suggestions` section.
- [ ] List only framework-level ideas discovered while building the example.
- [ ] Do not change Rust framework behavior for those ideas in this task.

### Task 5: Verification

**Files:**
- Create: `benchmarks/examples-world-contract.test.ts`

- [ ] Test that `WORLD.md` top-three targets are represented in `examples/world`.
- [ ] Test that `examples/world` uses no `.js`, `.cjs`, or `.mjs` app scripts.
- [ ] Test that provider cards include status, env, receipt, and adapter metadata.
- [ ] Test that `PLAN.md` contains the framework suggestions section.
- [ ] Run focused test and lightweight repo checks before committing.
