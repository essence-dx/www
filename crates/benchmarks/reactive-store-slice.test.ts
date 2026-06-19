const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function readMaybe(relativePath) {
  const fullPath = path.join(root, relativePath);
  return fs.existsSync(fullPath) ? fs.readFileSync(fullPath, "utf8") : null;
}

function read(relativePath) {
  const source = readMaybe(relativePath);
  assert.notEqual(source, null, `${relativePath} should exist`);
  return source;
}

function forgeRegistrySource() {
  return [
    "core/src/ecosystem/forge_registry.rs",
    "core/src/ecosystem/forge_registry_parts/registry_operations.rs",
    "core/src/ecosystem/forge_registry_parts/package_lanes.rs",
  ]
    .map(read)
    .join("\n");
}

test("Reactive Store Forge slice exposes upstream-derived core and React surfaces", () => {
  const forge = read("core/src/ecosystem/forge_reactive_store.rs");

  assert.match(forge, /REACTIVE_STORE_VERSION: &str = "0\.11\.0-dx\.1"/);
  assert.match(forge, /pub\(super\) fn reactive_store_templates\(\)/);
  assert.match(forge, /"js\/state\/reactive-store\/index\.ts"/);
  assert.match(forge, /"js\/state\/reactive-store\/types\.ts"/);
  assert.match(forge, /"js\/state\/reactive-store\/atom\.ts"/);
  assert.match(forge, /"js\/state\/reactive-store\/store\.ts"/);
  assert.match(forge, /"js\/state\/reactive-store\/shallow\.ts"/);
  assert.match(forge, /"js\/state\/reactive-store\/react\.ts"/);
  assert.match(forge, /"js\/state\/reactive-store\/context\.tsx"/);
  assert.match(forge, /"js\/state\/reactive-store\/metadata\.ts"/);
  assert.match(forge, /"js\/state\/reactive-store\/README\.md"/);

  assert.match(forge, /export interface Subscription/);
  assert.match(forge, /export type Observer<T>/);
  assert.match(forge, /export interface Atom<T>/);
  assert.match(forge, /export interface ReadonlyAtom<T>/);
  assert.match(forge, /export function batch/);
  assert.match(forge, /export function flush/);
  assert.match(forge, /export function createAtom<T>/);
  assert.match(forge, /export function createAsyncAtom<T>/);
  assert.match(forge, /export class Store<T, TActions extends StoreActionMap = never>/);
  assert.match(forge, /export class ReadonlyStore<T>/);
  assert.match(forge, /export function createStore<T>/);
  assert.match(forge, /StoreActionsFactory<T, TActions>/);
  assert.match(forge, /setState\(updater: \(prev: T\) => T\)/);
  assert.match(
    forge,
    /subscribe\(\s*observerOrFn: Observer<T> \| \(\(value: T\) => void\),\s*\): Subscription/,
  );
  assert.match(forge, /export function shallow<T>/);
  assert.match(forge, /export interface UseSelectorOptions<TSelected>/);
  assert.match(forge, /export function useSelector<TSource, TSelected = NoInfer<TSource>>/);
  assert.match(forge, /React\.useSyncExternalStore/);
  assert.match(forge, /export function useAtom<TValue>/);
  assert.match(forge, /export const useStore = </);
  assert.match(forge, /export function createStoreContext<TValue extends object>\(\)/);
  assert.match(forge, /createContext<TValue \| null>\(null\)/);
  assert.match(forge, /Context\.displayName = "StoreContext"/);
  assert.match(forge, /throw new Error\("Missing StoreProvider for StoreContext"\)/);
  assert.match(forge, /officialName: "Reactive Store"/);
  assert.match(forge, /packageId: "reactive\/store"/);
  assert.match(forge, /upstreamPackage: "@tanstack\/store"/);
  assert.match(forge, /reactUpstreamPackage: "@tanstack\/react-store"/);
  assert.match(forge, /sourceMirror: "G:\\\\WWW\\\\inspirations\\\\tanstack-store"/);
  assert.match(forge, /dxVersion: "0\.11\.0-dx\.1"/);
  assert.match(forge, /inspectedSourceFiles: \[/);
  assert.match(forge, /packages\/store\/src\/store\.ts/);
  assert.match(forge, /packages\/store\/src\/atom\.ts/);
  assert.match(forge, /packages\/react-store\/src\/useSelector\.ts/);
  assert.match(forge, /packages\/react-store\/src\/createStoreContext\.tsx/);
  assert.match(forge, /statusLabels: \["present", "stale", "missing-receipt", "blocked", "unsupported-surface"\]/);
  assert.match(forge, /dxStyleCompatibility/);
  assert.match(forge, /zedSourceMarkers/);
});

test("Reactive Store is registered with official naming and upstream provenance", () => {
  const registry = forgeRegistrySource();
  const mod = read("core/src/ecosystem/mod.rs");
  const catalog = read("examples/onboard/package-catalog.ts");
  const docs = read("docs/packages/reactive-store.md");
  const framework = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const reactiveStoreCatalog = catalog.match(
    /packageId: "reactive\/store"[\s\S]*?appOwnedBoundaries: \[[\s\S]*?\],\n  \},/,
  )?.[0];
  assert.ok(reactiveStoreCatalog, "Reactive Store catalog entry should be present");

  assert.match(mod, /mod forge_reactive_store;/);
  assert.match(registry, /use super::forge_reactive_store::\{REACTIVE_STORE_VERSION, reactive_store_templates\};/);
  assert.match(
    registry,
    /"reactive-store"\s*\| "@tanstack\/store"\s*\| "@tanstack\/react-store"\s*\| "tanstack-store"\s*=>\s*\{\s*"reactive\/store"\s*\}/,
  );
  assert.match(registry, /"reactive\/store" => reactive_store_registry_package\(\)/);
  assert.match(registry, /"Source-owned Reactive Store slice/);
  assert.match(registry, /registry_package\("reactive\/store"\)\?/);
  assert.match(registry, /"reactive\/store" => "@tanstack\/store"/);
  assert.match(registry, /"reactive\/store" => "dx-forge\/reactive-store"/);
  assert.match(registry, /upstream_reference: Some\("npm:@tanstack\/store@0\.11\.0; npm:@tanstack\/react-store@0\.11\.0"/);
  assert.match(registry, /Curated DX Forge advisory fixture records no known advisory findings for this Reactive Store slice/);

  assert.match(reactiveStoreCatalog, /packageId: "reactive\/store"/);
  assert.match(reactiveStoreCatalog, /aliases: \[\s*"reactive-store"/);
  assert.match(reactiveStoreCatalog, /role: "launch-state"/);
  assert.match(reactiveStoreCatalog, /command: "dx add reactive-store --write"/);
  assert.match(
    reactiveStoreCatalog,
    /sourceMirror: "G:\/WWW\/inspirations\/tanstack-store"/,
  );
  assert.match(reactiveStoreCatalog, /officialPackageName: "Reactive Store"/);
  assert.match(reactiveStoreCatalog, /upstream_package: "@tanstack\/store"/);
  assert.match(reactiveStoreCatalog, /based_on: "@tanstack\/react-store"/);
  assert.match(reactiveStoreCatalog, /"lib\/forge\/state\/reactive-store\/store\.ts"/);
  assert.match(reactiveStoreCatalog, /"lib\/forge\/state\/reactive-store\/react\.ts"/);
  assert.match(reactiveStoreCatalog, /"lib\/forge\/state\/reactive-store\/context\.tsx"/);
  assert.match(reactiveStoreCatalog, /"docs\/packages\/reactive-store\.md"/);
  assert.match(
    reactiveStoreCatalog,
    /"examples\/template\/reactive-store-receipt-hashes\.ts"/,
  );
  assert.match(
    reactiveStoreCatalog,
    /schema: "dx\.forge\.package\.dx_check_visibility"/,
  );
  assert.match(reactiveStoreCatalog, /"source-guard-runbook-fixture"/);
  assert.match(reactiveStoreCatalog, /"preview-manifest-materializer"/);
  assert.match(reactiveStoreCatalog, /receiptIntegrity: \{/);
  assert.match(reactiveStoreCatalog, /hashAlgorithm: "sha256"/);
  assert.match(
    reactiveStoreCatalog,
    /"docs\/packages\/reactive-store\.source-guard-runbook\.json"/,
  );
  assert.match(
    reactiveStoreCatalog,
    /"tools\/launch\/materialize-www-template\.ts"/,
  );
  assert.match(reactiveStoreCatalog, /reactive-store:receipt-hash-refresh/);
  assert.match(reactiveStoreCatalog, /dxIcon: "state:reactive-store"/);

  assert.match(docs, /^# Reactive Store/m);
  assert.match(docs, /Official DX package name: `Reactive Store`/);
  assert.match(docs, /Package id: `reactive\/store`/);
  assert.match(docs, /upstream_package: `@tanstack\/store`/);
  assert.match(docs, /source_mirror: `G:\/WWW\/inspirations\/tanstack-store`/);
  assert.match(docs, /Honesty label: `SOURCE-OWNED TEMPLATE STORE`/);
  assert.match(docs, /`createStore`/);
  assert.match(docs, /`createAtom`/);
  assert.match(docs, /`batch`/);
  assert.match(docs, /`useSelector`/);
  assert.match(docs, /dx-check visibility/);
  assert.match(docs, /present, stale, missing receipt, blocked, unsupported surface/);
  assert.match(framework, /state, reactive store, i18n/);
});

test("Reactive Store registry exposes selective surface installs", () => {
  const registry = forgeRegistrySource();
  const catalog = read("examples/onboard/package-catalog.ts");
  const docs = read("docs/packages/reactive-store.md");

  assert.match(registry, /fn reactive_store_registry_package\(\) -> DxForgeRegistryPackage/);
  assert.match(registry, /"reactive\/store" => reactive_store_registry_package\(\)/);
  assert.match(registry, /package\.allow_selective_imports = true;/);
  assert.match(registry, /package\.default_exports = vec!\["full"\.to_string\(\)\];/);
  assert.match(registry, /reactive_store_export\(\s*"full"/);
  assert.match(registry, /reactive_store_export\(\s*"core-store"/);
  assert.match(registry, /reactive_store_export\(\s*"atom-graph"/);
  assert.match(registry, /reactive_store_export\(\s*"comparison-helper"/);
  assert.match(registry, /reactive_store_export\(\s*"react-selector"/);
  assert.match(registry, /reactive_store_export\(\s*"react-context"/);

  const coreStoreExport = registry.match(
    /reactive_store_export\(\s*"core-store",\s*&\[(?<files>[\s\S]*?)\]\s*,\s*\)/,
  )?.groups.files;
  assert.ok(coreStoreExport, "core-store export should be declared");
  assert.match(coreStoreExport, /"js\/state\/reactive-store\/store\.ts"/);
  assert.match(coreStoreExport, /"js\/state\/reactive-store\/atom\.ts"/);
  assert.match(coreStoreExport, /"js\/state\/reactive-store\/types\.ts"/);
  assert.doesNotMatch(coreStoreExport, /"js\/state\/reactive-store\/react\.ts"/);

  const reactSelectorExport = registry.match(
    /reactive_store_export\(\s*"react-selector",\s*&\[(?<files>[\s\S]*?)\]\s*,\s*\)/,
  )?.groups.files;
  assert.ok(reactSelectorExport, "react-selector export should be declared");
  assert.match(reactSelectorExport, /"js\/state\/reactive-store\/react\.ts"/);
  assert.match(reactSelectorExport, /"js\/state\/reactive-store\/store\.ts"/);
  assert.match(reactSelectorExport, /"js\/state\/reactive-store\/atom\.ts"/);

  const reactContextExport = registry.match(
    /reactive_store_export\(\s*"react-context",\s*&\[(?<files>[\s\S]*?)\]\s*,\s*\)/,
  )?.groups.files;
  assert.ok(reactContextExport, "react-context export should be declared");
  assert.match(reactContextExport, /"js\/state\/reactive-store\/context\.tsx"/);
  assert.match(reactContextExport, /"js\/state\/reactive-store\/metadata\.ts"/);
  assert.doesNotMatch(reactContextExport, /"js\/state\/reactive-store\/store\.ts"/);
  assert.doesNotMatch(reactContextExport, /"js\/state\/reactive-store\/atom\.ts"/);

  assert.match(catalog, /selectiveSurfaceCommands: \[/);
  assert.match(catalog, /"dx forge add reactive\/store#core-store --write"/);
  assert.match(catalog, /"dx forge add reactive\/store#atom-graph --write"/);
  assert.match(catalog, /"dx forge add reactive\/store#comparison-helper --write"/);
  assert.match(catalog, /"dx forge add reactive\/store#react-selector --write"/);
  assert.match(catalog, /"dx forge add reactive\/store#react-context --write"/);
  assert.match(catalog, /selectedSurfaces: \["core-store", "atom-graph", "comparison-helper", "react-selector", "react-context"\]/);

  assert.match(docs, /Selective Surface Installs/);
  assert.match(docs, /`dx forge add reactive\/store#core-store --write`/);
  assert.match(docs, /`dx forge add reactive\/store#atom-graph --write`/);
  assert.match(docs, /`dx forge add reactive\/store#comparison-helper --write`/);
  assert.match(docs, /`dx forge add reactive\/store#react-selector --write`/);
  assert.match(docs, /`dx forge add reactive\/store#react-context --write`/);
  assert.match(docs, /`createStoreContext`/);
});
