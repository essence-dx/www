# DX WWW CLI

`dx-www-cli` exposes the `dx` command for creating, developing, checking, and
building DX WWW projects.

The current WWW project contract is App Router-shaped TSX authoring with an
extensionless `dx` config, source-owned build/dev/check tooling, dx-style
generated CSS, DX icon/import tooling, and `.dx/*` receipts. Older project
formats can still exist for migration, but they are not the starter app story.

## Common Commands

```bash
dx create my-app
dx dev
dx check . --json
dx build
```

## Current Project Shape

```text
my-app/
  dx
  app/
  components/
  server/
  styles/
  public/
  .dx/
```

## Configuration

New projects use the root `dx` file. Legacy TOML config may still be readable
for migration, but new docs and templates should prefer the extensionless
serializer-friendly config.

```dx
project(name=my-app version=0.1.0 kind=www-app)

www(app_dir=app output_dir=.dx/www/output)

dev(host=127.0.0.1 port=3000 hot_reload=true devtools=true)

style(
  mode=generated-css
  tokens=styles/theme.css
  generated_css=styles/generated.css
)

forge(policy=forge-first-no-node-modules)
check(score_scale=500 lighthouse=true)
```

## CLI Scope

- `dx dev`: local development server, hot reload, and dev-only tooling.
- `dx build`: source-owned production output and receipts.
- `dx check`: project contract, style/icon/import/build evidence.
- `dx style`: generated CSS build/check.
- `dx icons`: source-owned icon sync/check.
- `dx imports`: visible auto-import map sync/check.

The CLI should describe real framework behavior. Compatibility paths and legacy
artifacts should be named as compatibility paths, not as the public product
contract.
