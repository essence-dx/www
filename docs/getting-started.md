# Getting Started With DX WWW

DX WWW is a React-familiar, source-owned web framework in the DX toolchain. The default app shape is an App Router-style `app/` tree, DX Style CSS output, Forge package receipts, and a Rust-owned `dx new -> dx dev -> dx build -> dx check -> receipts` workflow.

This guide describes the official public path. Legacy initializer commands, single-entry component layouts, alternate route trees, and opaque binary-format starter docs are not part of the WWW starter contract.

## Prerequisites

- Rust 1.85 or newer.
- Node.js 20 or newer for lightweight benchmark and guard scripts.
- A modern browser for local preview and devtools checks.

## Install

From this workspace:

```powershell
cargo build -j 6 -p dx-www --no-default-features --features cli --bin dx-www
```

Use the generated binary or the workspace wrapper already configured for your DX checkout. The starter itself does not require `npm install` or `node_modules`.

## Create A Project

```powershell
dx new my-app
cd my-app
```

The starter uses visible source files:

```text
app/layout.tsx
app/page.tsx
components/
server/
styles/globals.css
styles/theme.css
styles/generated.css
public/
dx
.dx/
```

`app/` owns routes, layouts, metadata, loading states, error states, and page
surfaces. Route handlers such as `app/api/**/route.ts`, DX-native state
surfaces, and island routes are framework capabilities, but the minimal starter
emits only the root page. Add routes only when the product needs them.
`components/`, `server/`, `lib/`, `styles/`, `content/`, `openapi/`, and
`public/` are the normal project-facing source folders. `.dx/` is tool-owned
state and evidence.

There is no hidden React runtime behind the starter: supported React-shaped
syntax lowers into DX-owned state, event, and island runtime behavior, while
unsupported React APIs must produce diagnostics or require an explicit adapter
boundary. Browser receipts still need to be captured and imported before
readiness can claim live runtime proof.

## Run Development

```powershell
dx dev --host 127.0.0.1 --port 3000
```

Open `http://127.0.0.1:3000/`.

In dev, DX WWW may inject dev-only tools and hot-reload/runtime helpers. Production `dx build` output must stay free of devtools endpoints and dev-only assets.

## Author A Route

Edit `app/page.tsx`:

```tsx
export default function Page() {
  return (
    <main className="mx-auto grid max-w-3xl gap-6 p-6">
      <section className="grid gap-3">
        <p className="text-sm font-medium text-neutral-500">DX WWW</p>
        <h1 className="text-4xl font-semibold text-neutral-950">
          Source-owned web app
        </h1>
        <p className="text-base text-neutral-700">
          React-shaped TSX authoring, DX-native compilation, and receipts you can inspect.
        </p>
      </section>
    </main>
  );
}
```

Use React-style event names such as `onClick` for supported DOM-native events.
Quoted event values are interaction-class commands:

```tsx
<button onClick="scale-up bg-accent">Save</button>
```

Braced event values are logic and lower only when the compiler can prove the
state/action path:

```tsx
<button onClick={() => counterStore.increment(counterStore)}>Add</button>
```

Unsupported React runtime APIs report diagnostics instead of pretending they
work.

## Styling

Use `styles/theme.css` for human-owned tokens and project styling.
`styles/generated.css` is tool-owned DX Style output and should not be
hand-edited. DX Style owns Tailwind-like `className` strings, event-class
strings, motion class strings, grouped tokens such as
`hover:(bg-accent text-accent-foreground)`, generated atomic utilities, and
authored custom CSS.

```powershell
dx style build
dx check . --json
```

DX Style and DX Icons are part of the wider DX ecosystem and should also work
in compatible non-WWW projects where the project contract allows it. Use
first-party icons through `<Icon name="pack:check" />` and the root
`icons(...)` config instead of starter-local npm icon packages.

## Build Production

```powershell
dx build
```

Production output is written under `.dx/www/output`. The deploy contract separates public runtime bytes from evidence artifacts:

- Public runtime: route HTML, CSS, immutable assets, needed runtime chunks.
- Evidence: receipts, manifests, proof graphs, deploy contracts, observability metadata.

Routes that prove `data-dx-output-mode="tiny-static"` and `data-dx-js="none"` do not publish a route packet as public runtime bytes; packet/evidence data stays out of the deployable public surface unless a route actually needs runtime support.

Do not edit `.dx/www/output` directly. Change source, then rebuild.

## Check Receipts

```powershell
dx check . --json
dx www readiness --json --full
dx www agent-context --json --full
dx www docs-doctor --json
dx www docs-doctor --json --write-receipt
```

The public verification path is `dx new -> dx dev -> dx build -> dx check -> receipts`: create source, run dev locally, build production output, run the check, then trust receipts written by those commands or by the explicit readiness/docs-doctor receipt writers. `dx check` is the readiness command for project status. It should report stale imports, stale style/icon/package receipts, unsupported framework surfaces, and release-readiness blockers. `dx www readiness --json --full` replays the release-readiness graph, readiness proof breakdown, supported surfaces, and remaining benchmark gates. Local route-handler matrices, hash-only server-action replay ledgers, and provider-adapter matrices are evidence only; provider-hosted route-handler replay, distributed server-action replay, and multi-adapter smoke proof receive their own receipts. `dx www agent-context --json --full` is the compact handoff for agents and automation and should point to current evidence, blockers, replay commands, and benchmark gates. `dx www docs-doctor --json` scans the public WWW docs for stale App Router, output-path, and proof claims before release notes or worker prompts repeat them. Add `--write-receipt` when you want a command-owned readiness replay receipt for that docs-doctor run.

Browser receipts use explicit import commands after a real browser or page-snapshot run has produced JSON evidence:

```powershell
dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full
dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full
dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full
dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full
```

Those imports validate real browser JSON before canonical JSON/SR/machine writes. They do not turn local receipts into hosted provider, CDN, Lighthouse, cross-browser, or release-binary proof.
State-runtime and island browser receipts come from materialized proof fixtures;
new starter apps begin with only `app/page.tsx`.

## Delivery Modes

DX WWW chooses the smallest verified delivery mode available for the route:

- Static/no-JS HTML and CSS when a route needs no browser runtime.
- Compiler-lowered DOM actions for small state and event surfaces.
- Source-owned client islands when interactivity needs lazy browser chunks.
- Explicit adapter-boundary islands for React, Svelte, or other framework runtimes when a project opts into them.

Static/no-JS verification is limited to the source and output contract until a JS-disabled browser receipt is imported. `tiny-static` markers, `data-dx-js="none"`, and no-JS artifact receipts are useful evidence, while hosted provider and cross-browser comparisons remain separate benchmark gates.

The public claim is not full React or Next.js runtime parity. The goal is React-familiar authoring with DX-native semantics, no unsupported hook APIs, and precise diagnostics for unsupported surfaces.

## Next Steps

- Read `docs/DX_WWW_FRAMEWORK_STRUCTURE.md` for the source layout contract.
- Read `docs/dx-www-developer-contract.md` for strict WWW project rules.
- Use `dx www readiness --json --full` before making readiness claims.
- Use `dx www agent-context --json --full` before handing work to other agents.
- Use `dx www docs-doctor --json` before making public readiness claims.
- Use `dx www docs-doctor --json --write-receipt` when readiness checks need docs-doctor command replay proof.
- Keep status precise: if a route, package, or primitive is preview-only, label it preview-only.
