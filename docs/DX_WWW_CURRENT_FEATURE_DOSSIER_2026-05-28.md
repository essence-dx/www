# DX-WWW Current Feature Dossier

Generated: 2026-05-28 20:45:03 +06:00
Workspace: `G:\Dx\www`
Purpose: give another AI a dense, honest, source-grounded map of what DX-WWW currently is, what it already does, and what still needs product work.

> This dossier is intentionally long. It is a planning substrate, not a marketing page. It includes confirmed source features, evidence receipts, architecture decisions, current gaps, and a large file index so a later AI worker can reason with less guessing.

## Current Git Snapshot
```text
## dev...origin/dev [ahead 5]
 M benchmarks/dx-devtools-framework-integration.test.ts
 M benchmarks/repo-hygiene-audit.test.ts
 M docs/repo-hygiene.md
 M dx-www/src/cli/devtools/assets.rs
 M dx-www/src/cli/devtools/assets/runtime.ts
?? docs/DX_WWW_CURRENT_FEATURE_DOSSIER_2026-05-28.md
?? docs/superpowers/plans/2026-05-28-www-scorecard-closure-pass-2.md
?? dx-www/src/cli/devtools/assets/runtime/
```

Recent commits:
```text
0446122d chore: checkpoint before scorecard closure pass
f7e1d49e chore: harden repo hygiene scorecard
91b9ba23 chore: checkpoint before 12-flaw hardening
e2d38823 chore: sweep repo hygiene blockers
5352a080 chore: checkpoint before repo hygiene sweep
397bdd17 Refresh WWW template and onboard effects
bcdb5e49 Restore onboard Friday and pixel runtime
98cd4de7 Checkpoint onboard conversion state
18104daf Remove duplicate lowercase agent handoff
a17eb0b9 Expand lowercase DX-WWW agent handoff
e5ce3fb3 Document current DX-WWW agent handoff
bc3b2720 Refresh Forge receipts for green template check
```

## Executive Summary

DX-WWW is a source-owned web framework and toolchain living in `G:\Dx\www`.
It is written primarily in Rust, with TypeScript/TSX authoring at the user-facing edge.
The framework intentionally borrows the mental model of modern Next.js App Router projects while avoiding a normal Node/React/Next runtime as the framework core.

The most important current product identity is:

- familiar App Router authoring shape;
- Rust-owned CLI, compiler, resolver, builder, checker, and dev server;
- source-owned package governance through Forge;
- generated CSS through DX Style instead of project-local package pipelines;
- source-owned icon pipeline through DX Icons;
- `.dx/*` receipts and machine-readable state as first-class evidence;
- template/dev/onboard examples designed to run without project-local `node_modules`;
- devtools work that is intended to be available only during development;
- serializer-aware config and receipt philosophy, even though not every check feature is fully wired yet;
- honest distinction between what is proven and what is aspirational.

The framework is not currently a full drop-in implementation of React, React DOM, or Next.js.
It can render a bounded React-shaped TSX subset and supports many App Router-shaped conventions, but arbitrary ecosystem React packages still require either compatibility adapters or a much larger React-compatible runtime layer.
That is a deliberate architecture choice so far, because copying full React DOM semantics would add the cost and complexity DX-WWW is trying to avoid.

The current best framing for future planning is:

- DX-WWW is already a real framework, not a mock.
- DX-WWW is strongest when projects author in its supported App Router-shaped subset.
- DX-WWW should not promise full Next.js compatibility until the runtime contract truly supports it.
- The highest-value upgrades are likely around TSX feature coverage, devtools integration, hot reload quality, `dx check` completeness, template polish, and Forge package maturity.

## Six-Agent Research Inputs

This file incorporates six independent research lanes.

### Agent 1: Architecture And Public Contract

Findings:

- DX-WWW is presented as a source-owned Rust web framework.
- It uses Next-familiar vocabulary but does not embed Next.js as the runtime.
- It supports `app/` and `src/app/` style project roots.
- It supports pages, layouts, templates, loading states, error boundaries, not-found files, metadata, viewport metadata, and route handlers in a bounded compiler/runtime model.
- Public contracts emphasize `app`, `components`, `lib`, `server`, `styles`, `public`, `dx`, and `.dx`.
- The root docs distinguish between what is real today and what is not claimed.
- The docs still have some drift: richer launch/dashboard language exists in places, while the current minimal starter is intentionally much smaller.

### Agent 2: CLI And Command Surface

Findings:

- The main package is `dx-www`.
- The visible binary reports as `dx-www 1.0.0`.
- The installed binary evidence uses `G:\Dx\bin\dx-www.exe`.
- Important command surfaces include `dx new`, `dx create`, `dx dev`, `dx build`, `dx check`, `dx add`, `dx forge`, `dx serializer`, `dx style`, `dx icons`, `dx imports`, and `dx www agent-context`.
- `dx style build/watch/check` and `dx icons sync/check` exist as framework-owned workflows.
- `dx www agent-context` is a major handoff command for AI workers.

### Agent 3: Runtime And App Router

Findings:

- App Router discovery supports static, dynamic, catch-all, optional catch-all, route group, and parallel-slot style routes.
- Route handlers support common HTTP verbs and safe patterns around `Response`, `NextResponse`, redirects, headers, cookies, request body, query, and params.
- Metadata handling recognizes `metadata`, `generateMetadata`, `viewport`, and `generateViewport`.
- Build output includes route discovery, per-route HTML, route packet files, page graph data, execution contracts, server data, generated style, and streaming plan concepts.
- TSX rendering is bounded rather than arbitrary React execution.
- Runtime currently has client island and DOM action binder concepts.
- Hot reload currently exists, but the strongest proven path has been polling/full-page refresh rather than full React-style partial module replacement.

### Agent 4: Style, Icons, Forge, And Resolver

Findings:

- DX Style is a real related crate and CLI surface.
- DX Icons is source-owned through the media icon crate/pipeline.
- Forge package governance exists through core ecosystem files and CLI forge modules, not one single root `forge` crate.
- Generated CSS scans TSX/JSX/TS/JS/HTML/MDX-like content.
- The project should avoid full Tailwind parity claims unless the parser and rule generator prove those cases.
- Import resolution classifies relative imports, aliases, Forge imports, reviewed adapters, compiler intrinsics, blocked `node_modules`, and unresolved imports.
- The current minimal template does not visibly exercise every Forge package feature.

### Agent 5: Template And Example Projects

Findings:

- Current `dx new` uses a minimal starter.
- The minimal starter has one app page, one app layout, theme CSS, generated CSS, globals CSS, public logo/icon/favicon, README, `.gitignore`, and a `dx` config.
- It creates `.dx` folders for Forge, check, run, WWW output, receipts, deploy, and serializer state.
- Richer examples such as onboard/Friday/HelloGlow are separate from the minimal starter and should not be confused with default `dx new`.
- Devtools routes/assets exist in source, including session, route, diagnostics, source-map, CSS data, style preview, and guarded style apply concepts.

### Agent 6: Verification And Evidence

Findings:

- Installed binary smoke receipt exists and reports pass.
- Root readiness receipt reports source readiness as true but product readiness as false.
- The current template check receipt was not present at generation time.
- A current measured Lighthouse/web-perf template receipt was not found at generation time.
- Benchmark/test surface is broad, but this dossier did not rerun heavy full suites.
- The repo currently has an untracked superpowers plan file and is ahead of remote; those are state facts, not framework features.

## What DX-WWW Is

DX-WWW is a framework, a CLI, a compiler, a dev server, a package governance layer, a style compiler, an icon pipeline, a template system, and a receipt-driven project checker.

The guiding product idea is that a developer should get familiar modern web authoring without outsourcing the core product to opaque package-manager behavior.
Instead of starting from `npm install react next tailwind ...`, the project starts from DX-owned contracts and generated state.
The project can still learn from React and Next.js ergonomics, but it should not depend on Next.js to be useful.

DX-WWW is especially designed for:

- source-owned applications;
- AI-assisted maintenance;
- small, inspectable generated output;
- project-local evidence receipts;
- Forge-governed packages;
- no hidden project-local package maze for core framework features;
- explicit route and runtime contracts;
- templates that are easy to understand and easy to modify.

The framework currently favors correctness through a bounded renderer over full ecosystem compatibility.
That means the framework can be extremely fast in the cases it owns, but it must carefully expand its TSX/runtime coverage before claiming generic React/Next project compatibility.

## What DX-WWW Is Not

DX-WWW is not currently:

- a full React DOM reimplementation;
- a full Next.js runtime clone;
- a guarantee that arbitrary `node_modules` React packages will render unchanged;
- a browser-only static HTML generator with no runtime concepts;
- a normal Tailwind/Vite/Next wrapper;
- a marketing-only scaffold;
- a fake template that only works on a hand-authored demo.

The architecture is more original than that.
It is closer to a Rust-owned App Router-shaped compiler/runtime with source-owned package and style governance.

This distinction matters for future feature planning.
The best upgrades should strengthen DX-WWW's native path rather than accidentally pulling it back into being a normal React/Next wrapper.

## Current Feature Inventory

### 1. Project Creation

DX-WWW has a project creation path through `dx new` / `dx create`.
The current default public starter is intentionally minimal.
It creates a one-page App Router-shaped app rather than a large showcase site.

Current starter concepts:

- `app/page.tsx` as the main route;
- `app/layout.tsx` as the root layout;
- `styles/theme.css` for design tokens;
- `styles/generated.css` for generated framework/style output;
- `styles/globals.css` for global CSS;
- `public/logo.svg`, `public/icon.svg`, and `public/favicon.svg`;
- a `dx` extensionless config file;
- `.dx` working folders for framework receipts and generated machine state.

This is good for a production framework starter because it avoids dumping a whole demo site into every new project.
The richer UI examples should remain examples, not the default starter.

### 2. App Router-Shaped Authoring

DX-WWW recognizes the mental model of Next.js App Router without using Next.js as the internal engine.
The project can be authored with familiar file names and route segment conventions.

Supported or represented concepts include:

- `app/` root;
- `src/app/` root;
- `page.tsx`, `page.jsx`, `page.ts`, `page.js`;
- `layout.tsx`;
- `template.tsx`;
- `loading.tsx`;
- `error.tsx`;
- `not-found.tsx`;
- route groups;
- parallel slots;
- dynamic segments;
- catch-all segments;
- optional catch-all segments;
- app API route handlers.

The important caveat is that this is App Router-shaped, not full Next.js.
The supported behavior depends on what the DX-WWW parser/compiler/runtime can interpret.

### 3. TSX Rendering

DX-WWW includes a source renderer for TSX-like files.
It can interpret a bounded subset of React-shaped authoring.

Current renderer strengths:

- component-shaped authoring;
- props and simple JSX composition;
- intrinsic HTML elements;
- class names and style strings in supported forms;
- source-owned imports;
- selected framework intrinsics;
- basic event binding through runtime action systems;
- static route HTML generation;
- app route output generation.

Current renderer limits:

- not arbitrary JavaScript execution;
- not arbitrary React runtime semantics;
- not full hook/context/effect compatibility;
- not automatic support for any package that expects React DOM;
- not guaranteed support for complex third-party component libraries.

For future work, TSX coverage is one of the highest leverage areas.
Every supported authoring feature expands the class of real websites DX-WWW can own without paying the full React DOM tax.

### 4. Build System

DX-WWW has a Rust-owned build command.
The build process emits framework-owned output under `.dx/www/output` or equivalent configured output directories.

Observed/proven build concepts include:

- route discovery;
- route manifests;
- per-route HTML;
- packet-like route output;
- page graph data;
- app-router execution contracts;
- server data output;
- generated style output;
- public asset handling;
- no project-local `node_modules` creation in the installed smoke proof.

The build path is one of the strongest current parts of the framework because evidence receipts show a real installed-binary smoke build passing.

### 5. Dev Server

DX-WWW has a dev command/server path.
The dev server is intended to serve the App Router-shaped project directly.

Current dev concepts:

- local host/port configuration in the `dx` config;
- static asset serving;
- route serving;
- generated CSS serving;
- runtime script serving;
- hot reload/version endpoint support;
- devtools-only surfaces;
- source-map/diagnostic endpoints in devtools code.

The most honest current caveat is hot reload quality.
Hot reload exists, and TSX edit-refresh proof has existed in previous work, but full partial module replacement parity is not the current claim.

### 6. Hot Reload And Feedback Loop

DX-WWW has hot reload/dev feedback infrastructure.
This includes runtime scripts, version/polling concepts, and dev server routes.

Current strengths:

- full-page edit-refresh style proof has existed;
- source changes can be reflected through dev server output;
- route/runtime version endpoints exist;
- dev feedback is part of the architecture.

Current gaps:

- not full React Fast Refresh parity;
- SSE/event path proof has had mismatches in earlier reports;
- hot reload should become a formal benchmark/edit-cycle test;
- component-level state-preserving refresh is not the proven default.

Future feature planning should treat hot reload as a major differentiator.
Fast reload is one of the areas where owning both the framework and preview can beat generic editor/browser setups.

### 7. Route Handlers

DX-WWW supports route handler concepts under `app/api` and `src/app/api`.

Recognized patterns include:

- HTTP method exports such as `GET`, `HEAD`, `POST`, `PUT`, `PATCH`, `DELETE`, and `OPTIONS`;
- `new Response(...)`;
- `Response.json(...)`;
- `NextResponse`-like response patterns;
- redirects;
- headers;
- cookies;
- URL/query usage;
- request body usage;
- route params.

The route handler implementation is intentionally bounded and safe.
It should be expanded by adding real supported patterns, not by blindly executing arbitrary server code.

### 8. Metadata And Viewport

DX-WWW recognizes App Router metadata concepts.

Supported/represented concepts include:

- `metadata`;
- `generateMetadata`;
- `viewport`;
- `generateViewport`;
- page title output;
- description-like output;
- safe metadata extraction.

This feature is important because metadata is part of the real framework ergonomics people expect from Next-like authoring.

### 9. Styling Through DX Style

DX Style is a first-class source-owned styling path.
It is not just a CSS file.
The style system is intended to scan source files, generate CSS, and keep styling inside the DX ecosystem.

Current style concepts:

- `dx style build`;
- `dx style watch`;
- `dx style check`;
- generated CSS output;
- token-aware theme CSS;
- source scanning across TSX/JSX/TS/JS/HTML/MDX-like files;
- public generated CSS path in templates;
- no need for project-local Tailwind package installs for the starter path.

Important caution:

- do not claim complete Tailwind parity unless all arbitrary values, variants, plugins, and edge cases are actually supported.
- DX Style should be judged by its own contract and then expanded deliberately.

### 10. Icons Through DX Icons

DX Icons is the source-owned icon pipeline.
It replaces the need for default starter dependency on `lucide-react` or similar packages.

Current icon concepts:

- icon sync/check command surfaces;
- generated icon wrappers;
- source-owned icon assets;
- no runtime dependency on a React icon package for core framework examples.

The current minimal template may not actively exercise a large icon set.
That is fine for a starter, but the icon package should have richer examples and stronger package docs.

### 11. Forge Package Governance

Forge is the DX package governance idea.
It aims to make packages source-owned, reviewable, and compatible with AI-assisted maintenance.

Current Forge concepts:

- Forge registry/read-model files;
- package status receipts;
- root package status visibility;
- template package status folders;
- import classification;
- source-owned package policy;
- reviewed adapters;
- no hidden package maze as the framework default.

Forge should eventually become one of the most important DX differentiators:

- packages can be inspected before entering an app;
- AI can reason about package source without guessing;
- app teams can avoid mystery dependency chains;
- secure/adopted package state can be serialized into `.dx` receipts.

### 12. Import Resolver And Module Linker

DX-WWW includes import scanning/resolution infrastructure.

Current resolver categories include:

- relative imports;
- alias imports such as `@/*`;
- Forge imports;
- reviewed adapter imports;
- compiler intrinsic imports;
- blocked `node_modules` imports;
- unresolved imports.

This is central to the product.
If the resolver is excellent, DX-WWW can safely support more third-party-ish authoring while still preserving governance.

### 13. Devtools

DX-WWW has devtools source surfaces.
The devtools are intended for development only and should not leak production-only filesystem abstractions.

Current devtools concepts:

- `/_dx/devtools`;
- runtime JS;
- devtools CSS;
- session endpoint;
- route endpoint;
- diagnostics endpoint;
- source-map endpoint;
- CSS data endpoint;
- style preview endpoint;
- guarded style apply endpoint.

Future devtools work should connect visual editing directly to the framework:

- selected element;
- closest parent;
- route/component source mapping;
- style editing;
- box model;
- responsive breakpoints;
- generated DX Style output;
- direct code writes where safe;
- no AI-token waste for simple visual changes.

### 14. Diagnostics

DX-WWW has diagnostics and code-frame ambitions already represented in source and docs.

Current diagnostic areas:

- parser errors;
- route discovery errors;
- import resolution errors;
- build errors;
- framework contract errors;
- check receipts;
- machine-readable output;
- developer-facing source maps.

Diagnostics are a major future differentiator.
The framework should make errors feel like a senior engineer explained the fix in place.

### 15. `dx check`

DX-WWW includes a check command and receipt-oriented checking model.

Current/claimed check areas include:

- project structure;
- framework configuration;
- buildability;
- Forge package status;
- style/icon checks;
- web performance checks;
- Lighthouse-style scoring when available/configured;
- receipt generation;
- machine-readable output.

Honest current state:

- installed build smoke evidence exists;
- readiness evidence exists;
- current template check receipt was not present during this dossier generation;
- measured web-perf/Lighthouse receipt was not found during this dossier generation;
- check help routing and nested command polish have been known paper cuts.

Future `dx check` should become one of DX-WWW's biggest weapons.
It can score not only technical validity but best practices, file size, design patterns, security, accessibility, performance, and AI maintainability.

### 16. Serializer Alignment

The DX ecosystem has a serializer strategy:

- `.sr` files for token-efficient LLM-readable state;
- `.machine` files for fast tool-readable state;
- generated state under `.dx/serializer`;
- avoid inventing fresh JSON formats for every subsystem.

DX-WWW should continue aligning all check/devtools/package receipts with that serializer strategy.
Where JSON receipts exist today, future work can either preserve compatibility or add serializer-native mirrors.

### 17. Evidence Receipts

DX-WWW treats evidence as a product feature.
Receipts are not just logs.
They are state that future tools and AI agents can read.

Current receipt types observed:

- installed binary smoke build receipt;
- readiness receipt;
- Forge package status receipt;
- template package status receipt;
- expected check receipt path;
- expected preview manifest path.

The current best evidence at dossier time:

- installed binary smoke passed;
- root readiness says source-ready true;
- root readiness says product-ready false;
- template check receipt missing;
- preview manifest missing;
- web-perf receipt missing.

This should guide planning.
DX-WWW is real, but the evidence layer can become much stronger.

### 18. Accessibility

There is a dedicated accessibility crate in the workspace.
This implies DX-WWW wants accessibility to be part of the build/check ecosystem rather than a later plugin.

Future improvements:

- accessibility checks in `dx check`;
- route-level a11y receipts;
- semantic HTML checks;
- keyboard flow checks;
- color contrast checks;
- ARIA misuse warnings;
- devtools overlays for accessibility issues.

### 19. Browser Runtime

DX-WWW has browser and browser-micro crates.
The docs and crate descriptions point toward a small browser/WASM runtime concept.

Current browser/runtime concepts:

- small runtime scripts;
- client island manifests;
- DOM action binding;
- state/reactor/scheduler crates;
- interaction/form/query/offline/cache/morph-related packages;
- route runtime files.

This area is where DX-WWW can become radically different from React.
Instead of a full virtual DOM framework tax, it can ship only route-needed runtime.

### 20. State And Reactor

Workspace crates include `state`, `reactor`, `sched`, `interaction`, and related runtime primitives.

The design direction appears to be:

- small runtime state;
- fine-grained updates where supported;
- source-owned scheduling;
- event/action binding;
- no generic React runtime tax;
- route/component-specific runtime output.

Future feature work should define the public authoring API for state carefully.
This is a foundational decision.

### 21. Server, Cache, Offline, Query, Form, Auth, DB

The workspace contains many framework capability crates beyond the core compiler.

Current crate areas include:

- server;
- cache;
- offline;
- query;
- form;
- auth;
- db;
- db teleport;
- guard;
- observability;
- print;
- rtl;
- sync;
- packet;
- fallback.

These indicate a broader full-stack framework vision.
Not every crate should be treated as mature product surface yet.
The planner should inspect each crate before promising external users a full feature.

### 22. Templates And Examples

There are multiple example/template concepts in the repo history:

- minimal default template;
- onboard example;
- richer visual examples with HelloGlow and Friday;
- launch/dashboard language in older docs;
- benchmark fixtures;
- test fixtures.

Current product recommendation:

- default `dx new` should stay clean and minimal;
- examples should demonstrate rich framework features;
- onboarding/showcase can be impressive but separate;
- docs should clearly label examples vs default starter.

### 23. Agent Context And AI Workflow

DX-WWW has explicit AI-worker handoff concepts through docs and commands.

Important surfaces:

- `dx www agent-context`;
- manager handoff docs;
- lane/pass operating model;
- receipts;
- `.dx` machine state;
- status docs;
- repo hygiene docs.

This is a huge strategic advantage.
DX-WWW can be the first framework that treats AI maintainability as a core framework property.

### 24. Testing And Benchmarks

The repo has broad testing/benchmark directories and many fixture files.

Current observed areas:

- Rust tests;
- TypeScript/Node benchmarks;
- App Router contracts;
- route handler contracts;
- hot reload contracts;
- focused benchmark scripts;
- integration tests crate;
- fixture examples.

Honest caveat:

- this dossier did not run heavy tests;
- test surface exists, but current green status should be verified live before a launch claim.

### 25. Repo Structure

DX-WWW is not a small single-crate project.
It is a multi-crate framework workspace.
The source is split into core framework crates, CLI, examples, docs, related crates, tools, tests, and generated state.

The strongest future maintainability push should continue reducing giant files, clarifying ownership boundaries, and making every large feature discoverable through a small set of well-named modules.

## Workspace Crate Inventory

| Path | Crate | Description |
|---|---|---|
| `Cargo.toml` | `<workspace>` |  |
| `a11y/Cargo.toml` | `dx-www-a11y` | Compile-time accessibility auditor |
| `auth/Cargo.toml` | `dx-www-auth` | Binary authentication - Ed25519 tokens with passkey support |
| `benchmarks/Cargo.toml` | `dx-www-benchmarks` | Benchmark suite for dx-www framework |
| `benchmarks/binary-web-lab/Cargo.toml` | `binary-web-lab` |  |
| `binary/Cargo.toml` | `dx-www-binary` | The binary protocol that killed JSON and HTML |
| `browser/Cargo.toml` | `dx-www-browser` | Sub-20KB browser WASM runtime for dx-www - pure FFI, no wasm-bindgen |
| `browser-micro/Cargo.toml` | `dx-www-browser-micro` | Micro browser WASM runtime for dx-www - NO_STD Integer DOM (< 2 KB) |
| `build/Cargo.toml` | `dx-www-build` |  |
| `cache/Cargo.toml` | `dx-www-cache` | The Eternal Binary Cache Engine - 0ms second-visit LCP |
| `cli/Cargo.toml` | `dx-www-cli` | CLI tool for DX WWW Framework - Binary-first web development |
| `core/Cargo.toml` | `dx-www-compiler` | The Transpiler-to-Binary Pipeline - Converts .tsx to .dxb and .wasm |
| `db/Cargo.toml` | `dx-www-db` | Zero-copy database layer with compile-time SQL verification |
| `db-teleport/Cargo.toml` | `dx-www-db-teleport` | Reactive database caching with zero-copy binary responses |
| `demo/Cargo.toml` | `dx-www-demo` |  |
| `dom/Cargo.toml` | `dx-www-dom` | Binary DOM operations with zero-copy element manipulation |
| `dx-option/Cargo.toml` | `dx-option` | Official DX Binary Dawn Website |
| `dx-www/Cargo.toml` | `dx-www` | DX WWW Framework - Binary-first, multi-language web framework with file-system routing |
| `error/Cargo.toml` | `dx-www-error` | Binary error boundaries |
| `fallback/Cargo.toml` | `dx-www-fallback` | HTML fallback mode |
| `flow/Cargo.toml` | `flow` | Library-first local and remote AI runtime layer for DX: voice, typing, model routing, and embeddable Rust AI capabilities |
| `flow/crates/flow-browser-core/Cargo.toml` | `flow-browser-core` | WASM-friendly browser orchestration core for Flow local browser inference |
| `flow/crates/forge/Cargo.toml` | `forge` | Blazing-fast version control for massive media assets |
| `flow/crates/serializer/Cargo.toml` | `serializer` | Serializer - Token-efficient serialization format for LLM prompts |
| `form/Cargo.toml` | `dx-www-form` | Binary validation engine - compile-time schema validation with zero runtime overhead |
| `framework-core/Cargo.toml` | `dx-www-core` | Core WASM runtime for dx-www framework |
| `guard/Cargo.toml` | `dx-www-guard` | DOM integrity protection |
| `integrations/flow-forge/Cargo.toml` | `forge` | Blazing-fast version control for massive media assets |
| `integrations/flow-serializer/Cargo.toml` | `serializer` | Serializer - Token-efficient serialization format for LLM prompts |
| `interaction/Cargo.toml` | `dx-www-interaction` | User action preservation |
| `morph/Cargo.toml` | `dx-www-morph` | Binary DOM diffing and morphing engine |
| `observability/Cargo.toml` | `dx-www-observability` | Observability infrastructure for dx-www: tracing, metrics, and structured logging |
| `offline/Cargo.toml` | `dx-www-offline` | CRDT offline engine |
| `packet/Cargo.toml` | `dx-www-packet` | Zero-dependency binary protocol types for dx-www |
| `print/Cargo.toml` | `dx-www-print` | Print stylesheet generator |
| `query/Cargo.toml` | `dx-www-query` | Binary RPC data fetching - zero-parse request/response with cache |
| `reactor/Cargo.toml` | `dx-www-reactor` | Binary Dawn - Cross-platform I/O reactor with thread-per-core architecture |
| `related-crates/markdown/Cargo.toml` | `markdown` | DX Markdown (DXM) - Next-generation documentation format optimized for AI, machines, and humans |
| `related-crates/markdown/fuzz/Cargo.toml` | `markdown-fuzz` |  |
| `related-crates/media-icon/Cargo.toml` | `dx-icons` | World's fastest icon search engine with FST, rkyv, and semantic search |
| `related-crates/security/Cargo.toml` | `dx-security` | Binary-level security scanner with SIMD acceleration and cryptographic attestation |
| `related-crates/style/Cargo.toml` | `dx-style` | Binary-first CSS engine with zero-copy parsing and DX Serializer output |
| `related-crates/style/playground/Cargo.toml` | `dx-style-playground` |  |
| `router/Cargo.toml` | `dx-www-router` | File-based router for DX-WWW framework |
| `rtl/Cargo.toml` | `dx-www-rtl` | RTL detection and CSS flipping |
| `sched/Cargo.toml` | `dx-www-sched` | Priority-based task scheduler for WASM runtime |
| `server/Cargo.toml` | `dx-www-server` | The Holographic Server - SSR, Binary Streaming, and Delta Patching |
| `state/Cargo.toml` | `dx-www-state` | Binary state management - memory slots with dirty tracking |
| `sync/Cargo.toml` | `dx-www-sync` | Realtime binary WebSocket protocol |
| `tests/Cargo.toml` | `dx-www-integration-tests` |  |

## Important Source Files And Status

- `README.md`: present, 10122 bytes, 198 lines
- `DX.md`: present, 1329979 bytes, 2511 lines
- `CURRENT_STATE.md`: present, 27501 bytes, 507 lines
- `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`: present, 50675 bytes, 126 lines
- `docs/dx-www-developer-contract.md`: present, 7015 bytes, 86 lines
- `docs/DX_WWW_MANAGER_HANDOFF.md`: present, 28247 bytes, 145 lines
- `docs/root-workspace-status.md`: present, 57977 bytes, 222 lines
- `docs/repo-hygiene.md`: present, 7384 bytes, 129 lines
- `dx-www/src/cli/mod.rs`: present, 975150 bytes, 22888 lines
- `dx-www/src/cli/source_render.rs`: missing
- `dx-www/src/cli/devtools/mod.rs`: present, 2573 bytes, 79 lines
- `dx-www/src/cli/project_check.rs`: missing
- `core/src/ecosystem/forge_registry.rs`: present, 431022 bytes, 10286 lines
- `core/src/ecosystem/dx_check_receipt.rs`: present, 650080 bytes, 14043 lines
- `core/src/ecosystem/router/app_router.rs`: missing
- `core/src/ecosystem/build_types.rs`: missing
- `examples/template/app/page.tsx`: present, 651 bytes, 16 lines
- `examples/template/app/layout.tsx`: present, 381 bytes, 15 lines
- `examples/template/dx`: present, 349 bytes, 13 lines
- `examples/template/styles/theme.css`: present, 233 bytes, 11 lines

## Receipt And Machine-State Snapshot

- `examples/template/.dx/receipts/check/check-latest.json`: missing at generation time
- `.dx/receipts/build/installed-binary-smoke-latest.json`: present, 34506 bytes, modified 2026-05-28 08:12:04
```json
{
  "schema": "dx.build.installedBinarySmoke",
  "schemaRevision": 1,
  "binary": "G:\\Dx\\bin\\dx-www.exe",
  "binaryDefault": "G:\\Dx\\bin\\dx-www.exe",
  "binaryOverride": false,
  "binaryRole": "installed-default",
  "binaryIdentity": {
    "path": "G:\\Dx\\bin\\dx-www.exe",
    "present": true,
    "kind": "file",
    "byteLength": 69445632,
    "modifiedMs": 1779934303075,
    "sha256": "a4010527e9752aab9b53d3650a7775b60144cc3771ba6a201f8704c5845d91c0"
  },
  "binarySourceFreshness": {
    "required": true,
    "repoRoot": "G:\\Dx\\www",
```
- `.dx/receipts/build/readiness.json`: present, 3749 bytes, modified 2026-05-25 12:08:12
```json
{
  "consumers": {
    "dx_cli": "read .dx/receipts/build/readiness.json for source/product score split",
    "dx_www": "render source-ready and governed-runtime-pending status without parsing prose",
    "zed_preview": "open zed_handoff and graph_consumer_snapshot for editor preview details"
  },
  "content_freshness": {
    "checked_document_count": 53,
    "comparison": "manifest-self-check",
    "consumer_action": "compare content-docs hash_manifest.document_hashes against current source bytes and reject unsafe receipt paths before trusting older receipts",
    "current": true,
    "hash_algorithm": "blake3-16",
    "missing_paths": [],
    "receipt_section": "content-docs.json#freshness_contract",
    "runtime_proof": false,
    "safe_path_roots": [
      "docs",
      "content"
```
- `.dx/forge/package-status.json`: present, 21862 bytes, modified 2026-05-26 06:21:16
```json
{
    "schema":  "dx.forge.package_status",
    "status":  "root-visibility-only",
    "generated_at":  "2026-05-26T00:20:42.2821873+00:00",
    "package_policy":  "forge-first-no-node-modules",
    "no_node_modules_required":  true,
    "package_count":  5,
    "package_lane_visibility":  [
                                    {
                                        "official_package_name":  "Backend Platform Client",
                                        "package_id":  "supabase/client",
                                        "status":  "present",
                                        "receipt_status":  "present",
                                        "package_receipt_path":  ".dx/forge/receipts/root-supabase-client-visibility.json",
                                        "receipt_hash_refresh":  {
                                                                     "schema":  "dx.forge.package.receipt_hash_refresh",
                                                                     "status":  "current",
                                                                     "helper_path":  "",
```
- `examples/template/.dx/forge/package-status.json`: present, 344263 bytes, modified 2026-05-28 20:06:41
```json
{
  "schema": "dx.www_template.forge_package_status",
  "status": "lock-backed",
  "source": "examples/template/forge-package-lock.ts",
  "generated_at_unix_ms": 1779789066986,
  "package_policy": "forge-packages-versioned-by-content-not-node_modules",
  "no_node_modules_required": true,
  "catalog_package_count": 30,
  "package_count": 20,
  "locked_package_count": 20,
  "locked_package_names": [
    "shadcn/ui/button",
    "state/zustand",
    "tanstack/query",
    "validation/zod",
    "forms/react-hook-form",
    "db/drizzle-sqlite",
    "instantdb/react",
```
- `examples/template/public/preview-manifest.json`: missing at generation time

## Framework Gaps And Opportunities

1. DX-WWW is already a real source-owned framework with real compiler/build/dev/check/devtools surfaces.
2. The strongest proven evidence at generation time is installed-binary smoke build evidence.
3. The root readiness receipt says source readiness is true but product readiness is false.
4. The current minimal starter is cleaner than older richer docs imply.
5. Full React/Next ecosystem compatibility is not current reality.
6. TSX rendering is bounded and should be expanded intentionally.
7. Devtools are a strategic unlock but still need full product wiring.
8. `dx check` has the potential to be game-changing, but the check evidence needs stronger current receipts.
9. Forge is strategically important, but current examples need stronger visible package adoption.
10. DX Style and DX Icons are real ownership moves, but should avoid overclaiming full Tailwind/lucide ecosystem parity.
11. Hot reload exists but should be turned into a measured benchmark and improved beyond full-page refresh.
12. The repo still has structural debt around very large CLI/source files and some stale docs.

The framework should be marketed as ambitious, fast, source-owned, and AI-native.
It should not be marketed as a magic full Next.js replacement until the missing runtime semantics are actually implemented.

## Best Feature Bets For The Next Planning AI

The next AI planning pass should strongly consider these feature priorities:

1. Formalize the public TSX support matrix.
2. Add more TSX syntax coverage where it unlocks real apps without full React DOM.
3. Make `dx check` serializer-native and plugin-extensible.
4. Add first-class best-practice packs as `.sr` rules.
5. Turn Lighthouse/web-perf into a reliable desktop/mobile check path with current receipts.
6. Integrate devtools into every dev project automatically.
7. Build visual style editing that writes DX Style/code directly.
8. Strengthen hot reload with a measured edit-cycle test.
9. Make source maps reliable from rendered DOM to TSX source.
10. Expand Forge package adoption examples.
11. Add package security/adoption/provenance checks to Forge.
12. Clean stale docs so default starter, examples, and launch docs are not confused.
13. Split large framework files only where it improves ownership.
14. Keep default starter minimal and professional.
15. Build richer examples separately from `dx new`.
16. Improve App Router parity where it fits DX-WWW's native model.
17. Improve route handler server code coverage.
18. Add a component compatibility cookbook.
19. Add a migration guide for React/Next projects into DX-WWW-native authoring.
20. Add an honest "unsupported React patterns" diagnostic with suggested rewrites.
21. Make generated `.dx` state explainable to AI workers.
22. Add `dx www doctor` or strengthen `dx check` as the one command for truth.
23. Keep performance receipts local and current.
24. Build a public benchmark story based on owned evidence.
25. Preserve the no-project-local-node_modules core path while defining optional external package interop honestly.

## Feature Pattern Evidence Index

The following index is generated from source search. It is not a semantic proof by itself, but it tells a future worker where to inspect each feature.

### Pattern: `App Router`
- `AGENTS.md:27:- `.tsx` App Router-shaped authoring under `app/`.`
- `CHANGELOG.md:11:  diagnostics/code frames, route graph/status, App Router support,`
- `CHANGELOG.md:25:- Reframed App Router request-props, navigation control-flow, metadata, and`
- `CHANGELOG.md:32:  plus focused App Router request, navigation, metadata, and render-plan`
- `CHANGELOG.md:47:  App Router route handlers. Safe `Response.json(...)` and `new Response(...)``
- `CHANGELOG.md:66:  DX-owned App Router navigation control-flow scanner. The source-safe scanner`
- `CHANGELOG.md:75:- Added source-owned execution for route-handler header reads. Safe App Router`
- `CHANGELOG.md:94:  App Router route handlers. Raw text bodies now bind through the same DX`
- `CHANGELOG.md:103:  safe App Router route handlers. `form.get("field")` now resolves from an`
- `CHANGELOG.md:120:- Preserved App Router required versus optional catch-all semantics in the`
- `CHANGELOG.md:139:- Aligned build diagnostics with `src/app` App Router roots. Segment files and`
- `CHANGELOG.md:158:  reads. The safe App Router route-handler interpreter now resolves`
- `CHANGELOG.md:166:- Added source-owned App Router navigation support for `RedirectType.push` and`
- `CHANGELOG.md:229:- Fixed explicit `HEAD` route-handler dispatch. The source-owned App Router`
- `CHANGELOG.md:330:- Added bounded source-owned execution for `new Response(...)` in App Router`
- `CHANGELOG.md:404:- Added bounded source-owned `notFound()` support to the App Router route`
- `CHANGELOG.md:435:  with the outer App Router matcher for absolute request URLs while preserving`
- `CHANGELOG.md:493:  `dx.www.moduleGraph` contract, so App Router routes can retain graph edges`
- `CHANGELOG.md:509:- Added source-build server-data manifest consistency for App Router routes.`
- `CHANGELOG.md:528:  TSX/App Router diagnostics stay aligned without claiming broader`
- `CHANGELOG.md:537:  App Router routes and components now keep graph edges for imports such as`
- `CHANGELOG.md:626:- Aligned core App Router route-handler execution with route-group and`
- `CHANGELOG.md:628:  non-public App Router path segments such as `(internal)` and `@admin` while`
- `CHANGELOG.md:665:- Aligned core App Router route-handler execution with Next-familiar route`
- `CHANGELOG.md:768:- Added source-build discovery for destructured App Router route-handler helper`
- `CHANGELOG.md:885:- Added App Router route-handler alias export discovery. The source-build`
- `CHANGELOG.md:953:- Added source-owned App Router request-value decoding. Page and API route`
- `CHANGELOG.md:998:  so deploy output cannot look complete while dropping a non-GET App Router`

### Pattern: `route handler`
- `CHANGELOG.md:47:  App Router route handlers. Safe `Response.json(...)` and `new Response(...)``
- `CHANGELOG.md:76:  route handlers now resolve `request.headers.get(...)`, `const headers =`
- `CHANGELOG.md:94:  App Router route handlers. Raw text bodies now bind through the same DX`
- `CHANGELOG.md:103:  safe App Router route handlers. `form.get("field")` now resolves from an`
- `CHANGELOG.md:111:  Router route handlers. `request.nextUrl.pathname`, `request.nextUrl.search`,`
- `CHANGELOG.md:140:  route handlers under `src/app` now use the same source-owned parse/code-frame`
- `CHANGELOG.md:331:  route handlers. The safe interpreter now handles literal/text response`
- `CHANGELOG.md:995:  `/api/checkout` POST route handler as `skipped-build-execution` with`
- `CHANGELOG.md:1506:- Completed Lane 5 Forge package reality for Payments, AI SDK, and Automation Connectors. The launch template now lock-backs `payments/stripe-js`, `ai/vercel-ai`, and `automations/n8n` through the Forge source manifest, package lock, cache manifests, package-add receipts, safety archives, and package-status/read-model surfaces. The default template materializes editable Stripe checkout/client/server/webhook helpers and checkout/webhook route handlers, editable AI route/client/helper files with `/api/ai/*` route handlers, and source-owned n8n connector catalog/readiness/local dry-run receipt helpers. The visible package-reality model now gives these packages real controls and scores while keeping live Stripe credentials, AI provider keys/model execution, and n8n workflow execution as app-owned adapter boundaries. Guarded with red/green lane 5 package reality, green Forge lock check, green shared Forge/template guard, and green package source guards for Payments, AI SDK, and Automation Connectors receipt freshness; no heavy builds, servers, installs, deploys, or live provider calls ran.`
- `CHANGELOG.md:1670:- Began the Next.js source-merge compatibility lane. The official `vercel/next.js` mirror already exists at `G:/WWW/inspirations/nextjs` with MIT license/provenance, so DX WWW now records a compatibility matrix in `docs/NEXTJS_COMPATIBILITY_MAP.md` instead of copying the framework into the runtime. The map covers App Router, layouts, route handlers, metadata, server/client components, server actions, redirects/not-found, image/font/script boundaries, middleware, static export, and cache/revalidate, and labels reuse/direct, adapter-boundary, and not-useful-for-DX areas. Added `dx-www/src/cli/app_router_execution/next_navigation.rs` and wired it into generic TSX App Router execution so safe literal `redirect()`, `permanentRedirect()`, and `notFound()` imports from `next/navigation` emit `dx.next.appRouterControlFlow`, `NEXT_REDIRECT`, `NEXT_HTTP_ERROR_FALLBACK;404`, `data-dx-next-navigation-control-flow`, `data-dx-next-redirect`, `data-dx-next-not-found`, and safe head hints without `node_modules`. Guarded with red/green `dx run --test benchmarks/nextjs-compatibility-map.test.ts`, `dx run --check benchmarks/nextjs-compatibility-map.test.ts`, and targeted `rustfmt`. Targeted Cargo remains blocked by unrelated missing source-engine modules in the dirty worktree, so this is source-guarded but not compile-green.`
- `CHANGELOG.md:1750:- Added Type-Safe API helper drift attribution. The lane 8 package `api/trpc` keeps official **Type-Safe API** naming while `examples/template/type-safe-api-receipt-hashes.ts --check --json` now reports exact `tracked_files`, `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, `missing_mirror_files`, and `mirror_problem_count` fields. A new temp-root fixture proves stale evidence isolated to `docs/packages/api-trpc.source-guard-runbook.json` and `tools/launch/materialize-www-template.ts` does not falsely mark the route handler, launch health contract, starter dashboard component, dashboard helper, or Forge slice stale. The helper also writes the current attribution mirrors into package-status and refreshed the Type-Safe API receipt/package-status/read-model hashes after the source-guard runbook fixture and shared materializer moved. Re-inspected upstream tRPC `11.17.0` package metadata, `initTRPC`, fetch adapter, client factory, HTTP batch link, React Query binding, and TanStack options proxy sources for `initTRPC.context().create()`, `createCallerFactory`, `fetchRequestHandler`, `createTRPCClient`, `httpBatchLink`, `createTRPCReact`, and subscription options proxy metadata. Guarded with red/green `dx run --test ./benchmarks/trpc-receipt-hash-refresh.test.ts`, helper stale check, helper `--write`, and helper `--check --json`. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, live tRPC route execution, WebSocket/subscription proof, dependency installation, and browser visual proof were skipped. Next action: carry path-level helper attribution into the Rust DX Studio/check-panel Type-Safe API row.`
- `CHANGELOG.md:2434:- Added Type-Safe API dx-check visibility metadata. The `api/trpc` dashboard workflow receipt now carries `dx.forge.package.dx_check_visibility` for present, stale, missing-receipt, blocked, and unsupported-surface states across the launch dashboard workflow, starter dashboard workflow, and App Router tRPC route handler. The launch catalog, generated dashboard helper, CLI package discovery JSON, Forge metadata, and package docs surface the same receipt path and `type_safe_api_*` metrics. Guarded with red/green `dx run --test ./benchmarks/trpc-dx-check-visibility-receipt.test.ts`, plus `dx run --test ./benchmarks/trpc-forge-slice.test.ts` and `dx run --check ./benchmarks/trpc-dx-check-visibility-receipt.test.ts`; a scoped rustfmt check still reports pre-existing formatting drift in unrelated CLI lines, and heavy checks/live tRPC runtime proof were skipped.`
- `CHANGELOG.md:2611:- Recorded the launch runtime blockers found live: zero visible auth, payment, form/Zod, state/query, animation, 3D canvas, docs/markdown, automations, database/backend, or Studio edit-marker surfaces on `/launch`; manifest-advertised `/automations`, `/ui`, `/database`, and `/backend` routes returned 404 in the generated app; the AI route's exported `const POST` was not detected by the current route handler path; and the dynamic tRPC route was not reachable at `/api/trpc/health`.`
- `CHANGELOG.md:2613:- Added a live runtime materialization bridge for the generated launch template. The bridge writes `.html` pages, a passthrough pages layout, CSS/JS runtime assets, a Studio preview manifest, and function-export API route handlers so the current `target/debug/dx-www.exe` can serve `/launch`, `/automations`, `/ui`, `/database`, `/backend`, `/api/auth/session`, `/api/checkout`, `/api/ai/chat`, and `/api/trpc/health` without rebuilding the Rust binary or creating app-local `node_modules`.`
- `CHANGELOG.md:2678:- Completed the Better Auth CLI catalog provenance pass: the launch metadata now lists the full Better Auth exported file set, including account deletion, Next route handlers, `.env.example`, README, and curated provenance tied to the local Better Auth source mirror.`
- `CHANGELOG.md:2882:- Added `auth/better-auth` as a source-owned Forge package slice based on Better Auth `1.6.11`, including server options, Next route handlers, React client creation, env examples, package metadata, and Forge receipt docs.`
- `CURRENT_STATE.md:65:- Metadata and route handler receipt surfaces.`
- `CURRENT_STATE.md:152:- route handlers using Web `Request`/`Response`, `NextRequest`, supported HTTP methods, params, dynamic segments, cookies, headers, streaming, body/form data, CORS, and cache revalidation`
- `CURRENT_STATE.md:160:- Next route handlers: https://nextjs.org/docs/app/api-reference/file-conventions/route`
- `CURRENT_STATE.md:500:- route handlers`
- `benchmarks/app-api-route-handler-root-precedence.test.ts:12:test("App API route handler dynamic matches keep root precedence deterministic", () => {`
- `benchmarks/app-api-route-handler-extensions.test.mjs:26:test("App API route matcher accepts Next-familiar route handlers", () => {`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:238:- Verify 15 routes, assets, CSS, source maps/hash, server-data, route handler evidence.`
- `benchmarks/authentication-package-status-read-model.test.ts:188:    "Authentication App Router route handler surface is missing",`
- `benchmarks/authentication-package-status-read-model.test.ts:210:    "Authentication readiness route handler surface is missing",`
- `benchmarks/authentication-lock-backed-template.test.ts:347:  assertNoUnselectedProviders(routeHandler, "materialized auth route handler");`
- `benchmarks/automation-route-handler-provider-boundary.test.mjs:62:    "source-owned route handler must keep live provider execution visibly false",`
- `tools/worktree/www-agent4-commit-slice-plan.cjs:35:    title: "App Router, route handlers, routing, and runtime semantics",`

### Pattern: `generateMetadata`
- `CHANGELOG.md:1652:- Added TSX App Router request-prop, Link, and Image lowering. Generic TSX execution now passes matched route params and query search params into `build_tsx_source_render_surface`, emits `data-dx-tsx-page-prop-bindings`, and publishes a `dx.tsx.requestPropBindings` contract in `__DX_TSX_SOURCE_RENDER__`. The bounded source renderer resolves safe `params.slug`, `params["slug"]`, `searchParams.query`, `searchParams["query"]`, simple destructuring aliases like `const { slug } = params`, renamed aliases like `const { slug: postSlug } = params`, and member aliases like `const preview = searchParams.query` into static DOM snapshots, layout/template App Router shell composition, and source-owned component return previews when those values are already known from DX route matching. Static-safe `next/link` imports lower `<Link href="/path">` to source-owned `<a>` output, and static-safe `next/image` imports lower `<Image src alt width height />` to source-owned `<img>` output with DX framework markers instead of unresolved component noise. This improves React/App Router compatibility without `node_modules`, but still does not claim complete Next runtime parity: async Server Components, arbitrary request code, cookies/headers, dynamic `generateMetadata`, rich Link prefetch semantics, Next Image optimization, and generic JavaScript execution remain blocked. Guarded with red/green request-prop, Link, and Image Node guards, focused `dx run --test benchmarks/tsx-next-image-compat.test.ts benchmarks/tsx-next-link-compat.test.ts benchmarks/tsx-app-router-page-props.test.ts benchmarks/nextjs-compatibility-map.test.ts`, TS syntax checks, targeted `rustfmt --edition 2024 --check`, targeted `cargo check -q -p dx-www --no-default-features --features cli --bin dx-www` with unrelated pre-existing warnings, scoped diff check, and conflict-marker scan. Broader `public-framework-contract` was attempted and currently fails outside this TSX slice because `dx-www/src/cli/mod.rs` no longer contains the expected `styles/globals.css` starter contract marker. Full builds, broad suites, local servers, browser automation, package installs, deploys, and live browser parity proof were skipped. Next action: add a dynamic `/blog/[slug]?preview=1` TSX fixture proving visible route/search aliases, `next/link` anchors, and `next/image` media through the generic App Router shell.`
- `core/src/ecosystem/forge_fumadocs.rs:1423:export async function generateMetadata(`
- `docs/NEXTJS_COMPATIBILITY_MAP.md:23:| metadata | `export const metadata` plus safe literal and request-bound `generateMetadata` return compatibility | Adapter boundary | Static metadata extraction is useful. DX can read safe literal `generateMetadata` return objects, `alternates.canonical`, and a safe request-bound `generateMetadata` subset for `params`/`searchParams` values supplied by the DX request map; arbitrary dynamic metadata and metadata streaming remain explicit DX-owned follow-ups. |`
- `docs/NEXTJS_COMPATIBILITY_MAP.md:65:The next source-owned metadata slice now reads safe literal `generateMetadata()` return objects and a safe request-bound `generateMetadata()` subset:`
- `docs/NEXTJS_COMPATIBILITY_MAP.md:70:- `generateMetadata()` safe optional chaining reads such as `params?.slug`, `params?.["slug"]`, `searchParams?.preview`, and `(await searchParams)?.preview` resolve from the same request map when the value is known.`
- `docs/NEXTJS_COMPATIBILITY_MAP.md:71:- Safe parameter root aliases such as `generateMetadata({ params: routeParams, searchParams: queryParams })`, `routeParams.slug`, and `queryParams?.preview` resolve from the same request map while recording the canonical `params.*` / `searchParams.*` binding.`
- `docs/NEXTJS_COMPATIBILITY_MAP.md:72:- Safe props object destructuring such as `generateMetadata(props)`, `const { params: routeParams, searchParams: queryParams } = props`, `routeParams.slug`, and `queryParams?.preview` resolves from the same request map without executing arbitrary metadata code.`
- `docs/NEXTJS_COMPATIBILITY_MAP.md:74:- Safe const-arrow generateMetadata exports such as `export const generateMetadata = async ({ params, searchParams }) => { return { ... }; }` resolve the same known request values without importing Next runtime, React/RSC, Turbopack, or `node_modules`.`
- `benchmarks/next-intl-slice.test.ts:175:  assert.match(route, /generateMetadata/);`
- `benchmarks/next-custom-transforms-receipt.test.ts:225:    "metadata-and-generateMetadata",`
- `benchmarks/next-custom-transforms-receipt.test.ts:322:    "metadata-and-generateMetadata",`
- `benchmarks/nextjs-compatibility-map.test.ts:107:test("DX App Router records safe generateMetadata as source-owned metadata compatibility", () => {`
- `benchmarks/nextjs-compatibility-map.test.ts:117:  assert.match(metadata, /source_kind": "generateMetadata"/);`
- `benchmarks/nextjs-compatibility-map.test.ts:129:  assert.match(docs, /safe literal `generateMetadata`/);`
- `benchmarks/nextjs-compatibility-map.test.ts:130:  assert.match(docs, /request-bound `generateMetadata`/);`
- `DX.md:1986:- TSX App Router request-prop, Link, and Image lowering, 2026-05-22: generic TSX App Router execution now passes matched route params and query search params into `build_tsx_source_render_surface`, exposes `data-dx-tsx-page-prop-bindings`, and publishes a `dx.tsx.requestPropBindings` contract in `__DX_TSX_SOURCE_RENDER__`. The source-owned renderer resolves safe `params.slug`, `params["slug"]`, `searchParams.query`, `searchParams["query"]`, simple destructuring aliases like `const { slug } = params`, renamed aliases like `const { slug: postSlug } = params`, and member aliases like `const preview = searchParams.query` into static DOM snapshots, layout/template composition, and source-owned component return previews when those values are already known from DX route matching. It also recognizes static-safe `next/link` and `next/image` imports, lowering `<Link href="/path">` to source-owned `<a>` output and `<Image src alt width height />` to source-owned `<img>` output with `data-dx-framework-component` markers, so common App Router navigation/media no longer look like unresolved components. This is a real App Router compatibility step without `node_modules`; it is not external framework runtime clone parity because async Server Components, arbitrary request code, cookies/headers, dynamic `generateMetadata`, rich Link prefetch semantics, Next Image optimization, and generic JavaScript execution remain blocked. Files changed: `dx-www/src/cli/app_router_execution.rs`, `dx-www/src/cli/app_router_execution/source_render.rs`, `benchmarks/tsx-app-router-page-props.test.ts`, `benchmarks/tsx-next-link-compat.test.ts`, `benchmarks/tsx-next-image-compat.test.ts`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification: red/green request-prop, Link, and Image Node guards, focused TSX/Next guard bundle `dx run --test benchmarks/tsx-next-image-compat.test.ts benchmarks/tsx-next-link-compat.test.ts benchmarks/tsx-app-router-page-props.test.ts benchmarks/nextjs-compatibility-map.test.ts`, `dx run --check` for touched TS guards, targeted `rustfmt --edition 2024 --check`, targeted `cargo check -q -p dx-www --no-default-features --features cli --bin dx-www` with pre-existing unrelated warnings, scoped `git diff --check`, and conflict-marker scan. Broader `public-framework-contract` was attempted and currently fails outside this TSX slice because `dx-www/src/cli/mod.rs` no longer contains the expected `styles/globals.css` starter contract marker. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, and live browser parity proof. Exact next action: add a live/source fixture proving a dynamic `/blog/[slug]?preview=1` TSX route visibly renders route/search aliases, `next/link` anchors, and `next/image` media through the generic App Router shell.`
- `dx-www/src/dev/dev_feedback.rs:1507:        if normalized.starts_with("export function generateMetadata")`
- `dx-www/src/dev/dev_feedback.rs:1508:            || normalized.starts_with("export async function generateMetadata")`
- `dx-www/src/dev/dev_feedback.rs:1509:            || normalized.starts_with("export const generateMetadata")`
- `dx-www/src/dev/dev_feedback.rs:1510:            || normalized.starts_with("export let generateMetadata")`
- `dx-www/src/dev/dev_feedback.rs:1511:            || normalized.starts_with("export var generateMetadata")`
- `dx-www/src/dev/dev_feedback.rs:2060:export async function generateMetadata() {`
- `benchmarks/tsx-app-router-metadata-request-props.test.ts:12:test("safe generateMetadata request-bound reads support optional chaining without runtime takeover", () => {`
- `benchmarks/tsx-app-router-metadata-request-props.test.ts:28:  assert.match(compatibilityMap, /generateMetadata/(/).*safe optional chaining reads/s);`
- `benchmarks/tsx-app-router-metadata-request-props.test.ts:34:test("safe generateMetadata request-bound reads support destructured quoted defaults", () => {`
- `benchmarks/tsx-app-router-metadata-request-props.test.ts:52:test("safe generateMetadata request-bound reads support parameter aliases", () => {`
- `benchmarks/tsx-app-router-metadata-request-props.test.ts:59:  assert.match(metadata, /generateMetadata/(/{ params: routeParams, searchParams: queryParams /}/)/);`
- `benchmarks/tsx-app-router-metadata-request-props.test.ts:72:test("safe generateMetadata request-bound reads support const arrow exports", () => {`

### Pattern: `generateViewport`
- `dx-www/src/cli/app_router_execution/next_custom_transforms.rs:432:export function generateViewport() { return { themeColor: "#101827" }; }`
- `dx-www/src/cli/app_router_execution/next_custom_transforms.rs:478:                .any(|conflict| conflict["kind"] == "viewport-and-generateViewport")`
- `dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs:12:const VIEWPORT_CONFLICT: &str = "viewport-and-generateViewport";`
- `dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs:69:        .any(|export| export.name == "generateViewport");`
- `dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs:99:        .find(|export| export.name == "generateViewport");`
- `dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs:393:        "metadata" | "generateMetadata" | "viewport" | "generateViewport"`
- `dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs:398:    matches!(name, "generateMetadata" | "generateViewport")`
- `dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs:467:export async function generateViewport() {`
- `dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs:486:                .contains(&"viewport-and-generateViewport")`
- `dx-www/src/cli/app_router_execution/next_custom_transforms/conflicts.rs:206:                "viewport-and-generateViewport",`
- `dx-www/src/cli/app_router_execution/next_custom_transforms/conflicts.rs:207:                "Next rejects exporting viewport and generateViewport from the same server entry.",`
- `dx-www/src/cli/app_router_execution/metadata.rs:656:        "source_kind": "generateViewport",`
- `dx-www/src/cli/app_router_execution/metadata.rs:679:            "Reads safe literal fields returned directly from generateViewport().",`
- `dx-www/src/cli/app_router_execution/metadata.rs:1046:    generate_exported_named_function_parts(source, "generateViewport")`
- `dx-www/src/cli/app_router_execution/metadata.rs:1047:        .or_else(|| generate_const_arrow_function_parts(source, "generateViewport"))`
- `dx-www/src/cli/app_router_execution/metadata.rs:2583:                const generateViewport = () => ({`
- `dx-www/src/cli/app_router_execution/metadata.rs:2585:                    themeColor: "Local generateViewport helper",`
- `dx-www/src/cli/app_router_execution/metadata.rs:2587:                return <main>{generateViewport().width}</main>;`
- `dx-www/src/cli/app_router_execution/metadata.rs:2627:                const generateViewport = () => ({`
- `dx-www/src/cli/app_router_execution/metadata.rs:2629:                    themeColor: "Local generateViewport helper",`
- `dx-www/src/cli/app_router_execution/metadata.rs:2634:            export const generateViewport = async () => ({`
- `dx-www/src/cli/app_router_execution/metadata.rs:2636:                themeColor: "Exported generateViewport",`
- `dx-www/src/cli/app_router_execution/metadata.rs:2659:        .expect("exported generateViewport should be extracted after local helpers are skipped");`
- `dx-www/src/cli/app_router_execution/metadata.rs:2661:        assert_eq!(viewport["source_kind"], "generateViewport");`
- `dx-www/src/cli/app_router_execution/metadata.rs:2665:            "Exported generateViewport"`
- `dx-www/src/cli/app_router_execution/metadata.rs:2693:            export const generateViewport = () => ({`
- `dx-www/src/cli/app_router_execution/metadata.rs:2950:            export const generateViewport = async ({ searchParams }) => {`
- `benchmarks/tsx-app-router-viewport-export-diagnostics.test.cjs:27:  assert.match(metadataExports, /"viewport" /| "generateViewport"/);`

### Pattern: `hot reload`
- `CHANGELOG.md:10:  DX dev feedback, hot reload, the basic overlay at `/_dx/feedback`,`
- `CHANGELOG.md:2631:- Fixed dev-server refresh staleness by applying `no-store` cache headers to dev HTML/static responses, disabling the legacy response cache while hot reload is enabled, and wiring the Axum server to the existing watcher with an incrementing hot-reload version for app, pages, components, server/api, styles, public, and Forge route/source manifest changes.`
- `demo/PRODUCTION_READINESS_ANALYSIS.md`
- `browser/src/style_loader.rs:350:    // SAFETY: Access static hot reload manager, single-threaded WASM`
- `demo/DX_BINARY_STYLE_ANALYSIS.md:346:- Development: Fast builds, easy debugging, instant hot reload`
- `core/README.md:336:# Start development server with hot reload`
- `worker-lanes/WWW_7_AGENT_5_PASS_CLOSEOUT_PROMPT.md:38:- hot reload/dev feedback focused checks are green against the current contract; do not reintroduce unsupported SSE overclaims`
- `worker-lanes/WWW_7_AGENT_5_PASS_CLOSEOUT_PROMPT.md:71:- Keep the hot reload contract honest against the currently supported protocol.`
- `worker-lanes/WWW_30_AGENT_FINAL_POLISH_PROMPT.md:28:- Previously claimed by earlier workers: real `dx build`, cargo check/build, dev server, hot reload endpoint, focused suites, and route/build/App Router groups were green. Treat those as stale until re-verified in the current checkout.`
- `worker-lanes/WWW_30_AGENT_FINAL_POLISH_PROMPT.md:145:- Probe `/`, `/favicon.svg`, hot reload endpoint.`
- `worker-lanes/WWW_30_AGENT_FINAL_POLISH_PROMPT.md:161:- Verify hot reload protocol/source checks.`
- `cli/src/main.rs:37:    /// Start development server with hot reload`
- `cli/src/main.rs:47:        /// Disable hot reload`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:55:- Keep basic DX-owned hot reload and error overlay only.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:172:Goal: reduce warnings in dev server, hot reload, diagnostics, overlay code.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:175:- Preserve hot reload protocol and error UX.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:177:- hot reload/dev feedback focused tests.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:210:- Verify route 200, hot reload endpoint, favicon/static asset.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:276:- focused hot reload test or HTTP proof.`
- `cli/README.md:46:# Disable hot reload`
- `tools/worktree/www-agent4-commit-slice-plan.cjs:65:    title: "Dev server, hot reload, diagnostics, and overlay feedback",`
- `tools/worktree/www-agent4-commit-slice-plan.cjs:68:    checks: ["focused hot reload or diagnostics benchmark"],`
- `tools/worktree/www-agent2-ownership-map.cjs:88:    decision: "commit with hot reload, overlay, diagnostics, or dev server work",`
- `tools/worktree/www-agent2-ownership-map.cjs:181:  ["dev-feedback-lanes", "dev server, hot reload, overlay, and diagnostics changes"],`
- `tools/worktree/lane12-ownership-rules.cjs:210:    reason: "hot reload and dev server loop belong to Lane 7",`
- `docs/deployment/environment.md:134:- Relaxed CSP headers (allows hot reload)`
- `server/tests/property_tests.rs:383:/// Development CSP should allow hot reloading features.`
- `server/tests/property_tests.rs:389:    // Development should allow unsafe-eval for hot reloading`

### Pattern: `devtools`
- `CSS.md:12:DX Devtools also has a generated catalog at `dx-www/src/cli/devtools/css_data.generated.json` from the same MDN data commit.`
- `CHANGELOG.md:2088:- Added the State Management receipt-hash refresh helper. The lane 2 package `state/zustand` keeps official **State Management** naming while `examples/template/state-management-receipt-hashes.ts` now checks or refreshes selected SHA-256 hashes across the dashboard workflow receipt, `.dx/forge/package-status.json`, and `forge-package-status-read-model.ts`. The helper validates package id/name, upstream `zustand` `5.0.13`, source mirror `G:/WWW/inspirations/zustand`, `hash_algorithm: sha256`, receipt visibility hashes, package-status selected-surface mirrors, typed read-model hashes, generated-starter path fallback, and publishes `receipt_hash_refresh` / `receiptHashRefresh` plus `state-management:receipt-hash-refresh` for Zed/DX Studio without executing browser storage or reading secrets. Re-inspected upstream Zustand package metadata, README, vanilla store, React hook, traditional equality helpers, persist, subscribeWithSelector, DevTools, Redux, Immer, and shallow sources for `createStore`, `create`, `useStore`, `createWithEqualityFn`, `useStoreWithEqualityFn`, `persist`, `createJSONStorage`, `subscribeWithSelector`, `devtools`, `redux`, `immer`, `shallow`, and `useShallow`. Guarded with red/green `dx run --test ./benchmarks/state-management-receipt-hash-refresh.test.ts`, helper `--write` / `--check --json`, helper/test syntax checks, package-status/receipt JSON parse, scoped `git diff --check`, and conflict-marker scan. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo, live browser storage hydration, DevTools runtime proof, and browser visual proof were skipped. Next action: surface helper freshness in the DX Studio/check-panel State Management package row beside `state_management_dx_style_compatibility_present`.`
- `CHANGELOG.md:2140:- Added the State Management DX Studio/check-panel dx-style handoff. The lane 2 package `state/zustand` keeps official **State Management** naming while `core/src/ecosystem/dx_check_receipt.rs` now includes `state_management_dx_style_compatibility_present` and `state_management_dx_style_compatibility_missing` in the State Management package-lane row, deriving them from `dx.forge.package.dx_style_compatibility` and surfacing a missing-style next action without claiming browser storage or theme runtime proof. The launch shell, static `/launch` runtime fixture, edit contract, runtime materializer, and Studio manifest now expose `data-dx-check-package-lane-dx-style-status` for Studio/Zed discovery. Re-inspected upstream Zustand `5.0.13` package metadata/export map plus vanilla store, React hook, equality helpers, middleware exports, persist, subscribeWithSelector, DevTools, Redux, Immer, shallow, and README sources for `createStore`, `create`, `useStore`, `createWithEqualityFn`, `useStoreWithEqualityFn`, `persist`, `createJSONStorage`, `subscribeWithSelector`, `devtools`, `redux`, `immer`, `shallow`, and `useShallow`. Guarded with red/green `dx run --test ./benchmarks/zustand-dx-check-package-lane-panel.test.ts`, `dx run --test ./benchmarks/zustand-dx-style-compatibility.test.ts`, touched Node syntax checks, scoped `git diff --check`, and a line-anchored conflict-marker scan. Targeted Cargo is blocked by unrelated shared `dx_check_receipt.rs` compile drift in neighboring rows: duplicate Forms `receipt_hash_refresh`, missing Payments helpers, and stale quick-fix fields. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live browser proof were skipped. Next action: rerun the single State Management Cargo fixture once the shared reader compiles, then add a State Management receipt-hash helper for selected dashboard files.`
- `CHANGELOG.md:2971:- Extended the Zustand DevTools middleware coverage with a source-owned `devtools` bridge for Redux DevTools connect/init/send/cleanup, guarded dispatch/time-travel messages, CLI/catalog metadata, and a launch counter store named `DX Launch Counter`.`
- `docs/repo-hygiene.md:27:- `devtools-runtime-large`: `dx-www/src/cli/devtools/assets/runtime.ts` must`
- `docs/repo-hygiene.md:30:- `devtools-style-ops-large`: `dx-www/src/cli/devtools/style_ops.rs` must stay`
- `docs/repo-hygiene.md:33:- `devtools-css-large`: `dx-devtools/styles/devtools.css` must stay a small`
- `docs/repo-hygiene.md:34:  public import wrapper over the split `styles/devtools/*.css` fragments.`
- `docs/repo-hygiene.md:99:- `dx-devtools/.dx/``
- `docs/repo-hygiene.md:124:- `devtools-style-ops-large`: Rust tests now live in`
- `docs/repo-hygiene.md:125:  `dx-www/src/cli/devtools/style_ops_tests.rs`, leaving`
- `docs/repo-hygiene.md:126:  `dx-www/src/cli/devtools/style_ops.rs` under budget.`
- `docs/repo-hygiene.md:127:- `devtools-runtime-large`: the injected runtime is served by Rust from ordered`
- `docs/repo-hygiene.md:128:  `dx-www/src/cli/devtools/assets/runtime/*.ts` fragments; `runtime.ts` is now a`
- `docs/repo-hygiene.md:130:- `devtools-css-large`: the standalone Devtools app keeps`
- `docs/repo-hygiene.md:131:  `dx-devtools/styles/devtools.css` as its import entry, backed by ordered`
- `docs/repo-hygiene.md:132:  `dx-devtools/styles/devtools/*.css` fragments.`
- `docs/repo-hygiene.md:145:- `dx-www/src/cli/devtools/assets/runtime/*.ts` fragment ownership should stay`
- `docs/repo-hygiene.md:147:- `dx-devtools/styles/devtools/*.css` fragment ownership should stay below the`
- `core/src/ecosystem/forge_tanstack_query.rs:23:        ("js/query/devtools.tsx", TANSTACK_QUERY_DEVTOOLS_TSX),`
- `core/src/ecosystem/forge_tanstack_query.rs:461:    "unselected devtools runtime panel",`
- `core/src/ecosystem/forge_tanstack_query.rs:1204:} from "@tanstack/react-query-devtools";`
- `core/src/ecosystem/forge_tanstack_query.rs:4302:    "query/devtools.tsx",`
- `core/src/ecosystem/forge_tanstack_query.rs:4350:    "Runtime dependency installation and version review for optional devtools, persistence, streaming, and broadcast packages",`
- `core/src/ecosystem/forge_tanstack_query.rs:4444:      name: "@tanstack/react-query-devtools",`
- `core/src/ecosystem/forge_tanstack_query.rs:4511:- `query/devtools.tsx` adds opt-in React Query Devtools helpers around `ReactQueryDevtools`, `ReactQueryDevtoolsPanel`, and `DevtoolsPanelOptions`.`
- `core/src/ecosystem/forge_tanstack_query.rs:4546:Install or provide `@tanstack/react-query`, `@tanstack/react-query-persist-client`, `@tanstack/query-persist-client-core`, `@tanstack/query-async-storage-persister`, `@tanstack/query-sync-storage-persister`, `@tanstack/react-query-devtools`, `@tanstack/query-broadcast-client-experimental`, `@tanstack/react-query-next-experimental`, `broadcast-channel`, Next.js, and React in the app runtime when using persistence, devtools, cross-tab sync, or streamed hydration. Forge owns these adapter files and receipts; it does not vendor upstream internals.`
- `docs/superpowers/plans/2026-05-28-www-scorecard-closure-pass-2.md:28:- Modify: `dx-www/src/cli/devtools/assets.rs``

### Pattern: `style-apply`
- `docs/superpowers/plans/2026-05-28-dx-devtools-framework-integration.md:18:4. Protocol + Data: expose dev-only session, route, diagnostics, source-map, style-preview, and style-apply endpoints.`
- `benchmarks/dx-devtools-framework-integration.test.ts:317:    "style-apply should reject non-local write requests before source mutation",`
- `benchmarks/dx-devtools-framework-integration.test.ts:324:    /devtools|//_dx//devtools|style-preview|style-apply/i,`
- `benchmarks/dx-devtools-framework-integration.test.ts:344:  assertSourceMatches(devRust, /style-apply/, "dev-server code should expose style-apply");`
- `benchmarks/dx-devtools-framework-integration.test.ts:347:    ///_dx//devtools|style-preview|style-apply/i,`
- `benchmarks/dx-devtools-framework-integration.test.ts:1019:test("style-apply mutates only the expected source file after safety checks", () => {`
- `benchmarks/dx-devtools-framework-integration.test.ts:1079:  assertSourceMatches(applySnippet, /style-apply|style_apply|apply_style_change/i, "style-apply endpoint or handler should exist");`
- `benchmarks/dx-devtools-framework-integration.test.ts:1083:    "style-apply should gate source writes behind a local request check",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1088:    "style-apply local gate should require loopback Host and validate Origin/Referer when present",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1093:    "style-apply local gate should use parsed loopback IPs instead of hostname prefixes",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1095:  assertSourceDoesNotMatch(loopbackAuthority, /starts_with/("127/."/)/, "style-apply local gate should reject 127.* lookalike hostnames");`
- `benchmarks/dx-devtools-framework-integration.test.ts:1096:  assertSourceMatches(nonLocalPayload, /non-local-devtools-write/, "style-apply non-local rejections should be explicit");`
- `benchmarks/dx-devtools-framework-integration.test.ts:1100:    "style-apply should validate the requested file is inside the project source boundary",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1105:    "style-apply should name and constrain the exact source file it will mutate",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1110:    "style-apply should not treat arbitrary source target kinds as writable",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1115:    "style-apply should only write authored CSS-like source paths",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1120:    "style-apply should require a declaration-shaped expected source range",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1125:    "style-apply should reject exact ranges whose declaration property differs from the requested property",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1130:    "style-apply should perform one explicit source write when the request is safe",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1135:    "style-apply should not perform broad filesystem mutation",`
- `dx-www/src/cli/devtools/style_ops_tests.rs:242:    let project = temp_project("style-apply");`
- `dx-www/src/cli/devtools/style_ops_tests.rs:262:    let project = temp_project("style-apply-mismatch");`
- `dx-www/src/cli/devtools/style_ops_tests.rs:279:    let project = temp_project("style-apply-unsafe-target");`
- `dx-www/src/cli/devtools/style_ops.rs:12:pub(crate) const STYLE_APPLY_OPERATION: &str = "style-apply";`
- `dx-www/src/cli/devtools/protocol.rs:18:const STYLE_APPLY_ENDPOINT: &str = "/_dx/devtools/style-apply";`
- `dx-www/src/cli/devtools/protocol.rs:293:                "message": "style-apply requires a structured dx.visual_edit.style_operation payload with an exact source target.",`
- `dx-www/src/cli/devtools/protocol.rs:309:            "message": "CSS text payloads are preview-only. style-apply writes only exact structured source targets.",`
- `dx-www/src/cli/devtools/protocol.rs:342:        "message": "style-apply writes require a loopback devtools request. Open dx dev through http://localhost, http://127.0.0.1, or http://[::1] to apply source changes.",`

### Pattern: `style-preview`
- `docs/superpowers/plans/2026-05-28-dx-devtools-framework-integration.md:18:4. Protocol + Data: expose dev-only session, route, diagnostics, source-map, style-preview, and style-apply endpoints.`
- `dx-www/src/cli/devtools/style_ops_tests.rs:215:    let project = temp_project("style-preview");`
- `dx-www/src/cli/devtools/style_ops.rs:11:pub(crate) const STYLE_PREVIEW_OPERATION: &str = "style-preview";`
- `dx-www/src/cli/devtools/protocol.rs:17:const STYLE_PREVIEW_ENDPOINT: &str = "/_dx/devtools/style-preview";`
- `dx-www/src/cli/devtools/protocol.rs:449:            "/_dx/devtools/style-preview": ["POST"],`
- `dx-www/src/cli/devtools/assets/runtime/part-02-protocol.ts:60:    const request = styleRequest("style-preview");`
- `dx-www/src/cli/devtools/assets/runtime/part-01-boot.ts:12:    stylePreview: "/_dx/devtools/style-preview",`
- `benchmarks/dx-devtools-framework-integration.test.ts:324:    /devtools|//_dx//devtools|style-preview|style-apply/i,`
- `benchmarks/dx-devtools-framework-integration.test.ts:343:  assertSourceMatches(devRust, /style-preview/, "dev-server code should expose style-preview");`
- `benchmarks/dx-devtools-framework-integration.test.ts:347:    ///_dx//devtools|style-preview|style-apply/i,`
- `benchmarks/dx-devtools-framework-integration.test.ts:995:test("style-preview is read-only and cannot mutate project source", () => {`
- `benchmarks/dx-devtools-framework-integration.test.ts:1010:  assertSourceMatches(previewSnippet, /style-preview|style_preview/i, "style-preview endpoint or handler should exist");`
- `benchmarks/dx-devtools-framework-integration.test.ts:1014:    "style-preview should advertise a read-only/no-source-mutation contract",`
- `benchmarks/dx-devtools-framework-integration.test.ts:1016:  assertNoSourceMutation(previewSnippet, "style-preview");`

### Pattern: `dx-style`
- `Cargo.lock:1656:name = "dx-style"`
- `Cargo.lock:1715: "dx-style",`
- `Cargo.lock:1837: "dx-style",`
- `Cargo.lock:1969: "dx-style",`
- `AGENTS.md:33:- dx-style generated CSS and checks.`
- `AGENTS.md:68:- `styles/generated.css` is dx-style output/evidence and should not be`
- `AGENTS.md:156:- Use semantic tokens and dx-style classes; avoid hardcoded color drift.`
- `build/src/style.rs:152:    /// 3. Compiles changed files using the dx-style CLI.`
- `build/src/style.rs:161:    /// - dx-style CLI fails.`
- `build/src/style.rs:219:    const existing = document.querySelector(`link[data-dx-style="${url}"]`);`
- `build/src/style.rs:418:            .arg("dx-style")`
- `build/src/style.rs:438:            .map_err(|e| BuildError::Style(format!("Failed to execute dx-style CLI: {}", e)))?;`
- `build/src/style.rs:443:                "dx-style compilation failed for {:?}: {}",`
- `worker-lanes/WWW_7_AGENT_5_PASS_CLOSEOUT_PROMPT.md:32:- DX-owned source build, App Router compatibility, Forge-first packages, dx-style, receipts, and proof gates are the real target`
- `worker-lanes/WWW_30_AGENT_FINAL_POLISH_PROMPT.md:172:AGENT 20 - dx-style Integration`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:31:- `cargo check -p dx-style --lib -j 1`: passed.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:115:- Group dirty files by domain: dx-www core, dx-style, Forge, build-smoke, template, docs, generated, vendor/reference.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:138:- Separate build/runtime, dx-style, Forge, docs, generated proof.`
- `build/Cargo.toml:41:dx-style = { path = "../related-crates/style" }`
- `CSS.md:43:- For DX Devtools and `dx-style`, the important baseline is not only property count. We need to preserve the full syntax string, the MDN status metadata, and generated value hints for practical controls.`
- `dx-www/tests/diagnostics_cli.rs:113:    .expect("write app route with invalid dx-style class");`
- `dx-www/tests/diagnostics_cli.rs:123:        "build should fail for invalid dx-style class"`
- `dx-www/tests/diagnostics_cli.rs:128:        stderr.contains("DX-WWW error: Unsupported dx-style class"),`
- `dx-www/tests/diagnostics_cli.rs:140:        "invalid dx-style source should not emit route output"`
- `dx-www/tests/diagnostics_cli.rs:160:    .expect("write component with invalid dx-style class");`
- `dx-www/tests/diagnostics_cli.rs:170:        "build should fail for invalid component dx-style class"`
- `dx-www/tests/diagnostics_cli.rs:175:        stderr.contains("DX-WWW error: Unsupported dx-style class"),`
- `dx-www/tests/diagnostics_cli.rs:190:        "invalid component dx-style source should not emit route output"`

### Pattern: `dx-icon`
- `Cargo.lock:1578:name = "dx-icons"`
- `Cargo.lock:1713: "dx-icons",`
- `Cargo.lock:1836: "dx-icons",`
- `Cargo.lock:1879: "dx-icons",`
- `CHANGELOG.md:577:  preview pages no longer expose `www/template`, `dx-icons`, or aggregate`
- `CHANGELOG.md:2609:- Re-centered the generated launch template on the public TSX route path: `app/page.tsx` is now the default `/` dashboard, `app/launch/page.tsx` stays as the `/launch` alias, and the dev runtime has a scoped TSX App Router launch responder that reads `LaunchShell` source markers instead of relying on legacy page materialization for the default proof. A fresh generated app on `http://127.0.0.1:3000/` returned 200 with `data-dx-route="/"`, no literal `{children}`, one form, one canvas, stable `data-dx-icon` markers, and `/favicon.svg` returning 200.`
- `CHANGELOG.md:2617:- Completed the 68/100 urgent launch-readiness fix pass at source and binary level: the source tree fixes the missing `js/openapi/dx-launch.yaml` Forge path mapping, emits the TSX `/launch` route/components before Forge package materialization can fail, serves `/favicon.svg` from `public/favicon.svg`, preserves stable `data-dx-icon` markers through runtime icon rendering, and adds mobile overflow plus canvas pixel-proof guards. `target/debug/dx-www.exe` was rebuilt from the current source, and a generation smoke confirmed `dx-www new --template launch` now writes `app/launch/page.tsx`, `components/launch/launch-shell.tsx`, `openapi/dx-launch.yaml`, and `public/favicon.svg`.`
- `CHANGELOG.md:2690:- Added the Supabase profile workflow to the generated launch dashboard: `supabase/client` now carries aliases/source mirror/provenance/exported-file/env/boundary/receipt/docs metadata, advertises generated `lib/supabase/metadata.ts` discovery, materializes `lib/supabase/profile-workflow.ts` plus `components/launch/supabase-profile-workflow.tsx`, imports real `getDxSupabaseCurrentProfile` and `upsertDxSupabaseProfile` helpers, mounts from the shell's `account-data-dashboard`, uses `<dx-icon name="database:supabase" />`, exposes package-owned editable local profile fields from the typed `dxSupabaseProfileFields` descriptor list, a safe fixture action, missing-config upsert receipts, registers DX Studio editable surfaces and preview package-surface metadata for the account-data section plus profile workflow, records `examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json`, materializes the generated-starter receipt at `.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json`, exposes that receipt through `data-dx-supabase-receipt-path` on the visible Supabase dashboard workflows, and maps the data dashboard's local `profiles` read-model action through `readDxSupabaseProfilesReadModel` as `data-dx-component="supabase-schema-query-workflow"` plus `data-dx-dashboard-workflow="supabase-schema-query"` instead of hiding behind package cards.`
- `CHANGELOG.md:2692:- Added a Drizzle dashboard workflow for the starter: `db/drizzle-sqlite` now materializes a real `dashboard-workflow.ts` helper around the existing Drizzle analytics, join, CTE, and query helpers, exposes `readDrizzleDashboardQueryPlan` with Drizzle `.toSQL()` SQL/params previews, records professional source mirror/provenance/export/receipt metadata, documents the package plus source guard and deferred runtime boundaries in `docs/packages/db-drizzle-sqlite.md`, is consumed by `examples/dashboard` through a visible `<dx-icon name="pack:database" />` readiness panel, and now powers the generated `/launch` data section as a selectable SQLite read-model workflow with query-plan preview, a safe local runtime receipt, mission-control database card status/detail updates, and a source-owned dashboard workflow receipt copied by the runtime materializer.`
- `CHANGELOG.md:2712:- Professionalized `tanstack/query` for the starter dashboard with a visible lifecycle settings workflow over TanStack Query's real `focusManager` and `onlineManager`, theme-token controls, `<dx-icon name="pack:tanstack-query" />`, and Forge metadata for aliases, source mirror, provenance, exported files, required env, app-owned boundaries, and receipt paths.`
- `CHANGELOG.md:2744:- Professionalized `validation/zod` for the starter dashboard with real `dashboard-settings.ts` slices that import Zod v4 and use `strictObject`, `safeParse`, metadata, and `flattenError`, plus visible `LaunchZodDashboardSettings` and `ZodSettingsValidator` workflows using theme tokens, `<dx-icon name="pack:validation-zod" />`, source-mirror/provenance/export/receipt metadata, and a narrow source guard.`
- `CHANGELOG.md:2762:- Added `dx-icon` download-card icons, enlarged the DX mascot eyes while reducing the smile, and changed the landing to an explicit full-height scroll container for local Web Preview/browser reliability. This follow-up is local-only until the user approves the next Vercel deployment.`
- `CHANGELOG.md:2770:- Corrected the public DX-WWW icon/style story: the launch landing uses `<dx-icon name="pack:name" />` and `dx-icons` metadata instead of external icon branding, the Vercel export bridge preserves DX icon metadata with a static fallback, and README/framework docs now lead with React-familiar `.tsx`, dx-style generated CSS, Forge, Check, static export, and no template-local `node_modules`; binary style output is deprecated for now and premature zero-hydration claims stay out of the launch pitch.`
- `CHANGELOG.md:2801:- Added a visible `animation/motion` starter-dashboard workflow with the materialized `js/motion/dashboard-workflow.ts` API, Motion stage selection, reorder preview, local motion policy receipts, source mirror/provenance/export metadata, `<dx-icon name="pack:motion" />`, package docs, and a narrow dashboard guard.`
- `CHANGELOG.md:2822:- Professionalized `api/trpc` for the dashboard starter with source-mirror/provenance/alias/exported-file/receipt metadata, theme-token dashboard workflow styling, `<dx-icon name="api:trpc" />`, and a guarded visible `typed-api-health` workflow around the safe local `launchEvent` action.`
- `CHANGELOG.md:2911:- Corrected the icon lane: removed the standalone `icons/lucide-react` Forge package slice and routed launch icons through DX Icons. App/source code now uses the front-facing `Icon` component or compiler-owned `<icon name="set:name" />` source tag, while `<dx-icon>` remains the lower runtime-compatible tag. The launch proof is `examples/template/icon-status.tsx`, and Lucide remains only an upstream ecosystem dependency where another package legitimately owns it, such as Fumadocs source plugins.`
- `CHANGELOG.md:2915:- Added the `wasm/bindgen` starter-dashboard workflow: `examples/dashboard` now imports `WasmBindgenWorkflow`, shows typed source mirror/provenance/export/receipt/app-owned boundary metadata, records the `launch-runtime-wasm-compute-dashboard` preview surface, updates launch package catalog discoverability, uses `<dx-icon name="pack:wasm-bindgen" />`, and runs a safe local `WebAssembly.instantiate()` add receipt without requiring template-local `node_modules`.`
- `cli/Cargo.lock:1259:name = "dx-icons"`
- `cli/Cargo.lock:1478: "dx-icons",`
- `build/tests/icon_tree_shaking.rs:40:        content.push_str(&format!("    <dx-icon name=/"{}/" />/n", icon_name));`
- `build/tests/icon_tree_shaking.rs:311:            content.push_str(&format!("    <dx-icon name=/"{}/" />/n", icon_name));`
- `core/Cargo.toml:85:dx-icon = { package = "dx-icons", path = "../related-crates/media-icon", default-features = false, features = ["rayon"] }`
- `DX.md:2948:- Browser-readiness scope: the runtime launch CSS adds mobile overflow guards for narrow 390px viewports, the icon materialization path preserves stable `data-dx-icon` and `data-icon-source="dx-icons"` markers even when `<dx-icon>` becomes SVG, and the scene runtime annotates canvas pixel samples with `data-dx-scene-pixel-proof`.`
- `DX.md:2979:- Launch dashboard workflow: `/launch` now uses `LaunchAutomationBridgeStatus` inside `data-dx-component="launch-automation-dashboard-workflow"` with connector filtering, release workflow intent editing, source-file provenance, visible credential schema/auth-kind/credential-type/required-env markers, workflow-node readiness/tool state, `<dx-icon name="pack:n8n" />`, a safe redacted draft receipt handoff to `G:/Dx/.dx/receipts/automations/launch-release-notification.json`, and a visible Zed run-handoff action for `G:/Dx/.dx/receipts/automations/run-latest.json`. The workflow publishes `LaunchAutomationDashboardState` into `examples/template/automation-mission-summary.tsx` so `data-dx-component="launch-automation-mission-summary"` updates the real mission dashboard with selected connector, auth schema, credential types, required env, workflow-node readiness, and `data-dx-automation-dashboard-state`, while `launch-shell.tsx` stays focused on dashboard composition.`
- `DX.md:2981:- Starter dashboard proof: `examples/dashboard/src/lib/n8nAutomationBridge.ts` and `examples/dashboard/src/components/AutomationWorkflowPanel.tsx` consume the same source-owned package shape with `data-dx-package="automations/n8n"`, connector selection, source-file provenance, credential-boundary readiness, `<dx-icon name="pack:workflow" />`, and a safe local redacted receipt action.`
- `DX.md:3004:- Motion dashboard usage: `/launch` now mounts `data-dx-component="launch-motion-dashboard-workflow"` / `data-dx-dashboard-workflow="motion-panel-orchestration"` as a real dashboard interaction and the materialized runtime route updates `data-dx-component="launch-motion-dashboard-summary"`, `#mission-motion-policy`, and `data-dx-dashboard-motion-reduced` from Motion advance/reorder/reset/toggle-reduced-motion actions. `examples/dashboard/src/components/MotionDashboardWorkflow.tsx` still consumes the package through a visible `animated-readiness` workflow backed by the materialized `js/motion/dashboard-workflow.ts` API, `<dx-icon name="pack:motion" />`, stage selection, reorder preview, reduced-motion policy toggle, local policy receipt, source mirror/provenance metadata, package docs, and `benchmarks/motion-dashboard-workflow.test.ts` coverage.`
- `DX.md:3018:- `supabase/client`: Supabase SSR source-owned slice for `createBrowserClient`, `createServerClient`, password auth server actions, public env validation, profile/storage helpers, password recovery, OTP/OAuth helpers, typed `postgres_changes` realtime subscription helpers, professional source/provenance/export/env/receipt/docs metadata, generated `lib/supabase/metadata.ts` discovery, and a minimal `profiles` RLS seed. Add with `dx add supabase/client --write` or the `db/supabase` / `supabase/ssr` / `backend/supabase` aliases. The generated `/launch` dashboard materializes `lib/supabase/profile-workflow.ts` plus `components/launch/supabase-profile-workflow.tsx`, imports `readSupabasePublicConfig`, `getDxSupabaseCurrentProfile`, `upsertDxSupabaseProfile`, `dxSupabaseProfileFields`, `readDxSupabaseProfilesReadModel`, and package-owned profile workflow receipt helpers, mounts the workflow from `data-dx-section="account-data-dashboard"`, renders `<dx-icon name="database:supabase" />`, provides editable local profile fields from the typed descriptor list, an honest missing-config state, a safe fixture action, an upsert receipt interaction, records `examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json`, materializes the generated-starter copy at `.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json`, exposes the same receipt through `data-dx-supabase-receipt-path`, and exposes a `data-dx-component="supabase-schema-query-workflow"` local `profiles` read-model workflow in the data dashboard. DX Studio can source-select `supabase-account-data-dashboard`, `supabase-profile-workflow`, and `supabase-schema-query-workflow` through the launch edit contract and package-surface metadata without absolute positioning or template-local `node_modules`. Applications still own Supabase project provisioning, dependency versions, redirect URLs, SQL rollout, profile/table RLS, provider credentials, realtime policy, and service-role secret handling.`
- `DX.md:3022:- Drizzle dashboard professionalization: the dashboard starter consumes `db/drizzle-sqlite` through `examples/dashboard/src/components/DrizzleDashboardWorkflow.tsx` with a visible `data-dx-drizzle-receipt-path`, typed `readDrizzleDashboardRuntimeReadiness`, and `data-dx-drizzle-runtime-dependencies` for the app-owned `drizzle-orm` plus `better-sqlite3` boundary, and the generated `/launch` route now consumes the same lane through `LaunchDrizzleDashboardData` as a selectable SQLite read-model workflow for launch pipeline, published content, and author workload views. The Forge slice exports `db/drizzle/dashboard-workflow.ts` with `readDrizzleDashboardOverview` over the existing Drizzle analytics, join, CTE, and query helpers plus `readDrizzleDashboardQueryPlan` for safe Drizzle `.toSQL()` SQL/params previews. Metadata records aliases, `G:/WWW/inspirations/drizzle-orm` provenance, exported files, no required env, app-owned runtime/query-plan boundaries, receipt paths, `docs/packages/db-drizzle-sqlite.md` with source guard/deferred runtime boundaries, DX-owned `<dx-icon name="pack:database" />` usage, mission-control database card status/detail updates through stable `data-dx-backend-*` markers, DX Studio/Web Preview selectors for the Drizzle read-model actions, `data-dx-source` and `data-dx-drizzle-receipt-path` source-to-receipt traceability, a CLI/Zed package-surface record that points to the real `LaunchDrizzleDashboardData` workflow, and the source-owned receipt `examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json`.`

### Pattern: `Forge`
- `AGENTS.md:35:- Forge source-owned package lanes and receipts.`
- `AGENTS.md:83:## Forge Receipt Rule`
- `AGENTS.md:87:If Forge goes yellow because of stale hashes, run the appropriate helper:`
- `benchmarks/authentication-lock-backed-template.test.ts:73:    "package-status must count auth/better-auth as a locked Forge package",`
- `benchmarks/authentication-receipt-hash-refresh.test.ts:324:        "} as const satisfies LaunchForgePackageLaneVisibility;",`
- `benchmarks/automations-bridge.test.ts:260:test("automations Forge package exposes a real n8n-shaped public API", () => {`
- `benchmarks/better-auth-dashboard-workflow.test.ts:348:  assert.match(runtimeMaterializer, /function materializeForgeReceipts/);`
- `benchmarks/better-auth-session-helper.test.ts:35:test("auth/better-auth metadata exposes Forge discovery and dashboard receipts", () => {`
- `benchmarks/better-auth-session-helper.test.ts:53:  assert.match(launchStatus, /import /{ dxBetterAuthForgePackage /} from "@//auth//better-auth//metadata";/);`
- `benchmarks/better-auth-session-helper.test.ts:54:  assert.match(launchStatus, /data-dx-auth-package-id=/{dxBetterAuthForgePackage/.packageId/}/);`
- `benchmarks/better-auth-session-helper.test.ts:55:  assert.match(launchStatus, /data-dx-auth-receipt-path=/{dxBetterAuthForgePackage/.receiptPaths/.package/}/);`
- `benchmarks/better-auth-session-helper.test.ts:56:  assert.match(launchStatus, /data-dx-auth-source-mirror=/{dxBetterAuthForgePackage/.sourceMirror/}/);`
- `benchmarks/cli-forge-audit-options-split-safety.test.mjs:24:  assert.match(cliMod, /use self::forge_audit_options::/{/s*parse_forge_audit_options,/s*DxForgeAuditCommandOptions,/s*/};/s);`
- `benchmarks/cli-forge-audit-options-split-safety.test.mjs:38:  assert.match(optionsSource, /pub/(super/) struct DxForgeAuditCommandOptions/);`
- `benchmarks/cli-forge-adoption-options-split-safety.test.mjs:62:    /use self::forge_adoption_options::/{/s*parse_forge_adoption_report_options,/s*parse_forge_adoption_smoke_options,/s*DxForgeAdoptionReportCommandOptions,/s*DxForgeAdoptionSmokeCommandOptions,?/s*/};/s,`
- `benchmarks/cli-forge-adoption-options-split-safety.test.mjs:77:    /pub/(super/) struct DxForgeAdoptionSmokeCommandOptions/,`
- `benchmarks/cli-forge-adoption-options-split-safety.test.mjs:81:    /pub/(super/) struct DxForgeAdoptionReportCommandOptions/,`
- `benchmarks/cli-forge-add-options-split-safety.test.mjs:24:  assert.match(cliMod, /use self::forge_add_options::/{/s*parse_forge_add_options,/s*DxForgeAddCommandOptions,/s*/};/s);`
- `benchmarks/cli-forge-add-options-split-safety.test.mjs:38:  assert.match(optionsSource, /pub/(super/) struct DxForgeAddCommandOptions/);`
- `benchmarks/cli-forge-add-options-split-safety.test.mjs:40:  assert.match(optionsSource, /Forge package id required/);`
- `benchmarks/cli-forge-hosted-registry-smoke-split-safety.test.cjs:20:test("Forge hosted registry smoke reports and renderers live outside cli mod.rs", () => {`
- `benchmarks/cli-forge-hosted-registry-smoke-split-safety.test.cjs:44:    "struct DxForgeHostedRegistrySmokeReport",`
- `benchmarks/cli-forge-hosted-registry-smoke-split-safety.test.cjs:45:    "struct DxForgeHostedRegistrySmokeCheck",`
- `benchmarks/cli-forge-hosted-registry-smoke-split-safety.test.cjs:63:    "pub(super) struct DxForgeHostedRegistrySmokeReport",`
- `benchmarks/cli-forge-hosted-registry-smoke-split-safety.test.cjs:64:    "struct DxForgeHostedRegistrySmokeCheck",`
- `benchmarks/cli-forge-hosted-registry-smoke-split-safety.test.cjs:82:    "DX Forge hosted registry smoke",`
- `benchmarks/cli-forge-hosted-registry-smoke-split-safety.test.cjs:83:    "# DX Forge Hosted Registry Smoke",`
- `benchmarks/cli-forge-launch-page-split-safety.test.cjs:10:test("Forge launch page command wrapper lives outside cli mod.rs", () => {`

### Pattern: `serializer`
- `Cargo.lock:1619: "dx-serializer",`
- `Cargo.lock:1634:name = "dx-serializer"`
- `Cargo.lock:1665: "dx-serializer",`
- `Cargo.lock:1714: "dx-serializer",`
- `Cargo.lock:3968: "dx-serializer",`
- `browser/src/stream_reader.rs:210:                // by the HtipStream deserializer.`
- `auth/src/lib.rs:196:    use serde::{Deserialize, Deserializer, Serialize, Serializer};`
- `auth/src/lib.rs:198:    pub fn serialize<S>(sig: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>`
- `auth/src/lib.rs:202:        BASE64_URL.encode(sig).serialize(serializer)`
- `auth/src/lib.rs:205:    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>`
- `auth/src/lib.rs:207:        D: Deserializer<'de>,`
- `auth/src/lib.rs:209:        let s = String::deserialize(deserializer)?;`
- `benches/htip_benchmarks.rs:8:use dx_www_binary::serializer::HtipWriter;`
- `benches/htip_benchmarks.rs:9:use dx_www_binary::deserializer::HtipStream;`
- `binary/tests/validation_property_tests.rs:11:    DxBinaryError, MAGIC_BYTES, VERSION, deserializer::HtipStream, protocol::HtipHeader,`
- `binary/tests/validation_property_tests.rs:12:    serializer::HtipWriter,`
- `binary/tests/integration.rs:5:use dx_www_binary::{deserializer::HtipStream, opcodes::*, serializer::HtipWriter};`
- `binary/src/serializer.rs:18:/// HTIP writer (server-side serializer)`
- `binary/src/lib.rs:20://!      ├─► serializer.rs                                    │`
- `binary/src/lib.rs:29://!      └─► bincode stream ──────► HTTP/2 ──────► deserializer.rs`
- `binary/src/lib.rs:75://! use dx_www_binary::serializer::HtipWriter;`
- `binary/src/lib.rs:89://! use dx_www_binary::deserializer::HtipStream;`
- `binary/src/lib.rs:103:pub mod deserializer;`
- `binary/src/lib.rs:107:pub mod serializer;`
- `binary/src/lib.rs:112:pub use deserializer::HtipStream;`
- `binary/src/lib.rs:115:pub use serializer::HtipWriter;`
- `cli/src/dev.rs:42:    println!("   Config: dx (serializer format)");`
- `binary/src/htip_bridge.rs:1://! # HTIP Bridge: Deserializer → DOM`

### Pattern: `no node_modules`
- `core/src/ecosystem/project_check.rs:781:            "Position DX-WWW as React-familiar TSX with Forge packages, dx-style CSS, and no node_modules by default.",`
- `tools/next-rust-merge/coordinator-checks.cjs:57:      "no node_modules graph requirement",`
- `benchmarks/drizzle-launch-proof.test.ts:66:  assert.match(drizzleProof, /no node_modules/i);`
- `core/src/ecosystem/forge_scorecard.rs:244:            "The launch package set proves no install scripts, no package lifecycle hooks, and no node_modules creation for these packages.".to_string(),`
- `benchmarks/dx-build-installed-smoke-human-report.test.cjs:251:    /CSS output proof: not eligible /(/.dx//build//styles//app/.css; hash: yes; source map: present; linked: yes; linked in CSS: no; source-map hash: map123; source-map JSON: yes; sources: 0; artifact sources: 1; no node_modules: yes; lifecycle scripts executed: no; source-owned: yes; external runtime required: no; external runtime executed: no/)/,`
- `benchmarks/dx-build-installed-smoke-human-report.test.cjs:255:    /Public asset proof: eligible /(/.dx//build//public//icons//mark-abc123/.svg; hash: yes; hashed filename: yes; public source: yes; public output: yes; source-derived: yes; no node_modules: yes; lifecycle scripts executed: no; source-owned: yes; external runtime required: no; external runtime executed: no; size: 7//7 bytes; size match: yes/)/,`
- `benchmarks/dx-build-installed-smoke-human-report.test.cjs:337:    /- GET //api//health /(app//api//health//route/.ts/): executed; status 200//200; source-owned: yes; no node_modules: yes/,`
- `tools/launch/runtime-template/pages/automations.html:57:          <span class="pill">no node_modules required</span>`
- `dx-www/tests/fixtures/forge-pages/forge-site.html:1:<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>DX-WWW /forge</title></head><body><main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50"><section class="mx-auto grid max-w-5xl gap-6"><div class="grid gap-4 border-b border-neutral-800 pb-7"><p class="text-sm font-medium uppercase text-neutral-400">DX Forge launch evidence</p><h1 class="max-w-3xl text-5xl font-semibold">Source-owned package firewall for selected web code.</h1><p class="max-w-3xl text-lg text-neutral-300"> This compact page is generated from the same release-proof and package-scorecard models used by the DX CLI. </p><div class="flex flex-wrap gap-3"><button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md bg-neutral-950 px-4 py-2 text-sm font-medium text-white shadow-sm"><svg aria-hidden class="size-5" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"><path d="m21 21-4.35-4.35"></svg> Review evidence </button></div></div><section class="grid gap-4 md:grid-cols-3"><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">Release gate</p><h2 class="mt-2 text-2xl font-semibold">Needs review</h2><p class="mt-3 text-sm text-neutral-300">DX check 95 / 100, traffic yellow, launch findings 0.</p><p class="mt-2 text-sm text-neutral-300">Rollback coverage 0%; package docs coverage 100%.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">Package scorecard</p><h2 class="mt-2 text-2xl font-semibold">100 / 100</h2><p class="mt-3 text-sm text-neutral-300">29 verified, 29 source-owned, 0 node_modules packages.</p><p class="mt-2 text-sm text-neutral-300">Generated <timestamp>.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">Benchmark evidence</p><h2 class="mt-2 text-2xl font-semibold">DXPK proof path</h2><p class="mt-3 text-sm text-neutral-300">No benchmark history snapshot is attached yet.</p></article></section><section class="grid gap-4 md:grid-cols-3"><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">shadcn/ui/button</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">4 files, 2920 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves curated UI component materialization, not full shadcn registry parity yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">shadcn/ui/badge</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">4 files, 2701 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This adds a real status/tag primitive for launch templates, not full shadcn registry parity yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">shadcn/ui/card</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">3 files, 2419 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This expands curated UI coverage to a layout component, not the full shadcn registry yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">shadcn/ui/label</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">3 files, 1016 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This adds a real accessible form-label primitive for launch templates, not full shadcn registry parity yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">shadcn/ui/separator</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">3 files, 1409 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This adds a real layout separator primitive for launch templates, not full shadcn registry parity yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">shadcn/ui/field</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">5 files, 7442 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This adds real form layout, description, and error primitives for launch templates, not full shadcn registry parity yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">shadcn/ui/item</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">5 files, 6909 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This adds real list row, media, content, and action primitives for launch templates, not full shadcn registry parity yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">shadcn/ui/input</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">2 files, 664 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This adds a real form primitive for launch templates, not full shadcn registry parity yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">shadcn/ui/textarea</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">2 files, 584 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This expands launch form coverage to long-form text fields, not full shadcn registry parity yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">dx/icon/search</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">3 files, 1516 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves selected asset packaging, not every icon library or tree-shaking scenario yet.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">auth/better-auth</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">21 files, 45631 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves source-owned Better Auth/OAuth launch wiring, not a complete hosted identity platform, account system, organization model, or database policy.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">animation/motion</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">17 files, 64581 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves Motion React launch wiring, not every gesture, layout projection, timeline, or DOM animation API.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">i18n/next-intl</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">40 files, 66135 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves next-intl App Router launch wiring, not complete translation operations, locale SEO policy, or domain routing governance.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">tanstack/query</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">38 files, 119878 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves TanStack Query launch wiring, not a replacement for every observer, devtools, persistence, or offline sync feature.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">validation/zod</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">19 files, 50683 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves Zod validation launch wiring, not a universal schema governance, policy, or data-access authorization layer.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">forms/react-hook-form</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">6 files, 13386 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves React Hook Form launch wiring, not a replacement for app-specific schema design, accessibility review, or authorization policy.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">payments/stripe-js</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">10 files, 69735 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves Stripe.js browser payment wiring, not server PaymentIntent creation, webhooks, pricing, fraud, tax, dispute, or compliance policy.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">automations/n8n</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">6 files, 26371 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves n8n metadata and DX CLI automation bridge wiring, not live workflow execution, credential setup, or canvas parity.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">state/zustand</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">13 files, 43062 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves a Zustand-compatible launch state slice with explicit app-owned Immer dependency boundaries, not every middleware, upstream multi-store DevTools tracking, or durable application storage.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">ai/vercel-ai</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">35 files, 55220 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves Vercel AI SDK route and client wiring, not provider account setup, model safety policy, persistence, or rate limiting.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">api/trpc</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">28 files, 68754 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves tRPC App Router wiring, typed clients, and subscription transport shape, not application authorization, procedure design, request limits, stream fan-out, or persistence policy.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">content/fumadocs-next</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">31 files, 59392 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves Fumadocs docs, source plugin frontmatter, navigation snapshot helpers, toc summary helpers, LLMs route materialization, OpenAPI virtual docs, OpenAPI request proxy source wiring, OpenAPI request code usage, dynamic search, static search-index export, and client preset materialization, not source plugin taxonomy, navigation policy, toc policy, slug/canonical URL policy, OpenAPI schema governance, proxy allowed origins/auth forwarding policy, request code sample policy, AI crawler policy, search UI, multilingual/vector policy, content governance, or automatic merging of existing Next config.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">content/react-markdown</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">13 files, 31679 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves markdown rendering for launch content, not raw HTML trust, plugin governance, moderation, or full MDX/docs infrastructure.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">supabase/client</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">31 files, 125507 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves Supabase SSR client materialization, not deployed RLS correctness, Auth redirect setup, or secret management.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">db/drizzle-sqlite</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">18 files, 46908 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves a SQLite-first Drizzle data slice, not every Drizzle driver, dialect, migration, or database hosting scenario.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">instantdb/react</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">28 files, 58772 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves InstantDB React launch wiring, not dashboard rules, production auth policy, or complete realtime data governance.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">wasm/bindgen</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">6 files, 66170 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves launch-template loading for generated wasm-bindgen modules, not Rust macro or CLI replacement.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">3d/launch-scene</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">19 files, 135524 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves a source-owned Web Preview scene, not a replacement for the Three engine, React Three Fiber renderer, Drei helper catalog, Spline editor, 3D asset pipeline, or GPU QA.</p></article><article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">migration/static-site</p><h2 class="mt-2 text-xl font-semibold">Source-owned and verified</h2><p class="mt-3 text-sm text-neutral-300">4 files, 5493 source bytes, scripts blocked yes.</p><p class="mt-2 text-sm text-neutral-400">This proves a static content migration seed, not full WordPress plugin, theme, CMS, or dynamic-site migration.</p></article></section><section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">Evidence model</p><h2 class="mt-2 text-2xl font-semibold">Detailed package proof loads outside the first route payload</h2><p class="mt-3 text-sm text-neutral-300"> Provenance, advisory, license-review, local-doc, and rollback details are written to <code>forge.evidence.json</code>. </p><p class="mt-2 text-sm text-neutral-400"> The first route keeps only release status, package summaries, benchmark evidence, and launch boundaries. </p></section><section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5"><p class="text-sm font-medium text-neutral-400">Honest launch boundary</p><h2 class="mt-2 text-2xl font-semibold">Not universal npm yet</h2><ul class="mt-4 grid gap-3 text-sm text-neutral-300"><li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">Forge v1 is not a universal npm replacement; it is a source-owned package firewall for curated packages first.</li><li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">The launch package set proves no install scripts, no package lifecycle hooks, and no node_modules creation for these packages.</li><li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">Arbitrary npm, Cargo, pip, and Go package ingestion still needs provenance, advisory metadata, license review, and ecosystem-specific sandboxing.</li></ul></section></section></main></body></html>`
- `dx-www/tests/fixtures/forge-pages/forge-site-source.html:264:                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">The launch package set proves no install scripts, no package lifecycle hooks, and no node_modules creation for these packages.</li>`
- `dx-www/tests/fixtures/forge-markdown/scorecard-public.md:108:- The launch package set proves no install scripts, no package lifecycle hooks, and no node_modules creation for these packages.`
- `dx-www/tests/fixtures/forge-markdown/forge-pages-bundle-shape.md:39:| `readiness badge` | `forge-readiness-badge.json` | `true` | badge reports passed and no node_modules |`
- `dx-www/tests/fixtures/forge-markdown/forge-pages-bundle-shape.md:53:| `publish bundle dependency boundary` | `node_modules` | `true` | no node_modules directory in publish bundle |`
- `dx-www/tests/fixtures/forge-golden/pages-bundle-shape.json:125:      "message": "badge reports passed and no node_modules"`
- `dx-www/tests/fixtures/forge-golden/pages-bundle-shape.json:241:      "message": "no node_modules directory in publish bundle"`
- `tools/build/installed-smoke/route-handler-stale-evidence.ts:55:    details.push(`no node_modules: ${formatYesNo(routeHandlerEvidenceDeclaresNoNodeModules(entry))}`);`
- `benchmarks/installed-smoke-route-handler-stale-evidence.test.ts:303:    /- source-build-manifest: GET //api//stale /(app//api//stale//route/.ts/); source-owned: yes; no node_modules: yes/,`
- `benchmarks/lane7-motion-lock-promotion.test.ts:129:    "forge-owned source slice; no node_modules install or browser animation proof performed",`
- `benchmarks/lane7-3d-lock-promotion.test.ts:135:    "forge-owned source slice; no node_modules install, browser WebGL proof, or screenshot proof performed",`
- `tools/build/installed-smoke/human-report.ts:87:      `no node_modules: ${formatYesNo(item.declaresNoNodeModules)}`,`
- `flow/crates/forge/tests/integration.rs:158:        "forge-owned source slice; no node_modules install performed"`
- `tools/build/installed-smoke/human-report-proof.ts:24:      `no node_modules: ${formatYesNo(styleOutput.declaresNoNodeModules)}; ` +`
- `tools/build/installed-smoke/human-report-proof.ts:38:      `no node_modules: ${formatYesNo(publicAssetOutput.declaresNoNodeModules)}; ` +`
- `benchmarks/measure-forge-source-owned-package-review.ts:111:      "It proves curated package docs, receipts, advisory placeholders, update traffic, local-edit yellow review, rollback, and no node_modules for the example app path.",`
- `benchmarks/measure-forge-source-owned-package-review.ts:237:    no_node_modules: gate("no node_modules", input.noNodeModules, input.noNodeModules ? 1 : 0, 1),`
- `benchmarks/measure-forge-installability-snapshot.ts:42:      "Command is intentionally skipped; this row is not a live shadcn benchmark and creates no node_modules.",`
- `flow/crates/forge/src/packages.rs:420:        boundary: "forge-owned source slice; no node_modules install performed".to_string(),`
- `benchmarks/reports/forge-installability-history/snapshots/2026-05-18T18-45-24-641Z.json:86:      "evidence": "Command is intentionally skipped; this row is not a live shadcn benchmark and creates no node_modules."`

### Pattern: `node_modules`
- `benchmarks/app-router-page-extensions-build-loop.test.mjs:141:  assert.match(appRouterStyleAssets, /node_modules_required": false/);`
- `benchmarks/app-router-server-data-build-contract.test.ts:217:  assert.match(appRouterExecution, /json_attr_value/(server_data/.get/("node_modules_required"/)/)/);`
- `benchmarks/app-router-server-data-build-contract.test.ts:349:  assert.match(serverContract, /node_modules_required: false/);`
- `benchmarks/app-router-server-data-build-contract.test.ts:381:  assert.match(serverDataProofTest, /!root/.join/("node_modules"/)/.exists/(/)/);`
- `benchmarks/authentication-dx-check-package-lane-panel.test.ts:372:    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));`
- `benchmarks/better-auth-dashboard-workflow.test.ts:353:  assert.equal(forgeReceipt.node_modules_policy, "no-template-local-node_modules");`
- `benchmarks/better-auth-live-runtime.test.ts:27:  assert.equal(fs.existsSync(path.join(dir, "node_modules")), false);`
- `CHANGELOG.md:12:  Forge-first/no-node_modules behavior, and Turbopack reference/provenance stay`
- `CHANGELOG.md:39:  Router roots, filters source files without `node_modules`, and computes`
- `CHANGELOG.md:58:  pointing through `node_modules` are reported as`
- `CHANGELOG.md:60:  `node_modules_required=false`. Guarded with red/green`
- `CHANGELOG.md:68:  comments/string literals, avoid `node_modules`, and leave dynamic redirect`
- `CHANGELOG.md:125:  React/RSC, Turbopack, or template-local `node_modules`. Guarded with`
- `CHANGELOG.md:133:  template-local `node_modules` outside the DX runtime path. Guarded with`
- `CHANGELOG.md:154:  template-local `node_modules` execution. Guarded with red/green focused`
- `CHANGELOG.md:179:  without using Next runtime, Turbopack, or template-local `node_modules`.`
- `CHANGELOG.md:198:  without relying on Next runtime, Turbopack, or `node_modules`. Guarded with`
- `CHANGELOG.md:207:  template-local `node_modules` out of the DX core path. Guarded with red/green`
- `CHANGELOG.md:217:  `node_modules` dependency. Guarded with red/green`
- `CHANGELOG.md:266:  external bundler HMR, or template-local `node_modules` path is claimed.`
- `CHANGELOG.md:344:  requiring project-local `node_modules`. Guarded with red/green`
- `CHANGELOG.md:353:  targets without requiring `node_modules` or claiming Turbopack/package-manager`
- `CHANGELOG.md:445:  requiring `node_modules` or claiming TypeScript compiler parity. Guarded with`
- `CHANGELOG.md:485:  `next/tsconfig` without requiring `node_modules`, and preserves root/later`
- `CHANGELOG.md:496:  package installs or `node_modules` resolution. Guarded with red/green`
- `CHANGELOG.md:539:  route invalidation without package installs or `node_modules` resolution.`
- `CHANGELOG.md:659:  over broad fallback aliases, while unsafe `node_modules` export targets are`
- `CHANGELOG.md:820:  count, skipped unsafe-method count, and no-node_modules/lifecycle flags, so`

### Pattern: `client island`
- `CHANGELOG.md:1878:- Added a TSX client island manifest for generic App Router responses. The source render surface now emits `dx.tsx.clientIslandManifest`, the runtime response exposes `data-dx-client-islands`, and the served HTML includes `__DX_TSX_CLIENT_ISLANDS__` beside `__DX_DOM_ACTION_BINDER__`. The manifest maps safe intrinsic button/input/form action targets to event slot IDs, state slots, the generated DOM action binder, and the compiler-owned state runtime without requiring `node_modules`; `dx.tsx.renderPlan` now marks `client_island_manifest` and `generated_dom_action_binder` as runtime readiness surfaces. This improves client island and browser event proof, but it still does not claim arbitrary Client Component execution, React synthetic events, context/effect semantics, or full hydration. Guarded with red/green `dx run --test benchmarks/public-framework-contract.test.ts`, benchmark syntax check, and touched Rust formatting. Full builds, broad suites, local servers, browser automation, deploys, and live browser event proof were skipped. Next action: add live browser proof that a generated client island dispatches through `__DX_STATE_GRAPH_RUNTIME__`.`
- `CHANGELOG.md:2084:- Added bounded TSX class-list and class-call prop lowering. Source-owned component previews now resolve common React `className` expressions built from static arrays, `condition && "class"` entries, safe template prop bindings, `.filter(Boolean).join(" ")`, and bounded `cn(...)` / `clsx(...)` calls with literal, prop-bound, template, logical, and ternary arguments. The renderer still rejects object/array `clsx` payloads, function calls, spreads, hooks, runtime data reads, and arbitrary JavaScript. Guarded with the public framework contract source guard and touched Rust formatting; targeted Cargo did not complete because shared `dx_check_receipt.rs` package-lane work was still moving under parallel launch workers. Next action: add safe event/form binding metadata for source-owned client islands.`
- `CURRENT_STATE.md:235:| Client islands | Artifacts exist. | Not React client islands yet. |`
- `CURRENT_STATE.md:676:6. Add a client island root that can hydrate/attach to server HTML.`
- `DX.md:1954:- G-drive DX TS/TSX runner, 2026-05-22: `dx run` is now the DX-owned front door for `.ts` and `.tsx` scripts/tests. It validates sources through the DX-WWW OXC compiler path, then executes via a G-drive Bun runtime while refusing C-drive Node fallback. The root CLI forwards `dx run ...` into DX-WWW, DX-owned legacy CommonJS guards/helpers were converted to `.ts`, and docs/manifests now point at `dx run --test` / `dx run --check`. `G:/Dx/bin/dx.exe` and `G:/Dx/bin/dx-www.exe` were refreshed from release builds and `G:/Dx/bin` is already on the User PATH. Verification: `dx run --check ./examples/template/app/launch/page.tsx --json`, `dx run --test ./benchmarks/tsx-next-link-compat.test.ts`, `dx run --test ./benchmarks/forge-safety-archive-source-guard.test.ts`, `dx www version`, no legacy CommonJS guard files or direct Node test/check commands in DX-owned sources, `git diff --check`, and conflict-marker scan. Demanding truth: this removes the public Node/CommonJS guard workflow, but full generic React/App Router execution still needs import execution, component evaluation, hooks/effects/context, client islands, and browser event parity.`
- `DX.md:2200:- TSX client island manifest, 2026-05-22: generic App Router responses now publish `dx.tsx.clientIslandManifest` from the source render surface, expose `data-dx-client-islands`, and inject `__DX_TSX_CLIENT_ISLANDS__` beside the generated DOM action binder. The manifest groups safe intrinsic button/input/form action targets, event slot IDs, state slots, `__DX_DOM_ACTION_BINDER__`, and `__DX_STATE_GRAPH_RUNTIME__` into source-owned client-island records with no `node_modules` requirement. `dx.tsx.renderPlan` now marks `client_island_manifest` and `generated_dom_action_binder` in runtime readiness. This improves client island and browser event proof without claiming arbitrary Client Component execution, React synthetic events, context/effect semantics, or full hydration. Verification scope: red/green `dx run --test benchmarks/public-framework-contract.test.ts`, syntax check, and rustfmt on touched App Router files. Heavy/runtime checks skipped: local servers, browser automation, deploys, full builds, broad suites, and live event-click proof. Exact next action: add live browser proof that a generated client island dispatches through `__DX_STATE_GRAPH_RUNTIME__`.`
- `DX.md:2392:- TSX class-list and class-call prop lowering, 2026-05-22: the source-owned App Router renderer now lowers bounded React class-list expressions such as `["button", active && "is-active", variant === "primary" && "button-primary"].filter(Boolean).join(" ")` plus `cn("button", active && "is-active")` / `clsx("button", variant === "primary" ? "is-primary" : "is-secondary")` when every item resolves from literals, caller prop bindings, template bindings, or static conditions. This improves normal shadcn-style `className` rendering for source-owned components without evaluating object/array `clsx` payloads, function calls, spreads, hooks, runtime data, or arbitrary JavaScript. Guarded with the public framework contract source guard; targeted Cargo remains blocked by unrelated shared package-lane churn while other workers are editing `dx_check_receipt.rs`. Exact next action: lower the next common TSX subset, safe event/form binding metadata for source-owned client islands, behind the same no-arbitrary-execution boundary.`
- `docs/root-workspace-todo.md:846:- A good Next.js App Router interactive page with a small client island.`
- `docs/root-workspace-status.md:22:- `next_app_router_conformance_report()` now compiles strict no-`node_modules` App Router fixtures for static pages, client islands, form/server actions, dynamic routes, and metadata/image/font-like surfaces.`
- `core/src/delivery/tests.rs:6473:            .contains("www client islands")`
- `core/src/delivery/tests.rs:6638:    let island = manifest.islands.first().expect("client island");`
- `core/src/delivery/contract.rs:101:    /// State belongs to one component/client island by default.`
- `core/src/delivery/client_island.rs:223:        "/* www client islands v{} route={} *//n{}",`
- `core/src/delivery/client_island.rs:815:        "/* www client islands v1 route={} *//n(() => {{/n  const islands = document.querySelectorAll('[data-dx-island]');/n",`
- `core/src/delivery/app_route.rs:167:/// One client island that can resume after the shell flush.`
- `docs/NEXTJS_COMPATIBILITY_MAP.md:24:| server/client components | React-familiar TSX with compiler-owned client island lowering | Adapter boundary | Keep `"use client"` support honest. Full React Server Components runtime is not copied. |`
- `docs/NEXTJS_COMPATIBILITY_MAP.md:41:- React rendering, layout/page composition, client islands, metadata, route handlers, server actions, image/font/script handling, middleware, static export, and cache/revalidate.`
- `docs/DX_WWW_MANAGER_HANDOFF.md:36:The biggest framework unlock is still full generic TSX/App Router rendering: layout/page composition, imports, props, common hooks, client islands, and route handlers as first-class behavior. Until that is real, DX-WWW is promising but not production-complete as a DX-owned WWW framework.`
- `docs/DX_WWW_MANAGER_HANDOFF.md:76:This is an honest middle step: www can now explain the state and interaction meaning of a route without claiming full React hook execution. The next renderer task is to lower this state/event graph into generated JS client islands for the common React subset.`
- `docs/DX_WWW_MANAGER_HANDOFF.md:246:This is a stronger common-subset bridge, not a fake Next.js replacement claim. It makes local component usage visible and partially materialized for DX Studio/check without running arbitrary JavaScript. Remaining TSX engine work is ordered child graphs, real safe component function execution for a narrow subset, visible response-DOM attachment, client island lowering, effects/context/reducer boundaries, and browser runtime proof.`
- `dx-www/src/cli/app_router_semantics.rs:118:            "client island bundling",`
- `dx-www/src/cli/app_router_semantics.rs:479:            "next_step": "lower state/event graph into generated JS client islands"`
- `dx-www/src/cli/app_router_execution/state_runtime.rs:59:            "bundle client islands with full source imports"`
- `dx-www/src/cli/app_router_execution/source_render.rs:212:            "Bind generated state/event runtime operations to the real rendered DOM for client islands.",`
- `dx-www/src/cli/app_router_execution/source_render.rs:1837:            "Publishes safe source-owned client island targets for generated DOM action binding.",`
- `benchmarks/fair-counter/measure-fair-counter.ts:463:- Next.js ships much more runtime JavaScript for this tiny client island, even after removing Tailwind, fonts, images, and extra demo content.`
- `benchmarks/fair-counter/measure-fair-counter.ts:490:        model: "App Router + React client island",`
- `dx-www/src/cli/forge_react_starter_benchmark.rs:266:        || client_islands_runtime.contains("www client islands")`

### Pattern: `server data`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:157:- Clean warning causes in metadata, request props, render plan, source render, server data.`
- `core/src/delivery/server_contract.rs:287:/// Route-local server data manifest emitted for async server component pages.`
- `core/src/delivery/tests.rs:6298:    .expect("server data manifest");`
- `dx-www/tests/source_build_server_data.rs:96:        .expect("manifest server data routes");`
- `dx-www/tests/source_build_server_data.rs:154:        .expect("dynamic docs server data output");`
- `dx-www/tests/source_build_server_data.rs:162:        .expect("literal docs server data output");`
- `core/src/ecosystem/forge_zustand.rs:1382:Keep server data, authorization, and durable storage in application code. Use this slice for browser-local launch state and reviewed template defaults.`
- `dx-www/src/cli/next_familiar_fixtures.rs:531:                "cache_control": "no-store for server data, immutable packet assets",`
- `benchmarks/next-rust-merge-coordinator.test.cjs:629:          nextAction: "write source-owned server data",`

### Pattern: `diagnostic`
- `Cargo.lock:4814:name = "oxc_diagnostics"`
- `Cargo.lock:4826:name = "oxc_diagnostics"`
- `Cargo.lock:4880: "oxc_diagnostics 0.22.1",`
- `Cargo.lock:4901: "oxc_diagnostics 0.56.5",`
- `Cargo.lock:4918: "oxc_diagnostics 0.56.5",`
- `benchmarks/app-router-build-route-discovery-output.test.ts:12:test("App Router build emits route discovery and skipped-route diagnostics", () => {`
- `benchmarks/app-router-build-route-discovery-output.test.ts:15:  const diagnostics = read("dx-www/src/cli/app_page_route_diagnostics.rs");`
- `benchmarks/app-router-build-route-discovery-output.test.ts:31:    /let skipped_route_summaries =/s+app_page_route_diagnostics::discover_skipped_page_route_summaries/(input/.cwd/);/,`
- `benchmarks/app-router-build-route-discovery-output.test.ts:51:  assert.match(diagnostics, /pub/(super/) fn discover_skipped_page_route_summaries/);`
- `benchmarks/app-router-invalid-segment-diagnostics.test.ts:12:test("App Router invalid page-route segments carry explicit discovery diagnostics", () => {`
- `benchmarks/app-router-invalid-segment-diagnostics.test.ts:15:  const cliRouteDiagnostics = read("dx-www/src/cli/app_page_route_diagnostics.rs");`
- `benchmarks/app-router-invalid-segment-diagnostics.test.ts:26:  assert.match(cliMod, /mod app_page_route_diagnostics;/);`
- `benchmarks/app-router-invalid-segment-diagnostics.test.ts:32:  assert.match(cliRouteDiagnostics, /discover_page_routes_reports_skipped_route_diagnostics/);`
- `benchmarks/app-router-invalid-segment-diagnostics.test.ts:40:  assert.match(discovery, /source_discovery_reports_skipped_unsupported_app_page_route_diagnostics/);`
- `benchmarks/app-router-server-data-build-contract.test.ts:108:  assert.match(appRouterBuildCommand, /app_route_diagnostics::validate_app_route_handlers/);`
- `benchmarks/app-router-server-data-build-contract.test.ts:109:  assert.match(appRouterBuildCommand, /app_route_diagnostics::validate_app_route_source/);`
- `benchmarks/app-router-server-data-build-contract.test.ts:110:  assert.match(appRouterBuildCommand, /app_route_diagnostics::app_route_compile_error/);`
- `PLAN.md:63:- [ ] Keep a separate full catalog in `.dx/imports/import-map.json` for diagnostics.`
- `PLAN.md:78:- [ ] Expose alias resolution in route/source-render diagnostics.`
- `PLAN.md:79:- [ ] Fail with a precise diagnostic if an alias points to a stale or missing generated artifact.`
- `PLAN.md:93:- [ ] Keep all stale diagnostics source-owned, exact, and machine-readable.`
- `worker-lanes/WWW_7_AGENT_5_PASS_CLOSEOUT_PROMPT.md:40:- Forge adoption/release threshold checks are green through the strict release-gate score, while broad diagnostic scores remain visible`
- `worker-lanes/WWW_7_AGENT_5_PASS_CLOSEOUT_PROMPT.md:82:- Preserve the strict release-gate score and keep broader diagnostic scores visible.`
- `worker-lanes/WWW_30_AGENT_FINAL_POLISH_PROMPT.md:57:- Harden the lane: edge cases, stale source guards, `.ts` test standard, no-node boundaries, diagnostics, and artifact truth.`
- `worker-lanes/WWW_30_AGENT_FINAL_POLISH_PROMPT.md:169:- Verify diagnostics/code-frame/unsupported class errors.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:69:- Harden the lane: edge cases, diagnostics, stale artifact behavior, platform behavior, and maintainability.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:172:Goal: reduce warnings in dev server, hot reload, diagnostics, overlay code.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:232:- focused diagnostics/browser proof.`

### Pattern: `Lighthouse`
- `CHANGELOG.md:2645:- Added a Studio/Web Preview dx-check panel contract. The preview manifest now publishes a `check_panel` field for `dx check --latest-receipt --json`, the launch shell and runtime-safe `/launch.html` page expose stable `data-dx-check-*` markers, and the edit/materialization contracts map the health panel back to source while keeping Lighthouse, CDP, E2E, lint, format, package install, and full-build execution opt-in.`
- `CHANGELOG.md:2684:- Added web-performance receipt fields for Core Web Vitals, request count, transfer size, and a governed Rust CDP attach plan. URL mode now writes `.dx/receipts/check/web-perf/cdp-plan.json` with device profiles, CDP domains/sequence, metric sources, score model, and blocked-until conditions while keeping measured scores behind explicit runtime governance or Lighthouse JSON import.`
- `CHANGELOG.md:2685:- Hardened Lighthouse JSON import receipts for `dx check web-perf`: incomplete reports now expose `score_completeness`, missing categories, and a partial measurement status instead of claiming a full 400-point total from partial category evidence.`
- `CHANGELOG.md:2771:- Added launch-worker direction for a Rust-owned `dx check web-perf` lane that can record Lighthouse-compatible category scores, Core Web Vitals, request/transfer counts, and total 400-point status without depending on random npm packages.`
- `demo/WHY_DX_WWW_IS_GAME_CHANGING.md:333:- Lighthouse: 85/100`
- `demo/WHY_DX_WWW_IS_GAME_CHANGING.md:341:- Lighthouse: 100/100/100/100`
- `demo/WHY_DX_WWW_IS_GAME_CHANGING.md:349:- Lighthouse: 100/100/100/100`
- `demo/WHY_DX_WWW_IS_GAME_CHANGING.md:357:- **Perfect Lighthouse scores**`
- `demo/WHY_DX_WWW_IS_GAME_CHANGING.md:469:- **Perfect** Lighthouse scores`
- `docs/superpowers/plans/2026-05-26-dx-www-honest-web-perf-scoring.md:5:**Goal:** Prevent DX-WWW orchestration and serializer artifacts from awarding or implying measured Lighthouse proof when web performance evidence is partial, source-only, or audit-only.`
- `docs/superpowers/plans/2026-05-26-dx-www-honest-web-perf-scoring.md:7:**Architecture:** Keep this slice in DX-WWW because the faulty aggregate scoring is in the DX-WWW extension-list orchestrator, not the DX Check engine crate. Preserve existing JSON receipts and command behavior, but make score projection stricter and add explicit serializer-native estimated/proof fields. Do not run Lighthouse, start servers, or alter the DX CLI launch receipt pipeline in this slice.`
- `docs/superpowers/plans/2026-05-26-dx-www-honest-web-perf-scoring.md:58:            "findings": ["missing Lighthouse proof"],`
- `docs/superpowers/plans/2026-05-26-dx-www-honest-web-perf-scoring.md:166:### Task 3: Keep Doctor and Lighthouse Device Proof Honest`
- `docs/superpowers/plans/2026-05-26-dx-www-honest-web-perf-scoring.md:173:Add focused tests proving URL-only receipts score `0`, `web_perf_report_measured` requires `score_completeness.complete=true` plus a numeric total, and a single Lighthouse import cannot be labeled as `--device both`.`
- `docs/superpowers/plans/2026-05-26-dx-www-honest-web-perf-scoring.md:189:- [ ] **Step 4: Guard single Lighthouse proof from both-device claims**`
- `demo/demo_full.html:217:                <span class="vs-label">Lighthouse</span>`
- `demo/PRODUCTION_READINESS_ANALYSIS.md`
- `demo/server.rs:52:    println!("Optimized for 100/100/100/100 Lighthouse scores");`
- `demo/index_enhanced.html:6:    <meta name="description" content="DX-WWW Framework - Binary-first web framework. 1.4KB WASM, Binary CSS, 100/100/100/100 Lighthouse.">`
- `demo/index_enhanced.html:159:                                <strong>Lighthouse:</strong> ${this.state.performance.lighthouse_performance}/100/100/100<br>`
- `core/src/ecosystem/dx_check_receipt.rs:14651:        "message": "Lighthouse was skipped by default.",`
- `core/src/ecosystem/dx_check_receipt.rs:14652:        "next_action": "Run an approved Lighthouse adapter."`
- `examples/onboard/template-shell.tsx:2446:        Lighthouse, CDP, E2E, and other expensive checks stay skipped until the`
- `DX.md:2808:- Public source hygiene now has `dx imports sync|check` for readable generated import maps, and `dx check web-perf` for Rust-owned web-performance receipts with Core Web Vitals, request/transfer evidence, and an attach-only Chrome DevTools Protocol collector plan. URL mode now writes `.dx/receipts/check/web-perf/cdp-plan.json` with device profiles, CDP domains/sequence, metric sources, score model, and explicit blocked-until conditions; exact Lighthouse parity can be imported through `--from-lighthouse report.json`, incomplete Lighthouse imports expose missing score categories instead of claiming a 400-point total, and URL/CDP mode still must not claim measured scores until governed runtime collection runs.`
- `DX.md:2814:- DX-WWW can now consume the root `dx-check` command-center receipt directly: `read_dx_check_latest_panel` and `dx check --latest-receipt --json` read `.dx/receipts/check/check-latest.json`, validate the embedded `dx.check.zed_panel` payload, and return a compact `dx.www.check_panel` state for score meters, bucket breakdowns, blockers, warnings, quick fixes, and last-run metadata. The panel preserves `weight_profile: "dx-check.launch-default"` plus per-bucket weights so Web Preview and future Zed GPUI can render the 500-point model without guessing. It also carries `scoring_config` from `dx.check.scoring_config`, so panels can show default, detected-but-not-applied, or invalid future weight-config states without changing the active score. This is a read bridge only; it does not run Lighthouse, CDP, lint, format, or E2E execution by itself.`

### Pattern: `web-perf`
- `CHANGELOG.md:2628:- Added a framework risk register to `dx doctor`. The doctor receipt now emits `dx.framework.riskRegister`, `score_ceiling`, and explicit risks for TSX runtime parity, the oversized CLI module, Forge package overclaiming, static export proof, web-performance proof, Studio manifests, public-story crowding, and source-copying/legal policy so DX-WWW cannot report a fake 100/100 while known launch risks remain.`
- `CHANGELOG.md:2681:- Added public launch CLI surfaces for `dx style build|watch|check`, `dx imports sync|check`, `dx check web-perf`, and `dx deploy vercel`. The new Rust-owned command paths write dx-style CSS/import/deploy/web-performance receipts and keep URL performance collection/deploy execution honest until governed runtime approval.`
- `CHANGELOG.md:2684:- Added web-performance receipt fields for Core Web Vitals, request count, transfer size, and a governed Rust CDP attach plan. URL mode now writes `.dx/receipts/check/web-perf/cdp-plan.json` with device profiles, CDP domains/sequence, metric sources, score model, and blocked-until conditions while keeping measured scores behind explicit runtime governance or Lighthouse JSON import.`
- `CHANGELOG.md:2685:- Hardened Lighthouse JSON import receipts for `dx check web-perf`: incomplete reports now expose `score_completeness`, missing categories, and a partial measurement status instead of claiming a full 400-point total from partial category evidence.`
- `CHANGELOG.md:2771:- Added launch-worker direction for a Rust-owned `dx check web-perf` lane that can record Lighthouse-compatible category scores, Core Web Vitals, request/transfer counts, and total 400-point status without depending on random npm packages.`
- `benchmarks/cli-help-text-split-safety.test.ts:119:    "dx check web-perf: Measure WWW performance evidence",`
- `docs/DX_WWW_MANAGER_HANDOFF.md:44:5. One-command launch doctor: `dx doctor` checks TSX routes, dx-style, imports, packages, static output, web-perf receipts, and Studio manifest readiness.`
- `docs/DX_WWW_MANAGER_HANDOFF.md:62:`dx doctor` now includes `framework_risks` with schema `dx.framework.riskRegister`. This prevents the tool from casually reporting a fake 100/100 while known framework risks remain, including TSX runtime gaps, the oversized CLI module, Forge package overclaim risk, static export proof, missing web-performance receipts, Studio manifest gaps, public-story crowding, and source-copying/legal risk.`
- `docs/superpowers/plans/2026-05-26-dx-www-honest-web-perf-scoring.md:59:            "diagnostics": [{ "id": "web-perf-proof-missing" }]`
- `TODO.md:2789:- [x] Public `.tsx` framework contract surfaced in CLI/docs: `dx style build|watch|check`, `dx imports sync|check`, `dx check web-perf`, and `dx deploy vercel` now have source-owned command paths and receipt/manifest contracts.`
- `TODO.md:2955:- [x] Add source-owned web-performance receipt evidence fields and governed CDP plan output for Core Web Vitals, request count, transfer size, attach-only browser policy, device profiles, CDP collection sequence, metric sources, and score model while live URL measurement stays governed.`
- `TODO.md:2961:- [ ] Finish Rust CDP live collection for `dx check web-perf --url ...` so URL mode can produce measured Performance, Accessibility, Best Practices, SEO, and total scores without Lighthouse JSON import.`
- `TODO.md:2988:- [x] Added the launch-worker direction for a Rust-owned `dx check web-perf` lane that records Lighthouse-compatible Performance, Accessibility, Best Practices, SEO, and total 400-point status without depending on random npm packages.`
- `core/src/ecosystem/dx_check_receipt.rs:7317:            bucket_weight("web-performance", "Web Performance"),`
- `core/src/ecosystem/dx_check_receipt.rs:7384:        assert_eq!(report.view_model.bucket_rows[0].id, "web-performance");`
- `core/src/ecosystem/dx_check_receipt.rs:7392:            Some("dx check web-perf --url <url> --json")`
- `core/src/ecosystem/dx_check_receipt.rs:7419:        assert_eq!(zed.sections[0].id, "web-performance");`
- `core/src/ecosystem/dx_check_receipt.rs:7424:            Some("dx check web-perf --url <url> --json")`
- `core/src/ecosystem/dx_check_receipt.rs:13257:            Some("dx check web-perf --url http://localhost:3000 --json")`
- `core/src/ecosystem/dx_check_receipt.rs:14625:        { "id": "web-performance", "label": "Web Performance", "weight": 100 },`
- `core/src/ecosystem/dx_check_receipt.rs:14660:        "command": "dx check web-perf --url <url> --json"`
- `core/src/ecosystem/dx_check_receipt.rs:14665:        "id": "web-performance",`
- `examples/onboard/framework-completeness.ts:203:    ["core/src/ecosystem/project_check.rs", ".dx/receipts/check/web-perf/report.json"],`
- `examples/onboard/framework-completeness.ts:204:    ["dx check", "web-perf", "project-contract"],`
- `DX.md:2808:- Public source hygiene now has `dx imports sync|check` for readable generated import maps, and `dx check web-perf` for Rust-owned web-performance receipts with Core Web Vitals, request/transfer evidence, and an attach-only Chrome DevTools Protocol collector plan. URL mode now writes `.dx/receipts/check/web-perf/cdp-plan.json` with device profiles, CDP domains/sequence, metric sources, score model, and explicit blocked-until conditions; exact Lighthouse parity can be imported through `--from-lighthouse report.json`, incomplete Lighthouse imports expose missing score categories instead of claiming a 400-point total, and URL/CDP mode still must not claim measured scores until governed runtime collection runs.`
- `DX.md:2879:- Risk-register guard: `dx doctor` now emits `framework_risks` with schema `dx.framework.riskRegister` and a `score_ceiling`. Known risks such as incomplete generic TSX runtime parity, the oversized CLI module, Forge package overclaiming, missing static/export/web-performance proof, missing Studio manifests, public-story crowding, and source-copying/legal risk now cap the doctor score instead of allowing a fake 100/100.`
- `benchmarks/web-perf-receipt-mode-contract.test.ts:9:test("web-perf receipts are split between dev and static-build modes", () => {`
- `benchmarks/web-perf-receipt-mode-contract.test.ts:17:    ".dx/receipts/check/web-perf/dev/report.json",`

### Pattern: `accessibility`
- `a11y/tests/property_tests.rs:3://! These tests verify the correctness of accessibility rule detection`
- `a11y/src/lib.rs:3://! Catch accessibility issues at compile-time, not in production.`
- `a11y/src/lib.rs:68:/// AST analyzer for accessibility`
- `a11y/README.md:4:Compile-time accessibility auditor for the dx-www framework.`
- `a11y/README.md:8:This crate provides compile-time accessibility checking using OXC parser to analyze JSX/TSX components and ensure they meet accessibility standards.`
- `DX.md:1992:- Forms Studio source-guard/runbook entry, 2026-05-22: lane 6 now publishes the lower helper freshness guard `forms-package-metrics-helper-freshness-path-arrays` beside `forms-generated-starter-materialization` in `dx-www/src/cli/studio_manifest.rs` for official **Forms** (`forms/react-hook-form`). Studio/Zed can now discover the exact source-only command `cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib`, the package-owned fixture path `docs/packages/forms.source-guard-runbook.json`, and the lower dx-check proof that `forms_receipt_hash_refresh_current`, `forms_receipt_hash_refresh_stale`, and `forms_receipt_hash_refresh_missing` come from helper path arrays while `forms_hash_mismatch` stays byte-derived. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form/package.json`, `src/index.ts`, `src/useForm.ts`, `src/useFormContext.tsx`, `src/controller.tsx`, `src/useFieldArray.ts`, `src/types/form.ts`, `src/types/errors.ts`, and `src/types/resolvers.ts`; public APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, `FieldErrors`, and `UseFormReturn`. Files changed: `dx-www/src/cli/studio_manifest.rs`, `benchmarks/forms-dx-check-package-lane-panel.test.ts`, `docs/packages/forms-react-hook-form.md`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`, plus refreshed Forms receipt/package-status/read-model hashes after the Studio manifest and docs changed. Verification scope: red/green Forms package-lane Studio guard, Forms helper `--write`, helper `--check --json`, focused Forms receipt/read-model/docs/output guards, TS syntax checks, JSON parse, scoped diff check, and conflict-marker scan. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, live browser submission proof, persistence, spam protection, accessibility QA, Cargo compilation, Rust manifest execution, and native Zed rendering. Exact next action: expose the Forms Studio manifest source-guard ids and helper freshness fixture paths in a generated `public/preview-manifest.json` fixture snapshot so Zed can diff route-level source guards without executing Rust.`
- `DX.md:2006:- Forms lower dx-check receipt surface, 2026-05-22: lane 6 now hash-backs `core/src/ecosystem/project_check/forms_dx_check.rs` as the selected `forms-lower-dx-check` surface for official **Forms** (`forms/react-hook-form`). The Forms dashboard workflow receipt, `.dx/forge/package-status.json`, typed read model, source-guard runbook fixture, and static `/launch` package-lane markers now agree on 7 tracked helper files; `forms:receipt-hash-refresh` reports the lower dx-check module in exact `tracked_files` / `current_files` arrays so helper metric drift becomes stale-detectable without claiming browser submission proof. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form/package.json`, `src/index.ts`, `src/useForm.ts`, `src/useFormContext.tsx`, `src/controller.tsx`, `src/useFieldArray.ts`, `src/types/form.ts`, `src/types/errors.ts`, and `src/types/resolvers.ts`; public APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, `FieldErrors`, and `UseFormReturn`. Files changed: `benchmarks/forms-receipt-hash-refresh.test.ts`, `benchmarks/forms-dx-check-package-lane-panel.test.ts`, the Forms dashboard workflow receipt, `.dx/forge/package-status.json`, `forge-package-status-read-model.ts`, `examples/template/launch-shell.tsx`, `examples/template/runtime-pages/index.html`, `docs/packages/forms-react-hook-form.md`, `docs/packages/forms.source-guard-runbook.json`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Forms receipt-helper and package-lane guards, Forms helper `--write`, helper `--check --json`, focused read-model/package-doc/output guards, TS syntax checks, Forms JSON parse, scoped diff check, and conflict-marker scan. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, live browser submission proof, persistence, spam protection, accessibility QA, Cargo compilation, Rust manifest execution, and native Zed rendering. Exact next action: publish `forms-package-metrics-helper-freshness-path-arrays` through `dx-www/src/cli/studio_manifest.rs` source-guard/runbook metadata so Zed/DX Studio can rerun the lower dx-check helper freshness proof without opening the JSON fixture.`
- `DX.md:2014:- Forms lower dx-check helper freshness metrics, 2026-05-22: lane 6 now promotes official **Forms** (`forms/react-hook-form`) helper freshness into `core/src/ecosystem/project_check/forms_dx_check.rs` instead of leaving it only in the Studio/check-panel read model. The lower Forge `dx check` section now emits `forms_receipt_hash_refresh_current`, `forms_receipt_hash_refresh_stale`, and `forms_receipt_hash_refresh_missing` from the package-status `receipt_hash_refresh` payload, treats non-empty `stale_files` / `stale_mirror_files` as stale and `missing_files` / `missing_mirror_files` as missing, and keeps `forms_hash_mismatch` byte-derived from selected Forms source hashes. The package-owned fixture `forms_package_metrics_reports_helper_freshness_from_path_arrays` proves stale helper path arrays can name `docs/packages/forms-react-hook-form.md` or the read-model mirror while `forms_hash_mismatch` stays clean. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form/package.json`, `src/index.ts`, `src/useForm.ts`, `src/useFormContext.tsx`, `src/controller.tsx`, `src/useFieldArray.ts`, `src/types/form.ts`, `src/types/errors.ts`, and `src/types/resolvers.ts`; public APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, `FieldErrors`, and `UseFormReturn`. Files changed: `core/src/ecosystem/project_check/forms_dx_check.rs`, `benchmarks/forms-dx-check-output.test.ts`, `docs/packages/forms-react-hook-form.md`, `docs/packages/forms.source-guard-runbook.json`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, refreshed Forms receipt/package-status/read-model mirrors, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green `cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib`, targeted Forms Node guards, Forms helper `--write`, helper `--check --json`, syntax/JSON checks, Rust formatting, and scoped hygiene checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, live browser submission proof, persistence, spam protection, accessibility QA, and native Zed rendering. Exact next action: hash-back `core/src/ecosystem/project_check/forms_dx_check.rs` in the Forms receipt helper as a selected lower dx-check surface so helper metric drift becomes stale-detectable through `forms:receipt-hash-refresh`.`
- `DX.md:2022:- Forms static helper path-list markers, 2026-05-22: lane 6 now exposes official **Forms** (`forms/react-hook-form`) stale/missing helper path-list markers before a fresh dx-check receipt exists. `examples/template/launch-shell.tsx` now pins the static Forms template to 6 tracked files and emits empty `hash_refresh_stale_file_list` / `hash_refresh_missing_file_list`; `examples/template/runtime-pages/index.html` mirrors `data-dx-check-package-lane-hash-refresh-stale-file-list=""` and `data-dx-check-package-lane-hash-refresh-missing-file-list=""`; and `docs/packages/forms.source-guard-runbook.json` advertises both markers for Zed/DX Studio consumers. The generated-starter guard proves the same markers survive `tools/launch/materialize-www-template.ts` into generated `pages/index.html` without claiming browser submission proof. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form`: `package.json`, `src/index.ts`, `src/useForm.ts`, `src/useFormContext.tsx`, `src/controller.tsx`, `src/useFieldArray.ts`, `src/types/form.ts`, `src/types/errors.ts`, and `src/types/resolvers.ts`; public APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, `FieldErrors`, and `UseFormReturn`. Files changed: `benchmarks/forms-dx-check-package-lane-panel.test.ts`, `examples/template/launch-shell.tsx`, `examples/template/runtime-pages/index.html`, `docs/packages/forms-react-hook-form.md`, `docs/packages/forms.source-guard-runbook.json`, refreshed Forms receipt/package-status/read-model mirrors, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green `dx run --test ./benchmarks/forms-dx-check-package-lane-panel.test.ts`, Forms helper `--write`, helper `--check --json`, focused Forms Node guard bundle, TS syntax checks, and Forms JSON parse checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, Rust manifest execution, live browser submission proof, persistence, spam protection, accessibility QA, and native Zed rendering. Exact next action: promote Forms helper freshness metrics into `core/src/ecosystem/project_check/forms_dx_check.rs` so lower `dx check` output and the Studio check-panel use the same `receipt_hash_refresh` path-array evidence.`
- `DX.md:2032:- Forms Rust helper path-array metrics, 2026-05-22: lane 6 now treats official **Forms** (`forms/react-hook-form`) receipt helper path arrays as authoritative dx-check/check-panel freshness evidence. `core/src/ecosystem/dx_check_receipt.rs` now counts non-empty `receipt_hash_refresh.stale_files` or `stale_mirror_files` as stale and non-empty `missing_files` or `missing_mirror_files` as missing, so stale path attribution cannot be hidden by stale numeric count mirrors. The Forms check-panel fixture now proves a stale-helper-only row names `docs/packages/forms-react-hook-form.md` while `forms_hash_mismatch` stays 0, and the package docs/runbook record the path attribution policy. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form`: `package.json`, `src/index.ts`, `src/useForm.ts`, `src/useFormContext.tsx`, `src/controller.tsx`, `src/useFieldArray.ts`, `src/types/form.ts`, `src/types/errors.ts`, and `src/types/resolvers.ts`; public APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, `FieldErrors`, and `UseFormReturn`. Files changed: `core/src/ecosystem/dx_check_receipt.rs`, `benchmarks/forms-dx-check-package-lane-panel.test.ts`, `docs/packages/forms-react-hook-form.md`, `docs/packages/forms.source-guard-runbook.json`, refreshed Forms receipt/package-status/read-model mirrors, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green `dx run --test ./benchmarks/forms-dx-check-package-lane-panel.test.ts`, Forms helper `--write`, helper `--check --json`, focused Forms Node guard bundle, TS syntax checks, JSON parse checks, `rustfmt --edition 2024 --check core/src/ecosystem/dx_check_receipt.rs`, and targeted `cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_forms_package_lane_hash_row --lib` with pre-existing unrelated warnings. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, live browser submission proof, persistence, spam protection, accessibility QA, and native Zed rendering. Exact next action: add static `/launch` Forms stale/missing helper path-list markers so Studio can show exact helper drift before a fresh dx-check receipt exists.`
- `DX.md:2062:- Forms receipt helper path attribution, 2026-05-22: lane 6 now reports exact freshness paths for official **Forms** (`forms/react-hook-form`). `examples/template/forms-receipt-hashes.ts --check --json` now emits `tracked_files`, `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, and `missing_mirror_files`, and mirrors those fields into `.dx/forge/package-status.json`, `examples/template/forge-package-status-read-model.ts`, and the package-owned source-guard runbook fixture so Zed/DX Studio can show the affected source or mirror path without parsing raw helper output. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form`: package metadata, public exports, `useForm`, context provider, controller, field array, and resolver/form types; public APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, and `FieldErrors`. Files changed: `examples/template/forms-receipt-hashes.ts`, `benchmarks/forms-receipt-hash-refresh.test.ts`, `benchmarks/forms-package-status-read-model.test.ts`, `benchmarks/forms-react-hook-form-package-doc.test.ts`, refreshed Forms receipt/package-status/read-model mirrors, `docs/packages/forms.source-guard-runbook.json`, `docs/packages/forms-react-hook-form.md`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Forms receipt-helper/read-model guards, helper `--write`, helper `--check --json`, focused Forms package guard bundle, TS syntax checks, JSON parse checks, scoped diff whitespace check, and conflict-marker scan. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, Rust manifest execution, live browser submission proof, persistence, spam protection, accessibility QA, and native Zed rendering. Exact next action: surface the same Forms helper attribution arrays in the Rust DX Studio/check-panel Forms row so editors can show exact drift paths directly from `dx check` output.`
- `DX.md:2068:- Backend Platform Client receipt coverage for preview-manifest materializer drift, 2026-05-22: lane 10 now hash-backs `tools/launch/materialize-www-template.ts` in the official **Backend Platform Client** (`supabase/client`) dashboard workflow receipt, `.dx/forge/package-status.json`, typed read model, package-owned source-guard runbook fixture, and `backend-platform-client:receipt-hash-refresh` helper output. The helper reports `source_guard_runbook_fixture`, `preview_manifest_materializer`, `tracked_files`, per-file freshness, and seven tracked SHA-256 files; package-status also exposes selected surface `backend-platform-client-preview-manifest-materializer`, making generated `public/preview-manifest.json` fixture emission drift stale-detectable without claiming hosted Supabase runtime proof. Upstream source re-inspected from `G:/WWW/inspirations/supabase/package.json`, the Next.js user-management profile form, SSR browser/server clients, and UI Library Next.js middleware; public APIs used remain `createBrowserClient`, `createServerClient`, `auth.getUser`, `from('profiles').select`, and `from('profiles').upsert`. Files changed: `examples/template/backend-platform-client-receipt-hashes.ts`, `benchmarks/supabase-receipt-hash-refresh.test.ts`, refreshed Backend Platform Client receipt/package-status/read-model hash mirrors, `docs/packages/backend-platform-client.source-guard-runbook.json`, `docs/packages/supabase-client.md`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Backend Platform Client receipt-helper materializer guard, helper `--write`, helper `--check --json`, focused Backend Platform Client guard bundle, syntax/JSON checks, and scoped hygiene checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, Rust manifest execution, `dx www preview-manifest --json`, hosted Supabase runtime proof, accessibility review, and browser visual proof. Exact next action: add a materializer-only stale fixture proving `backend-platform-client:receipt-hash-refresh` blames only `tools/launch/materialize-www-template.ts` while selected Supabase dashboard/source hashes remain current.`
- `DX.md:2094:- Backend Platform Client generated preview-manifest runbook fixture exposure, 2026-05-22: lane 10 now emits the official **Backend Platform Client** (`supabase/client`) source-guard runbook fixture into generated starter `public/preview-manifest.json`. `tools/launch/materialize-www-template.ts` writes `docs/packages/backend-platform-client.source-guard-runbook.json` into root `sourceGuardRunbookFixtures` and the `/launch` `routes[].sourceGuardRunbookFixtures` entry with `backend-platform-client-lower-dx-check-helper-freshness`, `SOURCE-ONLY`, `runtimeProof: false`, and `backend-platform-client:receipt-hash-refresh`, so generated apps can point Zed/DX Studio back to the package-owned Backend Platform Client helper-freshness runbook without parsing Rust or claiming hosted Supabase runtime proof. The package fixture now records the preview-manifest contract, and the package docs/framework structure explain the generated metadata handoff. Upstream source re-inspected from `G:/WWW/inspirations/supabase/package.json`, the Next.js user-management profile form, SSR browser/server clients, and UI Library Next.js middleware; public APIs used remain `createBrowserClient`, `createServerClient`, `auth.getUser`, `from('profiles').select`, and `from('profiles').upsert`. Files changed: `tools/launch/materialize-www-template.ts`, `benchmarks/supabase-dx-check-package-lane-panel.test.ts`, `docs/packages/backend-platform-client.source-guard-runbook.json`, `docs/packages/supabase-client.md`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green generated preview-manifest fixture assertion, fixture preview contract assertion, helper refresh/check, focused Backend Platform Client guard bundle, syntax/JSON checks, and scoped hygiene checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, Rust manifest execution, `dx www preview-manifest --json`, hosted Supabase runtime proof, accessibility review, and browser visual proof. Exact next action: add `tools/launch/materialize-www-template.ts` to the Backend Platform Client receipt hash helper as a selected preview-manifest materializer surface so future generated fixture drift is stale-detectable through `backend-platform-client:receipt-hash-refresh`.`
- `DX.md:2096:- Forms Studio manifest receipt surface, 2026-05-22: lane 6 now hash-backs `dx-www/src/cli/studio_manifest.rs` as the selected `forms-studio-manifest` surface in the official **Forms** (`forms/react-hook-form`) dashboard workflow receipt, `.dx/forge/package-status.json`, typed read model, static `/launch` package-lane markers, and `forms:receipt-hash-refresh` helper output. The helper reports 6 tracked files while keeping `docs/packages/forms.source-guard-runbook.json` as `source_guard_runbook_fixture`, so structured Studio `fixture_path` drift for `forms-generated-starter-materialization` becomes stale-detectable without claiming browser submission proof. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form`: `package.json`, public exports, `useForm`, context provider, controller, field array, and form/error/resolver types; public APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, and `FieldErrors`. Files changed: Forms receipt helper/tests, dashboard workflow receipt, package-status/read-model mirrors, Forms package docs/runbook, static launch markers, and `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`. Verification scope: red/green Forms receipt/read-model/package-lane tests, helper `--write`, and helper `--check --json`; final syntax/JSON/hygiene checks are scoped before handoff. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, Rust manifest execution, live browser submission proof, persistence, spam protection, and accessibility QA. Exact next action: add structured Forms helper attribution arrays (`current_files`, `stale_files`, `missing_files`, and mirror drift paths) so `forms:receipt-hash-refresh` can show the exact stale Studio/read-model path without opening raw receipts.`
- `DX.md:2126:- Backend Platform Client source-guard helper-freshness runbook, 2026-05-22: lane 10 now publishes the official **Backend Platform Client** (`supabase/client`) helper-freshness Rust fixture through Zed/DX Studio source-guard metadata. `dx-www/src/cli/studio_manifest.rs` adds `backend-platform-client-lower-dx-check-helper-freshness` to `source_guard_index`, `/launch` `source_guard_ids`, `/launch` `source_guard_runbook_index.fixture_paths[]`, runbook contracts, and runbook commands with fixture path `docs/packages/backend-platform-client.source-guard-runbook.json`. The package fixture now records `helper_freshness_guard`, `source_guard_fixture_paths`, receipt freshness metrics, and `backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean` so Studio can show helper-only stale evidence while `backend_platform_client_hash_mismatch` stays byte-derived. Upstream source re-inspected from `G:/WWW/inspirations/supabase`: root package metadata, user-management profile form, SSR browser/server clients, and UI Library Next.js middleware; public APIs used remain `createBrowserClient`, `createServerClient`, `auth.getUser`/`getClaims`, `from('profiles').select`, and `from('profiles').upsert`. Files changed: `dx-www/src/cli/studio_manifest.rs`, `benchmarks/supabase-dx-check-package-lane-panel.test.ts`, `docs/packages/backend-platform-client.source-guard-runbook.json`, `docs/packages/supabase-client.md`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, refreshed Backend Platform Client receipt/package-status/read-model hash mirrors, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Backend Platform Client source-guard fixture guard, helper `--write`, helper `--check --json`, focused Supabase guard bundle, syntax/JSON checks, Rust manifest formatting check, project-check Rust formatting check, and scoped hygiene checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, Rust manifest execution, hosted Supabase runtime proof, accessibility audit, and browser visual proof. Exact next action: add generated preview-manifest fixture exposure for `docs/packages/backend-platform-client.source-guard-runbook.json` so starter snapshots point at the same helper-freshness runbook without parsing the Rust manifest.`
- `DX.md:2138:- Forms structured Studio fixture-path metadata, 2026-05-22: lane 6 now publishes the official **Forms** (`forms/react-hook-form`) source-guard runbook fixture through structured Rust Studio manifest fields. `dx-www/src/cli/studio_manifest.rs` uses `studio_source_guard_with_fixture` for `forms-generated-starter-materialization`, adds the `/launch` `source_guard_runbook_index.fixture_paths[]` entry for `docs/packages/forms.source-guard-runbook.json`, and mirrors the same fixture path on the runbook contract and command so Zed/DX Studio can resolve the Forms runbook without parsing proof strings. The package fixture now records the `source_guard_index[].fixture_path`, `source_guard_runbook_index[].fixture_paths[]`, `contracts[].fixture_path`, and `commands[].fixture_path` handoff fields while staying `SOURCE-ONLY` and not claiming browser submission proof. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form`: `package.json`, `src/index.ts`, `src/useForm.ts`, `src/useFormContext.tsx`, `src/controller.tsx`, `src/useController.ts`, `src/useFieldArray.ts`, `src/types/form.ts`, `src/types/errors.ts`, and `src/types/resolvers.ts`; public APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, and `FieldErrors`. Files changed: `dx-www/src/cli/studio_manifest.rs`, `benchmarks/forms-dx-check-package-lane-panel.test.ts`, `docs/packages/forms.source-guard-runbook.json`, `docs/packages/forms-react-hook-form.md`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, refreshed Forms receipt/package-status/read-model hash mirrors, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Forms structured fixture-path guard, helper freshness refresh/check, focused Forms Node guard bundle, syntax/JSON checks, and scoped hygiene checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, Rust manifest execution, `dx www preview-manifest --json`, live Forms browser submission proof, persistence, spam protection, and accessibility QA. Exact next action: add `dx-www/src/cli/studio_manifest.rs` to the Forms receipt hash helper as a selected Studio manifest surface so future structured fixture-path drift is stale-detectable through `forms:receipt-hash-refresh`.`
- `DX.md:2156:- Backend Platform Client Rust helper-freshness metrics, 2026-05-22: lane 10 now promotes official **Backend Platform Client** (`supabase/client`) `receipt_hash_refresh` helper evidence into both Rust dx-check surfaces. `core/src/ecosystem/project_check/backend_platform_client_dx_check.rs` emits `backend_platform_client_receipt_hash_refresh_current`, `backend_platform_client_receipt_hash_refresh_stale`, and `backend_platform_client_receipt_hash_refresh_missing`, treats stale or missing helper freshness as stale receipt evidence, and keeps `backend_platform_client_hash_mismatch` byte-derived. `core/src/ecosystem/dx_check_receipt.rs` mirrors those metrics in the DX Studio/check-panel Backend Platform Client package row, including a stale-helper-only fixture that flips only the helper payload while selected Supabase source hashes stay current. Upstream source re-inspected from `G:/WWW/inspirations/supabase`: user-management account form, SSR browser/server clients, UI Library Next.js registry and client/server/middleware files; public APIs used remain `createBrowserClient`, `createServerClient`, `auth.getUser`, `from('profiles').select`, and `from('profiles').upsert`. Files changed: `core/src/ecosystem/project_check/backend_platform_client_dx_check.rs`, `core/src/ecosystem/dx_check_receipt.rs`, `benchmarks/supabase-dx-check-output.test.ts`, `benchmarks/supabase-dx-check-package-lane-panel.test.ts`, `benchmarks/supabase-package-status-read-model.test.ts`, Backend Platform Client package-status/read-model metric mirrors, `docs/packages/supabase-client.md`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green focused Backend Platform Client dx-check output, package-lane panel, read-model, receipt helper, TS syntax, JSON parse, project-check Rust format, and scoped hygiene checks; shared `dx_check_receipt.rs` rustfmt remains blocked by unrelated pre-existing formatting drift outside lane 10. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, hosted Supabase Auth/database/Storage/Realtime/RPC/Edge Function proof, accessibility audit, and browser visual proof. Exact next action: publish `backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean` through the Backend Platform Client Studio/Zed source-guard runbook fixture after the shared manifest surface is calm enough to edit.`
- `DX.md:2158:- Forms preview-manifest runbook fixture exposure, 2026-05-22: lane 6 now emits the official **Forms** (`forms/react-hook-form`) source-guard runbook fixture into generated starter `public/preview-manifest.json`. `tools/launch/materialize-www-template.ts` writes `docs/packages/forms.source-guard-runbook.json` into root `sourceGuardRunbookFixtures` and the `/launch` `routes[].sourceGuardRunbookFixtures` list with `forms-generated-starter-materialization`, `honestyLabel: SOURCE-ONLY`, `runtimeProof: false`, and `forms:receipt-hash-refresh`, so generated apps can point Zed/DX Studio back to the package-owned Forms runbook without parsing Rust or claiming browser submission proof. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form`: `package.json`, `src/index.ts`, `src/useForm.ts`, `src/useFormContext.tsx`, `src/controller.tsx`, `src/useController.ts`, `src/useFieldArray.ts`, `src/types/form.ts`, `src/types/errors.ts`, and `src/types/resolvers.ts`; public APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, and `FieldErrors`. Files changed: `tools/launch/materialize-www-template.ts`, `benchmarks/forms-dx-check-package-lane-panel.test.ts`, `docs/packages/forms.source-guard-runbook.json`, `docs/packages/forms-react-hook-form.md`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, refreshed Forms receipt/package-status/read-model hash mirrors, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Forms generated preview-manifest fixture assertions, fixture preview contract assertions, helper freshness refresh/check, focused Forms Node guard bundle, syntax/JSON checks, and scoped hygiene checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, Rust manifest execution, `dx www preview-manifest --json`, live Forms browser submission proof, persistence, spam protection, and accessibility QA. Exact next action: add structured `fixture_path` metadata for `forms-generated-starter-materialization` in the Rust Studio manifest once the shared manifest file is calm enough to edit.`
- `DX.md:2202:- Backend Platform Client receipt tracks source-guard runbook fixture, 2026-05-22: lane 10 now includes `docs/packages/backend-platform-client.source-guard-runbook.json` in the official **Backend Platform Client** (`supabase/client`) dashboard workflow receipt, `.dx/forge/package-status.json`, and `forge-package-status-read-model.ts`. `examples/template/backend-platform-client-receipt-hashes.ts --check --json` now reports `source_guard_runbook_fixture` and 6 tracked files, and `--write` refreshes the fixture hash plus the package-status/read-model `receipt_hash_refresh` mirrors so stale Studio runbook metadata is visible through `backend-platform-client:receipt-hash-refresh` without claiming hosted Supabase runtime proof. Upstream source re-inspected from `G:/WWW/inspirations/supabase`: user-management account form, SSR browser/server clients, UI Library Next.js registry and client/server/middleware files; public APIs used remain `createBrowserClient`, `createServerClient`, `auth.getUser`, `from('profiles').select`, and `from('profiles').upsert`. Files changed: `examples/template/backend-platform-client-receipt-hashes.ts`, `benchmarks/supabase-receipt-hash-refresh.test.ts`, `docs/packages/supabase-client.md`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, refreshed Backend Platform Client receipt/package-status/read-model hashes, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green `dx run --test ./benchmarks/supabase-receipt-hash-refresh.test.ts`, helper `--write`, helper `--check --json`, focused Backend Platform Client read-model/package-lane/dx-check/dashboard/launch guards, syntax checks, and scoped hygiene checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, hosted Supabase Auth/database/Storage/Realtime/RPC/Edge Function proof, accessibility audit, and browser visual proof. Exact next action: add lower-level Backend Platform Client `receipt_hash_refresh` current/stale/missing metrics to Rust dx-check/check-panel output so helper-only drift is visible without a selected-file hash mismatch.`
- `DX.md:2240:- Backend Platform Client source-guard runbook JSON fixture, 2026-05-22: lane 10 now has `docs/packages/backend-platform-client.source-guard-runbook.json` as the package-owned fixture for official **Backend Platform Client** (`supabase/client`) dx-style Rust output source-guard metadata. The fixture mirrors `backend-platform-client-dx-style-rust-check-output`, the exact targetable command `cargo test -q -p dx-www-compiler backend_platform_client_dx_style_missing_metric_and_finding_flip --lib`, the lightweight guard `dx run --test ./benchmarks/supabase-dx-check-output.test.ts`, the `/launch` `source_guard_runbook_index` contract, Zed/DX Studio metric markers, receipt hash helper metadata, upstream Supabase provenance, app-owned boundaries, and `SOURCE-ONLY` runtime limitations without claiming hosted Supabase runtime proof. `dx-www/src/cli/studio_manifest.rs`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, and `docs/packages/supabase-client.md` now point to that JSON fixture so workers and tools can read the lane runbook contract without parsing raw Rust. Upstream source re-inspected from `G:/WWW/inspirations/supabase`: the Next.js user-management account form, SSR browser/server clients, and UI Library Next.js Supabase client registry files; public APIs used remain `createBrowserClient`, `createServerClient`, `auth.getUser`, `from('profiles').select`, and `from('profiles').upsert`. Files changed: `docs/packages/backend-platform-client.source-guard-runbook.json`, `benchmarks/supabase-dx-check-package-lane-panel.test.ts`, `dx-www/src/cli/studio_manifest.rs`, `docs/DX_WWW_FRAMEWORK_STRUCTURE.md`, `docs/packages/supabase-client.md`, refreshed Backend Platform Client receipt/package-status/read-model hashes, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Backend Platform Client source-guard fixture guard, focused Backend Platform Client dx-check output/read-model/hash-refresh checks, fixture JSON parse, syntax checks, and scoped hygiene checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, Rust manifest execution, hosted Supabase Auth/database/Storage/Realtime/RPC/Edge Function proof, accessibility audit, and browser visual proof. Exact next action: add the runbook fixture to the Backend Platform Client receipt hash manifest so fixture drift becomes stale-detectable.`
- `DX.md:2282:- Backend Platform Client Rust dx-style dx-check output, 2026-05-22: lane 10 now consumes official **Backend Platform Client** (`supabase/client`) `dx_style_compatibility` evidence in `core/src/ecosystem/project_check/backend_platform_client_dx_check.rs`. The lower-level Rust dx-check output now emits `backend_platform_client_dx_style_compatibility_present` and `backend_platform_client_dx_style_compatibility_missing`, raises `backend-platform-client-missing-dx-style-compatibility` when the package-status row loses `dx.forge.package.dx_style_compatibility`, and adds the targetable fixture `backend_platform_client_dx_style_missing_metric_and_finding_flip` beside the existing byte-derived SHA-256 hash mismatch fixture. Upstream source re-inspected from `G:/WWW/inspirations/supabase`: user-management `account-form.tsx`, SSR `client.ts`, SSR `server.ts`, Studio API docs, and package metadata; public APIs used remain `createBrowserClient`, `createServerClient`, `auth.getUser`, `from('profiles').select`, and `from('profiles').upsert`. Files changed: `core/src/ecosystem/project_check/backend_platform_client_dx_check.rs`, `benchmarks/supabase-dx-check-output.test.ts`, `docs/packages/supabase-client.md`, Backend Platform Client receipt/package-status/read-model hashes, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green `dx run --test ./benchmarks/supabase-dx-check-output.test.ts`, helper hash refresh/check, JS/Rust syntax and formatting checks, and scoped hygiene checks. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, hosted Supabase runtime proof, accessibility audit, browser visual proof, and Cargo compilation unless the shared Rust workspace is calm. Exact next action: publish the `backend_platform_client_dx_style_missing_metric_and_finding_flip` command into a Zed/DX Studio source-guard runbook after the Rust manifest/runbook surface is ready for lane 10.`
- `DX.md:2330:- Backend Platform Client dx-style compatibility metadata, 2026-05-22: lane 10 now exposes official **Backend Platform Client** (`supabase/client`) dx-style evidence for the visible profile and schema-query workflow surfaces. `examples/template/supabase-profile-workflow.tsx` and `examples/template/data-status.tsx` now carry `data-dx-style-surface="backend-platform-client"` and `data-dx-token-scope="supabase/client"`; the Backend Platform Client dashboard receipt, `.dx/forge/package-status.json`, and `forge-package-status-read-model.ts` now publish `dx.forge.package.dx_style_compatibility`, `backend_platform_client_dx_style_compatibility_present`, `backend_platform_client_dx_style_compatibility_missing`, and `backend-platform-client:dx-style-compatibility` without claiming hosted Supabase runtime proof. Upstream source re-inspected from `G:/WWW/inspirations/supabase`: Next.js user-management account form, SSR browser/server client helpers, Studio API docs, and package metadata; public APIs used remain `createBrowserClient`, `createServerClient`, `auth.getUser`, `from('profiles').select`, and `from('profiles').upsert`. Files changed: `examples/template/supabase-profile-workflow.tsx`, `examples/template/data-status.tsx`, Backend Platform Client receipt/package-status/read-model hashes, `benchmarks/supabase-package-status-read-model.test.ts`, `docs/packages/supabase-client.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Backend Platform Client package-status read-model guard and package-owned hash refresh. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, hosted Supabase Auth/database/Storage/Realtime/Edge Function proof, accessibility audit, and browser visual proof. Exact next action: consume `dx_style_compatibility` in the Rust dx-check/check-panel Backend Platform Client row so CLI JSON can emit the present/missing style metrics from package-status.`
- `DX.md:2502:- Forms receipt-hash refresh helper, 2026-05-22: lane 6 now has a package-owned helper at `examples/template/forms-receipt-hashes.ts` for official **Forms** receipt maintenance. The helper validates `package_id: forms/react-hook-form`, official package naming, upstream `react-hook-form` `7.75.0` provenance, `hash_algorithm: sha256`, selected receipt `file_hashes`, generated-starter `examples/template/` path fallback, package-status selected-surface mirrors, and typed read-model hashes. `--check --json` emits `dx.forge.package.receipt_hash_refresh` with `forms:receipt-hash-refresh`; `--write` refreshes the dashboard workflow receipt, `.dx/forge/package-status.json`, and `forge-package-status-read-model.ts` together without running browser submission proof, installing packages, reading secrets, or claiming accessibility/persistence/runtime proof. Upstream source re-inspected from `G:/WWW/inspirations/react-hook-form`: `package.json`, `src/index.ts`, `src/useForm.ts`, `src/useFormContext.tsx`, `src/controller.tsx`, `src/useController.ts`, `src/useFieldArray.ts`, and `src/types/form.ts`; APIs used remain `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, and `FieldErrors`. Files changed: `examples/template/forms-receipt-hashes.ts`, `benchmarks/forms-receipt-hash-refresh.test.ts`, `docs/packages/forms-react-hook-form.md`, the Forms dashboard workflow receipt/package-status/read-model hash mirrors, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Forms receipt-hash helper guard, helper syntax check, real helper `--check --json` / `--write` / `--check --json`, existing Forms package-status/doc/dx-check/panel guards, JSON parse, and scoped hygiene checks. Heavy checks skipped: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo, and live Forms browser submission proof. Exact next action: mirror the helper freshness JSON into the Forms package-status/read-model row as `receiptHashRefresh` so Zed/DX Studio can show helper freshness without opening raw receipt JSON.`
- `DX.md:2548:- UI Components receipt hash refresh helper, 2026-05-22: lane 20 now has a package-owned helper for official **UI Components** receipt hash maintenance at `examples/template/ui-components-receipt-hashes.ts`. The helper validates `package_id: shadcn/ui/button`, `official_package_name: UI Components`, `hash_algorithm: sha256`, and the selected UI-owned receipt `file_hashes` array, supports generated-starter `examples/template/` path fallback, reports `dx.forge.package.receipt_hash_refresh` JSON with `ui-components:receipt-hash-refresh` Zed visibility, and refreshes only receipt SHA-256 values plus the `dx_check_visibility.file_hashes` mirror with `--write`. It does not run browser UI runtime proof, install packages, read secrets, or claim accessibility review. Upstream source re-inspected from `G:/WWW/inspirations/shadcn-ui` and `G:/WWW/inspirations/radix-primitives`: shadcn `package.json`, v4 `button.tsx`, Radix `@radix-ui/react-slot` metadata, and Radix `slot.tsx`; public APIs used remain `Button`, `buttonVariants`, `Slot`, and `createSlot`, with `Label` and `Separator` kept as inspected provenance for the selected lane. Files changed: `examples/template/ui-components-receipt-hashes.ts`, `benchmarks/ui-components-receipt-hash-refresh.test.ts`, `docs/packages/ui-components.md`, the UI Components dashboard controls receipt guard list, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green `dx run --test ./benchmarks/ui-components-receipt-hash-refresh.test.ts`, `dx run --check ./examples/template/ui-components-receipt-hashes.ts`, and `node tools/launch/run-template-receipt-helper.js examples/template/ui-components-receipt-hashes.ts --check --json`. Heavy checks skipped: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo, and live browser UI runtime proof. Exact next action: mirror the helper JSON status into the UI Components package-status/read-model row so Zed/DX Studio can show helper freshness without opening raw receipt JSON.`
- `DX.md:2788:- Latest dx-style font-stretch pass (2026-05-22): `related-crates/style/src/core/engine/utility/mod.rs` now emits normal CSS for Tailwind font-stretch helpers including `font-stretch-condensed`, `font-stretch-semi-expanded`, `font-stretch-50%`, `font-stretch-[62.5%]`, and `font-stretch-(--dx-font-stretch)`. The generated-output parity receipt now marks `font-stretch-condensed` supported and moves the honest unsupported fixture to `forced-color-adjust-auto`, so DX-WWW still blocks unsupported scanned forced-color-adjust classes instead of claiming full Tailwind parity. Verification passed so far: red/green targeted `dx run --test ./benchmarks/dx-style-launch-contract.test.ts -- --test-name-pattern "dx-style font-stretch utilities generate normal CSS|dx-style Tailwind parity receipt records generated-output support honestly"`. Skipped by instruction so far: full builds, broad suites, servers, browser automation, package installs, deploys, cargo builds/checks, and broad Tailwind parity testing. Exact next action: implement `forced-color-adjust-auto` / `forced-color-adjust-none` before any full Tailwind accessibility claim.`
- `DX.md:2970:- Boundary: no local server, full build, or governed browser QA ran in this pass; live accessibility/browser proof and persisted route choreography remain governed.`
- `DX.md:3003:- `animation/motion`: Motion `12.38.0` source-owned slice for `motion/react`, `DxMotionProvider` / MotionConfig defaults, `MotionControlledStatus` / `useDxAnimationControls` over `useAnimationControls`, `useAnimation`, `animationControls`, and `LegacyAnimationControls`, `MotionFrameTicker` / `useDxFrameClock` over `useAnimationFrame`, `useTime`, and `useCycle`, `MotionPageVisibilityBadge` / `useDxPageVisibility` over `usePageInView`, `MotionWillChangeBox` / `useDxWillChange` over `useWillChange` and `WillChangeMotionValue`, `DxLazyMotionProvider` / `MotionLazyBox` / `dxLazyMotion` over `LazyMotion`, `domAnimation`, `domMax`, `domMin`, and `m`, `DxMotionLayoutGroup` / `MotionLayoutItem` / `dxMotionLayoutId` / `useDxInstantLayoutTransition` over `LayoutGroup`, `layoutId`, `layoutDependency`, `layoutRoot`, and `useInstantLayoutTransition`, `MotionValueMeter` / `useDxMotionValueMeter` over `useMotionValue`, `useTransform`, `useMotionTemplate`, `useMotionValueEvent`, and `useVelocity`, `DxMotionPresence` / AnimatePresence + LayoutGroup defaults, `DxReorderGroup` / `DxReorderItem` / `useDxReorderControls`, `MotionReveal`, launch variants, transitions, viewport defaults, `useInView`, `useReducedMotion`, scoped `useAnimate` press feedback helpers, and `useScroll`/`useSpring` scroll progress. Add with `dx add motion/react --write` or the `motion` / `framer-motion` aliases. The runtime-safe `/launch` proof uses `data-dx-package="animation/motion"`, `data-dx-component="motion-animation-card"`, advance/reorder/reset/toggle-reduced-motion controls, Arrow/Home/End keyboard reorder markers, `data-dx-motion-reduced`, and `--dx-motion-*` theme tokens, with `dx run --test ./benchmarks/motion-launch-materialized.test.ts` and `dx run --test ./benchmarks/motion-runtime-interaction.test.ts` as the narrow generated-source and browser-behavior guards. Applications still own global motion policy, imperative animation sequencing, frame sampling policy, page visibility policy, will-change performance-hint policy, LazyMotion feature-bundle selection, strict `m` migration, choreography, gestures, route transitions, MotionValue semantics, reorder persistence, cross-list drag targets, governed keyboard accessibility QA, scroll-linked information architecture, performance budgets, and reduced-motion review.`
- `DX.md:3005:- Motion accessible order controls: `moveMotionDashboardStage` now backs visible `move-stage-previous` and `move-stage-next` interactions in the starter dashboard and no-`node_modules` `/launch` runtime proof, and the stage list now supports Arrow/Home/End keyboard reorder markers while final browser accessibility QA remains governed.`

### Pattern: `a11y`
- `a11y/README.md:2:# dx-www-a11y`
- `a11y/README.md:14:dx-www-a11y = { path = "../a11y" }`
- `a11y/tests/property_tests.rs:1://! Property-based tests for dx-www-a11y`
- `a11y/tests/property_tests.rs:6:use dx_www_a11y::{A11yReport, A11ySeverity, ASTAnalyzer};`
- `a11y/tests/property_tests.rs:72:    fn prop_a11y_rule_detection(`
- `a11y/CHANGELOG.md:4:All notable changes to dx-www-a11y will be documented in this file. The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.`
- `a11y/Cargo.toml:2:name = "dx-www-a11y"`
- `a11y/Cargo.toml:9:keywords = ["wasm", "web", "accessibility", "dx", "a11y"]`
- `a11y/src/lib.rs:1://! # dx-a11y — Compile-Time Accessibility Auditor`
- `Cargo.toml:4:    "a11y",`
- `Cargo.toml:59:dx-www-a11y = { path = "a11y" }`
- `Cargo.lock:1758:name = "dx-www-a11y"`
- `Cargo.lock:1881: "dx-www-a11y",`
- `Cargo.lock:2033: "dx-www-a11y",`
- `docs/SECURITY.md:109:The following crates contain no unsafe code and have `#![forbid(unsafe_code)]` enabled: -`dx-www-a11y` - Accessibility analysis -`dx-www-auth` - Authentication (uses safe crypto wrappers) -`dx-www-cache` - Caching layer -`dx-www-db` - Database abstractions -`dx-www-db-teleport` - Database teleport utilities -`dx-www-debug` - Debugging utilities -`dx-www-dom` - DOM abstractions -`dx-www-error` - Error types -`dx-www-fallback` - Fallback rendering -`dx-www-form` - Form handling -`dx-www-guard` - Route guards -`dx-www-interaction` - User interaction handling -`dx-www-offline` - Offline support -`dx-www-print` - Print stylesheets -`dx-www-query` - Query handling -`dx-www-rtl` - RTL support -`dx-www-sched` - Scheduling -`dx-www-state` - State management -`dx-www-sync` - Synchronization`
- `docs/repo-hygiene.md:77:- Root crates such as `a11y/`, `auth/`, `binary/`, `browser/`, `cache/`,`
- `core/src/www_config.rs:180:    pub a11y: bool,`
- `core/src/www_config.rs:189:    &["forms", "query", "auth", "sync", "offline", "a11y", "i18n"];`
- `core/src/www_config.rs:198:    ("a11y", &[]),          // a11y has no dependencies`
- `core/src/www_config.rs:221:        if self.a11y {`
- `core/src/www_config.rs:222:            features.push("a11y");`
- `core/src/www_config.rs:238:            "a11y" => self.a11y,`
- `core/src/www_config.rs:267:            "a11y" => {`
- `core/src/www_config.rs:268:                self.a11y = true;`
- `core/src/www_config.rs:302:            "a11y" => {`
- `core/src/www_config.rs:303:                self.a11y = false;`
- `core/src/www_config.rs:772:            "a11y" => config.features.a11y = value == "true",`
- `core/src/www_config.rs:1203:    merged.features.a11y = base.features.a11y || override_config.features.a11y;`

### Pattern: `template`
- `browser-micro/src/lib.rs:21:    fn host_clone_template(id: u32) -> u32;`
- `browser-micro/src/lib.rs:36:/// Render template by ID to body (id=0)`
- `browser-micro/src/lib.rs:38:pub extern "C" fn render(template_id: u32) -> u32 {`
- `browser-micro/src/lib.rs:40:        let node = host_clone_template(template_id);`
- `AGENTS.md:11:- Current template proof is green:`
- `AGENTS.md:12:  - `dx check examples/template --json` -> `100/100`, `green`.`
- `AGENTS.md:13:  - `examples/template/.dx/receipts/check/check-latest.json` -> `500/500`.`
- `AGENTS.md:75:- Use `www` and `template` for current public concepts.`
- `AGENTS.md:90:node tools/launch/run-template-receipt-helper.js examples/template/<name>-receipt-hashes.ts --write`
- `AGENTS.md:96:dx check examples/template --json`
- `AGENTS.md:109:`examples/template/.dx/forge/package-status.json`; parallel writes can corrupt`
- `AGENTS.md:117:node --test benchmarks/www-template-score-gate.test.ts benchmarks/www-template-forge-reality.test.ts benchmarks/www-forge-package-status-read-model.test.ts benchmarks/dx-www-agent-context-command.test.ts benchmarks/www-template-source-honesty.test.ts`
- `AGENTS.md:118:dx check examples/template --json`
- `AGENTS.md:171:Do not claim `100/100` unless `dx check examples/template --json` is actually`
- `binary/tests/validation_property_tests.rs:34:        template_html in "[a-zA-Z0-9 ]{1,50}",`
- `binary/tests/validation_property_tests.rs:38:        writer.write_template(0, &template_html, vec![]);`
- `binary/tests/validation_property_tests.rs:71:        template_html in "[a-zA-Z0-9 ]{1,50}",`
- `binary/tests/validation_property_tests.rs:74:        writer.write_template(0, &template_html, vec![]);`
- `binary/tests/validation_property_tests.rs:106:        template_html in "[a-zA-Z0-9 ]{1,50}",`
- `binary/tests/validation_property_tests.rs:110:        writer.write_template(0, &template_html, vec![]);`
- `binary/tests/validation_property_tests.rs:143:        template_html in "[a-zA-Z0-9 ]{1,50}",`
- `binary/tests/validation_property_tests.rs:146:        writer.write_template(0, &template_html, vec![]);`
- `binary/tests/validation_property_tests.rs:174:        template_html in "[a-zA-Z0-9 ]{1,50}",`
- `binary/tests/validation_property_tests.rs:182:        writer.write_template(0, &template_html, vec![]);`
- `binary/tests/validation_property_tests.rs:215:        template_html in "[a-zA-Z0-9 ]{1,50}",`
- `binary/tests/validation_property_tests.rs:221:        writer.write_template(0, &template_html, vec![]);`
- `binary/tests/validation_property_tests.rs:264:        template_html in "[a-zA-Z0-9 ]{1,50}",`
- `binary/tests/validation_property_tests.rs:268:        writer.write_template(0, &template_html, vec![]);`

### Pattern: `dx new`
- `CHANGELOG.md:1564:- Added the default WWW template architecture contract. `dx new` now writes `dx.www.default_template.architecture_contract` metadata into the generated template manifest, readiness receipt, standalone `.dx/forge/template-readiness/zed-template-handoff.json`, and readiness bundle, and `dx templates --json` reports the same contract from `dx-www/src/cli/default_template_contract.rs`. The launch readiness-bundle consumer now reads the standalone Zed handoff first and exposes the contract fields in JSON, terminal, and Markdown output, proving DX-WWW is the runtime foundation while React/RSC/Node/NAPI, `node_modules`, and external framework clone parity remain false. Guarded with red/green `node --test ./benchmarks/default-www-template-contract.test.ts`; full builds, broad suites, servers, browser automation, package installs, and deploys were skipped by policy.`
- `CHANGELOG.md:2014:- Added the static `/` WebAssembly Bridge package-lane template. The lane 18 package `wasm/bindgen` keeps official **WebAssembly Bridge** naming while the receipt-less dx-check panel now exposes `data-dx-check-package-lane-template="wasm/bindgen"`, upstream `wasm-bindgen` `0.2.121` provenance from `G:/WWW/inspirations/wasm-bindgen`, the dashboard workflow receipt path, `data-dx-check-package-lane-dx-style-status="present"`, `data-dx-style-surface="theme-token"`, `data-dx-token-scope="wasm/bindgen"`, and the receipt-backed `webassembly_bridge_*` metric vocabulary before a fresh dx-check receipt is loaded. The launch shell, static runtime page, DX Studio edit contract, runtime materializer, and Rust Studio manifest include WebAssembly Bridge in the `dx-check-health-panel` package filter while live generated-Wasm runtime proof remains app-owned. Re-inspected upstream wasm-bindgen Cargo metadata and generated web-target reference glue for `__wbg_load`, `WebAssembly.instantiateStreaming`, `initSync`, and default async init exports. Guarded with red/green `dx run --test ./benchmarks/wasm-bindgen-dx-check-package-lane-panel.test.ts`; focused source checks, JSON parse, syntax checks, Rust manifest formatting, diff hygiene, and conflict-marker scans are recorded in the worker closeout. `dx run --test ./benchmarks/wasm-bindgen-hash-receipt.test.ts` was attempted and remained stale because shared launch files changed again after the lane 18 hash refresh. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, live generated-Wasm execution, and browser visual proof were skipped. Next action: add a generated-starter materialization guard proving the WebAssembly Bridge static package-lane template survives `dx new` output after shared launch-file hash churn settles.`
- `CHANGELOG.md:2072:- Added Payments static `/launch` package-lane helper markers. The lane 12 package `payments/stripe-js` keeps official **Payments** naming while the receipt-less static dx-check panel now exposes `data-dx-check-package-lane-template="payments/stripe-js"`, upstream `@stripe/stripe-js` `9.6.0` provenance from `G:/WWW/inspirations/stripe-js`, the billing workflow receipt path, dx-style status, `payments:receipt-hash-refresh`, and helper tracked/stale/missing counts before `.dx/receipts/check/check-latest.json` exists. The DX Studio edit contract, launch runtime materializer, and Rust Studio manifest now include `payments/stripe-js` in the `dx-check-health-panel` package filter so Studio/Zed can discover stale helper state from static source and materialized manifests without claiming live Stripe Checkout or webhook runtime proof. Re-inspected upstream Stripe.js package metadata, loader/shared sources, public loader types, Stripe payment APIs, and Embedded Checkout types for `loadStripe`, `loadStripe.setLoadParameters`, `stripe.confirmPayment`, `stripe.retrievePaymentIntent`, `stripe.createEmbeddedCheckoutPage`, and `StripeEmbeddedCheckoutOptions.fetchClientSecret`. Guarded with red/green `dx run --test ./benchmarks/payments-dx-check-package-lane-panel.test.ts`, helper `--check --json`, helper `--write`, JS syntax checks, touched-file Rust formatting, scoped hygiene checks, and conflict-marker scan. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, live Stripe Checkout, webhook delivery, and browser visual proof were skipped. Next action: add a generated-starter materialization guard proving the Payments static package-lane template survives `dx new` output and then migrate remaining user-facing CLI copy from `dx add stripe-js --write` to official Payments naming.`
- `CHANGELOG.md:2110:- Added the AI SDK static /launch AI SDK package-lane marker. The lane 16 package `ai/vercel-ai` keeps official **AI SDK** naming while the receipt-less static dx-check panel now exposes `data-dx-check-package-lane-template="ai/vercel-ai"`, `data-dx-check-package-lane-row="ai/vercel-ai"`, missing/missing-receipt defaults, upstream `ai` `7.0.0-canary.146` provenance from `G:/WWW/inspirations/vercel-ai`, the launch assistant receipt path, `data-dx-check-package-lane-dx-style-status="present"`, `data-dx-style-surface="ai-sdk"`, and `data-dx-token-scope="ai/vercel-ai"` before a fresh dx-check receipt is loaded. Re-inspected upstream `packages/ai/package.json`, `packages/ai/src/index.ts`, `packages/ai/src/generate-text/stream-text.ts`, and `packages/ai/src/ui/default-chat-transport.ts` for `streamText`, `DefaultChatTransport`, `gateway`, `createGateway`, `createProviderRegistry`, and `tool`. Guarded with red/green `dx run --test ./benchmarks/vercel-ai-dx-check-package-lane-panel.test.ts` plus focused AI SDK guards, syntax/JSON checks, and scoped hygiene checks. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo, live provider/model streaming, and browser visual proof were skipped. Next action: add a generated-starter materialization guard proving the AI SDK static package-lane marker survives `dx new` output.`
- `CHANGELOG.md:2120:- Added the Type-Safe API static launch package-lane template. The lane 8 package `api/trpc` keeps official **Type-Safe API** naming while the receipt-less `/launch` dx-check panel now exposes `data-dx-check-package-lane-template="api/trpc"`, `data-dx-check-package-lane-row="api/trpc"`, upstream `@trpc/server` `11.17.0`, source mirror `G:/WWW/inspirations/trpc`, the dashboard workflow receipt path, and `type-safe-api:receipt-hash-refresh` helper markers. `launch-shell.tsx`, `runtime-pages/launch.html`, `dx-studio-edit-contract.ts`, the runtime materializer, and the Rust Studio manifest now include `api/trpc` in the dx-check package filter so Studio/Zed can discover helper freshness before a fresh dx-check receipt exists, without claiming live tRPC route execution. Re-inspected upstream tRPC server/client metadata and sources for `initTRPC.context().create()`, `createCallerFactory`, `fetchRequestHandler`, `createTRPCClient`, `httpBatchLink`, and `createTRPCReact`. Guarded with red/green `dx run --test ./benchmarks/trpc-dx-check-package-lane-panel.test.ts`, the focused Type-Safe API guard bundle, benchmark syntax check, helper `--check --json`, package-status JSON parse, scoped hygiene checks, and `rustfmt --edition 2024 --check ./dx-www/src/cli/studio_manifest.rs`. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, and live Type-Safe API router/client/server/auth/persistence proof were skipped. Next action: add a generated-starter materialization guard proving the Type-Safe API static package-lane template survives `dx new` output.`
- `CHANGELOG.md:2144:- Added the Automation Connectors static DX Studio/check-panel package row. The lane 19 package `automations/n8n` keeps official **Automation Connectors** naming while the static `/launch` dx-check panel now exposes `data-dx-check-package-lane-row="automations/n8n"`, official package naming, `n8n-nodes-base` `2.22.0` provenance from `G:/WWW/inspirations/n8n/packages/nodes-base`, receipt path, hash-refresh helper markers, and `data-dx-check-package-lane-dx-style-status="present"` without claiming live n8n runtime proof. `dx-studio-edit-contract.ts`, the runtime materializer, and the Rust Studio manifest now include Automation Connectors in the `dx-check-health-panel` package filter, and the Studio package surface indexes the dx-check row plus `data-dx-style-surface="automation-connectors"` / `data-dx-token-scope="automations/n8n"`. Re-inspected upstream `package.json`, `ManualTrigger.node.ts`, `Slack.node.ts`, and `SlackV2.node.ts` for `INodeType`, `INodeTypeDescription`, `IVersionedNodeType`, and `IExecuteFunctions`. Guarded with red/green `dx run --test ./benchmarks/automations-dx-check-package-lane-panel.test.ts` and refreshed Automation Connectors receipt hashes with the package helper. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo, browser visual proof, and live provider execution were skipped. Next action: add a generated-starter materialization guard proving the Automation Connectors package row survives `dx new` output before rerunning shared Cargo panel fixtures.`
- `CHANGELOG.md:2456:- Wired the official **Forms** package receipt into generated starters. `forms/react-hook-form` now keeps upstream `react-hook-form` `7.75.0` from `G:/WWW/inspirations/react-hook-form` as provenance while `examples/template/launch-route-contract.ts` exposes `packageWorkflowReceipts.formsDashboardWorkflow`, `launchRouteMaterializedFiles` lists `.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json`, and `dx-www/src/cli/mod.rs` includes `NEXT_FAMILIAR_FORMS_DASHBOARD_RECEIPT_JSON` so `dx new` writes the Forms dashboard workflow receipt beside `components/launch/launch-lead-form.tsx`. The receipt hashes were refreshed after adding the route contract as source evidence, and `docs/packages/forms-react-hook-form.md` now documents the generated-starter receipt path. Upstream APIs re-inspected: `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, `FieldErrors`, and `subscribe`. Verification used red/green `dx run --test ./benchmarks/forms-react-hook-form-package-doc.test.ts`; heavy checks, servers, browser automation, package installs, deploys, `just run`, and live form runtime proof were skipped. Next action is consuming the generated Forms receipt in shared dx-check/Zed package-status freshness rows.`
- `CHANGELOG.md:2677:- Wired the Better Auth workflow receipt into the generated-starter CLI path: `dx new` now materializes `.dx/forge/receipts/auth-better-auth.json`, includes it in the generated launch receipt source package and readiness file list, and exposes Better Auth aliases, exported files, dashboard usage, DX icon metadata, source mirror, required env, and exact receipt paths in launch catalog metadata.`
- `CHANGELOG.md:2686:- Updated the generated `dx new` starter toward the public App Router `.tsx` contract with dashboard, settings, auth, and billing routes, professional component folders, `styles/theme.dx.css`, `styles/app.generated.css`, generated import-map targets, and no template-local `node_modules`.`
- `CHANGELOG.md:2697:- Professionalized `content/fumadocs-next` for the starter dashboard and generated `/launch`: the Forge slice now exports `lib/fumadocs/dashboard-workflow.ts` plus `components/dashboard/fumadocs-docs-workflow.tsx`, package catalog metadata now carries structured `/launch` `dashboardUsage`, `dxIcon: "pack:fumadocs"`, and a source-owned dashboard workflow receipt, `dx new` now materializes that receipt into `.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json`, `examples/dashboard` consumes it through a visible Documentation System workflow, and `/launch` now mounts a `docs-help-changelog` workflow with page-tree selection, mission-control docs summary updates, route readiness, OpenAPI env readiness, LLM export readiness, changelog notes, DX icon markers, `docs-workflow` runtime styling, and a safe local route receipt instead of a Fumadocs proof card. The live runtime materializer and conversion-proof preview manifest now expose `launch-fumadocs-docs-workflow` as the editable Documentation System surface instead of the stale `fumadocs-docs-navigation-proof` selector, and the page-tree buttons/receipt output now carry `aria-pressed`, `role="status"`, and `aria-live="polite"` semantics for the governed browser pass.`
- `CHANGELOG.md:2720:- Split the shadcn/ui `/launch` dashboard controls contract into `shadcn-dashboard-controls-contract.tsx`, materialized it through `dx new`, and moved receipt construction/package metadata out of the UI component while preserving the same visible workflow.`
- `CHANGELOG.md:2742:- Wired the real Zustand dashboard workflow into `dx new` materialization metadata: generated starters now list and write `components/launch/state-zustand-dashboard.tsx` plus `.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json`, and `launch-route-contract.ts` exposes `zustandDashboardStateWorkflow` so the LaunchShell store import is backed by a copied source file and receipt instead of the older counter-only proof.`
- `CHANGELOG.md:2869:- Added the redacted no-execution Stripe billing workflow receipt at `examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json` and wired it into the package catalog, Forge metadata, and `dx new` materialized `.dx/forge/receipts` output so generated apps can carry the payment boundary proof without secrets, fake success, or a detached docs-only receipt.`
- `CHANGELOG.md:2879:- Wired `dx new` to materialize the launch package slices used by the shell without creating `node_modules`.`
- `CHANGELOG.md:2940:- Wired the InstantDB dashboard workflow receipt into `dx new` materialization so generated starters write `.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json` alongside the visible `/launch` workflow and route contract instead of relying on source-only receipt proof.`
- `CHANGELOG.md:2997:- Added the generated `.dx/forge/template-readiness/launch-readiness-bundle.json` artifact plus `dx forge launch-readiness-bundle` so operators can inspect the launch bundle after `dx new` without running a dev server, preview, or full build.`
- `CHANGELOG.md:3086:- Added generated-template file materialization for the `/launch` route in `dx new`, including the route page, launch route contract, launch shell, package catalog, counter, markdown preview, tRPC health placeholder, and runtime catalog loader.`
- `CHANGELOG.md:3087:- Tightened the generated launch component manifest so the auth session, AI chat, Zod validation, realtime, and WebAssembly companion files stay visible to DX CLI/Zed after `dx new`.`
- `CHANGELOG.md:3088:- Added targeted `dx new` coverage that verifies every launch-template manifest file is materialized and the generated package catalog carries the full launch package set.`
- `CHANGELOG.md:3093:- Added `dx new` materialization for the `/launch` route shell, including `components/launch/launch-shell.tsx`, route contract metadata, package catalog, local state preview, typed API health placeholder, and content preview without package installs or `node_modules`.`
- `CHANGELOG.md:3094:- Added the generated starter UI Forge doc at `.dx/forge/docs/dx-www-starter-ui.md` so `dx new` starters have matching source-manifest and documentation artifacts.`
- `CHANGELOG.md:3184:- Generated-template verification now also covers the `dx new` `.dx/forge/template-manifest.json` `generated_files` map, with `dx run --test benchmarks/launch-template-shell.test.ts` and the focused `dx_new_creates_next_familiar_template_surface_without_node_modules` Cargo test showing green test output.`
- `DX.md:1906:- Default WWW template architecture contract, 2026-05-23: `dx new` now writes a source-owned architecture contract for the default Next-familiar starter through `dx-www/src/cli/default_template_contract.rs`, `.dx/forge/template-manifest.json`, template readiness receipts, the standalone `.dx/forge/template-readiness/zed-template-handoff.json`, the readiness bundle, and `dx templates --json`. The readiness-bundle consumer now prefers that standalone Zed handoff and surfaces the DX runtime contract in JSON, terminal, and Markdown summaries so React/RSC/Node/NAPI, `node_modules`, and external framework clone parity stay visibly false. The generated root `dx` file says DX-WWW runtime is authoritative, node resolver behavior is not the default foundation, Forge remains source-owned, and dx-style/dx-check/Zed/Studio receipt surfaces are first-class. Verification: red/green `node --test ./benchmarks/default-www-template-contract.test.ts`; no builds, servers, package installs, browser checks, or broad Cargo suites were run.`
- `DX.md:2328:- Static `/launch` WebAssembly Bridge package-lane template, 2026-05-22: lane 18 now exposes official **WebAssembly Bridge** (`wasm/bindgen`) in the receipt-less dx-check panel before `.dx/receipts/check/check-latest.json` exists. `examples/template/launch-shell.tsx` owns the typed static template row and `examples/template/runtime-pages/index.html` mirrors `data-dx-check-package-lane-template="wasm/bindgen"`, official package naming, upstream `wasm-bindgen` `0.2.121` provenance from `G:/WWW/inspirations/wasm-bindgen`, the dashboard workflow receipt path, `data-dx-check-package-lane-dx-style-status="present"`, `data-dx-style-surface="theme-token"`, `data-dx-token-scope="wasm/bindgen"`, and the receipt-backed `webassembly_bridge_*` metric vocabulary while live generated-Wasm runtime proof remains app-owned. The source DX Studio edit contract, runtime materializer, and Rust Studio manifest now include `wasm/bindgen` in the `dx-check-health-panel` package filter. Upstream source re-inspected from `G:/WWW/inspirations/wasm-bindgen`: `Cargo.toml` and `crates/cli/tests/reference/targets-target-web.js`; public APIs used remain generated `__wbg_load`, `WebAssembly.instantiateStreaming`, `initSync`, and default async init exports. Files changed: `examples/template/launch-shell.tsx`, `examples/template/runtime-pages/index.html`, `examples/template/dx-studio-edit-contract.ts`, `tools/launch/materialize-www-template.ts`, `dx-www/src/cli/studio_manifest.rs`, `benchmarks/wasm-bindgen-dx-check-package-lane-panel.test.ts`, `examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json`, `examples/template/.dx/forge/package-status.json`, `examples/template/forge-package-status-read-model.ts`, `docs/packages/wasm-bindgen.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green WebAssembly Bridge static package-lane guard, focused source checks, JSON parse, Rust manifest formatting, and scoped hygiene checks passed; `dx run --test ./benchmarks/wasm-bindgen-hash-receipt.test.ts` was attempted and remained stale because shared launch files changed again after the lane 18 hash refresh (`launch-shell.tsx`, `runtime-pages/launch.html`, and `materialize-www-template.ts`). Heavy checks skipped: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, and live generated-Wasm/browser proof. Exact next action: add a generated-starter materialization guard proving the WebAssembly Bridge static package-lane template survives `dx new` output after shared launch-file hash churn settles.`
- `DX.md:2382:- Payments static /launch package-lane helper markers, 2026-05-22: lane 12 now exposes official **Payments** (`payments/stripe-js`) in the receipt-less static `/launch` dx-check panel before `.dx/receipts/check/check-latest.json` exists. `examples/template/runtime-pages/index.html` carries `data-dx-check-package-lane-template="payments/stripe-js"`, official package naming, upstream `@stripe/stripe-js` `9.6.0` provenance from `G:/WWW/inspirations/stripe-js`, the billing workflow receipt path, dx-style status, and `payments:receipt-hash-refresh` helper freshness counts. `examples/template/dx-studio-edit-contract.ts`, `tools/launch/materialize-www-template.ts`, and `dx-www/src/cli/studio_manifest.rs` now include `payments/stripe-js` in the `dx-check-health-panel` package filter so Studio/Zed can find stale helper state from static source and materialized manifests without claiming live Stripe Checkout or webhook runtime proof. Upstream source re-inspected from `G:/WWW/inspirations/stripe-js/package.json`, `src/pure.ts`, `src/shared.ts`, `types/shared.d.ts`, `types/stripe-js/stripe.d.ts`, and `types/stripe-js/embedded-checkout.d.ts`; public APIs used remain `loadStripe`, `loadStripe.setLoadParameters`, `stripe.confirmPayment`, `stripe.retrievePaymentIntent`, `stripe.createEmbeddedCheckoutPage`, and `StripeEmbeddedCheckoutOptions.fetchClientSecret`. Files changed: `benchmarks/payments-dx-check-package-lane-panel.test.ts`, `examples/template/runtime-pages/index.html`, `examples/template/dx-studio-edit-contract.ts`, `tools/launch/materialize-www-template.ts`, `dx-www/src/cli/studio_manifest.rs`, `docs/packages/payments-stripe-js.md`, refreshed Payments receipt/package-status/read-model hashes, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Payments static package-lane guard, package-owned helper `--check --json`, helper `--write`, syntax checks for the touched JS files, Rust formatting check for the touched Studio manifest, scoped hygiene checks, and conflict-marker scan. Heavy checks skipped by policy: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, live Stripe Checkout, webhook delivery, and browser visual proof. Exact next action: add a generated-starter materialization guard proving the Payments static package-lane template survives `dx new` output, then migrate remaining user-facing CLI copy from `dx add stripe-js --write` to official Payments naming.`
- `DX.md:2422:- AI SDK static /launch AI SDK package-lane marker, 2026-05-22: lane 16 now exposes official **AI SDK** (`ai/vercel-ai`) inside the receipt-less static `/launch` dx-check panel before `.dx/receipts/check/check-latest.json` exists. `examples/template/runtime-pages/index.html` carries `data-dx-check-package-lane-template="ai/vercel-ai"`, `data-dx-check-package-lane-row="ai/vercel-ai"`, official package naming, upstream `ai` `7.0.0-canary.146` provenance from `G:/WWW/inspirations/vercel-ai`, the launch assistant receipt path, `data-dx-check-package-lane-dx-style-status="present"`, `data-dx-style-surface="ai-sdk"`, and `data-dx-token-scope="ai/vercel-ai"` without claiming live model streaming or browser proof. Upstream source re-inspected from `packages/ai/package.json`, `packages/ai/src/index.ts`, `packages/ai/src/generate-text/stream-text.ts`, and `packages/ai/src/ui/default-chat-transport.ts`; public APIs used remain `streamText`, `DefaultChatTransport`, `gateway`, `createGateway`, `createProviderRegistry`, and `tool`. Files changed: `benchmarks/vercel-ai-dx-check-package-lane-panel.test.ts`, `examples/template/runtime-pages/index.html`, `docs/packages/ai-vercel-ai.md`, AI SDK receipt/package-status/read-model hashes, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green AI SDK static package-lane marker guard, focused AI SDK Node guard bundle, syntax/JSON checks, and scoped hygiene checks. Heavy checks skipped: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo while shared reader drift remains active, live provider/model streaming, and browser visual proof. Exact next action: add a generated-starter materialization guard proving the AI SDK static package-lane marker survives `dx new` output before rerunning shared Rust fixtures.`
- `DX.md:2428:- Type-Safe API static launch package-lane template, 2026-05-22: lane 8 now exposes official **Type-Safe API** (`api/trpc`) in the receipt-less `/launch` dx-check panel with `data-dx-check-package-lane-template="api/trpc"`, `data-dx-check-package-lane-row="api/trpc"`, upstream `@trpc/server` `11.17.0`, source mirror `G:/WWW/inspirations/trpc`, dashboard workflow receipt path, and `type-safe-api:receipt-hash-refresh` helper markers. The launch shell, static runtime page, DX Studio edit contract, runtime materializer, and Rust Studio manifest include `api/trpc` in the dx-check panel package filter so Studio/Zed can discover helper freshness before a fresh dx-check receipt exists. Upstream source re-inspected from `G:/WWW/inspirations/trpc`: `packages/server/package.json`, `packages/server/src/unstable-core-do-not-import/initTRPC.ts`, `packages/server/src/adapters/fetch/fetchRequestHandler.ts`, `packages/client/src/createTRPCClient.ts`, `packages/client/src/links/httpBatchLink.ts`, and `packages/react-query/src/createTRPCReact.tsx`; public APIs used remain `initTRPC.context().create()`, `createCallerFactory`, `fetchRequestHandler`, `createTRPCClient`, `httpBatchLink`, and `createTRPCReact`. Files changed: `benchmarks/trpc-dx-check-package-lane-panel.test.ts`, `examples/template/launch-shell.tsx`, `examples/template/runtime-pages/index.html`, `examples/template/dx-studio-edit-contract.ts`, `tools/launch/materialize-www-template.ts`, `dx-www/src/cli/studio_manifest.rs`, `docs/packages/api-trpc.md`, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Type-Safe API package-lane guard, focused helper/read-model/dx-check guard bundle, benchmark syntax check, helper `--check --json`, package-status JSON parse, scoped hygiene checks, and `rustfmt --edition 2024 --check ./dx-www/src/cli/studio_manifest.rs`. Heavy checks skipped: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, and live tRPC route execution/client/server/auth/persistence proof. Exact next action: add a generated-starter materialization guard proving the Type-Safe API static package-lane template survives `dx new` output before rerunning shared Cargo panel fixtures.`
- `DX.md:2446:- Automation Connectors static DX Studio/check-panel package row, 2026-05-22: lane 19 now exposes official **Automation Connectors** (`automations/n8n`) directly in the static `/launch` dx-check panel. `examples/template/runtime-pages/index.html` now carries `data-dx-check-package-lane-row="automations/n8n"`, official package naming, upstream provenance `n8n-nodes-base` `2.22.0` from `G:/WWW/inspirations/n8n/packages/nodes-base`, the launch workflow receipt path, `data-dx-check-package-lane-dx-style-status="present"`, hash-refresh helper markers, `data-dx-style-surface="automation-connectors"`, and `data-dx-token-scope="automations/n8n"` without claiming live n8n runtime proof. `examples/template/dx-studio-edit-contract.ts`, `tools/launch/materialize-www-template.ts`, and `dx-www/src/cli/studio_manifest.rs` now include `automations/n8n` in the `dx-check-health-panel` package filter and package-surface marker index so Zed/DX Studio can find the row before a fresh dx-check receipt is loaded. Upstream source re-inspected from `G:/WWW/inspirations/n8n/packages/nodes-base`: `package.json`, `nodes/ManualTrigger/ManualTrigger.node.ts`, `nodes/Slack/Slack.node.ts`, and `nodes/Slack/V2/SlackV2.node.ts`; APIs used for this proof are `INodeType`, `INodeTypeDescription`, `IVersionedNodeType`, and `IExecuteFunctions`. Files changed: `benchmarks/automations-dx-check-package-lane-panel.test.ts`, the static launch runtime page, Studio edit contract, runtime materializer, Rust Studio manifest, Automation Connectors docs/receipt hashes, `DX.md`, `TODO.md`, and `CHANGELOG.md`. Verification scope: red/green Automation Connectors package-lane panel guard and package helper `--check --json` after `--write`. Heavy checks skipped: full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo, browser visual proof, and live Automation Connectors provider execution. Exact next action: add a generated-starter materialization guard proving the Automation Connectors row survives `dx new` output before rerunning shared Cargo panel fixtures.`

### Pattern: `dx build`
- `AGENTS.md:29:- `dx build`.`
- `binary/Cargo.toml:56:server = []                              # Only for dx build tool`
- `benchmarks/app-router-page-extensions-build-loop.test.mjs:74:test("dx build App Router page discovery accepts Next-familiar page extensions and src/app roots", () => {`
- `binary/src/lib.rs:16://! Server (dx build)              Network              Client (dx-www-runtime)`
- `binary/src/lib.rs:72://! ### Server-side (in dx build tool):`
- `binary/src/serializer.rs:5://! This runs in the dx build tool, not in the browser.`
- `benchmarks/app-router-server-data-build-contract.test.ts:354:  assert.match(serverDataProofTest, /project_cli/.cmd_build/(/)/.expect/("dx build"/)/);`
- `CHANGELOG.md:14:  power `dx build` or `dx dev`.`
- `CHANGELOG.md:37:- Extended Next-familiar `dx build` fixture coverage to `src/app` and`
- `CHANGELOG.md:136:  Cargo/rustc work, and no `dx build` integration or external bundler runtime adoption is`
- `CHANGELOG.md:209:  touched-file rustfmt; no `dx build` cache integration, Cargo proof, or`
- `CHANGELOG.md:227:  no live AI provider, browser, `dx build`, or Turbopack proof is claimed.`
- `CHANGELOG.md:237:- Published route-level server-data summaries in the top-level `dx build``
- `CHANGELOG.md:277:- Tightened the installed `dx build` smoke output contract for source-module`
- `CHANGELOG.md:292:  browser, `dx build`, or Turbopack proof is claimed.`
- `CHANGELOG.md:321:  `dx build`, or external bundler runtime adoption is claimed.`
- `CHANGELOG.md:382:  provider billing, tool execution, browser proof, `dx build` proof, or`
- `CHANGELOG.md:421:- Tightened installed `dx build` smoke proof for source-build server-data`
- `CHANGELOG.md:438:  no live provider execution, browser proof, `dx build` proof, or Turbopack`
- `CHANGELOG.md:824:  no installed-binary refresh, live `dx build`, server, browser, broad build,`
- `CHANGELOG.md:993:- Hardened the installed `dx build` smoke deploy-adapter route-handler proof.`
- `CHANGELOG.md:1002:  refresh, live `dx build`, browser/server proof, broad builds, and deploys`
- `CHANGELOG.md:1073:- Hardened the installed `dx build` smoke route-handler contract. The tiny`
- `CHANGELOG.md:1081:  installed-binary refresh, live `dx build` product proof, servers, browser`
- `CHANGELOG.md:1170:- Hardened the installed `dx build` smoke around App Router execution`
- `CHANGELOG.md:1239:- Hardened the installed `dx build` smoke around server-data route contracts.`
- `CHANGELOG.md:1315:  Cargo builds, local servers, browser automation, deploys, full `dx build``
- `CHANGELOG.md:1336:- Tightened the installed `dx build` smoke around graph evidence. The smoke`

### Pattern: `dx dev`
- `AGENTS.md:28:- `dx dev`.`
- `CHANGELOG.md:14:  power `dx build` or `dx dev`.`
- `CHANGELOG.md:2626:- Split App Router route matching into `dx-www/src/cli/app_api_routes.rs` and `dx-www/src/cli/app_page_routes.rs`, reducing the giant CLI module's ownership of dynamic API/page lookup while preserving the existing `dx dev` request path.`
- `core/README.md:338:dx dev --entry pages --port 3000 ````
- `docs/DX_WWW_MANAGER_HANDOFF.md:13:  `examples/template`, `dx dev --host 127.0.0.1 --port 3000`, HTTP `/`,`
- `docs/dx-www-developer-contract.md:37:The starter runs with `dx dev` and does not require `npm install` or `node_modules`. The dev path now compiles the starter through the React-shaped App Route compiler slice, producing a canonical `DxPageGraph`, crawlable fallback HTML, JS interaction output for the supported state/button subset, and a round-tripped `DXPK` packet.`
- `docs/build-graph-model.md:127:The previous Turbo Tasks executor, execution handoff, and Zed execution panel artifacts are removed from the active graph CLI. DX-WWW is not adopting real Turbopack or Turbo Tasks runtime/build execution; those upstream pieces are reference/provenance only. The allowed Lane 2 surface is now adapter/diff/status evidence that can inform a future DX-owned cache runner without executing upstream scheduler semantics, opening Turbo Persistence, or making Turbopack part of `dx build` or `dx dev`.`
- `docs/getting-started.md:55:dx dev ````
- `docs/root-workspace-todo.md:520:- [x] Execute `app/api/**/route.ts` handlers in `dx dev` through a safe DX-WWW server runtime with typed request/response serialization.`
- `docs/root-workspace-todo.md:532:- [x] Add server-action POST endpoints in `dx dev` and preview using the compiled action protocol receipts.`
- `docs/superpowers/plans/2026-05-28-dx-devtools-framework-integration.md:5:Connect the existing DX Devtools surface into DX WWW at the framework `dx dev` layer so every project receives the dev-only runtime automatically, while `dx build` remains free of Devtools assets, routes, and production abstractions.`
- `benchmarks/cli-dev-options-split-safety.test.ts:19:test("dx dev option parsing and listener binding live outside the giant cli module", () => {`
- `benchmarks/cli-dev-options-split-safety.test.ts:40:  assert.ok(devOptionsUse, "dx dev command should import dev option helpers from dev_options");`
- `benchmarks/cli-dev-options-split-safety.test.ts:52:    "dx dev must parse user options before binding the listener",`
- `benchmarks/cli-help-text-split-safety.test.ts:73:    "dx dev help should reuse shared help-argument detection",`
- `benchmarks/cli-help-text-split-safety.test.ts:78:    "dx dev should not carry a stale inline help matcher",`
- `benchmarks/cli-help-text-split-safety.test.ts:116:    "dx dev --host 127.0.0.1 --port 3000 --no-hot-reload",`
- `worker-lanes/WWW_30_AGENT_FINAL_POLISH_PROMPT.md:144:- Verify `dx dev` or existing server.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:35:- `dx dev --host 127.0.0.1 --port 3000`: passed, running PID 18900.`
- `worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md:209:- Start or probe `dx dev` safely.`
- `DX.md:12:  DX-WWW goal, and Turbopack does not power `dx build` or `dx dev`.`
- `DX.md:263:  live `dx dev` server run. Verification: red/green`
- `DX.md:2813:- Receipt policy: receipts and provenance stay under `.dx/` and should be surfaced through `dx check`, Studio/Web Preview, and actionable error states. The happy path should remain `dx new`, `dx dev`, `dx check`, and `dx deploy`.`
- `examples/todo-app/README.md:16:cd examples/todo-app dx dev ````
- `examples/template/package.json:7:    "dev": "dx dev",`
- `TODO.md:13:  Turbopack does not power `dx build` or `dx dev`.`
- `TODO.md:159:  `dx dev` diagnostic receipts and rerun the focused Cargo test after the`
- `TODO.md:211:  compilation after the shared Rust queue is stable, plus live `dx dev` page`

### Pattern: `dx check`
- `AGENTS.md:12:  - `dx check examples/template --json` -> `100/100`, `green`.`
- `AGENTS.md:36:- `dx check` and agent-facing receipts.`
- `AGENTS.md:96:dx check examples/template --json`
- `AGENTS.md:118:dx check examples/template --json`
- `AGENTS.md:171:Do not claim `100/100` unless `dx check examples/template --json` is actually`
- `CHANGELOG.md:1504:- Hardened DX Build product proof. The readiness gate now consumes `dx check launch --json` as `receipts.checkLaunch`, requires approved launch proof before product readiness, and rejects passing smoke from candidate override binaries as installed-release proof. The installed-smoke receipt now records `binaryRole`, `binaryDefault`, and `binaryOverride`; live default installed smoke remains stale with 11 blockers, while the debug candidate passes but is labeled `candidate-override`. Guarded with red/green installed-smoke and readiness-gate Node tests; no release rebuild, server, install, deploy, or binary promotion ran.`
- `CHANGELOG.md:1542:- Tightened the DX Build readiness-gate runtime-proof action contract. `run-governed-runtime-proof` now points its receipt metadata at `.dx/receipts/check/check-latest.json`, matching the `dx check launch --json` command it asks operators to run, and the consumer snapshot carries the same receipt path plus explicit `consumers.friday` flags and a top-level Friday `requiredActions` consumer row for DX CLI/DX-WWW/Zed/Friday consumers. Guarded with red/green `node --test ./benchmarks/dx-build-readiness-gate.test.ts`; no builds, binaries, installs, servers, deploys, or runtime proof ran.`
- `CHANGELOG.md:1894:- Added the Forge root-dx golden path dashboard row. The launch template now includes a root extensionless `dx` manifest for `shadcn/ui/button`, a real local filesystem registry publish under `.dx/forge/registry/local`, selected visible install/update receipts for the `#button` export, `dx forge status --json` evidence, a typed `forge-golden-path-contract.ts`, a React `forge-golden-path-panel.tsx`, and static `/launch` markers so Studio can inspect the lifecycle before hydration. `core/src/ecosystem/forge_security.rs` now prefers local-registry content for `dx-forge/local-registry` rollback receipts, so remove/write plus rollback/write can restore a selected visible file after source-manifest tracking is removed. Guarded with the prebuilt `dx-www.exe` publish/add/update/remove/status smokes, `dx check --json`, `dx run --test ./benchmarks/forge-golden-path-launch-proof.test.ts`, and targeted `cargo test -q -p dx-www-compiler forge_local_registry_remove_receipt_rolls_back_from_registry_content --lib`. Demanding truth: the dx-check smoke is currently 66/100 red because other package-lock hashes are stale, and update dry-run still does not persist a receipt. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live launch-template remove write were skipped. Next action: persist dry-run receipts and repair stale package-lock lanes so dx-check can turn green.`
- `CHANGELOG.md:2230:- Added Database ORM Rust dx-style compatibility checks. The lane 9 package `db/drizzle-sqlite` keeps official **Database ORM** naming while `core/src/ecosystem/project_check/database_orm_dx_check.rs` now consumes `dx.forge.package.dx_style_compatibility` from package-status and emits `database_orm_dx_style_compatibility_present`, `database_orm_dx_style_compatibility_missing`, and `database-orm-missing-dx-style-compatibility`. The existing Rust package-status fixture now carries the positive source-owned style evidence so `dx check` proves the present path beside receipt and hash metrics without claiming live SQLite visual proof. Re-inspected upstream Drizzle SQLite DB and better-sqlite3 driver sources for `withReplicas`, `selectDistinct`, `$count`, and `drizzle`, and re-inspected `core/src/ecosystem/forge_drizzle.rs`. Guarded with red/green `dx run --test ./benchmarks/drizzle-package-status-read-model.test.ts`, syntax check, Database ORM receipt and replica-routing guards, touched Database ORM module rustfmt, and targeted `cargo test -q -p dx-www-compiler dx_check_reports_database_orm_package_status_visibility --lib` with only the existing unrelated Three Scene warning. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, live SQLite/replica proof, and browser visual proof were skipped; `rustfmt --check --config skip_children=true ./core/src/ecosystem/project_check.rs` remains blocked by unrelated pre-existing dx-style formatting drift near `mask-radial-[100%_100%]`. Next action: add a tiny Database ORM Rust fixture that removes `dx_style_compatibility` from a temporary package-status row and proves the missing metric plus finding flip together.`
- `CHANGELOG.md:2280:- Added Database ORM dx-style compatibility visibility. The lane 9 package `db/drizzle-sqlite` now keeps official **Database ORM** naming while exposing `dx.forge.package.dx_style_compatibility` for the visible launch read-model workflow. `LaunchDrizzleDashboardData` carries `data-dx-style-surface="database-orm"`, the Forge metadata and launch package catalog publish the same compatibility contract, and the dashboard receipt/package-status/read-model row adds `database_orm_dx_style_compatibility_present` plus `database_orm_dx_style_compatibility_missing` without claiming browser visual proof. Re-inspected upstream Drizzle `sqlite-core/db.ts` and `better-sqlite3/driver.ts` for `withReplicas`, `selectDistinct`, `$count`, and `drizzle`. Guarded with red/green Database ORM package-status and receipt visibility tests, the replica-routing guard, syntax checks, package-status/receipt JSON parse, and touched-module Rust formatting. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, live SQLite/replica runtime proof, and browser visual proof were skipped. Next action: wire Database ORM dx-style compatibility present/missing metrics into Rust `dx check` findings.`
- `CHANGELOG.md:2320:- Added AI SDK Rust dx-check output. The lane 16 package `ai/vercel-ai` now maps official **AI SDK** naming, upstream `ai` `7.0.0-canary.146` provenance, selected package-status/receipt evidence, and `ai_sdk_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/ai_sdk_dx_check.rs` and `core/src/ecosystem/project_check.rs`. It emits `ai_sdk_package_present`, `ai_sdk_receipt_present`, `ai_sdk_receipt_stale`, `ai_sdk_missing_receipt`, `ai_sdk_blocked_surface`, `ai_sdk_unsupported_surface`, `ai_sdk_hash_manifest_present`, and `ai_sdk_hash_mismatch`, with findings for missing package-status, missing receipts, stale receipts, blocked provider/runtime boundaries, unsupported surfaces, and stale hash-backed materialized files. The checker compares SHA-256 bytes for front-facing app files while treating upstream/core/docs hashes as provenance metadata, and it does not claim live model streaming proof. Re-inspected upstream AI SDK package metadata, `streamText`, `DefaultChatTransport`, `convertToModelMessages`, and `createProviderRegistry` sources. Guarded with red/green `cargo test -q -p dx-www-compiler dx_check_reports_ai_sdk_package_status_visibility --lib`; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live AI provider/model runtime proof were skipped. Next action: extract the repeated Rust package-status receipt/hash comparator into a shared checker utility and move AI SDK onto it.`
- `CHANGELOG.md:2342:- Wired Reactive Store into Rust dx-check output with byte-derived hash freshness. The lane 4 package `reactive/store` now maps official **Reactive Store** naming, upstream `@tanstack/store` / `@tanstack/react-store` `0.11.0` provenance, selected package-status/receipt evidence, SHA-256 hash-manifest visibility, and `reactive_store_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/reactive_store_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, unsupported, and hash mismatch states now produce `reactive-store-*` findings; hash-backed selected surfaces compare current file bytes instead of only checking for file existence, while live React runtime proof remains explicitly unclaimed. Re-inspected upstream React Store `createStoreContext.tsx`, React Store `index.ts`, and Store package manifests for `createStoreContext`, `StoreProvider`, `useStoreContext`, and the `@tanstack/store` export boundary. Guarded with red/green `dx run --test ./benchmarks/reactive-store-dx-check-output.test.ts`, `dx run --check ./benchmarks/reactive-store-dx-check-output.test.ts`, existing Reactive Store package-status and slice guards, touched-module Rust formatting, and scoped hygiene checks. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, and live React runtime proof were skipped. Next action: add a small Rust fixture that mutates a temporary Reactive Store hash-backed file and proves `reactive_store_hash_mismatch` flips without a broad Cargo suite.`
- `CHANGELOG.md:2344:- Added Documentation System Rust dx-check output. The lane 14 package `content/fumadocs-next` now maps official **Documentation System** naming, upstream `fumadocs` `16.8.12` provenance, selected package-status/receipt evidence, SHA-256 source-hash freshness, dx-style compatibility state, and `documentation_system_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/documentation_system_dx_check.rs` and `core/src/ecosystem/project_check.rs`. It publishes `documentation_system_package_present`, `documentation_system_receipt_present`, `documentation_system_receipt_stale`, `documentation_system_missing_receipt`, `documentation_system_blocked_surface`, `documentation_system_unsupported_surface`, `documentation_system_hash_manifest_present`, `documentation_system_hash_mismatch`, `documentation_system_dx_style_compatibility_present`, and `documentation_system_dx_style_compatibility_missing`, and emits `documentation-system-missing-package-status`, `documentation-system-missing-receipt`, `documentation-system-stale-receipt`, `documentation-system-blocked-surface`, `documentation-system-unsupported-surface`, `documentation-system-hash-mismatch`, and `documentation-system-missing-dx-style-compatibility` findings without claiming live Fumadocs renderer runtime proof. Re-inspected upstream core package metadata, loader, breadcrumb, LLM export, Orama search server/client, and OpenAPI server entrypoints for `loader`, `getBreadcrumbItems`, `llms`, `createFromSource`, `useDocsSearch`, and `createOpenAPI`; refreshed the Documentation System dashboard receipt, package-status row, and read-model hashes for the current package catalog and package doc bytes. Guarded with red/green `dx run --test ./benchmarks/fumadocs-dx-check-output.test.ts`, the existing Documentation System dashboard workflow guard, syntax/JSON checks, touched-module Rust formatting, and scoped diff/conflict checks; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, and live Fumadocs renderer/search/OpenAPI runtime proof were skipped. Next action: extract the repeated Rust package-status receipt/hash/dx-style helper pattern into a shared checker utility after one more lane proves the shape.`
- `CHANGELOG.md:2346:- Added Backend Platform Client Rust dx-check output. The lane 10 package `supabase/client` now maps official **Backend Platform Client** naming, upstream `@supabase/ssr + @supabase/supabase-js` provenance, selected package-status/receipt evidence, hash-manifest visibility, and `backend_platform_client_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/backend_platform_client_dx_check.rs` and `core/src/ecosystem/project_check.rs`. It publishes `backend_platform_client_package_present`, `backend_platform_client_receipt_present`, `backend_platform_client_receipt_stale`, `backend_platform_client_missing_receipt`, `backend_platform_client_blocked_surface`, `backend_platform_client_unsupported_surface`, `backend_platform_client_hash_manifest_present`, and `backend_platform_client_hash_mismatch`, and emits `backend-platform-client-missing-package-status`, `backend-platform-client-missing-receipt`, `backend-platform-client-stale-receipt`, `backend-platform-client-blocked-surface`, `backend-platform-client-unsupported-surface`, and `backend-platform-client-hash-mismatch` findings without claiming hosted Supabase runtime proof. Re-inspected upstream user-management account/profile source, SSR browser/server client examples, and Studio Project API docs for `createBrowserClient`, `createServerClient`, `auth.getUser`, `from('profiles').select`, and `from('profiles').upsert`. Guarded with red/green `dx run --test ./benchmarks/supabase-dx-check-output.test.ts`, focused Supabase package-status/dashboard guards, syntax/JSON checks, scoped Rust formatting, and scoped diff/conflict checks; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, governed browser QA, and hosted Supabase reads/writes/auth/realtime proof were skipped. Next action: add actual SHA-256 byte comparison inside Rust dx-check for Backend Platform Client selected files.`
- `CHANGELOG.md:2348:- Wired Markdown & MDX Content into Rust dx-check output. The lane 15 package `content/react-markdown` now maps official **Markdown & MDX Content** naming, upstream `react-markdown` and `mdx` provenance, selected package-status/receipt evidence, and `markdown_mdx_content_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/markdown_mdx_content_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, and unsupported states now produce `markdown-mdx-content-*` findings while keeping live Markdown/MDX renderer proof explicitly unclaimed. Re-inspected upstream React Markdown and MDX entrypoints for `Markdown`, `MarkdownAsync`, `MarkdownHooks`, `defaultUrlTransform`, `MDXProvider`, `useMDXComponents`, `compile`, `compileSync`, `createProcessor`, and `nodeTypes`. Guarded with red/green `dx run --test ./benchmarks/markdown-mdx-content-slice.test.ts`, syntax check, touched-module Rust formatting, and scoped diff/conflict checks; targeted Cargo proof was attempted but blocked by unrelated shared-tree/module and concurrent Cargo activity. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live Markdown/MDX runtime proof were skipped. Next action: compare Markdown & MDX Content receipt file hashes in Rust dx-check so stale selected surfaces are byte-derived.`
- `CHANGELOG.md:2350:- Wired UI Components into Rust dx-check output. The lane 20 package `shadcn/ui/button` now maps official **UI Components** naming, upstream `shadcn-ui` `0.0.1` plus Radix primitive provenance, selected package-status/receipt evidence, and metrics `ui_components_package_present`, `ui_components_receipt_present`, `ui_components_receipt_stale`, `ui_components_missing_receipt`, `ui_components_blocked_surface`, and `ui_components_unsupported_surface` into the core Forge `dx check` section through `core/src/ecosystem/project_check/ui_components_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, and unsupported states now produce `ui-components-missing-package-status`, `ui-components-missing-receipt`, `ui-components-stale-receipt`, `ui-components-blocked-surface`, and `ui-components-unsupported-surface` findings from `.dx/forge/package-status.json`, selected surfaces, and the dashboard controls receipt while keeping the lane `SOURCE-ONLY` and not claiming browser UI runtime proof. Re-inspected upstream shadcn package metadata, v4 `button.tsx`, `label.tsx`, `separator.tsx`, and Radix `slot.tsx` for `Button`, `buttonVariants`, `Slot.Root`, `Label`, `Separator`, `createSlot`, and primitive `Root` exports. Guarded with red/green `dx run --test ./benchmarks/ui-components-dx-check-output.test.ts`, `dx run --check ./benchmarks/ui-components-dx-check-output.test.ts`, targeted Rust `cargo test -q -p dx-www-compiler dx_check_reports_ui_components_package_status_visibility`, focused UI Components package-status/doc guards, touched-module `rustfmt --check`, and scoped hygiene checks; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, full workspace Cargo, and live browser UI proof were skipped. Next action: add SHA-256 `file_hashes` to the UI Components dashboard controls receipt and compare them for byte-derived stale detection.`
- `CHANGELOG.md:2354:- Wired Type-Safe API into Rust dx-check output. The lane 8 package `api/trpc` now maps official **Type-Safe API** naming, upstream `@trpc/server` `11.17.0` provenance, selected package-status/receipt evidence, and `type_safe_api_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/type_safe_api_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, and unsupported-surface states now produce `type-safe-api-*` findings from `.dx/forge/package-status.json`, launch-template receipt paths, and selected-surface state without claiming live tRPC route execution. Re-inspected upstream tRPC manifests plus `initTRPC`, fetch adapter, typed client, HTTP batch link, and React Query hook factory sources for `initTRPC.context().create()`, `createCallerFactory`, `fetchRequestHandler`, `createTRPCClient`, `httpBatchLink`, and `createTRPCReact`. Guarded with red/green `dx run --test ./benchmarks/trpc-rust-dx-check-metrics.test.ts`, `dx run --check ./benchmarks/trpc-rust-dx-check-metrics.test.ts`, `dx run --test ./benchmarks/trpc-package-status-read-model.test.ts`, `dx run --test ./benchmarks/trpc-dx-check-visibility-receipt.test.ts`, targeted `cargo test -q -p dx-www-compiler dx_check_reports_type_safe_api_package_status_visibility`, and scoped hygiene checks. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live Type-Safe API runtime proof were skipped. Next action: add SHA-256 `file_hashes` to the Type-Safe API dashboard workflow receipt and compare them in Rust dx-check for byte-derived stale detection.`
- `CHANGELOG.md:2360:- Wired Motion & Animation into Rust dx-check output. The lane 13 package `animation/motion` now maps official **Motion & Animation** naming, upstream `motion` `12.38.0` provenance, selected package-status/receipt evidence, SHA-256 hash-manifest visibility, and `motion_animation_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/motion_animation_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, unsupported, and hash-manifest mismatch states now produce `motion-animation-*` findings while keeping live browser animation proof explicitly unclaimed. Re-inspected upstream `packages/motion/src/react.ts`, `packages/framer-motion/src/index.ts`, `packages/framer-motion/src/value/use-scroll.ts`, `packages/framer-motion/src/components/Reorder/Item.tsx`, `packages/framer-motion/src/motion/features/definitions.ts`, and `packages/framer-motion/package.json` for `MotionConfig`, `motion`, `m`, `LazyMotion`, `domAnimation`, `useScroll`, `useReducedMotion`, `useAnimate`, and `Reorder.Item`. Guarded with red/green `dx run --test ./benchmarks/motion-dx-check-output.test.ts`, the focused Motion package-status read-model guard, syntax check, touched-module Rust formatting, and scoped diff/conflict checks; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, and live Motion browser animation proof were skipped. Next action: add actual SHA-256 digest comparison inside Rust dx-check so `motion_animation_hash_mismatch` is byte-derived.`
- `CHANGELOG.md:2364:- Wired Internationalization into Rust dx-check output. The lane 7 package `i18n/next-intl` now maps official **Internationalization** package-status evidence into the Forge `dx check` section through `core/src/ecosystem/project_check/internationalization_dx_check.rs` and the shared `project_check.rs` integration hook. It emits `internationalization_package_present`, `internationalization_receipt_present`, `internationalization_receipt_stale`, `internationalization_missing_receipt`, `internationalization_blocked_surface`, and `internationalization_unsupported_surface`, and raises `internationalization-missing-package-status`, `internationalization-missing-receipt`, `internationalization-stale-receipt`, `internationalization-blocked-surface`, and `internationalization-unsupported-surface` without claiming live locale routing proof. Re-inspected upstream `next-intl` `4.12.0` package metadata, `NextIntlClientProvider`, React hooks, middleware, and the current Forge slice for `NextIntlClientProvider`, `useTranslations`, `useLocale`, `useFormatter`, `defineRouting`, `createNavigation`, `getRequestConfig`, and `createMiddleware`. Guarded with red/green `dx run --test ./benchmarks/next-intl-dx-check-output.test.ts`, targeted Rust `cargo test -q -p dx-www-compiler dx_check_reports_internationalization_package_status_visibility --lib`, and scoped formatting/diff/conflict checks. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, governed browser locale routing proof, SEO alternate-link review, and runtime dependency proof were skipped. Next action: add SHA-256 source/file hashes to the Internationalization dashboard workflow receipt so stale selected surfaces can be derived from file evidence.`
- `CHANGELOG.md:2366:- Added Forms Rust dx-check output. The lane 6 package `forms/react-hook-form` now maps official **Forms** naming, upstream `react-hook-form` `7.75.0` provenance, selected package-status/receipt evidence, and `forms_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/forms_dx_check.rs` and `core/src/ecosystem/project_check.rs`. It publishes `forms_package_present`, `forms_receipt_present`, `forms_receipt_stale`, `forms_missing_receipt`, `forms_blocked_surface`, and `forms_unsupported_surface`, and emits `forms-missing-package-status`, `forms-missing-receipt`, `forms-stale-receipt`, `forms-blocked-surface`, and `forms-unsupported-surface` findings without claiming browser submission proof. Re-inspected upstream `src/index.ts`, `src/useForm.ts`, `src/useFormContext.tsx`, `src/controller.tsx`, `src/useController.ts`, `src/useFieldArray.ts`, and `src/types/form.ts` for `useForm`, `FormProvider`, `useFormContext`, `register`, `handleSubmit`, `Controller`, `useController`, `useFieldArray`, `Resolver`, and `FieldErrors`. Guarded with red/green `dx run --test ./benchmarks/forms-dx-check-output.test.ts`, `dx run --check ./benchmarks/forms-dx-check-output.test.ts`, focused Forms source guards, scoped Rust formatting, and scoped diff/conflict checks; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live Forms browser submission proof were skipped. Next action: add SHA-256 `file_hashes` to the Forms dashboard workflow receipt and compare them for hash-derived stale detection.`
- `CHANGELOG.md:2368:- Wired 3D Scene System into Rust dx-check output. The lane 17 package `3d/launch-scene` now maps official **3D Scene System** naming, upstream `three + @react-three/fiber + @react-three/drei` provenance, selected package-status/receipt evidence, SHA-256 hash-manifest visibility, and `three_scene_system_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/three_scene_system_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, unsupported, and hash-manifest mismatch states now produce `three-scene-system-*` findings while keeping live browser/WebGL proof explicitly unclaimed. Re-inspected upstream Three `WebGLRenderer`, `Raycaster`, and `Box3`; React Three Fiber `createRoot`, `RootState`, `frameloop`, `setDpr`, and `state.raycaster`; and Drei `KeyboardControls`, `Bounds.fit`, `PerformanceMonitor`, `AdaptiveDpr`, and `meshBounds`. Guarded with red/green `dx run --test ./benchmarks/three-scene-dx-check-output.test.ts`, `dx run --check ./benchmarks/three-scene-dx-check-output.test.ts`, `dx run --test ./benchmarks/three-scene-package-doc.test.ts`, lane-specific launch package-status guard, package-status/receipt JSON parse, and scoped `rustfmt --check`; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, Cargo compilation, and live WebGL runtime proof were skipped. Next action: add actual SHA-256 digest comparison inside Rust dx-check so `three_scene_system_hash_mismatch` is byte-derived.`
- `CHANGELOG.md:2370:- Wired Data Fetching & Cache into Rust dx-check output. The lane 3 package `tanstack/query` now maps official **Data Fetching & Cache** naming, upstream `@tanstack/react-query` `5.100.10` provenance, selected package-status/receipt evidence, and metrics `data_fetching_cache_package_present`, `data_fetching_cache_receipt_present`, `data_fetching_cache_receipt_stale`, `data_fetching_cache_missing_receipt`, `data_fetching_cache_blocked_surface`, and `data_fetching_cache_unsupported_surface` into the core Forge `dx check` section through `core/src/ecosystem/project_check/data_fetching_cache_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, and unsupported states now produce `data-fetching-cache-missing-package-status`, `data-fetching-cache-missing-receipt`, `data-fetching-cache-stale-receipt`, `data-fetching-cache-blocked-surface`, and `data-fetching-cache-unsupported-surface` findings from package-status rows and the dashboard workflow receipt while keeping query keys, fetchers, network execution, persistence, broadcast policy, and runtime proof app-owned without claiming live QueryClient runtime proof. Re-inspected upstream `package.json` and `packages/query-core/src/queryClient.ts` for `setQueryDefaults`, `getQueryDefaults`, `invalidateQueries`, `ensureQueryData`, `prefetchQuery`, and `cancelQueries`. Guarded with red/green `dx run --test ./benchmarks/tanstack-query-dx-check-output.test.ts`, `dx run --check ./benchmarks/tanstack-query-dx-check-output.test.ts`, and scoped Rust formatting/diff/conflict checks; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live QueryClient runtime proof were skipped. Next action: add SHA-256 `file_hashes` to the Data Fetching & Cache dashboard workflow receipt and compare them for hash-derived stale detection.`
- `CHANGELOG.md:2372:- Wired Automation Connectors into Rust dx-check output. The lane 19 package `automations/n8n` now maps official **Automation Connectors** naming, upstream `n8n-nodes-base` `2.22.0` provenance, selected package-status/receipt evidence, and `automation_connectors_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/automation_connectors_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, and unsupported states now produce `automation-connectors-*` findings from package-status rows and referenced receipts while keeping credential storage, provider accounts, and live workflow execution app-owned. Re-inspected upstream `packages/nodes-base/package.json`, `nodes/Slack/Slack.node.ts`, and `credentials/SlackApi.credentials.ts` for `VersionedNodeType`, `INodeTypeBaseDescription`, `IVersionedNodeType`, `ICredentialType`, `IAuthenticateGeneric`, and `ICredentialTestRequest`. Guarded with red/green `cargo test -q -p dx-www-compiler dx_check_reports_automation_connectors_package_status_visibility`, `dx run --test ./benchmarks/automations-package-status-read-model.test.ts`, `dx run --check ./benchmarks/automations-package-status-read-model.test.ts`, and touched-module rustfmt; a broader root rustfmt check was attempted and blocked by pre-existing sibling-lane formatting drift in `forms_dx_check.rs` and `three_scene_system_dx_check.rs`. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live Automation Connectors provider execution were skipped. Next action: add and compare Automation Connectors receipt file hashes so dx-check can compute stale selected surfaces from source drift.`
- `CHANGELOG.md:2374:- Wired AI SDK into the shared package-status read model. The lane 16 package `ai/vercel-ai` now exposes official **AI SDK** naming, upstream `ai` `7.0.0-canary.146` provenance from `G:/WWW/inspirations/vercel-ai`, selected `ai-chat-route`, `ai-dashboard-readiness`, `ai-dashboard-assistant-component`, and `ai-launch-assistant-dashboard-workflow` surfaces, `ai_sdk_*` dx-check metrics including hash manifest/mismatch signals, and Zed/DX Studio surfaces `ai-sdk:chat-route`, `ai-sdk:dashboard-readiness`, `ai-sdk:dashboard-assistant-component`, and `ai-sdk:launch-assistant-dashboard-workflow` through `examples/template/.dx/forge/package-status.json`, `examples/template/forge-package-status-read-model.ts`, `examples/template/forge-package-status.ts`, the AI SDK launch assistant receipt, and `docs/packages/ai-vercel-ai.md`. Re-inspected upstream AI SDK stream, UI transport, and provider registry sources for `streamText`, `convertToModelMessages`, `toUIMessageStreamResponse`, `tool`, `DefaultChatTransport`, and `createProviderRegistry`. Guarded with red/green `dx run --test ./benchmarks/vercel-ai-package-status-read-model.test.ts`, `dx run --check ./benchmarks/vercel-ai-package-status-read-model.test.ts`, and the focused AI SDK guard bundle. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live model/provider runtime proof were skipped. Next action: feed `ai_sdk_*` package-status metrics into Rust `dx check` JSON/finding output without changing the app-owned runtime boundary.`
- `CHANGELOG.md:2376:- Wired Payments into Rust dx-check output. The lane 12 package `payments/stripe-js` now maps official **Payments** naming, upstream `@stripe/stripe-js` `9.6.0` provenance, selected package-status/receipt evidence, and `payments_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/payments_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, and unsupported states now produce `payments-*` findings from package-status rows and the referenced billing workflow receipt while keeping Stripe credentials, Price IDs, webhooks, entitlements, and live Checkout execution app-owned. Re-inspected upstream `package.json`, `README.md`, `src/pure.ts`, `src/shared.ts`, `types/stripe-js/stripe.d.ts`, and `types/stripe-js/checkout.d.ts` for `loadStripe`, `loadStripe.setLoadParameters`, `stripe.confirmPayment`, `stripe.retrievePaymentIntent`, `stripe.createEmbeddedCheckoutPage`, and `StripeEmbeddedCheckoutOptions.fetchClientSecret`. Guarded with red/green `dx run --test ./benchmarks/payments-dx-check-output.test.ts`, `dx run --check ./benchmarks/payments-dx-check-output.test.ts`, and scoped `rustfmt --edition 2024 --check ./core/src/ecosystem/project_check/payments_dx_check.rs`; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live Stripe runtime proof were skipped. The attempted `project_check.rs` rustfmt check still reports unrelated pre-existing formatting drift in `forms_dx_check.rs`. Next action: add SHA-256 `file_hashes` to the Payments billing workflow receipt and compare them for hash-derived stale detection.`
- `CHANGELOG.md:2378:- Wired Validation & Schemas into Rust dx-check output. The lane 5 package `validation/zod` now maps official **Validation & Schemas** naming, upstream `zod` `4.4.3` provenance, selected package-status/receipt evidence, and `validation_schemas_*` metrics into the core Forge `dx check` section through `core/src/ecosystem/project_check/validation_schemas_dx_check.rs` and `core/src/ecosystem/project_check.rs`. Missing package-status, missing receipt, stale, blocked, and unsupported states now produce `validation-schemas-*` findings from package-status rows and referenced receipts while keeping browser/runtime proof explicitly unclaimed. Re-inspected upstream `packages/zod/package.json`, `packages/zod/src/v4/classic/external.ts`, `packages/zod/src/v4/classic/schemas.ts`, `packages/zod/src/v4/core/errors.ts`, and `packages/zod/src/v4/core/json-schema-processors.ts` for `safeParse`, `safeParseAsync`, `z.strictObject`, `z.flattenError`, `z.treeifyError`, `z.prettifyError`, `z.toJSONSchema`, `z.fromJSONSchema`, `.meta()`, and `.readonly()`. Guarded with red/green `dx run --test ./benchmarks/zod-package-status-read-model.test.ts`, touched-file Rust formatting, and scoped syntax/diff/conflict checks; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live Validation & Schemas runtime proof were skipped. Next action: add SHA-256 `file_hashes` to the dashboard settings receipt and compare them in Rust dx-check for hash-derived stale detection.`
- `CHANGELOG.md:2380:- Wired Backend Platform Client into the shared package-status read model. The lane 10 package `supabase/client` now exposes official **Backend Platform Client** naming, upstream `@supabase/ssr + @supabase/supabase-js` provenance, selected `supabase-profile-workflow` and `supabase-schema-query-workflow` surfaces, `backend_platform_client_*` dx-check metrics, SHA-256 file hashes, receipt path `examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json`, and Zed receipt surfaces `backend-platform-client:supabase-profile-workflow` plus `backend-platform-client:supabase-schema-query-workflow` through `examples/template/.dx/forge/package-status.json`, `examples/template/forge-package-status-read-model.ts`, and `examples/template/forge-package-status.ts`. Re-inspected upstream Supabase user-management account/profile source, SSR client/server examples, and Studio Project API docs for `createBrowserClient`, `createServerClient`, `auth.getUser`, `from('profiles').select`, and `from('profiles').upsert`. Guarded with red/green `dx run --test ./benchmarks/supabase-package-status-read-model.test.ts`, the focused Supabase bundle, `dx run --check`, receipt/package-status JSON parse, and scoped diff/conflict/trailing-whitespace scans. Full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live hosted Supabase runtime proof were skipped. Next action: map `backend_platform_client_*` package-status metrics into Rust `dx check` JSON/finding output.`
- `CHANGELOG.md:2382:- Wired Documentation System into the shared package-status read model. The lane 14 package `content/fumadocs-next` now exposes official **Documentation System** naming, upstream `fumadocs` `16.8.12` provenance, selected `docs-app-router`, `dashboard-help-workflow`, `llm-export`, `openapi-reference`, and `search-index` surfaces, SHA-256 source hashes, `dx.forge.package.dx_style_compatibility`, `documentation_system_*` dx-check metrics, and Zed receipt surface `documentation-system:docs-help-changelog` through `examples/template/forge-package-status-read-model.ts`, `examples/template/forge-package-status.ts`, `examples/template/.dx/forge/package-status.json`, the Documentation System dashboard receipt, and `docs/packages/content-fumadocs-next.md`. Re-inspected upstream `packages/core/package.json`, source loader, breadcrumb, LLM export, Orama search server, search client, and OpenAPI server entrypoints for `loader`, `getBreadcrumbItems`, `llms`, `createFromSource`, `useDocsSearch`, and `createOpenAPI`. Guarded with red/green `dx run --test ./benchmarks/fumadocs-dashboard-workflow.test.ts` and JSON parse for package-status plus the Documentation System receipt; full builds, broad suites, local servers, browser automation, package installs, deploys, `just run`, and live Fumadocs renderer/OpenAPI/search runtime proof were skipped by policy. Next action: feed `documentation_system_*` package-status metrics into Rust `dx check` JSON/finding output.`

## Public Template File Snapshot

- `examples/template/.dx/forge/package-status.json` (344263 bytes)
- `examples/template/.dx/icons/sync.sr` (183 bytes)
- `examples/template/.dx/imports/import-map.json` (1101 bytes)
- `examples/template/.dx/imports/sync.sr` (267 bytes)
- `examples/template/.dx/receipts/style/build.json` (224936 bytes)
- `examples/template/.dx/style/build.sr` (328 bytes)
- `examples/template/.gitignore` (30 bytes)
- `examples/template/app/layout.tsx` (381 bytes)
- `examples/template/app/page.tsx` (651 bytes)
- `examples/template/components/auto-imports.ts` (960 bytes)
- `examples/template/components/icons/icon.tsx` (458 bytes)
- `examples/template/dx` (349 bytes)
- `examples/template/package.json` (183 bytes)
- `examples/template/public/favicon.svg` (572 bytes)
- `examples/template/public/icon.svg` (728 bytes)
- `examples/template/public/logo.svg` (910 bytes)
- `examples/template/README.md` (121 bytes)
- `examples/template/styles/generated.css` (7011 bytes)
- `examples/template/styles/globals.css` (2086 bytes)
- `examples/template/styles/theme.css` (233 bytes)
- `examples/template/tsconfig.json` (276 bytes)

## Source Tree Index

This section intentionally makes the dossier long enough to be useful for planning. It lists a broad source inventory so another AI can locate framework areas quickly.
1. `.dx/forge/docs/dx-icon-search.md` | ext `.md` | 2307 bytes
2. `.dx/forge/docs/dx-www-vertical-forge.md` | ext `.md` | 1584 bytes
3. `.dx/forge/docs/shadcn-ui-button.md` | ext `.md` | 2514 bytes
4. `.dx/forge/package-status.json` | ext `.json` | 21862 bytes
5. `.dx/forge/receipts/20260525T234626025501500Z-shadcn-ui-button.json` | ext `.json` | 4825 bytes
6. `.dx/forge/receipts/20260525T234626035689100Z-dx-icon-search.json` | ext `.json` | 4140 bytes
7. `.dx/forge/receipts/20260525T234645803579500Z-dx-www-vertical-forge.json` | ext `.json` | 2151 bytes
8. `.dx/forge/receipts/root-animation-motion-visibility.json` | ext `.json` | 575 bytes
9. `.dx/forge/receipts/root-content-fumadocs-next-visibility.json` | ext `.json` | 577 bytes
10. `.dx/forge/receipts/root-payments-stripe-js-visibility.json` | ext `.json` | 562 bytes
11. `.dx/forge/receipts/root-supabase-client-visibility.json` | ext `.json` | 574 bytes
12. `.dx/forge/source-manifest.json` | ext `.json` | 5779 bytes
13. `.dx/receipts/deploy/deploy-plan-1779463125669.json` | ext `.json` | 15370 bytes
14. `.dx/receipts/deploy/deploy-plan-1779463416258.json` | ext `.json` | 15370 bytes
15. `.dx/receipts/deploy/deploy-plan-1779463416303.json` | ext `.json` | 15370 bytes
16. `.dx/receipts/deploy/deploy-plan-1779463841545.json` | ext `.json` | 15370 bytes
17. `.dx/receipts/deploy/deploy-plan-1779464122422.json` | ext `.json` | 15370 bytes
18. `.dx/receipts/deploy/deploy-plan-1779464538334.json` | ext `.json` | 15370 bytes
19. `.dx/receipts/deploy/deploy-plan-1779464897146.json` | ext `.json` | 15370 bytes
20. `.dx/receipts/deploy/deploy-plan-1779465553798.json` | ext `.json` | 15370 bytes
21. `.dx/receipts/deploy/deploy-plan-1779466032054.json` | ext `.json` | 15370 bytes
22. `.dx/receipts/deploy/deploy-plan-1779466707986.json` | ext `.json` | 15370 bytes
23. `.dx/receipts/deploy/deploy-plan-1779467281870.json` | ext `.json` | 15370 bytes
24. `.dx/receipts/deploy/deploy-plan-1779467297380.json` | ext `.json` | 15370 bytes
25. `.dx/receipts/deploy/deploy-plan-1779468171873.json` | ext `.json` | 15370 bytes
26. `.dx/receipts/deploy/deploy-plan-1779468779974.json` | ext `.json` | 15370 bytes
27. `.dx/receipts/deploy/deploy-plan-1779469135705.json` | ext `.json` | 15370 bytes
28. `.dx/receipts/deploy/deploy-plan-1779469506169.json` | ext `.json` | 15370 bytes
29. `.dx/receipts/deploy/deploy-plan-1779470005200.json` | ext `.json` | 15370 bytes
30. `.dx/receipts/deploy/deploy-plan-1779470248260.json` | ext `.json` | 15370 bytes
31. `.dx/receipts/deploy/deploy-plan-1779470601657.json` | ext `.json` | 15370 bytes
32. `.dx/receipts/deploy/deploy-plan-1779470994139.json` | ext `.json` | 15370 bytes
33. `.dx/receipts/deploy/deploy-plan-1779471295736.json` | ext `.json` | 15370 bytes
34. `.dx/receipts/deploy/deploy-plan-1779471763218.json` | ext `.json` | 15370 bytes
35. `.dx/receipts/deploy/deploy-plan-1779472140801.json` | ext `.json` | 15370 bytes
36. `.dx/receipts/deploy/deploy-plan-1779474591148.json` | ext `.json` | 15370 bytes
37. `.dx/receipts/deploy/deploy-plan-1779474816381.json` | ext `.json` | 15370 bytes
38. `.dx/receipts/deploy/deploy-plan-latest.json` | ext `.json` | 15804 bytes
39. `.dx/receipts/deploy/deploy-status-1779463137488.json` | ext `.json` | 6560 bytes
40. `.dx/receipts/deploy/deploy-status-1779463416977.json` | ext `.json` | 6560 bytes
41. `.dx/receipts/deploy/deploy-status-1779463715324.json` | ext `.json` | 6560 bytes
42. `.dx/receipts/deploy/deploy-status-1779463841528.json` | ext `.json` | 6560 bytes
43. `.dx/receipts/deploy/deploy-status-1779464122494.json` | ext `.json` | 6560 bytes
44. `.dx/receipts/deploy/deploy-status-1779464538428.json` | ext `.json` | 6560 bytes
45. `.dx/receipts/deploy/deploy-status-1779464897452.json` | ext `.json` | 6560 bytes
46. `.dx/receipts/deploy/deploy-status-1779465553939.json` | ext `.json` | 6560 bytes
47. `.dx/receipts/deploy/deploy-status-1779466032136.json` | ext `.json` | 6560 bytes
48. `.dx/receipts/deploy/deploy-status-1779466708082.json` | ext `.json` | 6560 bytes
49. `.dx/receipts/deploy/deploy-status-1779467282186.json` | ext `.json` | 6560 bytes
50. `.dx/receipts/deploy/deploy-status-1779467297503.json` | ext `.json` | 6560 bytes
51. `.dx/receipts/deploy/deploy-status-1779468172004.json` | ext `.json` | 6560 bytes
52. `.dx/receipts/deploy/deploy-status-1779468780174.json` | ext `.json` | 6560 bytes
53. `.dx/receipts/deploy/deploy-status-1779469135827.json` | ext `.json` | 6560 bytes
54. `.dx/receipts/deploy/deploy-status-1779469506194.json` | ext `.json` | 6560 bytes
55. `.dx/receipts/deploy/deploy-status-1779470005239.json` | ext `.json` | 6560 bytes
56. `.dx/receipts/deploy/deploy-status-1779470248288.json` | ext `.json` | 6560 bytes
57. `.dx/receipts/deploy/deploy-status-1779470605759.json` | ext `.json` | 6560 bytes
58. `.dx/receipts/deploy/deploy-status-1779471000568.json` | ext `.json` | 6560 bytes
59. `.dx/receipts/deploy/deploy-status-1779471303229.json` | ext `.json` | 6560 bytes
60. `.dx/receipts/deploy/deploy-status-1779471769488.json` | ext `.json` | 6560 bytes
61. `.dx/receipts/deploy/deploy-status-1779472145022.json` | ext `.json` | 6560 bytes
62. `.dx/receipts/deploy/deploy-status-1779474601217.json` | ext `.json` | 6560 bytes
63. `.dx/receipts/deploy/deploy-status-1779474822275.json` | ext `.json` | 6560 bytes
64. `.dx/receipts/deploy/deploy-status-latest.json` | ext `.json` | 6759 bytes
65. `.dx/receipts/deploy/deploy-vercel-1779464538451.json` | ext `.json` | 3237 bytes
66. `.dx/receipts/deploy/deploy-vercel-latest.json` | ext `.json` | 3237 bytes
67. `.dx/receipts/deploy/provider-capability-matrix.json` | ext `.json` | 5139 bytes
68. `.dx/receipts/graph/consumer-snapshot.json` | ext `.json` | 4227 bytes
69. `.dx/receipts/graph/latest.json` | ext `.json` | 46501 bytes
70. `.dx/receipts/next-rust/vendor-boundary.json` | ext `.json` | 24803 bytes
71. `.dx/receipts/next-rust/vendor-boundary-consumer.json` | ext `.json` | 3554 bytes
72. `.dx/receipts/style/build.json` | ext `.json` | 238062 bytes
73. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json` | ext `.json` | 9984 bytes
74. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json` | ext `.json` | 27765 bytes
75. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json` | ext `.json` | 13271 bytes
76. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json` | ext `.json` | 18656 bytes
77. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json` | ext `.json` | 16298 bytes
78. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-content-react-markdown-source-guard.json` | ext `.json` | 1409 bytes
79. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json` | ext `.json` | 12176 bytes
80. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json` | ext `.json` | 4421 bytes
81. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json` | ext `.json` | 16690 bytes
82. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json` | ext `.json` | 12580 bytes
83. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json` | ext `.json` | 13954 bytes
84. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json` | ext `.json` | 9196 bytes
85. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json` | ext `.json` | 9330 bytes
86. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json` | ext `.json` | 16104 bytes
87. `.dx/template-app-browser-preview/.dx/forge/receipts/20260522T001543090604200Z-shadcn-ui-button.json` | ext `.json` | 4860 bytes
88. `.dx/template-app-browser-preview/.dx/forge/receipts/20260522T003036137384500Z-state-zustand.json` | ext `.json` | 11494 bytes
89. `.dx/template-app-browser-preview/.dx/forge/receipts/20260522T005833305654100Z-tanstack-query.json` | ext `.json` | 27521 bytes
90. `.dx/template-app-browser-preview/.dx/forge/receipts/20260522T130626134038800Z-shadcn-ui-button--variant-export-button.json` | ext `.json` | 4099 bytes
91. `.dx/template-app-browser-preview/.dx/forge/receipts/20260522T130639403843800Z-shadcn-ui-button--variant-export-button.json` | ext `.json` | 5254 bytes
92. `.dx/template-app-browser-preview/.dx/forge/receipts/20260522T140316675860400Z-state-zustand.json` | ext `.json` | 18216 bytes
93. `.dx/template-app-browser-preview/.dx/forge/receipts/20260522T140316726407800Z-tanstack-query.json` | ext `.json` | 45574 bytes
94. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json` | ext `.json` | 14671 bytes
95. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-template-dashboard-app.json` | ext `.json` | 573 bytes
96. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json` | ext `.json` | 9137 bytes
97. `.dx/template-app-browser-preview/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json` | ext `.json` | 10853 bytes
98. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T040605313883900Z-validation-zod.json` | ext `.json` | 15426 bytes
99. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T042135893711600Z-forms-react-hook-form.json` | ext `.json` | 8859 bytes
100. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T043303927907800Z-reactive-store.json` | ext `.json` | 8944 bytes
101. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T043535517142400Z-db-drizzle-sqlite.json` | ext `.json` | 15854 bytes
102. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T043543113033500Z-instantdb-react.json` | ext `.json` | 20864 bytes
103. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T043550519761300Z-supabase-client.json` | ext `.json` | 24003 bytes
104. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T043555816843800Z-content-react-markdown.json` | ext `.json` | 12451 bytes
105. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T043556606727800Z-api-trpc.json` | ext `.json` | 23728 bytes
106. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T043601481661400Z-content-fumadocs-next.json` | ext `.json` | 24663 bytes
107. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T043607238984400Z-i18n-next-intl.json` | ext `.json` | 27989 bytes
108. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T044826575Z-ai-vercel-ai.json` | ext `.json` | 25830 bytes
109. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T044826575Z-automations-n8n.json` | ext `.json` | 7029 bytes
110. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T044826575Z-payments-stripe-js.json` | ext `.json` | 10737 bytes
111. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T0510051989887Z-content-react-markdown.json` | ext `.json` | 12733 bytes
112. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T052300000000000Z-auth-better-auth.json` | ext `.json` | 4865 bytes
113. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T054511265000000Z-content-fumadocs-next.json` | ext `.json` | 24699 bytes
114. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T060000000000000Z-wasm-bindgen.json` | ext `.json` | 6875 bytes
115. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T061500000000000Z-animation-motion.json` | ext `.json` | 6569 bytes
116. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T061500000000000Z-content-fumadocs-next.json` | ext `.json` | 38633 bytes
117. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T063000000000000Z-3d-launch-scene.json` | ext `.json` | 7691 bytes
118. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T064956924988500Z-update-dry-run-api-trpc.json` | ext `.json` | 36568 bytes
119. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T123555744570300Z-update-dry-run-instantdb-react.json` | ext `.json` | 36747 bytes
120. `.dx/template-app-browser-preview/.dx/forge/receipts/20260523T123731296201200Z-update-dry-run-instantdb-react.json` | ext `.json` | 36747 bytes
121. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142027528811400Z-content-react-markdown.json` | ext `.json` | 18312 bytes
122. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142027622478500Z-content-fumadocs-next.json` | ext `.json` | 40068 bytes
123. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142027690741900Z-i18n-next-intl.json` | ext `.json` | 48170 bytes
124. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142400591499100Z-validation-zod.json` | ext `.json` | 26374 bytes
125. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142400708734700Z-forms-react-hook-form.json` | ext `.json` | 11627 bytes
126. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142400760743800Z-db-drizzle-sqlite.json` | ext `.json` | 25571 bytes
127. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142400830225500Z-instantdb-react.json` | ext `.json` | 37686 bytes
128. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142400912725600Z-supabase-client.json` | ext `.json` | 41032 bytes
129. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142400995725100Z-content-react-markdown.json` | ext `.json` | 18249 bytes
130. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142401049820100Z-content-fumadocs-next.json` | ext `.json` | 39860 bytes
131. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142401125454300Z-i18n-next-intl.json` | ext `.json` | 49307 bytes
132. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142401205412900Z-payments-stripe-js.json` | ext `.json` | 16246 bytes
133. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142401270535800Z-ai-vercel-ai.json` | ext `.json` | 45427 bytes
134. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142401338734900Z-automations-n8n.json` | ext `.json` | 11665 bytes
135. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142401383893200Z-wasm-bindgen--variant-web-target-source.json` | ext `.json` | 16131 bytes
136. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142401447389200Z-animation-motion--variant-react-dashboard-source.json` | ext `.json` | 43715 bytes
137. `.dx/template-app-browser-preview/.dx/forge/receipts/20260525T142401544793700Z-3d-launch-scene--variant-source-owned-renderer-boundary.json` | ext `.json` | 51563 bytes
138. `.dx/template-app-browser-preview/.dx/forge/receipts/3d-launch-scene-dashboard-workflow.json` | ext `.json` | 14036 bytes
139. `.dx/template-app-browser-preview/.dx/forge/receipts/auth-better-auth.json` | ext `.json` | 17516 bytes
140. `.dx/template-app-browser-preview/.dx/forge/template-readiness/ai-sdk.json` | ext `.json` | 1830 bytes
141. `.dx/template-app-browser-preview/.dx/forge/template-readiness/authentication.json` | ext `.json` | 1902 bytes
142. `.dx/template-app-browser-preview/.dx/forge/template-readiness/automation-connectors.json` | ext `.json` | 2096 bytes
143. `.dx/template-app-browser-preview/.dx/forge/template-readiness/database-api.json` | ext `.json` | 3949 bytes
144. `.dx/template-app-browser-preview/.dx/forge/template-readiness/launch-companion-doc-receipts.json` | ext `.json` | 2634 bytes
145. `.dx/template-app-browser-preview/.dx/forge/template-readiness/launch-readiness-bundle.json` | ext `.json` | 2033 bytes
146. `.dx/template-app-browser-preview/.dx/forge/template-readiness/launch-route.json` | ext `.json` | 2618 bytes
147. `.dx/template-app-browser-preview/.dx/forge/template-readiness/launch-runtime-approval-request.json` | ext `.json` | 1704 bytes
148. `.dx/template-app-browser-preview/.dx/forge/template-readiness/launch-runtime-checklist.json` | ext `.json` | 1910 bytes
149. `.dx/template-app-browser-preview/.dx/forge/template-readiness/launch-runtime-evidence.json` | ext `.json` | 1843 bytes
150. `.dx/template-app-browser-preview/.dx/forge/template-readiness/launch-scene-readiness.json` | ext `.json` | 1548 bytes
151. `.dx/template-app-browser-preview/.dx/forge/template-readiness/launch-verification-lane.json` | ext `.json` | 2106 bytes
152. `.dx/template-app-browser-preview/.dx/forge/template-readiness/payments.json` | ext `.json` | 2547 bytes
153. `.dx/template-app-browser-preview/.dx/forge/template-readiness/zed-template-handoff.json` | ext `.json` | 1365 bytes
154. `.dx/template-app-browser-preview/app/api/ai/chat/route.ts` | ext `.ts` | 1516 bytes
155. `.dx/template-app-browser-preview/app/api/auth/[...all]/route.ts` | ext `.ts` | 89 bytes
156. `.dx/template-app-browser-preview/app/api/auth/readiness/route.ts` | ext `.ts` | 974 bytes
157. `.dx/template-app-browser-preview/app/api/auth/session/route.ts` | ext `.ts` | 648 bytes
158. `.dx/template-app-browser-preview/app/api/automations/n8n/dry-run/route.ts` | ext `.ts` | 5795 bytes
159. `.dx/template-app-browser-preview/app/api/checkout/route.ts` | ext `.ts` | 3182 bytes
160. `.dx/template-app-browser-preview/app/api/database-api/readiness/route.ts` | ext `.ts` | 214 bytes
161. `.dx/template-app-browser-preview/app/api/database-orm/readiness/route.ts` | ext `.ts` | 214 bytes
162. `.dx/template-app-browser-preview/app/api/instant/readiness/route.ts` | ext `.ts` | 201 bytes
163. `.dx/template-app-browser-preview/app/api/payments/stripe-js/readiness/route.ts` | ext `.ts` | 6035 bytes
164. `.dx/template-app-browser-preview/app/api/query-cache/readiness/route.ts` | ext `.ts` | 386 bytes
165. `.dx/template-app-browser-preview/app/api/stripe/webhook/route.ts` | ext `.ts` | 4106 bytes
166. `.dx/template-app-browser-preview/app/api/supabase/readiness/route.ts` | ext `.ts` | 204 bytes
167. `.dx/template-app-browser-preview/app/api/trpc/health/route.ts` | ext `.ts` | 1637 bytes
168. `.dx/template-app-browser-preview/auth/better-auth/.env.example` | ext `.example` | 234 bytes
169. `.dx/template-app-browser-preview/auth/better-auth/account-deletion.ts` | ext `.ts` | 493 bytes
170. `.dx/template-app-browser-preview/auth/better-auth/accounts.ts` | ext `.ts` | 590 bytes
171. `.dx/template-app-browser-preview/auth/better-auth/account-security.ts` | ext `.ts` | 557 bytes
172. `.dx/template-app-browser-preview/auth/better-auth/client.ts` | ext `.ts` | 646 bytes
173. `.dx/template-app-browser-preview/auth/better-auth/dashboard.ts` | ext `.ts` | 1265 bytes
174. `.dx/template-app-browser-preview/auth/better-auth/email-password.ts` | ext `.ts` | 560 bytes
175. `.dx/template-app-browser-preview/auth/better-auth/metadata.ts` | ext `.ts` | 1488 bytes
176. `.dx/template-app-browser-preview/auth/better-auth/options.ts` | ext `.ts` | 2015 bytes
177. `.dx/template-app-browser-preview/auth/better-auth/profile.ts` | ext `.ts` | 477 bytes
178. `.dx/template-app-browser-preview/auth/better-auth/providers/google/.env.example` | ext `.example` | 40 bytes
179. `.dx/template-app-browser-preview/auth/better-auth/providers/google/callback.ts` | ext `.ts` | 357 bytes
180. `.dx/template-app-browser-preview/auth/better-auth/providers/google/config.ts` | ext `.ts` | 659 bytes
181. `.dx/template-app-browser-preview/auth/better-auth/providers/google/README.md` | ext `.md` | 365 bytes
182. `.dx/template-app-browser-preview/auth/better-auth/providers/google/route.ts` | ext `.ts` | 566 bytes
183. `.dx/template-app-browser-preview/auth/better-auth/README.md` | ext `.md` | 663 bytes
184. `.dx/template-app-browser-preview/auth/better-auth/route.ts` | ext `.ts` | 1892 bytes
185. `.dx/template-app-browser-preview/auth/better-auth/server.ts` | ext `.ts` | 1414 bytes
186. `.dx/template-app-browser-preview/auth/better-auth/session.ts` | ext `.ts` | 454 bytes
187. `.dx/template-app-browser-preview/auth/better-auth/session-management.ts` | ext `.ts` | 344 bytes
188. `.dx/template-app-browser-preview/auth/better-auth/social.ts` | ext `.ts` | 586 bytes
189. `.dx/template-app-browser-preview/components/template-app/dashboard-query-cache.ts` | ext `.ts` | 5454 bytes
190. `.dx/template-app-browser-preview/favicon.svg` | ext `.svg` | 405 bytes
191. `.dx/template-app-browser-preview/lib/ai/provider-boundary.ts` | ext `.ts` | 1975 bytes
192. `.dx/template-app-browser-preview/lib/automations/n8n/catalog.ts` | ext `.ts` | 5243 bytes
193. `.dx/template-app-browser-preview/lib/automations/n8n/readiness.ts` | ext `.ts` | 2606 bytes
194. `.dx/template-app-browser-preview/lib/automations/n8n/receipt.ts` | ext `.ts` | 2952 bytes
195. `.dx/template-app-browser-preview/lib/database-api/source-contract.ts` | ext `.ts` | 7871 bytes
196. `.dx/template-app-browser-preview/lib/payments/stripe-js/checkout.ts` | ext `.ts` | 13303 bytes
197. `.dx/template-app-browser-preview/lib/payments/stripe-js/config.ts` | ext `.ts` | 4005 bytes
198. `.dx/template-app-browser-preview/lib/payments/stripe-js/dashboard-checkout.ts` | ext `.ts` | 4034 bytes
199. `.dx/template-app-browser-preview/lib/payments/stripe-js/server.ts` | ext `.ts` | 17143 bytes
200. `.dx/template-app-browser-preview/lib/supabase/env.ts` | ext `.ts` | 1771 bytes
201. `.dx/template-app-browser-preview/pages/_layout.html` | ext `.html` | 32 bytes
202. `.dx/template-app-browser-preview/pages/automations.html` | ext `.html` | 2899 bytes
203. `.dx/template-app-browser-preview/pages/backend.html` | ext `.html` | 3669 bytes
204. `.dx/template-app-browser-preview/pages/dashboard.html` | ext `.html` | 143137 bytes
205. `.dx/template-app-browser-preview/pages/database.html` | ext `.html` | 2901 bytes
206. `.dx/template-app-browser-preview/pages/favicon.svg.html` | ext `.html` | 405 bytes
207. `.dx/template-app-browser-preview/pages/index.html` | ext `.html` | 169136 bytes
208. `.dx/template-app-browser-preview/pages/login.html` | ext `.html` | 4174 bytes
209. `.dx/template-app-browser-preview/pages/logout.html` | ext `.html` | 3073 bytes
210. `.dx/template-app-browser-preview/pages/ui.html` | ext `.html` | 3062 bytes
211. `.dx/template-app-browser-preview/public/favicon.svg` | ext `.svg` | 405 bytes
212. `.dx/template-app-browser-preview/public/launch-runtime.js` | ext `.js` | 221757 bytes
213. `.dx/template-app-browser-preview/public/preview-manifest.json` | ext `.json` | 125542 bytes
214. `.dx/template-app-browser-preview/server.cjs` | ext `.cjs` | 2100 bytes
215. `.dx/template-app-browser-preview/server/auth/better-auth.ts` | ext `.ts` | 4509 bytes
216. `.dx/template-app-browser-preview/server/database-api/readiness.ts` | ext `.ts` | 7138 bytes
217. `.dx/template-app-browser-preview/server/database-orm/readiness.ts` | ext `.ts` | 4140 bytes
218. `.dx/template-app-browser-preview/server/instant/readiness.ts` | ext `.ts` | 3039 bytes
219. `.dx/template-app-browser-preview/server/query-cache/readiness.ts` | ext `.ts` | 9539 bytes
220. `.dx/template-app-browser-preview/server/supabase/readiness.ts` | ext `.ts` | 3392 bytes
221. `.dx/template-app-browser-preview/styles/generated.css` | ext `.css` | 504 bytes
222. `.dx/template-app-browser-preview/styles/globals.css` | ext `.css` | 56812 bytes
223. `.dx/template-app-browser-preview/styles/theme.css` | ext `.css` | 1105 bytes
224. `.dx/template-dev-server.err.log` | ext `.log` | 158 bytes
225. `.dx/template-dev-server.out.log` | ext `.log` | 0 bytes
226. `.github/workflows/bench.yml` | ext `.yml` | 4844 bytes
227. `.github/workflows/ci.yml` | ext `.yml` | 4567 bytes
228. `.github/workflows/forge-ci.yml` | ext `.yml` | 4840 bytes
229. `.github/workflows/release.yml` | ext `.yml` | 3705 bytes
230. `.gitignore` | ext `.gitignore` | 2621 bytes
231. `.kiro/specs/production-excellence/design.md` | ext `.md` | 18688 bytes
232. `.kiro/specs/production-excellence/requirements.md` | ext `.md` | 8528 bytes
233. `.kiro/specs/production-excellence/tasks.md` | ext `.md` | 13983 bytes
234. `.kiro/specs/production-readiness/design.md` | ext `.md` | 20055 bytes
235. `.kiro/specs/production-readiness/requirements.md` | ext `.md` | 10709 bytes
236. `.kiro/specs/production-readiness/tasks.md` | ext `.md` | 13186 bytes
237. `a11y/Cargo.toml` | ext `.toml` | 476 bytes
238. `a11y/CHANGELOG.md` | ext `.md` | 374 bytes
239. `a11y/LICENSE` | ext `<none>` | 559 bytes
240. `a11y/LICENSE-APACHE` | ext `<none>` | 5871 bytes
241. `a11y/LICENSE-MIT` | ext `<none>` | 1093 bytes
242. `a11y/README.md` | ext `.md` | 370 bytes
243. `a11y/src/lib.rs` | ext `.rs` | 10565 bytes
244. `a11y/tests/property_tests.rs` | ext `.rs` | 14293 bytes
245. `AGENTS.md` | ext `.md` | 5476 bytes
246. `auth/Cargo.toml` | ext `.toml` | 1197 bytes
247. `auth/CHANGELOG.md` | ext `.md` | 374 bytes
248. `auth/LICENSE` | ext `<none>` | 559 bytes
249. `auth/LICENSE-APACHE` | ext `<none>` | 5871 bytes
250. `auth/LICENSE-MIT` | ext `<none>` | 1093 bytes
251. `auth/README.md` | ext `.md` | 533 bytes
252. `auth/src/lib.rs` | ext `.rs` | 35007 bytes
253. `auth/tests/property_tests.rs` | ext `.rs` | 30731 bytes
254. `benches/delta_benchmarks.rs` | ext `.rs` | 5679 bytes
255. `benches/htip_benchmarks.rs` | ext `.rs` | 6585 bytes
256. `benches/parser_benchmarks.rs` | ext `.rs` | 8980 bytes
257. `benches/ssr_benchmarks.rs` | ext `.rs` | 10330 bytes
258. `benchmarks/ai-route-handler-provider-boundary.test.mjs` | ext `.mjs` | 1526 bytes
259. `benchmarks/app-api-route-handler-extensions.test.mjs` | ext `.mjs` | 10911 bytes
260. `benchmarks/app-api-route-handler-root-precedence.test.ts` | ext `.ts` | 1053 bytes
261. `benchmarks/app-router-build-output-shared-segments.test.cjs` | ext `.cjs` | 1210 bytes
262. `benchmarks/app-router-build-route-discovery-output.test.ts` | ext `.ts` | 2726 bytes
263. `benchmarks/app-router-discovery-specificity-summary.test.cjs` | ext `.cjs` | 1619 bytes
264. `benchmarks/app-router-discovery-summary.test.cjs` | ext `.cjs` | 1252 bytes
265. `benchmarks/app-router-duplicate-param-names.test.cjs` | ext `.cjs` | 1420 bytes
266. `benchmarks/app-router-dynamic-collision-shape.test.cjs` | ext `.cjs` | 1132 bytes
267. `benchmarks/app-router-execution-shared-segments.test.cjs` | ext `.cjs` | 1214 bytes
268. `benchmarks/app-router-filesystem-specificity.test.cjs` | ext `.cjs` | 909 bytes
269. `benchmarks/app-router-invalid-non-path-segments.test.ts` | ext `.ts` | 1657 bytes
270. `benchmarks/app-router-invalid-param-segments.test.cjs` | ext `.cjs` | 1457 bytes
271. `benchmarks/app-router-invalid-segment-diagnostics.test.ts` | ext `.ts` | 2718 bytes
272. `benchmarks/app-router-page-collision-metadata.test.cjs` | ext `.cjs` | 846 bytes
273. `benchmarks/app-router-page-discovery-collisions.test.cjs` | ext `.cjs` | 1075 bytes
274. `benchmarks/app-router-page-extensions-build-loop.test.mjs` | ext `.mjs` | 7963 bytes
275. `benchmarks/app-router-page-request-normalization.test.cjs` | ext `.cjs` | 681 bytes
276. `benchmarks/app-router-route-precedence-vector.test.cjs` | ext `.cjs` | 1176 bytes
277. `benchmarks/app-router-route-root-precedence.test.cjs` | ext `.cjs` | 858 bytes
278. `benchmarks/app-router-segment-file-discovery.test.cjs` | ext `.cjs` | 1264 bytes
279. `benchmarks/app-router-server-data-build-contract.test.ts` | ext `.ts` | 29598 bytes
280. `benchmarks/app-router-shape-collision-peers.test.ts` | ext `.ts` | 1020 bytes
281. `benchmarks/app-router-shared-segment-classifier.test.cjs` | ext `.cjs` | 1504 bytes
282. `benchmarks/app-router-source-owned-vocabulary.test.cjs` | ext `.cjs` | 3051 bytes
283. `benchmarks/app-router-src-app-segment-depth.test.cjs` | ext `.cjs` | 1000 bytes
284. `benchmarks/app-router-static-segment-decoding.test.cjs` | ext `.cjs` | 811 bytes
285. `benchmarks/app-router-terminal-catch-all.test.cjs` | ext `.cjs` | 1421 bytes
286. `benchmarks/authentication-dx-check-output.test.ts` | ext `.ts` | 5089 bytes
287. `benchmarks/authentication-dx-check-package-lane-panel.test.ts` | ext `.ts` | 19340 bytes
288. `benchmarks/authentication-dx-check-visibility.test.ts` | ext `.ts` | 4201 bytes
289. `benchmarks/authentication-lane-naming.test.ts` | ext `.ts` | 4751 bytes
290. `benchmarks/authentication-lock-backed-template.test.ts` | ext `.ts` | 18377 bytes
291. `benchmarks/authentication-package-status-read-model.test.ts` | ext `.ts` | 16170 bytes
292. `benchmarks/authentication-receipt-hash-refresh.test.ts` | ext `.ts` | 19874 bytes
293. `benchmarks/automation-route-handler-provider-boundary.test.mjs` | ext `.mjs` | 2350 bytes
294. `benchmarks/automations-bridge.test.ts` | ext `.ts` | 19769 bytes
295. `benchmarks/automations-dashboard-workflow.test.ts` | ext `.ts` | 4572 bytes
296. `benchmarks/automations-dx-check-package-lane-panel.test.ts` | ext `.ts` | 9249 bytes
297. `benchmarks/automations-dx-style-compatibility.test.ts` | ext `.ts` | 4766 bytes
298. `benchmarks/automations-generated-starter-receipt.test.ts` | ext `.ts` | 4339 bytes
299. `benchmarks/automations-launch-visible-proof.test.ts` | ext `.ts` | 23048 bytes
300. `benchmarks/automations-package-status-read-model.test.ts` | ext `.ts` | 23088 bytes
301. `benchmarks/automations-receipt-hash-refresh.test.ts` | ext `.ts` | 16631 bytes
302. `benchmarks/benches/delta_benchmarks.rs` | ext `.rs` | 5584 bytes
303. `benchmarks/benches/htip_benchmarks.rs` | ext `.rs` | 6429 bytes
304. `benchmarks/benches/parser_benchmarks.rs` | ext `.rs` | 8259 bytes
305. `benchmarks/benches/ssr_benchmarks.rs` | ext `.rs` | 10497 bytes
306. `benchmarks/benchmark-report-scope-contract.test.cjs` | ext `.cjs` | 1846 bytes
307. `benchmarks/better-auth-dashboard-workflow.test.ts` | ext `.ts` | 17504 bytes
308. `benchmarks/better-auth-live-runtime.test.ts` | ext `.ts` | 5057 bytes
309. `benchmarks/better-auth-session-helper.test.ts` | ext `.ts` | 17474 bytes
310. `benchmarks/binary-web-lab/Cargo.lock` | ext `.lock` | 21041 bytes
311. `benchmarks/binary-web-lab/Cargo.toml` | ext `.toml` | 274 bytes
312. `benchmarks/binary-web-lab/src/main.rs` | ext `.rs` | 35566 bytes
313. `benchmarks/Cargo.toml` | ext `.toml` | 930 bytes
314. `benchmarks/cli-add-args-split-safety.test.mjs` | ext `.mjs` | 1099 bytes
315. `benchmarks/cli-app-api-route-split-safety.test.ts` | ext `.ts` | 2715 bytes
316. `benchmarks/cli-deploy-adapter-contract-split-safety.test.ts` | ext `.ts` | 2594 bytes
317. `benchmarks/cli-dev-http-split-safety.test.ts` | ext `.ts` | 8766 bytes
318. `benchmarks/cli-dev-options-split-safety.test.ts` | ext `.ts` | 3205 bytes
319. `benchmarks/cli-forge-add-options-split-safety.test.mjs` | ext `.mjs` | 2161 bytes
320. `benchmarks/cli-forge-adoption-options-split-safety.test.mjs` | ext `.mjs` | 2959 bytes
321. `benchmarks/cli-forge-audit-options-split-safety.test.mjs` | ext `.mjs` | 2093 bytes
322. `benchmarks/cli-forge-beta-options-split-safety.test.mjs` | ext `.mjs` | 3142 bytes
323. `benchmarks/cli-forge-ci-snippets-options-split-safety.test.mjs` | ext `.mjs` | 2044 bytes
324. `benchmarks/cli-forge-doctor-split-safety.test.cjs` | ext `.cjs` | 2800 bytes
325. `benchmarks/cli-forge-evidence-options-split-safety.test.mjs` | ext `.mjs` | 2089 bytes
326. `benchmarks/cli-forge-hosted-registry-smoke-split-safety.test.cjs` | ext `.cjs` | 3110 bytes
327. `benchmarks/cli-forge-init-app-options-split-safety.test.mjs` | ext `.mjs` | 2134 bytes
328. `benchmarks/cli-forge-launch-copy-review-split-safety.test.cjs` | ext `.cjs` | 2249 bytes
329. `benchmarks/cli-forge-launch-page-split-safety.test.cjs` | ext `.cjs` | 2164 bytes
330. `benchmarks/cli-forge-materialize-static-assets-split-safety.test.cjs` | ext `.cjs` | 2436 bytes
331. `benchmarks/cli-forge-packages-command-split-safety.test.cjs` | ext `.cjs` | 1662 bytes
332. `benchmarks/cli-forge-packages-options-split-safety.test.mjs` | ext `.mjs` | 2270 bytes
333. `benchmarks/cli-forge-provenance-command-split-safety.test.cjs` | ext `.cjs` | 1510 bytes
334. `benchmarks/cli-forge-provenance-options-split-safety.test.mjs` | ext `.mjs` | 2320 bytes
335. `benchmarks/cli-forge-public-add-split-safety.test.cjs` | ext `.cjs` | 2167 bytes
336. `benchmarks/cli-forge-public-evidence-options-split-safety.test.mjs` | ext `.mjs` | 2906 bytes
337. `benchmarks/cli-forge-public-status-split-safety.test.cjs` | ext `.cjs` | 3536 bytes
338. `benchmarks/cli-forge-publisher-key-options-split-safety.test.mjs` | ext `.mjs` | 3670 bytes
339. `benchmarks/cli-forge-publish-options-split-safety.test.mjs` | ext `.mjs` | 2400 bytes
340. `benchmarks/cli-forge-publish-plan-options-split-safety.test.mjs` | ext `.mjs` | 2264 bytes
341. `benchmarks/cli-forge-react-starter-benchmark-split-safety.test.cjs` | ext `.cjs` | 2386 bytes
342. `benchmarks/cli-forge-registry-options-split-safety.test.mjs` | ext `.mjs` | 3541 bytes
343. `benchmarks/cli-forge-release-candidate-command-split-safety.test.cjs` | ext `.cjs` | 1661 bytes
344. `benchmarks/cli-forge-release-candidate-report-split-safety.test.cjs` | ext `.cjs` | 3100 bytes
345. `benchmarks/cli-forge-release-candidate-split-safety.test.cjs` | ext `.cjs` | 1596 bytes
346. `benchmarks/cli-forge-release-dashboard-command-split-safety.test.cjs` | ext `.cjs` | 1615 bytes
347. `benchmarks/cli-forge-release-dashboard-split-safety.test.cjs` | ext `.cjs` | 1572 bytes
348. `benchmarks/cli-forge-release-proof-split-safety.test.cjs` | ext `.cjs` | 1935 bytes
349. `benchmarks/cli-forge-release-history-command-split-safety.test.ts` | ext `.ts` | 1803 bytes
350. `benchmarks/cli-forge-release-operations-options-split-safety.test.mjs` | ext `.mjs` | 2484 bytes
351. `benchmarks/cli-forge-release-review-options-split-safety.test.mjs` | ext `.mjs` | 2315 bytes
352. `benchmarks/cli-forge-remote-lifecycle-split-safety.test.cjs` | ext `.cjs` | 2942 bytes
353. `benchmarks/cli-forge-smoke-options-split-safety.test.mjs` | ext `.mjs` | 3053 bytes
354. `benchmarks/cli-forge-trust-policy-command-split-safety.test.cjs` | ext `.cjs` | 1500 bytes
355. `benchmarks/cli-forge-trust-policy-options-split-safety.test.mjs` | ext `.mjs` | 2445 bytes
356. `benchmarks/cli-forge-trust-regression-command-split-safety.test.cjs` | ext `.cjs` | 1612 bytes
357. `benchmarks/cli-forge-trust-regression-options-split-safety.test.mjs` | ext `.mjs` | 2623 bytes
358. `benchmarks/cli-forge-update-options-split-safety.test.mjs` | ext `.mjs` | 2655 bytes
359. `benchmarks/cli-formatting-split-safety.test.mjs` | ext `.mjs` | 1234 bytes
360. `benchmarks/cli-help-text-split-safety.test.ts` | ext `.ts` | 4530 bytes
361. `benchmarks/cli-launch-report-options-split-safety.test.mjs` | ext `.mjs` | 3453 bytes
362. `benchmarks/cli-migrate-options-split-safety.test.mjs` | ext `.mjs` | 4718 bytes
363. `benchmarks/cli-naming-split-safety.test.mjs` | ext `.mjs` | 1644 bytes
364. `benchmarks/cli-new-command-split-safety.test.ts` | ext `.ts` | 1603 bytes
365. `benchmarks/cli-next-rust-status-split-safety.test.cjs` | ext `.cjs` | 1589 bytes
366. `benchmarks/cli-options-split-safety.test.mjs` | ext `.mjs` | 1334 bytes
367. `benchmarks/cli-preview-contract-split-safety.test.mjs` | ext `.mjs` | 4662 bytes
368. `benchmarks/cli-preview-options-split-safety.test.mjs` | ext `.mjs` | 2602 bytes
369. `benchmarks/cli-promote-options-split-safety.test.mjs` | ext `.mjs` | 4000 bytes
370. `benchmarks/cli-rollback-options-split-safety.test.mjs` | ext `.mjs` | 4243 bytes
371. `benchmarks/cli-studio-json-surface-split-safety.test.mjs` | ext `.mjs` | 3334 bytes
372. `benchmarks/cli-template-options-split-safety.test.mjs` | ext `.mjs` | 3938 bytes
373. `benchmarks/cli-update-options-split-safety.test.mjs` | ext `.mjs` | 4155 bytes
374. `benchmarks/compare-forge-large-content.ts` | ext `.ts` | 16908 bytes
375. `benchmarks/compare-forge-launch-delivery.ts` | ext `.ts` | 10756 bytes
376. `benchmarks/compare-forge-live-frameworks.ts` | ext `.ts` | 13473 bytes
377. `benchmarks/compare-forge-medium-route.ts` | ext `.ts` | 16770 bytes
378. `benchmarks/compare-forge-public-routes.ts` | ext `.ts` | 7611 bytes
379. `benchmarks/compare-forge-static-competitors.ts` | ext `.ts` | 17332 bytes
380. `benchmarks/default-www-template-contract.test.ts` | ext `.ts` | 27273 bytes
381. `benchmarks/diagnostics-code-frame-contract.test.mjs` | ext `.mjs` | 32326 bytes
382. `benchmarks/drizzle-dashboard-workflow.test.ts` | ext `.ts` | 6314 bytes
383. `benchmarks/drizzle-dx-check-package-lane-panel.test.ts` | ext `.ts` | 15133 bytes
384. `benchmarks/drizzle-dx-check-visibility-receipt.test.ts` | ext `.ts` | 5360 bytes
385. `benchmarks/drizzle-launch-proof.test.ts` | ext `.ts` | 17130 bytes
386. `benchmarks/drizzle-package-status-read-model.test.ts` | ext `.ts` | 15704 bytes
387. `benchmarks/drizzle-receipt-hash-refresh.test.ts` | ext `.ts` | 28353 bytes
388. `benchmarks/drizzle-sqlite-analytics-slice.test.ts` | ext `.ts` | 1697 bytes
389. `benchmarks/drizzle-sqlite-conflict-writes-slice.test.ts` | ext `.ts` | 1719 bytes
390. `benchmarks/drizzle-sqlite-cte-query-slice.test.ts` | ext `.ts` | 1859 bytes
391. `benchmarks/drizzle-sqlite-joins-slice.test.ts` | ext `.ts` | 1603 bytes
392. `benchmarks/drizzle-sqlite-migrations-slice.test.ts` | ext `.ts` | 1396 bytes
393. `benchmarks/drizzle-sqlite-mutations-slice.test.ts` | ext `.ts` | 1832 bytes
394. `benchmarks/drizzle-sqlite-prepared-queries-slice.test.ts` | ext `.ts` | 1800 bytes
395. `benchmarks/drizzle-sqlite-relational-query-slice.test.ts` | ext `.ts` | 1252 bytes
396. `benchmarks/drizzle-sqlite-replica-routing-slice.test.ts` | ext `.ts` | 3115 bytes
397. `benchmarks/drizzle-sqlite-set-operations-slice.test.ts` | ext `.ts` | 2087 bytes
398. `benchmarks/drizzle-sqlite-transactions-slice.test.ts` | ext `.ts` | 1435 bytes
399. `benchmarks/drizzle-sqlite-views-slice.test.ts` | ext `.ts` | 2215 bytes
400. `benchmarks/dx-api-router-http-method-detection.test.cjs` | ext `.cjs` | 1201 bytes
401. `benchmarks/dx-app-router-catch-all-semantics.test.cjs` | ext `.cjs` | 1340 bytes
402. `benchmarks/dx-build-black-box-e2e.test.ts` | ext `.ts` | 10786 bytes
403. `benchmarks/dx-build-cli-contract.test.ts` | ext `.ts` | 7951 bytes
404. `benchmarks/dx-build-cli-help.test.ts` | ext `.ts` | 1778 bytes
405. `benchmarks/dx-build-css-pipeline-boundary.test.cjs` | ext `.cjs` | 20902 bytes
406. `benchmarks/dx-build-graph-core-map.test.cjs` | ext `.cjs` | 9665 bytes
407. `benchmarks/dx-build-graph-receipt.test.ts` | ext `.ts` | 15837 bytes
408. `benchmarks/dx-build-graph-turbo-tasks-adapter.test.ts` | ext `.ts` | 7284 bytes
409. `benchmarks/dx-build-installed-smoke.test.ts` | ext `.ts` | 87936 bytes
410. `benchmarks/dx-build-installed-smoke-cli-errors.test.cjs` | ext `.cjs` | 7595 bytes
411. `benchmarks/dx-build-installed-smoke-compact-report.test.ts` | ext `.ts` | 5397 bytes
412. `benchmarks/dx-build-installed-smoke-human-report.test.cjs` | ext `.cjs` | 12008 bytes
413. `benchmarks/dx-build-installed-smoke-no-node-boundary.test.ts` | ext `.ts` | 19891 bytes
414. `benchmarks/dx-build-installed-smoke-report.test.cjs` | ext `.cjs` | 12719 bytes
415. `benchmarks/dx-build-next-familiar-fixtures-contract.test.cjs` | ext `.cjs` | 5432 bytes
416. `benchmarks/dx-build-readiness-gate.test.ts` | ext `.ts` | 43888 bytes
417. `benchmarks/dx-build-source-receipt-bundle-contract.test.cjs` | ext `.cjs` | 1264 bytes
418. `benchmarks/dx-check-zed-panel-schema-compat.test.cjs` | ext `.cjs` | 1030 bytes
419. `benchmarks/dx-contract-professional-names.test.ts` | ext `.ts` | 4786 bytes
420. `benchmarks/dx-dev-asset-refresh-precision.test.cjs` | ext `.cjs` | 15163 bytes
421. `benchmarks/dx-dev-existing-server-reuse.test.ts` | ext `.ts` | 2298 bytes
422. `benchmarks/dx-dev-extension-toolchain-hook.test.ts` | ext `.ts` | 3273 bytes
423. `benchmarks/dx-dev-feedback-contract.test.cjs` | ext `.cjs` | 8092 bytes
424. `benchmarks/dx-dev-feedback-overlay-recovery.test.ts` | ext `.ts` | 8940 bytes
425. `benchmarks/dx-dev-feedback-overlay-recovery-contract.test.ts` | ext `.ts` | 7141 bytes
426. `benchmarks/dx-dev-feedback-style-diagnostics.test.ts` | ext `.ts` | 18503 bytes
427. `benchmarks/dx-dev-hot-reload-live-edit.test.ts` | ext `.ts` | 12201 bytes
428. `benchmarks/dx-dev-hot-reload-malformed-sse.test.cjs` | ext `.cjs` | 11083 bytes
429. `benchmarks/dx-dev-hot-reload-protocol.test.cjs` | ext `.cjs` | 28076 bytes
430. `benchmarks/dx-dev-reload-src-app-root.test.cjs` | ext `.cjs` | 2691 bytes
431. `benchmarks/dx-dev-server-black-box.test.ts` | ext `.ts` | 16817 bytes
432. `benchmarks/dx-dev-server-launch-smoke.test.cjs` | ext `.cjs` | 3562 bytes
433. `benchmarks/dx-dev-template-component-hot-reload-contract.test.ts` | ext `.ts` | 1329 bytes
434. `benchmarks/dx-devtools-framework-integration.test.ts` | ext `.ts` | 61379 bytes
435. `benchmarks/dx-hot-reload-diagnostics-publish.test.cjs` | ext `.cjs` | 5971 bytes
436. `benchmarks/dx-hot-reload-issue-stream.test.cjs` | ext `.cjs` | 4466 bytes
437. `benchmarks/dx-hot-reload-resource-normalization.test.cjs` | ext `.cjs` | 5418 bytes
438. `benchmarks/dx-router-request-normalization.test.cjs` | ext `.cjs` | 1969 bytes
439. `benchmarks/dx-run-extension-list-orchestrator.test.ts` | ext `.ts` | 4981 bytes
440. `benchmarks/dx-scope-removal-contract.test.cjs` | ext `.cjs` | 4328 bytes
441. `benchmarks/dx-studio-preview-manifest.test.ts` | ext `.ts` | 37934 bytes
442. `benchmarks/dx-style-compile-boundary.test.mjs` | ext `.mjs` | 1255 bytes
443. `benchmarks/dx-style-container-size-v43-source-guard.test.ts` | ext `.ts` | 1837 bytes
444. `benchmarks/dx-style-drift-fixture-consumer.test.ts` | ext `.ts` | 14227 bytes
445. `benchmarks/dx-style-font-features-v42-source-guard.test.ts` | ext `.ts` | 1654 bytes
446. `benchmarks/dx-style-lane8-no-runtime-integration.test.cjs` | ext `.cjs` | 2679 bytes
447. `benchmarks/dx-style-launch-contract.test.ts` | ext `.ts` | 171758 bytes
448. `benchmarks/dx-style-live-comparison-receipt-accuracy.test.cjs` | ext `.cjs` | 38437 bytes
449. `benchmarks/dx-style-live-tailwind-v43-comparison.test.cjs` | ext `.cjs` | 13566 bytes
450. `benchmarks/dx-style-no-tailwind-runtime-guard.test.ts` | ext `.ts` | 4685 bytes
451. `benchmarks/dx-style-official-tailwind-fixture-matrix.test.cjs` | ext `.cjs` | 40462 bytes
452. `benchmarks/dx-style-package-ownership-read-model.test.ts` | ext `.ts` | 7633 bytes
453. `benchmarks/dx-style-postcss-compatibility.test.ts` | ext `.ts` | 6657 bytes
454. `benchmarks/dx-style-pruning-contract.test.ts` | ext `.ts` | 925 bytes
455. `benchmarks/dx-style-template-token-compat.test.ts` | ext `.ts` | 8229 bytes
456. `benchmarks/dx-style-typography-v43-source-guard.test.ts` | ext `.ts` | 2061 bytes
457. `benchmarks/dx-style-v43-color-palette.test.cjs` | ext `.cjs` | 7313 bytes
458. `benchmarks/dx-style-v43-container-query-variants.test.cjs` | ext `.cjs` | 14847 bytes
459. `benchmarks/dx-style-v43-css-directive-parity.test.cjs` | ext `.cjs` | 13366 bytes
460. `benchmarks/dx-style-v43-css-directives.test.cjs` | ext `.cjs` | 6825 bytes
461. `benchmarks/dx-style-v43-gap-matrix.test.ts` | ext `.ts` | 13354 bytes
462. `benchmarks/dx-style-v43-interaction-transform.test.cjs` | ext `.cjs` | 5099 bytes
463. `benchmarks/dx-style-v43-logical-layout.test.cjs` | ext `.cjs` | 8503 bytes
464. `benchmarks/dx-style-v43-palette-parity.test.cjs` | ext `.cjs` | 34512 bytes
465. `benchmarks/dx-style-v43-source-scanner-contract.test.ts` | ext `.ts` | 7262 bytes
466. `benchmarks/dx-style-v43-source-scanner-fixture-matrix.test.cjs` | ext `.cjs` | 2385 bytes
467. `benchmarks/dx-style-v43-typography-effects.test.cjs` | ext `.cjs` | 3966 bytes
468. `benchmarks/dx-style-v43-variant-selector-parity.test.cjs` | ext `.cjs` | 18009 bytes
469. `benchmarks/dx-www-agent-context-command.test.ts` | ext `.ts` | 2201 bytes
470. `benchmarks/dx-www-architecture-scope.test.cjs` | ext `.cjs` | 2405 bytes
471. `benchmarks/dx-www-architecture-scope-contract.test.cjs` | ext `.cjs` | 3359 bytes
472. `benchmarks/dx-www-cli-paths.test.ts` | ext `.ts` | 912 bytes
473. `benchmarks/dx-www-cli-paths.ts` | ext `.ts` | 2002 bytes
474. `benchmarks/dx-www-conversion-proof.test.ts` | ext `.ts` | 48672 bytes
475. `benchmarks/dx-www-current-status-docs.test.ts` | ext `.ts` | 6088 bytes
476. `benchmarks/dx-www-final-integration-score.test.ts` | ext `.ts` | 11258 bytes
477. `benchmarks/dx-www-framework-completeness.test.ts` | ext `.ts` | 3633 bytes
478. `benchmarks/dx-www-parser-launch-extensions.test.mjs` | ext `.mjs` | 2662 bytes
479. `benchmarks/dx-www-public-contract-cleanup.test.ts` | ext `.ts` | 3693 bytes
480. `benchmarks/fair-counter/astro/astro.config.mjs` | ext `.mjs` | 105 bytes
481. `benchmarks/fair-counter/astro/package.json` | ext `.json` | 220 bytes
482. `benchmarks/fair-counter/astro/package-lock.json` | ext `.json` | 181932 bytes
483. `benchmarks/fair-counter/astro/src/bench-data.js` | ext `.js` | 1575 bytes
484. `benchmarks/fair-counter/astro/src/components/BenchmarkLayout.astro` | ext `.astro` | 2356 bytes
485. `benchmarks/fair-counter/astro/src/pages/big-dashboard.astro` | ext `.astro` | 1807 bytes
486. `benchmarks/fair-counter/astro/src/pages/index.astro` | ext `.astro` | 3141 bytes
487. `benchmarks/fair-counter/astro/src/pages/medium-cards.astro` | ext `.astro` | 1493 bytes
488. `benchmarks/fair-counter/astro/src/pages/medium-docs.astro` | ext `.astro` | 949 bytes
489. `benchmarks/fair-counter/htmx/package.json` | ext `.json` | 224 bytes
490. `benchmarks/fair-counter/htmx/package-lock.json` | ext `.json` | 566 bytes
491. `benchmarks/fair-counter/htmx/public/index.html` | ext `.html` | 2772 bytes
492. `benchmarks/fair-counter/htmx/server.mjs` | ext `.mjs` | 1962 bytes
493. `benchmarks/fair-counter/measure-fair-counter.ts` | ext `.ts` | 18358 bytes
494. `benchmarks/fair-counter/next/app/bench-data.js` | ext `.js` | 1587 bytes
495. `benchmarks/fair-counter/next/app/big-dashboard/page.jsx` | ext `.jsx` | 215 bytes
496. `benchmarks/fair-counter/next/app/big-dashboard/revenue-dashboard.jsx` | ext `.jsx` | 2128 bytes
497. `benchmarks/fair-counter/next/app/counter.jsx` | ext `.jsx` | 671 bytes
498. `benchmarks/fair-counter/next/app/globals.css` | ext `.css` | 2956 bytes
499. `benchmarks/fair-counter/next/app/layout.jsx` | ext `.jsx` | 313 bytes
500. `benchmarks/fair-counter/next/app/medium-cards/card-catalog.jsx` | ext `.jsx` | 1650 bytes
501. `benchmarks/fair-counter/next/app/medium-cards/page.jsx` | ext `.jsx` | 198 bytes
502. `benchmarks/fair-counter/next/app/medium-docs/page.jsx` | ext `.jsx` | 1140 bytes
503. `benchmarks/fair-counter/next/app/page.jsx` | ext `.jsx` | 468 bytes
504. `benchmarks/fair-counter/next/next.config.mjs` | ext `.mjs` | 54 bytes
505. `benchmarks/fair-counter/next/package.json` | ext `.json` | 274 bytes
506. `benchmarks/fair-counter/next/package-lock.json` | ext `.json` | 30895 bytes
507. `benchmarks/fair-counter/svelte/index.html` | ext `.html` | 423 bytes
508. `benchmarks/fair-counter/svelte/package.json` | ext `.json` | 291 bytes
509. `benchmarks/fair-counter/svelte/package-lock.json` | ext `.json` | 39137 bytes
510. `benchmarks/fair-counter/svelte/src/App.svelte` | ext `.svelte` | 5328 bytes
511. `benchmarks/fair-counter/svelte/src/main.js` | ext `.js` | 153 bytes
512. `benchmarks/fair-counter/svelte/src/style.css` | ext `.css` | 2868 bytes
513. `benchmarks/fair-counter/svelte/vite.config.js` | ext `.js` | 156 bytes
514. `benchmarks/fixtures/build-graph/minimal-app/.dx/deploy/vercel-plan.json` | ext `.json` | 162 bytes
515. `benchmarks/fixtures/build-graph/minimal-app/.dx/forge/source-manifest.json` | ext `.json` | 308 bytes
516. `benchmarks/fixtures/build-graph/minimal-app/.dx/receipts/check/latest.json` | ext `.json` | 135 bytes
517. `benchmarks/fixtures/build-graph/minimal-app/app/page.tsx` | ext `.tsx` | 119 bytes
518. `benchmarks/fixtures/build-graph/minimal-app/components/LaunchPanel.tsx` | ext `.tsx` | 150 bytes
519. `benchmarks/fixtures/build-graph/minimal-app/public/logo.svg` | ext `.svg` | 96 bytes
520. `benchmarks/fixtures/build-graph/minimal-app/styles/app.generated.css` | ext `.css` | 90 bytes
521. `benchmarks/flow-forge-schema-format.test.cjs` | ext `.cjs` | 2387 bytes
522. `benchmarks/flow-forge-workspace-boundary.test.cjs` | ext `.cjs` | 678 bytes
523. `benchmarks/forge-adoption-browser-smoke.test.ts` | ext `.ts` | 2561 bytes
524. `benchmarks/forge-cli-module-boundary.test.ts` | ext `.ts` | 3086 bytes
525. `benchmarks/forge-dashboard-receipt-truth.test.ts` | ext `.ts` | 4981 bytes
526. `benchmarks/forge-golden-path-launch-proof.test.ts` | ext `.ts` | 4796 bytes
527. `benchmarks/forge-installability-snapshot.test.ts` | ext `.ts` | 7352 bytes
528. `benchmarks/forge-large-content.test.ts` | ext `.ts` | 2225 bytes
529. `benchmarks/forge-live-framework-harness.test.ts` | ext `.ts` | 3568 bytes
530. `benchmarks/forge-medium-route.test.ts` | ext `.ts` | 2195 bytes
531. `benchmarks/forge-no-lucide-package-lane.test.ts` | ext `.ts` | 1477 bytes
532. `benchmarks/forge-package-row-maturity-classification.test.ts` | ext `.ts` | 3440 bytes
533. `benchmarks/forge-package-update-rehearsal.test.ts` | ext `.ts` | 3063 bytes
534. `benchmarks/forge-public-routes.test.ts` | ext `.ts` | 1823 bytes
535. `benchmarks/forge-remove-command.test.ts` | ext `.ts` | 1444 bytes
536. `benchmarks/forge-safety-archive-source-guard.test.ts` | ext `.ts` | 5371 bytes
537. `benchmarks/forge-source-owned-package-review.test.ts` | ext `.ts` | 7012 bytes
538. `benchmarks/forge-static-competitors.test.ts` | ext `.ts` | 3447 bytes
539. `benchmarks/forge-status-panel-token-classes.test.cjs` | ext `.cjs` | 3268 bytes
540. `benchmarks/forge-update-local-registry-command.test.ts` | ext `.ts` | 45936 bytes
541. `benchmarks/forms-dx-check-output.test.ts` | ext `.ts` | 3922 bytes
542. `benchmarks/forms-dx-check-package-lane-panel.test.ts` | ext `.ts` | 26738 bytes
543. `benchmarks/forms-package-status-read-model.test.ts` | ext `.ts` | 11486 bytes
544. `benchmarks/forms-react-hook-form-package-doc.test.ts` | ext `.ts` | 5678 bytes
545. `benchmarks/forms-receipt-hash-refresh.test.ts` | ext `.ts` | 17360 bytes
546. `benchmarks/fumadocs-dashboard-workflow.test.ts` | ext `.ts` | 42301 bytes
547. `benchmarks/fumadocs-dx-check-output.test.ts` | ext `.ts` | 5324 bytes
548. `benchmarks/fumadocs-dx-check-package-lane-panel.test.ts` | ext `.ts` | 22084 bytes
549. `benchmarks/fumadocs-llms-route-handler-contract.test.cjs` | ext `.cjs` | 1544 bytes
550. `benchmarks/fumadocs-llms-slice.test.ts` | ext `.ts` | 2198 bytes
551. `benchmarks/fumadocs-navigation-slice.test.ts` | ext `.ts` | 6887 bytes
552. `benchmarks/fumadocs-openapi-code-usage-slice.test.ts` | ext `.ts` | 3635 bytes
553. `benchmarks/fumadocs-openapi-proxy-slice.test.ts` | ext `.ts` | 3606 bytes
554. `benchmarks/fumadocs-openapi-slice.test.ts` | ext `.ts` | 3945 bytes
555. `benchmarks/fumadocs-receipt-hash-refresh.test.ts` | ext `.ts` | 21157 bytes
556. `benchmarks/fumadocs-route-handler-proxy-query-contract.test.cjs` | ext `.cjs` | 1639 bytes
557. `benchmarks/fumadocs-search-slice.test.ts` | ext `.ts` | 3087 bytes
558. `benchmarks/fumadocs-source-plugins-slice.test.ts` | ext `.ts` | 4372 bytes
559. `benchmarks/fumadocs-toc-slice.test.ts` | ext `.ts` | 3477 bytes
560. `benchmarks/generated-artifact-ignore-contract.test.ts` | ext `.ts` | 11421 bytes
561. `benchmarks/installed-smoke-css-asset-output-proof.test.cjs` | ext `.cjs` | 27115 bytes
562. `benchmarks/installed-smoke-manifest-server-data-routes.test.cjs` | ext `.cjs` | 20165 bytes
563. `benchmarks/installed-smoke-route-handler-graph.test.cjs` | ext `.cjs` | 10550 bytes
564. `benchmarks/installed-smoke-route-handler-manifest.test.cjs` | ext `.cjs` | 7731 bytes
565. `benchmarks/installed-smoke-route-handler-readiness.test.cjs` | ext `.cjs` | 1818 bytes
566. `benchmarks/installed-smoke-route-handler-receipts.test.cjs` | ext `.cjs` | 10304 bytes
567. `benchmarks/installed-smoke-route-handler-stale-evidence.test.ts` | ext `.ts` | 21140 bytes
568. `benchmarks/installed-smoke-route-output.test.cjs` | ext `.cjs` | 7698 bytes
569. `benchmarks/installed-smoke-source-map-provenance.test.ts` | ext `.ts` | 7501 bytes
570. `benchmarks/installed-smoke-source-module-resolver.test.cjs` | ext `.cjs` | 10662 bytes
571. `benchmarks/instantdb-auth-actions-slice.test.ts` | ext `.ts` | 2397 bytes
572. `benchmarks/instantdb-auth-boundary-slice.test.ts` | ext `.ts` | 2293 bytes
573. `benchmarks/instantdb-auth-capability-slice.test.ts` | ext `.ts` | 1131 bytes
574. `benchmarks/instantdb-auth-snapshot-slice.test.ts` | ext `.ts` | 2207 bytes
575. `benchmarks/instantdb-batch-mutation-slice.test.ts` | ext `.ts` | 1868 bytes
576. `benchmarks/instantdb-config-env-slice.test.ts` | ext `.ts` | 3592 bytes
577. `benchmarks/instantdb-create-slice.test.ts` | ext `.ts` | 1450 bytes
578. `benchmarks/instantdb-cursors-slice.test.ts` | ext `.ts` | 2576 bytes
579. `benchmarks/instantdb-dashboard-workflow.test.ts` | ext `.ts` | 16460 bytes
580. `benchmarks/instantdb-diagnostics-slice.test.ts` | ext `.ts` | 2319 bytes
581. `benchmarks/instantdb-dx-check-package-lane-panel.test.ts` | ext `.ts` | 17982 bytes
582. `benchmarks/instantdb-dx-check-visibility.test.ts` | ext `.ts` | 10441 bytes
583. `benchmarks/instantdb-dx-style-compatibility.test.ts` | ext `.ts` | 6628 bytes
584. `benchmarks/instantdb-links-slice.test.ts` | ext `.ts` | 1841 bytes
585. `benchmarks/instantdb-merge-slice.test.ts` | ext `.ts` | 1608 bytes
586. `benchmarks/instantdb-next-ssr-slice.test.ts` | ext `.ts` | 3248 bytes
587. `benchmarks/instantdb-oauth-slice.test.ts` | ext `.ts` | 2739 bytes
588. `benchmarks/instantdb-pagination-slice.test.ts` | ext `.ts` | 2354 bytes
589. `benchmarks/instantdb-perms-slice.test.ts` | ext `.ts` | 1512 bytes
590. `benchmarks/instantdb-query-local-slice.test.ts` | ext `.ts` | 2315 bytes
591. `benchmarks/instantdb-receipt-hash-refresh.test.ts` | ext `.ts` | 15586 bytes
592. `benchmarks/instantdb-receipt-hash-visibility.test.ts` | ext `.ts` | 4564 bytes
593. `benchmarks/instantdb-room-topics-slice.test.ts` | ext `.ts` | 1764 bytes
594. `benchmarks/instantdb-route-handler-slice.test.ts` | ext `.ts` | 2474 bytes
595. `benchmarks/instantdb-rules-lookup-slice.test.ts` | ext `.ts` | 2764 bytes
596. `benchmarks/instantdb-rust-dx-check-visibility.test.ts` | ext `.ts` | 3217 bytes
597. `benchmarks/instantdb-status-slice.test.ts` | ext `.ts` | 2162 bytes
598. `benchmarks/instantdb-storage-slice.test.ts` | ext `.ts` | 2634 bytes
599. `benchmarks/instantdb-streams-slice.test.ts` | ext `.ts` | 1847 bytes
600. `benchmarks/instantdb-strict-update-slice.test.ts` | ext `.ts` | 1840 bytes
601. `benchmarks/instantdb-subscriptions-slice.test.ts` | ext `.ts` | 2291 bytes
602. `benchmarks/instantdb-sync-table-slice.test.ts` | ext `.ts` | 3223 bytes
603. `benchmarks/lane12-worktree-ownership.test.cjs` | ext `.cjs` | 38385 bytes
604. `benchmarks/lane4-forge-materialization-cache.test.ts` | ext `.ts` | 7984 bytes
605. `benchmarks/lane4-runtime-safe-readiness-route.test.ts` | ext `.ts` | 22198 bytes
606. `benchmarks/lane7-3d-lock-promotion.test.ts` | ext `.ts` | 7272 bytes
607. `benchmarks/lane7-3d-next-materialized-template.test.ts` | ext `.ts` | 4334 bytes
608. `benchmarks/lane7-default-template-surface.test.ts` | ext `.ts` | 6180 bytes
609. `benchmarks/lane7-motion-lock-promotion.test.ts` | ext `.ts` | 6587 bytes
610. `benchmarks/lane7-motion-source-owned-template.test.ts` | ext `.ts` | 5107 bytes
611. `benchmarks/lane7-webassembly-lock-promotion.test.ts` | ext `.ts` | 5447 bytes
612. `benchmarks/lane7-webassembly-source-owned-template.test.ts` | ext `.ts` | 4250 bytes
613. `benchmarks/launch-budget-config.test.ts` | ext `.ts` | 5938 bytes
614. `benchmarks/launch-compile-gate.test.ts` | ext `.ts` | 31326 bytes
615. `benchmarks/launch-docs-honesty.test.cjs` | ext `.cjs` | 4600 bytes
616. `benchmarks/launch-evidence-acceptance-digest.test.ts` | ext `.ts` | 3118 bytes
617. `benchmarks/launch-evidence-acceptance-index.test.ts` | ext `.ts` | 3144 bytes
618. `benchmarks/launch-evidence-friday-baton.test.ts` | ext `.ts` | 3030 bytes
619. `benchmarks/launch-evidence-restart-closeout.test.ts` | ext `.ts` | 2718 bytes
620. `benchmarks/launch-evidence-restart-dispatch.test.ts` | ext `.ts` | 2679 bytes
621. `benchmarks/launch-evidence-restart-signoff.test.ts` | ext `.ts` | 2675 bytes
622. `benchmarks/launch-evidence-restart-snapshot.test.ts` | ext `.ts` | 2232 bytes
623. `benchmarks/launch-live-runtime-guard.test.ts` | ext `.ts` | 9106 bytes
624. `benchmarks/launch-package-identity.test.ts` | ext `.ts` | 2582 bytes
625. `benchmarks/launch-package-maturity-classification.test.ts` | ext `.ts` | 3657 bytes
626. `benchmarks/launch-package-slices.test.ts` | ext `.ts` | 13650 bytes
627. `benchmarks/launch-readiness-gate.test.ts` | ext `.ts` | 18404 bytes
628. `benchmarks/launch-readme-overclaim-guard.test.cjs` | ext `.cjs` | 947 bytes
629. `benchmarks/launch-receipt-docs-honesty.test.cjs` | ext `.cjs` | 1190 bytes
630. `benchmarks/launch-runtime-materializer.test.ts` | ext `.ts` | 25690 bytes
631. `benchmarks/launch-scene-dashboard-workflow.test.ts` | ext `.ts` | 29213 bytes
632. `benchmarks/launch-scene-readiness.test.ts` | ext `.ts` | 4382 bytes
633. `benchmarks/launch-scene-runtime.test.ts` | ext `.ts` | 38979 bytes
634. `benchmarks/launch-stabilize-coordinator.test.cjs` | ext `.cjs` | 61997 bytes
635. `benchmarks/launch-stabilize-route-probe.test.cjs` | ext `.cjs` | 7444 bytes
636. `benchmarks/markdown-mdx-content-dx-check-hash-refresh.test.ts` | ext `.ts` | 3383 bytes
637. `benchmarks/markdown-mdx-content-receipt-hash-refresh.test.ts` | ext `.ts` | 7623 bytes
638. `benchmarks/markdown-mdx-content-slice.test.ts` | ext `.ts` | 32176 bytes
639. `benchmarks/mdx-docs-source-build-contract.test.cjs` | ext `.cjs` | 7219 bytes
640. `benchmarks/measure-all-size-matrix.ts` | ext `.ts` | 11978 bytes
641. `benchmarks/measure-current-status.ts` | ext `.ts` | 12732 bytes
642. `benchmarks/measure-forge-adoption-browser-smoke.ts` | ext `.ts` | 23612 bytes
643. `benchmarks/measure-forge-installability-snapshot.ts` | ext `.ts` | 35811 bytes
644. `benchmarks/measure-forge-package-update-rehearsal.ts` | ext `.ts` | 15579 bytes
645. `benchmarks/measure-forge-source-owned-package-review.ts` | ext `.ts` | 17477 bytes
646. `benchmarks/measure-framework-scorecard.ts` | ext `.ts` | 11355 bytes
647. `benchmarks/measure-real-routes.ts` | ext `.ts` | 32495 bytes
648. `benchmarks/measure-vertical-proof.ts` | ext `.ts` | 55384 bytes
649. `benchmarks/motion-dashboard-workflow.test.ts` | ext `.ts` | 19510 bytes
650. `benchmarks/motion-dx-check-output.test.ts` | ext `.ts` | 4394 bytes
651. `benchmarks/motion-dx-check-package-lane-panel.test.ts` | ext `.ts` | 21114 bytes
652. `benchmarks/motion-launch-materialized.test.ts` | ext `.ts` | 5175 bytes
653. `benchmarks/motion-launch-proof.test.ts` | ext `.ts` | 8219 bytes
654. `benchmarks/motion-package-status-read-model.test.ts` | ext `.ts` | 7746 bytes
655. `benchmarks/motion-receipt-hash-refresh.test.ts` | ext `.ts` | 16798 bytes
656. `benchmarks/motion-runtime-interaction.test.ts` | ext `.ts` | 12822 bytes
657. `benchmarks/motion-slice.test.ts` | ext `.ts` | 13004 bytes
658. `benchmarks/next-custom-transforms-receipt.test.ts` | ext `.ts` | 11713 bytes
659. `benchmarks/next-intl-dashboard-workflow.test.ts` | ext `.ts` | 32505 bytes
660. `benchmarks/next-intl-dx-check-output.test.ts` | ext `.ts` | 4444 bytes
661. `benchmarks/next-intl-dx-check-package-lane-panel.test.ts` | ext `.ts` | 5262 bytes
662. `benchmarks/next-intl-dx-check-visibility.test.ts` | ext `.ts` | 5246 bytes
663. `benchmarks/next-intl-launch-package-lane-template.test.ts` | ext `.ts` | 20205 bytes
664. `benchmarks/next-intl-package-status-read-model.test.ts` | ext `.ts` | 7850 bytes
665. `benchmarks/next-intl-receipt-hash-refresh.test.ts` | ext `.ts` | 17120 bytes
666. `benchmarks/next-intl-receipt-hash-visibility.test.ts` | ext `.ts` | 6592 bytes
667. `benchmarks/next-intl-slice.test.ts` | ext `.ts` | 30406 bytes
668. `benchmarks/nextjs-compatibility-map.test.ts` | ext `.ts` | 12607 bytes
669. `benchmarks/next-rust-active-scope-artifact-truth.test.ts` | ext `.ts` | 2474 bytes
670. `benchmarks/next-rust-active-scope-cli.test.cjs` | ext `.cjs` | 13833 bytes
671. `benchmarks/next-rust-conflict-markers.test.cjs` | ext `.cjs` | 2698 bytes
672. `benchmarks/next-rust-giant-cli-mod.test.cjs` | ext `.cjs` | 2468 bytes
673. `benchmarks/next-rust-merge-audit-comparison.test.cjs` | ext `.cjs` | 14846 bytes
674. `benchmarks/next-rust-merge-coordinator.test.cjs` | ext `.cjs` | 35810 bytes
675. `benchmarks/next-rust-reference-only-scope.test.cjs` | ext `.cjs` | 2063 bytes
676. `benchmarks/next-rust-schema-status-noise.test.cjs` | ext `.cjs` | 3725 bytes
677. `benchmarks/next-rust-source-map-adapter.test.cjs` | ext `.cjs` | 3216 bytes
678. `benchmarks/next-rust-task-input-adapter.test.cjs` | ext `.cjs` | 3896 bytes
679. `benchmarks/next-rust-vendor-boundary.test.cjs` | ext `.cjs` | 47520 bytes
680. `benchmarks/payments-dx-check-output.test.ts` | ext `.ts` | 3698 bytes
681. `benchmarks/payments-dx-check-package-lane-panel.test.ts` | ext `.ts` | 24352 bytes
682. `benchmarks/payments-dx-check-visibility-receipt.test.ts` | ext `.ts` | 3892 bytes
683. `benchmarks/payments-dx-style-compatibility.test.ts` | ext `.ts` | 4059 bytes
684. `benchmarks/payments-hash-receipt.test.ts` | ext `.ts` | 3557 bytes
685. `benchmarks/payments-package-doc.test.ts` | ext `.ts` | 3612 bytes
686. `benchmarks/payments-package-status-read-model.test.ts` | ext `.ts` | 7160 bytes
687. `benchmarks/payments-receipt-hash-refresh.test.ts` | ext `.ts` | 12398 bytes
688. `benchmarks/payments-stripe-js-package-doc.test.ts` | ext `.ts` | 4066 bytes
689. `benchmarks/project-scan-src-app-contract.test.cjs` | ext `.cjs` | 1449 bytes
690. `benchmarks/provider-route-handler-boundary-truth.test.mjs` | ext `.mjs` | 2966 bytes
691. `benchmarks/public-framework-contract.test.ts` | ext `.ts` | 34390 bytes
692. `benchmarks/public-framework-tools.test.ts` | ext `.ts` | 15930 bytes
693. `benchmarks/public-v1-error-wording.test.cjs` | ext `.cjs` | 610 bytes
694. `benchmarks/reactive-store-dx-check-output.test.ts` | ext `.ts` | 5575 bytes
695. `benchmarks/reactive-store-dx-check-package-lane-panel.test.ts` | ext `.ts` | 14954 bytes
696. `benchmarks/reactive-store-package-status-read-model.test.ts` | ext `.ts` | 9698 bytes
697. `benchmarks/reactive-store-receipt-hash-refresh.test.ts` | ext `.ts` | 16325 bytes
698. `benchmarks/reactive-store-slice.test.ts` | ext `.ts` | 11635 bytes
699. `benchmarks/react-starter-benchmark-scope.test.cjs` | ext `.cjs` | 2244 bytes
700. `benchmarks/record-forge-public-release.ts` | ext `.ts` | 6838 bytes
701. `benchmarks/repo-hygiene-audit.test.ts` | ext `.ts` | 3458 bytes
702. `benchmarks/reports/all-size-framework-comparison.json` | ext `.json` | 10007 bytes
703. `benchmarks/reports/all-size-framework-comparison.md` | ext `.md` | 4766 bytes
704. `benchmarks/reports/binary-web-lab.json` | ext `.json` | 8103 bytes
705. `benchmarks/reports/binary-web-lab.md` | ext `.md` | 3276 bytes
706. `benchmarks/reports/current-status.json` | ext `.json` | 6633 bytes
707. `benchmarks/reports/current-status.md` | ext `.md` | 2177 bytes
708. `benchmarks/reports/dx-www-ecosystem-benchmark-2026-05-18.md` | ext `.md` | 6269 bytes
709. `benchmarks/reports/fair-counter-comparison.json` | ext `.json` | 10561 bytes
710. `benchmarks/reports/fair-counter-comparison.md` | ext `.md` | 2471 bytes
711. `benchmarks/reports/forge-adoption-browser-smoke.json` | ext `.json` | 12155 bytes
712. `benchmarks/reports/forge-adoption-browser-smoke.md` | ext `.md` | 1208 bytes
713. `benchmarks/reports/forge-installability-history/index.json` | ext `.json` | 2804 bytes
714. `benchmarks/reports/forge-installability-history/index.md` | ext `.md` | 1147 bytes
715. `benchmarks/reports/forge-installability-history/snapshots/2026-05-18T18-45-24-641Z.json` | ext `.json` | 5843 bytes
716. `benchmarks/reports/forge-installability-snapshot.json` | ext `.json` | 5843 bytes
717. `benchmarks/reports/forge-installability-snapshot.md` | ext `.md` | 2764 bytes
718. `benchmarks/reports/forge-installability-snapshot.provenance.json` | ext `.json` | 1885 bytes
719. `benchmarks/reports/forge-installability-snapshot.provenance.md` | ext `.md` | 839 bytes
720. `benchmarks/reports/forge-large-content-comparison.json` | ext `.json` | 7296 bytes
721. `benchmarks/reports/forge-large-content-comparison.md` | ext `.md` | 2936 bytes
722. `benchmarks/reports/forge-launch-delivery-comparison.json` | ext `.json` | 3756 bytes
723. `benchmarks/reports/forge-launch-delivery-comparison.md` | ext `.md` | 2129 bytes
724. `benchmarks/reports/forge-live-framework-harness.json` | ext `.json` | 6211 bytes
725. `benchmarks/reports/forge-live-framework-harness.md` | ext `.md` | 2668 bytes
726. `benchmarks/reports/forge-medium-route-comparison.json` | ext `.json` | 6658 bytes
727. `benchmarks/reports/forge-medium-route-comparison.md` | ext `.md` | 2562 bytes
728. `benchmarks/reports/forge-package-update-rehearsal.json` | ext `.json` | 13586 bytes
729. `benchmarks/reports/forge-package-update-rehearsal.md` | ext `.md` | 1302 bytes
730. `benchmarks/reports/forge-package-vertical-comparison.json` | ext `.json` | 1566 bytes
731. `benchmarks/reports/forge-package-vertical-comparison.md` | ext `.md` | 1196 bytes
732. `benchmarks/reports/forge-public-release-history.json` | ext `.json` | 6728 bytes
733. `benchmarks/reports/forge-public-release-history.md` | ext `.md` | 1340 bytes
734. `benchmarks/reports/forge-public-route-comparison.json` | ext `.json` | 6306 bytes
735. `benchmarks/reports/forge-public-route-comparison.md` | ext `.md` | 2515 bytes
736. `benchmarks/reports/forge-release-readiness-trend.json` | ext `.json` | 1490 bytes
737. `benchmarks/reports/forge-release-readiness-trend.md` | ext `.md` | 1519 bytes
738. `benchmarks/reports/forge-source-owned-package-review.json` | ext `.json` | 100147 bytes
739. `benchmarks/reports/forge-source-owned-package-review.md` | ext `.md` | 5839 bytes
740. `benchmarks/reports/forge-static-competitor-evidence.json` | ext `.json` | 28608 bytes
741. `benchmarks/reports/forge-static-competitor-evidence.md` | ext `.md` | 4591 bytes
742. `benchmarks/reports/framework-scorecard.json` | ext `.json` | 5142 bytes
743. `benchmarks/reports/framework-scorecard.md` | ext `.md` | 2980 bytes
744. `benchmarks/reports/game-changing-ideas.md` | ext `.md` | 5569 bytes
745. `benchmarks/reports/latest.json` | ext `.json` | 6633 bytes
746. `benchmarks/reports/latest.md` | ext `.md` | 2177 bytes
747. `benchmarks/reports/latest-compression.json` | ext `.json` | 971 bytes
748. `benchmarks/reports/real-route-comparison.json` | ext `.json` | 119195 bytes
749. `benchmarks/reports/real-route-comparison.md` | ext `.md` | 3373 bytes
750. `benchmarks/reports/vertical-proof-history/20260516T082918Z-forge-icon.json` | ext `.json` | 6544 bytes
751. `benchmarks/reports/vertical-proof-history/20260516T082918Z-forge-icon.md` | ext `.md` | 1477 bytes
752. `benchmarks/reports/vertical-proof-history/20260516T082940Z-forge-icon.json` | ext `.json` | 9450 bytes
753. `benchmarks/reports/vertical-proof-history/20260516T082940Z-forge-icon.md` | ext `.md` | 1467 bytes
754. `benchmarks/reports/vertical-proof-history/20260516T083032Z-forge-icon.json` | ext `.json` | 9510 bytes
755. `benchmarks/reports/vertical-proof-history/20260516T083032Z-forge-icon.md` | ext `.md` | 1464 bytes
756. `benchmarks/reports/vertical-proof-history/20260516T093129Z-forge-combo.json` | ext `.json` | 9921 bytes
757. `benchmarks/reports/vertical-proof-history/20260516T093129Z-forge-combo.md` | ext `.md` | 1481 bytes
758. `benchmarks/reports/vertical-proof-history/20260516T134057Z-forge-site.json` | ext `.json` | 10146 bytes
759. `benchmarks/reports/vertical-proof-history/20260516T134057Z-forge-site.md` | ext `.md` | 1526 bytes
760. `benchmarks/reports/vertical-proof-history/20260516T134412Z-forge-site.json` | ext `.json` | 10867 bytes
761. `benchmarks/reports/vertical-proof-history/20260516T134412Z-forge-site.md` | ext `.md` | 1910 bytes
762. `benchmarks/reports/vertical-proof-history/20260516T140546Z-forge-site.json` | ext `.json` | 10229 bytes
763. `benchmarks/reports/vertical-proof-history/20260516T140546Z-forge-site.md` | ext `.md` | 2004 bytes
764. `benchmarks/reports/vertical-proof-history/20260516T142234Z-forge-site.json` | ext `.json` | 10186 bytes
765. `benchmarks/reports/vertical-proof-history/20260516T142234Z-forge-site.md` | ext `.md` | 1996 bytes
766. `benchmarks/reports/vertical-proof-history/20260516T161641Z-forge-site.json` | ext `.json` | 8776 bytes
767. `benchmarks/reports/vertical-proof-history/20260516T161641Z-forge-site.md` | ext `.md` | 2057 bytes
768. `benchmarks/reports/vertical-proof-history/20260516T161649Z-forge-scorecard.json` | ext `.json` | 7287 bytes
769. `benchmarks/reports/vertical-proof-history/20260516T161649Z-forge-scorecard.md` | ext `.md` | 1711 bytes
770. `benchmarks/reports/vertical-proof-history/20260516T161655Z-forge-ci.json` | ext `.json` | 7170 bytes
771. `benchmarks/reports/vertical-proof-history/20260516T161655Z-forge-ci.md` | ext `.md` | 1681 bytes
772. `benchmarks/reports/vertical-proof-history/20260516T170339Z-forge-site.json` | ext `.json` | 10218 bytes
773. `benchmarks/reports/vertical-proof-history/20260516T170339Z-forge-site.md` | ext `.md` | 2078 bytes
774. `benchmarks/reports/vertical-proof-history/20260516T170342Z-forge-scorecard.json` | ext `.json` | 9499 bytes
775. `benchmarks/reports/vertical-proof-history/20260516T170342Z-forge-scorecard.md` | ext `.md` | 2119 bytes
776. `benchmarks/reports/vertical-proof-history/20260516T170346Z-forge-ci.json` | ext `.json` | 9344 bytes
777. `benchmarks/reports/vertical-proof-history/20260516T170346Z-forge-ci.md` | ext `.md` | 2081 bytes
778. `benchmarks/reports/vertical-proof-history/20260516T170410Z-forge-evidence.json` | ext `.json` | 11321 bytes
779. `benchmarks/reports/vertical-proof-history/20260516T170410Z-forge-evidence.md` | ext `.md` | 2115 bytes
780. `benchmarks/reports/vertical-proof-history/20260516T175834Z-forge-site.json` | ext `.json` | 10219 bytes
781. `benchmarks/reports/vertical-proof-history/20260516T175834Z-forge-site.md` | ext `.md` | 2086 bytes
782. `benchmarks/reports/vertical-proof-history/20260516T175837Z-forge-scorecard.json` | ext `.json` | 9485 bytes
783. `benchmarks/reports/vertical-proof-history/20260516T175837Z-forge-scorecard.md` | ext `.md` | 2122 bytes
784. `benchmarks/reports/vertical-proof-history/20260516T175840Z-forge-ci.json` | ext `.json` | 9344 bytes
785. `benchmarks/reports/vertical-proof-history/20260516T175840Z-forge-ci.md` | ext `.md` | 2085 bytes
786. `benchmarks/reports/vertical-proof-history/20260516T175842Z-forge-evidence.json` | ext `.json` | 11284 bytes
787. `benchmarks/reports/vertical-proof-history/20260516T175842Z-forge-evidence.md` | ext `.md` | 2113 bytes
788. `benchmarks/reports/vertical-proof-history/20260516T185225Z-forge-releases.json` | ext `.json` | 9455 bytes
789. `benchmarks/reports/vertical-proof-history/20260516T185225Z-forge-releases.md` | ext `.md` | 2140 bytes
790. `benchmarks/reports/vertical-proof-history/20260516T204543Z-forge-changelog.json` | ext `.json` | 9490 bytes
791. `benchmarks/reports/vertical-proof-history/20260516T204543Z-forge-changelog.md` | ext `.md` | 2156 bytes
792. `benchmarks/reports/vertical-proof-history/20260517T082145Z-forge-adoption.json` | ext `.json` | 9510 bytes
793. `benchmarks/reports/vertical-proof-history/20260517T082145Z-forge-adoption.md` | ext `.md` | 2173 bytes
794. `benchmarks/reports/vertical-proof-history/index.json` | ext `.json` | 14347 bytes
795. `benchmarks/reports/vertical-proof-history/index.md` | ext `.md` | 3879 bytes
796. `benchmarks/reports/vertical-proof-measurement.json` | ext `.json` | 9510 bytes
797. `benchmarks/reports/vertical-proof-measurement.md` | ext `.md` | 2173 bytes
798. `benchmarks/reports/vertical-proof-triage.md` | ext `.md` | 625 bytes
799. `benchmarks/report-snapshot-status.js` | ext `.js` | 1333 bytes
800. `benchmarks/route-css-cache-contract.test.ts` | ext `.ts` | 917 bytes
801. `benchmarks/route-handler-cookie-direct-read.test.cjs` | ext `.cjs` | 1977 bytes
802. `benchmarks/route-handler-formdata-live-black-box.test.ts` | ext `.ts` | 8528 bytes
803. `benchmarks/route-handler-header-destructure.test.cjs` | ext `.cjs` | 1592 bytes
804. `benchmarks/route-handler-head-method-parity.test.mjs` | ext `.mjs` | 1784 bytes
805. `benchmarks/route-handler-json-destructure-alias.test.cjs` | ext `.cjs` | 1802 bytes
806. `benchmarks/route-handler-live-dev-black-box.test.ts` | ext `.ts` | 11279 bytes
807. `benchmarks/route-handler-readiness-interpreters.test.mjs` | ext `.mjs` | 29864 bytes
808. `benchmarks/route-handler-readiness-path-normalization.test.cjs` | ext `.cjs` | 1008 bytes
809. `benchmarks/route-handler-request-clone-body.test.cjs` | ext `.cjs` | 5176 bytes
810. `benchmarks/route-handler-response-header-aliases.test.cjs` | ext `.cjs` | 1841 bytes
811. `benchmarks/route-handler-search-params-destructure.test.cjs` | ext `.cjs` | 1437 bytes
812. `benchmarks/route-handler-source-owned-receipts.test.cjs` | ext `.cjs` | 7559 bytes
813. `benchmarks/route-handler-text-body-projection.test.ts` | ext `.ts` | 2486 bytes
814. `benchmarks/route-handler-typed-body-alias.test.cjs` | ext `.cjs` | 3256 bytes
815. `benchmarks/route-handler-typed-request-aliases.test.ts` | ext `.ts` | 2218 bytes
816. `benchmarks/route-handler-unsupported-body-diagnostics.test.ts` | ext `.ts` | 2791 bytes
817. `benchmarks/route-handler-url-search-param-fallback.test.ts` | ext `.ts` | 2108 bytes
818. `benchmarks/rust-warning-inventory-contract.test.ts` | ext `.ts` | 2615 bytes
819. `benchmarks/shadcn-dashboard-workflow.test.ts` | ext `.ts` | 6663 bytes
820. `benchmarks/shadcn-item-launch-usage.test.ts` | ext `.ts` | 1144 bytes
821. `benchmarks/shadcn-launch-dashboard-workflow.test.ts` | ext `.ts` | 14911 bytes
822. `benchmarks/source-build-app-router-route-segments.test.cjs` | ext `.cjs` | 1924 bytes
823. `benchmarks/source-build-app-router-route-shape-validation.test.cjs` | ext `.cjs` | 1468 bytes
824. `benchmarks/source-build-css-import-invalidation-contract.test.cjs` | ext `.cjs` | 1536 bytes
825. `benchmarks/source-build-css-source-map-artifact-contract.test.cjs` | ext `.cjs` | 1370 bytes
826. `benchmarks/source-build-ecmascript-reference-only-contract.test.cjs` | ext `.cjs` | 1261 bytes
827. `benchmarks/source-build-emitted-output-artifacts-contract.test.cjs` | ext `.cjs` | 1600 bytes
828. `benchmarks/source-build-emitted-output-invalidation-contract.test.cjs` | ext `.cjs` | 1238 bytes
829. `benchmarks/source-build-graph-source-modules.test.cjs` | ext `.cjs` | 1606 bytes
830. `benchmarks/source-build-image-metadata-source-guard.test.cjs` | ext `.cjs` | 17748 bytes
831. `benchmarks/source-build-route-handler-readiness-contract.test.cjs` | ext `.cjs` | 1883 bytes
832. `benchmarks/source-build-route-output-layout-segments.test.cjs` | ext `.cjs` | 1665 bytes
833. `benchmarks/source-build-server-data-readiness-contract.test.cjs` | ext `.cjs` | 16199 bytes
834. `benchmarks/source-build-server-data-route-params.test.cjs` | ext `.cjs` | 2171 bytes
835. `benchmarks/source-resolver-config-extends-boundary.test.ts` | ext `.ts` | 2583 bytes
836. `benchmarks/source-resolver-config-extends-source-guard.test.cjs` | ext `.cjs` | 3200 bytes
837. `benchmarks/source-resolver-config-source-guard.test.cjs` | ext `.cjs` | 25084 bytes
838. `benchmarks/source-resolver-package-export-boundary.test.cjs` | ext `.cjs` | 1599 bytes
839. `benchmarks/source-resolver-source-condition-guard.test.cjs` | ext `.cjs` | 3318 bytes
840. `benchmarks/src/lib.rs` | ext `.rs` | 2242 bytes
841. `benchmarks/state-management-receipt-hash-refresh.test.ts` | ext `.ts` | 12508 bytes
842. `benchmarks/stripe-payment-launch-proof.test.ts` | ext `.ts` | 12809 bytes
843. `benchmarks/stripe-rhf-checkout-flow.test.ts` | ext `.ts` | 42231 bytes
844. `benchmarks/supabase-dashboard-workflow.test.ts` | ext `.ts` | 20542 bytes
845. `benchmarks/supabase-dx-check-output.test.ts` | ext `.ts` | 7075 bytes
846. `benchmarks/supabase-dx-check-package-lane-panel.test.ts` | ext `.ts` | 20249 bytes
847. `benchmarks/supabase-env-boundary.test.ts` | ext `.ts` | 43208 bytes
848. `benchmarks/supabase-launch-visible-proof.test.ts` | ext `.ts` | 3997 bytes
849. `benchmarks/supabase-package-status-read-model.test.ts` | ext `.ts` | 15636 bytes
850. `benchmarks/supabase-receipt-hash-refresh.test.ts` | ext `.ts` | 12053 bytes
851. `benchmarks/supply-chain-boundary-contract.test.ts` | ext `.ts` | 6141 bytes
852. `benchmarks/tanstack-query-dashboard-workflow.test.ts` | ext `.ts` | 10022 bytes
853. `benchmarks/tanstack-query-dx-check-output.test.ts` | ext `.ts` | 4079 bytes
854. `benchmarks/tanstack-query-dx-check-package-lane-panel.test.ts` | ext `.ts` | 21524 bytes
855. `benchmarks/tanstack-query-dx-style-compatibility.test.ts` | ext `.ts` | 5788 bytes
856. `benchmarks/tanstack-query-hash-receipt.test.ts` | ext `.ts` | 5151 bytes
857. `benchmarks/tanstack-query-receipt-hash-refresh.test.ts` | ext `.ts` | 17137 bytes
858. `benchmarks/tanstack-query-slice.test.ts` | ext `.ts` | 63216 bytes
859. `benchmarks/temp-cache-ignore-hygiene.test.ts` | ext `.ts` | 2064 bytes
860. `benchmarks/template-forms-validation-receipt-wiring.test.ts` | ext `.ts` | 3746 bytes
861. `benchmarks/template-readiness-execution-proof.test.ts` | ext `.ts` | 9678 bytes
862. `benchmarks/template-shell.test.ts` | ext `.ts` | 115733 bytes
863. `benchmarks/test-format-standard.test.ts` | ext `.ts` | 2465 bytes
864. `benchmarks/three-scene-dx-check-output.test.ts` | ext `.ts` | 6060 bytes
865. `benchmarks/three-scene-dx-check-package-lane-panel.test.ts` | ext `.ts` | 15901 bytes
866. `benchmarks/three-scene-package-doc.test.ts` | ext `.ts` | 9471 bytes
867. `benchmarks/three-scene-receipt-hash-refresh.test.ts` | ext `.ts` | 22427 bytes
868. `benchmarks/trpc-dashboard-workflow.test.ts` | ext `.ts` | 7642 bytes
869. `benchmarks/trpc-dx-check-package-lane-panel.test.ts` | ext `.ts` | 20992 bytes
870. `benchmarks/trpc-dx-check-visibility-receipt.test.ts` | ext `.ts` | 5762 bytes
871. `benchmarks/trpc-forge-slice.test.ts` | ext `.ts` | 31423 bytes
872. `benchmarks/trpc-launch-runtime-proof.test.ts` | ext `.ts` | 14049 bytes
873. `benchmarks/trpc-package-status-read-model.test.ts` | ext `.ts` | 10273 bytes
874. `benchmarks/trpc-receipt-hash-refresh.test.ts` | ext `.ts` | 24916 bytes
875. `benchmarks/trpc-rust-dx-check-metrics.test.ts` | ext `.ts` | 3281 bytes
876. `benchmarks/tsx-app-router-black-box-fixtures.test.ts` | ext `.ts` | 18448 bytes
877. `benchmarks/tsx-app-router-boundary-wrapper-scope.test.ts` | ext `.ts` | 1502 bytes
878. `benchmarks/tsx-app-router-client-metadata-diagnostics.test.cjs` | ext `.cjs` | 1478 bytes
879. `benchmarks/tsx-app-router-error-boundary.test.cjs` | ext `.cjs` | 1013 bytes
880. `benchmarks/tsx-app-router-error-boundary-props.test.cjs` | ext `.cjs` | 1083 bytes
881. `benchmarks/tsx-app-router-loading-boundary.test.cjs` | ext `.cjs` | 1045 bytes
882. `benchmarks/tsx-app-router-metadata-head-tags.test.cjs` | ext `.cjs` | 1053 bytes
883. `benchmarks/tsx-app-router-metadata-merge.test.cjs` | ext `.cjs` | 1374 bytes
884. `benchmarks/tsx-app-router-metadata-open-graph.test.cjs` | ext `.cjs` | 1030 bytes
885. `benchmarks/tsx-app-router-metadata-request-props.test.ts` | ext `.ts` | 8275 bytes
886. `benchmarks/tsx-app-router-metadata-viewport.test.cjs` | ext `.cjs` | 1067 bytes
887. `benchmarks/tsx-app-router-navigation-control-flow.test.ts` | ext `.ts` | 2184 bytes
888. `benchmarks/tsx-app-router-not-found-boundary.test.cjs` | ext `.cjs` | 1454 bytes
889. `benchmarks/tsx-app-router-page-props.test.ts` | ext `.ts` | 8206 bytes
890. `benchmarks/tsx-app-router-redirect-boundary.test.cjs` | ext `.cjs` | 2246 bytes
891. `benchmarks/tsx-app-router-render-plan-runtime-readiness.test.ts` | ext `.ts` | 1557 bytes
892. `benchmarks/tsx-app-router-render-search-params.test.cjs` | ext `.cjs` | 1028 bytes
893. `benchmarks/tsx-app-router-state-runtime-operations.test.ts` | ext `.ts` | 1008 bytes
894. `benchmarks/tsx-app-router-static-metadata-export-surface.test.ts` | ext `.ts` | 3502 bytes
895. `benchmarks/tsx-app-router-template-boundary.test.cjs` | ext `.cjs` | 1061 bytes
896. `benchmarks/tsx-app-router-viewport-export-diagnostics.test.cjs` | ext `.cjs` | 1775 bytes
897. `benchmarks/tsx-next-image-compat.test.ts` | ext `.ts` | 1177 bytes
898. `benchmarks/tsx-next-link-compat.test.ts` | ext `.ts` | 1231 bytes
899. `benchmarks/ui-components-dx-check-output.test.ts` | ext `.ts` | 4610 bytes
900. `benchmarks/ui-components-dx-check-package-lane-panel.test.ts` | ext `.ts` | 21677 bytes
901. `benchmarks/ui-components-package-doc.test.ts` | ext `.ts` | 10158 bytes
902. `benchmarks/ui-components-package-status-read-model.test.ts` | ext `.ts` | 8240 bytes
903. `benchmarks/ui-components-receipt-hash-refresh.test.ts` | ext `.ts` | 10582 bytes
904. `benchmarks/vercel-ai-agent-slice.test.ts` | ext `.ts` | 2502 bytes
905. `benchmarks/vercel-ai-dashboard-workflow.test.ts` | ext `.ts` | 6382 bytes
906. `benchmarks/vercel-ai-dx-check-package-lane-panel.test.ts` | ext `.ts` | 13426 bytes
907. `benchmarks/vercel-ai-dx-check-visibility-receipt.test.ts` | ext `.ts` | 5015 bytes
908. `benchmarks/vercel-ai-dx-style-compatibility.test.ts` | ext `.ts` | 4669 bytes
909. `benchmarks/vercel-ai-embeddings-slice.test.ts` | ext `.ts` | 1863 bytes
910. `benchmarks/vercel-ai-image-generation-slice.test.ts` | ext `.ts` | 2268 bytes
911. `benchmarks/vercel-ai-launch-visible-proof.test.ts` | ext `.ts` | 17987 bytes
912. `benchmarks/vercel-ai-message-pruning-slice.test.ts` | ext `.ts` | 1932 bytes
913. `benchmarks/vercel-ai-model-policy-slice.test.ts` | ext `.ts` | 2407 bytes
914. `benchmarks/vercel-ai-object-generation-slice.test.ts` | ext `.ts` | 2755 bytes
915. `benchmarks/vercel-ai-official-naming.test.ts` | ext `.ts` | 4485 bytes
916. `benchmarks/vercel-ai-package-status-read-model.test.ts` | ext `.ts` | 4382 bytes
917. `benchmarks/vercel-ai-provider-freedom-slice.test.ts` | ext `.ts` | 1940 bytes
918. `benchmarks/vercel-ai-receipt-hash-refresh.test.ts` | ext `.ts` | 21997 bytes
919. `benchmarks/vercel-ai-rerank-slice.test.ts` | ext `.ts` | 2079 bytes
920. `benchmarks/vercel-ai-rust-hash-helper.test.ts` | ext `.ts` | 2187 bytes
921. `benchmarks/vercel-ai-speech-generation-slice.test.ts` | ext `.ts` | 2860 bytes
922. `benchmarks/vercel-ai-structured-output-slice.test.ts` | ext `.ts` | 1758 bytes
923. `benchmarks/vercel-ai-telemetry-slice.test.ts` | ext `.ts` | 2566 bytes
924. `benchmarks/vercel-ai-text-stream-slice.test.ts` | ext `.ts` | 2051 bytes
925. `benchmarks/vercel-ai-tool-approval-slice.test.ts` | ext `.ts` | 2632 bytes
926. `benchmarks/vercel-ai-transcription-slice.test.ts` | ext `.ts` | 2531 bytes
927. `benchmarks/vercel-ai-ui-message-stream-slice.test.ts` | ext `.ts` | 2663 bytes
928. `benchmarks/vercel-ai-upload-file-slice.test.ts` | ext `.ts` | 1999 bytes
929. `benchmarks/vercel-ai-video-generation-slice.test.ts` | ext `.ts` | 2725 bytes
930. `benchmarks/vertical-proof/components/ProofCard.tsx` | ext `.tsx` | 592 bytes
931. `benchmarks/vertical-proof/pages/index.html` | ext `.html` | 1333 bytes
932. `benchmarks/wasm-bindgen-dx-check-output.test.ts` | ext `.ts` | 3876 bytes
933. `benchmarks/wasm-bindgen-dx-check-package-lane-panel.test.ts` | ext `.ts` | 32038 bytes
934. `benchmarks/wasm-bindgen-hash-receipt.test.ts` | ext `.ts` | 8873 bytes
935. `benchmarks/wasm-bindgen-receipt-hash-refresh.test.ts` | ext `.ts` | 19366 bytes
936. `benchmarks/wasm-bindgen-slice.test.ts` | ext `.ts` | 23724 bytes
937. `benchmarks/web-perf-receipt-mode-contract.test.ts` | ext `.ts` | 1005 bytes
938. `benchmarks/www-agent1-process-proof.test.ts` | ext `.ts` | 5584 bytes
939. `benchmarks/www-agent2-ownership-map.test.ts` | ext `.ts` | 3304 bytes
940. `benchmarks/www-forge-media-restore.test.ts` | ext `.ts` | 4917 bytes
941. `benchmarks/www-forge-package-lock.test.ts` | ext `.ts` | 18904 bytes
942. `benchmarks/www-forge-package-status-read-model.test.ts` | ext `.ts` | 25792 bytes
943. `benchmarks/www-forge-remote-status.test.ts` | ext `.ts` | 4407 bytes
944. `benchmarks/www-forge-vcs-status.test.ts` | ext `.ts` | 4404 bytes
945. `benchmarks/www-onboard-conversion.test.cjs` | ext `.cjs` | 5555 bytes
946. `benchmarks/www-template-app-routes.test.ts` | ext `.ts` | 31552 bytes
947. `benchmarks/www-template-audit-cleanup.test.ts` | ext `.ts` | 23468 bytes
948. `benchmarks/www-template-content-docs-i18n-lock-promotion.test.ts` | ext `.ts` | 11731 bytes
949. `benchmarks/www-template-forge-reality.test.ts` | ext `.ts` | 40088 bytes
950. `benchmarks/www-template-forms-lock-promotion.test.ts` | ext `.ts` | 4331 bytes
951. `benchmarks/www-template-friday-particle-avatar.test.ts` | ext `.ts` | 2301 bytes
952. `benchmarks/www-template-lane5-package-reality.test.ts` | ext `.ts` | 12690 bytes
953. `benchmarks/www-template-link-integrity.test.ts` | ext `.ts` | 4877 bytes
954. `benchmarks/www-template-native-scroll.test.ts` | ext `.ts` | 14634 bytes
955. `benchmarks/www-template-next-config-boundary.test.ts` | ext `.ts` | 6142 bytes
956. `benchmarks/www-template-readiness-receipts.test.ts` | ext `.ts` | 5894 bytes
957. `benchmarks/www-template-receipt-helper-runner.test.ts` | ext `.ts` | 2557 bytes
958. `benchmarks/www-template-runtime-product-qa.test.ts` | ext `.ts` | 3666 bytes
959. `benchmarks/www-template-score-gate.test.ts` | ext `.ts` | 11303 bytes
960. `benchmarks/www-template-signature-effects.test.ts` | ext `.ts` | 3161 bytes
961. `benchmarks/www-template-source-honesty.test.ts` | ext `.ts` | 13873 bytes
962. `benchmarks/www-template-state-data-reality.test.ts` | ext `.ts` | 21883 bytes
963. `benchmarks/www-template-validation-lock-promotion.test.ts` | ext `.ts` | 4181 bytes
964. `benchmarks/zod-catalog-record-slice.test.ts` | ext `.ts` | 1985 bytes
965. `benchmarks/zod-codec-slice.test.ts` | ext `.ts` | 1768 bytes
966. `benchmarks/zod-coerce-input-slice.test.ts` | ext `.ts` | 2280 bytes
967. `benchmarks/zod-dashboard-settings-workflow.test.ts` | ext `.ts` | 25035 bytes
968. `benchmarks/zod-dx-check-package-lane-panel.test.ts` | ext `.ts` | 18339 bytes
969. `benchmarks/zod-dx-style-compatibility.test.ts` | ext `.ts` | 7016 bytes
970. `benchmarks/zod-error-policy-slice.test.ts` | ext `.ts` | 2289 bytes
971. `benchmarks/zod-file-slice.test.ts` | ext `.ts` | 1745 bytes
972. `benchmarks/zod-json-schema-import-slice.test.ts` | ext `.ts` | 2320 bytes
973. `benchmarks/zod-launch-visible-proof.test.ts` | ext `.ts` | 8703 bytes
974. `benchmarks/zod-object-composition-slice.test.ts` | ext `.ts` | 2279 bytes
975. `benchmarks/zod-package-status-read-model.test.ts` | ext `.ts` | 12895 bytes
976. `benchmarks/zod-receipt-hash-refresh.test.ts` | ext `.ts` | 13879 bytes
977. `benchmarks/zod-refinement-slice.test.ts` | ext `.ts` | 2027 bytes
978. `benchmarks/zod-registry-slice.test.ts` | ext `.ts` | 1586 bytes
979. `benchmarks/zod-stringbool-env-slice.test.ts` | ext `.ts` | 1390 bytes
980. `benchmarks/zod-template-literal-slice.test.ts` | ext `.ts` | 1461 bytes
981. `benchmarks/zod-transform-slice.test.ts` | ext `.ts` | 1823 bytes
982. `benchmarks/zustand-dx-check-package-lane-panel.test.ts` | ext `.ts` | 17988 bytes
983. `benchmarks/zustand-dx-style-compatibility.test.ts` | ext `.ts` | 4807 bytes
984. `benchmarks/zustand-launch-materialized.test.ts` | ext `.ts` | 9954 bytes
985. `benchmarks/zustand-slice.test.ts` | ext `.ts` | 36147 bytes
986. `binary/Cargo.toml` | ext `.toml` | 1457 bytes
987. `binary/CHANGELOG.md` | ext `.md` | 376 bytes
988. `binary/LICENSE` | ext `<none>` | 559 bytes
989. `binary/LICENSE-APACHE` | ext `<none>` | 5871 bytes
990. `binary/LICENSE-MIT` | ext `<none>` | 1093 bytes
991. `binary/README.md` | ext `.md` | 475 bytes
992. `binary/src/codec.rs` | ext `.rs` | 10278 bytes
993. `binary/src/delta.rs` | ext `.rs` | 21624 bytes
994. `binary/src/deserializer.rs` | ext `.rs` | 7969 bytes
995. `binary/src/htip_bridge.rs` | ext `.rs` | 14648 bytes
996. `binary/src/lib.rs` | ext `.rs` | 6749 bytes
997. `binary/src/opcodes.rs` | ext `.rs` | 21043 bytes
998. `binary/src/protocol.rs` | ext `.rs` | 6152 bytes
999. `binary/src/serializer.rs` | ext `.rs` | 7305 bytes
1000. `binary/src/signature.rs` | ext `.rs` | 2024 bytes
1001. `binary/src/string_table.rs` | ext `.rs` | 7787 bytes
1002. `binary/src/template.rs` | ext `.rs` | 2673 bytes
1003. `binary/tests/delta_property_tests.rs` | ext `.rs` | 13144 bytes
1004. `binary/tests/integration.rs` | ext `.rs` | 6152 bytes
1005. `binary/tests/validation_property_tests.rs` | ext `.rs` | 16077 bytes
1006. `browser/Cargo.toml` | ext `.toml` | 839 bytes
1007. `browser/CHANGELOG.md` | ext `.md` | 376 bytes
1008. `browser/js/dx-browser-host.js` | ext `.js` | 12151 bytes
1009. `browser/js/dx-loader.js` | ext `.js` | 6181 bytes
1010. `browser/LICENSE` | ext `<none>` | 559 bytes
1011. `browser/LICENSE-APACHE` | ext `<none>` | 5871 bytes
1012. `browser/LICENSE-MIT` | ext `<none>` | 1093 bytes
1013. `browser/README.md` | ext `.md` | 429 bytes
1014. `browser/src/allocator.rs` | ext `.rs` | 1643 bytes
1015. `browser/src/ecosystem.rs` | ext `.rs` | 4187 bytes
1016. `browser/src/lib.rs` | ext `.rs` | 16143 bytes
1017. `browser/src/node_registry.rs` | ext `.rs` | 1694 bytes
1018. `browser/src/patcher.rs` | ext `.rs` | 13824 bytes
1019. `browser/src/renderer.rs` | ext `.rs` | 10252 bytes
1020. `browser/src/stream_reader.rs` | ext `.rs` | 14660 bytes
1021. `browser/src/string_table.rs` | ext `.rs` | 2001 bytes
1022. `browser/src/style_loader.rs` | ext `.rs` | 10318 bytes
1023. `browser/src/template_cache.rs` | ext `.rs` | 2271 bytes
1024. `browser-micro/Cargo.toml` | ext `.toml` | 624 bytes
1025. `browser-micro/CHANGELOG.md` | ext `.md` | 382 bytes
1026. `browser-micro/LICENSE` | ext `<none>` | 559 bytes
1027. `browser-micro/LICENSE-APACHE` | ext `<none>` | 5871 bytes
1028. `browser-micro/LICENSE-MIT` | ext `<none>` | 1093 bytes
1029. `browser-micro/README.md` | ext `.md` | 405 bytes
1030. `browser-micro/src/allocator.rs` | ext `.rs` | 1134 bytes
1031. `browser-micro/src/lib.rs` | ext `.rs` | 1375 bytes
1032. `cache/Cargo.toml` | ext `.toml` | 1505 bytes
1033. `cache/CHANGELOG.md` | ext `.md` | 375 bytes
1034. `cache/LICENSE` | ext `<none>` | 559 bytes
1035. `cache/LICENSE-APACHE` | ext `<none>` | 5871 bytes
1036. `cache/LICENSE-MIT` | ext `<none>` | 1093 bytes
1037. `cache/README.md` | ext `.md` | 365 bytes
1038. `cache/src/crypto/keypair.rs` | ext `.rs` | 124 bytes
1039. `cache/src/crypto/mod.rs` | ext `.rs` | 2496 bytes
1040. `cache/src/crypto/verify.rs` | ext `.rs` | 92 bytes
1041. `cache/src/lib.rs` | ext `.rs` | 4268 bytes
1042. `cache/src/preload/mod.rs` | ext `.rs` | 2890 bytes
1043. `cache/src/storage/cache_api.rs` | ext `.rs` | 5304 bytes
1044. `cache/src/storage/indexeddb.rs` | ext `.rs` | 11463 bytes
1045. `cache/src/storage/mod.rs` | ext `.rs` | 3580 bytes
1046. `Cargo.lock` | ext `.lock` | 209307 bytes
1047. `Cargo.toml` | ext `.toml` | 3676 bytes
1048. `CHANGELOG.md` | ext `.md` | 1093793 bytes
1049. `cli/Cargo.lock` | ext `.lock` | 166624 bytes
1050. `cli/Cargo.toml` | ext `.toml` | 858 bytes
1051. `cli/README.md` | ext `.md` | 2984 bytes
1052. `cli/src/build.rs` | ext `.rs` | 5390 bytes
1053. `cli/src/dev.rs` | ext `.rs` | 8564 bytes
1054. `cli/src/main.rs` | ext `.rs` | 8048 bytes
1055. `cli/src/utils.rs` | ext `.rs` | 2972 bytes
1056. `components/icons/README.md` | ext `.md` | 196 bytes
1057. `components/icons/search.tsx` | ext `.tsx` | 496 bytes
1058. `components/ui/button.tsx` | ext `.tsx` | 1976 bytes
1059. `components/ui/README.md` | ext `.md` | 183 bytes
1060. `components/ui/slot.tsx` | ext `.tsx` | 642 bytes
1061. `CONTRIBUTING.md` | ext `.md` | 3959 bytes
1062. `core/.kiro/specs/cross-platform-io-reactor/design.md` | ext `.md` | 11352 bytes
1063. `core/.kiro/specs/cross-platform-io-reactor/requirements.md` | ext `.md` | 7986 bytes
1064. `core/.kiro/specs/cross-platform-io-reactor/tasks.md` | ext `.md` | 9408 bytes
1065. `core/Cargo.toml` | ext `.toml` | 2724 bytes
1066. `core/CHANGELOG.md` | ext `.md` | 369 bytes
1067. `core/LICENSE` | ext `<none>` | 559 bytes
1068. `core/LICENSE-APACHE` | ext `<none>` | 5871 bytes
1069. `core/LICENSE-MIT` | ext `<none>` | 1093 bytes
1070. `core/README.md` | ext `.md` | 11649 bytes
1071. `core/src/admin.rs` | ext `.rs` | 16554 bytes
1072. `core/src/analyzer.rs` | ext `.rs` | 9791 bytes
1073. `core/src/animation.rs` | ext `.rs` | 17619 bytes
1074. `core/src/binary_compiler.rs` | ext `.rs` | 37458 bytes
1075. `core/src/cmd.rs` | ext `.rs` | 8326 bytes
1076. `core/src/code_splitting.rs` | ext `.rs` | 18536 bytes
1077. `core/src/codegen.rs` | ext `.rs` | 6135 bytes
1078. `core/src/codegen_macro.rs` | ext `.rs` | 14080 bytes
1079. `core/src/codegen_micro.rs` | ext `.rs` | 19332 bytes
1080. `core/src/components.rs` | ext `.rs` | 56656 bytes
1081. `core/src/config.rs` | ext `.rs` | 9948 bytes
1082. `core/src/content.rs` | ext `.rs` | 19080 bytes
1083. `core/src/control.rs` | ext `.rs` | 20609 bytes
1084. `core/src/cron.rs` | ext `.rs` | 14702 bytes
1085. `core/src/delivery/app_route.rs` | ext `.rs` | 41967 bytes
1086. `core/src/delivery/client_boundary.rs` | ext `.rs` | 5391 bytes
1087. `core/src/delivery/client_island.rs` | ext `.rs` | 30597 bytes
1088. `core/src/delivery/conformance.rs` | ext `.rs` | 6270 bytes
1089. `core/src/delivery/contract.rs` | ext `.rs` | 31586 bytes
1090. `core/src/delivery/encoding.rs` | ext `.rs` | 7843 bytes
1091. `core/src/delivery/html.rs` | ext `.rs` | 12112 bytes
1092. `core/src/delivery/import_resolution.rs` | ext `.rs` | 7527 bytes
1093. `core/src/delivery/jsx_lowering.rs` | ext `.rs` | 50976 bytes
1094. `core/src/delivery/micro_js.rs` | ext `.rs` | 3287 bytes
1095. `core/src/delivery/mod.rs` | ext `.rs` | 5037 bytes
1096. `core/src/delivery/plan.rs` | ext `.rs` | 11027 bytes
1097. `core/src/delivery/react_state.rs` | ext `.rs` | 3600 bytes
1098. `core/src/delivery/route_handler_ai.rs` | ext `.rs` | 20395 bytes
1099. `core/src/delivery/route_handler_automations.rs` | ext `.rs` | 17558 bytes
1100. `core/src/delivery/route_handler_body_boundary.rs` | ext `.rs` | 3297 bytes
1101. `core/src/delivery/route_handler_compat.rs` | ext `.rs` | 37214 bytes
1102. `core/src/delivery/route_handler_database_orm.rs` | ext `.rs` | 6807 bytes
1103. `core/src/delivery/route_handler_fumadocs.rs` | ext `.rs` | 22007 bytes
1104. `core/src/delivery/route_handler_instant_readiness.rs` | ext `.rs` | 6642 bytes
1105. `core/src/delivery/route_handler_payments.rs` | ext `.rs` | 23974 bytes
1106. `core/src/delivery/route_handler_supabase.rs` | ext `.rs` | 7495 bytes
1107. `core/src/delivery/route_unit.rs` | ext `.rs` | 15624 bytes
1108. `core/src/delivery/samples.rs` | ext `.rs` | 3880 bytes
1109. `core/src/delivery/server_contract.rs` | ext `.rs` | 157399 bytes
1110. `core/src/delivery/tests.rs` | ext `.rs` | 259935 bytes
1111. `core/src/delivery/tsx_ast.rs` | ext `.rs` | 33278 bytes
1112. `core/src/delivery/types.rs` | ext `.rs` | 8594 bytes
1113. `core/src/delivery/vertical.rs` | ext `.rs` | 22686 bytes
1114. `core/src/delivery/vertical_interaction.rs` | ext `.rs` | 10423 bytes
1115. `core/src/delivery/vertical_render.rs` | ext `.rs` | 11659 bytes
1116. `core/src/dev_server.rs` | ext `.rs` | 6035 bytes
1117. `core/src/di.rs` | ext `.rs` | 18969 bytes
1118. `core/src/dx_parser/dx_format.rs` | ext `.rs` | 44199 bytes
1119. `core/src/dx_parser/mod.rs` | ext `.rs` | 742 bytes
1120. `core/src/ecosystem/config.rs` | ext `.rs` | 4796 bytes
1121. `core/src/ecosystem/content.rs` | ext `.rs` | 4862 bytes
1122. `core/src/ecosystem/dx_check_receipt.rs` | ext `.rs` | 650080 bytes
1123. `core/src/ecosystem/dx_style_receipts.rs` | ext `.rs` | 30776 bytes
1124. `core/src/ecosystem/features.rs` | ext `.rs` | 4088 bytes
1125. `core/src/ecosystem/fonts.rs` | ext `.rs` | 10064 bytes
1126. `core/src/ecosystem/forge_drizzle.rs` | ext `.rs` | 49156 bytes
1127. `core/src/ecosystem/forge_fumadocs.rs` | ext `.rs` | 63622 bytes
1128. `core/src/ecosystem/forge_instantdb.rs` | ext `.rs` | 63062 bytes
1129. `core/src/ecosystem/forge_motion.rs` | ext `.rs` | 66504 bytes
1130. `core/src/ecosystem/forge_n8n_automations.rs` | ext `.rs` | 27472 bytes
1131. `core/src/ecosystem/forge_next_intl.rs` | ext `.rs` | 71097 bytes
1132. `core/src/ecosystem/forge_r2_head.rs` | ext `.rs` | 6773 bytes
1133. `core/src/ecosystem/forge_react_hook_form.rs` | ext `.rs` | 14164 bytes
1134. `core/src/ecosystem/forge_react_markdown.rs` | ext `.rs` | 25253 bytes
1135. `core/src/ecosystem/forge_reactive_store.rs` | ext `.rs` | 22411 bytes
1136. `core/src/ecosystem/forge_registry.rs` | ext `.rs` | 431022 bytes
1137. `core/src/ecosystem/forge_remote_health.rs` | ext `.rs` | 9316 bytes
1138. `core/src/ecosystem/forge_root_manifest.rs` | ext `.rs` | 18473 bytes
1139. `core/src/ecosystem/forge_scorecard.rs` | ext `.rs` | 47321 bytes
1140. `core/src/ecosystem/forge_security.rs` | ext `.rs` | 264641 bytes
1141. `core/src/ecosystem/forge_stripe_js.rs` | ext `.rs` | 73577 bytes
1142. `core/src/ecosystem/forge_supabase.rs` | ext `.rs` | 128155 bytes
1143. `core/src/ecosystem/forge_tanstack_query.rs` | ext `.rs` | 124667 bytes
1144. `core/src/ecosystem/forge_three_scene.rs` | ext `.rs` | 2971 bytes
1145. `core/src/ecosystem/forge_trpc.rs` | ext `.rs` | 72508 bytes
1146. `core/src/ecosystem/forge_trust_policy.rs` | ext `.rs` | 35973 bytes
1147. `core/src/ecosystem/forge_vercel_ai.rs` | ext `.rs` | 69951 bytes
1148. `core/src/ecosystem/forge_wasm_bindgen.rs` | ext `.rs` | 67067 bytes
1149. `core/src/ecosystem/forge_zod.rs` | ext `.rs` | 52774 bytes
1150. `core/src/ecosystem/forge_zustand.rs` | ext `.rs` | 44547 bytes
1151. `core/src/ecosystem/icons.rs` | ext `.rs` | 6724 bytes
1152. `core/src/ecosystem/media.rs` | ext `.rs` | 13432 bytes
1153. `core/src/ecosystem/mod.rs` | ext `.rs` | 1527 bytes
1154. `core/src/ecosystem/project_check.rs` | ext `.rs` | 320347 bytes
1155. `core/src/ecosystem/project_check/ai_sdk_dx_check.rs` | ext `.rs` | 18967 bytes
1156. `core/src/ecosystem/project_check/authentication_dx_check.rs` | ext `.rs` | 27415 bytes
1157. `core/src/ecosystem/project_check/automation_connectors_dx_check.rs` | ext `.rs` | 31059 bytes
1158. `core/src/ecosystem/project_check/backend_platform_client_dx_check.rs` | ext `.rs` | 28903 bytes
1159. `core/src/ecosystem/project_check/data_fetching_cache_dx_check.rs` | ext `.rs` | 19768 bytes
1160. `core/src/ecosystem/project_check/database_orm_dx_check.rs` | ext `.rs` | 24333 bytes
1161. `core/src/ecosystem/project_check/documentation_system_dx_check.rs` | ext `.rs` | 24832 bytes
1162. `core/src/ecosystem/project_check/file_hashes.rs` | ext `.rs` | 8734 bytes
1163. `core/src/ecosystem/project_check/forms_dx_check.rs` | ext `.rs` | 24126 bytes
1164. `core/src/ecosystem/project_check/internationalization_dx_check.rs` | ext `.rs` | 18747 bytes
1165. `core/src/ecosystem/project_check/markdown_mdx_content_dx_check.rs` | ext `.rs` | 40578 bytes
1166. `core/src/ecosystem/project_check/motion_animation_dx_check.rs` | ext `.rs` | 18395 bytes
1167. `core/src/ecosystem/project_check/payments_dx_check.rs` | ext `.rs` | 24139 bytes
1168. `core/src/ecosystem/project_check/reactive_store_dx_check.rs` | ext `.rs` | 20116 bytes
1169. `core/src/ecosystem/project_check/realtime_app_database_dx_check.rs` | ext `.rs` | 11929 bytes
1170. `core/src/ecosystem/project_check/state_management_dx_check.rs` | ext `.rs` | 15173 bytes
1171. `core/src/ecosystem/project_check/three_scene_system_dx_check.rs` | ext `.rs` | 26767 bytes
1172. `core/src/ecosystem/project_check/type_safe_api_dx_check.rs` | ext `.rs` | 15289 bytes
1173. `core/src/ecosystem/project_check/ui_components_dx_check.rs` | ext `.rs` | 12258 bytes
1174. `core/src/ecosystem/project_check/validation_schemas_dx_check.rs` | ext `.rs` | 20790 bytes
1175. `core/src/ecosystem/project_check/wasm_bindgen_dx_check.rs` | ext `.rs` | 33154 bytes
1176. `core/src/ecosystem/property_tests.rs` | ext `.rs` | 25233 bytes
1177. `core/src/ecosystem/unit_tests.rs` | ext `.rs` | 13940 bytes
1178. `core/src/errors.rs` | ext `.rs` | 20244 bytes
1179. `core/src/feature_tree_shaking.rs` | ext `.rs` | 44281 bytes
1180. `core/src/forms.rs` | ext `.rs` | 15984 bytes
1181. `core/src/guards.rs` | ext `.rs` | 17555 bytes
1182. `core/src/handlers.rs` | ext `.rs` | 15801 bytes
1183. `core/src/hmr.rs` | ext `.rs` | 30794 bytes
1184. `core/src/islands.rs` | ext `.rs` | 16477 bytes
1185. `core/src/jobs.rs` | ext `.rs` | 20496 bytes
1186. `core/src/keepalive.rs` | ext `.rs` | 17598 bytes
1187. `core/src/lib.rs` | ext `.rs` | 16063 bytes
1188. `core/src/linker.rs` | ext `.rs` | 6400 bytes
1189. `core/src/liveview.rs` | ext `.rs` | 16672 bytes
1190. `core/src/loader.rs` | ext `.rs` | 10412 bytes
1191. `core/src/optimistic.rs` | ext `.rs` | 13423 bytes
1192. `core/src/packer.rs` | ext `.rs` | 8798 bytes
1193. `core/src/parser.rs` | ext `.rs` | 37304 bytes
1194. `core/src/progressive.rs` | ext `.rs` | 16204 bytes
1195. `core/src/pwa.rs` | ext `.rs` | 9575 bytes
1196. `core/src/reactivity.rs` | ext `.rs` | 12820 bytes
1197. `core/src/resumability.rs` | ext `.rs` | 12266 bytes
1198. `core/src/router.rs` | ext `.rs` | 14952 bytes
1199. `core/src/rpc.rs` | ext `.rs` | 9244 bytes
1200. `core/src/schema_parser.rs` | ext `.rs` | 6926 bytes
1201. `core/src/server_component.rs` | ext `.rs` | 11864 bytes
1202. `core/src/splitter.rs` | ext `.rs` | 25227 bytes
1203. `core/src/streaming.rs` | ext `.rs` | 14801 bytes
1204. `core/src/suspense.rs` | ext `.rs` | 15674 bytes
1205. `core/src/swc_parser.rs` | ext `.rs` | 1383 bytes
1206. `core/src/teleport.rs` | ext `.rs` | 13714 bytes
1207. `core/src/template_registry.rs` | ext `.rs` | 25484 bytes
1208. `core/src/transitions.rs` | ext `.rs` | 14022 bytes
1209. `core/src/turso.rs` | ext `.rs` | 22948 bytes
1210. `core/src/types.rs` | ext `.rs` | 16319 bytes
1211. `core/src/wasm_compiler.rs` | ext `.rs` | 18022 bytes
1212. `core/src/www_config.rs` | ext `.rs` | 58175 bytes
1213. `core/tests/parser_property_tests.rs` | ext `.rs` | 10459 bytes
1214. `core/tests/splitter_property_tests.proptest-regressions` | ext `.proptest-regressions` | 469 bytes
1215. `core/tests/splitter_property_tests.rs` | ext `.rs` | 11636 bytes
1216. `CSS.md` | ext `.md` | 2727 bytes
1217. `CURRENT_STATE.md` | ext `.md` | 27501 bytes
1218. `db/Cargo.toml` | ext `.toml` | 1344 bytes
1219. `db/CHANGELOG.md` | ext `.md` | 372 bytes
1220. `db/LICENSE` | ext `<none>` | 559 bytes
1221. `db/LICENSE-APACHE` | ext `<none>` | 5871 bytes
1222. `db/LICENSE-MIT` | ext `<none>` | 1093 bytes
1223. `db/README.md` | ext `.md` | 459 bytes
1224. `db/src/lib.rs` | ext `.rs` | 11773 bytes
1225. `db/src/pool.rs` | ext `.rs` | 15029 bytes
1226. `db/tests/property_tests.rs` | ext `.rs` | 9362 bytes
1227. `db-teleport/Cargo.toml` | ext `.toml` | 1186 bytes
1228. `db-teleport/CHANGELOG.md` | ext `.md` | 377 bytes
1229. `db-teleport/LICENSE` | ext `<none>` | 559 bytes
1230. `db-teleport/LICENSE-APACHE` | ext `<none>` | 5871 bytes
1231. `db-teleport/LICENSE-MIT` | ext `<none>` | 1093 bytes
1232. `db-teleport/README.md` | ext `.md` | 2896 bytes
1233. `db-teleport/src/cache.rs` | ext `.rs` | 6606 bytes
1234. `db-teleport/src/error.rs` | ext `.rs` | 973 bytes
1235. `db-teleport/src/lib.rs` | ext `.rs` | 1523 bytes
1236. `db-teleport/src/postgres.rs` | ext `.rs` | 8277 bytes
1237. `db-teleport/src/query.rs` | ext `.rs` | 2624 bytes
1238. `db-teleport/tests/property_tests.rs` | ext `.rs` | 11596 bytes
1239. `demo/app_state.sr` | ext `.sr` | 816 bytes
1240. `demo/PRODUCTION_READINESS_ANALYSIS.md` | ext `.md` | readiness analysis
1241. `demo/build_binary_css.rs` | ext `.rs` | 4857 bytes
1242. `demo/build_dxob.rs` | ext `.rs` | 16575 bytes
1243. `demo/build_dxob_final.rs` | ext `.rs` | 21530 bytes
1244. `demo/build_dxob_v2.rs` | ext `.rs` | 14783 bytes
1245. `demo/build_serializer_css.rs` | ext `.rs` | 4061 bytes
1246. `demo/Cargo.toml` | ext `.toml` | 425 bytes
1247. `demo/compress_binary.rs` | ext `.rs` | 3271 bytes
1248. `demo/css.dict` | ext `.dict` | 426 bytes
1249. `demo/demo_full.html` | ext `.html` | 16647 bytes
1250. `demo/DX_BINARY_STYLE_ANALYSIS.md` | ext `.md` | 9982 bytes
1251. `demo/dx_compress.rs` | ext `.rs` | 6382 bytes
1252. `demo/dx_compress_final.rs` | ext `.rs` | 7879 bytes
1253. `demo/dx_compress_v2.rs` | ext `.rs` | 7977 bytes
1254. `demo/dx_compress_v3.rs` | ext `.rs` | 6571 bytes
1255. `demo/dx_www_client.wasm` | ext `.wasm` | 1421 bytes
1256. `demo/dx_www_client.wasm.gz` | ext `.gz` | 777 bytes
1257. `demo/dx_www_client_tiny_opt.wasm` | ext `.wasm` | 196 bytes
1258. `demo/dx_www_client_tiny_opt.wasm.gz` | ext `.gz` | 201 bytes
1259. `demo/htip_generator.rs` | ext `.rs` | 5121 bytes
1260. `demo/index.html` | ext `.html` | 1921 bytes
1261. `demo/index_enhanced.html` | ext `.html` | 9658 bytes
1262. `demo/server.rs` | ext `.rs` | 18865 bytes
1263. `demo/site/dx-style.json` | ext `.json` | 1811 bytes
1264. `demo/site/index.html` | ext `.html` | 22641 bytes
1265. `demo/site/LaunchPage.dx.jsx` | ext `.jsx` | 23383 bytes
1266. `demo/site/site.css` | ext `.css` | 6313 bytes
1267. `demo/site/site.js` | ext `.js` | 7917 bytes
1268. `demo/src/bin/launch_codegen.rs` | ext `.rs` | 4470 bytes
1269. `demo/styles.binary` | ext `.binary` | 1822 bytes
1270. `demo/styles.binary.gz` | ext `.gz` | 913 bytes
1271. `demo/styles.css` | ext `.css` | 3157 bytes
1272. `demo/styles.css.br` | ext `.br` | 845 bytes
1273. `demo/styles.css.dict.br` | ext `.br` | 823 bytes
1274. `demo/styles.css.gz` | ext `.gz` | 967 bytes
1275. `demo/styles.css.lz4` | ext `.lz4` | 3172 bytes
1276. `demo/styles.css.std.br` | ext `.br` | 845 bytes
1277. `demo/styles.css.zst` | ext `.zst` | 3161 bytes
1278. `demo/styles.dxbd` | ext `.dxbd` | 1979 bytes
1279. `demo/styles.dxc` | ext `.dxc` | 837 bytes
1280. `demo/styles.dxc2` | ext `.dxc2` | 1542 bytes
1281. `demo/styles.dxc3` | ext `.dxc3` | 552 bytes
1282. `demo/styles.dxob` | ext `.dxob` | 569 bytes
1283. `demo/styles.dxs` | ext `.dxs` | 1822 bytes
1284. `demo/styles.dxs.br` | ext `.br` | 770 bytes
1285. `demo/styles.dxs.gz` | ext `.gz` | 783 bytes
1286. `demo/styles.dxs.lz4` | ext `.lz4` | 1895 bytes
1287. `demo/styles.dxs.zst` | ext `.zst` | 1884 bytes
1288. `demo/styles.sr` | ext `.sr` | 912 bytes
1289. `demo/test_decompression.rs` | ext `.rs` | 9433 bytes
1290. `demo/test_todo.html` | ext `.html` | 569 bytes
1291. `demo/todo.html` | ext `.html` | 16465 bytes
1292. `demo/WHY_DX_WWW_IS_GAME_CHANGING.md` | ext `.md` | 13714 bytes
1293. `docs/api/README.md` | ext `.md` | 4187 bytes
1294. `docs/api/versioning.md` | ext `.md` | 3633 bytes
1295. `docs/architecture.md` | ext `.md` | 1764 bytes
1296. `docs/benchmarks.md` | ext `.md` | 3981 bytes
1297. `docs/build-graph-model.md` | ext `.md` | 14577 bytes
1298. `docs/deployment/docker.md` | ext `.md` | 4857 bytes
1299. `docs/deployment/environment.md` | ext `.md` | 3870 bytes
1300. `docs/deployment/monitoring.md` | ext `.md` | 5312 bytes
1301. `docs/deployment/nginx.md` | ext `.md` | 6024 bytes
1302. `docs/deployment/systemd.md` | ext `.md` | 4227 bytes
1303. `docs/deployment/troubleshooting.md` | ext `.md` | 5941 bytes
1304. `docs/DX_WWW_CURRENT_FEATURE_DOSSIER_2026-05-28.md` | ext `.md` | 595572 bytes
1305. `docs/DX_WWW_FRAMEWORK_STRUCTURE.md` | ext `.md` | 50675 bytes
1306. `docs/DX_WWW_MANAGER_HANDOFF.md` | ext `.md` | 28247 bytes
1307. `docs/dx-www-developer-contract.md` | ext `.md` | 7015 bytes
1308. `docs/dx-www-route-state-standard.md` | ext `.md` | 2700 bytes
1309. `docs/forge-adoption-launch-checklist.md` | ext `.md` | 10473 bytes
1310. `docs/forge-ci-smoke.md` | ext `.md` | 27479 bytes
1311. `docs/forge-launch-limitations.md` | ext `.md` | 3553 bytes
1312. `docs/forge-public-beta-quickstart.md` | ext `.md` | 22070 bytes
1313. `docs/forge-public-launch-checklist.md` | ext `.md` | 19314 bytes
1314. `docs/forge-public-launch-handoff.md` | ext `.md` | 12707 bytes
1315. `docs/forge-real-project-adoption.md` | ext `.md` | 8821 bytes
1316. `docs/forge-shadcn-migration.md` | ext `.md` | 8139 bytes
1317. `docs/getting-started.md` | ext `.md` | 3742 bytes
1318. `docs/migration/v0-to-v1.md` | ext `.md` | 4407 bytes
1319. `docs/NEXTJS_COMPATIBILITY_MAP.md` | ext `.md` | 10624 bytes
1320. `docs/next-rust-merge-checkpoint.md` | ext `.md` | 1668 bytes
1321. `docs/packages/3d-scene-system.md` | ext `.md` | 16604 bytes
1322. `docs/packages/3d-scene-system.source-guard-runbook.json` | ext `.json` | 7115 bytes
1323. `docs/packages/ai-sdk.source-guard-runbook.json` | ext `.json` | 7598 bytes
1324. `docs/packages/ai-vercel-ai.md` | ext `.md` | 15614 bytes
1325. `docs/packages/animation-motion.md` | ext `.md` | 16276 bytes
1326. `docs/packages/api-trpc.md` | ext `.md` | 19987 bytes
1327. `docs/packages/api-trpc.source-guard-runbook.json` | ext `.json` | 7222 bytes
1328. `docs/packages/authentication.md` | ext `.md` | 16303 bytes
1329. `docs/packages/authentication.source-guard-runbook.json` | ext `.json` | 12255 bytes
1330. `docs/packages/automation-connectors.source-guard-runbook.json` | ext `.json` | 11960 bytes
1331. `docs/packages/automations-n8n.md` | ext `.md` | 17258 bytes
1332. `docs/packages/backend-platform-client.source-guard-runbook.json` | ext `.json` | 7653 bytes
1333. `docs/packages/content-fumadocs-next.md` | ext `.md` | 20944 bytes
1334. `docs/packages/content-fumadocs-next.source-guard-runbook.json` | ext `.json` | 7975 bytes
1335. `docs/packages/content-react-markdown.md` | ext `.md` | 14884 bytes
1336. `docs/packages/content-react-markdown.source-guard-runbook.json` | ext `.json` | 10669 bytes
1337. `docs/packages/database-orm.mirror-drift.fixture.json` | ext `.json` | 6031 bytes
1338. `docs/packages/database-orm.source-guard-runbook.json` | ext `.json` | 5814 bytes
1339. `docs/packages/data-fetching-cache.source-guard-runbook.json` | ext `.json` | 5965 bytes
1340. `docs/packages/db-drizzle-sqlite.md` | ext `.md` | 21690 bytes
1341. `docs/packages/forge-safety-archive.source-guard-runbook.json` | ext `.json` | 4730 bytes
1342. `docs/packages/forms.source-guard-runbook.json` | ext `.json` | 5877 bytes
1343. `docs/packages/forms-react-hook-form.md` | ext `.md` | 14573 bytes
1344. `docs/packages/instantdb-react.md` | ext `.md` | 16529 bytes
1345. `docs/packages/instantdb-react.source-guard-runbook.json` | ext `.json` | 7304 bytes
1346. `docs/packages/motion-animation.source-guard-runbook.json` | ext `.json` | 4302 bytes
1347. `docs/packages/next-intl.md` | ext `.md` | 15757 bytes
1348. `docs/packages/next-intl.source-guard-runbook.json` | ext `.json` | 6891 bytes
1349. `docs/packages/payments.source-guard-runbook.json` | ext `.json` | 7945 bytes
1350. `docs/packages/payments-stripe-js.md` | ext `.md` | 18204 bytes
1351. `docs/packages/reactive-store.md` | ext `.md` | 11580 bytes
1352. `docs/packages/reactive-store.source-guard-runbook.json` | ext `.json` | 5246 bytes
1353. `docs/packages/state-zustand.md` | ext `.md` | 12628 bytes
1354. `docs/packages/state-zustand.source-guard-runbook.json` | ext `.json` | 8104 bytes
1355. `docs/packages/supabase-client.md` | ext `.md` | 19730 bytes
1356. `docs/packages/tanstack-query.md` | ext `.md` | 15747 bytes
1357. `docs/packages/ui-components.md` | ext `.md` | 13900 bytes
1358. `docs/packages/ui-components.source-guard-runbook.json` | ext `.json` | 3885 bytes
1359. `docs/packages/validation-schemas.source-guard-runbook.json` | ext `.json` | 5557 bytes
1360. `docs/packages/validation-zod.md` | ext `.md` | 16856 bytes
1361. `docs/packages/wasm-bindgen.md` | ext `.md` | 17419 bytes
1362. `docs/packages/wasm-bindgen.source-guard-runbook.json` | ext `.json` | 8369 bytes
1363. `docs/repo-hygiene.md` | ext `.md` | 7384 bytes
1364. `docs/root-workspace-readme.md` | ext `.md` | 3824 bytes
1365. `docs/root-workspace-status.md` | ext `.md` | 57977 bytes
1366. `docs/root-workspace-todo.md` | ext `.md` | 68070 bytes
1367. `docs/SECURITY.md` | ext `.md` | 6569 bytes
1368. `docs/superpowers/plans/2026-05-22-dx-build-source-engine-completion.md` | ext `.md` | 12861 bytes
1369. `docs/superpowers/plans/2026-05-26-dx-www-honest-web-perf-scoring.md` | ext `.md` | 6944 bytes
1370. `docs/superpowers/plans/2026-05-26-dx-www-zed-panel-v1-receipt-compat.md` | ext `.md` | 5750 bytes
1371. `docs/superpowers/plans/2026-05-28-dx-devtools-framework-integration.md` | ext `.md` | 1649 bytes
1372. `docs/superpowers/plans/2026-05-28-dx-devtools-mdn-css-picker-polish.md` | ext `.md` | 3792 bytes
1373. `docs/superpowers/plans/2026-05-28-dx-devtools-production-controls-pass.md` | ext `.md` | 4888 bytes
1374. `docs/superpowers/plans/2026-05-28-dx-devtools-real-controls-production-followup.md` | ext `.md` | 2367 bytes
1375. `docs/superpowers/plans/2026-05-28-dx-devtools-real-controls-ts-guards.md` | ext `.md` | 2472 bytes
1376. `docs/superpowers/plans/2026-05-28-dx-devtools-source-owned-assets-split.md` | ext `.md` | 4411 bytes
1377. `docs/superpowers/plans/2026-05-28-dx-devtools-ts-control-hardening.md` | ext `.md` | 4657 bytes
1378. `docs/superpowers/plans/2026-05-28-dx-devtools-ui-polish-no-build.md` | ext `.md` | 3388 bytes
1379. `docs/superpowers/plans/2026-05-28-www-12-flaw-hardening.md` | ext `.md` | 4698 bytes
1380. `docs/superpowers/plans/2026-05-28-www-repo-hygiene-sweep.md` | ext `.md` | 2721 bytes
1381. `docs/superpowers/plans/2026-05-28-www-scorecard-closure-pass-2.md` | ext `.md` | 5058 bytes
1382. `docs/wire-format-audit.md` | ext `.md` | 9482 bytes
1383. `dom/Cargo.toml` | ext `.toml` | 661 bytes
1384. `dom/CHANGELOG.md` | ext `.md` | 373 bytes
1385. `dom/LICENSE` | ext `<none>` | 559 bytes
1386. `dom/LICENSE-APACHE` | ext `<none>` | 5871 bytes
1387. `dom/LICENSE-MIT` | ext `<none>` | 1093 bytes
1388. `dom/README.md` | ext `.md` | 346 bytes
1389. `dom/src/lib.rs` | ext `.rs` | 12087 bytes
1390. `dx` | ext `<none>` | 500 bytes
1391. `DX.md` | ext `.md` | 1329979 bytes
1392. `dx-devtools/.dx/forge/docs/3d-launch-scene.md` | ext `.md` | 6746 bytes
1393. `dx-devtools/.dx/forge/docs/ai-vercel-ai.md` | ext `.md` | 15275 bytes
1394. `dx-devtools/.dx/forge/docs/animation-motion.md` | ext `.md` | 5733 bytes
1395. `dx-devtools/.dx/forge/docs/api-trpc.md` | ext `.md` | 9082 bytes
1396. `dx-devtools/.dx/forge/docs/auth-better-auth.md` | ext `.md` | 8616 bytes
1397. `dx-devtools/.dx/forge/docs/automations-n8n.md` | ext `.md` | 3110 bytes
1398. `dx-devtools/.dx/forge/docs/content-fumadocs-next.md` | ext `.md` | 9199 bytes
1399. `dx-devtools/.dx/forge/docs/content-react-markdown.md` | ext `.md` | 5888 bytes
1400. `dx-devtools/.dx/forge/docs/db-drizzle-sqlite.md` | ext `.md` | 7691 bytes
1401. `dx-devtools/.dx/forge/docs/dx-icon-search.md` | ext `.md` | 2307 bytes
1402. `dx-devtools/.dx/forge/docs/dx-www-starter-ui.md` | ext `.md` | 313 bytes
1403. `dx-devtools/.dx/forge/docs/forms-react-hook-form.md` | ext `.md` | 5216 bytes
1404. `dx-devtools/.dx/forge/docs/i18n-next-intl.md` | ext `.md` | 8426 bytes
1405. `dx-devtools/.dx/forge/docs/instantdb-react.md` | ext `.md` | 6103 bytes
1406. `dx-devtools/.dx/forge/docs/launch-companions/ai-chat-status.md` | ext `.md` | 645 bytes
1407. `dx-devtools/.dx/forge/docs/launch-companions/auth-session-status.md` | ext `.md` | 612 bytes
1408. `dx-devtools/.dx/forge/docs/launch-companions/docs-status.md` | ext `.md` | 778 bytes
1409. `dx-devtools/.dx/forge/docs/launch-companions/drizzle-query-proof.md` | ext `.md` | 679 bytes
1410. `dx-devtools/.dx/forge/docs/launch-companions/next-intl-dashboard-locale.md` | ext `.md` | 670 bytes
1411. `dx-devtools/.dx/forge/docs/launch-companions/payments-status.md` | ext `.md` | 639 bytes
1412. `dx-devtools/.dx/forge/docs/launch-companions/query-cache-status.md` | ext `.md` | 605 bytes
1413. `dx-devtools/.dx/forge/docs/launch-companions/realtime-data-status.md` | ext `.md` | 593 bytes
1414. `dx-devtools/.dx/forge/docs/launch-companions/supabase-profile-workflow.md` | ext `.md` | 780 bytes
1415. `dx-devtools/.dx/forge/docs/launch-companions/validation-status.md` | ext `.md` | 1018 bytes
1416. `dx-devtools/.dx/forge/docs/launch-companions/wasm-interop-status.md` | ext `.md` | 622 bytes
1417. `dx-devtools/.dx/forge/docs/payments-stripe-js.md` | ext `.md` | 6991 bytes
1418. `dx-devtools/.dx/forge/docs/shadcn-ui-badge.md` | ext `.md` | 2571 bytes
1419. `dx-devtools/.dx/forge/docs/shadcn-ui-field.md` | ext `.md` | 3123 bytes
1420. `dx-devtools/.dx/forge/docs/shadcn-ui-input.md` | ext `.md` | 2305 bytes
1421. `dx-devtools/.dx/forge/docs/shadcn-ui-item.md` | ext `.md` | 3106 bytes
1422. `dx-devtools/.dx/forge/docs/shadcn-ui-label.md` | ext `.md` | 2654 bytes
1423. `dx-devtools/.dx/forge/docs/shadcn-ui-separator.md` | ext `.md` | 2692 bytes
1424. `dx-devtools/.dx/forge/docs/shadcn-ui-textarea.md` | ext `.md` | 2323 bytes
1425. `dx-devtools/.dx/forge/docs/state-zustand.md` | ext `.md` | 4197 bytes
1426. `dx-devtools/.dx/forge/docs/supabase-client.md` | ext `.md` | 8248 bytes
1427. `dx-devtools/.dx/forge/docs/tanstack-query.md` | ext `.md` | 8683 bytes
1428. `dx-devtools/.dx/forge/docs/validation-zod.md` | ext `.md` | 4993 bytes
1429. `dx-devtools/.dx/forge/docs/wasm-bindgen.md` | ext `.md` | 3074 bytes
1430. `dx-devtools/.dx/forge/docs/www-launch-template--variant-next-familiar.md` | ext `.md` | 14422 bytes
1431. `dx-devtools/.dx/forge/docs/www-starter-ui.md` | ext `.md` | 1850 bytes
1432. `dx-devtools/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json` | ext `.json` | 28763 bytes
1433. `dx-devtools/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json` | ext `.json` | 18930 bytes
1434. `dx-devtools/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json` | ext `.json` | 16525 bytes
1435. `dx-devtools/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json` | ext `.json` | 4453 bytes
1436. `dx-devtools/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json` | ext `.json` | 17018 bytes
1437. `dx-devtools/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json` | ext `.json` | 12740 bytes
1438. `dx-devtools/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json` | ext `.json` | 14133 bytes
1439. `dx-devtools/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json` | ext `.json` | 9427 bytes
1440. `dx-devtools/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json` | ext `.json` | 16538 bytes
1441. `dx-devtools/.dx/forge/receipts/20260524T173442657762100Z-www-starter-ui.json` | ext `.json` | 3256 bytes
1442. `dx-devtools/.dx/forge/receipts/20260524T173442869042100Z-shadcn-ui-badge.json` | ext `.json` | 4878 bytes
1443. `dx-devtools/.dx/forge/receipts/20260524T173442878103600Z-shadcn-ui-label.json` | ext `.json` | 4214 bytes
1444. `dx-devtools/.dx/forge/receipts/20260524T173442886275700Z-shadcn-ui-separator.json` | ext `.json` | 4265 bytes
1445. `dx-devtools/.dx/forge/receipts/20260524T173442895274500Z-shadcn-ui-field.json` | ext `.json` | 5391 bytes
1446. `dx-devtools/.dx/forge/receipts/20260524T173442905688500Z-shadcn-ui-item.json` | ext `.json` | 5368 bytes
1447. `dx-devtools/.dx/forge/receipts/20260524T173442914370900Z-shadcn-ui-input.json` | ext `.json` | 3516 bytes
1448. `dx-devtools/.dx/forge/receipts/20260524T173442922746000Z-shadcn-ui-textarea.json` | ext `.json` | 3546 bytes
1449. `dx-devtools/.dx/forge/receipts/20260524T173442932739500Z-dx-icon-search.json` | ext `.json` | 4140 bytes
1450. `dx-devtools/.dx/forge/receipts/20260524T173442947154600Z-auth-better-auth.json` | ext `.json` | 19135 bytes
1451. `dx-devtools/.dx/forge/receipts/20260524T173442977165400Z-animation-motion.json` | ext `.json` | 14053 bytes
1452. `dx-devtools/.dx/forge/receipts/20260524T173443016539600Z-i18n-next-intl.json` | ext `.json` | 27991 bytes
1453. `dx-devtools/.dx/forge/receipts/20260524T173443053384300Z-tanstack-query.json` | ext `.json` | 27520 bytes
1454. `dx-devtools/.dx/forge/receipts/20260524T173443088415700Z-validation-zod.json` | ext `.json` | 15425 bytes
1455. `dx-devtools/.dx/forge/receipts/20260524T173443115657400Z-forms-react-hook-form.json` | ext `.json` | 8095 bytes
1456. `dx-devtools/.dx/forge/receipts/20260524T173443145109200Z-payments-stripe-js.json` | ext `.json` | 10666 bytes
1457. `dx-devtools/.dx/forge/receipts/20260524T173443169202400Z-automations-n8n.json` | ext `.json` | 6726 bytes
1458. `dx-devtools/.dx/forge/receipts/20260524T173443195471200Z-state-zustand.json` | ext `.json` | 11493 bytes
1459. `dx-devtools/.dx/forge/receipts/20260524T173443229037200Z-ai-vercel-ai.json` | ext `.json` | 31035 bytes
1460. `dx-devtools/.dx/forge/receipts/20260524T173443260293000Z-api-trpc.json` | ext `.json` | 24147 bytes
1461. `dx-devtools/.dx/forge/receipts/20260524T173443296359100Z-content-fumadocs-next.json` | ext `.json` | 25910 bytes
1462. `dx-devtools/.dx/forge/receipts/20260524T173443333567900Z-content-react-markdown.json` | ext `.json` | 12450 bytes
1463. `dx-devtools/.dx/forge/receipts/20260524T173443365930300Z-supabase-client.json` | ext `.json` | 23948 bytes
1464. `dx-devtools/.dx/forge/receipts/20260524T173443406560900Z-db-drizzle-sqlite.json` | ext `.json` | 15853 bytes
1465. `dx-devtools/.dx/forge/receipts/20260524T173443438233400Z-instantdb-react.json` | ext `.json` | 20905 bytes
1466. `dx-devtools/.dx/forge/receipts/20260524T173443466062600Z-wasm-bindgen.json` | ext `.json` | 6557 bytes
1467. `dx-devtools/.dx/forge/receipts/20260524T173443483907900Z-3d-launch-scene.json` | ext `.json` | 14065 bytes
1468. `dx-devtools/.dx/forge/receipts/20260524T173443501027000Z-www-launch-template--variant-next-familiar.json` | ext `.json` | 47659 bytes
1469. `dx-devtools/.dx/forge/receipts/auth-better-auth.json` | ext `.json` | 17789 bytes
1470. `dx-devtools/.dx/forge/source-manifest.json` | ext `.json` | 156676 bytes
1471. `dx-devtools/.dx/forge/template-core-sources.json` | ext `.json` | 9277 bytes
1472. `dx-devtools/.dx/forge/template-manifest.json` | ext `.json` | 246116 bytes
1473. `dx-devtools/.dx/forge/template-readiness/launch-companion-doc-receipts.json` | ext `.json` | 12097 bytes
1474. `dx-devtools/.dx/forge/template-readiness/launch-readiness-bundle.json` | ext `.json` | 81883 bytes
1475. `dx-devtools/.dx/forge/template-readiness/launch-route.json` | ext `.json` | 38078 bytes
1476. `dx-devtools/.dx/forge/template-readiness/launch-runtime-approval-request.json` | ext `.json` | 2100 bytes
1477. `dx-devtools/.dx/forge/template-readiness/launch-runtime-checklist.json` | ext `.json` | 3293 bytes
1478. `dx-devtools/.dx/forge/template-readiness/launch-runtime-evidence.json` | ext `.json` | 4489 bytes
1479. `dx-devtools/.dx/forge/template-readiness/launch-verification-lane.json` | ext `.json` | 7151 bytes
1480. `dx-devtools/.dx/forge/template-readiness/zed-template-handoff.json` | ext `.json` | 16064 bytes
1481. `dx-devtools/.dx/receipts/style/check.json` | ext `.json` | 538446 bytes
1482. `dx-devtools/.dx/serializer/dx.machine` | ext `.machine` | 6024 bytes
1483. `dx-devtools/.gitignore` | ext `.gitignore` | 63 bytes
1484. `dx-devtools/app/api/ai/agent/route.ts` | ext `.ts` | 484 bytes
1485. `dx-devtools/app/api/ai/chat/route.ts` | ext `.ts` | 483 bytes
1486. `dx-devtools/app/api/ai/image/route.ts` | ext `.ts` | 484 bytes
1487. `dx-devtools/app/api/ai/object/route.ts` | ext `.ts` | 485 bytes
1488. `dx-devtools/app/api/ai/speech/route.ts` | ext `.ts` | 485 bytes
1489. `dx-devtools/app/api/ai/text-stream/route.ts` | ext `.ts` | 472 bytes
1490. `dx-devtools/app/api/ai/transcribe/route.ts` | ext `.ts` | 489 bytes
1491. `dx-devtools/app/api/ai/ui-stream/route.ts` | ext `.ts` | 486 bytes
1492. `dx-devtools/app/api/ai/upload-file/route.ts` | ext `.ts` | 490 bytes
1493. `dx-devtools/app/api/ai/video/route.ts` | ext `.ts` | 484 bytes
1494. `dx-devtools/app/api/checkout/route.ts` | ext `.ts` | 390 bytes
1495. `dx-devtools/app/api/health/route.ts` | ext `.ts` | 206 bytes
1496. `dx-devtools/app/api/instant/route.ts` | ext `.ts` | 375 bytes
1497. `dx-devtools/app/api/openapi/proxy/route.ts` | ext `.ts` | 958 bytes
1498. `dx-devtools/app/api/search/route.ts` | ext `.ts` | 349 bytes
1499. `dx-devtools/app/api/search-static/route.ts` | ext `.ts` | 397 bytes
1500. `dx-devtools/app/api/stripe/webhook/route.ts` | ext `.ts` | 380 bytes
1501. `dx-devtools/app/api/trpc/[trpc]/route.ts` | ext `.ts` | 536 bytes
1502. `dx-devtools/app/auth/page.tsx` | ext `.tsx` | 606 bytes
1503. `dx-devtools/app/billing/page.tsx` | ext `.tsx` | 571 bytes
1504. `dx-devtools/app/dashboard/page.tsx` | ext `.tsx` | 236 bytes
1505. `dx-devtools/app/docs/[[...slug]]/page.tsx` | ext `.tsx` | 1767 bytes
1506. `dx-devtools/app/docs/layout.tsx` | ext `.tsx` | 563 bytes
1507. `dx-devtools/app/error.tsx` | ext `.tsx` | 453 bytes
1508. `dx-devtools/app/instant-launch/page.tsx` | ext `.tsx` | 568 bytes
1509. `dx-devtools/app/launch/page.tsx` | ext `.tsx` | 776 bytes
1510. `dx-devtools/app/layout.tsx` | ext `.tsx` | 835 bytes
1511. `dx-devtools/app/llms.mdx/docs/[[...slug]]/route.ts` | ext `.ts` | 734 bytes
1512. `dx-devtools/app/llms.txt/route.ts` | ext `.ts` | 266 bytes
1513. `dx-devtools/app/llms-full.txt/route.ts` | ext `.ts` | 379 bytes
1514. `dx-devtools/app/loading.tsx` | ext `.tsx` | 256 bytes
1515. `dx-devtools/app/not-found.tsx` | ext `.tsx` | 298 bytes
1516. `dx-devtools/app/page.tsx` | ext `.tsx` | 328 bytes
1517. `dx-devtools/app/settings/page.tsx` | ext `.tsx` | 221 bytes
1518. `dx-devtools/auth/better-auth/.env.example` | ext `.example` | 362 bytes
1519. `dx-devtools/auth/better-auth/account-deletion.ts` | ext `.ts` | 1682 bytes
1520. `dx-devtools/auth/better-auth/accounts.ts` | ext `.ts` | 2730 bytes
1521. `dx-devtools/auth/better-auth/account-security.ts` | ext `.ts` | 2924 bytes
1522. `dx-devtools/auth/better-auth/client.ts` | ext `.ts` | 1263 bytes
1523. `dx-devtools/auth/better-auth/dashboard.ts` | ext `.ts` | 6219 bytes
1524. `dx-devtools/auth/better-auth/email-password.ts` | ext `.ts` | 1919 bytes
1525. `dx-devtools/auth/better-auth/metadata.ts` | ext `.ts` | 5699 bytes
1526. `dx-devtools/auth/better-auth/options.ts` | ext `.ts` | 2543 bytes
1527. `dx-devtools/auth/better-auth/profile.ts` | ext `.ts` | 2635 bytes
1528. `dx-devtools/auth/better-auth/providers/google/.env.example` | ext `.example` | 340 bytes
1529. `dx-devtools/auth/better-auth/providers/google/callback.ts` | ext `.ts` | 2228 bytes
1530. `dx-devtools/auth/better-auth/providers/google/config.ts` | ext `.ts` | 3023 bytes
1531. `dx-devtools/auth/better-auth/providers/google/README.md` | ext `.md` | 429 bytes
1532. `dx-devtools/auth/better-auth/providers/google/route.ts` | ext `.ts` | 654 bytes
1533. `dx-devtools/auth/better-auth/README.md` | ext `.md` | 5552 bytes
1534. `dx-devtools/auth/better-auth/route.ts` | ext `.ts` | 280 bytes
1535. `dx-devtools/auth/better-auth/server.ts` | ext `.ts` | 1216 bytes
1536. `dx-devtools/auth/better-auth/session.ts` | ext `.ts` | 630 bytes
1537. `dx-devtools/auth/better-auth/session-management.ts` | ext `.ts` | 1236 bytes
1538. `dx-devtools/auth/better-auth/social.ts` | ext `.ts` | 2034 bytes
1539. `dx-devtools/components/ai/ai-launch-assistant.tsx` | ext `.tsx` | 3547 bytes
1540. `dx-devtools/components/api-page.client.tsx` | ext `.tsx` | 366 bytes
1541. `dx-devtools/components/api-page.tsx` | ext `.tsx` | 404 bytes
1542. `dx-devtools/components/content/markdown.tsx` | ext `.tsx` | 2642 bytes
1543. `dx-devtools/components/content/markdown-components.tsx` | ext `.tsx` | 926 bytes
1544. `dx-devtools/components/content/markdown-metadata.ts` | ext `.ts` | 4287 bytes
1545. `dx-devtools/components/content/mdx-provider.tsx` | ext `.tsx` | 1941 bytes
1546. `dx-devtools/components/content/README.md` | ext `.md` | 455 bytes
1547. `dx-devtools/components/dashboard/fumadocs-docs-workflow.tsx` | ext `.tsx` | 5816 bytes
1548. `dx-devtools/components/dashboard/instantdb-dashboard-workflow.tsx` | ext `.tsx` | 5186 bytes
1549. `dx-devtools/components/dashboard/LaunchDashboard.tsx` | ext `.tsx` | 861 bytes
1550. `dx-devtools/components/dashboard/trpc-dashboard-workflow.tsx` | ext `.tsx` | 5087 bytes
1551. `dx-devtools/components/devtools/devtools-model.ts` | ext `.ts` | 106851 bytes
1552. `dx-devtools/components/devtools/DxBoxModelPanel.tsx` | ext `.tsx` | 10842 bytes
1553. `dx-devtools/components/devtools/DxCallStack.tsx` | ext `.tsx` | 6236 bytes
1554. `dx-devtools/components/devtools/DxCodeFrame.tsx` | ext `.tsx` | 4748 bytes
1555. `dx-devtools/components/devtools/DxCodeFrameLines.ts` | ext `.ts` | 5072 bytes
1556. `dx-devtools/components/devtools/DxDevtoolsShell.tsx` | ext `.tsx` | 109455 bytes
1557. `dx-devtools/components/devtools/DxErrorCausePanel.tsx` | ext `.tsx` | 6849 bytes
1558. `dx-devtools/components/devtools/DxErrorFeedbackPreview.tsx` | ext `.tsx` | 1450 bytes
1559. `dx-devtools/components/devtools/DxErrorOverlay.tsx` | ext `.tsx` | 25610 bytes
1560. `dx-devtools/components/devtools/DxErrorOverlayBodies.tsx` | ext `.tsx` | 17572 bytes
1561. `dx-devtools/components/devtools/DxErrorOverlayChrome.tsx` | ext `.tsx` | 10156 bytes
1562. `dx-devtools/components/devtools/DxErrorOverlayLayout.tsx` | ext `.tsx` | 3171 bytes
1563. `dx-devtools/components/devtools/DxErrorOverlayShell.tsx` | ext `.tsx` | 8299 bytes
1564. `dx-devtools/components/devtools/DxErrorOverlayToolbar.tsx` | ext `.tsx` | 11273 bytes
1565. `dx-devtools/components/devtools/DxHydrationDiff.tsx` | ext `.tsx` | 6587 bytes
1566. `dx-devtools/components/devtools/DxInspectorPanel.tsx` | ext `.tsx` | 22114 bytes
1567. `dx-devtools/components/devtools/DxSourcePreviewHeader.tsx` | ext `.tsx` | 1726 bytes
1568. `dx-devtools/components/devtools/DxTerminalLog.tsx` | ext `.tsx` | 6872 bytes
1569. `dx-devtools/components/devtools/DxTerminalText.tsx` | ext `.tsx` | 9330 bytes
1570. `dx-devtools/components/devtools/useDxDelayedRender.ts` | ext `.ts` | 712 bytes
1571. `dx-devtools/components/forms/SettingsForm.tsx` | ext `.tsx` | 929 bytes
1572. `dx-devtools/components/icons/icon.tsx` | ext `.tsx` | 724 bytes
1573. `dx-devtools/components/icons/README.md` | ext `.md` | 196 bytes
1574. `dx-devtools/components/icons/search.tsx` | ext `.tsx` | 496 bytes
1575. `dx-devtools/components/instant/instant-auth-boundary.tsx` | ext `.tsx` | 1368 bytes
1576. `dx-devtools/components/instant/instant-cursors.tsx` | ext `.tsx` | 733 bytes
1577. `dx-devtools/components/instant/instant-todos.tsx` | ext `.tsx` | 7185 bytes
1578. `dx-devtools/components/launch/ai-chat-status.tsx` | ext `.tsx` | 14009 bytes
1579. `dx-devtools/components/launch/auth-session-status.tsx` | ext `.tsx` | 33520 bytes
1580. `dx-devtools/components/launch/automation-mission-summary.tsx` | ext `.tsx` | 4295 bytes
1581. `dx-devtools/components/launch/automations/automations-metadata.ts` | ext `.ts` | 17509 bytes
1582. `dx-devtools/components/launch/automations-status.tsx` | ext `.tsx` | 22143 bytes
1583. `dx-devtools/components/launch/data-status.tsx` | ext `.tsx` | 10335 bytes
1584. `dx-devtools/components/launch/docs-status.tsx` | ext `.tsx` | 11371 bytes
1585. `dx-devtools/components/launch/drizzle-query-proof.tsx` | ext `.tsx` | 10286 bytes
1586. `dx-devtools/components/launch/dx-check-style-evidence-read-model.ts` | ext `.ts` | 7619 bytes
1587. `dx-devtools/components/launch/dx-studio-edit-contract.ts` | ext `.ts` | 37946 bytes
1588. `dx-devtools/components/launch/forge-golden-path-panel.tsx` | ext `.tsx` | 1362 bytes
1589. `dx-devtools/components/launch/forge-package-status.ts` | ext `.ts` | 4775 bytes
1590. `dx-devtools/components/launch/forge-package-status-read-model.ts` | ext `.ts` | 329621 bytes
1591. `dx-devtools/components/launch/forge-remote-head-health-contract.ts` | ext `.ts` | 3109 bytes
1592. `dx-devtools/components/launch/forge-remote-head-health-panel.tsx` | ext `.tsx` | 3328 bytes
1593. `dx-devtools/components/launch/forge-safety-archive-contract.ts` | ext `.ts` | 3985 bytes
1594. `dx-devtools/components/launch/forge-safety-archive-panel.tsx` | ext `.tsx` | 3914 bytes
1595. `dx-devtools/components/launch/forge-safety-archive-runbook.ts` | ext `.ts` | 5880 bytes
1596. `dx-devtools/components/launch/framework-completeness.ts` | ext `.ts` | 17454 bytes
1597. `dx-devtools/components/launch/icon-status.tsx` | ext `.tsx` | 229 bytes
1598. `dx-devtools/components/launch/instantdb-status.tsx` | ext `.tsx` | 19618 bytes
1599. `dx-devtools/components/launch/launch-dashboard-nav.tsx` | ext `.tsx` | 2943 bytes
1600. `dx-devtools/components/launch/launch-lead-form.tsx` | ext `.tsx` | 2950 bytes
1601. `dx-devtools/components/launch/launch-route-contract.ts` | ext `.ts` | 56819 bytes
1602. `dx-devtools/components/launch/launch-shell.tsx` | ext `.tsx` | 131354 bytes
1603. `dx-devtools/components/launch/launch-shell-evidence-loader.ts` | ext `.ts` | 672 bytes
1604. `dx-devtools/components/launch/launch-shell-style-evidence-drift.ts` | ext `.ts` | 645 bytes
1605. `dx-devtools/components/launch/motion-interaction-proof.tsx` | ext `.tsx` | 14708 bytes
1606. `dx-devtools/components/launch/next-intl-dashboard-locale.tsx` | ext `.tsx` | 10399 bytes
1607. `dx-devtools/components/launch/next-intl-dashboard-locale-contract.ts` | ext `.ts` | 6825 bytes
1608. `dx-devtools/components/launch/next-intl-status.tsx` | ext `.tsx` | 2876 bytes
1609. `dx-devtools/components/launch/package-catalog.ts` | ext `.ts` | 109504 bytes
1610. `dx-devtools/components/launch/payments-status.tsx` | ext `.tsx` | 20585 bytes
1611. `dx-devtools/components/launch/preview-style-evidence-read-model.ts` | ext `.ts` | 4969 bytes
1612. `dx-devtools/components/launch/preview-style-package-ownership-read-model.ts` | ext `.ts` | 1963 bytes
1613. `dx-devtools/components/launch/preview-style-package-panel-read-model.ts` | ext `.ts` | 12302 bytes
1614. `dx-devtools/components/launch/query-cache-status.tsx` | ext `.tsx` | 33604 bytes
1615. `dx-devtools/components/launch/query-dashboard-read-model.ts` | ext `.ts` | 2192 bytes
1616. `dx-devtools/components/launch/react-markdown-preview.tsx` | ext `.tsx` | 553 bytes
1617. `dx-devtools/components/launch/shadcn-dashboard-controls.tsx` | ext `.tsx` | 14751 bytes
1618. `dx-devtools/components/launch/shadcn-dashboard-controls-contract.tsx` | ext `.tsx` | 5079 bytes
1619. `dx-devtools/components/launch/state-zustand-counter.tsx` | ext `.tsx` | 5723 bytes
1620. `dx-devtools/components/launch/state-zustand-dashboard.tsx` | ext `.tsx` | 7327 bytes
1621. `dx-devtools/components/launch/supabase-profile-workflow.tsx` | ext `.tsx` | 7147 bytes
1622. `dx-devtools/components/launch/template-surface-registry.ts` | ext `.ts` | 17762 bytes
1623. `dx-devtools/components/launch/trpc-launch-contract.ts` | ext `.ts` | 1627 bytes
1624. `dx-devtools/components/launch/trpc-launch-health.tsx` | ext `.tsx` | 5579 bytes
1625. `dx-devtools/components/launch/wasm-interop-status.tsx` | ext `.tsx` | 6918 bytes
1626. `dx-devtools/components/launch/zod-dashboard-settings.tsx` | ext `.tsx` | 8017 bytes
1627. `dx-devtools/components/launch/zod-validation-status.tsx` | ext `.tsx` | 10799 bytes
1628. `dx-devtools/components/local/LaunchConsole.tsx` | ext `.tsx` | 145 bytes
1629. `dx-devtools/components/local/WelcomeCard.tsx` | ext `.tsx` | 2317 bytes
1630. `dx-devtools/components/markdown.tsx` | ext `.tsx` | 1519 bytes
1631. `dx-devtools/components/markdown-client.tsx` | ext `.tsx` | 1056 bytes
1632. `dx-devtools/components/marketing/Hero.tsx` | ext `.tsx` | 733 bytes
1633. `dx-devtools/components/mdx.tsx` | ext `.tsx` | 523 bytes
1634. `dx-devtools/components/scene/launch-scene.tsx` | ext `.tsx` | 23224 bytes
1635. `dx-devtools/components/ui/badge.tsx` | ext `.tsx` | 1205 bytes
1636. `dx-devtools/components/ui/button.tsx` | ext `.tsx` | 378 bytes
1637. `dx-devtools/components/ui/card.tsx` | ext `.tsx` | 214 bytes
1638. `dx-devtools/components/ui/field.tsx` | ext `.tsx` | 4277 bytes
1639. `dx-devtools/components/ui/input.tsx` | ext `.tsx` | 334 bytes
1640. `dx-devtools/components/ui/item.tsx` | ext `.tsx` | 4091 bytes
1641. `dx-devtools/components/ui/label.tsx` | ext `.tsx` | 323 bytes
1642. `dx-devtools/components/ui/README.md` | ext `.md` | 425 bytes
1643. `dx-devtools/components/ui/scroll-area.tsx` | ext `.tsx` | 295 bytes
1644. `dx-devtools/components/ui/separator.tsx` | ext `.tsx` | 675 bytes
1645. `dx-devtools/components/ui/slot.tsx` | ext `.tsx` | 642 bytes
1646. `dx-devtools/components/ui/textarea.tsx` | ext `.tsx` | 303 bytes
1647. `dx-devtools/content/docs/index.mdx` | ext `.mdx` | 1376 bytes
1648. `dx-devtools/content/docs/meta.json` | ext `.json` | 54 bytes
1649. `dx-devtools/db/drizzle/analytics.ts` | ext `.ts` | 1424 bytes
1650. `dx-devtools/db/drizzle/client.ts` | ext `.ts` | 549 bytes
1651. `dx-devtools/db/drizzle/cte-queries.ts` | ext `.ts` | 1780 bytes
1652. `dx-devtools/db/drizzle/dashboard-workflow.ts` | ext `.ts` | 4500 bytes
1653. `dx-devtools/db/drizzle/joins.ts` | ext `.ts` | 1426 bytes
1654. `dx-devtools/db/drizzle/metadata.ts` | ext `.ts` | 11994 bytes
1655. `dx-devtools/db/drizzle/migrations.ts` | ext `.ts` | 1133 bytes
1656. `dx-devtools/db/drizzle/mutations.ts` | ext `.ts` | 1034 bytes
1657. `dx-devtools/db/drizzle/prepared-queries.ts` | ext `.ts` | 1328 bytes
1658. `dx-devtools/db/drizzle/queries.ts` | ext `.ts` | 1839 bytes
1659. `dx-devtools/db/drizzle/README.md` | ext `.md` | 8013 bytes
1660. `dx-devtools/db/drizzle/relational-queries.ts` | ext `.ts` | 689 bytes
1661. `dx-devtools/db/drizzle/replicas.ts` | ext `.ts` | 1885 bytes
1662. `dx-devtools/db/drizzle/schema.ts` | ext `.ts` | 2120 bytes
1663. `dx-devtools/db/drizzle/set-operations.ts` | ext `.ts` | 3744 bytes
1664. `dx-devtools/db/drizzle/transactions.ts` | ext `.ts` | 706 bytes
1665. `dx-devtools/db/drizzle/upserts.ts` | ext `.ts` | 1403 bytes
1666. `dx-devtools/db/drizzle/views.ts` | ext `.ts` | 1399 bytes
1667. `dx-devtools/dx.disabled-by-codex-20260525-023429.txt` | ext `.txt` | 2704 bytes
1668. `dx-devtools/examples/launch-template/trpc-error-status.tsx` | ext `.tsx` | 736 bytes
1669. `dx-devtools/examples/launch-template/trpc-infinite-feed.tsx` | ext `.tsx` | 1776 bytes
1670. `dx-devtools/examples/launch-template/trpc-launch-contract.ts` | ext `.ts` | 1627 bytes
1671. `dx-devtools/examples/launch-template/trpc-launch-health.tsx` | ext `.tsx` | 5579 bytes
1672. `dx-devtools/examples/launch-template/trpc-request-policy.ts` | ext `.ts` | 910 bytes
1673. `dx-devtools/examples/launch-template/trpc-response-meta.ts` | ext `.ts` | 678 bytes
1674. `dx-devtools/examples/launch-template/trpc-server-readiness.ts` | ext `.ts` | 393 bytes
1675. `dx-devtools/examples/launch-template/trpc-streaming-client-status.tsx` | ext `.tsx` | 517 bytes
1676. `dx-devtools/examples/launch-template/trpc-subscription-status.tsx` | ext `.tsx` | 1492 bytes
1677. `dx-devtools/examples/launch-template/trpc-transformer-status.ts` | ext `.ts` | 498 bytes
1678. `dx-devtools/i18n/app-config.ts` | ext `.ts` | 758 bytes
1679. `dx-devtools/i18n/catalog-validation.ts` | ext `.ts` | 1243 bytes
1680. `dx-devtools/i18n/context-status.tsx` | ext `.tsx` | 1126 bytes
1681. `dx-devtools/i18n/dashboard-copy.ts` | ext `.ts` | 6765 bytes
1682. `dx-devtools/i18n/dashboard-locale-workflow.tsx` | ext `.tsx` | 7864 bytes
1683. `dx-devtools/i18n/domain-routing.ts` | ext `.ts` | 918 bytes
1684. `dx-devtools/i18n/error-policy.ts` | ext `.ts` | 1077 bytes
1685. `dx-devtools/i18n/extracted-copy.tsx` | ext `.tsx` | 1133 bytes
1686. `dx-devtools/i18n/extraction.ts` | ext `.ts` | 1537 bytes
1687. `dx-devtools/i18n/extraction-runner.ts` | ext `.ts` | 1311 bytes
1688. `dx-devtools/i18n/format-options.ts` | ext `.ts` | 814 bytes
1689. `dx-devtools/i18n/formats.ts` | ext `.ts` | 465 bytes
1690. `dx-devtools/i18n/formatter-cache.ts` | ext `.ts` | 1191 bytes
1691. `dx-devtools/i18n/formatting.tsx` | ext `.tsx` | 1712 bytes
1692. `dx-devtools/i18n/locale-guard.ts` | ext `.ts` | 413 bytes
1693. `dx-devtools/i18n/locale-links.tsx` | ext `.tsx` | 1473 bytes
1694. `dx-devtools/i18n/message-arguments.ts` | ext `.ts` | 1099 bytes
1695. `dx-devtools/i18n/messages/bn.json` | ext `.json` | 1332 bytes
1696. `dx-devtools/i18n/messages/en.json` | ext `.json` | 1332 bytes
1697. `dx-devtools/i18n/metadata.ts` | ext `.ts` | 7234 bytes
1698. `dx-devtools/i18n/middleware.ts` | ext `.ts` | 507 bytes
1699. `dx-devtools/i18n/navigation.ts` | ext `.ts` | 226 bytes
1700. `dx-devtools/i18n/navigation-actions.ts` | ext `.ts` | 766 bytes
1701. `dx-devtools/i18n/navigation-client.tsx` | ext `.tsx` | 1806 bytes
1702. `dx-devtools/i18n/next-config.ts` | ext `.ts` | 637 bytes
1703. `dx-devtools/i18n/provider.tsx` | ext `.tsx` | 956 bytes
1704. `dx-devtools/i18n/README.md` | ext `.md` | 8387 bytes
1705. `dx-devtools/i18n/request.ts` | ext `.ts` | 1067 bytes
1706. `dx-devtools/i18n/request-config.ts` | ext `.ts` | 1023 bytes
1707. `dx-devtools/i18n/request-runtime.ts` | ext `.ts` | 767 bytes
1708. `dx-devtools/i18n/rich-copy.tsx` | ext `.tsx` | 701 bytes
1709. `dx-devtools/i18n/route-boundary.ts` | ext `.ts` | 859 bytes
1710. `dx-devtools/i18n/route-types.ts` | ext `.ts` | 926 bytes
1711. `dx-devtools/i18n/routing.ts` | ext `.ts` | 668 bytes
1712. `dx-devtools/i18n/routing-policy.ts` | ext `.ts` | 1031 bytes
1713. `dx-devtools/i18n/runtime-core.ts` | ext `.ts` | 915 bytes
1714. `dx-devtools/i18n/server-context.ts` | ext `.ts` | 576 bytes
1715. `dx-devtools/i18n/server-extracted.ts` | ext `.ts` | 834 bytes
1716. `dx-devtools/i18n/server-provider.tsx` | ext `.tsx` | 1117 bytes
1717. `dx-devtools/i18n/type-contracts.ts` | ext `.ts` | 1622 bytes
1718. `dx-devtools/lib/ai/agent.ts` | ext `.ts` | 1325 bytes
1719. `dx-devtools/lib/ai/chat-route.ts` | ext `.ts` | 1813 bytes
1720. `dx-devtools/lib/ai/client-chat.tsx` | ext `.tsx` | 2346 bytes
1721. `dx-devtools/lib/ai/dashboard-readiness.ts` | ext `.ts` | 1257 bytes
1722. `dx-devtools/lib/ai/embeddings.ts` | ext `.ts` | 1328 bytes
1723. `dx-devtools/lib/ai/file-upload.ts` | ext `.ts` | 1102 bytes
1724. `dx-devtools/lib/ai/image-generation.ts` | ext `.ts` | 1503 bytes
1725. `dx-devtools/lib/ai/message-pruning.ts` | ext `.ts` | 781 bytes
1726. `dx-devtools/lib/ai/metadata.ts` | ext `.ts` | 7771 bytes
1727. `dx-devtools/lib/ai/model.ts` | ext `.ts` | 284 bytes
1728. `dx-devtools/lib/ai/model-policy.ts` | ext `.ts` | 842 bytes
1729. `dx-devtools/lib/ai/object-generation.ts` | ext `.ts` | 1433 bytes
1730. `dx-devtools/lib/ai/provider-freedom.ts` | ext `.ts` | 1073 bytes
1731. `dx-devtools/lib/ai/README.md` | ext `.md` | 5198 bytes
1732. `dx-devtools/lib/ai/reranking.ts` | ext `.ts` | 1040 bytes
1733. `dx-devtools/lib/ai/speech-generation.ts` | ext `.ts` | 1692 bytes
1734. `dx-devtools/lib/ai/structured-output.ts` | ext `.ts` | 1395 bytes
1735. `dx-devtools/lib/ai/telemetry.ts` | ext `.ts` | 3760 bytes
1736. `dx-devtools/lib/ai/text-stream.ts` | ext `.ts` | 1178 bytes
1737. `dx-devtools/lib/ai/tool-approval.ts` | ext `.ts` | 1203 bytes
1738. `dx-devtools/lib/ai/tools.ts` | ext `.ts` | 461 bytes
1739. `dx-devtools/lib/ai/transcription.ts` | ext `.ts` | 1433 bytes
1740. `dx-devtools/lib/ai/ui-message-stream.ts` | ext `.ts` | 1878 bytes
1741. `dx-devtools/lib/ai/video-generation.ts` | ext `.ts` | 1727 bytes
1742. `dx-devtools/lib/automations/n8n/bridge.ts` | ext `.ts` | 787 bytes
1743. `dx-devtools/lib/automations/n8n/catalog.ts` | ext `.ts` | 5243 bytes
1744. `dx-devtools/lib/automations/n8n/metadata.ts` | ext `.ts` | 12386 bytes
1745. `dx-devtools/lib/automations/n8n/readiness.ts` | ext `.ts` | 2047 bytes
1746. `dx-devtools/lib/automations/n8n/README.md` | ext `.md` | 3376 bytes
1747. `dx-devtools/lib/automations/n8n/receipt.ts` | ext `.ts` | 2623 bytes
1748. `dx-devtools/lib/forge/state/zustand/devtools.ts` | ext `.ts` | 4550 bytes
1749. `dx-devtools/lib/forge/state/zustand/immer.ts` | ext `.ts` | 1689 bytes
1750. `dx-devtools/lib/forge/state/zustand/index.ts` | ext `.ts` | 248 bytes
1751. `dx-devtools/lib/forge/state/zustand/metadata.ts` | ext `.ts` | 6234 bytes
1752. `dx-devtools/lib/forge/state/zustand/middleware.ts` | ext `.ts` | 3467 bytes
1753. `dx-devtools/lib/forge/state/zustand/persist.ts` | ext `.ts` | 6754 bytes
1754. `dx-devtools/lib/forge/state/zustand/react.ts` | ext `.ts` | 2177 bytes
1755. `dx-devtools/lib/forge/state/zustand/README.md` | ext `.md` | 8169 bytes
1756. `dx-devtools/lib/forge/state/zustand/redux.ts` | ext `.ts` | 1079 bytes
1757. `dx-devtools/lib/forge/state/zustand/shallow.ts` | ext `.ts` | 1356 bytes
1758. `dx-devtools/lib/forge/state/zustand/ssr-safe.ts` | ext `.ts` | 570 bytes
1759. `dx-devtools/lib/forge/state/zustand/traditional.ts` | ext `.ts` | 3493 bytes
1760. `dx-devtools/lib/forge/state/zustand/vanilla.ts` | ext `.ts` | 3319 bytes
1761. `dx-devtools/lib/forms/react-hook-form/example.tsx` | ext `.tsx` | 1015 bytes
1762. `dx-devtools/lib/forms/react-hook-form/fields.tsx` | ext `.tsx` | 3981 bytes
1763. `dx-devtools/lib/forms/react-hook-form/form.tsx` | ext `.tsx` | 2091 bytes
1764. `dx-devtools/lib/forms/react-hook-form/metadata.ts` | ext `.ts` | 2468 bytes
1765. `dx-devtools/lib/forms/react-hook-form/README.md` | ext `.md` | 1598 bytes
1766. `dx-devtools/lib/forms/react-hook-form/resolver.ts` | ext `.ts` | 2240 bytes
1767. `dx-devtools/lib/fumadocs/dashboard-workflow.ts` | ext `.ts` | 7083 bytes
1768. `dx-devtools/lib/fumadocs/layout.tsx` | ext `.tsx` | 184 bytes
1769. `dx-devtools/lib/fumadocs/llms.ts` | ext `.ts` | 1338 bytes
1770. `dx-devtools/lib/fumadocs/metadata.ts` | ext `.ts` | 9981 bytes
1771. `dx-devtools/lib/fumadocs/navigation.ts` | ext `.ts` | 2409 bytes
1772. `dx-devtools/lib/fumadocs/openapi.ts` | ext `.ts` | 1481 bytes
1773. `dx-devtools/lib/fumadocs/openapi-code-usage.ts` | ext `.ts` | 2008 bytes
1774. `dx-devtools/lib/fumadocs/README.md` | ext `.md` | 5037 bytes
1775. `dx-devtools/lib/fumadocs/route-contract.ts` | ext `.ts` | 4095 bytes
1776. `dx-devtools/lib/fumadocs/search.ts` | ext `.ts` | 939 bytes
1777. `dx-devtools/lib/fumadocs/search-client.ts` | ext `.ts` | 920 bytes
1778. `dx-devtools/lib/fumadocs/source.ts` | ext `.ts` | 810 bytes
1779. `dx-devtools/lib/fumadocs/source-plugins.tsx` | ext `.tsx` | 1999 bytes
1780. `dx-devtools/lib/fumadocs/toc.ts` | ext `.ts` | 1935 bytes
1781. `dx-devtools/lib/icons.ts` | ext `.ts` | 824 bytes
1782. `dx-devtools/lib/instant/auth.ts` | ext `.ts` | 1301 bytes
1783. `dx-devtools/lib/instant/client.ts` | ext `.ts` | 664 bytes
1784. `dx-devtools/lib/instant/dashboard-workflow.ts` | ext `.ts` | 8305 bytes
1785. `dx-devtools/lib/instant/diagnostics.ts` | ext `.ts` | 787 bytes
1786. `dx-devtools/lib/instant/env.ts` | ext `.ts` | 2912 bytes
1787. `dx-devtools/lib/instant/metadata.ts` | ext `.ts` | 9592 bytes
1788. `dx-devtools/lib/instant/mutations.ts` | ext `.ts` | 2070 bytes
1789. `dx-devtools/lib/instant/next-client.tsx` | ext `.tsx` | 973 bytes
1790. `dx-devtools/lib/instant/next-server.ts` | ext `.ts` | 302 bytes
1791. `dx-devtools/lib/instant/oauth.ts` | ext `.ts` | 916 bytes
1792. `dx-devtools/lib/instant/pagination.ts` | ext `.ts` | 623 bytes
1793. `dx-devtools/lib/instant/perms.ts` | ext `.ts` | 442 bytes
1794. `dx-devtools/lib/instant/queries.ts` | ext `.ts` | 312 bytes
1795. `dx-devtools/lib/instant/README.md` | ext `.md` | 6873 bytes
1796. `dx-devtools/lib/instant/route.ts` | ext `.ts` | 286 bytes
1797. `dx-devtools/lib/instant/rules.ts` | ext `.ts` | 852 bytes
1798. `dx-devtools/lib/instant/schema.ts` | ext `.ts` | 1039 bytes
1799. `dx-devtools/lib/instant/status.ts` | ext `.ts` | 214 bytes
1800. `dx-devtools/lib/instant/storage.ts` | ext `.ts` | 1972 bytes
1801. `dx-devtools/lib/instant/streams.ts` | ext `.ts` | 741 bytes
1802. `dx-devtools/lib/instant/subscriptions.ts` | ext `.ts` | 876 bytes
1803. `dx-devtools/lib/instant/sync-table.ts` | ext `.ts` | 1955 bytes
1804. `dx-devtools/lib/markdown-mdx-content/receipt.ts` | ext `.ts` | 7746 bytes
1805. `dx-devtools/lib/mdx/metadata.ts` | ext `.ts` | 4287 bytes
1806. `dx-devtools/lib/mdx/README.md` | ext `.md` | 455 bytes
1807. `dx-devtools/lib/payments/stripe-js/checkout.ts` | ext `.ts` | 13303 bytes
1808. `dx-devtools/lib/payments/stripe-js/client.ts` | ext `.ts` | 1433 bytes
1809. `dx-devtools/lib/payments/stripe-js/config.ts` | ext `.ts` | 4005 bytes
1810. `dx-devtools/lib/payments/stripe-js/dashboard-checkout.ts` | ext `.ts` | 4034 bytes
1811. `dx-devtools/lib/payments/stripe-js/metadata.ts` | ext `.ts` | 9310 bytes
1812. `dx-devtools/lib/payments/stripe-js/payment.ts` | ext `.ts` | 3163 bytes
1813. `dx-devtools/lib/payments/stripe-js/README.md` | ext `.md` | 8858 bytes
1814. `dx-devtools/lib/payments/stripe-js/server.ts` | ext `.ts` | 17143 bytes
1815. `dx-devtools/lib/query/activity.tsx` | ext `.tsx` | 2509 bytes
1816. `dx-devtools/lib/query/broadcast.ts` | ext `.ts` | 2334 bytes
1817. `dx-devtools/lib/query/cache.ts` | ext `.ts` | 3729 bytes
1818. `dx-devtools/lib/query/cache-events.ts` | ext `.ts` | 3473 bytes
1819. `dx-devtools/lib/query/client.ts` | ext `.ts` | 1567 bytes
1820. `dx-devtools/lib/query/client-lifecycle.ts` | ext `.ts` | 1762 bytes
1821. `dx-devtools/lib/query/dashboard-workflow.ts` | ext `.ts` | 7779 bytes
1822. `dx-devtools/lib/query/defaults.ts` | ext `.ts` | 3214 bytes
1823. `dx-devtools/lib/query/devtools.tsx` | ext `.tsx` | 1971 bytes
1824. `dx-devtools/lib/query/disabled.ts` | ext `.ts` | 1341 bytes
1825. `dx-devtools/lib/query/error-boundary.tsx` | ext `.tsx` | 1954 bytes
1826. `dx-devtools/lib/query/errors.ts` | ext `.ts` | 1974 bytes
1827. `dx-devtools/lib/query/fetch.ts` | ext `.ts` | 3949 bytes
1828. `dx-devtools/lib/query/hydration.ts` | ext `.ts` | 2492 bytes
1829. `dx-devtools/lib/query/infinite.ts` | ext `.ts` | 2089 bytes
1830. `dx-devtools/lib/query/keys.ts` | ext `.ts` | 709 bytes
1831. `dx-devtools/lib/query/lifecycle.tsx` | ext `.tsx` | 2833 bytes
1832. `dx-devtools/lib/query/matches.ts` | ext `.ts` | 1944 bytes
1833. `dx-devtools/lib/query/metadata.ts` | ext `.ts` | 12303 bytes
1834. `dx-devtools/lib/query/mutation.ts` | ext `.ts` | 1417 bytes
1835. `dx-devtools/lib/query/mutation-result.ts` | ext `.ts` | 3855 bytes
1836. `dx-devtools/lib/query/next-streaming.tsx` | ext `.tsx` | 2195 bytes
1837. `dx-devtools/lib/query/observers.ts` | ext `.ts` | 3786 bytes
1838. `dx-devtools/lib/query/persist.tsx` | ext `.tsx` | 5243 bytes
1839. `dx-devtools/lib/query/placeholder.ts` | ext `.ts` | 1482 bytes
1840. `dx-devtools/lib/query/prefetch.tsx` | ext `.tsx` | 1264 bytes
1841. `dx-devtools/lib/query/prefetch-hooks.tsx` | ext `.tsx` | 1910 bytes
1842. `dx-devtools/lib/query/provider.tsx` | ext `.tsx` | 845 bytes
1843. `dx-devtools/lib/query/queries.tsx` | ext `.tsx` | 3471 bytes
1844. `dx-devtools/lib/query/query-result.ts` | ext `.ts` | 4312 bytes
1845. `dx-devtools/lib/query/react-context.tsx` | ext `.tsx` | 3953 bytes
1846. `dx-devtools/lib/query/README.md` | ext `.md` | 11236 bytes
1847. `dx-devtools/lib/query/restoring.tsx` | ext `.tsx` | 1407 bytes
1848. `dx-devtools/lib/query/runtime.ts` | ext `.ts` | 1970 bytes
1849. `dx-devtools/lib/query/state.ts` | ext `.ts` | 4046 bytes
1850. `dx-devtools/lib/query/stream.ts` | ext `.ts` | 2204 bytes
1851. `dx-devtools/lib/query/suspense.tsx` | ext `.tsx` | 2476 bytes
1852. `dx-devtools/lib/query/sync-persist.ts` | ext `.ts` | 3048 bytes
1853. `dx-devtools/lib/react-markdown/metadata.ts` | ext `.ts` | 4287 bytes
1854. `dx-devtools/lib/react-markdown/README.md` | ext `.md` | 455 bytes
1855. `dx-devtools/lib/scene/bounds-report.ts` | ext `.ts` | 5163 bytes
1856. `dx-devtools/lib/scene/capability-report.ts` | ext `.ts` | 3623 bytes
1857. `dx-devtools/lib/scene/dashboard-controls.ts` | ext `.ts` | 1339 bytes
1858. `dx-devtools/lib/scene/dashboard-workflow.ts` | ext `.ts` | 6741 bytes
1859. `dx-devtools/lib/scene/frame-sample.ts` | ext `.ts` | 2955 bytes
1860. `dx-devtools/lib/scene/index.ts` | ext `.ts` | 4710 bytes
1861. `dx-devtools/lib/scene/interaction.ts` | ext `.ts` | 5719 bytes
1862. `dx-devtools/lib/scene/metadata.ts` | ext `.ts` | 8902 bytes
1863. `dx-devtools/lib/scene/performance-monitor.ts` | ext `.ts` | 3554 bytes
1864. `dx-devtools/lib/scene/preset.ts` | ext `.ts` | 4132 bytes
1865. `dx-devtools/lib/scene/preview-readiness.ts` | ext `.ts` | 3920 bytes
1866. `dx-devtools/lib/scene/r3f-renderer-adapter.ts` | ext `.ts` | 4490 bytes
1867. `dx-devtools/lib/scene/raycast-report.ts` | ext `.ts` | 3061 bytes
1868. `dx-devtools/lib/scene/README.md` | ext `.md` | 14296 bytes
1869. `dx-devtools/lib/scene/renderer-handoff.ts` | ext `.ts` | 1357 bytes
1870. `dx-devtools/lib/scene/types.ts` | ext `.ts` | 9108 bytes
1871. `dx-devtools/lib/scene/viewport-report.ts` | ext `.ts` | 3907 bytes
1872. `dx-devtools/lib/scene/webgl-runtime.ts` | ext `.ts` | 25356 bytes
1873. `dx-devtools/lib/supabase/.env.example` | ext `.example` | 154 bytes
1874. `dx-devtools/lib/supabase/auth-actions.ts` | ext `.ts` | 3183 bytes
1875. `dx-devtools/lib/supabase/auth-anonymous.ts` | ext `.ts` | 5136 bytes
1876. `dx-devtools/lib/supabase/auth-callback.ts` | ext `.ts` | 2617 bytes
1877. `dx-devtools/lib/supabase/auth-confirm.ts` | ext `.ts` | 3242 bytes
1878. `dx-devtools/lib/supabase/auth-guard.ts` | ext `.ts` | 2137 bytes
1879. `dx-devtools/lib/supabase/auth-identities.ts` | ext `.ts` | 5390 bytes
1880. `dx-devtools/lib/supabase/auth-mfa.ts` | ext `.ts` | 5433 bytes
1881. `dx-devtools/lib/supabase/auth-oauth.ts` | ext `.ts` | 2172 bytes
1882. `dx-devtools/lib/supabase/auth-otp.ts` | ext `.ts` | 2316 bytes
1883. `dx-devtools/lib/supabase/auth-session.ts` | ext `.ts` | 3007 bytes
1884. `dx-devtools/lib/supabase/avatar-storage.ts` | ext `.ts` | 4318 bytes
1885. `dx-devtools/lib/supabase/browser.ts` | ext `.ts` | 642 bytes
1886. `dx-devtools/lib/supabase/database-rows.ts` | ext `.ts` | 13721 bytes
1887. `dx-devtools/lib/supabase/edge-functions.ts` | ext `.ts` | 3228 bytes
1888. `dx-devtools/lib/supabase/env.ts` | ext `.ts` | 1771 bytes
1889. `dx-devtools/lib/supabase/metadata.ts` | ext `.ts` | 11576 bytes
1890. `dx-devtools/lib/supabase/password-recovery.ts` | ext `.ts` | 2836 bytes
1891. `dx-devtools/lib/supabase/profiles.ts` | ext `.ts` | 4272 bytes
1892. `dx-devtools/lib/supabase/profile-workflow.ts` | ext `.ts` | 4678 bytes
1893. `dx-devtools/lib/supabase/proxy.ts` | ext `.ts` | 1497 bytes
1894. `dx-devtools/lib/supabase/README.md` | ext `.md` | 13481 bytes
1895. `dx-devtools/lib/supabase/realtime-auth.ts` | ext `.ts` | 3233 bytes
1896. `dx-devtools/lib/supabase/realtime-broadcast.ts` | ext `.ts` | 3053 bytes
1897. `dx-devtools/lib/supabase/realtime-postgres.ts` | ext `.ts` | 2278 bytes
1898. `dx-devtools/lib/supabase/realtime-presence.ts` | ext `.ts` | 3619 bytes
1899. `dx-devtools/lib/supabase/rpc.ts` | ext `.ts` | 1611 bytes
1900. `dx-devtools/lib/supabase/schema.sql` | ext `.sql` | 1167 bytes
1901. `dx-devtools/lib/supabase/server.ts` | ext `.ts` | 1109 bytes
1902. `dx-devtools/lib/supabase/signed-storage.ts` | ext `.ts` | 6197 bytes
1903. `dx-devtools/lib/supabase/storage-objects.ts` | ext `.ts` | 6563 bytes
1904. `dx-devtools/lib/trpc/client.ts` | ext `.ts` | 1293 bytes
1905. `dx-devtools/lib/trpc/context.ts` | ext `.ts` | 829 bytes
1906. `dx-devtools/lib/trpc/dashboard-workflow.ts` | ext `.ts` | 5637 bytes
1907. `dx-devtools/lib/trpc/errors.ts` | ext `.ts` | 1740 bytes
1908. `dx-devtools/lib/trpc/http.ts` | ext `.ts` | 2590 bytes
1909. `dx-devtools/lib/trpc/metadata.ts` | ext `.ts` | 8958 bytes
1910. `dx-devtools/lib/trpc/provider.tsx` | ext `.tsx` | 2823 bytes
1911. `dx-devtools/lib/trpc/README.md` | ext `.md` | 9853 bytes
1912. `dx-devtools/lib/trpc/response-meta.ts` | ext `.ts` | 2176 bytes
1913. `dx-devtools/lib/trpc/route-handler.ts` | ext `.ts` | 1572 bytes
1914. `dx-devtools/lib/trpc/router.ts` | ext `.ts` | 3939 bytes
1915. `dx-devtools/lib/trpc/server.ts` | ext `.ts` | 754 bytes
1916. `dx-devtools/lib/trpc/server-caller.ts` | ext `.ts` | 1088 bytes
1917. `dx-devtools/lib/trpc/streaming-client.ts` | ext `.ts` | 3423 bytes
1918. `dx-devtools/lib/trpc/subscriptions.ts` | ext `.ts` | 1878 bytes
1919. `dx-devtools/lib/trpc/transformer.ts` | ext `.ts` | 975 bytes
1920. `dx-devtools/lib/utils.ts` | ext `.ts` | 119 bytes
1921. `dx-devtools/lib/validation/zod/catalog.ts` | ext `.ts` | 4302 bytes
1922. `dx-devtools/lib/validation/zod/codecs.ts` | ext `.ts` | 1261 bytes
1923. `dx-devtools/lib/validation/zod/coerce.ts` | ext `.ts` | 1304 bytes
1924. `dx-devtools/lib/validation/zod/dashboard-settings.ts` | ext `.ts` | 3601 bytes
1925. `dx-devtools/lib/validation/zod/env.ts` | ext `.ts` | 1337 bytes
1926. `dx-devtools/lib/validation/zod/errors.ts` | ext `.ts` | 1249 bytes
1927. `dx-devtools/lib/validation/zod/example.ts` | ext `.ts` | 4546 bytes
1928. `dx-devtools/lib/validation/zod/files.ts` | ext `.ts` | 1591 bytes
1929. `dx-devtools/lib/validation/zod/json-schema.ts` | ext `.ts` | 689 bytes
1930. `dx-devtools/lib/validation/zod/json-schema-import.ts` | ext `.ts` | 1319 bytes
1931. `dx-devtools/lib/validation/zod/metadata.ts` | ext `.ts` | 8158 bytes
1932. `dx-devtools/lib/validation/zod/objects.ts` | ext `.ts` | 1491 bytes
1933. `dx-devtools/lib/validation/zod/parse.ts` | ext `.ts` | 1593 bytes
1934. `dx-devtools/lib/validation/zod/patterns.ts` | ext `.ts` | 998 bytes
1935. `dx-devtools/lib/validation/zod/README.md` | ext `.md` | 9632 bytes
1936. `dx-devtools/lib/validation/zod/refinements.ts` | ext `.ts` | 2213 bytes
1937. `dx-devtools/lib/validation/zod/registry.ts` | ext `.ts` | 3212 bytes
1938. `dx-devtools/lib/validation/zod/schemas.ts` | ext `.ts` | 1275 bytes
1939. `dx-devtools/lib/validation/zod/transforms.ts` | ext `.ts` | 971 bytes
1940. `dx-devtools/motion/controls.tsx` | ext `.tsx` | 3884 bytes
1941. `dx-devtools/motion/dashboard-workflow.ts` | ext `.ts` | 13517 bytes
1942. `dx-devtools/motion/frame.tsx` | ext `.tsx` | 3470 bytes
1943. `dx-devtools/motion/layout.tsx` | ext `.tsx` | 2582 bytes
1944. `dx-devtools/motion/lazy.tsx` | ext `.tsx` | 1461 bytes
1945. `dx-devtools/motion/metadata.ts` | ext `.ts` | 7033 bytes
1946. `dx-devtools/motion/motion-values.tsx` | ext `.tsx` | 3918 bytes
1947. `dx-devtools/motion/page-visibility.tsx` | ext `.tsx` | 1588 bytes
1948. `dx-devtools/motion/presence.tsx` | ext `.tsx` | 2841 bytes
1949. `dx-devtools/motion/presets.ts` | ext `.ts` | 1062 bytes
1950. `dx-devtools/motion/provider.tsx` | ext `.tsx` | 859 bytes
1951. `dx-devtools/motion/README.md` | ext `.md` | 12014 bytes
1952. `dx-devtools/motion/reorder.tsx` | ext `.tsx` | 2980 bytes
1953. `dx-devtools/motion/reveal.tsx` | ext `.tsx` | 1444 bytes
1954. `dx-devtools/motion/scoped-animate.tsx` | ext `.tsx` | 1984 bytes
1955. `dx-devtools/motion/scroll-progress.tsx` | ext `.tsx` | 1753 bytes
1956. `dx-devtools/motion/will-change.tsx` | ext `.tsx` | 2275 bytes
1957. `dx-devtools/next.config.mjs` | ext `.mjs` | 395 bytes
1958. `dx-devtools/openapi/dx-launch.yaml` | ext `.yaml` | 2507 bytes
1959. `dx-devtools/public/d-logo.svg` | ext `.svg` | 249 bytes
1960. `dx-devtools/public/favicon.svg` | ext `.svg` | 249 bytes
1961. `dx-devtools/public/og-image.svg` | ext `.svg` | 1079 bytes
1962. `dx-devtools/public/robots.txt` | ext `.txt` | 23 bytes
1963. `dx-devtools/README.md` | ext `.md` | 1969 bytes
1964. `dx-devtools/server/actions.ts` | ext `.ts` | 124 bytes
1965. `dx-devtools/server/content/mdx.ts` | ext `.ts` | 1630 bytes
1966. `dx-devtools/server/launchCatalog.ts` | ext `.ts` | 157 bytes
1967. `dx-devtools/server/loaders.ts` | ext `.ts` | 121 bytes
1968. `dx-devtools/source.config.ts` | ext `.ts` | 572 bytes
1969. `dx-devtools/styles/app.generated.css` | ext `.css` | 6277 bytes
1970. `dx-devtools/styles/devtools.css` | ext `.css` | 299 bytes
1971. `dx-devtools/styles/devtools/part-01-foundation.css` | ext `.css` | 26753 bytes
1972. `dx-devtools/styles/devtools/part-02-panels.css` | ext `.css` | 23111 bytes
1973. `dx-devtools/styles/devtools/part-03-inspector.css` | ext `.css` | 25260 bytes
1974. `dx-devtools/styles/devtools/part-04-controls.css` | ext `.css` | 23887 bytes
1975. `dx-devtools/styles/devtools/part-05-responsive.css` | ext `.css` | 25027 bytes
1976. `dx-devtools/styles/global.css` | ext `.css` | 31 bytes
1977. `dx-devtools/styles/theme.dx.css` | ext `.css` | 1105 bytes
1978. `dx-devtools/styles/tokens.css` | ext `.css` | 26 bytes
1979. `dx-devtools/wasm/bindgen/dashboard-workflow.tsx` | ext `.tsx` | 3042 bytes
1980. `dx-devtools/wasm/bindgen/example.tsx` | ext `.tsx` | 555 bytes
1981. `dx-devtools/wasm/bindgen/loader.ts` | ext `.ts` | 35724 bytes
1982. `dx-devtools/wasm/bindgen/metadata.ts` | ext `.ts` | 8503 bytes
1983. `dx-devtools/wasm/bindgen/react.tsx` | ext `.tsx` | 13040 bytes
1984. `dx-devtools/wasm/bindgen/README.md` | ext `.md` | 5327 bytes
1985. `dx-option/Cargo.toml` | ext `.toml` | 1625 bytes
1986. `dx-option/dx.config.toml` | ext `.toml` | 1639 bytes
1987. `dx-option/src/animation.rs` | ext `.rs` | 12243 bytes
1988. `dx-option/src/forms.rs` | ext `.rs` | 18461 bytes
1989. `dx-option/src/lib.rs` | ext `.rs` | 716 bytes
1990. `dx-option/src/main.rs` | ext `.rs` | 2033 bytes
1991. `dx-option/src/query_builder.rs` | ext `.rs` | 18341 bytes
1992. `dx-option/src/three_d.rs` | ext `.rs` | 10813 bytes
1993. `dx-www/Cargo.toml` | ext `.toml` | 3782 bytes
1994. `dx-www/README.md` | ext `.md` | 3718 bytes
1995. `dx-www/src/api/mod.rs` | ext `.rs` | 21810 bytes
1996. `dx-www/src/app_router_segments.rs` | ext `.rs` | 16167 bytes
1997. `dx-www/src/assets/mod.rs` | ext `.rs` | 12046 bytes
1998. `dx-www/src/cli/add_args.rs` | ext `.rs` | 2113 bytes
1999. `dx-www/src/cli/agent_context.rs` | ext `.rs` | 13316 bytes
2000. `dx-www/src/cli/app_api_routes.rs` | ext `.rs` | 12379 bytes
2001. `dx-www/src/cli/app_page_route_diagnostics.rs` | ext `.rs` | 4659 bytes
2002. `dx-www/src/cli/app_page_routes.rs` | ext `.rs` | 67697 bytes
2003. `dx-www/src/cli/app_route_diagnostics.rs` | ext `.rs` | 18305 bytes
2004. `dx-www/src/cli/app_route_handler_build_output.rs` | ext `.rs` | 8878 bytes
2005. `dx-www/src/cli/app_route_handler_receipt.rs` | ext `.rs` | 3892 bytes
2006. `dx-www/src/cli/app_router_build_command.rs` | ext `.rs` | 10499 bytes
2007. `dx-www/src/cli/app_router_build_output.rs` | ext `.rs` | 32895 bytes
2008. `dx-www/src/cli/app_router_execution.rs` | ext `.rs` | 68233 bytes
2009. `dx-www/src/cli/app_router_execution/directives.rs` | ext `.rs` | 4871 bytes
2010. `dx-www/src/cli/app_router_execution/metadata.rs` | ext `.rs` | 118872 bytes
2011. `dx-www/src/cli/app_router_execution/next_custom_transforms.rs` | ext `.rs` | 19846 bytes
2012. `dx-www/src/cli/app_router_execution/next_custom_transforms/conflicts.rs` | ext `.rs` | 11601 bytes
2013. `dx-www/src/cli/app_router_execution/next_custom_transforms/contract.rs` | ext `.rs` | 6194 bytes
2014. `dx-www/src/cli/app_router_execution/next_custom_transforms/dynamic_imports.rs` | ext `.rs` | 13860 bytes
2015. `dx-www/src/cli/app_router_execution/next_custom_transforms/font_loaders.rs` | ext `.rs` | 14360 bytes
2016. `dx-www/src/cli/app_router_execution/next_custom_transforms/inline_server_actions.rs` | ext `.rs` | 9232 bytes
2017. `dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs` | ext `.rs` | 17762 bytes
2018. `dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports/scanner.rs` | ext `.rs` | 6114 bytes
2019. `dx-www/src/cli/app_router_execution/next_custom_transforms/page_config_exports.rs` | ext `.rs` | 14674 bytes
2020. `dx-www/src/cli/app_router_execution/next_custom_transforms/rsc_boundaries.rs` | ext `.rs` | 5020 bytes
2021. `dx-www/src/cli/app_router_execution/next_custom_transforms/server_actions.rs` | ext `.rs` | 5317 bytes
2022. `dx-www/src/cli/app_router_execution/next_navigation.rs` | ext `.rs` | 30654 bytes
2023. `dx-www/src/cli/app_router_execution/render_plan.rs` | ext `.rs` | 7550 bytes
2024. `dx-www/src/cli/app_router_execution/request_props.rs` | ext `.rs` | 30716 bytes
2025. `dx-www/src/cli/app_router_execution/source_render.rs` | ext `.rs` | 246767 bytes
2026. `dx-www/src/cli/app_router_execution/state_runtime.rs` | ext `.rs` | 25127 bytes
2027. `dx-www/src/cli/app_router_paths.rs` | ext `.rs` | 604 bytes
2028. `dx-www/src/cli/app_router_runtime_command.rs` | ext `.rs` | 10823 bytes
2029. `dx-www/src/cli/app_router_semantics.rs` | ext `.rs` | 23471 bytes
2030. `dx-www/src/cli/app_router_server_data.rs` | ext `.rs` | 8150 bytes
2031. `dx-www/src/cli/app_router_style_assets.rs` | ext `.rs` | 6779 bytes
2032. `dx-www/src/cli/app_segment_files.rs` | ext `.rs` | 6485 bytes
2033. `dx-www/src/cli/app_server_data_manifest.rs` | ext `.rs` | 17189 bytes
2034. `dx-www/src/cli/build_command.rs` | ext `.rs` | 18385 bytes
2035. `dx-www/src/cli/build_observability.rs` | ext `.rs` | 7369 bytes
2036. `dx-www/src/cli/build_options.rs` | ext `.rs` | 1008 bytes
2037. `dx-www/src/cli/build_promotion.rs` | ext `.rs` | 18599 bytes
2038. `dx-www/src/cli/build_rollback_verification.rs` | ext `.rs` | 10613 bytes
2039. `dx-www/src/cli/command_output.rs` | ext `.rs` | 1957 bytes
2040. `dx-www/src/cli/config_diagnostics.rs` | ext `.rs` | 2947 bytes
2041. `dx-www/src/cli/css_diagnostics.rs` | ext `.rs` | 6405 bytes
2042. `dx-www/src/cli/default_template_contract.rs` | ext `.rs` | 2801 bytes
2043. `dx-www/src/cli/default_template_materializer.rs` | ext `.rs` | 15597 bytes
2044. `dx-www/src/cli/default_template_sources.rs` | ext `.rs` | 2879 bytes
2045. `dx-www/src/cli/deploy_adapter_contract.rs` | ext `.rs` | 18688 bytes
2046. `dx-www/src/cli/dev_bridge.rs` | ext `.rs` | 2463 bytes
2047. `dx-www/src/cli/dev_command.rs` | ext `.rs` | 9049 bytes
2048. `dx-www/src/cli/dev_hot_reload_client.rs` | ext `.rs` | 26213 bytes
2049. `dx-www/src/cli/dev_http.rs` | ext `.rs` | 14911 bytes
2050. `dx-www/src/cli/dev_options.rs` | ext `.rs` | 17443 bytes
2051. `dx-www/src/cli/dev_response.rs` | ext `.rs` | 8983 bytes
2052. `dx-www/src/cli/dev_wire.rs` | ext `.rs` | 14155 bytes
2053. `dx-www/src/cli/devtools/assets.rs` | ext `.rs` | 2387 bytes
2054. `dx-www/src/cli/devtools/assets/devtools.css` | ext `.css` | 19965 bytes
2055. `dx-www/src/cli/devtools/assets/runtime.ts` | ext `.ts` | 472 bytes
2056. `dx-www/src/cli/devtools/assets/runtime/part-01-boot.ts` | ext `.ts` | 16156 bytes
2057. `dx-www/src/cli/devtools/assets/runtime/part-02-protocol.ts` | ext `.ts` | 14644 bytes
2058. `dx-www/src/cli/devtools/assets/runtime/part-03-controls.ts` | ext `.ts` | 18925 bytes
2059. `dx-www/src/cli/devtools/assets/runtime/part-04-render.ts` | ext `.ts` | 18652 bytes
2060. `dx-www/src/cli/devtools/assets/runtime/part-05-events.ts` | ext `.ts` | 18605 bytes
2061. `dx-www/src/cli/devtools/css_data.generated.json` | ext `.json` | 397724 bytes
2062. `dx-www/src/cli/devtools/css_data.rs` | ext `.rs` | 80 bytes
2063. `dx-www/src/cli/devtools/mod.rs` | ext `.rs` | 2573 bytes
2064. `dx-www/src/cli/devtools/protocol.rs` | ext `.rs` | 20157 bytes
2065. `dx-www/src/cli/devtools/source_map.rs` | ext `.rs` | 26216 bytes
2066. `dx-www/src/cli/devtools/style_ops.rs` | ext `.rs` | 32232 bytes
2067. `dx-www/src/cli/devtools/style_ops_tests.rs` | ext `.rs` | 13164 bytes
2068. `dx-www/src/cli/dx_check_latest_receipt.rs` | ext `.rs` | 7899 bytes
2069. `dx-www/src/cli/dx_style_support.rs` | ext `.rs` | 22467 bytes
2070. `dx-www/src/cli/extension_orchestrator.rs` | ext `.rs` | 21019 bytes
2071. `dx-www/src/cli/forge_add_options.rs` | ext `.rs` | 9823 bytes
2072. `dx-www/src/cli/forge_adoption_options.rs` | ext `.rs` | 12488 bytes
2073. `dx-www/src/cli/forge_audit_options.rs` | ext `.rs` | 4813 bytes
2074. `dx-www/src/cli/forge_beta_diagnostics.rs` | ext `.rs` | 21250 bytes
2075. `dx-www/src/cli/forge_beta_options.rs` | ext `.rs` | 17917 bytes
2076. `dx-www/src/cli/forge_ci_snippets.rs` | ext `.rs` | 20830 bytes
2077. `dx-www/src/cli/forge_ci_snippets_options.rs` | ext `.rs` | 8132 bytes
2078. `dx-www/src/cli/forge_doctor.rs` | ext `.rs` | 10425 bytes
2079. `dx-www/src/cli/forge_evidence_options.rs` | ext `.rs` | 6993 bytes
2080. `dx-www/src/cli/forge_failure_triage.rs` | ext `.rs` | 6743 bytes
2081. `dx-www/src/cli/forge_hosted_registry_smoke.rs` | ext `.rs` | 14230 bytes
2082. `dx-www/src/cli/forge_hosting_manifest.rs` | ext `.rs` | 8759 bytes
2083. `dx-www/src/cli/forge_init_app_options.rs` | ext `.rs` | 6548 bytes
2084. `dx-www/src/cli/forge_launch_changelog.rs` | ext `.rs` | 14664 bytes
2085. `dx-www/src/cli/forge_launch_copy_review.rs` | ext `.rs` | 41839 bytes
2086. `dx-www/src/cli/forge_launch_page.rs` | ext `.rs` | 3647 bytes
2087. `dx-www/src/cli/forge_migrated_route_benchmark.rs` | ext `.rs` | 20964 bytes
2088. `dx-www/src/cli/forge_migration_audit.rs` | ext `.rs` | 23840 bytes
2089. `dx-www/src/cli/forge_migration_workflow.rs` | ext `.rs` | 67189 bytes
2090. `dx-www/src/cli/forge_npm_import_plan.rs` | ext `.rs` | 14440 bytes
2091. `dx-www/src/cli/forge_operator_dashboard.rs` | ext `.rs` | 20246 bytes
2092. `dx-www/src/cli/forge_packages_command.rs` | ext `.rs` | 3102 bytes
2093. `dx-www/src/cli/forge_packages_options.rs` | ext `.rs` | 5655 bytes
2094. `dx-www/src/cli/forge_provenance.rs` | ext `.rs` | 32334 bytes
2095. `dx-www/src/cli/forge_provenance_command.rs` | ext `.rs` | 1874 bytes
2096. `dx-www/src/cli/forge_provenance_options.rs` | ext `.rs` | 7309 bytes
2097. `dx-www/src/cli/forge_public_add.rs` | ext `.rs` | 5011 bytes
2098. `dx-www/src/cli/forge_public_evidence.rs` | ext `.rs` | 29264 bytes
2099. `dx-www/src/cli/forge_public_evidence_options.rs` | ext `.rs` | 8872 bytes
2100. `dx-www/src/cli/forge_public_status.rs` | ext `.rs` | 40737 bytes
2101. `dx-www/src/cli/forge_publish_options.rs` | ext `.rs` | 8059 bytes
2102. `dx-www/src/cli/forge_publish_plan_options.rs` | ext `.rs` | 8912 bytes
2103. `dx-www/src/cli/forge_publisher_key_command.rs` | ext `.rs` | 4488 bytes
2104. `dx-www/src/cli/forge_publisher_key_options.rs` | ext `.rs` | 13643 bytes
2105. `dx-www/src/cli/forge_react_starter_benchmark.rs` | ext `.rs` | 29433 bytes
2106. `dx-www/src/cli/forge_registry_options.rs` | ext `.rs` | 15662 bytes
2107. `dx-www/src/cli/forge_release_bundle_inspect.rs` | ext `.rs` | 18577 bytes
2108. `dx-www/src/cli/forge_release_candidate.rs` | ext `.rs` | 25515 bytes
2109. `dx-www/src/cli/forge_release_candidate_command.rs` | ext `.rs` | 7876 bytes
2110. `dx-www/src/cli/forge_release_dashboard.rs` | ext `.rs` | 3988 bytes
2111. `dx-www/src/cli/forge_release_dashboard_command.rs` | ext `.rs` | 7015 bytes
2112. `dx-www/src/cli/forge_release_proof.rs` | ext `.rs` | 9230 bytes
2113. `dx-www/src/cli/forge_release_history.rs` | ext `.rs` | 30590 bytes
2114. `dx-www/src/cli/forge_release_operations_options.rs` | ext `.rs` | 10794 bytes
2115. `dx-www/src/cli/forge_release_review_options.rs` | ext `.rs` | 8917 bytes
2116. `dx-www/src/cli/forge_release_trend.rs` | ext `.rs` | 18761 bytes
2117. `dx-www/src/cli/forge_release_triage.rs` | ext `.rs` | 27413 bytes
2118. `dx-www/src/cli/forge_remote_lifecycle.rs` | ext `.rs` | 14322 bytes
2119. `dx-www/src/cli/forge_smoke_options.rs` | ext `.rs` | 15594 bytes
2120. `dx-www/src/cli/forge_static_asset_materialization.rs` | ext `.rs` | 19739 bytes
2121. `dx-www/src/cli/forge_static_migration_plan.rs` | ext `.rs` | 18692 bytes
2122. `dx-www/src/cli/forge_static_migration_smoke.rs` | ext `.rs` | 15618 bytes
2123. `dx-www/src/cli/forge_static_page_assets.rs` | ext `.rs` | 9383 bytes
2124. `dx-www/src/cli/forge_static_page_migration.rs` | ext `.rs` | 42528 bytes
2125. `dx-www/src/cli/forge_static_page_policy.rs` | ext `.rs` | 4211 bytes
2126. `dx-www/src/cli/forge_static_page_preview.rs` | ext `.rs` | 6514 bytes
2127. `dx-www/src/cli/forge_trust_policy_command.rs` | ext `.rs` | 2122 bytes
2128. `dx-www/src/cli/forge_trust_policy_options.rs` | ext `.rs` | 7299 bytes
2129. `dx-www/src/cli/forge_trust_regression.rs` | ext `.rs` | 18501 bytes
2130. `dx-www/src/cli/forge_trust_regression_command.rs` | ext `.rs` | 1980 bytes
2131. `dx-www/src/cli/forge_trust_regression_options.rs` | ext `.rs` | 7286 bytes
2132. `dx-www/src/cli/forge_update_options.rs` | ext `.rs` | 12077 bytes
2133. `dx-www/src/cli/formatting.rs` | ext `.rs` | 2099 bytes
2134. `dx-www/src/cli/generate_command.rs` | ext `.rs` | 4283 bytes
2135. `dx-www/src/cli/help_text.rs` | ext `.rs` | 42275 bytes
2136. `dx-www/src/cli/hosted_preview_contract.rs` | ext `.rs` | 14236 bytes
2137. `dx-www/src/cli/launch_adoption_report.rs` | ext `.rs` | 12065 bytes
2138. `dx-www/src/cli/launch_companion_receipts.rs` | ext `.rs` | 7872 bytes
2139. `dx-www/src/cli/launch_evidence_acceptance_digest.rs` | ext `.rs` | 16310 bytes
2140. `dx-www/src/cli/launch_evidence_acceptance_index.rs` | ext `.rs` | 17431 bytes
2141. `dx-www/src/cli/launch_evidence_archive_index.rs` | ext `.rs` | 16849 bytes
2142. `dx-www/src/cli/launch_evidence_archive_ledger.rs` | ext `.rs` | 17637 bytes
2143. `dx-www/src/cli/launch_evidence_archive_receipt.rs` | ext `.rs` | 14192 bytes
2144. `dx-www/src/cli/launch_evidence_closure_memo.rs` | ext `.rs` | 18236 bytes
2145. `dx-www/src/cli/launch_evidence_completion_ledger.rs` | ext `.rs` | 17716 bytes
2146. `dx-www/src/cli/launch_evidence_continuation_packet.rs` | ext `.rs` | 18749 bytes
2147. `dx-www/src/cli/launch_evidence_final_brief.rs` | ext `.rs` | 17804 bytes
2148. `dx-www/src/cli/launch_evidence_friday_baton.rs` | ext `.rs` | 17475 bytes
2149. `dx-www/src/cli/launch_evidence_handoff_capsule.rs` | ext `.rs` | 20341 bytes
2150. `dx-www/src/cli/launch_evidence_handoff_digest.rs` | ext `.rs` | 17809 bytes
2151. `dx-www/src/cli/launch_evidence_operator_index.rs` | ext `.rs` | 18777 bytes
2152. `dx-www/src/cli/launch_evidence_operator_resume_card.rs` | ext `.rs` | 18374 bytes
2153. `dx-www/src/cli/launch_evidence_operator_runbook.rs` | ext `.rs` | 20964 bytes
2154. `dx-www/src/cli/launch_evidence_operator_summary.rs` | ext `.rs` | 17588 bytes
2155. `dx-www/src/cli/launch_evidence_packet.rs` | ext `.rs` | 17630 bytes
2156. `dx-www/src/cli/launch_evidence_recovery_brief.rs` | ext `.rs` | 20035 bytes
2157. `dx-www/src/cli/launch_evidence_release_checklist.rs` | ext `.rs` | 16792 bytes
2158. `dx-www/src/cli/launch_evidence_release_seal.rs` | ext `.rs` | 16720 bytes
2159. `dx-www/src/cli/launch_evidence_restart_brief.rs` | ext `.rs` | 18358 bytes
2160. `dx-www/src/cli/launch_evidence_restart_checklist.rs` | ext `.rs` | 18907 bytes
2161. `dx-www/src/cli/launch_evidence_restart_closeout.rs` | ext `.rs` | 19189 bytes
2162. `dx-www/src/cli/launch_evidence_restart_dispatch.rs` | ext `.rs` | 19261 bytes
2163. `dx-www/src/cli/launch_evidence_restart_ledger.rs` | ext `.rs` | 18254 bytes
2164. `dx-www/src/cli/launch_evidence_restart_manifest.rs` | ext `.rs` | 18841 bytes
2165. `dx-www/src/cli/launch_evidence_restart_receipt.rs` | ext `.rs` | 17131 bytes
2166. `dx-www/src/cli/launch_evidence_restart_signoff.rs` | ext `.rs` | 18160 bytes
2167. `dx-www/src/cli/launch_evidence_restart_snapshot.rs` | ext `.rs` | 19196 bytes
2168. `dx-www/src/cli/launch_evidence_restart_summary.rs` | ext `.rs` | 19108 bytes
2169. `dx-www/src/cli/launch_evidence_resumption_index.rs` | ext `.rs` | 19843 bytes
2170. `dx-www/src/cli/launch_evidence_retention_policy.rs` | ext `.rs` | 18434 bytes
2171. `dx-www/src/cli/launch_evidence_retention_review.rs` | ext `.rs` | 18995 bytes
2172. `dx-www/src/cli/launch_evidence_share_manifest.rs` | ext `.rs` | 19155 bytes
2173. `dx-www/src/cli/launch_evidence_status_timeline.rs` | ext `.rs` | 20494 bytes
2174. `dx-www/src/cli/launch_manifest_drift.rs` | ext `.rs` | 18790 bytes
2175. `dx-www/src/cli/launch_readiness_bundle.rs` | ext `.rs` | 32797 bytes
2176. `dx-www/src/cli/launch_report_options.rs` | ext `.rs` | 6720 bytes
2177. `dx-www/src/cli/launch_runtime_approval_request.rs` | ext `.rs` | 13343 bytes
2178. `dx-www/src/cli/launch_runtime_checklist.rs` | ext `.rs` | 9544 bytes
2179. `dx-www/src/cli/launch_runtime_evidence.rs` | ext `.rs` | 15947 bytes
2180. `dx-www/src/cli/launch_runtime_evidence_completeness.rs` | ext `.rs` | 17696 bytes
2181. `dx-www/src/cli/launch_runtime_evidence_finalization.rs` | ext `.rs` | 22873 bytes
2182. `dx-www/src/cli/launch_runtime_evidence_import_plan.rs` | ext `.rs` | 14164 bytes
2183. `dx-www/src/cli/launch_runtime_evidence_review.rs` | ext `.rs` | 19529 bytes
2184. `dx-www/src/cli/launch_verification_lane.rs` | ext `.rs` | 13337 bytes
2185. `dx-www/src/cli/migrate_command.rs` | ext `.rs` | 3225 bytes
2186. `dx-www/src/cli/migrate_options.rs` | ext `.rs` | 9870 bytes
2187. `dx-www/src/cli/mod.rs` | ext `.rs` | 975150 bytes
2188. `dx-www/src/cli/naming.rs` | ext `.rs` | 2124 bytes
2189. `dx-www/src/cli/new_command.rs` | ext `.rs` | 87946 bytes
2190. `dx-www/src/cli/next_adapter_fixtures.rs` | ext `.rs` | 11160 bytes
2191. `dx-www/src/cli/next_familiar_fixtures.rs` | ext `.rs` | 25397 bytes
2192. `dx-www/src/cli/next_migration.rs` | ext `.rs` | 22417 bytes
2193. `dx-www/src/cli/next_migration_plan.rs` | ext `.rs` | 31748 bytes
2194. `dx-www/src/cli/next_rust_status.rs` | ext `.rs` | 3586 bytes
2195. `dx-www/src/cli/options.rs` | ext `.rs` | 3617 bytes
2196. `dx-www/src/cli/preview_command.rs` | ext `.rs` | 2510 bytes
2197. `dx-www/src/cli/preview_contract.rs` | ext `.rs` | 14521 bytes
2198. `dx-www/src/cli/preview_options.rs` | ext `.rs` | 3848 bytes
2199. `dx-www/src/cli/project_contract_hints.rs` | ext `.rs` | 12253 bytes
2200. `dx-www/src/cli/promote_command.rs` | ext `.rs` | 1977 bytes
2201. `dx-www/src/cli/promote_options.rs` | ext `.rs` | 5843 bytes
2202. `dx-www/src/cli/prove.rs` | ext `.rs` | 81944 bytes
2203. `dx-www/src/cli/prove_fixtures.rs` | ext `.rs` | 115203 bytes
2204. `dx-www/src/cli/prove_runtime.rs` | ext `.rs` | 9427 bytes
2205. `dx-www/src/cli/public_framework_tools.rs` | ext `.rs` | 266215 bytes
2206. `dx-www/src/cli/react_migration_plan.rs` | ext `.rs` | 43371 bytes
2207. `dx-www/src/cli/rollback_command.rs` | ext `.rs` | 2470 bytes
2208. `dx-www/src/cli/rollback_options.rs` | ext `.rs` | 6819 bytes
2209. `dx-www/src/cli/route_handler_runtime_env.rs` | ext `.rs` | 2056 bytes
2210. `dx-www/src/cli/route_request_values.rs` | ext `.rs` | 2142 bytes
2211. `dx-www/src/cli/script_runner.rs` | ext `.rs` | 8084 bytes
2212. `dx-www/src/cli/serializer_artifacts.rs` | ext `.rs` | 3434 bytes
2213. `dx-www/src/cli/server_action_runtime.rs` | ext `.rs` | 4366 bytes
2214. `dx-www/src/cli/studio_command.rs` | ext `.rs` | 9508 bytes
2215. `dx-www/src/cli/studio_json_surface.rs` | ext `.rs` | 5056 bytes
2216. `dx-www/src/cli/studio_manifest.rs` | ext `.rs` | 269997 bytes
2217. `dx-www/src/cli/studio_manifest/hot_reload_manifest.rs` | ext `.rs` | 12667 bytes
2218. `dx-www/src/cli/template_options.rs` | ext `.rs` | 11475 bytes
2219. `dx-www/src/cli/template_readiness.rs` | ext `.rs` | 11447 bytes
2220. `dx-www/src/cli/templates_command.rs` | ext `.rs` | 56416 bytes
2221. `dx-www/src/cli/tests.rs` | ext `.rs` | 847 bytes
2222. `dx-www/src/cli/tests/part_01.rs` | ext `.rs` | 150191 bytes
2223. `dx-www/src/cli/tests/part_02.rs` | ext `.rs` | 147693 bytes
2224. `dx-www/src/cli/tests/part_03.rs` | ext `.rs` | 151442 bytes
2225. `dx-www/src/cli/tests/part_04.rs` | ext `.rs` | 166317 bytes
2226. `dx-www/src/cli/tests/part_05.rs` | ext `.rs` | 84912 bytes
2227. `dx-www/src/cli/update_command.rs` | ext `.rs` | 2926 bytes
2228. `dx-www/src/cli/update_options.rs` | ext `.rs` | 8320 bytes
2229. `dx-www/src/config.rs` | ext `.rs` | 44195 bytes
2230. `dx-www/src/config_source.rs` | ext `.rs` | 29455 bytes
2231. `dx-www/src/data/mod.rs` | ext `.rs` | 20626 bytes
2232. `dx-www/src/dev/axum_server.rs` | ext `.rs` | 42410 bytes
2233. `dx-www/src/dev/dev_feedback.rs` | ext `.rs` | 98853 bytes
2234. `dx-www/src/dev/dev_feedback_diagnostics.rs` | ext `.rs` | 7475 bytes
2235. `dx-www/src/dev/diagnostic_snapshot.rs` | ext `.rs` | 15426 bytes
2236. `dx-www/src/dev/error_overlay.rs` | ext `.rs` | 64095 bytes
2237. `dx-www/src/dev/extension_toolchain.rs` | ext `.rs` | 22456 bytes
2238. `dx-www/src/dev/hot_reload.rs` | ext `.rs` | 8438 bytes
2239. `dx-www/src/dev/hot_reload_stream.rs` | ext `.rs` | 51082 bytes
2240. `dx-www/src/dev/mod.rs` | ext `.rs` | 14688 bytes
2241. `dx-www/src/dev/watcher.rs` | ext `.rs` | 8770 bytes
2242. `dx-www/src/diagnostics.rs` | ext `.rs` | 20590 bytes
2243. `dx-www/src/diagnostics/code_frame.rs` | ext `.rs` | 11949 bytes
2244. `dx-www/src/diagnostics/contract.rs` | ext `.rs` | 6036 bytes
2245. `dx-www/src/error.rs` | ext `.rs` | 89117 bytes
2246. `dx-www/src/error_pages.rs` | ext `.rs` | 20065 bytes
2247. `dx-www/src/hot_reload_protocol.rs` | ext `.rs` | 37409 bytes
2248. `dx-www/src/lib.rs` | ext `.rs` | 10417 bytes
2249. `dx-www/src/main.rs` | ext `.rs` | 1899 bytes
2250. `dx-www/src/next_rust.rs` | ext `.rs` | 28120 bytes
2251. `dx-www/src/next_rust_source_map_adapter.rs` | ext `.rs` | 15341 bytes
2252. `dx-www/src/next_rust_task_adapter.rs` | ext `.rs` | 21520 bytes
2253. `dx-www/src/parser/mod.rs` | ext `.rs` | 12920 bytes
2254. `dx-www/src/parser/script.rs` | ext `.rs` | 15349 bytes
2255. `dx-www/src/parser/style.rs` | ext `.rs` | 38276 bytes
2256. `dx-www/src/parser/template.rs` | ext `.rs` | 25750 bytes
2257. `dx-www/src/production/mod.rs` | ext `.rs` | 24245 bytes
2258. `dx-www/src/project.rs` | ext `.rs` | 40550 bytes
2259. `dx-www/src/property_tests.rs` | ext `.rs` | 10872 bytes
2260. `dx-www/src/router/layout.rs` | ext `.rs` | 6439 bytes
2261. `dx-www/src/router/matcher.rs` | ext `.rs` | 11492 bytes
2262. `dx-www/src/router/mod.rs` | ext `.rs` | 15774 bytes
2263. `dx-www/src/router/pattern.rs` | ext `.rs` | 13231 bytes
2264. `dx-www/tests/app_router_server_data.rs` | ext `.rs` | 9702 bytes
2265. `dx-www/tests/diagnostics_cli.rs` | ext `.rs` | 10817 bytes
2266. `dx-www/tests/dx_build_cli_contract.rs` | ext `.rs` | 2012 bytes
2267. `dx-www/tests/dx_build_tiny_app.rs` | ext `.rs` | 7786 bytes
2268. `dx-www/tests/fixtures/forge-golden/adoption-claims-shape.json` | ext `.json` | 1422 bytes
2269. `dx-www/tests/fixtures/forge-golden/adoption-proof-shape.json` | ext `.json` | 1204 bytes
2270. `dx-www/tests/fixtures/forge-golden/adoption-report-shape.json` | ext `.json` | 8070 bytes
2271. `dx-www/tests/fixtures/forge-golden/forge-changelog-budget-shape.json` | ext `.json` | 1355 bytes
2272. `dx-www/tests/fixtures/forge-golden/launch-changelog-shape.json` | ext `.json` | 1745 bytes
2273. `dx-www/tests/fixtures/forge-golden/pages-bundle-shape.json` | ext `.json` | 6812 bytes
2274. `dx-www/tests/fixtures/forge-golden/public-route-comparison-shape.json` | ext `.json` | 4115 bytes
2275. `dx-www/tests/fixtures/forge-golden/release-bundle-manifest-shape.json` | ext `.json` | 7225 bytes
2276. `dx-www/tests/fixtures/forge-golden/schema-required-fields.json` | ext `.json` | 17160 bytes
2277. `dx-www/tests/fixtures/forge-markdown/forge-adoption-report.md` | ext `.md` | 5530 bytes
2278. `dx-www/tests/fixtures/forge-markdown/forge-pages-bundle-shape.md` | ext `.md` | 3805 bytes
2279. `dx-www/tests/fixtures/forge-markdown/forge-public-launch-changelog.md` | ext `.md` | 1509 bytes
2280. `dx-www/tests/fixtures/forge-markdown/forge-public-release-history.md` | ext `.md` | 1263 bytes
2281. `dx-www/tests/fixtures/forge-markdown/forge-public-route-comparison.md` | ext `.md` | 2330 bytes
2282. `dx-www/tests/fixtures/forge-markdown/forge-release-manifest.md` | ext `.md` | 5072 bytes
2283. `dx-www/tests/fixtures/forge-markdown/public-evidence.md` | ext `.md` | 2024 bytes
2284. `dx-www/tests/fixtures/forge-markdown/release-proof-project.md` | ext `.md` | 7308 bytes
2285. `dx-www/tests/fixtures/forge-markdown/scorecard-public.md` | ext `.md` | 17073 bytes
2286. `dx-www/tests/fixtures/forge-pages/forge-site.html` | ext `.html` | 18267 bytes
2287. `dx-www/tests/fixtures/forge-pages/forge-site-source.html` | ext `.html` | 22185 bytes
2288. `dx-www/tests/forge_launch_smoke.rs` | ext `.rs` | 1851 bytes
2289. `dx-www/tests/route_handler_instant.rs` | ext `.rs` | 2018 bytes
2290. `dx-www/tests/source_build_css_pipeline.rs` | ext `.rs` | 14244 bytes
2291. `dx-www/tests/source_build_emitted_outputs.rs` | ext `.rs` | 2857 bytes
2292. `dx-www/tests/source_build_engine.rs` | ext `.rs` | 81034 bytes
2293. `dx-www/tests/source_build_graph_source_modules.rs` | ext `.rs` | 5328 bytes
2294. `dx-www/tests/source_build_image_metadata.rs` | ext `.rs` | 29736 bytes
2295. `dx-www/tests/source_build_mdx_docs.rs` | ext `.rs` | 14602 bytes
2296. `dx-www/tests/source_build_server_data.rs` | ext `.rs` | 18280 bytes
2297. `dx-www/tests/source_resolver_compat.rs` | ext `.rs` | 88202 bytes
2298. `dx-www/tests/source_resolver_config_extends.rs` | ext `.rs` | 16679 bytes
2299. `dx-www/tests/source_resolver_source_condition.rs` | ext `.rs` | 9691 bytes
2300. `error/Cargo.toml` | ext `.toml` | 540 bytes
2301. `error/CHANGELOG.md` | ext `.md` | 368 bytes
2302. `error/LICENSE` | ext `<none>` | 559 bytes
2303. `error/LICENSE-APACHE` | ext `<none>` | 5871 bytes
2304. `error/LICENSE-MIT` | ext `<none>` | 1093 bytes
2305. `error/README.md` | ext `.md` | 2286 bytes
2306. `error/src/lib.rs` | ext `.rs` | 16287 bytes
2307. `error/src/safe_sync.rs` | ext `.rs` | 14041 bytes
2308. `error/src/structured_errors.rs` | ext `.rs` | 25974 bytes
2309. `error/tests/property_tests.rs` | ext `.rs` | 18173 bytes
2310. `examples/blog/dx.config.json` | ext `.json` | 185 bytes
2311. `examples/blog/README.md` | ext `.md` | 1884 bytes
2312. `examples/blog/src/App.tsx` | ext `.tsx` | 627 bytes
2313. `examples/blog/src/components/Header.tsx` | ext `.tsx` | 1208 bytes
2314. `examples/blog/src/components/Layout.tsx` | ext `.tsx` | 455 bytes
2315. `examples/blog/src/components/PostCard.tsx` | ext `.tsx` | 1084 bytes
2316. `examples/blog/src/data/posts.ts` | ext `.ts` | 3194 bytes
2317. `examples/blog/src/pages/About.tsx` | ext `.tsx` | 1297 bytes
2318. `examples/blog/src/pages/Home.tsx` | ext `.tsx` | 1888 bytes
2319. `examples/blog/src/pages/NotFound.tsx` | ext `.tsx` | 347 bytes
2320. `examples/blog/src/pages/Post.tsx` | ext `.tsx` | 4182 bytes
2321. `examples/conversion-proof/.dx/forge/docs/dx-www-conversion-proof.md` | ext `.md` | 1415 bytes
2322. `examples/conversion-proof/.dx/forge/receipts/2026-05-21-convex-to-backend.json` | ext `.json` | 2495 bytes
2323. `examples/conversion-proof/.dx/forge/receipts/2026-05-21-dx-landing-to-index.json` | ext `.json` | 1429 bytes
2324. `examples/conversion-proof/.dx/forge/receipts/2026-05-21-shadcn-ui-to-ui.json` | ext `.json` | 2017 bytes
2325. `examples/conversion-proof/.dx/forge/receipts/2026-05-21-supabase-to-database.json` | ext `.json` | 2008 bytes
2326. `examples/conversion-proof/.dx/forge/source-manifest.json` | ext `.json` | 6139 bytes
2327. `examples/conversion-proof/.dx/vercel-landing/.gitignore` | ext `.gitignore` | 9 bytes
2328. `examples/conversion-proof/.dx/vercel-landing/.vercel/project.json` | ext `.json` | 119 bytes
2329. `examples/conversion-proof/.dx/vercel-landing/.vercel/README.txt` | ext `.txt` | 520 bytes
2330. `examples/conversion-proof/.dx/vercel-landing/favicon.svg` | ext `.svg` | 410 bytes
2331. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/android.svg` | ext `.svg` | 1893 bytes
2332. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/apple.svg` | ext `.svg` | 678 bytes
2333. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/apple-dark.svg` | ext `.svg` | 690 bytes
2334. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/chrome.svg` | ext `.svg` | 1183 bytes
2335. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/firefox.svg` | ext `.svg` | 12170 bytes
2336. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/linux.svg` | ext `.svg` | 11498 bytes
2337. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/rust.svg` | ext `.svg` | 5425 bytes
2338. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/rust-dark.svg` | ext `.svg` | 5425 bytes
2339. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/vercel.svg` | ext `.svg` | 169 bytes
2340. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/vercel-dark.svg` | ext `.svg` | 169 bytes
2341. `examples/conversion-proof/.dx/vercel-landing/icons/platform-svgl/windows.svg` | ext `.svg` | 297 bytes
2342. `examples/conversion-proof/.dx/vercel-landing/index.html` | ext `.html` | 50454 bytes
2343. `examples/conversion-proof/.dx/vercel-landing/launch-runtime.js` | ext `.js` | 45975 bytes
2344. `examples/conversion-proof/.dx/vercel-landing/preview-manifest.json` | ext `.json` | 12748 bytes
2345. `examples/conversion-proof/.dx/vercel-landing/styles/conversion-proof.css` | ext `.css` | 2870 bytes
2346. `examples/conversion-proof/.dx/vercel-landing/styles/dx-landing.css` | ext `.css` | 31526 bytes
2347. `examples/conversion-proof/.dx/vercel-landing/styles/launch-runtime.css` | ext `.css` | 8708 bytes
2348. `examples/conversion-proof/.dx/vercel-landing/thumbnails/amber.png` | ext `.png` | 1997498 bytes
2349. `examples/conversion-proof/.dx/vercel-landing/thumbnails/blue.png` | ext `.png` | 1820586 bytes
2350. `examples/conversion-proof/.dx/vercel-landing/thumbnails/cyan.png` | ext `.png` | 1649536 bytes
2351. `examples/conversion-proof/.dx/vercel-landing/thumbnails/emerald.png` | ext `.png` | 2028704 bytes
2352. `examples/conversion-proof/.dx/vercel-landing/thumbnails/fuchsia.png` | ext `.png` | 2124971 bytes
2353. `examples/conversion-proof/.dx/vercel-landing/thumbnails/green.png` | ext `.png` | 2030618 bytes
2354. `examples/conversion-proof/.dx/vercel-landing/thumbnails/green-variant.png` | ext `.png` | 1881470 bytes
2355. `examples/conversion-proof/.dx/vercel-landing/thumbnails/indigo.png` | ext `.png` | 1939092 bytes
2356. `examples/conversion-proof/.dx/vercel-landing/thumbnails/lime.png` | ext `.png` | 2497220 bytes
2357. `examples/conversion-proof/.dx/vercel-landing/thumbnails/orange.png` | ext `.png` | 1940167 bytes
2358. `examples/conversion-proof/.dx/vercel-landing/thumbnails/pink.png` | ext `.png` | 1911328 bytes
2359. `examples/conversion-proof/.dx/vercel-landing/thumbnails/purple.png` | ext `.png` | 1965442 bytes
2360. `examples/conversion-proof/.dx/vercel-landing/thumbnails/rainbow.png` | ext `.png` | 2061107 bytes
2361. `examples/conversion-proof/.dx/vercel-landing/thumbnails/red.png` | ext `.png` | 2235880 bytes
2362. `examples/conversion-proof/.dx/vercel-landing/thumbnails/rose.png` | ext `.png` | 2127207 bytes
2363. `examples/conversion-proof/.dx/vercel-landing/thumbnails/sky.png` | ext `.png` | 1333535 bytes
2364. `examples/conversion-proof/.dx/vercel-landing/thumbnails/teal.png` | ext `.png` | 1829741 bytes
2365. `examples/conversion-proof/.dx/vercel-landing/thumbnails/variant-1.png` | ext `.png` | 2214806 bytes
2366. `examples/conversion-proof/.dx/vercel-landing/thumbnails/variant-2.png` | ext `.png` | 1664310 bytes
2367. `examples/conversion-proof/.dx/vercel-landing/thumbnails/violet.png` | ext `.png` | 2315052 bytes
2368. `examples/conversion-proof/.dx/vercel-landing/thumbnails/yellow.png` | ext `.png` | 2024458 bytes
2369. `examples/conversion-proof/.dx/vercel-landing/vercel.json` | ext `.json` | 217 bytes
2370. `examples/conversion-proof/app/api/ai/chat/route.ts` | ext `.ts` | 298 bytes
2371. `examples/conversion-proof/app/api/auth/session/route.ts` | ext `.ts` | 214 bytes
2372. `examples/conversion-proof/app/api/checkout/route.ts` | ext `.ts` | 319 bytes
2373. `examples/conversion-proof/app/api/trpc/health/route.ts` | ext `.ts` | 367 bytes
2374. `examples/conversion-proof/CHANGELOG.md` | ext `.md` | 6823 bytes
2375. `examples/conversion-proof/components/ConversionRouteHeader.tsx` | ext `.tsx` | 687 bytes
2376. `examples/conversion-proof/components/RuntimeBoundaryPanel.tsx` | ext `.tsx` | 773 bytes
2377. `examples/conversion-proof/components/SourceSurfaceTable.tsx` | ext `.tsx` | 576 bytes
2378. `examples/conversion-proof/DX.md` | ext `.md` | 4739 bytes
2379. `examples/conversion-proof/dx-www-3000.err.log` | ext `.log` | 153 bytes
2380. `examples/conversion-proof/dx-www-3000.out.log` | ext `.log` | 0 bytes
2381. `examples/conversion-proof/forge/acceptance/fixtures/blocked-rendered-proof.sample.json` | ext `.json` | 1691 bytes
2382. `examples/conversion-proof/forge/acceptance/no-runtime-route-acceptance.json` | ext `.json` | 11326 bytes
2383. `examples/conversion-proof/forge/acceptance/prepare-rendered-proof-import.ts` | ext `.ts` | 6258 bytes
2384. `examples/conversion-proof/forge/acceptance/rendered-proof-evidence.schema.json` | ext `.json` | 8288 bytes
2385. `examples/conversion-proof/forge/acceptance/rendered-proof-evidence-authoring-guide.json` | ext `.json` | 6164 bytes
2386. `examples/conversion-proof/forge/acceptance/rendered-proof-import-plan.json` | ext `.json` | 4106 bytes
2387. `examples/conversion-proof/forge/acceptance/rendered-proof-runtime-approval-request.json` | ext `.json` | 4570 bytes
2388. `examples/conversion-proof/forge/acceptance/request-rendered-proof-runtime-approval.ts` | ext `.ts` | 2991 bytes
2389. `examples/conversion-proof/forge/acceptance/review-rendered-proof-completeness.ts` | ext `.ts` | 3851 bytes
2390. `examples/conversion-proof/forge/acceptance/summarize-rendered-proof-evidence-requirements.ts` | ext `.ts` | 2851 bytes
2391. `examples/conversion-proof/forge/acceptance/validate-rendered-proof-evidence.ts` | ext `.ts` | 5574 bytes
2392. `examples/conversion-proof/forge/conversion-manifests/convex-backend.json` | ext `.json` | 2894 bytes
2393. `examples/conversion-proof/forge/conversion-manifests/dx-landing.json` | ext `.json` | 953 bytes
2394. `examples/conversion-proof/forge/conversion-manifests/shadcn-ui.json` | ext `.json` | 2701 bytes
2395. `examples/conversion-proof/forge/conversion-manifests/supabase.json` | ext `.json` | 2463 bytes
2396. `examples/conversion-proof/forge/primitives/badge.ts` | ext `.ts` | 1167 bytes
2397. `examples/conversion-proof/forge/primitives/button.ts` | ext `.ts` | 1694 bytes
2398. `examples/conversion-proof/forge/primitives/card.ts` | ext `.ts` | 1001 bytes
2399. `examples/conversion-proof/forge/primitives/class-merge.ts` | ext `.ts` | 615 bytes
2400. `examples/conversion-proof/forge/primitives/dialog.ts` | ext `.ts` | 1121 bytes
2401. `examples/conversion-proof/forge/primitives/dropdown.ts` | ext `.ts` | 1097 bytes
2402. `examples/conversion-proof/forge/primitives/index.ts` | ext `.ts` | 805 bytes
2403. `examples/conversion-proof/forge/primitives/input.ts` | ext `.ts` | 1317 bytes
2404. `examples/conversion-proof/forge/primitives/metadata.json` | ext `.json` | 898 bytes
2405. `examples/conversion-proof/forge/primitives/sidebar.ts` | ext `.ts` | 1231 bytes
2406. `examples/conversion-proof/forge/primitives/slot.ts` | ext `.ts` | 701 bytes
2407. `examples/conversion-proof/forge/primitives/table.ts` | ext `.ts` | 1000 bytes
2408. `examples/conversion-proof/forge/primitives/tabs.ts` | ext `.ts` | 1135 bytes
2409. `examples/conversion-proof/forge/primitives/theme-provider.ts` | ext `.ts` | 798 bytes
2410. `examples/conversion-proof/forge/primitives/types.ts` | ext `.ts` | 964 bytes
2411. `examples/conversion-proof/forge/route-discovery/conversion-routes.json` | ext `.json` | 11142 bytes
2412. `examples/conversion-proof/forge/shims/launch-runtime-boundaries.json` | ext `.json` | 2219 bytes
2413. `examples/conversion-proof/forge/shims/README.md` | ext `.md` | 486 bytes
2414. `examples/conversion-proof/forge/shims/runtime-boundaries.ts` | ext `.ts` | 2457 bytes
2415. `examples/conversion-proof/forge/shims/test-adapters.ts` | ext `.ts` | 944 bytes
2416. `examples/conversion-proof/forge/source-surfaces/convex-backend.json` | ext `.json` | 4439 bytes
2417. `examples/conversion-proof/forge/source-surfaces/dx-landing.json` | ext `.json` | 2234 bytes
2418. `examples/conversion-proof/forge/source-surfaces/shadcn-ui.json` | ext `.json` | 4314 bytes
2419. `examples/conversion-proof/forge/source-surfaces/supabase.json` | ext `.json` | 4123 bytes
2420. `examples/conversion-proof/forge/visual-audits/convex-backend.json` | ext `.json` | 3379 bytes
2421. `examples/conversion-proof/forge/visual-audits/dx-landing.json` | ext `.json` | 1315 bytes
2422. `examples/conversion-proof/forge/visual-audits/shadcn-ui.json` | ext `.json` | 3090 bytes
2423. `examples/conversion-proof/forge/visual-audits/supabase.json` | ext `.json` | 3144 bytes
2424. `examples/conversion-proof/notices/convex-backend/LICENSE.md` | ext `.md` | 3861 bytes
2425. `examples/conversion-proof/notices/shadcn-ui/LICENSE.md` | ext `.md` | 1084 bytes
2426. `examples/conversion-proof/notices/supabase/LICENSE` | ext `<none>` | 11539 bytes
2427. `examples/conversion-proof/pages/_layout.html` | ext `.html` | 32 bytes
2428. `examples/conversion-proof/pages/automations.html` | ext `.html` | 2301 bytes
2429. `examples/conversion-proof/pages/backend.html` | ext `.html` | 2827 bytes
2430. `examples/conversion-proof/pages/database.html` | ext `.html` | 2548 bytes
2431. `examples/conversion-proof/pages/index.html` | ext `.html` | 43707 bytes
2432. `examples/conversion-proof/pages/launch.html` | ext `.html` | 45755 bytes
2433. `examples/conversion-proof/pages/ui.html` | ext `.html` | 2674 bytes
2434. `examples/conversion-proof/public/favicon.svg` | ext `.svg` | 410 bytes
2435. `examples/conversion-proof/public/icons/platform-svgl/android.svg` | ext `.svg` | 1893 bytes
2436. `examples/conversion-proof/public/icons/platform-svgl/apple.svg` | ext `.svg` | 678 bytes
2437. `examples/conversion-proof/public/icons/platform-svgl/apple-dark.svg` | ext `.svg` | 690 bytes
2438. `examples/conversion-proof/public/icons/platform-svgl/chrome.svg` | ext `.svg` | 1183 bytes
2439. `examples/conversion-proof/public/icons/platform-svgl/firefox.svg` | ext `.svg` | 12170 bytes
2440. `examples/conversion-proof/public/icons/platform-svgl/linux.svg` | ext `.svg` | 11498 bytes
2441. `examples/conversion-proof/public/icons/platform-svgl/rust.svg` | ext `.svg` | 5425 bytes
2442. `examples/conversion-proof/public/icons/platform-svgl/rust-dark.svg` | ext `.svg` | 5425 bytes
2443. `examples/conversion-proof/public/icons/platform-svgl/vercel.svg` | ext `.svg` | 169 bytes
2444. `examples/conversion-proof/public/icons/platform-svgl/vercel-dark.svg` | ext `.svg` | 169 bytes
2445. `examples/conversion-proof/public/icons/platform-svgl/windows.svg` | ext `.svg` | 297 bytes
2446. `examples/conversion-proof/public/launch-runtime.js` | ext `.js` | 48911 bytes
2447. `examples/conversion-proof/public/preview-manifest.json` | ext `.json` | 15548 bytes
2448. `examples/conversion-proof/public/thumbnails/amber.png` | ext `.png` | 1997498 bytes
2449. `examples/conversion-proof/public/thumbnails/blue.png` | ext `.png` | 1820586 bytes
2450. `examples/conversion-proof/public/thumbnails/cyan.png` | ext `.png` | 1649536 bytes
2451. `examples/conversion-proof/public/thumbnails/emerald.png` | ext `.png` | 2028704 bytes
2452. `examples/conversion-proof/public/thumbnails/fuchsia.png` | ext `.png` | 2124971 bytes
2453. `examples/conversion-proof/public/thumbnails/green.png` | ext `.png` | 2030618 bytes
2454. `examples/conversion-proof/public/thumbnails/green-variant.png` | ext `.png` | 1881470 bytes
2455. `examples/conversion-proof/public/thumbnails/indigo.png` | ext `.png` | 1939092 bytes
2456. `examples/conversion-proof/public/thumbnails/lime.png` | ext `.png` | 2497220 bytes
2457. `examples/conversion-proof/public/thumbnails/orange.png` | ext `.png` | 1940167 bytes
2458. `examples/conversion-proof/public/thumbnails/pink.png` | ext `.png` | 1911328 bytes
2459. `examples/conversion-proof/public/thumbnails/purple.png` | ext `.png` | 1965442 bytes
2460. `examples/conversion-proof/public/thumbnails/rainbow.png` | ext `.png` | 2061107 bytes
2461. `examples/conversion-proof/public/thumbnails/red.png` | ext `.png` | 2235880 bytes
2462. `examples/conversion-proof/public/thumbnails/rose.png` | ext `.png` | 2127207 bytes
2463. `examples/conversion-proof/public/thumbnails/sky.png` | ext `.png` | 1333535 bytes
2464. `examples/conversion-proof/public/thumbnails/teal.png` | ext `.png` | 1829741 bytes
2465. `examples/conversion-proof/public/thumbnails/variant-1.png` | ext `.png` | 2214806 bytes
2466. `examples/conversion-proof/public/thumbnails/variant-2.png` | ext `.png` | 1664310 bytes
2467. `examples/conversion-proof/public/thumbnails/violet.png` | ext `.png` | 2315052 bytes
2468. `examples/conversion-proof/public/thumbnails/yellow.png` | ext `.png` | 2024458 bytes
2469. `examples/conversion-proof/README.md` | ext `.md` | 6602 bytes
2470. `examples/conversion-proof/styles/conversion-proof.css` | ext `.css` | 2870 bytes
2471. `examples/conversion-proof/styles/dx-landing.css` | ext `.css` | 31526 bytes
2472. `examples/conversion-proof/styles/launch-runtime.css` | ext `.css` | 8711 bytes
2473. `examples/conversion-proof/TODO.md` | ext `.md` | 7622 bytes
2474. `examples/conversion-proof/tools/export-vercel-landing.ts` | ext `.ts` | 7214 bytes
2475. `examples/dashboard/dx.config.json` | ext `.json` | 211 bytes
2476. `examples/dashboard/README.md` | ext `.md` | 11682 bytes
2477. `examples/dashboard/src/App.tsx` | ext `.tsx` | 783 bytes
2478. `examples/dashboard/src/components/AiLaunchAssistant.tsx` | ext `.tsx` | 4045 bytes
2479. `examples/dashboard/src/components/AuthProvider.tsx` | ext `.tsx` | 2515 bytes
2480. `examples/dashboard/src/components/AutomationWorkflowPanel.tsx` | ext `.tsx` | 5307 bytes
2481. `examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx` | ext `.tsx` | 8298 bytes
2482. `examples/dashboard/src/components/DrizzleDashboardWorkflow.tsx` | ext `.tsx` | 5494 bytes
2483. `examples/dashboard/src/components/FumadocsDocsWorkflow.tsx` | ext `.tsx` | 5438 bytes
2484. `examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx` | ext `.tsx` | 6499 bytes
2485. `examples/dashboard/src/components/MotionDashboardWorkflow.tsx` | ext `.tsx` | 10546 bytes
2486. `examples/dashboard/src/components/ProtectedRoute.tsx` | ext `.tsx` | 1025 bytes
2487. `examples/dashboard/src/components/QueryDashboardWorkflow.tsx` | ext `.tsx` | 7532 bytes
2488. `examples/dashboard/src/components/ShadcnDashboardControls.tsx` | ext `.tsx` | 9970 bytes
2489. `examples/dashboard/src/components/Sidebar.tsx` | ext `.tsx` | 2243 bytes
2490. `examples/dashboard/src/components/StripePlanCheckout.tsx` | ext `.tsx` | 8276 bytes
2491. `examples/dashboard/src/components/TrpcDashboardWorkflow.tsx` | ext `.tsx` | 4655 bytes
2492. `examples/dashboard/src/components/WasmBindgenWorkflow.tsx` | ext `.tsx` | 4672 bytes
2493. `examples/dashboard/src/components/ZodSettingsValidator.tsx` | ext `.tsx` | 12384 bytes
2494. `examples/dashboard/src/components/ZustandSettingsPanel.tsx` | ext `.tsx` | 5328 bytes
2495. `examples/dashboard/src/lib/aiLaunchAssistant.ts` | ext `.ts` | 1259 bytes
2496. `examples/dashboard/src/lib/dashboardSettingsStore.ts` | ext `.ts` | 2813 bytes
2497. `examples/dashboard/src/lib/drizzleDashboard.ts` | ext `.ts` | 4719 bytes
2498. `examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts` | ext `.ts` | 7191 bytes
2499. `examples/dashboard/src/lib/forge/state/zustand.ts` | ext `.ts` | 10642 bytes
2500. `examples/dashboard/src/lib/forge/validation/zod/dashboard-settings.ts` | ext `.ts` | 4275 bytes
2501. `examples/dashboard/src/lib/fumadocsDocsWorkflow.ts` | ext `.ts` | 4124 bytes
2502. `examples/dashboard/src/lib/instantdbDashboard.ts` | ext `.ts` | 11850 bytes
2503. `examples/dashboard/src/lib/motionDashboardWorkflow.ts` | ext `.ts` | 14781 bytes
2504. `examples/dashboard/src/lib/n8nAutomationBridge.ts` | ext `.ts` | 8394 bytes
2505. `examples/dashboard/src/lib/queryDashboardWorkflow.ts` | ext `.ts` | 6524 bytes
2506. `examples/dashboard/src/lib/shadcnDashboardControls.ts` | ext `.ts` | 6630 bytes
2507. `examples/dashboard/src/lib/stripePlanCheckout.ts` | ext `.ts` | 5120 bytes
2508. `examples/dashboard/src/lib/trpcDashboardWorkflow.ts` | ext `.ts` | 5995 bytes
2509. `examples/dashboard/src/lib/wasmBindgenDashboard.ts` | ext `.ts` | 6920 bytes
2510. `examples/dashboard/src/lib/zodDashboardSettings.ts` | ext `.ts` | 4179 bytes
2511. `examples/dashboard/src/pages/Admin.tsx` | ext `.tsx` | 3894 bytes
2512. `examples/dashboard/src/pages/Dashboard.tsx` | ext `.tsx` | 8329 bytes
2513. `examples/dashboard/src/pages/Login.tsx` | ext `.tsx` | 3373 bytes
2514. `examples/dashboard/src/pages/Settings.tsx` | ext `.tsx` | 3441 bytes
2515. `examples/onboard/.dx/forge/docs/www-minimal-starter.md` | ext `.md` | 2625 bytes
2516. `examples/onboard/.dx/forge/package-status.json` | ext `.json` | 2729 bytes
2517. `examples/onboard/.dx/forge/package-status.sr` | ext `.sr` | 89 bytes
2518. `examples/onboard/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json` | ext `.json` | 27765 bytes
2519. `examples/onboard/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json` | ext `.json` | 18656 bytes
2520. `examples/onboard/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json` | ext `.json` | 16298 bytes
2521. `examples/onboard/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json` | ext `.json` | 4421 bytes
2522. `examples/onboard/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json` | ext `.json` | 16690 bytes
2523. `examples/onboard/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json` | ext `.json` | 12580 bytes
2524. `examples/onboard/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json` | ext `.json` | 13954 bytes
2525. `examples/onboard/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json` | ext `.json` | 9330 bytes
2526. `examples/onboard/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json` | ext `.json` | 16104 bytes
2527. `examples/onboard/.dx/forge/receipts/20260527T211349343763800Z-www-minimal-starter.json` | ext `.json` | 6661 bytes
2528. `examples/onboard/.dx/forge/receipts/auth-better-auth.json` | ext `.json` | 17516 bytes
2529. `examples/onboard/.dx/forge/receipts/packages/www-minimal-starter.json` | ext `.json` | 2267 bytes
2530. `examples/onboard/.dx/forge/source-manifest.json` | ext `.json` | 2891 bytes
2531. `examples/onboard/.dx/forge/template-core-sources.json` | ext `.json` | 3472 bytes
2532. `examples/onboard/.dx/forge/template-manifest.json` | ext `.json` | 2614 bytes
2533. `examples/onboard/.dx/icons/sync.sr` | ext `.sr` | 183 bytes
2534. `examples/onboard/.dx/imports/import-map.json` | ext `.json` | 1661 bytes
2535. `examples/onboard/.dx/imports/sync.sr` | ext `.sr` | 267 bytes
2536. `examples/onboard/.dx/receipts/graph/consumer-snapshot.json` | ext `.json` | 4857 bytes
2537. `examples/onboard/.dx/receipts/graph/latest.json` | ext `.json` | 39140 bytes
2538. `examples/onboard/.dx/receipts/icons/sync.json` | ext `.json` | 889 bytes
2539. `examples/onboard/.dx/receipts/imports/sync.json` | ext `.json` | 475 bytes
2540. `examples/onboard/.dx/receipts/run/dev-extension-toolchain.json` | ext `.json` | 896 bytes
2541. `examples/onboard/.dx/receipts/style/build.json` | ext `.json` | 292387 bytes
2542. `examples/onboard/.dx/run/onboard-dev-20260528031756.stderr.log` | ext `.log` | 150 bytes
2543. `examples/onboard/.dx/run/onboard-dev-20260528031756.stdout.log` | ext `.log` | 0 bytes
2544. `examples/onboard/.dx/serializer/dx.machine` | ext `.machine` | 832 bytes
2545. `examples/onboard/.dx/serializer/icons-sync.machine` | ext `.machine` | 357 bytes
2546. `examples/onboard/.dx/serializer/imports-sync.machine` | ext `.machine` | 442 bytes
2547. `examples/onboard/.dx/serializer/style-build.machine` | ext `.machine` | 528 bytes
2548. `examples/onboard/.dx/style/build.sr` | ext `.sr` | 330 bytes
2549. `examples/onboard/.gitignore` | ext `.gitignore` | 32 bytes
2550. `examples/onboard/ai-chat-status.tsx` | ext `.tsx` | 14009 bytes
2551. `examples/onboard/app/layout.tsx` | ext `.tsx` | 381 bytes
2552. `examples/onboard/app/page.tsx` | ext `.tsx` | 1626 bytes
2553. `examples/onboard/auth-session-status.tsx` | ext `.tsx` | 33469 bytes
2554. `examples/onboard/automation-mission-summary.tsx` | ext `.tsx` | 4295 bytes
2555. `examples/onboard/automations/automations-metadata.ts` | ext `.ts` | 17502 bytes
2556. `examples/onboard/automations-status.tsx` | ext `.tsx` | 22136 bytes
2557. `examples/onboard/components/auto-imports.ts` | ext `.ts` | 1295 bytes
2558. `examples/onboard/components/friday.tsx` | ext `.tsx` | 13116 bytes
2559. `examples/onboard/components/hello-glow.tsx` | ext `.tsx` | 2429 bytes
2560. `examples/onboard/components/icons/icon.tsx` | ext `.tsx` | 458 bytes
2561. `examples/onboard/components/ui/button.tsx` | ext `.tsx` | 2365 bytes
2562. `examples/onboard/data-status.tsx` | ext `.tsx` | 10328 bytes
2563. `examples/onboard/docs-status.tsx` | ext `.tsx` | 11371 bytes
2564. `examples/onboard/drizzle-query-proof.tsx` | ext `.tsx` | 10272 bytes
2565. `examples/onboard/dx` | ext `<none>` | 497 bytes
2566. `examples/onboard/dx-check-style-evidence-read-model.ts` | ext `.ts` | 7610 bytes
2567. `examples/onboard/dx-studio-edit-contract.ts` | ext `.ts` | 37856 bytes
2568. `examples/onboard/forge-golden-path-contract.ts` | ext `.ts` | 9288 bytes
2569. `examples/onboard/forge-golden-path-panel.tsx` | ext `.tsx` | 3413 bytes
2570. `examples/onboard/forge-package-status.ts` | ext `.ts` | 4755 bytes
2571. `examples/onboard/forge-package-status-read-model.ts` | ext `.ts` | 333437 bytes
2572. `examples/onboard/forge-remote-head-health-contract.ts` | ext `.ts` | 3107 bytes
2573. `examples/onboard/forge-remote-head-health-panel.tsx` | ext `.tsx` | 3328 bytes
2574. `examples/onboard/forge-safety-archive-contract.ts` | ext `.ts` | 3980 bytes
2575. `examples/onboard/forge-safety-archive-panel.tsx` | ext `.tsx` | 3914 bytes
2576. `examples/onboard/forge-safety-archive-runbook.ts` | ext `.ts` | 5835 bytes
2577. `examples/onboard/framework-completeness.ts` | ext `.ts` | 17250 bytes
2578. `examples/onboard/icon-status.tsx` | ext `.tsx` | 229 bytes
2579. `examples/onboard/instantdb-status.tsx` | ext `.tsx` | 19618 bytes
2580. `examples/onboard/launch-scene.tsx` | ext `.tsx` | 23224 bytes
2581. `examples/onboard/lib/utils.ts` | ext `.ts` | 761 bytes
2582. `examples/onboard/lib/zen-motion.tsx` | ext `.tsx` | 8676 bytes
2583. `examples/onboard/motion-interaction-proof.tsx` | ext `.tsx` | 14708 bytes
2584. `examples/onboard/next-intl-dashboard-locale.tsx` | ext `.tsx` | 10392 bytes
2585. `examples/onboard/next-intl-dashboard-locale-contract.ts` | ext `.ts` | 6812 bytes
2586. `examples/onboard/next-intl-status.tsx` | ext `.tsx` | 2858 bytes
2587. `examples/onboard/package-catalog.ts` | ext `.ts` | 108614 bytes
2588. `examples/onboard/payments-status.tsx` | ext `.tsx` | 20578 bytes
2589. `examples/onboard/preview-style-evidence-read-model.ts` | ext `.ts` | 4954 bytes
2590. `examples/onboard/preview-style-package-ownership-read-model.ts` | ext `.ts` | 8486 bytes
2591. `examples/onboard/preview-style-package-panel-read-model.ts` | ext `.ts` | 12256 bytes
2592. `examples/onboard/public/favicon.svg` | ext `.svg` | 572 bytes
2593. `examples/onboard/public/icon.svg` | ext `.svg` | 728 bytes
2594. `examples/onboard/public/logo.svg` | ext `.svg` | 910 bytes
2595. `examples/onboard/query-cache-status.tsx` | ext `.tsx` | 33581 bytes
2596. `examples/onboard/query-dashboard-read-model.ts` | ext `.ts` | 2192 bytes
2597. `examples/onboard/react-markdown-preview.tsx` | ext `.tsx` | 553 bytes
2598. `examples/onboard/README.md` | ext `.md` | 121 bytes
2599. `examples/onboard/scene/bounds-report.ts` | ext `.ts` | 5163 bytes
2600. `examples/onboard/scene/capability-report.ts` | ext `.ts` | 3623 bytes
2601. `examples/onboard/scene/dashboard-controls.ts` | ext `.ts` | 1339 bytes
2602. `examples/onboard/scene/dashboard-workflow.ts` | ext `.ts` | 6741 bytes
2603. `examples/onboard/scene/frame-sample.ts` | ext `.ts` | 2955 bytes
2604. `examples/onboard/scene/index.ts` | ext `.ts` | 4710 bytes
2605. `examples/onboard/scene/interaction.ts` | ext `.ts` | 5719 bytes
2606. `examples/onboard/scene/metadata.ts` | ext `.ts` | 8867 bytes
2607. `examples/onboard/scene/performance-monitor.ts` | ext `.ts` | 3554 bytes
2608. `examples/onboard/scene/preset.ts` | ext `.ts` | 4132 bytes
2609. `examples/onboard/scene/preview-readiness.ts` | ext `.ts` | 3920 bytes
2610. `examples/onboard/scene/r3f-renderer-adapter.ts` | ext `.ts` | 4490 bytes
2611. `examples/onboard/scene/raycast-report.ts` | ext `.ts` | 3061 bytes
2612. `examples/onboard/scene/README.md` | ext `.md` | 14298 bytes
2613. `examples/onboard/scene/renderer-handoff.ts` | ext `.ts` | 1357 bytes
2614. `examples/onboard/scene/types.ts` | ext `.ts` | 9108 bytes
2615. `examples/onboard/scene/viewport-report.ts` | ext `.ts` | 3907 bytes
2616. `examples/onboard/scene/webgl-runtime.ts` | ext `.ts` | 25356 bytes
2617. `examples/onboard/shadcn-dashboard-controls.tsx` | ext `.tsx` | 14750 bytes
2618. `examples/onboard/shadcn-dashboard-controls-contract.tsx` | ext `.tsx` | 5065 bytes
2619. `examples/onboard/state-zustand-counter.tsx` | ext `.tsx` | 5723 bytes
2620. `examples/onboard/state-zustand-dashboard.tsx` | ext `.tsx` | 7331 bytes
2621. `examples/onboard/styles/generated.css` | ext `.css` | 10952 bytes
2622. `examples/onboard/styles/globals.css` | ext `.css` | 132914 bytes
2623. `examples/onboard/styles/theme.css` | ext `.css` | 1520 bytes
2624. `examples/onboard/supabase-profile-workflow.tsx` | ext `.tsx` | 7140 bytes
2625. `examples/onboard/supabase-profile-workflow-state.ts` | ext `.ts` | 4678 bytes
2626. `examples/onboard/template-dashboard-nav.tsx` | ext `.tsx` | 2943 bytes
2627. `examples/onboard/template-lead-form.tsx` | ext `.tsx` | 2954 bytes
2628. `examples/onboard/template-route-contract.ts` | ext `.ts` | 57023 bytes
2629. `examples/onboard/template-shell.tsx` | ext `.tsx` | 130631 bytes
2630. `examples/onboard/template-shell-evidence-loader.ts` | ext `.ts` | 688 bytes
2631. `examples/onboard/template-shell-style-evidence-drift.ts` | ext `.ts` | 657 bytes
2632. `examples/onboard/template-surface-registry.ts` | ext `.ts` | 17582 bytes
2633. `examples/onboard/trpc-launch-contract.ts` | ext `.ts` | 1615 bytes
2634. `examples/onboard/trpc-launch-health.tsx` | ext `.tsx` | 5572 bytes
2635. `examples/onboard/wasm-interop-status.tsx` | ext `.tsx` | 6918 bytes
2636. `examples/onboard/zod-dashboard-settings.tsx` | ext `.tsx` | 8017 bytes
2637. `examples/onboard/zod-validation-status.tsx` | ext `.tsx` | 10790 bytes
2638. `examples/template/.dx/forge/package-status.json` | ext `.json` | 344263 bytes
2639. `examples/template/.dx/icons/sync.sr` | ext `.sr` | 183 bytes
2640. `examples/template/.dx/imports/import-map.json` | ext `.json` | 1101 bytes
2641. `examples/template/.dx/imports/sync.sr` | ext `.sr` | 267 bytes
2642. `examples/template/.dx/receipts/style/build.json` | ext `.json` | 224936 bytes
2643. `examples/template/.dx/style/build.sr` | ext `.sr` | 328 bytes
2644. `examples/template/.gitignore` | ext `.gitignore` | 30 bytes
2645. `examples/template/app/layout.tsx` | ext `.tsx` | 381 bytes
2646. `examples/template/app/page.tsx` | ext `.tsx` | 651 bytes
2647. `examples/template/components/auto-imports.ts` | ext `.ts` | 960 bytes
2648. `examples/template/components/icons/icon.tsx` | ext `.tsx` | 458 bytes
2649. `examples/template/dx` | ext `<none>` | 349 bytes
2650. `examples/template/package.json` | ext `.json` | 183 bytes
2651. `examples/template/public/favicon.svg` | ext `.svg` | 572 bytes
2652. `examples/template/public/icon.svg` | ext `.svg` | 728 bytes
2653. `examples/template/public/logo.svg` | ext `.svg` | 910 bytes
2654. `examples/template/README.md` | ext `.md` | 121 bytes
2655. `examples/template/styles/generated.css` | ext `.css` | 7011 bytes
2656. `examples/template/styles/globals.css` | ext `.css` | 2086 bytes
2657. `examples/template/styles/theme.css` | ext `.css` | 233 bytes
2658. `examples/template/tsconfig.json` | ext `.json` | 276 bytes
2659. `examples/todo-app/dx.config.json` | ext `.json` | 187 bytes
2660. `examples/todo-app/README.md` | ext `.md` | 436 bytes
2661. `examples/todo-app/src/AddTodo.tsx` | ext `.tsx` | 1673 bytes
2662. `examples/todo-app/src/App.tsx` | ext `.tsx` | 3501 bytes
2663. `examples/todo-app/src/Filter.tsx` | ext `.tsx` | 852 bytes
2664. `examples/todo-app/src/TodoItem.tsx` | ext `.tsx` | 974 bytes
2665. `examples/todo-app/src/TodoList.tsx` | ext `.tsx` | 805 bytes
2666. `fallback/Cargo.toml` | ext `.toml` | 365 bytes
2667. `fallback/CHANGELOG.md` | ext `.md` | 378 bytes
2668. `fallback/LICENSE` | ext `<none>` | 559 bytes
2669. `fallback/LICENSE-APACHE` | ext `<none>` | 5871 bytes
2670. `fallback/LICENSE-MIT` | ext `<none>` | 1093 bytes
2671. `fallback/README.md` | ext `.md` | 337 bytes
2672. `fallback/src/lib.rs` | ext `.rs` | 8765 bytes
2673. `flow/Cargo.lock` | ext `.lock` | 206935 bytes
2674. `flow/Cargo.toml` | ext `.toml` | 2792 bytes
2675. `flow/crates/flow-browser-core/Cargo.toml` | ext `.toml` | 375 bytes
2676. `flow/crates/flow-browser-core/README.md` | ext `.md` | 525 bytes
2677. `flow/crates/flow-browser-core/src/engine.rs` | ext `.rs` | 8956 bytes
2678. `flow/crates/flow-browser-core/src/lib.rs` | ext `.rs` | 66 bytes
2679. `flow/crates/flow-browser-core/src/types.rs` | ext `.rs` | 2575 bytes
2680. `flow/crates/forge/benches/ingest.rs` | ext `.rs` | 2461 bytes
2681. `flow/crates/forge/build.rs` | ext `.rs` | 62 bytes
2682. `flow/crates/forge/Cargo.lock` | ext `.lock` | 103443 bytes
2683. `flow/crates/forge/Cargo.toml` | ext `.toml` | 1564 bytes
2684. `flow/crates/forge/CHANGELOG.md` | ext `.md` | 6891 bytes
2685. `flow/crates/forge/docs/FORGE_99_PLAN.md` | ext `.md` | 2493 bytes
2686. `flow/crates/forge/docs/FORGE_STATUS.md` | ext `.md` | 14278 bytes
2687. `flow/crates/forge/README.md` | ext `.md` | 6120 bytes
2688. `flow/crates/forge/src/chunking/cdc.rs` | ext `.rs` | 1034 bytes
2689. `flow/crates/forge/src/chunking/mod.rs` | ext `.rs` | 620 bytes
2690. `flow/crates/forge/src/chunking/structure_aware/csp.rs` | ext `.rs` | 1021 bytes
2691. `flow/crates/forge/src/chunking/structure_aware/exr.rs` | ext `.rs` | 1189 bytes
2692. `flow/crates/forge/src/chunking/structure_aware/mod.rs` | ext `.rs` | 59 bytes
2693. `flow/crates/forge/src/chunking/structure_aware/mp4.rs` | ext `.rs` | 3055 bytes
2694. `flow/crates/forge/src/chunking/structure_aware/uasset.rs` | ext `.rs` | 1146 bytes
2695. `flow/crates/forge/src/cli/add.rs` | ext `.rs` | 5972 bytes
2696. `flow/crates/forge/src/cli/auth.rs` | ext `.rs` | 6576 bytes
2697. `flow/crates/forge/src/cli/checkout.rs` | ext `.rs` | 6880 bytes
2698. `flow/crates/forge/src/cli/checkout_archive.rs` | ext `.rs` | 10169 bytes
2699. `flow/crates/forge/src/cli/commit.rs` | ext `.rs` | 2891 bytes
2700. `flow/crates/forge/src/cli/diff.rs` | ext `.rs` | 4093 bytes
2701. `flow/crates/forge/src/cli/init.rs` | ext `.rs` | 272 bytes
2702. `flow/crates/forge/src/cli/jobs.rs` | ext `.rs` | 3126 bytes
2703. `flow/crates/forge/src/cli/log.rs` | ext `.rs` | 1542 bytes
2704. `flow/crates/forge/src/cli/mod.rs` | ext `.rs` | 282 bytes
2705. `flow/crates/forge/src/cli/package.rs` | ext `.rs` | 5890 bytes
2706. `flow/crates/forge/src/cli/pull.rs` | ext `.rs` | 22049 bytes
2707. `flow/crates/forge/src/cli/push.rs` | ext `.rs` | 31656 bytes
2708. `flow/crates/forge/src/cli/remote.rs` | ext `.rs` | 2722 bytes
2709. `flow/crates/forge/src/cli/status.rs` | ext `.rs` | 3665 bytes
2710. `flow/crates/forge/src/cli/sync.rs` | ext `.rs` | 5673 bytes
2711. `flow/crates/forge/src/cli/train_dict.rs` | ext `.rs` | 1494 bytes
2712. `flow/crates/forge/src/cli/vibe_demo.rs` | ext `.rs` | 3209 bytes
2713. `flow/crates/forge/src/core/chunk.rs` | ext `.rs` | 461 bytes
2714. `flow/crates/forge/src/core/hash.rs` | ext `.rs` | 1156 bytes
2715. `flow/crates/forge/src/core/manifest.rs` | ext `.rs` | 3164 bytes
2716. `flow/crates/forge/src/core/mod.rs` | ext `.rs` | 71 bytes
2717. `flow/crates/forge/src/core/repository.rs` | ext `.rs` | 10386 bytes
2718. `flow/crates/forge/src/db/metadata.rs` | ext `.rs` | 16107 bytes
2719. `flow/crates/forge/src/db/mod.rs` | ext `.rs` | 19 bytes
2720. `flow/crates/forge/src/jobs/mod.rs` | ext `.rs` | 10818 bytes
2721. `flow/crates/forge/src/lib.rs` | ext `.rs` | 3160 bytes
2722. `flow/crates/forge/src/main.rs` | ext `.rs` | 8891 bytes
2723. `flow/crates/forge/src/mirror/auth.rs` | ext `.rs` | 2078 bytes
2724. `flow/crates/forge/src/mirror/backends/bitbucket.rs` | ext `.rs` | 2355 bytes
2725. `flow/crates/forge/src/mirror/backends/dropbox.rs` | ext `.rs` | 5760 bytes
2726. `flow/crates/forge/src/mirror/backends/gdrive.rs` | ext `.rs` | 3193 bytes
2727. `flow/crates/forge/src/mirror/backends/github.rs` | ext `.rs` | 3386 bytes
2728. `flow/crates/forge/src/mirror/backends/gitlab.rs` | ext `.rs` | 3197 bytes
2729. `flow/crates/forge/src/mirror/backends/mega.rs` | ext `.rs` | 5327 bytes
2730. `flow/crates/forge/src/mirror/backends/pinterest.rs` | ext `.rs` | 2478 bytes
2731. `flow/crates/forge/src/mirror/backends/r2.rs` | ext `.rs` | 2199 bytes
2732. `flow/crates/forge/src/mirror/backends/sketchfab.rs` | ext `.rs` | 2118 bytes
2733. `flow/crates/forge/src/mirror/backends/soundcloud.rs` | ext `.rs` | 2425 bytes
2734. `flow/crates/forge/src/mirror/backends/youtube.rs` | ext `.rs` | 2888 bytes
2735. `flow/crates/forge/src/mirror/dispatcher.rs` | ext `.rs` | 2439 bytes
2736. `flow/crates/forge/src/mirror/media_type.rs` | ext `.rs` | 1491 bytes
2737. `flow/crates/forge/src/mirror/mod.rs` | ext `.rs` | 3483 bytes
2738. `flow/crates/forge/src/mirror/records.rs` | ext `.rs` | 14124 bytes
2739. `flow/crates/forge/src/packages.rs` | ext `.rs` | 35183 bytes
2740. `flow/crates/forge/src/recovery/mod.rs` | ext `.rs` | 5418 bytes
2741. `flow/crates/forge/src/store/cas.rs` | ext `.rs` | 3917 bytes
2742. `flow/crates/forge/src/store/compression.rs` | ext `.rs` | 1231 bytes
2743. `flow/crates/forge/src/store/mod.rs` | ext `.rs` | 51 bytes
2744. `flow/crates/forge/src/store/pack.rs` | ext `.rs` | 2064 bytes
2745. `flow/crates/forge/src/sync/mod.rs` | ext `.rs` | 63772 bytes
2746. `flow/crates/forge/src/transport/mod.rs` | ext `.rs` | 55 bytes
2747. `flow/crates/forge/src/transport/protocol.rs` | ext `.rs` | 8479 bytes
2748. `flow/crates/forge/src/transport/quic.rs` | ext `.rs` | 7042 bytes
2749. `flow/crates/forge/src/transport/repository.rs` | ext `.rs` | 23048 bytes
2750. `flow/crates/forge/src/util/human.rs` | ext `.rs` | 783 bytes
2751. `flow/crates/forge/src/util/ignore.rs` | ext `.rs` | 2510 bytes
2752. `flow/crates/forge/src/util/mod.rs` | ext `.rs` | 52 bytes
2753. `flow/crates/forge/src/util/progress.rs` | ext `.rs` | 664 bytes
2754. `flow/crates/forge/tests/integration.rs` | ext `.rs` | 25023 bytes
2755. `flow/crates/forge/TODO.md` | ext `.md` | 4281 bytes
2756. `flow/crates/serializer/.github/CODEOWNERS` | ext `<none>` | 237 bytes
2757. `flow/crates/serializer/.github/workflows/ci.yml` | ext `.yml` | 2128 bytes
2758. `flow/crates/serializer/.github/workflows/release.yml` | ext `.yml` | 1293 bytes
2759. `flow/crates/serializer/.gitignore` | ext `.gitignore` | 186 bytes
2760. `flow/crates/serializer/.gitmodules` | ext `.gitmodules` | 78 bytes
2761. `flow/crates/serializer/Cargo.toml` | ext `.toml` | 1834 bytes
2762. `flow/crates/serializer/CHANGELOG.md` | ext `.md` | 5785 bytes
2763. `flow/crates/serializer/CONTRIBUTING.md` | ext `.md` | 2646 bytes
2764. `flow/crates/serializer/docs/DX_SERIALIZER.md` | ext `.md` | 5632 bytes
2765. `flow/crates/serializer/docs/INTEGRATIONS.md` | ext `.md` | 4259 bytes
2766. `flow/crates/serializer/docs/PRODUCTION_READY.md` | ext `.md` | 4306 bytes
2767. `flow/crates/serializer/docs/TUI.md` | ext `.md` | 6953 bytes
2768. `flow/crates/serializer/examples/main.rs` | ext `.rs` | 1493 bytes
2769. `flow/crates/serializer/examples/parts/agent_syntax.rs` | ext `.rs` | 3385 bytes
2770. `flow/crates/serializer/examples/parts/arrays.rs` | ext `.rs` | 1346 bytes
2771. `flow/crates/serializer/examples/parts/arrays_of_arrays.rs` | ext `.rs` | 1420 bytes
2772. `flow/crates/serializer/examples/parts/decode_strict.rs` | ext `.rs` | 456 bytes
2773. `flow/crates/serializer/examples/parts/delimiters.rs` | ext `.rs` | 625 bytes
2774. `flow/crates/serializer/examples/parts/empty_and_root.rs` | ext `.rs` | 613 bytes
2775. `flow/crates/serializer/examples/parts/mixed_arrays.rs` | ext `.rs` | 1994 bytes
2776. `flow/crates/serializer/examples/parts/objects.rs` | ext `.rs` | 2356 bytes
2777. `flow/crates/serializer/examples/parts/round_trip.rs` | ext `.rs` | 1236 bytes
2778. `flow/crates/serializer/examples/parts/structs.rs` | ext `.rs` | 1458 bytes
2779. `flow/crates/serializer/examples/parts/tabular.rs` | ext `.rs` | 2339 bytes
2780. `flow/crates/serializer/LICENSE` | ext `<none>` | 1146 bytes
2781. `flow/crates/serializer/README.md` | ext `.md` | 21674 bytes
2782. `flow/crates/serializer/rustfmt.toml` | ext `.toml` | 1350 bytes
2783. `flow/crates/serializer/src/cli/main.rs` | ext `.rs` | 11927 bytes
2784. `flow/crates/serializer/src/constants.rs` | ext `.rs` | 1772 bytes
2785. `flow/crates/serializer/src/decode/expansion.rs` | ext `.rs` | 8790 bytes
2786. `flow/crates/serializer/src/decode/mod.rs` | ext `.rs` | 8658 bytes
2787. `flow/crates/serializer/src/decode/parser.rs` | ext `.rs` | 65558 bytes
2788. `flow/crates/serializer/src/decode/scanner.rs` | ext `.rs` | 21953 bytes
2789. `flow/crates/serializer/src/decode/validation.rs` | ext `.rs` | 3951 bytes
2790. `flow/crates/serializer/src/encode/folding.rs` | ext `.rs` | 4784 bytes
2791. `flow/crates/serializer/src/encode/mod.rs` | ext `.rs` | 27930 bytes
2792. `flow/crates/serializer/src/encode/primitives.rs` | ext `.rs` | 2286 bytes
2793. `flow/crates/serializer/src/encode/writer.rs` | ext `.rs` | 9873 bytes
2794. `flow/crates/serializer/src/lib.rs` | ext `.rs` | 12816 bytes
2795. `flow/crates/serializer/src/llm/conversation.rs` | ext `.rs` | 20013 bytes
2796. `flow/crates/serializer/src/llm/mod.rs` | ext `.rs` | 1169 bytes
2797. `flow/crates/serializer/src/llm/packed.rs` | ext `.rs` | 56808 bytes
2798. `flow/crates/serializer/src/llm/schema.rs` | ext `.rs` | 26979 bytes
2799. `flow/crates/serializer/src/llm/util.rs` | ext `.rs` | 9593 bytes
2800. `flow/crates/serializer/src/tui/app.rs` | ext `.rs` | 33299 bytes
2801. `flow/crates/serializer/src/tui/components/diff_viewer.rs` | ext `.rs` | 3127 bytes
2802. `flow/crates/serializer/src/tui/components/editor.rs` | ext `.rs` | 2194 bytes
2803. `flow/crates/serializer/src/tui/components/file_browser.rs` | ext `.rs` | 5829 bytes
2804. `flow/crates/serializer/src/tui/components/help_screen.rs` | ext `.rs` | 2796 bytes
2805. `flow/crates/serializer/src/tui/components/history_panel.rs` | ext `.rs` | 3060 bytes
2806. `flow/crates/serializer/src/tui/components/mod.rs` | ext `.rs` | 541 bytes
2807. `flow/crates/serializer/src/tui/components/repl_panel.rs` | ext `.rs` | 3347 bytes
2808. `flow/crates/serializer/src/tui/components/settings_panel.rs` | ext `.rs` | 6004 bytes
2809. `flow/crates/serializer/src/tui/components/stats_bar.rs` | ext `.rs` | 2473 bytes
2810. `flow/crates/serializer/src/tui/components/status_bar.rs` | ext `.rs` | 2975 bytes
2811. `flow/crates/serializer/src/tui/events.rs` | ext `.rs` | 744 bytes
2812. `flow/crates/serializer/src/tui/keybindings.rs` | ext `.rs` | 3238 bytes
2813. `flow/crates/serializer/src/tui/mod.rs` | ext `.rs` | 1225 bytes
2814. `flow/crates/serializer/src/tui/repl_command.rs` | ext `.rs` | 3965 bytes
2815. `flow/crates/serializer/src/tui/state/app_state.rs` | ext `.rs` | 8425 bytes
2816. `flow/crates/serializer/src/tui/state/editor_state.rs` | ext `.rs` | 2227 bytes
2817. `flow/crates/serializer/src/tui/state/file_state.rs` | ext `.rs` | 2962 bytes
2818. `flow/crates/serializer/src/tui/state/mod.rs` | ext `.rs` | 406 bytes
2819. `flow/crates/serializer/src/tui/state/repl_state.rs` | ext `.rs` | 4643 bytes
2820. `flow/crates/serializer/src/tui/theme.rs` | ext `.rs` | 3584 bytes
2821. `flow/crates/serializer/src/tui/ui.rs` | ext `.rs` | 7345 bytes
2822. `flow/crates/serializer/src/types/delimiter.rs` | ext `.rs` | 2339 bytes
2823. `flow/crates/serializer/src/types/errors.rs` | ext `.rs` | 10954 bytes
2824. `flow/crates/serializer/src/types/folding.rs` | ext `.rs` | 2749 bytes
2825. `flow/crates/serializer/src/types/mod.rs` | ext `.rs` | 422 bytes
2826. `flow/crates/serializer/src/types/options.rs` | ext `.rs` | 5424 bytes
2827. `flow/crates/serializer/src/types/value.rs` | ext `.rs` | 14312 bytes
2828. `flow/crates/serializer/src/utils/literal.rs` | ext `.rs` | 2544 bytes
2829. `flow/crates/serializer/src/utils/mod.rs` | ext `.rs` | 3506 bytes
2830. `flow/crates/serializer/src/utils/number.rs` | ext `.rs` | 6140 bytes
2831. `flow/crates/serializer/src/utils/string.rs` | ext `.rs` | 9432 bytes
2832. `flow/crates/serializer/src/utils/validation.rs` | ext `.rs` | 2174 bytes
2833. `flow/crates/serializer/tests/agent_syntax.rs` | ext `.rs` | 6512 bytes
2834. `flow/crates/serializer/tests/arrays.rs` | ext `.rs` | 2427 bytes
2835. `flow/crates/serializer/tests/delimiters.rs` | ext `.rs` | 3980 bytes
2836. `flow/crates/serializer/tests/errors.rs` | ext `.rs` | 14051 bytes
2837. `flow/crates/serializer/tests/numeric.rs` | ext `.rs` | 633 bytes
2838. `flow/crates/serializer/tests/objects.rs` | ext `.rs` | 1179 bytes
2839. `flow/crates/serializer/tests/real_world.rs` | ext `.rs` | 2550 bytes
2840. `flow/crates/serializer/tests/round_trip.rs` | ext `.rs` | 1279 bytes
2841. `flow/crates/serializer/tests/spec_fixtures.rs` | ext `.rs` | 5781 bytes
2842. `flow/crates/serializer/tests/unicode.rs` | ext `.rs` | 483 bytes
2843. `flow/docs/API.md` | ext `.md` | 11898 bytes
2844. `flow/docs/ARCHITECTURE.md` | ext `.md` | 2910 bytes
2845. `flow/docs/BROWSER_RELEASE.md` | ext `.md` | 2280 bytes
2846. `flow/docs/CLIENT_HANDOFF.md` | ext `.md` | 1196 bytes
2847. `flow/docs/COMPETITIVE_SCORECARD.md` | ext `.md` | 5673 bytes
2848. `flow/docs/DEVELOPMENT.md` | ext `.md` | 1312 bytes
2849. `flow/docs/FLOW_100_PLAN.md` | ext `.md` | 2424 bytes
2850. `flow/docs/FLOW_STATUS.md` | ext `.md` | 3981 bytes
2851. `flow/docs/MODELS.md` | ext `.md` | 9820 bytes
2852. `flow/docs/OCR.md` | ext `.md` | 2230 bytes
2853. `flow/docs/WAKEWORD_TRAINING.md` | ext `.md` | 2158 bytes
2854. `flow/DX.md` | ext `.md` | 7511 bytes
2855. `flow/FLOW-TODO.md` | ext `.md` | 26641 bytes
2856. `flow/README.md` | ext `.md` | 30926 bytes
2857. `flow/src/always_on/mod.rs` | ext `.rs` | 41 bytes
2858. `flow/src/always_on/runtime.rs` | ext `.rs` | 7386 bytes
2859. `flow/src/audio/features.rs` | ext `.rs` | 5037 bytes
2860. `flow/src/audio/loader.rs` | ext `.rs` | 3982 bytes
2861. `flow/src/audio/mod.rs` | ext `.rs` | 299 bytes
2862. `flow/src/audio/player.rs` | ext `.rs` | 3264 bytes
2863. `flow/src/audio/recorder.rs` | ext `.rs` | 8884 bytes
2864. `flow/src/audio/resample.rs` | ext `.rs` | 2952 bytes
2865. `flow/src/audio/vad.rs` | ext `.rs` | 4849 bytes
2866. `flow/src/audio/wakeword.rs` | ext `.rs` | 8839 bytes
2867. `flow/src/bin/flow-dictate.rs` | ext `.rs` | 47160 bytes
2868. `flow/src/browser/catalog.rs` | ext `.rs` | 9499 bytes
2869. `flow/src/browser/engine.rs` | ext `.rs` | 15154 bytes
2870. `flow/src/browser/extension_smoke.rs` | ext `.rs` | 26946 bytes
2871. `flow/src/browser/mod.rs` | ext `.rs` | 327 bytes
2872. `flow/src/browser/pack_recovery.rs` | ext `.rs` | 11639 bytes
2873. `flow/src/browser/pack_reuse.rs` | ext `.rs` | 8555 bytes
2874. `flow/src/browser/types.rs` | ext `.rs` | 9704 bytes
2875. `flow/src/browser/webllm_acceleration.rs` | ext `.rs` | 8776 bytes
2876. `flow/src/cli/args.rs` | ext `.rs` | 73522 bytes
2877. `flow/src/cli/chat.rs` | ext `.rs` | 5896 bytes
2878. `flow/src/cli/commands.rs` | ext `.rs` | 233201 bytes
2879. `flow/src/cli/mod.rs` | ext `.rs` | 148 bytes
2880. `flow/src/codex/adapter.rs` | ext `.rs` | 25504 bytes
2881. `flow/src/codex/mod.rs` | ext `.rs` | 76 bytes
2882. `flow/src/codex/types.rs` | ext `.rs` | 5210 bytes
2883. `flow/src/competitive/mod.rs` | ext `.rs` | 86 bytes
2884. `flow/src/competitive/progress.rs` | ext `.rs` | 5546 bytes
2885. `flow/src/competitive/scorecard.rs` | ext `.rs` | 12988 bytes
2886. `flow/src/config/export.rs` | ext `.rs` | 12483 bytes
2887. `flow/src/config/mod.rs` | ext `.rs` | 113 bytes
2888. `flow/src/config/release.rs` | ext `.rs` | 13937 bytes
2889. `flow/src/config/types.rs` | ext `.rs` | 11426 bytes
2890. `flow/src/dx/facade.rs` | ext `.rs` | 14038 bytes
2891. `flow/src/dx/mod.rs` | ext `.rs` | 39 bytes
2892. `flow/src/embed/mod.rs` | ext `.rs` | 78 bytes
2893. `flow/src/embed/registry.rs` | ext `.rs` | 14686 bytes
2894. `flow/src/embed/types.rs` | ext `.rs` | 4926 bytes
2895. `flow/src/experience/accessibility.rs` | ext `.rs` | 12417 bytes
2896. `flow/src/experience/activation.rs` | ext `.rs` | 3689 bytes
2897. `flow/src/experience/always_on.rs` | ext `.rs` | 6913 bytes
2898. `flow/src/experience/audio.rs` | ext `.rs` | 2480 bytes
2899. `flow/src/experience/audit.rs` | ext `.rs` | 4721 bytes
2900. `flow/src/experience/automation.rs` | ext `.rs` | 5746 bytes
2901. `flow/src/experience/bridges.rs` | ext `.rs` | 6419 bytes
2902. `flow/src/experience/bundle.rs` | ext `.rs` | 4365 bytes
2903. `flow/src/experience/capture.rs` | ext `.rs` | 5552 bytes
2904. `flow/src/experience/command.rs` | ext `.rs` | 3591 bytes
2905. `flow/src/experience/consent.rs` | ext `.rs` | 2530 bytes
2906. `flow/src/experience/contracts.rs` | ext `.rs` | 6262 bytes
2907. `flow/src/experience/control.rs` | ext `.rs` | 9516 bytes
2908. `flow/src/experience/dictation.rs` | ext `.rs` | 12162 bytes
2909. `flow/src/experience/editor.rs` | ext `.rs` | 2135 bytes
2910. `flow/src/experience/embedded.rs` | ext `.rs` | 5057 bytes
2911. `flow/src/experience/engine.rs` | ext `.rs` | 9187 bytes
2912. `flow/src/experience/executors.rs` | ext `.rs` | 8335 bytes
2913. `flow/src/experience/facade.rs` | ext `.rs` | 4378 bytes
2914. `flow/src/experience/health.rs` | ext `.rs` | 11689 bytes
2915. `flow/src/experience/host.rs` | ext `.rs` | 5032 bytes
2916. `flow/src/experience/hostdictation.rs` | ext `.rs` | 4665 bytes
2917. `flow/src/experience/hostkit.rs` | ext `.rs` | 25814 bytes
2918. `flow/src/experience/installer.rs` | ext `.rs` | 5838 bytes
2919. `flow/src/experience/lifecycle.rs` | ext `.rs` | 5314 bytes
2920. `flow/src/experience/microphone.rs` | ext `.rs` | 2219 bytes
2921. `flow/src/experience/mod.rs` | ext `.rs` | 5917 bytes
2922. `flow/src/experience/modules.rs` | ext `.rs` | 19133 bytes
2923. `flow/src/experience/onboarding.rs` | ext `.rs` | 3816 bytes
2924. `flow/src/experience/overlay.rs` | ext `.rs` | 3164 bytes
2925. `flow/src/experience/permissions.rs` | ext `.rs` | 4858 bytes
2926. `flow/src/experience/persistence.rs` | ext `.rs` | 3225 bytes
2927. `flow/src/experience/presenters.rs` | ext `.rs` | 3370 bytes
2928. `flow/src/experience/proofing.rs` | ext `.rs` | 16212 bytes
2929. `flow/src/experience/recovery.rs` | ext `.rs` | 3391 bytes
2930. `flow/src/experience/runtime_policy.rs` | ext `.rs` | 4171 bytes
2931. `flow/src/experience/selection.rs` | ext `.rs` | 6425 bytes
2932. `flow/src/experience/session.rs` | ext `.rs` | 7363 bytes
2933. `flow/src/experience/snooze.rs` | ext `.rs` | 3492 bytes
2934. `flow/src/experience/status.rs` | ext `.rs` | 3191 bytes
2935. `flow/src/experience/stores.rs` | ext `.rs` | 10796 bytes
2936. `flow/src/experience/supervisor.rs` | ext `.rs` | 5624 bytes
2937. `flow/src/experience/types.rs` | ext `.rs` | 7648 bytes
2938. `flow/src/experience/typing.rs` | ext `.rs` | 18005 bytes
2939. `flow/src/experience/wake.rs` | ext `.rs` | 2178 bytes
2940. `flow/src/experience/wakedetect.rs` | ext `.rs` | 10784 bytes
2941. `flow/src/experience/workspace.rs` | ext `.rs` | 7307 bytes
2942. `flow/src/forge_bridge/mod.rs` | ext `.rs` | 35 bytes
2943. `flow/src/forge_bridge/plan.rs` | ext `.rs` | 2685 bytes
2944. `flow/src/friday/artifacts.rs` | ext `.rs` | 23882 bytes
2945. `flow/src/friday/checks.rs` | ext `.rs` | 15244 bytes
2946. `flow/src/friday/dashboard_export.rs` | ext `.rs` | 47294 bytes
2947. `flow/src/friday/dashboard_host_bridge.rs` | ext `.rs` | 7928 bytes
2948. `flow/src/friday/dashboard_host_runner.rs` | ext `.rs` | 70711 bytes
2949. `flow/src/friday/dashboard_product_ui.rs` | ext `.rs` | 21676 bytes
2950. `flow/src/friday/dashboard_release_checklist.rs` | ext `.rs` | 24240 bytes
2951. `flow/src/friday/dashboard_runner_release.rs` | ext `.rs` | 23646 bytes
2952. `flow/src/friday/execution_handoff.rs` | ext `.rs` | 11486 bytes
2953. `flow/src/friday/live_ui.rs` | ext `.rs` | 9845 bytes
2954. `flow/src/friday/mod.rs` | ext `.rs` | 24139 bytes
2955. `flow/src/friday/multimodal.rs` | ext `.rs` | 43879 bytes
2956. `flow/src/friday/readiness.rs` | ext `.rs` | 14135 bytes
2957. `flow/src/friday/release_qa.rs` | ext `.rs` | 14781 bytes
2958. `flow/src/friday/research.rs` | ext `.rs` | 29968 bytes
2959. `flow/src/friday/route_visuals.rs` | ext `.rs` | 8159 bytes
2960. `flow/src/friday/runtime.rs` | ext `.rs` | 12684 bytes
2961. `flow/src/friday/ui.rs` | ext `.rs` | 37108 bytes
2962. `flow/src/friday/verification.rs` | ext `.rs` | 9569 bytes
2963. `flow/src/friday/workspace.rs` | ext `.rs` | 16518 bytes
2964. `flow/src/lib.rs` | ext `.rs` | 12999 bytes
2965. `flow/src/long_context/mod.rs` | ext `.rs` | 35 bytes
2966. `flow/src/long_context/plan.rs` | ext `.rs` | 2323 bytes
2967. `flow/src/main.rs` | ext `.rs` | 167 bytes
2968. `flow/src/models/llm.rs` | ext `.rs` | 47486 bytes
2969. `flow/src/models/mod.rs` | ext `.rs` | 154 bytes
2970. `flow/src/models/ocr.rs` | ext `.rs` | 7405 bytes
2971. `flow/src/models/stt.rs` | ext `.rs` | 12767 bytes
2972. `flow/src/models/stt_complex.rs.bak` | ext `.bak` | 19579 bytes
2973. `flow/src/models/stt_simple.rs` | ext `.rs` | 3894 bytes
2974. `flow/src/models/tts.rs` | ext `.rs` | 6992 bytes
2975. `flow/src/pipeline/mod.rs` | ext `.rs` | 67 bytes
2976. `flow/src/pipeline/voice.rs` | ext `.rs` | 3265 bytes
2977. `flow/src/prompt/mod.rs` | ext `.rs` | 47 bytes
2978. `flow/src/prompt/serializer.rs` | ext `.rs` | 3693 bytes
2979. `flow/src/provider_catalog/mod.rs` | ext `.rs` | 35 bytes
2980. `flow/src/provider_catalog/plan.rs` | ext `.rs` | 1999 bytes
2981. `flow/src/remote/mod.rs` | ext `.rs` | 39 bytes
2982. `flow/src/remote/policy.rs` | ext `.rs` | 5223 bytes
2983. `flow/src/runtime/broker.rs` | ext `.rs` | 15135 bytes
2984. `flow/src/runtime/catalog.rs` | ext `.rs` | 22172 bytes
2985. `flow/src/runtime/local.rs` | ext `.rs` | 14979 bytes
2986. `flow/src/runtime/mod.rs` | ext `.rs` | 148 bytes
2987. `flow/src/runtime/types.rs` | ext `.rs` | 8702 bytes
2988. `flow/src/search/metasearch_api.rs` | ext `.rs` | 6584 bytes
2989. `flow/src/search/mod.rs` | ext `.rs` | 88 bytes
2990. `flow/src/search/plan.rs` | ext `.rs` | 5552 bytes
2991. `flow/src/storage/flowpack.rs` | ext `.rs` | 5017 bytes
2992. `flow/src/storage/mod.rs` | ext `.rs` | 43 bytes
2993. `flow/src/utils/mod.rs` | ext `.rs` | 61 bytes
2994. `flow/src/utils/system.rs` | ext `.rs` | 8828 bytes
2995. `flow/src/workspace/mod.rs` | ext `.rs` | 1765 bytes
2996. `flow/src/writing/grammar.rs` | ext `.rs` | 3407 bytes
2997. `flow/src/writing/mod.rs` | ext `.rs` | 41 bytes
2998. `flow/src/zed/adapter.rs` | ext `.rs` | 9896 bytes
2999. `flow/src/zed/mod.rs` | ext `.rs` | 76 bytes
3000. `flow/src/zed/types.rs` | ext `.rs` | 2798 bytes
3001. `flow/src/zeroclaw/adapter.rs` | ext `.rs` | 20537 bytes
3002. `flow/src/zeroclaw/mod.rs` | ext `.rs` | 76 bytes
3003. `flow/src/zeroclaw/types.rs` | ext `.rs` | 4035 bytes
3004. `form/Cargo.toml` | ext `.toml` | 899 bytes
3005. `form/CHANGELOG.md` | ext `.md` | 374 bytes
3006. `form/LICENSE` | ext `<none>` | 559 bytes
3007. `form/LICENSE-APACHE` | ext `<none>` | 5871 bytes
3008. `form/LICENSE-MIT` | ext `<none>` | 1093 bytes
3009. `form/README.md` | ext `.md` | 2154 bytes
3010. `form/src/lib.rs` | ext `.rs` | 13387 bytes
3011. `framework-core/Cargo.toml` | ext `.toml` | 662 bytes
3012. `framework-core/CHANGELOG.md` | ext `.md` | 374 bytes
3013. `framework-core/LICENSE` | ext `<none>` | 559 bytes
3014. `framework-core/LICENSE-APACHE` | ext `<none>` | 5871 bytes
3015. `framework-core/LICENSE-MIT` | ext `<none>` | 1093 bytes
3016. `framework-core/README.md` | ext `.md` | 350 bytes
3017. `framework-core/src/icon/component.rs` | ext `.rs` | 1732 bytes
3018. `framework-core/src/icon/macros.rs` | ext `.rs` | 1507 bytes
3019. `framework-core/src/icon/mod.rs` | ext `.rs` | 1472 bytes
3020. `framework-core/src/icon/parser.rs` | ext `.rs` | 2052 bytes
3021. `framework-core/src/icon/tests.rs` | ext `.rs` | 4798 bytes
3022. `framework-core/src/lib.rs` | ext `.rs` | 16347 bytes
3023. `guard/Cargo.toml` | ext `.toml` | 469 bytes
3024. `guard/CHANGELOG.md` | ext `.md` | 375 bytes
3025. `guard/LICENSE` | ext `<none>` | 559 bytes
3026. `guard/LICENSE-APACHE` | ext `<none>` | 5871 bytes
3027. `guard/LICENSE-MIT` | ext `<none>` | 1093 bytes
3028. `guard/README.md` | ext `.md` | 321 bytes
3029. `guard/src/lib.rs` | ext `.rs` | 8233 bytes
3030. `integrations/flow-forge/benches/ingest.rs` | ext `.rs` | 2461 bytes
3031. `integrations/flow-forge/build.rs` | ext `.rs` | 62 bytes
3032. `integrations/flow-forge/Cargo.lock` | ext `.lock` | 103443 bytes
3033. `integrations/flow-forge/Cargo.toml` | ext `.toml` | 1564 bytes
3034. `integrations/flow-forge/CHANGELOG.md` | ext `.md` | 6891 bytes
3035. `integrations/flow-forge/docs/FORGE_99_PLAN.md` | ext `.md` | 2493 bytes
3036. `integrations/flow-forge/docs/FORGE_STATUS.md` | ext `.md` | 14303 bytes
3037. `integrations/flow-forge/README.md` | ext `.md` | 6145 bytes
3038. `integrations/flow-forge/src/chunking/cdc.rs` | ext `.rs` | 1034 bytes
3039. `integrations/flow-forge/src/chunking/mod.rs` | ext `.rs` | 620 bytes
3040. `integrations/flow-forge/src/chunking/structure_aware/csp.rs` | ext `.rs` | 1021 bytes
3041. `integrations/flow-forge/src/chunking/structure_aware/exr.rs` | ext `.rs` | 1189 bytes
3042. `integrations/flow-forge/src/chunking/structure_aware/mod.rs` | ext `.rs` | 59 bytes
3043. `integrations/flow-forge/src/chunking/structure_aware/mp4.rs` | ext `.rs` | 3055 bytes
3044. `integrations/flow-forge/src/chunking/structure_aware/uasset.rs` | ext `.rs` | 1146 bytes
3045. `integrations/flow-forge/src/cli/add.rs` | ext `.rs` | 5972 bytes
3046. `integrations/flow-forge/src/cli/auth.rs` | ext `.rs` | 6576 bytes
3047. `integrations/flow-forge/src/cli/checkout.rs` | ext `.rs` | 7046 bytes
3048. `integrations/flow-forge/src/cli/checkout_archive.rs` | ext `.rs` | 10887 bytes
3049. `integrations/flow-forge/src/cli/commit.rs` | ext `.rs` | 2891 bytes
3050. `integrations/flow-forge/src/cli/diff.rs` | ext `.rs` | 4093 bytes
3051. `integrations/flow-forge/src/cli/init.rs` | ext `.rs` | 272 bytes
3052. `integrations/flow-forge/src/cli/jobs.rs` | ext `.rs` | 3126 bytes
3053. `integrations/flow-forge/src/cli/log.rs` | ext `.rs` | 1542 bytes
3054. `integrations/flow-forge/src/cli/mod.rs` | ext `.rs` | 282 bytes
3055. `integrations/flow-forge/src/cli/package.rs` | ext `.rs` | 5890 bytes
3056. `integrations/flow-forge/src/cli/pull.rs` | ext `.rs` | 22049 bytes
3057. `integrations/flow-forge/src/cli/push.rs` | ext `.rs` | 31656 bytes
3058. `integrations/flow-forge/src/cli/remote.rs` | ext `.rs` | 2722 bytes
3059. `integrations/flow-forge/src/cli/status.rs` | ext `.rs` | 3665 bytes
3060. `integrations/flow-forge/src/cli/sync.rs` | ext `.rs` | 5673 bytes
3061. `integrations/flow-forge/src/cli/train_dict.rs` | ext `.rs` | 1494 bytes
3062. `integrations/flow-forge/src/cli/vibe_demo.rs` | ext `.rs` | 3209 bytes
3063. `integrations/flow-forge/src/core/chunk.rs` | ext `.rs` | 461 bytes
3064. `integrations/flow-forge/src/core/hash.rs` | ext `.rs` | 1156 bytes
3065. `integrations/flow-forge/src/core/manifest.rs` | ext `.rs` | 3164 bytes
3066. `integrations/flow-forge/src/core/mod.rs` | ext `.rs` | 71 bytes
3067. `integrations/flow-forge/src/core/repository.rs` | ext `.rs` | 10386 bytes
3068. `integrations/flow-forge/src/db/metadata.rs` | ext `.rs` | 16107 bytes
3069. `integrations/flow-forge/src/db/mod.rs` | ext `.rs` | 19 bytes
3070. `integrations/flow-forge/src/jobs/mod.rs` | ext `.rs` | 10818 bytes
3071. `integrations/flow-forge/src/lib.rs` | ext `.rs` | 3185 bytes
3072. `integrations/flow-forge/src/main.rs` | ext `.rs` | 8891 bytes
3073. `integrations/flow-forge/src/mirror/auth.rs` | ext `.rs` | 2078 bytes
3074. `integrations/flow-forge/src/mirror/backends/bitbucket.rs` | ext `.rs` | 2355 bytes
3075. `integrations/flow-forge/src/mirror/backends/dropbox.rs` | ext `.rs` | 5760 bytes
3076. `integrations/flow-forge/src/mirror/backends/gdrive.rs` | ext `.rs` | 3193 bytes
3077. `integrations/flow-forge/src/mirror/backends/github.rs` | ext `.rs` | 3386 bytes
3078. `integrations/flow-forge/src/mirror/backends/gitlab.rs` | ext `.rs` | 3197 bytes
3079. `integrations/flow-forge/src/mirror/backends/mega.rs` | ext `.rs` | 5327 bytes
3080. `integrations/flow-forge/src/mirror/backends/pinterest.rs` | ext `.rs` | 2478 bytes
3081. `integrations/flow-forge/src/mirror/backends/r2.rs` | ext `.rs` | 2199 bytes
3082. `integrations/flow-forge/src/mirror/backends/sketchfab.rs` | ext `.rs` | 2118 bytes
3083. `integrations/flow-forge/src/mirror/backends/soundcloud.rs` | ext `.rs` | 2425 bytes
3084. `integrations/flow-forge/src/mirror/backends/youtube.rs` | ext `.rs` | 2888 bytes
3085. `integrations/flow-forge/src/mirror/dispatcher.rs` | ext `.rs` | 2439 bytes
3086. `integrations/flow-forge/src/mirror/media_type.rs` | ext `.rs` | 1491 bytes
3087. `integrations/flow-forge/src/mirror/mod.rs` | ext `.rs` | 3483 bytes
3088. `integrations/flow-forge/src/mirror/records.rs` | ext `.rs` | 14124 bytes
3089. `integrations/flow-forge/src/packages.rs` | ext `.rs` | 36195 bytes
3090. `integrations/flow-forge/src/recovery/mod.rs` | ext `.rs` | 5418 bytes
3091. `integrations/flow-forge/src/store/cas.rs` | ext `.rs` | 3917 bytes
3092. `integrations/flow-forge/src/store/compression.rs` | ext `.rs` | 1231 bytes
3093. `integrations/flow-forge/src/store/mod.rs` | ext `.rs` | 51 bytes
3094. `integrations/flow-forge/src/store/pack.rs` | ext `.rs` | 2064 bytes
3095. `integrations/flow-forge/src/sync/mod.rs` | ext `.rs` | 63772 bytes
3096. `integrations/flow-forge/src/transport/mod.rs` | ext `.rs` | 55 bytes
3097. `integrations/flow-forge/src/transport/protocol.rs` | ext `.rs` | 8479 bytes
3098. `integrations/flow-forge/src/transport/quic.rs` | ext `.rs` | 7042 bytes
3099. `integrations/flow-forge/src/transport/repository.rs` | ext `.rs` | 23048 bytes
3100. `integrations/flow-forge/src/util/human.rs` | ext `.rs` | 783 bytes
3101. `integrations/flow-forge/src/util/ignore.rs` | ext `.rs` | 2510 bytes
3102. `integrations/flow-forge/src/util/mod.rs` | ext `.rs` | 52 bytes
3103. `integrations/flow-forge/src/util/progress.rs` | ext `.rs` | 664 bytes
3104. `integrations/flow-forge/tests/integration.rs` | ext `.rs` | 25383 bytes
3105. `integrations/flow-forge/TODO.md` | ext `.md` | 4281 bytes
3106. `integrations/flow-forge-bridge/mod.rs` | ext `.rs` | 35 bytes
3107. `integrations/flow-forge-bridge/plan.rs` | ext `.rs` | 2685 bytes
3108. `integrations/flow-serializer/.github/CODEOWNERS` | ext `<none>` | 237 bytes
3109. `integrations/flow-serializer/.github/workflows/ci.yml` | ext `.yml` | 2128 bytes
3110. `integrations/flow-serializer/.github/workflows/release.yml` | ext `.yml` | 1293 bytes
3111. `integrations/flow-serializer/.gitignore` | ext `.gitignore` | 186 bytes
3112. `integrations/flow-serializer/.gitmodules` | ext `.gitmodules` | 78 bytes
3113. `integrations/flow-serializer/Cargo.toml` | ext `.toml` | 1834 bytes
3114. `integrations/flow-serializer/CHANGELOG.md` | ext `.md` | 5785 bytes
3115. `integrations/flow-serializer/CONTRIBUTING.md` | ext `.md` | 2646 bytes
3116. `integrations/flow-serializer/docs/DX_SERIALIZER.md` | ext `.md` | 5632 bytes
3117. `integrations/flow-serializer/docs/INTEGRATIONS.md` | ext `.md` | 4259 bytes
3118. `integrations/flow-serializer/docs/PRODUCTION_READY.md` | ext `.md` | 4306 bytes
3119. `integrations/flow-serializer/docs/TUI.md` | ext `.md` | 6953 bytes
3120. `integrations/flow-serializer/examples/main.rs` | ext `.rs` | 1493 bytes
3121. `integrations/flow-serializer/examples/parts/agent_syntax.rs` | ext `.rs` | 3385 bytes
3122. `integrations/flow-serializer/examples/parts/arrays.rs` | ext `.rs` | 1346 bytes
3123. `integrations/flow-serializer/examples/parts/arrays_of_arrays.rs` | ext `.rs` | 1420 bytes
3124. `integrations/flow-serializer/examples/parts/decode_strict.rs` | ext `.rs` | 456 bytes
3125. `integrations/flow-serializer/examples/parts/delimiters.rs` | ext `.rs` | 625 bytes
3126. `integrations/flow-serializer/examples/parts/empty_and_root.rs` | ext `.rs` | 613 bytes
3127. `integrations/flow-serializer/examples/parts/mixed_arrays.rs` | ext `.rs` | 1994 bytes
3128. `integrations/flow-serializer/examples/parts/objects.rs` | ext `.rs` | 2356 bytes
3129. `integrations/flow-serializer/examples/parts/round_trip.rs` | ext `.rs` | 1236 bytes
3130. `integrations/flow-serializer/examples/parts/structs.rs` | ext `.rs` | 1458 bytes
3131. `integrations/flow-serializer/examples/parts/tabular.rs` | ext `.rs` | 2339 bytes
3132. `integrations/flow-serializer/LICENSE` | ext `<none>` | 1146 bytes
3133. `integrations/flow-serializer/README.md` | ext `.md` | 21674 bytes
3134. `integrations/flow-serializer/rustfmt.toml` | ext `.toml` | 1350 bytes
3135. `integrations/flow-serializer/src/cli/main.rs` | ext `.rs` | 11927 bytes
3136. `integrations/flow-serializer/src/constants.rs` | ext `.rs` | 1772 bytes
3137. `integrations/flow-serializer/src/decode/expansion.rs` | ext `.rs` | 8790 bytes
3138. `integrations/flow-serializer/src/decode/mod.rs` | ext `.rs` | 8658 bytes
3139. `integrations/flow-serializer/src/decode/parser.rs` | ext `.rs` | 65558 bytes
3140. `integrations/flow-serializer/src/decode/scanner.rs` | ext `.rs` | 21953 bytes
3141. `integrations/flow-serializer/src/decode/validation.rs` | ext `.rs` | 3951 bytes
3142. `integrations/flow-serializer/src/encode/folding.rs` | ext `.rs` | 4784 bytes
3143. `integrations/flow-serializer/src/encode/mod.rs` | ext `.rs` | 27930 bytes
3144. `integrations/flow-serializer/src/encode/primitives.rs` | ext `.rs` | 2286 bytes
3145. `integrations/flow-serializer/src/encode/writer.rs` | ext `.rs` | 9873 bytes
3146. `integrations/flow-serializer/src/lib.rs` | ext `.rs` | 12816 bytes
3147. `integrations/flow-serializer/src/llm/conversation.rs` | ext `.rs` | 20013 bytes
3148. `integrations/flow-serializer/src/llm/mod.rs` | ext `.rs` | 1169 bytes
3149. `integrations/flow-serializer/src/llm/packed.rs` | ext `.rs` | 56808 bytes
3150. `integrations/flow-serializer/src/llm/schema.rs` | ext `.rs` | 26979 bytes
3151. `integrations/flow-serializer/src/llm/util.rs` | ext `.rs` | 9593 bytes
3152. `integrations/flow-serializer/src/tui/app.rs` | ext `.rs` | 33299 bytes
3153. `integrations/flow-serializer/src/tui/components/diff_viewer.rs` | ext `.rs` | 3127 bytes
3154. `integrations/flow-serializer/src/tui/components/editor.rs` | ext `.rs` | 2194 bytes
3155. `integrations/flow-serializer/src/tui/components/file_browser.rs` | ext `.rs` | 5829 bytes
3156. `integrations/flow-serializer/src/tui/components/help_screen.rs` | ext `.rs` | 2796 bytes
3157. `integrations/flow-serializer/src/tui/components/history_panel.rs` | ext `.rs` | 3060 bytes
3158. `integrations/flow-serializer/src/tui/components/mod.rs` | ext `.rs` | 541 bytes
3159. `integrations/flow-serializer/src/tui/components/repl_panel.rs` | ext `.rs` | 3347 bytes
3160. `integrations/flow-serializer/src/tui/components/settings_panel.rs` | ext `.rs` | 6004 bytes
3161. `integrations/flow-serializer/src/tui/components/stats_bar.rs` | ext `.rs` | 2473 bytes
3162. `integrations/flow-serializer/src/tui/components/status_bar.rs` | ext `.rs` | 2975 bytes
3163. `integrations/flow-serializer/src/tui/events.rs` | ext `.rs` | 744 bytes
3164. `integrations/flow-serializer/src/tui/keybindings.rs` | ext `.rs` | 3238 bytes
3165. `integrations/flow-serializer/src/tui/mod.rs` | ext `.rs` | 1225 bytes
3166. `integrations/flow-serializer/src/tui/repl_command.rs` | ext `.rs` | 3965 bytes
3167. `integrations/flow-serializer/src/tui/state/app_state.rs` | ext `.rs` | 8425 bytes
3168. `integrations/flow-serializer/src/tui/state/editor_state.rs` | ext `.rs` | 2227 bytes
3169. `integrations/flow-serializer/src/tui/state/file_state.rs` | ext `.rs` | 2962 bytes
3170. `integrations/flow-serializer/src/tui/state/mod.rs` | ext `.rs` | 406 bytes
3171. `integrations/flow-serializer/src/tui/state/repl_state.rs` | ext `.rs` | 4643 bytes
3172. `integrations/flow-serializer/src/tui/theme.rs` | ext `.rs` | 3584 bytes
3173. `integrations/flow-serializer/src/tui/ui.rs` | ext `.rs` | 7345 bytes
3174. `integrations/flow-serializer/src/types/delimiter.rs` | ext `.rs` | 2339 bytes
3175. `integrations/flow-serializer/src/types/errors.rs` | ext `.rs` | 10954 bytes
3176. `integrations/flow-serializer/src/types/folding.rs` | ext `.rs` | 2749 bytes
3177. `integrations/flow-serializer/src/types/mod.rs` | ext `.rs` | 422 bytes
3178. `integrations/flow-serializer/src/types/options.rs` | ext `.rs` | 5424 bytes
3179. `integrations/flow-serializer/src/types/value.rs` | ext `.rs` | 14312 bytes
3180. `integrations/flow-serializer/src/utils/literal.rs` | ext `.rs` | 2544 bytes
3181. `integrations/flow-serializer/src/utils/mod.rs` | ext `.rs` | 3506 bytes
3182. `integrations/flow-serializer/src/utils/number.rs` | ext `.rs` | 6140 bytes
3183. `integrations/flow-serializer/src/utils/string.rs` | ext `.rs` | 9432 bytes
3184. `integrations/flow-serializer/src/utils/validation.rs` | ext `.rs` | 2174 bytes
3185. `integrations/flow-serializer/tests/agent_syntax.rs` | ext `.rs` | 6512 bytes
3186. `integrations/flow-serializer/tests/arrays.rs` | ext `.rs` | 2427 bytes
3187. `integrations/flow-serializer/tests/delimiters.rs` | ext `.rs` | 3980 bytes
3188. `integrations/flow-serializer/tests/errors.rs` | ext `.rs` | 14051 bytes
3189. `integrations/flow-serializer/tests/numeric.rs` | ext `.rs` | 633 bytes
3190. `integrations/flow-serializer/tests/objects.rs` | ext `.rs` | 1179 bytes
3191. `integrations/flow-serializer/tests/real_world.rs` | ext `.rs` | 2550 bytes
3192. `integrations/flow-serializer/tests/round_trip.rs` | ext `.rs` | 1279 bytes
3193. `integrations/flow-serializer/tests/spec_fixtures.rs` | ext `.rs` | 5781 bytes
3194. `integrations/flow-serializer/tests/unicode.rs` | ext `.rs` | 483 bytes
3195. `integrations/n8n-nodes-base/AUTOMATIONS-BRIDGE.md` | ext `.md` | 3980 bytes
3196. `integrations/n8n-nodes-base/credentials/ActionNetworkApi.credentials.ts` | ext `.ts` | 900 bytes
3197. `integrations/n8n-nodes-base/credentials/ActiveCampaignApi.credentials.ts` | ext `.ts` | 864 bytes
3198. `integrations/n8n-nodes-base/credentials/AcuitySchedulingApi.credentials.ts` | ext `.ts` | 529 bytes
3199. `integrations/n8n-nodes-base/credentials/AcuitySchedulingOAuth2Api.credentials.ts` | ext `.ts` | 1173 bytes
3200. `integrations/n8n-nodes-base/credentials/AdaloApi.credentials.ts` | ext `.ts` | 1067 bytes
3201. `integrations/n8n-nodes-base/credentials/AffinityApi.credentials.ts` | ext `.ts` | 400 bytes
3202. `integrations/n8n-nodes-base/credentials/AgileCrmApi.credentials.ts` | ext `.ts` | 763 bytes
3203. `integrations/n8n-nodes-base/credentials/AirtableApi.credentials.ts` | ext `.ts` | 808 bytes
3204. `integrations/n8n-nodes-base/credentials/AirtableOAuth2Api.credentials.ts` | ext `.ts` | 1157 bytes
3205. `integrations/n8n-nodes-base/credentials/AirtableTokenApi.credentials.ts` | ext `.ts` | 1067 bytes
3206. `integrations/n8n-nodes-base/credentials/AirtopApi.credentials.ts` | ext `.ts` | 1082 bytes
3207. `integrations/n8n-nodes-base/credentials/AlienVaultApi.credentials.ts` | ext `.ts` | 995 bytes
3208. `integrations/n8n-nodes-base/credentials/Amqp.credentials.ts` | ext `.ts` | 973 bytes
3209. `integrations/n8n-nodes-base/credentials/ApiTemplateIoApi.credentials.ts` | ext `.ts` | 772 bytes
3210. `integrations/n8n-nodes-base/credentials/AsanaApi.credentials.ts` | ext `.ts` | 757 bytes
3211. `integrations/n8n-nodes-base/credentials/AsanaOAuth2Api.credentials.ts` | ext `.ts` | 1091 bytes
3212. `integrations/n8n-nodes-base/credentials/Auth0ManagementApi.credentials.ts` | ext `.ts` | 2184 bytes
3213. `integrations/n8n-nodes-base/credentials/AutopilotApi.credentials.ts` | ext `.ts` | 404 bytes
3214. `integrations/n8n-nodes-base/credentials/Aws.credentials.ts` | ext `.ts` | 2516 bytes
3215. `integrations/n8n-nodes-base/credentials/AwsAssumeRole.credentials.ts` | ext `.ts` | 5006 bytes
3216. `integrations/n8n-nodes-base/credentials/AzureStorageOAuth2Api.credentials.ts` | ext `.ts` | 869 bytes
3217. `integrations/n8n-nodes-base/credentials/AzureStorageSharedKeyApi.credentials.ts` | ext `.ts` | 3317 bytes
3218. `integrations/n8n-nodes-base/credentials/BambooHrApi.credentials.ts` | ext `.ts` | 501 bytes
3219. `integrations/n8n-nodes-base/credentials/BannerbearApi.credentials.ts` | ext `.ts` | 416 bytes
3220. `integrations/n8n-nodes-base/credentials/BaserowApi.credentials.ts` | ext `.ts` | 1974 bytes
3221. `integrations/n8n-nodes-base/credentials/BaserowTokenApi.credentials.ts` | ext `.ts` | 1014 bytes
3222. `integrations/n8n-nodes-base/credentials/BeeminderApi.credentials.ts` | ext `.ts` | 762 bytes
3223. `integrations/n8n-nodes-base/credentials/BeeminderOAuth2Api.credentials.ts` | ext `.ts` | 1132 bytes
3224. `integrations/n8n-nodes-base/credentials/BitbucketAccessTokenApi.credentials.ts` | ext `.ts` | 1201 bytes
3225. `integrations/n8n-nodes-base/credentials/BitbucketApi.credentials.ts` | ext `.ts` | 513 bytes
3226. `integrations/n8n-nodes-base/credentials/BitlyApi.credentials.ts` | ext `.ts` | 398 bytes
3227. `integrations/n8n-nodes-base/credentials/BitlyOAuth2Api.credentials.ts` | ext `.ts` | 1522 bytes
3228. `integrations/n8n-nodes-base/credentials/BitwardenApi.credentials.ts` | ext `.ts` | 1100 bytes
3229. `integrations/n8n-nodes-base/credentials/BoxOAuth2Api.credentials.ts` | ext `.ts` | 1765 bytes
3230. `integrations/n8n-nodes-base/credentials/BrandfetchApi.credentials.ts` | ext `.ts` | 774 bytes
3231. `integrations/n8n-nodes-base/credentials/BrevoApi.credentials.ts` | ext `.ts` | 779 bytes
3232. `integrations/n8n-nodes-base/credentials/BubbleApi.credentials.ts` | ext `.ts` | 1235 bytes
3233. `integrations/n8n-nodes-base/credentials/CalApi.credentials.ts` | ext `.ts` | 825 bytes
3234. `integrations/n8n-nodes-base/credentials/CalendlyApi.credentials.ts` | ext `.ts` | 1032 bytes
3235. `integrations/n8n-nodes-base/credentials/CalendlyOAuth2Api.credentials.ts` | ext `.ts` | 1119 bytes
3236. `integrations/n8n-nodes-base/credentials/CarbonBlackApi.credentials.ts` | ext `.ts` | 1168 bytes
3237. `integrations/n8n-nodes-base/credentials/ChargebeeApi.credentials.ts` | ext `.ts` | 510 bytes
3238. `integrations/n8n-nodes-base/credentials/CircleCiApi.credentials.ts` | ext `.ts` | 411 bytes
3239. `integrations/n8n-nodes-base/credentials/CiscoMerakiApi.credentials.ts` | ext `.ts` | 1028 bytes
3240. `integrations/n8n-nodes-base/credentials/CiscoSecureEndpointApi.credentials.ts` | ext `.ts` | 2861 bytes
3241. `integrations/n8n-nodes-base/credentials/CiscoUmbrellaApi.credentials.ts` | ext `.ts` | 2021 bytes
3242. `integrations/n8n-nodes-base/credentials/CiscoWebexOAuth2Api.credentials.ts` | ext `.ts` | 1439 bytes
3243. `integrations/n8n-nodes-base/credentials/ClearbitApi.credentials.ts` | ext `.ts` | 808 bytes
3244. `integrations/n8n-nodes-base/credentials/ClickUpApi.credentials.ts` | ext `.ts` | 755 bytes
3245. `integrations/n8n-nodes-base/credentials/ClickUpOAuth2Api.credentials.ts` | ext `.ts` | 1094 bytes
3246. `integrations/n8n-nodes-base/credentials/ClockifyApi.credentials.ts` | ext `.ts` | 748 bytes
3247. `integrations/n8n-nodes-base/credentials/CloudflareApi.credentials.ts` | ext `.ts` | 772 bytes
3248. `integrations/n8n-nodes-base/credentials/CockpitApi.credentials.ts` | ext `.ts` | 543 bytes
3249. `integrations/n8n-nodes-base/credentials/CodaApi.credentials.ts` | ext `.ts` | 609 bytes
3250. `integrations/n8n-nodes-base/credentials/common/aws/descriptions.ts` | ext `.ts` | 3296 bytes
3251. `integrations/n8n-nodes-base/credentials/common/aws/system-credentials-utils.test.ts` | ext `.ts` | 29265 bytes
3252. `integrations/n8n-nodes-base/credentials/common/aws/system-credentials-utils.ts` | ext `.ts` | 11037 bytes
3253. `integrations/n8n-nodes-base/credentials/common/aws/types.ts` | ext `.ts` | 4367 bytes
3254. `integrations/n8n-nodes-base/credentials/common/aws/utils.test.ts` | ext `.ts` | 23132 bytes
3255. `integrations/n8n-nodes-base/credentials/common/aws/utils.ts` | ext `.ts` | 12889 bytes
3256. `integrations/n8n-nodes-base/credentials/common/http.ts` | ext `.ts` | 373 bytes
3257. `integrations/n8n-nodes-base/credentials/ContentfulApi.credentials.ts` | ext `.ts` | 1119 bytes
3258. `integrations/n8n-nodes-base/credentials/ConvertApi.credentials.ts` | ext `.ts` | 1204 bytes
3259. `integrations/n8n-nodes-base/credentials/ConvertKitApi.credentials.ts` | ext `.ts` | 1272 bytes
3260. `integrations/n8n-nodes-base/credentials/CopperApi.credentials.ts` | ext `.ts` | 1014 bytes
3261. `integrations/n8n-nodes-base/credentials/CortexApi.credentials.ts` | ext `.ts` | 951 bytes
3262. `integrations/n8n-nodes-base/credentials/CrateDb.credentials.ts` | ext `.ts` | 1091 bytes
3263. `integrations/n8n-nodes-base/credentials/CrowdStrikeOAuth2Api.credentials.ts` | ext `.ts` | 2077 bytes
3264. `integrations/n8n-nodes-base/credentials/Crypto.credentials.ts` | ext `.ts` | 1761 bytes
3265. `integrations/n8n-nodes-base/credentials/CurrentsApi.credentials.ts` | ext `.ts` | 899 bytes
3266. `integrations/n8n-nodes-base/credentials/CustomerIoApi.credentials.ts` | ext `.ts` | 2726 bytes
3267. `integrations/n8n-nodes-base/credentials/DatabricksApi.credentials.ts` | ext `.ts` | 1258 bytes
3268. `integrations/n8n-nodes-base/credentials/DatabricksOAuth2Api.credentials.ts` | ext `.ts` | 1472 bytes
3269. `integrations/n8n-nodes-base/credentials/DatadogApi.credentials.ts` | ext `.ts` | 1695 bytes
3270. `integrations/n8n-nodes-base/credentials/DeepLApi.credentials.ts` | ext `.ts` | 1073 bytes
3271. `integrations/n8n-nodes-base/credentials/DemioApi.credentials.ts` | ext `.ts` | 527 bytes
3272. `integrations/n8n-nodes-base/credentials/DfirIrisApi.credentials.ts` | ext `.ts` | 1556 bytes
3273. `integrations/n8n-nodes-base/credentials/DhlApi.credentials.ts` | ext `.ts` | 380 bytes
3274. `integrations/n8n-nodes-base/credentials/DiscordBotApi.credentials.ts` | ext `.ts` | 800 bytes
3275. `integrations/n8n-nodes-base/credentials/DiscordOAuth2Api.credentials.ts` | ext `.ts` | 2039 bytes
3276. `integrations/n8n-nodes-base/credentials/DiscordWebhookApi.credentials.ts` | ext `.ts` | 647 bytes
3277. `integrations/n8n-nodes-base/credentials/DiscourseApi.credentials.ts` | ext `.ts` | 1259 bytes
3278. `integrations/n8n-nodes-base/credentials/DisqusApi.credentials.ts` | ext `.ts` | 552 bytes
3279. `integrations/n8n-nodes-base/credentials/DriftApi.credentials.ts` | ext `.ts` | 565 bytes
3280. `integrations/n8n-nodes-base/credentials/DriftOAuth2Api.credentials.ts` | ext `.ts` | 1081 bytes
3281. `integrations/n8n-nodes-base/credentials/DropboxApi.credentials.ts` | ext `.ts` | 1064 bytes
3282. `integrations/n8n-nodes-base/credentials/DropboxOAuth2Api.credentials.ts` | ext `.ts` | 1531 bytes
3283. `integrations/n8n-nodes-base/credentials/DropcontactApi.credentials.ts` | ext `.ts` | 823 bytes
3284. `integrations/n8n-nodes-base/credentials/DynatraceApi.credentials.ts` | ext `.ts` | 914 bytes
3285. `integrations/n8n-nodes-base/credentials/EgoiApi.credentials.ts` | ext `.ts` | 526 bytes
3286. `integrations/n8n-nodes-base/credentials/ElasticsearchApi.credentials.ts` | ext `.ts` | 1415 bytes
3287. `integrations/n8n-nodes-base/credentials/ElasticSecurityApi.credentials.ts` | ext `.ts` | 2264 bytes
3288. `integrations/n8n-nodes-base/credentials/EmeliaApi.credentials.ts` | ext `.ts` | 392 bytes
3289. `integrations/n8n-nodes-base/credentials/ERPNextApi.credentials.ts` | ext `.ts` | 2942 bytes
3290. `integrations/n8n-nodes-base/credentials/EventbriteApi.credentials.ts` | ext `.ts` | 412 bytes
3291. `integrations/n8n-nodes-base/credentials/EventbriteOAuth2Api.credentials.ts` | ext `.ts` | 1117 bytes
3292. `integrations/n8n-nodes-base/credentials/F5BigIpApi.credentials.ts` | ext `.ts` | 921 bytes
3293. `integrations/n8n-nodes-base/credentials/FacebookGraphApi.credentials.ts` | ext `.ts` | 773 bytes
3294. `integrations/n8n-nodes-base/credentials/FacebookGraphApiOAuth2Api.credentials.ts` | ext `.ts` | 2236 bytes
3295. `integrations/n8n-nodes-base/credentials/FacebookGraphAppApi.credentials.ts` | ext `.ts` | 604 bytes
3296. `integrations/n8n-nodes-base/credentials/FacebookGraphAppOAuth2Api.credentials.ts` | ext `.ts` | 613 bytes
3297. `integrations/n8n-nodes-base/credentials/FacebookLeadAdsOAuth2Api.credentials.ts` | ext `.ts` | 1272 bytes
3298. `integrations/n8n-nodes-base/credentials/FigmaApi.credentials.ts` | ext `.ts` | 398 bytes
3299. `integrations/n8n-nodes-base/credentials/FigmaOAuth2Api.credentials.ts` | ext `.ts` | 1825 bytes
3300. `integrations/n8n-nodes-base/credentials/FileMaker.credentials.ts` | ext `.ts` | 688 bytes
3301. `integrations/n8n-nodes-base/credentials/FilescanApi.credentials.ts` | ext `.ts` | 1063 bytes
3302. `integrations/n8n-nodes-base/credentials/FlowApi.credentials.ts` | ext `.ts` | 505 bytes
3303. `integrations/n8n-nodes-base/credentials/FormIoApi.credentials.ts` | ext `.ts` | 2297 bytes
3304. `integrations/n8n-nodes-base/credentials/FormstackApi.credentials.ts` | ext `.ts` | 421 bytes
3305. `integrations/n8n-nodes-base/credentials/FormstackOAuth2Api.credentials.ts` | ext `.ts` | 1182 bytes
3306. `integrations/n8n-nodes-base/credentials/FortiGateApi.credentials.ts` | ext `.ts` | 863 bytes
3307. `integrations/n8n-nodes-base/credentials/FreshdeskApi.credentials.ts` | ext `.ts` | 1029 bytes
3308. `integrations/n8n-nodes-base/credentials/FreshserviceApi.credentials.ts` | ext `.ts` | 726 bytes
3309. `integrations/n8n-nodes-base/credentials/FreshworksCrmApi.credentials.ts` | ext `.ts` | 1156 bytes
3310. `integrations/n8n-nodes-base/credentials/Ftp.credentials.ts` | ext `.ts` | 734 bytes
3311. `integrations/n8n-nodes-base/credentials/GetResponseApi.credentials.ts` | ext `.ts` | 770 bytes
3312. `integrations/n8n-nodes-base/credentials/GetResponseOAuth2Api.credentials.ts` | ext `.ts` | 1128 bytes
3313. `integrations/n8n-nodes-base/credentials/GhostAdminApi.credentials.ts` | ext `.ts` | 1302 bytes
3314. `integrations/n8n-nodes-base/credentials/GhostContentApi.credentials.ts` | ext `.ts` | 1057 bytes
3315. `integrations/n8n-nodes-base/credentials/GithubApi.credentials.ts` | ext `.ts` | 1082 bytes
3316. `integrations/n8n-nodes-base/credentials/GithubOAuth2Api.credentials.ts` | ext `.ts` | 1751 bytes
3317. `integrations/n8n-nodes-base/credentials/GitlabApi.credentials.ts` | ext `.ts` | 933 bytes
3318. `integrations/n8n-nodes-base/credentials/GitlabOAuth2Api.credentials.ts` | ext `.ts` | 1212 bytes
3319. `integrations/n8n-nodes-base/credentials/GitPassword.credentials.ts` | ext `.ts` | 621 bytes
3320. `integrations/n8n-nodes-base/credentials/GmailOAuth2Api.credentials.ts` | ext `.ts` | 786 bytes
3321. `integrations/n8n-nodes-base/credentials/GongApi.credentials.ts` | ext `.ts` | 1105 bytes
3322. `integrations/n8n-nodes-base/credentials/GongOAuth2Api.credentials.ts` | ext `.ts` | 2105 bytes
3323. `integrations/n8n-nodes-base/credentials/GoogleAdsOAuth2Api.credentials.ts` | ext `.ts` | 683 bytes
3324. `integrations/n8n-nodes-base/credentials/GoogleAnalyticsOAuth2Api.credentials.ts` | ext `.ts` | 593 bytes
3325. `integrations/n8n-nodes-base/credentials/GoogleApi.credentials.ts` | ext `.ts` | 8087 bytes
3326. `integrations/n8n-nodes-base/credentials/GoogleBigQueryOAuth2Api.credentials.ts` | ext `.ts` | 631 bytes
3327. `integrations/n8n-nodes-base/credentials/GoogleBooksOAuth2Api.credentials.ts` | ext `.ts` | 518 bytes
3328. `integrations/n8n-nodes-base/credentials/GoogleBusinessProfileOAuth2Api.credentials.ts` | ext `.ts` | 1188 bytes
3329. `integrations/n8n-nodes-base/credentials/GoogleCalendarOAuth2Api.credentials.ts` | ext `.ts` | 589 bytes
3330. `integrations/n8n-nodes-base/credentials/GoogleChatOAuth2Api.credentials.ts` | ext `.ts` | 625 bytes
3331. `integrations/n8n-nodes-base/credentials/GoogleCloudNaturalLanguageOAuth2Api.credentials.ts` | ext `.ts` | 632 bytes
3332. `integrations/n8n-nodes-base/credentials/GoogleCloudStorageOAuth2Api.credentials.ts` | ext `.ts` | 795 bytes
3333. `integrations/n8n-nodes-base/credentials/GoogleContactsOAuth2Api.credentials.ts` | ext `.ts` | 530 bytes
3334. `integrations/n8n-nodes-base/credentials/GoogleDocsOAuth2Api.credentials.ts` | ext `.ts` | 616 bytes
3335. `integrations/n8n-nodes-base/credentials/GoogleDriveOAuth2Api.credentials.ts` | ext `.ts` | 985 bytes
3336. `integrations/n8n-nodes-base/credentials/GoogleFirebaseCloudFirestoreOAuth2Api.credentials.ts` | ext `.ts` | 627 bytes
3337. `integrations/n8n-nodes-base/credentials/GoogleFirebaseRealtimeDatabaseOAuth2Api.credentials.ts` | ext `.ts` | 1090 bytes
3338. `integrations/n8n-nodes-base/credentials/GoogleOAuth2Api.credentials.ts` | ext `.ts` | 1063 bytes
3339. `integrations/n8n-nodes-base/credentials/GooglePerspectiveOAuth2Api.credentials.ts` | ext `.ts` | 545 bytes
3340. `integrations/n8n-nodes-base/credentials/GoogleSheetsOAuth2Api.credentials.ts` | ext `.ts` | 1080 bytes
3341. `integrations/n8n-nodes-base/credentials/GoogleSheetsTriggerOAuth2Api.credentials.ts` | ext `.ts` | 1091 bytes
3342. `integrations/n8n-nodes-base/credentials/GoogleSlidesOAuth2Api.credentials.ts` | ext `.ts` | 583 bytes
3343. `integrations/n8n-nodes-base/credentials/GoogleTasksOAuth2Api.credentials.ts` | ext `.ts` | 518 bytes
3344. `integrations/n8n-nodes-base/credentials/GoogleTranslateOAuth2Api.credentials.ts` | ext `.ts` | 542 bytes
3345. `integrations/n8n-nodes-base/credentials/GotifyApi.credentials.ts` | ext `.ts` | 1079 bytes
3346. `integrations/n8n-nodes-base/credentials/GoToWebinarOAuth2Api.credentials.ts` | ext `.ts` | 1117 bytes
3347. `integrations/n8n-nodes-base/credentials/GrafanaApi.credentials.ts` | ext `.ts` | 1027 bytes
3348. `integrations/n8n-nodes-base/credentials/GristApi.credentials.ts` | ext `.ts` | 1339 bytes
3349. `integrations/n8n-nodes-base/credentials/GSuiteAdminOAuth2Api.credentials.ts` | ext `.ts` | 868 bytes
3350. `integrations/n8n-nodes-base/credentials/GumroadApi.credentials.ts` | ext `.ts` | 406 bytes
3351. `integrations/n8n-nodes-base/credentials/HaloPSAApi.credentials.ts` | ext `.ts` | 1730 bytes
3352. `integrations/n8n-nodes-base/credentials/HarvestApi.credentials.ts` | ext `.ts` | 626 bytes
3353. `integrations/n8n-nodes-base/credentials/HarvestOAuth2Api.credentials.ts` | ext `.ts` | 1115 bytes
3354. `integrations/n8n-nodes-base/credentials/HelpScoutOAuth2Api.credentials.ts` | ext `.ts` | 1144 bytes
3355. `integrations/n8n-nodes-base/credentials/HighLevelApi.credentials.ts` | ext `.ts` | 766 bytes
3356. `integrations/n8n-nodes-base/credentials/HighLevelOAuth2Api.credentials.ts` | ext `.ts` | 1864 bytes
3357. `integrations/n8n-nodes-base/credentials/HomeAssistantApi.credentials.ts` | ext `.ts` | 708 bytes
3358. `integrations/n8n-nodes-base/credentials/HttpBasicAuth.credentials.ts` | ext `.ts` | 645 bytes
3359. `integrations/n8n-nodes-base/credentials/HttpBearerAuth.credentials.ts` | ext `.ts` | 1081 bytes
3360. `integrations/n8n-nodes-base/credentials/HttpCustomAuth.credentials.ts` | ext `.ts` | 857 bytes
3361. `integrations/n8n-nodes-base/credentials/HttpDigestAuth.credentials.ts` | ext `.ts` | 648 bytes
3362. `integrations/n8n-nodes-base/credentials/HttpHeaderAuth.credentials.ts` | ext `.ts` | 941 bytes
3363. `integrations/n8n-nodes-base/credentials/HttpMultipleHeadersAuth.credentials.ts` | ext `.ts` | 1659 bytes
3364. `integrations/n8n-nodes-base/credentials/HttpQueryAuth.credentials.ts` | ext `.ts` | 585 bytes
3365. `integrations/n8n-nodes-base/credentials/HttpSslAuth.credentials.ts` | ext `.ts` | 1186 bytes
3366. `integrations/n8n-nodes-base/credentials/HubspotApi.credentials.ts` | ext `.ts` | 1090 bytes
3367. `integrations/n8n-nodes-base/credentials/HubspotAppToken.credentials.ts` | ext `.ts` | 784 bytes
3368. `integrations/n8n-nodes-base/credentials/HubspotDeveloperApi.credentials.ts` | ext `.ts` | 1927 bytes
3369. `integrations/n8n-nodes-base/credentials/HubspotOAuth2Api.credentials.ts` | ext `.ts` | 1518 bytes
3370. `integrations/n8n-nodes-base/credentials/HumanticAiApi.credentials.ts` | ext `.ts` | 409 bytes
3371. `integrations/n8n-nodes-base/credentials/HunterApi.credentials.ts` | ext `.ts` | 392 bytes
3372. `integrations/n8n-nodes-base/credentials/HybridAnalysisApi.credentials.ts` | ext `.ts` | 843 bytes
3373. `integrations/n8n-nodes-base/credentials/icons/AlienVault.png` | ext `.png` | 2705 bytes
3374. `integrations/n8n-nodes-base/credentials/icons/Auth0.dark.svg` | ext `.svg` | 479 bytes
3375. `integrations/n8n-nodes-base/credentials/icons/Auth0.svg` | ext `.svg` | 479 bytes
3376. `integrations/n8n-nodes-base/credentials/icons/AWS.dark.svg` | ext `.svg` | 5901 bytes
3377. `integrations/n8n-nodes-base/credentials/icons/AWS.svg` | ext `.svg` | 5903 bytes
3378. `integrations/n8n-nodes-base/credentials/icons/Azure.svg` | ext `.svg` | 1487 bytes
3379. `integrations/n8n-nodes-base/credentials/icons/Calendly.svg` | ext `.svg` | 3663 bytes
3380. `integrations/n8n-nodes-base/credentials/icons/Cisco.dark.svg` | ext `.svg` | 7391 bytes
3381. `integrations/n8n-nodes-base/credentials/icons/Cisco.svg` | ext `.svg` | 7405 bytes
3382. `integrations/n8n-nodes-base/credentials/icons/ConvertApi.png` | ext `.png` | 10406 bytes
3383. `integrations/n8n-nodes-base/credentials/icons/CrowdStrike.dark.svg` | ext `.svg` | 2215 bytes
3384. `integrations/n8n-nodes-base/credentials/icons/CrowdStrike.svg` | ext `.svg` | 2225 bytes
3385. `integrations/n8n-nodes-base/credentials/icons/databricks.dark.svg` | ext `.svg` | 873 bytes
3386. `integrations/n8n-nodes-base/credentials/icons/databricks.svg` | ext `.svg` | 873 bytes
3387. `integrations/n8n-nodes-base/credentials/icons/Datadog.svg` | ext `.svg` | 3519 bytes
3388. `integrations/n8n-nodes-base/credentials/icons/DfirIris.svg` | ext `.svg` | 34983 bytes
3389. `integrations/n8n-nodes-base/credentials/icons/Dynatrace.svg` | ext `.svg` | 3685 bytes
3390. `integrations/n8n-nodes-base/credentials/icons/Elastic.svg` | ext `.svg` | 1931 bytes
3391. `integrations/n8n-nodes-base/credentials/icons/F5.svg` | ext `.svg` | 3490 bytes
3392. `integrations/n8n-nodes-base/credentials/icons/Filescan.svg` | ext `.svg` | 13434 bytes
3393. `integrations/n8n-nodes-base/credentials/icons/Fortinet.svg` | ext `.svg` | 482 bytes
3394. `integrations/n8n-nodes-base/credentials/icons/Google.svg` | ext `.svg` | 687 bytes
3395. `integrations/n8n-nodes-base/credentials/icons/highLevel.svg` | ext `.svg` | 804 bytes
3396. `integrations/n8n-nodes-base/credentials/icons/Hybrid.png` | ext `.png` | 22622 bytes
3397. `integrations/n8n-nodes-base/credentials/icons/IBM.dark.svg` | ext `.svg` | 2360 bytes
3398. `integrations/n8n-nodes-base/credentials/icons/IBM.svg` | ext `.svg` | 2362 bytes
3399. `integrations/n8n-nodes-base/credentials/icons/Imperva.dark.svg` | ext `.svg` | 706 bytes
3400. `integrations/n8n-nodes-base/credentials/icons/Imperva.svg` | ext `.svg` | 706 bytes
3401. `integrations/n8n-nodes-base/credentials/icons/jwt.svg` | ext `.svg` | 3360 bytes
3402. `integrations/n8n-nodes-base/credentials/icons/Kibana.svg` | ext `.svg` | 362 bytes
3403. `integrations/n8n-nodes-base/credentials/icons/Malcore.png` | ext `.png` | 13238 bytes
3404. `integrations/n8n-nodes-base/credentials/icons/Microsoft.svg` | ext `.svg` | 272 bytes
3405. `integrations/n8n-nodes-base/credentials/icons/microsoftSharePoint.svg` | ext `.svg` | 4862 bytes
3406. `integrations/n8n-nodes-base/credentials/icons/Miro.svg` | ext `.svg` | 936 bytes
3407. `integrations/n8n-nodes-base/credentials/icons/Mist.svg` | ext `.svg` | 1482 bytes
3408. `integrations/n8n-nodes-base/credentials/icons/Okta.dark.svg` | ext `.svg` | 6095 bytes
3409. `integrations/n8n-nodes-base/credentials/icons/Okta.svg` | ext `.svg` | 6095 bytes
3410. `integrations/n8n-nodes-base/credentials/icons/OpenCTI.png` | ext `.png` | 25426 bytes
3411. `integrations/n8n-nodes-base/credentials/icons/Qualys.svg` | ext `.svg` | 944 bytes
3412. `integrations/n8n-nodes-base/credentials/icons/Rapid7InsightVm.svg` | ext `.svg` | 1895 bytes
3413. `integrations/n8n-nodes-base/credentials/icons/RecordedFuture.dark.svg` | ext `.svg` | 2523 bytes
3414. `integrations/n8n-nodes-base/credentials/icons/RecordedFuture.svg` | ext `.svg` | 2523 bytes
3415. `integrations/n8n-nodes-base/credentials/icons/Sekoia.svg` | ext `.svg` | 2201 bytes
3416. `integrations/n8n-nodes-base/credentials/icons/Shuffler.svg` | ext `.svg` | 226 bytes
3417. `integrations/n8n-nodes-base/credentials/icons/SolarWindsIpam.svg` | ext `.svg` | 5139 bytes
3418. `integrations/n8n-nodes-base/credentials/icons/SolarWindsObservability.svg` | ext `.svg` | 17298 bytes
3419. `integrations/n8n-nodes-base/credentials/icons/Sysdig.Black.svg` | ext `.svg` | 5079 bytes
3420. `integrations/n8n-nodes-base/credentials/icons/Sysdig.White.svg` | ext `.svg` | 5090 bytes
3421. `integrations/n8n-nodes-base/credentials/icons/Trellix.svg` | ext `.svg` | 549 bytes
3422. `integrations/n8n-nodes-base/credentials/icons/Twake.png` | ext `.png` | 3673 bytes
3423. `integrations/n8n-nodes-base/credentials/icons/VirusTotal.svg` | ext `.svg` | 222 bytes
3424. `integrations/n8n-nodes-base/credentials/icons/vmware.dark.svg` | ext `.svg` | 2378 bytes
3425. `integrations/n8n-nodes-base/credentials/icons/vmware.svg` | ext `.svg` | 2378 bytes
3426. `integrations/n8n-nodes-base/credentials/icons/Wazuh.png` | ext `.png` | 3494 bytes
3427. `integrations/n8n-nodes-base/credentials/icons/Zabbix.svg` | ext `.svg` | 1540 bytes
3428. `integrations/n8n-nodes-base/credentials/icons/Zscaler.svg` | ext `.svg` | 659 bytes
3429. `integrations/n8n-nodes-base/credentials/Imap.credentials.ts` | ext `.ts` | 1459 bytes
3430. `integrations/n8n-nodes-base/credentials/ImpervaWafApi.credentials.ts` | ext `.ts` | 1014 bytes
3431. `integrations/n8n-nodes-base/credentials/IntercomApi.credentials.ts` | ext `.ts` | 794 bytes
3432. `integrations/n8n-nodes-base/credentials/InvoiceNinjaApi.credentials.ts` | ext `.ts` | 1754 bytes
3433. `integrations/n8n-nodes-base/credentials/IterableApi.credentials.ts` | ext `.ts` | 1056 bytes
3434. `integrations/n8n-nodes-base/credentials/JenkinsApi.credentials.ts` | ext `.ts` | 624 bytes
3435. `integrations/n8n-nodes-base/credentials/JinaAiApi.credentials.ts` | ext `.ts` | 836 bytes
3436. `integrations/n8n-nodes-base/credentials/JiraSoftwareCloudApi.credentials.ts` | ext `.ts` | 1084 bytes
3437. `integrations/n8n-nodes-base/credentials/JiraSoftwareCloudOAuth2Api.credentials.ts` | ext `.ts` | 2297 bytes
3438. `integrations/n8n-nodes-base/credentials/JiraSoftwareServerApi.credentials.ts` | ext `.ts` | 1086 bytes
3439. `integrations/n8n-nodes-base/credentials/JiraSoftwareServerPatApi.credentials.ts` | ext `.ts` | 968 bytes
3440. `integrations/n8n-nodes-base/credentials/JotFormApi.credentials.ts` | ext `.ts` | 900 bytes
3441. `integrations/n8n-nodes-base/credentials/JwtAuth.credentials.ts` | ext `.ts` | 2266 bytes
3442. `integrations/n8n-nodes-base/credentials/Kafka.credentials.ts` | ext `.ts` | 1883 bytes
3443. `integrations/n8n-nodes-base/credentials/KeapOAuth2Api.credentials.ts` | ext `.ts` | 1100 bytes
3444. `integrations/n8n-nodes-base/credentials/KibanaApi.credentials.ts` | ext `.ts` | 1300 bytes
3445. `integrations/n8n-nodes-base/credentials/KoBoToolboxApi.credentials.ts` | ext `.ts` | 1067 bytes
3446. `integrations/n8n-nodes-base/credentials/Ldap.credentials.ts` | ext `.ts` | 2183 bytes
3447. `integrations/n8n-nodes-base/credentials/LemlistApi.credentials.ts` | ext `.ts` | 1006 bytes
3448. `integrations/n8n-nodes-base/credentials/LinearApi.credentials.ts` | ext `.ts` | 837 bytes
3449. `integrations/n8n-nodes-base/credentials/LinearOAuth2Api.credentials.ts` | ext `.ts` | 2114 bytes
3450. `integrations/n8n-nodes-base/credentials/LineNotifyOAuth2Api.credentials.ts` | ext `.ts` | 1138 bytes
3451. `integrations/n8n-nodes-base/credentials/LingvaNexApi.credentials.ts` | ext `.ts` | 404 bytes
3452. `integrations/n8n-nodes-base/credentials/LinkedInCommunityManagementOAuth2Api.credentials.ts` | ext `.ts` | 1276 bytes
3453. `integrations/n8n-nodes-base/credentials/LinkedInOAuth2Api.credentials.ts` | ext `.ts` | 2298 bytes
3454. `integrations/n8n-nodes-base/credentials/LoneScaleApi.credentials.ts` | ext `.ts` | 751 bytes
3455. `integrations/n8n-nodes-base/credentials/Magento2Api.credentials.ts` | ext `.ts` | 869 bytes
3456. `integrations/n8n-nodes-base/credentials/MailcheckApi.credentials.ts` | ext `.ts` | 404 bytes
3457. `integrations/n8n-nodes-base/credentials/MailchimpApi.credentials.ts` | ext `.ts` | 797 bytes
3458. `integrations/n8n-nodes-base/credentials/MailchimpOAuth2Api.credentials.ts` | ext `.ts` | 1282 bytes
3459. `integrations/n8n-nodes-base/credentials/MailerLiteApi.credentials.ts` | ext `.ts` | 1372 bytes
3460. `integrations/n8n-nodes-base/credentials/MailgunApi.credentials.ts` | ext `.ts` | 1227 bytes
3461. `integrations/n8n-nodes-base/credentials/MailjetEmailApi.credentials.ts` | ext `.ts` | 1229 bytes
3462. `integrations/n8n-nodes-base/credentials/MailjetSmsApi.credentials.ts` | ext `.ts` | 767 bytes
3463. `integrations/n8n-nodes-base/credentials/MalcoreApi.credentials.ts` | ext `.ts` | 1067 bytes
3464. `integrations/n8n-nodes-base/credentials/MandrillApi.credentials.ts` | ext `.ts` | 400 bytes
3465. `integrations/n8n-nodes-base/credentials/MarketstackApi.credentials.ts` | ext `.ts` | 575 bytes
3466. `integrations/n8n-nodes-base/credentials/MatrixApi.credentials.ts` | ext `.ts` | 544 bytes
3467. `integrations/n8n-nodes-base/credentials/MattermostApi.credentials.ts` | ext `.ts` | 1201 bytes
3468. `integrations/n8n-nodes-base/credentials/MauticApi.credentials.ts` | ext `.ts` | 1051 bytes
3469. `integrations/n8n-nodes-base/credentials/MauticOAuth2Api.credentials.ts` | ext `.ts` | 1346 bytes
3470. `integrations/n8n-nodes-base/credentials/MediumApi.credentials.ts` | ext `.ts` | 402 bytes
3471. `integrations/n8n-nodes-base/credentials/MediumOAuth2Api.credentials.ts` | ext `.ts` | 1411 bytes
3472. `integrations/n8n-nodes-base/credentials/MessageBirdApi.credentials.ts` | ext `.ts` | 415 bytes
3473. `integrations/n8n-nodes-base/credentials/MetabaseApi.credentials.ts` | ext `.ts` | 1758 bytes
3474. `integrations/n8n-nodes-base/credentials/MicrosoftAzureCosmosDbSharedKeyApi.credentials.ts` | ext `.ts` | 3029 bytes
3475. `integrations/n8n-nodes-base/credentials/MicrosoftAzureMonitorOAuth2Api.credentials.ts` | ext `.ts` | 2573 bytes
3476. `integrations/n8n-nodes-base/credentials/MicrosoftDynamicsOAuth2Api.credentials.ts` | ext `.ts` | 2793 bytes
3477. `integrations/n8n-nodes-base/credentials/MicrosoftEntraOAuth2Api.credentials.ts` | ext `.ts` | 1773 bytes
3478. `integrations/n8n-nodes-base/credentials/MicrosoftExcelOAuth2Api.credentials.ts` | ext `.ts` | 1413 bytes
3479. `integrations/n8n-nodes-base/credentials/MicrosoftGraphSecurityOAuth2Api.credentials.ts` | ext `.ts` | 505 bytes
3480. `integrations/n8n-nodes-base/credentials/MicrosoftOAuth2Api.credentials.ts` | ext `.ts` | 2001 bytes
3481. `integrations/n8n-nodes-base/credentials/MicrosoftOneDriveOAuth2Api.credentials.ts` | ext `.ts` | 580 bytes
3482. `integrations/n8n-nodes-base/credentials/MicrosoftOutlookOAuth2Api.credentials.ts` | ext `.ts` | 1186 bytes
3483. `integrations/n8n-nodes-base/credentials/MicrosoftSharePointOAuth2Api.credentials.ts` | ext `.ts` | 1280 bytes
3484. `integrations/n8n-nodes-base/credentials/MicrosoftSql.credentials.ts` | ext `.ts` | 2186 bytes
3485. `integrations/n8n-nodes-base/credentials/MicrosoftTeamsOAuth2Api.credentials.ts` | ext `.ts` | 1835 bytes
3486. `integrations/n8n-nodes-base/credentials/MicrosoftToDoOAuth2Api.credentials.ts` | ext `.ts` | 568 bytes
3487. `integrations/n8n-nodes-base/credentials/MindeeInvoiceApi.credentials.ts` | ext `.ts` | 1005 bytes
3488. `integrations/n8n-nodes-base/credentials/MindeeReceiptApi.credentials.ts` | ext `.ts` | 1005 bytes
3489. `integrations/n8n-nodes-base/credentials/MiroOAuth2Api.credentials.ts` | ext `.ts` | 1297 bytes
3490. `integrations/n8n-nodes-base/credentials/MispApi.credentials.ts` | ext `.ts` | 1159 bytes
3491. `integrations/n8n-nodes-base/credentials/MistApi.credentials.ts` | ext `.ts` | 1238 bytes
3492. `integrations/n8n-nodes-base/credentials/MoceanApi.credentials.ts` | ext `.ts` | 688 bytes
3493. `integrations/n8n-nodes-base/credentials/MondayComApi.credentials.ts` | ext `.ts` | 923 bytes
3494. `integrations/n8n-nodes-base/credentials/MondayComOAuth2Api.credentials.ts` | ext `.ts` | 1175 bytes
3495. `integrations/n8n-nodes-base/credentials/MongoDb.credentials.ts` | ext `.ts` | 3163 bytes
3496. `integrations/n8n-nodes-base/credentials/MonicaCrmApi.credentials.ts` | ext `.ts` | 925 bytes
3497. `integrations/n8n-nodes-base/credentials/Mqtt.credentials.ts` | ext `.ts` | 2921 bytes
3498. `integrations/n8n-nodes-base/credentials/Msg91Api.credentials.ts` | ext `.ts` | 430 bytes
3499. `integrations/n8n-nodes-base/credentials/MySql.credentials.ts` | ext `.ts` | 1898 bytes
3500. `integrations/n8n-nodes-base/credentials/N8nApi.credentials.ts` | ext `.ts` | 1033 bytes
3501. `integrations/n8n-nodes-base/credentials/NasaApi.credentials.ts` | ext `.ts` | 719 bytes
3502. `integrations/n8n-nodes-base/credentials/NetlifyApi.credentials.ts` | ext `.ts` | 749 bytes
3503. `integrations/n8n-nodes-base/credentials/NetscalerAdcApi.credentials.ts` | ext `.ts` | 1100 bytes
3504. `integrations/n8n-nodes-base/credentials/NextCloudApi.credentials.ts` | ext `.ts` | 1281 bytes
3505. `integrations/n8n-nodes-base/credentials/NextCloudOAuth2Api.credentials.ts` | ext `.ts` | 1309 bytes
3506. `integrations/n8n-nodes-base/credentials/NocoDb.credentials.ts` | ext `.ts` | 704 bytes
3507. `integrations/n8n-nodes-base/credentials/NocoDbApiToken.credentials.ts` | ext `.ts` | 938 bytes
3508. `integrations/n8n-nodes-base/credentials/NotionApi.credentials.ts` | ext `.ts` | 1260 bytes
3509. `integrations/n8n-nodes-base/credentials/NotionOAuth2Api.credentials.ts` | ext `.ts` | 1101 bytes
3510. `integrations/n8n-nodes-base/credentials/NpmApi.credentials.ts` | ext `.ts` | 881 bytes
3511. `integrations/n8n-nodes-base/credentials/OAuth1Api.credentials.ts` | ext `.ts` | 1387 bytes
3512. `integrations/n8n-nodes-base/credentials/OAuth2Api.credentials.ts` | ext `.ts` | 6441 bytes
3513. `integrations/n8n-nodes-base/credentials/OdooApi.credentials.ts` | ext `.ts` | 847 bytes
3514. `integrations/n8n-nodes-base/credentials/OktaApi.credentials.ts` | ext `.ts` | 1200 bytes
3515. `integrations/n8n-nodes-base/credentials/OneSimpleApi.credentials.ts` | ext `.ts` | 412 bytes
3516. `integrations/n8n-nodes-base/credentials/OnfleetApi.credentials.ts` | ext `.ts` | 657 bytes
3517. `integrations/n8n-nodes-base/credentials/OpenAiApi.credentials.ts` | ext `.ts` | 2356 bytes
3518. `integrations/n8n-nodes-base/credentials/OpenCTIApi.credentials.ts` | ext `.ts` | 812 bytes
3519. `integrations/n8n-nodes-base/credentials/OpenWeatherMapApi.credentials.ts` | ext `.ts` | 803 bytes
3520. `integrations/n8n-nodes-base/credentials/OracleDBApi.credentials.ts` | ext `.ts` | 5709 bytes
3521. `integrations/n8n-nodes-base/credentials/OrbitApi.credentials.ts` | ext `.ts` | 663 bytes
3522. `integrations/n8n-nodes-base/credentials/OuraApi.credentials.ts` | ext `.ts` | 780 bytes
3523. `integrations/n8n-nodes-base/credentials/PaddleApi.credentials.ts` | ext `.ts` | 593 bytes
3524. `integrations/n8n-nodes-base/credentials/PagerDutyApi.credentials.ts` | ext `.ts` | 408 bytes
3525. `integrations/n8n-nodes-base/credentials/PagerDutyOAuth2Api.credentials.ts` | ext `.ts` | 1078 bytes
3526. `integrations/n8n-nodes-base/credentials/PayPalApi.credentials.ts` | ext `.ts` | 732 bytes
3527. `integrations/n8n-nodes-base/credentials/PeekalinkApi.credentials.ts` | ext `.ts` | 822 bytes
3528. `integrations/n8n-nodes-base/credentials/PerplexityApi.credentials.ts` | ext `.ts` | 1138 bytes
3529. `integrations/n8n-nodes-base/credentials/PhantombusterApi.credentials.ts` | ext `.ts` | 790 bytes
3530. `integrations/n8n-nodes-base/credentials/PhilipsHueOAuth2Api.credentials.ts` | ext `.ts` | 1202 bytes
3531. `integrations/n8n-nodes-base/credentials/PipedriveApi.credentials.ts` | ext `.ts` | 751 bytes
3532. `integrations/n8n-nodes-base/credentials/PipedriveOAuth2Api.credentials.ts` | ext `.ts` | 1117 bytes
3533. `integrations/n8n-nodes-base/credentials/PlivoApi.credentials.ts` | ext `.ts` | 490 bytes
3534. `integrations/n8n-nodes-base/credentials/Postgres.credentials.ts` | ext `.ts` | 1805 bytes
3535. `integrations/n8n-nodes-base/credentials/PostHogApi.credentials.ts` | ext `.ts` | 902 bytes
3536. `integrations/n8n-nodes-base/credentials/PostmarkApi.credentials.ts` | ext `.ts` | 793 bytes
3537. `integrations/n8n-nodes-base/credentials/ProfitWellApi.credentials.ts` | ext `.ts` | 454 bytes
3538. `integrations/n8n-nodes-base/credentials/PushbulletOAuth2Api.credentials.ts` | ext `.ts` | 1112 bytes
3539. `integrations/n8n-nodes-base/credentials/PushcutApi.credentials.ts` | ext `.ts` | 396 bytes
3540. `integrations/n8n-nodes-base/credentials/PushoverApi.credentials.ts` | ext `.ts` | 1095 bytes
3541. `integrations/n8n-nodes-base/credentials/QRadarApi.credentials.ts` | ext `.ts` | 795 bytes
3542. `integrations/n8n-nodes-base/credentials/QualysApi.credentials.ts` | ext `.ts` | 1429 bytes
3543. `integrations/n8n-nodes-base/credentials/QuestDb.credentials.ts` | ext `.ts` | 1096 bytes
3544. `integrations/n8n-nodes-base/credentials/QuickBaseApi.credentials.ts` | ext `.ts` | 589 bytes
3545. `integrations/n8n-nodes-base/credentials/QuickBooksOAuth2Api.credentials.ts` | ext `.ts` | 1505 bytes
3546. `integrations/n8n-nodes-base/credentials/RabbitMQ.credentials.ts` | ext `.ts` | 3121 bytes
3547. `integrations/n8n-nodes-base/credentials/RaindropOAuth2Api.credentials.ts` | ext `.ts` | 1123 bytes
3548. `integrations/n8n-nodes-base/credentials/Rapid7InsightVmApi.credentials.ts` | ext `.ts` | 1235 bytes
3549. `integrations/n8n-nodes-base/credentials/RecordedFutureApi.credentials.ts` | ext `.ts` | 1073 bytes
3550. `integrations/n8n-nodes-base/credentials/RedditOAuth2Api.credentials.ts` | ext `.ts` | 1549 bytes
3551. `integrations/n8n-nodes-base/credentials/Redis.credentials.ts` | ext `.ts` | 1295 bytes
3552. `integrations/n8n-nodes-base/credentials/RocketchatApi.credentials.ts` | ext `.ts` | 1050 bytes
3553. `integrations/n8n-nodes-base/credentials/RundeckApi.credentials.ts` | ext `.ts` | 901 bytes
3554. `integrations/n8n-nodes-base/credentials/S3.credentials.ts` | ext `.ts` | 1045 bytes
3555. `integrations/n8n-nodes-base/credentials/SalesforceJwtApi.credentials.ts` | ext `.ts` | 3796 bytes
3556. `integrations/n8n-nodes-base/credentials/SalesforceOAuth2Api.credentials.ts` | ext `.ts` | 1661 bytes
3557. `integrations/n8n-nodes-base/credentials/SalesmateApi.credentials.ts` | ext `.ts` | 542 bytes
3558. `integrations/n8n-nodes-base/credentials/SeaTableApi.credentials.ts` | ext `.ts` | 1930 bytes
3559. `integrations/n8n-nodes-base/credentials/SecurityScorecardApi.credentials.ts` | ext `.ts` | 456 bytes
3560. `integrations/n8n-nodes-base/credentials/SegmentApi.credentials.ts` | ext `.ts` | 787 bytes
3561. `integrations/n8n-nodes-base/credentials/SekoiaApi.credentials.ts` | ext `.ts` | 808 bytes
3562. `integrations/n8n-nodes-base/credentials/SendGridApi.credentials.ts` | ext `.ts` | 750 bytes
3563. `integrations/n8n-nodes-base/credentials/SendyApi.credentials.ts` | ext `.ts` | 520 bytes
3564. `integrations/n8n-nodes-base/credentials/SentryIoApi.credentials.ts` | ext `.ts` | 756 bytes
3565. `integrations/n8n-nodes-base/credentials/SentryIoOAuth2Api.credentials.ts` | ext `.ts` | 1242 bytes
3566. `integrations/n8n-nodes-base/credentials/SentryIoServerApi.credentials.ts` | ext `.ts` | 907 bytes
3567. `integrations/n8n-nodes-base/credentials/ServiceNowBasicApi.credentials.ts` | ext `.ts` | 1283 bytes
3568. `integrations/n8n-nodes-base/credentials/ServiceNowOAuth2Api.credentials.ts` | ext `.ts` | 1600 bytes
3569. `integrations/n8n-nodes-base/credentials/Sftp.credentials.ts` | ext `.ts` | 1241 bytes
3570. `integrations/n8n-nodes-base/credentials/ShopifyAccessTokenApi.credentials.ts` | ext `.ts` | 1321 bytes
3571. `integrations/n8n-nodes-base/credentials/ShopifyApi.credentials.ts` | ext `.ts` | 1597 bytes
3572. `integrations/n8n-nodes-base/credentials/ShopifyOAuth2Api.credentials.ts` | ext `.ts` | 1997 bytes
3573. `integrations/n8n-nodes-base/credentials/ShufflerApi.credentials.ts` | ext `.ts` | 988 bytes
3574. `integrations/n8n-nodes-base/credentials/Signl4Api.credentials.ts` | ext `.ts` | 484 bytes
3575. `integrations/n8n-nodes-base/credentials/SlackApi.credentials.ts` | ext `.ts` | 1612 bytes
3576. `integrations/n8n-nodes-base/credentials/SlackOAuth2Api.credentials.ts` | ext `.ts` | 2959 bytes
3577. `integrations/n8n-nodes-base/credentials/Sms77Api.credentials.ts` | ext `.ts` | 1023 bytes
3578. `integrations/n8n-nodes-base/credentials/Smtp.credentials.ts` | ext `.ts` | 1167 bytes
3579. `integrations/n8n-nodes-base/credentials/Snowflake.credentials.ts` | ext `.ts` | 3064 bytes
3580. `integrations/n8n-nodes-base/credentials/SnowflakeOAuth2Api.credentials.ts` | ext `.ts` | 3423 bytes
3581. `integrations/n8n-nodes-base/credentials/SolarWindsIpamApi.credentials.ts` | ext `.ts` | 1938 bytes
3582. `integrations/n8n-nodes-base/credentials/SolarWindsObservabilityApi.credentials.ts` | ext `.ts` | 1581 bytes
3583. `integrations/n8n-nodes-base/credentials/SplunkApi.credentials.ts` | ext `.ts` | 1274 bytes
3584. `integrations/n8n-nodes-base/credentials/SpotifyOAuth2Api.credentials.ts` | ext `.ts` | 1472 bytes
3585. `integrations/n8n-nodes-base/credentials/SshPassword.credentials.ts` | ext `.ts` | 759 bytes
3586. `integrations/n8n-nodes-base/credentials/SshPrivateKey.credentials.ts` | ext `.ts` | 1016 bytes
3587. `integrations/n8n-nodes-base/credentials/StackbyApi.credentials.ts` | ext `.ts` | 396 bytes
3588. `integrations/n8n-nodes-base/credentials/StoryblokContentApi.credentials.ts` | ext `.ts` | 426 bytes
3589. `integrations/n8n-nodes-base/credentials/StoryblokManagementApi.credentials.ts` | ext `.ts` | 454 bytes
3590. `integrations/n8n-nodes-base/credentials/StrapiApi.credentials.ts` | ext `.ts` | 1223 bytes
3591. `integrations/n8n-nodes-base/credentials/StrapiTokenApi.credentials.ts` | ext `.ts` | 1589 bytes
3592. `integrations/n8n-nodes-base/credentials/StravaOAuth2Api.credentials.ts` | ext `.ts` | 2020 bytes
3593. `integrations/n8n-nodes-base/credentials/StripeApi.credentials.ts` | ext `.ts` | 1351 bytes
3594. `integrations/n8n-nodes-base/credentials/SupabaseApi.credentials.ts` | ext `.ts` | 1029 bytes
3595. `integrations/n8n-nodes-base/credentials/SurveyMonkeyApi.credentials.ts` | ext `.ts` | 946 bytes
3596. `integrations/n8n-nodes-base/credentials/SurveyMonkeyOAuth2Api.credentials.ts` | ext `.ts` | 1292 bytes
3597. `integrations/n8n-nodes-base/credentials/SyncroMspApi.credentials.ts` | ext `.ts` | 505 bytes
3598. `integrations/n8n-nodes-base/credentials/SysdigApi.credentials.ts` | ext `.ts` | 864 bytes
3599. `integrations/n8n-nodes-base/credentials/TaigaApi.credentials.ts` | ext `.ts` | 973 bytes
3600. `integrations/n8n-nodes-base/credentials/TapfiliateApi.credentials.ts` | ext `.ts` | 428 bytes

## Suggested Reading Order For The Next AI

1. Read `README.md` for the public promise.
2. Read `CURRENT_STATE.md` for the React/Next compatibility truth.
3. Read `docs/DX_WWW_FRAMEWORK_STRUCTURE.md` for folder and template contracts.
4. Read `docs/dx-www-developer-contract.md` for what contributors must preserve.
5. Read `docs/DX_WWW_MANAGER_HANDOFF.md` for current operational status.
6. Read `dx-www/src/cli/mod.rs` only as a map, then jump into smaller CLI modules.
7. Read `dx-www/src/cli/source_render.rs` to understand TSX support limits.
8. Read `core/src/ecosystem/forge_registry.rs` and related Forge files.
9. Read `core/src/ecosystem/dx_check_receipt.rs` and `dx-www/src/cli/project_check.rs` for check/receipt strategy.
10. Read `dx-www/src/cli/devtools` for visual editing and devtools wiring.
11. Read `examples/template` to see the current default starter.
12. Read richer examples only after understanding that they are not the default starter.

## Final Planning Position

DX-WWW has enough real infrastructure to justify serious next-feature planning.
The best path is not to imitate every framework at once.
The best path is to make DX-WWW unbeatable at the things it uniquely owns:

- source-owned routing;
- source-owned packages;
- source-owned style and icons;
- AI-readable receipts;
- fast dev server;
- precise devtools;
- transparent checks;
- template clarity;
- safe migration from React-shaped authoring into DX-native authoring.

The future planning AI should treat this dossier as the map of the current terrain.
The next features should make this terrain sharper, faster, and easier to trust.
