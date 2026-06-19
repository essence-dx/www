const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Zustand Forge slice exposes real middleware and selector APIs", () => {
  const forge = read("core/src/ecosystem/forge_zustand.rs");
  const counter = read("examples/template/state-zustand-counter.tsx");
  const dashboardControl = read("examples/template/state-zustand-dashboard.tsx");
  const catalog = read("examples/template/package-catalog.ts");
  const dashboardReceipt = JSON.parse(
    read("examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json"),
  );
  const launchRoute = read("examples/template/app/page.tsx");
  const launchShell = read("examples/template/template-shell.tsx");
  const routeContract = read("examples/template/template-route-contract.ts");
  const studioContract = read("examples/template/dx-studio-edit-contract.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const runtimePage = read("tools/launch/runtime-template/pages/index.html");
  const runtimeJs = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const dashboardSettings = read("examples/dashboard/src/pages/Settings.tsx");
  const dashboardPanel = read("examples/dashboard/src/components/ZustandSettingsPanel.tsx");
  const dashboardStore = read("examples/dashboard/src/lib/dashboardSettingsStore.ts");
  const dashboardForge = read("examples/dashboard/src/lib/forge/state/zustand.ts");
  const shadcnDashboardControls = read("examples/template/shadcn-dashboard-controls.tsx");
  const packageDocs = read("docs/packages/state-zustand.md");
  const cli = read("dx-www/src/cli/mod.rs");
  const dx = read("DX.md");
  const changelog = read("CHANGELOG.md");

  assert.match(forge, /ZUSTAND_VERSION: &str = "5\.0\.13-dx\.10"/);
  assert.match(forge, /"js\/state\/zustand\/traditional\.ts"/);
  assert.match(forge, /"js\/state\/zustand\/redux\.ts"/);
  assert.match(forge, /"js\/state\/zustand\/ssr-safe\.ts"/);
  assert.match(forge, /"js\/state\/zustand\/devtools\.ts"/);
  assert.match(forge, /"js\/state\/zustand\/immer\.ts"/);
  assert.match(forge, /const ZUSTAND_TRADITIONAL_TS: &str = r#"/);
  assert.match(forge, /const ZUSTAND_REDUX_TS: &str = r#"/);
  assert.match(forge, /const ZUSTAND_SSR_SAFE_TS: &str = r#"/);
  assert.match(forge, /const ZUSTAND_DEVTOOLS_TS: &str = r#"/);
  assert.match(forge, /const ZUSTAND_IMMER_TS: &str = r#"/);
  assert.match(forge, /export \* from "\.\/traditional";/);
  assert.match(forge, /export \* from "\.\/redux";/);
  assert.match(forge, /export \* from "\.\/ssr-safe";/);
  assert.match(forge, /export \* from "\.\/devtools";/);
  assert.doesNotMatch(forge, /export \* from "\.\/immer";/);
  assert.match(forge, /type Get<TType, TKey, TFallback>/);
  assert.match(forge, /export type Mutate<TStore, TMutators>/);
  assert.match(forge, /export interface StoreMutators<TStore, TPayload> \{\}/);
  assert.match(forge, /export type StoreMutatorIdentifier = keyof StoreMutators/);
  assert.match(forge, /export type StateCreator<\s*TState,\s*MutatorsIn extends/);
  assert.match(forge, /store: Mutate<StoreApi<TState>, MutatorsIn>/);
  assert.match(forge, /type CreateStore = \{/);
  assert.match(forge, /initializer: StateCreator<TState, \[\], MutatorsOut>/);
  assert.match(forge, /Mutate<StoreApi<TState>, MutatorsOut>/);
  assert.match(forge, /const createStoreImpl = <\s*TState,\s*MutatorsOut/);
  assert.match(forge, /export const createStore = \(\(initializer\?: StateCreator<unknown>\) =>/);
  assert.match(forge, /type Create = \{/);
  assert.match(forge, /UseBoundStore<Mutate<StoreApi<TState>, MutatorsOut>>/);
  assert.match(forge, /const createImpl = <\s*TState,\s*MutatorsOut/);
  assert.match(forge, /export const create = \(\(initializer\?: StateCreator<unknown>\) =>/);
  assert.match(forge, /export function useStoreWithEqualityFn/);
  assert.match(forge, /export const createWithEqualityFn/);
  assert.match(forge, /React\.useSyncExternalStore/);
  assert.match(
    forge,
    /type CreateWithEqualityFn = \{\s*<TState, MutatorsOut extends \[StoreMutatorIdentifier, unknown\]\[\] = \[\]>/,
  );
  assert.match(
    forge,
    /UseBoundStoreWithEqualityFn<Mutate<StoreApi<TState>, MutatorsOut>>/,
  );
  assert.match(
    forge,
    /const createWithEqualityFnImpl = <\s*TState,\s*MutatorsOut extends \[StoreMutatorIdentifier, unknown\]\[\] = \[\],\s*>/,
  );
  assert.match(
    forge,
    /import type \{\s*Listener,\s*StateCreator,\s*StoreApi,\s*StoreMutatorIdentifier,\s*\} from "\.\/vanilla";/,
  );
  assert.match(forge, /export type Write<TBase, TOverlay>/);
  assert.match(forge, /export type StoreSubscribeWithSelector<TState> = \{/);
  assert.match(forge, /export type WithSelectorSubscribe<TStore> = TStore extends \{/);
  assert.match(forge, /declare module "\.\/vanilla" \{/);
  assert.match(forge, /"zustand\/subscribeWithSelector": WithSelectorSubscribe<TStore>/);
  assert.match(forge, /export type SubscribeWithSelector = </);
  assert.match(
    forge,
    /initializer: StateCreator<\s*TState,\s*\[\.\.\.MutatorsIn, \["zustand\/subscribeWithSelector", never\]\],\s*MutatorsOut\s*>/,
  );
  assert.match(
    forge,
    /export const subscribeWithSelector =\s*subscribeWithSelectorImpl as unknown as SubscribeWithSelector/,
  );
  assert.match(
    forge,
    /creator: StateCreator<TState, MutatorsIn, MutatorsOut, TSlice>/,
  );
  assert.match(
    forge,
    /StateCreator<Write<TState, TSlice>, MutatorsIn, MutatorsOut>/,
  );
  assert.match(forge, /export interface DevtoolsOptions/);
  assert.match(forge, /export type NamedSet/);
  assert.match(forge, /__REDUX_DEVTOOLS_EXTENSION__/);
  assert.match(forge, /connection\.init\(initialState\)/);
  assert.match(forge, /connection\.send\(toDevtoolsAction/);
  assert.match(forge, /devtools: \{\s*cleanup/);
  assert.match(forge, /export const devtools = devtoolsImpl/);
  assert.match(forge, /import \{ produce, type Draft \} from "immer";/);
  assert.match(
    forge,
    /export type ImmerSetState<TState> = \{/,
  );
  assert.match(forge, /export type WithImmer<TStore> = TStore extends \{/);
  assert.match(forge, /"zustand\/immer": WithImmer<TStore>/);
  assert.match(forge, /export type Immer = </);
  assert.match(
    forge,
    /initializer: StateCreator<\s*TState,\s*\[\.\.\.MutatorsIn, \["zustand\/immer", never\]\],\s*MutatorsOut,\s*Result\s*>/,
  );
  assert.match(
    forge,
    /StateCreator<\s*TState,\s*MutatorsIn,\s*\[\["zustand\/immer", never\], \.\.\.MutatorsOut\],\s*Result\s*>/,
  );
  assert.match(forge, /typeof updater === "function"\s*\?\s*produce/);
  assert.match(forge, /export const immer = immerImpl as unknown as Immer/);
  assert.match(forge, /export type ReduxAction = \{ type: string/);
  assert.match(forge, /export function redux<TState, TAction extends ReduxAction>/);
  assert.match(forge, /dispatchFromDevtools: true/);
  assert.match(forge, /apiWithDispatch\.dispatch = dispatch/);
  assert.match(forge, /export function ssrSafe<TState>/);
  assert.match(forge, /export const unstable_ssrSafe = ssrSafe/);
  assert.match(forge, /Cannot set state of Zustand store in SSR/);
  assert.match(forge, /export type JsonStorageOptions/);
  assert.match(
    forge,
    /import type \{\s*StateCreator,\s*StoreApi,\s*StoreMutatorIdentifier,\s*\} from "\.\/vanilla";/,
  );
  assert.match(forge, /reviver\?: \(key: string, value: unknown\) => unknown/);
  assert.match(forge, /replacer\?: \(key: string, value: unknown\) => unknown/);
  assert.match(forge, /onRehydrateStorage\?:/);
  assert.match(
    forge,
    /export type PersistApi<TState, TPersistedState = TState> = \{/,
  );
  assert.match(
    forge,
    /setOptions: \(\s*options: Partial<PersistOptions<TState, TPersistedState>>,\s*\) => void/,
  );
  assert.match(
    forge,
    /getOptions: \(\) => Partial<PersistOptions<TState, TPersistedState>>/,
  );
  assert.match(forge, /type WithPersist<TStore, TPersistedState> = TStore extends \{/);
  assert.match(forge, /"zustand\/persist": WithPersist<TStore, TPayload>/);
  assert.match(forge, /type Persist = </);
  assert.match(
    forge,
    /initializer: StateCreator<\s*TState,\s*\[\.\.\.MutatorsIn, \["zustand\/persist", unknown\]\],\s*MutatorsOut\s*>/,
  );
  assert.match(
    forge,
    /StateCreator<\s*TState,\s*MutatorsIn,\s*\[\["zustand\/persist", TPersistedState\], \.\.\.MutatorsOut\]\s*>/,
  );
  assert.match(forge, /export const persist = persistImpl as unknown as Persist/);
  assert.match(forge, /onHydrate: \(listener: PersistListener<TState>\) => \(\) => void/);
  assert.match(forge, /onFinishHydration: \(listener: PersistListener<TState>\) => \(\) => void/);
  assert.match(forge, /const hydrationListeners = new Set<PersistListener<TState>>\(\)/);
  assert.match(forge, /const finishHydrationListeners = new Set<PersistListener<TState>>\(\)/);
  assert.match(forge, /optionsWithDefaults\.onRehydrateStorage\?\.\(get\(\)\)/);
  assert.match(forge, /"useStoreWithEqualityFn"/);
  assert.match(forge, /"createWithEqualityFn"/);
  assert.match(forge, /"curried create"/);
  assert.match(forge, /"curried createStore"/);
  assert.match(forge, /"Mutate"/);
  assert.match(forge, /"StoreMutators"/);
  assert.match(forge, /"StoreMutatorIdentifier"/);
  assert.match(forge, /"SubscribeWithSelector"/);
  assert.match(forge, /"StoreSubscribeWithSelector"/);
  assert.match(forge, /"WithSelectorSubscribe"/);
  assert.match(forge, /"Write"/);
  assert.match(forge, /"devtools"/);
  assert.match(forge, /"immer"/);
  assert.match(forge, /"WithImmer"/);
  assert.match(forge, /"ImmerSetState"/);
  assert.match(forge, /"redux"/);
  assert.match(forge, /"unstable_ssrSafe"/);
  assert.match(forge, /"PersistApi"/);
  assert.match(forge, /"PersistOptions"/);
  assert.match(forge, /"persist mutator typing"/);
  assert.match(forge, /aliases: \[/);
  assert.match(forge, /officialName: "State Management"/);
  assert.match(forge, /"zustand"/);
  assert.match(forge, /"npm\/zustand"/);
  assert.match(forge, /"pmndrs\/zustand"/);
  assert.match(forge, /upstreamPackage: "zustand"/);
  assert.match(forge, /upstreamVersion: "5\.0\.13"/);
  assert.match(forge, /sourceMirror: "G:\\\\WWW\\\\inspirations\\\\zustand"/);
  assert.match(forge, /upstreamReference: "npm:zustand@5\.0\.13"/);
  assert.match(forge, /repository: "https:\/\/github\.com\/pmndrs\/zustand"/);
  assert.match(forge, /license: "MIT"/);
  assert.match(forge, /requiredEnv: \[\]/);
  assert.match(forge, /appOwnedBoundaries: \[/);
  assert.match(forge, /exportedFiles: \[/);
  assert.match(forge, /receiptPaths: \[/);
  assert.match(forge, /docs\/packages\/state-zustand\.md/);
  assert.match(
    forge,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-state-zustand-dashboard-workflow\.json/,
  );
  assert.match(forge, /dxIcon: "state:zustand"/);
  assert.match(forge, /dashboardUsage: \{/);
  assert.match(forge, /visibleComponent: "LaunchDashboardStateControl"/);
  assert.match(forge, /runtimeSurface: \{/);
  assert.match(forge, /materializedFile: "pages\/index\.html"/);
  assert.match(forge, /componentMarker: 'data-dx-component="launch-dashboard-state-workflow"'/);
  assert.match(forge, /persistenceKey: "dx-template-dashboard-settings"/);
  assert.match(forge, /counterProof: \{/);
  assert.match(forge, /starterDashboard: \{/);
  assert.match(forge, /component: "ZustandSettingsPanel"/);
  assert.match(forge, /persistKey: "dx-dashboard-settings"/);
  assert.match(forge, /actions: \["toggle-density", "toggle-sidebar", "toggle-command-hints", "rehydrate-settings", "clear-saved-settings", "save-settings"\]/);
  assert.match(forge, /hydrationEvents: \["onHydrate", "onFinishHydration"\]/);
  assert.match(forge, /hydrationMarkers: \["data-dx-zustand-hydration-event", "data-dx-zustand-hydration-count"\]/);
  assert.match(forge, /"state\/zustand\/traditional\.ts"/);
  assert.match(forge, /"state\/zustand\/devtools\.ts"/);
  assert.match(forge, /"state\/zustand\/immer\.ts"/);
  assert.match(forge, /immerImportPath: "@\/lib\/forge\/state\/zustand\/immer"/);
  assert.match(forge, /"state\/zustand\/redux\.ts"/);
  assert.match(forge, /"state\/zustand\/ssr-safe\.ts"/);
  assert.match(forge, /Traditional equality selector helpers are included/);
  assert.match(forge, /DevTools extension bridge is included/);
  assert.match(forge, /Immer middleware.*included/);
  assert.match(forge, /Redux middleware is included/);
  assert.match(forge, /SSR mutation safety is included/);
  assert.match(forge, /persist hydration lifecycle/);
  assert.match(forge, /persist mutator typing/);
  assert.match(forge, /Curried vanilla and React factory overloads are included/);
  assert.match(forge, /Vanilla mutator type contracts.*included/);
  assert.match(forge, /middleware mutator typing/);

  assert.match(
    counter,
    /import \{\s*combine,\s*createStore,\s*createWithEqualityFn,\s*devtools,\s*persist,\s*redux,\s*shallow,\s*subscribeWithSelector,\s*unstable_ssrSafe,\s*useStore,\s*type Mutate,\s*type ReduxState,\s*type StoreApi,\s*\}/,
  );
  assert.match(counter, /import \{ immer \} from "@\/lib\/forge\/state\/zustand\/immer";/);
  assert.doesNotMatch(counter, /type PersistApi/);
  assert.doesNotMatch(counter, /as LaunchCounterPersistApi/);
  assert.match(counter, /import \* as React from "react";/);
  assert.match(counter, /type LaunchCounterStore = LaunchCounterModel & ReduxState/);
  assert.match(
    counter,
    /type LaunchCounterAuditApi = Mutate<\s*StoreApi<LaunchCounterAudit>,\s*\[\["zustand\/subscribeWithSelector", never\], \["zustand\/immer", never\]\]\s*>/,
  );
  assert.match(counter, /const launchCounterAuditStore = createStore<LaunchCounterAudit>\(\)\(/);
  assert.match(counter, /subscribeWithSelector\(\s*immer\(\s*combine\(/);
  assert.match(counter, /const launchCounterAudit = launchCounterAuditStore as LaunchCounterAuditApi/);
  assert.match(counter, /state\.lastAction = action/);
  assert.match(counter, /state\.updates \+= 1/);
  assert.match(counter, /type LaunchCounterAction/);
  assert.match(counter, /function launchCounterReducer/);
  assert.match(counter, /createWithEqualityFn<LaunchCounterStore>\(\)\(/);
  assert.match(counter, /devtools\(/);
  assert.match(counter, /name: "DX Launch Counter"/);
  assert.match(counter, /unstable_ssrSafe\(/);
  assert.match(
    counter,
    /redux\(launchCounterReducer,\s*\{\s*count: 0,\s*reviewMode: false,\s*\}\)/,
  );
  assert.match(counter, /useLaunchCounter\(\s*\(state\) => \(\{/);
  assert.match(counter, /useStore\(launchCounterAudit,\s*\(state\) => \(\{/);
  assert.match(counter, /,\s*shallow,\s*\)/);
  assert.match(counter, /dispatch\(\{ type: "increment" \}\)/);
  assert.match(counter, /launchCounterAudit\.getState\(\)\.noteAction\("increment"\)/);
  assert.match(counter, /dispatch\(\{ type: "reset" \}\)/);
  assert.match(counter, /launchCounterAudit\.getState\(\)\.noteAction\("reset"\)/);
  assert.match(counter, /useLaunchCounter\.persist\.hasHydrated\(\)/);
  assert.match(counter, /useLaunchCounter\.persist\.onHydrate/);
  assert.match(counter, /useLaunchCounter\.persist\.onFinishHydration/);
  assert.match(
    counter,
    /launchCounterAudit\.subscribe\(\s*\(state\) => state\.lastAction/,
  );
  assert.match(counter, /useLaunchCounter\.persist\.rehydrate\(\)/);
  assert.match(counter, /data-dx-package="state\/zustand"/);
  assert.match(counter, /data-dx-component="zustand-state-card"/);
  assert.match(counter, /data-dx-zustand-store="launch-counter"/);
  assert.match(counter, /data-dx-zustand-count=\{String\(count\)\}/);
  assert.match(counter, /data-dx-zustand-toggle-state=\{reviewMode \? "enabled" : "disabled"\}/);
  assert.match(counter, /data-dx-zustand-action="increment"/);
  assert.match(counter, /data-dx-zustand-action="reset"/);
  assert.match(counter, /data-dx-zustand-action="toggle-review-mode"/);
  assert.match(counter, /data-dx-zustand-action="rehydrate"/);
  assert.match(counter, /data-dx-zustand-persist-key="dx-launch-counter"/);
  assert.match(counter, /data-dx-zustand-hydration/);
  assert.match(counter, /data-dx-zustand-vanilla-store/);
  assert.match(counter, /dispatch\(\{ type: "toggleReviewMode" \}\)/);

  assert.match(routeContract, /"components\/template-app\/state-zustand-counter\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/state-zustand-dashboard\.tsx"/);
  assert.match(
    routeContract,
    /"\.dx\/forge\/receipts\/2026-05-22-state-zustand-dashboard-workflow\.json"/,
  );
  assert.match(routeContract, /zustandDashboardStateWorkflow: \{/);
  assert.match(routeContract, /packageId: "state\/zustand"/);
  assert.match(routeContract, /component: "launch-dashboard-state-shell"/);
  assert.match(routeContract, /dashboardWorkflow: "ui-state-persistence"/);
  assert.match(routeContract, /sourceFile: "examples\/template\/state-zustand-dashboard\.tsx"/);
  assert.match(routeContract, /materializedFile: "components\/template-app\/state-zustand-dashboard\.tsx"/);
  assert.match(
    routeContract,
    /materializedReceiptFile:\s*"\.dx\/forge\/receipts\/2026-05-22-state-zustand-dashboard-workflow\.json"/,
  );
  assert.match(routeContract, /"data-dx-zustand-rehydrate-state"/);
  assert.match(studioContract, /id: "launch-dashboard-state-shell"/);
  assert.match(studioContract, /selector: '\[data-dx-component="launch-dashboard-state-shell"\]'/);
  assert.match(studioContract, /sourceFile: "examples\/template\/template-shell\.tsx"/);
  assert.match(studioManifest, /"launch-dashboard-state-shell"/);
  assert.match(studioManifest, /\[data-dx-component=\\"launch-dashboard-state-shell\\"\]/);
  assert.match(launchRoute, /import \{ TemplateLandingPage \} from "@\/components\/template-app\/landing-page";/);
  assert.match(launchRoute, /source-owned App Router/);
  assert.match(launchRoute, /<TemplateLandingPage metrics=\{wwwFrameworkMetrics\} \/>/);
  assert.doesNotMatch(launchRoute, /TemplateShell|templateRouteContract|DxIntlProvider/);
  assert.match(
    launchShell,
    /import \{\s*LaunchDashboardStateControl,\s*useLaunchDashboardSettings,\s*\} from "\.\/state-zustand-dashboard";/,
  );
  assert.match(launchShell, /function LaunchDashboardStateShell/);
  assert.match(launchShell, /useLaunchDashboardSettings\(/);
  assert.match(launchShell, /data-dx-component="launch-dashboard-state-shell"/);
  assert.match(launchShell, /data-dx-dashboard-workflow="ui-state-persistence"/);
  assert.match(launchShell, /data-dx-zustand-store="launch-dashboard-settings"/);
  assert.match(launchShell, /data-dx-zustand-persist-key="dx-template-dashboard-settings"/);
  assert.match(launchShell, /data-dx-zustand-dashboard-density=\{density\}/);
  assert.match(launchShell, /data-dx-zustand-dashboard-focus=\{focus\}/);
  assert.match(launchShell, /data-dx-zustand-command-hints=\{commandHints \? "enabled" : "disabled"\}/);
  assert.match(launchShell, /data-dx-zustand-rehydrate-state=\{rehydrateState\}/);
  assert.match(launchShell, /density === "compact" \? "grid gap-3" : "grid gap-4"/);
  assert.match(launchShell, /<LaunchDashboardStateControl \/>/);
  assert.match(launchShell, /<LaunchDashboardStateShell>/);
  assert.match(launchShell, /<\/LaunchDashboardStateShell>/);
  assert.match(dashboardControl, /createWithEqualityFn<LaunchDashboardSettingsState>/);
  assert.match(dashboardControl, /persist\(/);
  assert.match(dashboardControl, /name: "dx-template-dashboard-settings"/);
  assert.match(dashboardControl, /partialize: \(\{ commandHints, density, focus, savedAt \}\)/);
  assert.match(dashboardControl, /useLaunchDashboardSettings\.persist\.hasHydrated\(\)/);
  assert.match(dashboardControl, /useLaunchDashboardSettings\.persist\.onHydrate/);
  assert.match(dashboardControl, /useLaunchDashboardSettings\.persist\.onFinishHydration/);
  assert.match(dashboardControl, /useLaunchDashboardSettings\.persist\.rehydrate\(\)/);
  assert.match(dashboardControl, /type LaunchDashboardRehydrateState = "idle" \| "loading"/);
  assert.match(dashboardControl, /rehydrateState: LaunchDashboardRehydrateState/);
  assert.match(dashboardControl, /setRehydrateState: \(state: LaunchDashboardRehydrateState\) => void/);
  assert.match(dashboardControl, /data-dx-zustand-action="rehydrate-dashboard-settings"/);
  assert.match(dashboardControl, /data-dx-zustand-hydration-event=\{hydrated \? "onFinishHydration" : "onHydrate"\}/);
  assert.match(dashboardControl, /data-dx-zustand-rehydrate-state=\{rehydrateState\}/);
  assert.match(dashboardControl, /disabled=\{rehydrateState === "loading"\}/);
  assert.match(dashboardControl, /setRehydrateState\("loading"\)/);
  assert.match(dashboardControl, /setRehydrateState\("idle"\)/);
  assert.match(dashboardControl, /data-dx-component="launch-dashboard-state-workflow"/);
  assert.match(dashboardControl, /data-dx-zustand-store="launch-dashboard-settings"/);
  assert.match(dashboardControl, /data-dx-zustand-persist-key="dx-template-dashboard-settings"/);
  assert.match(dashboardControl, /data-dx-zustand-dashboard-density=\{density\}/);
  assert.match(dashboardControl, /data-dx-zustand-dashboard-focus=\{focus\}/);
  assert.match(dashboardControl, /data-dx-zustand-command-hints=\{commandHints \? "enabled" : "disabled"\}/);
  assert.match(dashboardControl, /data-dx-zustand-action="set-dashboard-density"/);
  assert.match(dashboardControl, /data-dx-zustand-action="select-dashboard-focus"/);
  assert.match(dashboardControl, /data-dx-zustand-action="toggle-command-hints"/);
  assert.match(dashboardControl, /data-dx-zustand-action="save-dashboard-settings"/);
  assert.match(dashboardControl, /data-dx-zustand-action="reset-dashboard-settings"/);
  assert.match(runtimePage, /data-dx-component="zustand-state-card"/);
  assert.match(runtimePage, /data-dx-package="state\/zustand"/);
  assert.match(runtimePage, /data-dx-zustand-store="launch-counter"/);
  assert.match(runtimePage, /data-dx-zustand-action="increment"/);
  assert.match(runtimePage, /data-dx-zustand-action="reset"/);
  assert.match(runtimePage, /data-dx-zustand-action="toggle-review-mode"/);
  assert.match(runtimePage, /data-dx-zustand-action="rehydrate"/);
  assert.match(runtimePage, /data-dx-zustand-persist-key="dx-launch-counter"/);
  assert.match(runtimePage, /data-dx-component="launch-dashboard-state-workflow"/);
  assert.match(runtimePage, /data-dx-zustand-store="launch-dashboard-settings"/);
  assert.match(runtimePage, /data-dx-zustand-persist-key="dx-template-dashboard-settings"/);
  assert.match(runtimePage, /data-dx-zustand-action="set-dashboard-density"/);
  assert.match(runtimePage, /data-dx-zustand-action="select-dashboard-focus"/);
  assert.match(runtimePage, /data-dx-zustand-action="save-dashboard-settings"/);
  assert.match(runtimePage, /data-dx-zustand-action="reset-dashboard-settings"/);
  assert.match(runtimePage, /data-dx-zustand-action="rehydrate-dashboard-settings"/);
  assert.match(runtimePage, /data-dx-zustand-hydration-event="onFinishHydration"/);
  assert.match(runtimePage, /data-dx-zustand-rehydrate-state="idle"/);
  assert.match(runtimeJs, /localStorage\.getItem\("dx-launch-counter"\)/);
  assert.match(runtimeJs, /localStorage\.setItem\(\s*"dx-launch-counter"/);
  assert.match(runtimeJs, /localStorage\.getItem\("dx-template-dashboard-settings"\)/);
  assert.match(runtimeJs, /localStorage\.setItem\(\s*"dx-template-dashboard-settings"/);
  assert.match(runtimeJs, /function applyLaunchDashboardSettings/);
  assert.match(runtimeJs, /function bindLaunchDashboardSettings/);
  assert.match(runtimeJs, /function markLaunchDashboardHydration/);
  assert.match(runtimeJs, /function markLaunchDashboardRehydrateState/);
  assert.match(runtimeJs, /#dashboard-settings-rehydrate/);
  assert.match(runtimeJs, /data-dx-zustand-hydration-event/);
  assert.match(runtimeJs, /data-dx-zustand-rehydrate-state/);
  assert.match(runtimeJs, /Rehydrated dx-template-dashboard-settings/);
  assert.match(runtimeJs, /#state-rehydrate/);
  assert.match(runtimeJs, /Rehydrated dx-launch-counter/);
  assert.match(runtimeJs, /state\.reviewMode/);

  assert.match(dashboardSettings, /import \{ ZustandSettingsPanel \} from ['"]\.\.\/components\/ZustandSettingsPanel['"];/);
  assert.match(dashboardSettings, /<ZustandSettingsPanel \/>/);
  assert.match(dashboardForge, /export const createStore =/);
  assert.match(dashboardForge, /export const subscribeWithSelector =/);
  assert.match(dashboardForge, /export function createJSONStorage/);
  assert.match(dashboardForge, /export function persist/);
  assert.match(dashboardForge, /export interface PersistApi/);
  assert.match(dashboardForge, /export type PersistListener/);
  assert.match(dashboardForge, /const baseSetState = api\.setState/);
  assert.match(dashboardForge, /api\.setState = \(partial, replace\) =>/);
  assert.match(dashboardForge, /onHydrate: \(listener: PersistListener<TState>\) => \(\) => void/);
  assert.match(dashboardForge, /onFinishHydration: \(listener: PersistListener<TState>\) => \(\) => void/);
  assert.match(dashboardForge, /const hydrationListeners = new Set<PersistListener<TState>>/);
  assert.match(dashboardForge, /const finishHydrationListeners = new Set<PersistListener<TState>>/);
  assert.match(dashboardForge, /export function shallow/);
  assert.match(dashboardStore, /createStore<DashboardSettingsState>/);
  assert.match(dashboardStore, /persist\(/);
  assert.match(dashboardStore, /subscribeWithSelector\(/);
  assert.match(dashboardStore, /name: 'dx-dashboard-settings'/);
  assert.match(dashboardStore, /partialize: pickDashboardSettings/);
  assert.match(dashboardStore, /rehydrateDashboardSettings/);
  assert.match(dashboardStore, /clearSavedDashboardSettings/);
  assert.match(dashboardStore, /persist\.clearStorage\(\)/);
  assert.doesNotMatch(dashboardStore, /clearSavedDashboardSettings\(\) \{[\s\S]{0,160}setState/);
  assert.match(dashboardStore, /dashboardSettingsStore/);
  assert.match(dashboardPanel, /data-dx-package="state\/zustand"/);
  assert.match(dashboardPanel, /data-dx-component="dashboard-zustand-settings-workflow"/);
  assert.match(dashboardPanel, /data-dx-zustand-store="dashboard-settings"/);
  assert.match(dashboardPanel, /data-dx-zustand-persist-key="dx-dashboard-settings"/);
  assert.match(dashboardPanel, /data-dx-zustand-hydrated=/);
  assert.match(dashboardPanel, /data-dx-zustand-hydration-event=/);
  assert.match(dashboardPanel, /data-dx-zustand-hydration-count=/);
  assert.match(dashboardPanel, /onHydrate/);
  assert.match(dashboardPanel, /onFinishHydration/);
  assert.match(dashboardPanel, /<dx-icon name="pack:state"/);
  assert.match(dashboardPanel, /data-dx-zustand-action="toggle-density"/);
  assert.match(dashboardPanel, /data-dx-zustand-action="toggle-sidebar"/);
  assert.match(dashboardPanel, /data-dx-zustand-action="rehydrate-settings"/);
  assert.match(dashboardPanel, /data-dx-zustand-action="clear-saved-settings"/);
  assert.match(dashboardPanel, /data-dx-zustand-action="save-settings"/);
  assert.match(
    shadcnDashboardControls,
    /import \{ useLaunchDashboardSettings \} from "\.\/state-zustand-dashboard";/,
  );
  assert.match(shadcnDashboardControls, /useLaunchDashboardSettings\(/);
  assert.match(
    shadcnDashboardControls,
    /data-dx-package="state\/zustand,shadcn\/ui\/button"/,
  );
  assert.match(
    shadcnDashboardControls,
    /data-dx-zustand-store="launch-dashboard-settings"/,
  );
  assert.match(
    shadcnDashboardControls,
    /data-dx-zustand-persist-key="dx-template-dashboard-settings"/,
  );
  assert.match(shadcnDashboardControls, /data-dx-zustand-action="set-dashboard-density"/);
  assert.match(shadcnDashboardControls, /data-dx-zustand-action="select-dashboard-focus"/);
  assert.match(shadcnDashboardControls, /data-dx-zustand-action="toggle-command-hints"/);
  assert.match(shadcnDashboardControls, /setDensity\(option\.id\)/);
  assert.match(shadcnDashboardControls, /function selectQueue\(nextQueue: DashboardQueue/);
  assert.match(shadcnDashboardControls, /setFocus\(nextQueue\)/);
  assert.match(shadcnDashboardControls, /selectQueue\(option\.id\)/);
  assert.match(shadcnDashboardControls, /toggleCommandHints\(\)/);
  assert.match(shadcnDashboardControls, /save\(\)/);
  assert.match(shadcnDashboardControls, /setRehydrateState: state\.setRehydrateState/);
  assert.match(shadcnDashboardControls, /data-dx-zustand-rehydrate-state=\{rehydrateState\}/);
  assert.match(shadcnDashboardControls, /disabled=\{rehydrateState === "loading"\}/);

  assert.match(catalog, /persist hydration lifecycle/);
  assert.match(catalog, /"state\/zustand": \{\s*name: "State Management"/);
  assert.doesNotMatch(catalog, /name: "Zustand Dashboard State"/);
  assert.doesNotMatch(catalog, /"createDxStore"/);
  assert.doesNotMatch(catalog, /"useLaunchCounterStore"/);
  assert.match(catalog, /"createWithEqualityFn"/);
  assert.match(catalog, /"persist"/);
  assert.match(catalog, /"LaunchDashboardStateControl"/);
  assert.match(catalog, /"dx-template-dashboard-settings"/);
  assert.match(catalog, /"components\/template-app\/template-shell\.tsx"/);
  assert.match(catalog, /"components\/template-app\/shadcn-dashboard-controls\.tsx"/);
  assert.match(catalog, /"LaunchShadcnDashboardControls"/);
  assert.match(catalog, /rehydrate dashboard settings/);
  assert.match(
    catalog,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-state-zustand-dashboard-workflow\.json/,
  );
  assert.match(catalog, /docs\/packages\/state-zustand\.md/);
  assert.match(catalog, /component: "LaunchDashboardStateControl"/);
  assert.match(catalog, /component: "LaunchDashboardStateShell"/);
  assert.match(catalog, /data-dx-component="launch-dashboard-state-shell"/);
  assert.match(catalog, /runtimeSurface: \{/);
  assert.match(catalog, /materializedFile: "pages\/index\.html"/);
  assert.match(catalog, /selector: '\[data-dx-component="launch-dashboard-state-shell"\]'/);
  assert.match(catalog, /"subscribeWithSelector"/);
  assert.match(catalog, /"redux"/);
  assert.match(packageDocs, /# State Management/);
  assert.match(packageDocs, /Official DX package name: `State Management`/);
  assert.match(packageDocs, /Upstream package: `zustand`/);
  assert.doesNotMatch(packageDocs, /# Zustand Dashboard State/);
  assert.match(packageDocs, /`state\/zustand`/);
  assert.match(packageDocs, /G:\/WWW\/inspirations\/zustand/);
  assert.match(packageDocs, /persist\.rehydrate/);
  assert.match(packageDocs, /hasHydrated/);
  assert.match(packageDocs, /onHydrate/);
  assert.match(packageDocs, /onFinishHydration/);
  assert.match(packageDocs, /createWithEqualityFn/);
  assert.match(packageDocs, /shallow/);
  assert.match(packageDocs, /data-dx-component="launch-dashboard-state-shell"/);
  assert.match(packageDocs, /data-dx-zustand-rehydrate-state/);
  assert.match(
    packageDocs,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-state-zustand-dashboard-workflow\.json/,
  );
  assert.match(packageDocs, /no template-local `node_modules`/);
  assert.match(packageDocs, /Browser\/Web Preview click evidence remains governed/);
  assert.match(
    cli,
    /include_str!\("..\/..\/..\/examples\/template\/state-zustand-dashboard\.tsx"\)/,
  );
  assert.match(
    cli,
    /include_str!\(\s*"..\/..\/..\/examples\/template\/\.dx\/forge\/receipts\/2026-05-22-state-zustand-dashboard-workflow\.json"\s*\)/,
  );
  assert.match(cli, /"components\/template-app\/state-zustand-dashboard\.tsx"/);
  assert.match(
    cli,
    /"\.dx\/forge\/receipts\/2026-05-22-state-zustand-dashboard-workflow\.json"/,
  );
  assert.match(
    cli,
    /"state\/zustand" => vec!\[\s*"create",\s*"createStore",\s*"Mutate",\s*"StoreMutators",\s*"StoreMutatorIdentifier",\s*"SubscribeWithSelector",\s*"StoreSubscribeWithSelector",\s*"WithSelectorSubscribe",\s*"Write",\s*"persist",\s*"createJSONStorage",\s*"PersistApi",\s*"immer",\s*"redux",\s*"createWithEqualityFn",\s*"unstable_ssrSafe",\s*"devtools",\s*\]/,
  );
  assert.match(cli, /"package_id": "state\/zustand", "name": "State Management"/);
  assert.match(cli, /"upstream_package": "zustand"/);
  assert.match(studioManifest, /"front_facing_name": "State Management"/);
  assert.doesNotMatch(studioManifest, /"front_facing_name": "Zustand Dashboard State"/);
  assert.match(dx, /Immer middleware/);
  assert.match(dx, /app-owned `immer` package dependency/);
  assert.match(dx, /mutator typing surface/);
  assert.match(dx, /middleware mutator typing/);
  assert.match(dx, /persist mutator typing/);
  assert.match(dx, /persist hydration events/);
  assert.match(changelog, /Zustand vanilla mutator typing/);
  assert.match(changelog, /Zustand middleware mutator typing/);
  assert.match(changelog, /Zustand persist mutator typing/);
  assert.match(changelog, /Zustand Immer middleware/);
  assert.match(changelog, /Zustand dashboard persist hydration events/);

  assert.equal(dashboardReceipt.schema, "dx.forge.package_dashboard_receipt");
  assert.equal(dashboardReceipt.packageId, "state/zustand");
  assert.equal(dashboardReceipt.packageName, "State Management");
  assert.equal(dashboardReceipt.officialPackageName, "State Management");
  assert.equal(dashboardReceipt.upstreamPackage, "zustand");
  assert.equal(dashboardReceipt.upstreamVersion, "5.0.13");
  assert.equal(dashboardReceipt.visibleComponent, "LaunchDashboardStateControl");
  assert.equal(dashboardReceipt.persistKey, "dx-template-dashboard-settings");
  assert.equal(dashboardReceipt.nodeModulesRequired, false);
  assert.ok(dashboardReceipt.realApis.includes("createWithEqualityFn"));
  assert.ok(dashboardReceipt.realApis.includes("persist"));
  assert.ok(dashboardReceipt.realApis.includes("onHydrate"));
  assert.ok(dashboardReceipt.realApis.includes("onFinishHydration"));
  assert.ok(dashboardReceipt.interactions.includes("set-dashboard-density"));
  assert.ok(dashboardReceipt.interactions.includes("select-dashboard-focus"));
  assert.ok(dashboardReceipt.interactions.includes("preview-dashboard-receipt"));
  assert.ok(dashboardReceipt.interactions.includes("save-dashboard-settings"));
  assert.ok(dashboardReceipt.interactions.includes("rehydrate-dashboard-settings"));
  assert.ok(
    dashboardReceipt.sourceFiles.includes("examples/template/shadcn-dashboard-controls.tsx"),
  );
  assert.ok(
    dashboardReceipt.sourceFiles.includes("examples/template/template-shell.tsx"),
  );
  assert.ok(
    dashboardReceipt.sourceFiles.includes("examples/template/template-route-contract.ts"),
  );
  assert.ok(dashboardReceipt.sourceFiles.includes("dx-www/src/cli/mod.rs"));
  assert.ok(dashboardReceipt.sourceFiles.includes("docs/packages/state-zustand.md"));
  assert.ok(
    dashboardReceipt.materializedFiles.includes("components/template-app/shadcn-dashboard-controls.tsx"),
  );
  assert.ok(
    dashboardReceipt.materializedFiles.includes("components/template-app/template-shell.tsx"),
  );
  assert.ok(dashboardReceipt.materializedFiles.includes("pages/index.html"));
  assert.ok(dashboardReceipt.materializedFiles.includes("public/launch-runtime.js"));
  assert.ok(dashboardReceipt.materializedFiles.includes("styles/launch-runtime.css"));
  assert.ok(dashboardReceipt.materializedFiles.includes("public/preview-manifest.json"));
  assert.ok(dashboardReceipt.markers.includes('data-dx-component="launch-dashboard-state-workflow"'));
  assert.ok(dashboardReceipt.markers.includes('data-dx-component="shadcn-dashboard-controls"'));
  assert.ok(dashboardReceipt.markers.includes('data-dx-component="launch-dashboard-state-shell"'));
  assert.ok(dashboardReceipt.markers.includes('data-dx-dashboard-workflow="ui-state-persistence-shell"'));
  assert.ok(dashboardReceipt.markers.includes('data-dx-zustand-action="rehydrate-dashboard-settings"'));
  assert.ok(dashboardReceipt.markers.includes("data-dx-zustand-hydration-event"));
  assert.ok(dashboardReceipt.markers.includes("data-dx-zustand-rehydrate-state"));
  assert.ok(dashboardReceipt.markers.includes('data-dx-zustand-store="launch-dashboard-settings"'));
  assert.ok(
    dashboardReceipt.markers.includes(
      'data-dx-zustand-persist-key="dx-template-dashboard-settings"',
    ),
  );
});
