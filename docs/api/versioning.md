# WWW Public API Stability

This document defines the public API stability contract for `dx-www`. It is the
release-facing source of truth for what outside developers can build on without
depending on internal implementation details.

## Stability Goals

WWW is a source-owned web framework with a React/Next-familiar authoring model.
The public contract is intentionally smaller than the internal implementation:
developers should be able to learn the stable surface quickly, and advanced
tools should be able to verify that surface through `.dx` receipts.

The stable contract is:

- clear command names,
- clear project folders,
- clear TSX authoring syntax,
- clear DX-native state semantics,
- clear generated output and receipt locations,
- clear compatibility boundaries.

## Versioning Policy

WWW follows Semantic Versioning for stable public APIs.

```text
MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
```

- `MAJOR`: incompatible public API change.
- `MINOR`: backwards-compatible public feature addition.
- `PATCH`: backwards-compatible bug fix, performance fix, or documentation fix.
- `PRERELEASE`: release candidate, preview, or experimental build.

All workspace crates should move together when the framework release moves. That
keeps compiler, CLI, state, style, icons, Forge, and receipt contracts aligned.

## Stable Public Surfaces

These surfaces are public and must remain stable across patch releases:

- Commands: `dx new`, `dx dev`, `dx build`, `dx check`,
  `dx www agent-context`, `dx style`, `dx icons`, `dx imports`, and
  `dx env`.
- Project folders: `app/`, `components/`, `composables/`, `utils/`, `lib/`,
  `lib/stores/`, `server/`, `styles/`, `public/`, `.dx/`.
- Config: the extensionless root `dx` file.
- Route authoring: `app/**/page.tsx`, `app/**/layout.tsx`,
  `app/api/**/route.ts`, metadata-like exports, and route params.
- TSX syntax: intrinsic elements, props, `className`, React-style DOM event
  names such as `onClick`, `onInput`, `onPointerMove`, and camelCase island
  directives such as `clientLoad`, `clientVisible`, `clientIdle`, `clientOnly`.
- Runtime state: DX-native `state`, `derived`, `effect`, `action`, and
  framework-owned global stores under `lib/stores/`.
- Event semantics: quoted event values such as `onClick="scale-up bg-accent"`
  are interaction-class commands; braced event values are logic that must lower
  safely or fail with a diagnostic.
- Styling: DX Style-owned `className`, event-class strings, motion class
  strings, grouped tokens such as `hover:(bg-accent text-accent-foreground)`,
  generated atomic utilities, and authored custom CSS.
- Assets and primitives: source-owned image, font, script, wasm, DX Style, and
  first-party DX Icon behavior through `<Icon name="pack:check" />`.
- Proof surfaces: `.dx/receipts/**`, `.sr` serializer receipts, `.machine`
  contracts, `.dx/www/output`, and `dx www agent-context --json`.
- Devtools contract: dev-only injection and dev-only endpoints under
  `/_dx/devtools/*`; production `dx build` output must stay Devtools-free.

## Compatibility Boundaries

WWW is React/Next-familiar in authoring. It is not a hidden React DOM runtime by
default.

React-style source is accepted when the compiler can lower it into WWW's
source-owned runtime. React hook-like APIs such as `useState`, `useEffect`,
`useMemo`, `useRef`, and `useContext` are not independent public runtime APIs.
They must either:

1. lower exactly to DX-native state/effect/action semantics, or
2. fail with a precise diagnostic that explains the unsupported pattern.

Silent no-op compatibility APIs are not allowed.

Full React, Next.js, Svelte, or other framework runtimes belong behind explicit
adapter or island boundaries. Those adapters are public only when their contract
documents the runtime requirement, generated files, receipts, and production
payload impact.

## Stability Tiers

### Stable

Stable APIs are documented, covered by checks or tests, and supported for
production apps.

Examples:

- project folder names,
- command names and major flags,
- extensionless `dx` config,
- DX-native state primitives,
- Devtools dev-only boundary,
- `.dx` receipt locations and schema names.

Breaking a stable API requires a major release or a documented migration gate.

### Preview

Preview APIs are usable but can still change with clear release notes and
migration guidance.

Examples:

- new adapter-boundary islands,
- new provider deployment adapters,
- new Devtools editing panels,
- new Forge package materialization flows.

Preview APIs must not be marketed as stable unless their receipts and docs are
current.

### Internal

Internal APIs are implementation details. They can move without a public
migration window.

Examples:

- private Rust modules,
- generated helper structs that are not documented,
- fixture-only routes,
- benchmark harness internals,
- compatibility probes under test fixtures.

Internal paths should not be imported by user apps.

## Breaking Change Definition

The following are breaking changes for stable public APIs:

- removing or renaming a stable command,
- removing a stable project folder convention,
- changing the extensionless `dx` config format incompatibly,
- changing documented TSX event or island directive semantics,
- changing DX-native state semantics incompatibly,
- moving public generated output or receipt paths without migration,
- shipping Devtools code in production output,
- turning a documented no-JS route into a JS-required route without a source
  reason and receipt.

The following are not breaking changes:

- adding a new command or flag,
- adding a new route primitive,
- adding a new receipt field,
- improving performance without changing documented behavior,
- fixing behavior that contradicted the documented contract,
- strengthening diagnostics for unsupported React compatibility patterns.

## Deprecation Process

Stable public APIs use this deprecation process:

1. Document the replacement in release notes and this docs set.
2. Keep the old API working for at least one minor release unless it is a
   security or data-safety issue.
3. Emit an actionable diagnostic or check warning when the old API is used.
4. Provide a migration command, receipt, or explicit manual migration step.
5. Remove the old API only in a major release or after the documented migration
   window closes.

Rust items that are public crate APIs should use `#[deprecated]` with a
replacement note. CLI and TSX-facing features should use diagnostics that name
the source file, the unsupported API, and the replacement.

## Public Proof Requirements

Claims about public behavior should be tied to evidence:

- `dx check . --json` for project health,
- `dx build` for production output,
- `dx www agent-context --json --full` for agent handoff,
- `.dx/receipts/**` for machine-readable proof,
- `.sr` and `.machine` artifacts for durable generated state,
- browser/import receipts for browser-only claims,
- hosted/provider receipts for hosted/provider claims.

If proof is missing, the public API should say what is supported locally and
what requires additional provider or browser evidence.

## See Also

- [Developer contract](../dx-www-developer-contract.md)
- [Framework structure](../DX_WWW_FRAMEWORK_STRUCTURE.md)
- [Getting started](../getting-started.md)
- [Benchmark methodology](../benchmarks.md)
