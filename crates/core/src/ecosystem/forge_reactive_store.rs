pub(super) const REACTIVE_STORE_VERSION: &str = "0.11.0-dx.1";

pub(super) fn reactive_store_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/state/reactive-store/index.ts", REACTIVE_STORE_INDEX_TS),
        ("js/state/reactive-store/types.ts", REACTIVE_STORE_TYPES_TS),
        ("js/state/reactive-store/atom.ts", REACTIVE_STORE_ATOM_TS),
        ("js/state/reactive-store/store.ts", REACTIVE_STORE_STORE_TS),
        (
            "js/state/reactive-store/shallow.ts",
            REACTIVE_STORE_SHALLOW_TS,
        ),
        ("js/state/reactive-store/react.ts", REACTIVE_STORE_REACT_TS),
        (
            "js/state/reactive-store/context.tsx",
            REACTIVE_STORE_CONTEXT_TS,
        ),
        (
            "js/state/reactive-store/metadata.ts",
            REACTIVE_STORE_METADATA_TS,
        ),
        (
            "js/state/reactive-store/README.md",
            REACTIVE_STORE_README_MD,
        ),
    ]
}

const REACTIVE_STORE_INDEX_TS: &str = r#"export * from "./types";
export * from "./atom";
export * from "./store";
export * from "./shallow";
export * from "./react";
export * from "./context";
export * from "./metadata";
"#;

const REACTIVE_STORE_TYPES_TS: &str = r#"export type Selection<TSelected> = Readable<TSelected>;

export type Observer<T> = {
  next?: (value: T) => void;
  error?: (err: unknown) => void;
  complete?: () => void;
};

export interface Subscription {
  unsubscribe: () => void;
}

export interface Subscribable<T> {
  subscribe: (observer: Observer<T> | ((value: T) => void)) => Subscription;
}

export interface Readable<T> extends Subscribable<T> {
  get: () => T;
}

export interface BaseAtom<T> extends Readable<T> {}

export interface Atom<T> extends BaseAtom<T> {
  set: ((fn: (prevVal: T) => T) => void) & ((value: T) => void);
}

export interface AtomOptions<T> {
  compare?: (prev: T, next: T) => boolean;
}

export type AnyAtom = BaseAtom<any>;

export interface ReadonlyAtom<T> extends BaseAtom<T> {}
"#;

const REACTIVE_STORE_ATOM_TS: &str = r#"import type { Atom, AtomOptions, Observer, ReadonlyAtom, Subscription } from "./types";

type InternalAtom<T> = {
  value: T;
  getter?: (prev?: T) => T;
  compare: (prev: T, next: T) => boolean;
  subscribers: Set<Observer<T>>;
  dependencies: Set<InternalAtom<unknown>>;
  dependents: Set<InternalAtom<unknown>>;
  dirty: boolean;
  get: () => T;
};

type AsyncAtomState<TData, TError = unknown> =
  | { status: "pending" }
  | { status: "done"; data: TData }
  | { status: "error"; error: TError };

let activeAtom: InternalAtom<unknown> | undefined;
let batchDepth = 0;
const pendingAtoms = new Set<InternalAtom<unknown>>();

const defaultCompare = Object.is as <T>(prev: T, next: T) => boolean;

export function toObserver<T>(
  nextHandler?: Observer<T> | ((value: T) => void),
  errorHandler?: (error: unknown) => void,
  completionHandler?: () => void,
): Observer<T> {
  const isObserver = typeof nextHandler === "object" && nextHandler !== null;
  const observer = isObserver ? nextHandler : undefined;

  return {
    next: (isObserver ? nextHandler.next : nextHandler)?.bind(observer),
    error: (isObserver ? nextHandler.error : errorHandler)?.bind(observer),
    complete: (isObserver ? nextHandler.complete : completionHandler)?.bind(
      observer,
    ),
  };
}

export function batch(fn: () => void) {
  try {
    batchDepth += 1;
    fn();
  } finally {
    batchDepth -= 1;
    if (batchDepth === 0) {
      flush();
    }
  }
}

export function flush(): void {
  if (batchDepth > 0) return;

  while (pendingAtoms.size > 0) {
    const atoms = [...pendingAtoms];
    pendingAtoms.clear();

    for (const atom of atoms) {
      const shouldNotify = atom.getter ? recompute(atom) : true;
      if (!shouldNotify) continue;

      notifySubscribers(atom);
      for (const dependent of atom.dependents) {
        dependent.dirty = true;
        pendingAtoms.add(dependent);
      }
    }
  }
}

export function createAsyncAtom<T>(
  getValue: () => Promise<T>,
  options?: AtomOptions<AsyncAtomState<T>>,
): ReadonlyAtom<AsyncAtomState<T>> {
  const atom = createAtom<AsyncAtomState<T>>({ status: "pending" }, options);

  getValue().then(
    (data) => atom.set({ status: "done", data }),
    (error) => atom.set({ status: "error", error }),
  );

  return atom;
}

export function createAtom<T>(
  getValue: (prev?: NoInfer<T>) => T,
  options?: AtomOptions<T>,
): ReadonlyAtom<T>;
export function createAtom<T>(initialValue: T, options?: AtomOptions<T>): Atom<T>;
export function createAtom<T>(
  valueOrFn: T | ((prev?: T) => T),
  options: AtomOptions<T> = {},
): Atom<T> | ReadonlyAtom<T> {
  const isComputed = typeof valueOrFn === "function";
  const getter = isComputed ? (valueOrFn as (prev?: T) => T) : undefined;
  const atom: InternalAtom<T> = {
    value: isComputed ? undefined as T : valueOrFn,
    getter,
    compare: options.compare ?? defaultCompare,
    subscribers: new Set(),
    dependencies: new Set(),
    dependents: new Set(),
    dirty: isComputed,
    get() {
      trackDependency(atom);
      if (atom.getter && atom.dirty) {
        recompute(atom);
      }
      return atom.value;
    },
  };

  const publicAtom: ReadonlyAtom<T> = {
    get: atom.get,
    subscribe(observerOrFn) {
      const observer = toObserver(observerOrFn);
      atom.subscribers.add(observer);
      atom.get();

      return {
        unsubscribe: () => {
          atom.subscribers.delete(observer);
        },
      };
    },
  };

  if (!isComputed) {
    (publicAtom as Atom<T>).set = (valueOrFn: T | ((prev: T) => T)) => {
      const nextValue =
        typeof valueOrFn === "function"
          ? (valueOrFn as (prev: T) => T)(atom.value)
          : valueOrFn;

      if (atom.compare(atom.value, nextValue)) return;

      atom.value = nextValue;
      pendingAtoms.add(atom);
      flush();
    };
  }

  return publicAtom;
}

function trackDependency<T>(atom: InternalAtom<T>) {
  if (!activeAtom || activeAtom === atom) return;
  activeAtom.dependencies.add(atom as InternalAtom<unknown>);
  atom.dependents.add(activeAtom);
}

function recompute<T>(atom: InternalAtom<T>) {
  if (!atom.getter) return false;

  for (const dependency of atom.dependencies) {
    dependency.dependents.delete(atom as InternalAtom<unknown>);
  }
  atom.dependencies.clear();

  const previousActiveAtom = activeAtom;
  activeAtom = atom as InternalAtom<unknown>;
  try {
    const previous = atom.value;
    const next = atom.getter(previous);
    atom.dirty = false;

    if (previous !== undefined && atom.compare(previous, next)) {
      return false;
    }

    atom.value = next;
    return true;
  } finally {
    activeAtom = previousActiveAtom;
  }
}

function notifySubscribers<T>(atom: InternalAtom<T>) {
  for (const subscriber of atom.subscribers) {
    try {
      subscriber.next?.(atom.value);
    } catch (error) {
      subscriber.error?.(error);
    }
  }
}
"#;

const REACTIVE_STORE_STORE_TS: &str = r#"import { createAtom, toObserver } from "./atom";
import type { Atom, Observer, Subscription } from "./types";

export type StoreAction = (...args: Array<any>) => any;

export type StoreActionMap = Record<string, StoreAction>;

export type StoreActionsFactory<T, TActions extends StoreActionMap> = (store: {
  setState: Store<T>["setState"];
  get: Store<T>["get"];
}) => TActions;

type NonFunction<T> = T extends (...args: Array<any>) => any ? never : T;

export class Store<T, TActions extends StoreActionMap = never> {
  private atom: Atom<T>;
  public readonly actions!: TActions;

  constructor(initialValue: T);
  constructor(
    initialValue: NonFunction<T>,
    actionsFactory: StoreActionsFactory<T, TActions>,
  );
  constructor(
    initialValue: NonFunction<T>,
    actionsFactory?: StoreActionsFactory<T, TActions>,
  ) {
    this.atom = createAtom(initialValue as T);

    this.get = this.get.bind(this);
    this.setState = this.setState.bind(this);
    this.subscribe = this.subscribe.bind(this);

    if (actionsFactory) {
      this.actions = actionsFactory(this);
    }
  }

  public setState(updater: (prev: T) => T) {
    this.atom.set(updater);
  }

  public get state() {
    return this.atom.get();
  }

  public get() {
    return this.state;
  }

  public subscribe(
    observerOrFn: Observer<T> | ((value: T) => void),
  ): Subscription {
    return this.atom.subscribe(toObserver(observerOrFn));
  }
}

export class ReadonlyStore<T> implements Omit<Store<T>, "setState" | "actions"> {
  private atom: ReturnType<typeof createAtom<T>>;

  constructor(getValue: (prev?: NoInfer<T>) => T) {
    this.atom = createAtom(getValue);

    this.get = this.get.bind(this);
    this.subscribe = this.subscribe.bind(this);
  }

  public get state() {
    return this.atom.get();
  }

  public get() {
    return this.state;
  }

  public subscribe(
    observerOrFn: Observer<T> | ((value: T) => void),
  ): Subscription {
    return this.atom.subscribe(toObserver(observerOrFn));
  }
}

export function createStore<T>(
  getValue: (prev?: NoInfer<T>) => T,
): ReadonlyStore<T>;
export function createStore<T>(initialValue: T): Store<T>;
export function createStore<T, TActions extends StoreActionMap>(
  initialValue: NonFunction<T>,
  actions: StoreActionsFactory<T, TActions>,
): Store<T, TActions>;
export function createStore<T, TActions extends StoreActionMap>(
  valueOrFn: T | ((prev?: T) => T),
  actions?: StoreActionsFactory<T, TActions>,
): Store<T, TActions> | Store<T> | ReadonlyStore<T> {
  if (typeof valueOrFn === "function") {
    return new ReadonlyStore(valueOrFn as (prev?: NoInfer<T>) => T);
  }

  if (actions) {
    return new Store(valueOrFn as NonFunction<T>, actions);
  }

  return new Store(valueOrFn);
}
"#;

const REACTIVE_STORE_SHALLOW_TS: &str = r#"export function shallow<T>(objA: T, objB: T) {
  if (Object.is(objA, objB)) {
    return true;
  }

  if (
    typeof objA !== "object" ||
    objA === null ||
    typeof objB !== "object" ||
    objB === null
  ) {
    return false;
  }

  if (objA instanceof Map && objB instanceof Map) {
    if (objA.size !== objB.size) return false;
    for (const [key, value] of objA) {
      if (!objB.has(key) || !Object.is(value, objB.get(key))) return false;
    }
    return true;
  }

  if (objA instanceof Set && objB instanceof Set) {
    if (objA.size !== objB.size) return false;
    for (const value of objA) {
      if (!objB.has(value)) return false;
    }
    return true;
  }

  if (objA instanceof Date && objB instanceof Date) {
    return objA.getTime() === objB.getTime();
  }

  const keysA = getOwnKeys(objA);
  if (keysA.length !== getOwnKeys(objB).length) {
    return false;
  }

  for (const key of keysA) {
    if (
      !Object.prototype.hasOwnProperty.call(objB, key) ||
      !Object.is(objA[key as keyof T], objB[key as keyof T])
    ) {
      return false;
    }
  }

  return true;
}

function getOwnKeys(obj: object): Array<string | symbol> {
  return (Object.keys(obj) as Array<string | symbol>).concat(
    Object.getOwnPropertySymbols(obj),
  );
}
"#;

const REACTIVE_STORE_REACT_TS: &str = r#"import * as React from "react";

import { createAtom } from "./atom";
import { createStore } from "./store";
import type { Atom, Readable } from "./types";

export interface UseSelectorOptions<TSelected> {
  compare?: (a: TSelected, b: TSelected) => boolean;
}

type SelectionSource<T> = Pick<Readable<T>, "get" | "subscribe">;

function defaultCompare<T>(a: T, b: T) {
  return Object.is(a, b);
}

export function useSelector<TSource, TSelected = NoInfer<TSource>>(
  source: SelectionSource<TSource>,
  selector: (snapshot: TSource) => TSelected = (snapshot) =>
    snapshot as unknown as TSelected,
  options?: UseSelectorOptions<TSelected>,
): TSelected {
  const compare = options?.compare ?? defaultCompare;
  const selectedRef = React.useRef<TSelected | undefined>(undefined);
  const hasSelectedRef = React.useRef(false);

  const subscribe = React.useCallback(
    (handleStoreChange: () => void) => {
      const subscription = source.subscribe(handleStoreChange);
      return () => subscription.unsubscribe();
    },
    [source],
  );

  const getSelectedSnapshot = React.useCallback(() => {
    const nextSelected = selector(source.get());
    if (
      hasSelectedRef.current &&
      compare(selectedRef.current as TSelected, nextSelected)
    ) {
      return selectedRef.current as TSelected;
    }

    hasSelectedRef.current = true;
    selectedRef.current = nextSelected;
    return nextSelected;
  }, [compare, selector, source]);

  return React.useSyncExternalStore(
    subscribe,
    getSelectedSnapshot,
    getSelectedSnapshot,
  );
}

export function useAtom<TValue>(
  atom: Atom<TValue>,
  options?: UseSelectorOptions<TValue>,
): [TValue, Atom<TValue>["set"]] {
  return [useSelector(atom, undefined, options), atom.set];
}

export const useStore = <TSource, TSelected = NoInfer<TSource>>(
  source: SelectionSource<TSource>,
  selector: (snapshot: TSource) => TSelected = (snapshot) =>
    snapshot as unknown as TSelected,
  compare?: (a: TSelected, b: TSelected) => boolean,
) => useSelector(source, selector, { compare });

export function useCreateAtom<TValue>(initialValue: TValue) {
  const ref = React.useRef<Atom<TValue> | undefined>(undefined);
  ref.current ??= createAtom(initialValue);
  return ref.current;
}

export function useCreateStore<TValue>(initialValue: TValue) {
  const ref = React.useRef<ReturnType<typeof createStore<TValue>> | undefined>(
    undefined,
  );
  ref.current ??= createStore(initialValue);
  return ref.current;
}
"#;

const REACTIVE_STORE_CONTEXT_TS: &str = r#"import { createContext, useContext } from "react";
import type { PropsWithChildren, ReactElement } from "react";

export function createStoreContext<TValue extends object>(): {
  StoreProvider: (
    props: {
      value: TValue;
    } & PropsWithChildren,
  ) => ReactElement;
  useStoreContext: () => TValue;
} {
  const Context = createContext<TValue | null>(null);
  Context.displayName = "StoreContext";

  function StoreProvider({
    children,
    value,
  }: PropsWithChildren<{
    value: TValue;
  }>) {
    return <Context.Provider value={value}>{children}</Context.Provider>;
  }

  function useStoreContext() {
    const value = useContext(Context);

    if (value === null) {
      throw new Error("Missing StoreProvider for StoreContext");
    }

    return value;
  }

  return {
    StoreProvider,
    useStoreContext,
  };
}
"#;

const REACTIVE_STORE_METADATA_TS: &str = r#"export const reactiveStorePackageMetadata = {
  officialName: "Reactive Store",
  packageId: "reactive/store",
  aliases: [
    "reactive-store",
    "@tanstack/store",
    "@tanstack/react-store",
    "tanstack-store",
  ],
  upstreamPackage: "@tanstack/store",
  reactUpstreamPackage: "@tanstack/react-store",
  sourceMirror: "G:\\WWW\\inspirations\\tanstack-store",
  upstreamVersion: "0.11.0",
  dxVersion: "0.11.0-dx.1",
  inspectedSourceFiles: [
    "packages/store/package.json",
    "packages/react-store/package.json",
    "packages/store/src/store.ts",
    "packages/store/src/atom.ts",
    "packages/store/src/types.ts",
    "packages/store/src/shallow.ts",
    "packages/react-store/src/useSelector.ts",
    "packages/react-store/src/useAtom.ts",
    "packages/react-store/src/useStore.ts",
    "packages/react-store/src/createStoreContext.tsx",
    "docs/quick-start.md",
    "docs/framework/react/quick-start.md",
  ],
  surfaces: [
    {
      id: "core-store",
      files: ["store.ts", "atom.ts", "types.ts"],
      upstreamApis: ["Store", "ReadonlyStore", "createStore"],
      status: "present",
    },
    {
      id: "atom-graph",
      files: ["atom.ts", "types.ts"],
      upstreamApis: ["createAtom", "createAsyncAtom", "batch", "flush"],
      status: "present",
    },
    {
      id: "comparison-helper",
      files: ["shallow.ts"],
      upstreamApis: ["shallow"],
      status: "present",
    },
    {
      id: "react-selector",
      files: ["react.ts"],
      upstreamApis: ["useSelector", "useAtom", "useStore"],
      status: "present",
    },
    {
      id: "react-context",
      files: ["context.tsx"],
      upstreamApis: ["createStoreContext"],
      status: "present",
    },
  ],
  requiredEnv: [],
  appOwnedBoundaries: [
    "State shape, action taxonomy, mutation policy, persistence, and sensitive-state review",
    "React render-granularity expectations, dependency installation, and any exact upstream package swap",
  ],
  statusLabels: ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
  dxCheckVisibility: {
    receiptSchema: "dx.forge.reactive_store_receipt",
    receiptPath: ".dx/forge/receipts/packages/reactive-store.json",
    statuses: ["present", "stale", "missing receipt", "blocked", "unsupported surface"],
  },
  dxStyleCompatibility: {
    uiSurface: "none",
    note: "Reactive Store ships editable state source. Any visible UI that consumes it remains app-owned and must pass the app dx-style check.",
  },
  zedSourceMarkers: [
    "lib/forge/state/reactive-store/store.ts#createStore",
    "lib/forge/state/reactive-store/atom.ts#createAtom",
    "lib/forge/state/reactive-store/shallow.ts#shallow",
    "lib/forge/state/reactive-store/react.ts#useSelector",
    "lib/forge/state/reactive-store/context.tsx#createStoreContext",
  ],
} as const;

export type ReactiveStoreSurfaceId =
  (typeof reactiveStorePackageMetadata.surfaces)[number]["id"];

export type ReactiveStoreDxCheckStatus =
  (typeof reactiveStorePackageMetadata.statusLabels)[number];

export type ReactiveStoreReceiptInput = {
  selectedSurfaces: readonly ReactiveStoreSurfaceId[];
  files: readonly string[];
  hashes?: Readonly<Record<string, string>>;
  status?: ReactiveStoreDxCheckStatus;
};

export function createReactiveStoreForgeReceipt(
  input: ReactiveStoreReceiptInput,
) {
  return {
    schema: "dx.forge.reactive_store_receipt",
    official_package_name: "Reactive Store",
    package_id: "reactive/store",
    upstream_package: "@tanstack/store",
    based_on: "@tanstack/react-store",
    source_mirror: reactiveStorePackageMetadata.sourceMirror,
    upstream_version: reactiveStorePackageMetadata.upstreamVersion,
    selected_surfaces: input.selectedSurfaces,
    files: input.files,
    hashes: input.hashes ?? {},
    status: input.status ?? "present",
    provenance: {
      upstream_package: "@tanstack/store",
      source_mirror: reactiveStorePackageMetadata.sourceMirror,
      inspected_source_files: reactiveStorePackageMetadata.inspectedSourceFiles,
    },
    runtime_limitations: [
      "Source and materialization are verified; browser runtime proof is app-owned until a governed run is approved.",
      "No package manager install or node_modules workflow is performed by this Forge slice.",
    ],
  };
}
"#;

const REACTIVE_STORE_README_MD: &str = r#"# Reactive Store

Official DX package name: `Reactive Store`

Package id: `reactive/store`

Provenance metadata:

- upstream_package: `@tanstack/store`
- based_on: `@tanstack/react-store`
- source_mirror: `G:/WWW/inspirations/tanstack-store`
- upstream_version: `0.11.0`

Honesty label: `SOURCE-ONLY`

This Forge slice materializes selected, editable state source for a DX project. It is based on inspected upstream Store and React adapter public APIs, but it does not claim to vendor every internal scheduler or every framework adapter from the upstream repository.

## Selected Surfaces

- `types.ts` exposes `Observer`, `Subscription`, `Readable`, `Atom`, `ReadonlyAtom`, and atom option contracts.
- `atom.ts` exposes `createAtom`, `createAsyncAtom`, `batch`, `flush`, and `toObserver`.
- `store.ts` exposes `Store`, `ReadonlyStore`, `createStore`, `StoreActionMap`, and `StoreActionsFactory`.
- `shallow.ts` exposes the object/Map/Set/Date shallow comparison helper.
- `react.ts` exposes `useSelector`, `useAtom`, `useStore`, `useCreateAtom`, and `useCreateStore`.
- `context.tsx` exposes `createStoreContext` for typed React context transport of app-owned atoms and stores.
- `metadata.ts` exposes official package metadata, provenance, app-owned boundaries, dx-check status labels, and a receipt constructor.

## Selective Surface Installs

The default add path materializes the full editable slice. Forge also exposes narrow surface selectors for apps that only need one part of Reactive Store:

- `dx forge add reactive/store#core-store --write` materializes `store.ts`, its `atom.ts` dependency, `types.ts`, metadata, and README without React selector hooks or shallow comparison.
- `dx forge add reactive/store#atom-graph --write` materializes `atom.ts`, `types.ts`, metadata, and README.
- `dx forge add reactive/store#comparison-helper --write` materializes `shallow.ts`, metadata, and README.
- `dx forge add reactive/store#react-selector --write` materializes `react.ts` plus the store, atom, type, metadata, and README files it imports.
- `dx forge add reactive/store#react-context --write` materializes `context.tsx`, metadata, and README without forcing store, atom, selector, or comparison files.

## Front-Facing Files

Default materialization writes to:

- `lib/forge/state/reactive-store/index.ts`
- `lib/forge/state/reactive-store/types.ts`
- `lib/forge/state/reactive-store/atom.ts`
- `lib/forge/state/reactive-store/store.ts`
- `lib/forge/state/reactive-store/shallow.ts`
- `lib/forge/state/reactive-store/react.ts`
- `lib/forge/state/reactive-store/context.tsx`
- `lib/forge/state/reactive-store/metadata.ts`
- `lib/forge/state/reactive-store/README.md`

## dx-check Visibility

The package metadata uses these states for consumers: present, stale, missing receipt, blocked, unsupported surface.

The source-owned receipt helper records selected surfaces, files, optional hashes, provenance, source mirror, runtime limitations, and app-owned boundaries. A generated application should keep that receipt under `.dx/forge/receipts/packages/reactive-store.json`.

## App-Owned Boundaries

Applications still own state shape, action taxonomy, mutation conventions, persisted storage, sensitive-state policy, React render-performance expectations, and any exact upstream dependency swap. Browser proof is intentionally deferred until the app runs an approved runtime check.

## Source Guard

Run the narrow guard with:

```powershell
dx run --test .\benchmarks\reactive-store-slice.test.ts
```
"#;
