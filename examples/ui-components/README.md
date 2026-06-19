# UI Components for DX WWW

Source-owned Forge UI Components for the DX ecosystem.

This example was created with `dx new ui-components`, then populated with a
WWW-native registry-shaped component surface. Source-owned components live as
first-party `components/ui/*`; deeper browser-engine behaviors stay marked as
adapter boundaries until their WWW runtime receipts exist.

The first Forge receipts cover the baseline source-owned primitives and DX Icon
package. The remaining registry files are represented as first-party
`components/ui/*` source. Components that need deeper browser engines, such as
dialogs, menus, popovers, command palettes, carousel, charts, and toaster
behavior, are marked with adapter boundaries instead of importing npm packages.

The upstream shadcn-ui mirror is recorded in
`lib/ui-components/provenance.ts`. Front-facing DX naming is **UI Components**;
`shadcn-ui` remains provenance.

## Framework Contract

- No `node_modules` folder is required for this project.
- DX Style owns Tailwind-like class authoring, grouped utility syntax,
  event-class strings, and generated CSS; do not hand-edit generated output.
- DX Icon is the first-party icon path through `<Icon />`, `icons(...)`, and
  generated icon receipts.
- Stateful or interactive primitives should use WWW-native state/action/event
  semantics or an explicit adapter boundary; do not hide React DOM package
  runtime behind component demos.
- `dx check --json` is the project-health proof surface for this example.
