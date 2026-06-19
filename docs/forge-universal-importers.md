# Forge Universal Importers

Forge universal importers are source-review surfaces for external ecosystems.
Their job is to describe how package source can be reviewed and sliced into
Forge-owned source, then materialize inspected source directories when the
review gate accepts them. They do not make package-manager installs the
authority for a project.

Universal here means one shared review firewall for the currently modeled
ecosystems: npm, pip, cargo, Go, JSR, pub, Maven, NuGet, Composer, RubyGems,
SwiftPM, Hex, and CRAN. It is not universal package compatibility, automatic
install execution, or a package-manager replacement. Live acquisition currently
exists only for npm through the explicit non-executing `dx forge acquire npm`
path.

The CLI accepts common registry aliases for operator convenience, but receipts,
source paths, and machine artifacts keep canonical Forge ecosystem segments:
`npm`, `pip`, `cargo`, `go`, `jsr`, `pub`, `maven`, `nuget`, `composer`, `gem`,
`swift`, `hex`, and `cran`. For example, `pypi` normalizes to `pip`,
`crates.io` normalizes to `cargo`, `golang` normalizes to `go`, `pub.dev`
normalizes to `pub`, `packagist` normalizes to `composer`, `rubygems`
normalizes to `gem`, and `dotnet` normalizes to `nuget`.

Package identities are validated before they become package ids, receipt names,
or materialized source paths. npm and JSR accept normal package names and scoped
`@scope/name` names; pip, NuGet, RubyGems, pub, Hex, and CRAN use their
registry-shaped identifiers; Maven accepts `group:artifact` or `group/artifact`;
Composer requires `vendor/package`; Go and Swift accept path-shaped module
identities. URLs, whitespace, traversal, shell metacharacters, and package-manager
commands are rejected before Forge writes receipts.

## Shared Rules

- Default import planning is non-executing.
- Quarantine is not importable app source.
- Live fetching stays behind explicit acquisition commands and receipts; npm is
  the first implemented live fetcher.
- Materialization requires an accepted import receipt.
- Import rewrites require accepted materialization receipts.
- A package can only claim a 100/100 Forge import score when provenance,
  integrity, SBOM, license review, advisory coverage, and no-install receipts
  are all explicit; declaration-only marker files stay below green.
- npm live acquisition is available through `dx forge acquire npm <package>`;
  `dx forge add npm <package>` is a compatibility alias for the same
  no-install acquisition path.
- `dx add pip/foo`, `dx add cargo/foo`, `dx add go/foo`, `dx add jsr/foo`,
  `dx add pub/foo`, `dx add maven/foo`, `dx add nuget/foo`,
  `dx add composer/foo`, `dx add gem/foo`, `dx add swift/foo`,
  `dx add hex/foo`, and `dx add cran/foo` remain unsupported unless an accepted
  import receipt exists for that package.

## npm

Acquisition support downloads npm package tarballs through the registry
packument, verifies declared `sha512` integrity when present, extracts only the
package-root source tree into `.dx/cache/npm/<package>/package`, and records
source-owned evidence without creating an install tree or running lifecycle
scripts. Import write support can then materialize the inspected source
directory, for example `dx forge import npm lodash --write --source-dir
.dx/cache/npm/lodash/package`.

Plan inputs:

- npm registry packument
- `package.json`
- `exports` map
- declared license and advisory metadata
- package tarball, tarball integrity metadata, and file list

Manual-review triggers:

- preinstall, install, or postinstall lifecycle script
- native gyp or prebuild artifact
- dynamic require or import expression
- obfuscated or minified runtime blob
- large unreviewed dependency graph
- missing license or integrity declaration

Forbidden default commands: `npm install`, `pnpm install`, `yarn install`, and
`bun install`.

## pip

Plan inputs:

- PyPI JSON metadata
- wheel `METADATA` and `RECORD`
- sdist `PKG-INFO`
- `pyproject.toml` declarations
- wheel and source distribution archives

Manual-review triggers:

- `setup.py` execution requirement
- native extension or binary wheel
- dynamic version or build backend execution
- console script entrypoint side effects
- missing license or integrity declaration

Forbidden default commands: `pip install`, `uv pip install`, and
`python setup.py`.

## cargo

Plan inputs:

- crates.io metadata
- `Cargo.toml`
- `Cargo.lock` metadata
- crate checksum declaration
- crate archive and file manifest

Manual-review triggers:

- `build.rs` script
- proc-macro crate
- native links or bindgen requirement
- git dependency
- unreviewed unsafe or FFI-heavy surface
- missing license or integrity declaration

Forbidden default commands: `cargo add`, `cargo build`, and `cargo install`.

## go

Plan inputs:

- Go module proxy metadata
- `go.mod`
- `go.sum` metadata
- module version declaration
- module zip and file list

Manual-review triggers:

- `go generate` directive
- cgo requirement
- `replace` directive requiring local context
- private module source
- vendored binary asset
- missing license or integrity declaration

Forbidden default commands: `go get`, `go install`, and `go generate`.

## jsr

Plan inputs:

- JSR package metadata
- `jsr.json` or `deno.json`
- exports metadata
- source archive, module source files, and source integrity metadata

Manual-review triggers:

- Deno permission or unstable API boundary
- `deno task` or generator requirement
- dynamic import expression
- npm compatibility bridge requirement
- missing license or integrity declaration

Forbidden default commands: `deno add`, `deno install`, `deno task`, and
`jsr add`.

## pub

Plan inputs:

- pub.dev package metadata
- `pubspec.yaml`
- `pubspec.lock` metadata
- package archive and file list

Manual-review triggers:

- `build_runner` or generated source requirement
- Flutter plugin native platform channel
- FFI or native asset requirement
- missing license or integrity declaration

Forbidden default commands: `dart pub get`, `flutter pub get`, and
`dart run build_runner`.

## maven

Plan inputs:

- Maven Central metadata
- `pom.xml`
- Gradle module metadata
- source jar archive and checksum metadata

Manual-review triggers:

- Maven plugin execution
- Gradle build script execution
- JNI or native library requirement
- annotation processor or code generator requirement
- missing license or integrity declaration

Forbidden default commands: `mvn install`, `mvn package`, `gradle build`, and
`gradle publish`.

## nuget

Plan inputs:

- NuGet registration metadata
- `.nuspec` metadata
- project file metadata
- `nupkg` archive and source package metadata

Manual-review triggers:

- MSBuild target or props execution
- native runtime asset
- source generator or analyzer execution
- missing license or integrity declaration

Forbidden default commands: `dotnet restore`, `dotnet build`, and
`nuget install`.

## composer

Plan inputs:

- Packagist metadata
- `composer.json`
- `composer.lock` metadata
- package archive and source file list

Manual-review triggers:

- Composer script execution
- PHP extension or native module requirement
- autoload side-effect boundary
- missing license or integrity declaration

Forbidden default commands: `composer install`, `composer update`, and
`php artisan`.

## gem

Plan inputs:

- RubyGems metadata
- `.gemspec`
- `Gemfile.lock` metadata
- gem archive and source file list

Manual-review triggers:

- native extension build
- Rake task or install hook
- dynamic require boundary
- missing license or integrity declaration

Forbidden default commands: `gem install`, `bundle install`, and `rake`.

## swift

Plan inputs:

- Swift Package Index metadata
- `Package.swift`
- `Package.resolved` metadata
- source archive and checksum metadata

Manual-review triggers:

- Swift package plugin execution
- system library or binary target
- macro plugin or generated source requirement
- missing license or integrity declaration

Forbidden default commands: `swift package resolve`, `swift build`, and
`swift test`.

## hex

Plan inputs:

- Hex package metadata
- `mix.exs`
- `rebar.config` metadata
- `mix.lock` metadata
- package tarball, source archive, and package file list

Manual-review triggers:

- Mix compiler or generated source requirement
- Rebar hook or build plugin requirement
- NIF or port-driver native code
- private Hex organization dependency
- missing license or integrity declaration

Forbidden default commands: `mix deps.get`, `mix compile`,
`mix archive.install`, and `rebar3 compile`.

## cran

Plan inputs:

- CRAN metadata
- `DESCRIPTION`
- `NAMESPACE`
- source package tarball and file list

Manual-review triggers:

- configure or cleanup script
- compiled native code in `src/`
- R package installation hook
- missing license or integrity declaration

Forbidden default commands: `R CMD INSTALL`, `install.packages`,
`pak::pkg_install`, and `renv::restore`.

## Claim Boundary

These importers do not support every package from every ecosystem. They are
source-review contracts with inspected-source materialization. npm is the first
live acquisition path, and it still only records reviewed source evidence; other
ecosystems remain plan/materialization gated until their explicit non-executing
fetchers exist. A green plan means Forge knows what it would inspect and which
gates would block materialization. A green `--write --source-dir <path>` receipt
means Forge copied reviewed source files into app-owned Forge paths and recorded
provenance. If source files are present but the accepted materialization receipt
is missing, Forge reports the boundary as incomplete instead of source-owned. No
mode runs package-manager installs, package lifecycle scripts, or package build
code automatically.
