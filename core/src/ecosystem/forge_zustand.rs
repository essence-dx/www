pub(super) const ZUSTAND_VERSION: &str = "5.0.13-dx.10";

pub(super) fn zustand_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/state/zustand/index.ts", ZUSTAND_INDEX_TS),
        ("js/state/zustand/vanilla.ts", ZUSTAND_VANILLA_TS),
        ("js/state/zustand/react.ts", ZUSTAND_REACT_TS),
        ("js/state/zustand/traditional.ts", ZUSTAND_TRADITIONAL_TS),
        ("js/state/zustand/middleware.ts", ZUSTAND_MIDDLEWARE_TS),
        ("js/state/zustand/devtools.ts", ZUSTAND_DEVTOOLS_TS),
        ("js/state/zustand/immer.ts", ZUSTAND_IMMER_TS),
        ("js/state/zustand/redux.ts", ZUSTAND_REDUX_TS),
        ("js/state/zustand/ssr-safe.ts", ZUSTAND_SSR_SAFE_TS),
        ("js/state/zustand/shallow.ts", ZUSTAND_SHALLOW_TS),
        ("js/state/zustand/persist.ts", ZUSTAND_PERSIST_TS),
        ("js/state/zustand/metadata.ts", ZUSTAND_METADATA_TS),
        ("js/state/zustand/README.md", ZUSTAND_README_MD),
    ]
}

const ZUSTAND_INDEX_TS: &str = r#"export * from "./vanilla";
export * from "./react";
export * from "./traditional";
export * from "./middleware";
export * from "./devtools";
export * from "./redux";
export * from "./ssr-safe";
export * from "./persist";
export * from "./shallow";
"#;

const ZUSTAND_VANILLA_TS: &str = r#"export type Listener<TState> = (
  state: TState,
  previousState: TState,
) => void;

export type SetState<TState> = (
  partial:
    | TState
    | Partial<TState>
    | ((state: TState) => TState | Partial<TState>),
  replace?: boolean,
) => void;

export type ExtractState<TStore> = TStore extends {
  getState: () => infer TState;
}
  ? TState
  : never;

type Get<TType, TKey, TFallback> = TKey extends keyof TType
  ? TType[TKey]
  : TFallback;

export type Mutate<TStore, TMutators> = number extends
  TMutators["length" & keyof TMutators]
  ? TStore
  : TMutators extends []
    ? TStore
    : TMutators extends [[infer MutatorId, infer MutatorPayload], ...infer Rest]
      ? Mutate<
          StoreMutators<TStore, MutatorPayload>[
            MutatorId & StoreMutatorIdentifier
          ],
          Rest
        >
      : never;

export type StateCreator<
  TState,
  MutatorsIn extends [StoreMutatorIdentifier, unknown][] = [],
  MutatorsOut extends [StoreMutatorIdentifier, unknown][] = [],
  Result = TState,
> = ((
  set: Get<Mutate<StoreApi<TState>, MutatorsIn>, "setState", never>,
  get: Get<Mutate<StoreApi<TState>, MutatorsIn>, "getState", never>,
  store: Mutate<StoreApi<TState>, MutatorsIn>,
) => Result) & { $$storeMutators?: MutatorsOut };

export interface StoreMutators<TStore, TPayload> {}
export type StoreMutatorIdentifier = keyof StoreMutators<unknown, unknown>;

export interface StoreApi<TState> {
  setState: SetState<TState>;
  getState: () => TState;
  getInitialState: () => TState;
  subscribe: (listener: Listener<TState>) => () => void;
}

type CreateStore = {
  <TState, MutatorsOut extends [StoreMutatorIdentifier, unknown][] = []>(
    initializer: StateCreator<TState, [], MutatorsOut>,
  ): Mutate<StoreApi<TState>, MutatorsOut>;
  <TState>(): <MutatorsOut extends [StoreMutatorIdentifier, unknown][] = []>(
    initializer: StateCreator<TState, [], MutatorsOut>,
  ) => Mutate<StoreApi<TState>, MutatorsOut>;
};

const createStoreImpl = <
  TState,
  MutatorsOut extends [StoreMutatorIdentifier, unknown][] = [],
>(
  initializer: StateCreator<TState, [], MutatorsOut>,
): Mutate<StoreApi<TState>, MutatorsOut> => {
  let state: TState;
  let initialState: TState;
  const listeners = new Set<Listener<TState>>();

  const setState: SetState<TState> = (partial, replace) => {
    const nextState =
      typeof partial === "function"
        ? (partial as (state: TState) => TState | Partial<TState>)(state)
        : partial;

    if (Object.is(nextState, state)) {
      return;
    }

    const previousState = state;
    state =
      replace || typeof nextState !== "object" || nextState === null
        ? (nextState as TState)
        : Object.assign({}, state, nextState);

    listeners.forEach((listener) => listener(state, previousState));
  };

  const api: StoreApi<TState> = {
    setState,
    getState: () => state,
    getInitialState: () => initialState,
    subscribe: (listener) => {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
  };

  initialState = state = initializer(setState, api.getState, api);
  return api as Mutate<StoreApi<TState>, MutatorsOut>;
};

export const createStore = ((initializer?: StateCreator<unknown>) =>
  initializer ? createStoreImpl(initializer) : createStoreImpl) as CreateStore;
"#;

const ZUSTAND_REACT_TS: &str = r#"import * as React from "react";

import {
  createStore,
  type ExtractState,
  type Mutate,
  type StateCreator,
  type StoreApi,
  type StoreMutatorIdentifier,
} from "./vanilla";

type ReadonlyStoreApi<TState> = Pick<
  StoreApi<TState>,
  "getState" | "getInitialState" | "subscribe"
>;

const identity = <TValue>(value: TValue) => value;

export function useStore<TStore extends ReadonlyStoreApi<unknown>>(
  api: TStore,
): ExtractState<TStore>;
export function useStore<TStore extends ReadonlyStoreApi<unknown>, TSlice>(
  api: TStore,
  selector: (state: ExtractState<TStore>) => TSlice,
): TSlice;
export function useStore<TState, TSlice>(
  api: ReadonlyStoreApi<TState>,
  selector: (state: TState) => TSlice = identity as (
    state: TState,
  ) => TSlice,
) {
  const slice = React.useSyncExternalStore(
    api.subscribe,
    () => selector(api.getState()),
    () => selector(api.getInitialState()),
  );
  React.useDebugValue(slice);
  return slice;
}

export type UseBoundStore<TStore extends ReadonlyStoreApi<unknown>> = {
  (): ExtractState<TStore>;
  <TSlice>(selector: (state: ExtractState<TStore>) => TSlice): TSlice;
} & TStore;

type Create = {
  <TState, MutatorsOut extends [StoreMutatorIdentifier, unknown][] = []>(
    initializer: StateCreator<TState, [], MutatorsOut>,
  ): UseBoundStore<Mutate<StoreApi<TState>, MutatorsOut>>;
  <TState>(): <MutatorsOut extends [StoreMutatorIdentifier, unknown][] = []>(
    initializer: StateCreator<TState, [], MutatorsOut>,
  ) => UseBoundStore<Mutate<StoreApi<TState>, MutatorsOut>>;
};

const createImpl = <
  TState,
  MutatorsOut extends [StoreMutatorIdentifier, unknown][] = [],
>(
  initializer: StateCreator<TState, [], MutatorsOut>,
): UseBoundStore<Mutate<StoreApi<TState>, MutatorsOut>> => {
  const api = createStore(initializer);
  const useBoundStore = ((selector?: (state: TState) => unknown) =>
    useStore(api, selector as never)) as UseBoundStore<
    Mutate<StoreApi<TState>, MutatorsOut>
  >;

  Object.assign(useBoundStore, api);
  return useBoundStore;
};

export const create = ((initializer?: StateCreator<unknown>) =>
  initializer ? createImpl(initializer) : createImpl) as Create;
"#;

const ZUSTAND_TRADITIONAL_TS: &str = r#"import * as React from "react";

import {
  createStore,
  type ExtractState,
  type Mutate,
  type StateCreator,
  type StoreApi,
  type StoreMutatorIdentifier,
} from "./vanilla";

type ReadonlyStoreApi<TState> = Pick<
  StoreApi<TState>,
  "getState" | "getInitialState" | "subscribe"
>;

export type EqualityFn<TValue> = (left: TValue, right: TValue) => boolean;
type AnyEqualityFn = <TValue>(left: TValue, right: TValue) => boolean;

const identity = <TValue>(value: TValue) => value;
const objectIs: AnyEqualityFn = Object.is;

export function useStoreWithEqualityFn<TStore extends ReadonlyStoreApi<unknown>>(
  api: TStore,
): ExtractState<TStore>;
export function useStoreWithEqualityFn<
  TStore extends ReadonlyStoreApi<unknown>,
  TSlice,
>(
  api: TStore,
  selector: (state: ExtractState<TStore>) => TSlice,
  equalityFn?: EqualityFn<TSlice>,
): TSlice;
export function useStoreWithEqualityFn<TState, TSlice>(
  api: ReadonlyStoreApi<TState>,
  selector: (state: TState) => TSlice = identity as (
    state: TState,
  ) => TSlice,
  equalityFn: EqualityFn<TSlice> = Object.is,
) {
  const sliceRef = React.useRef<{ value: TSlice } | undefined>(undefined);

  const getSnapshot = () => {
    const nextSlice = selector(api.getState());
    const previous = sliceRef.current;
    if (previous && equalityFn(previous.value, nextSlice)) {
      return previous.value;
    }
    sliceRef.current = { value: nextSlice };
    return nextSlice;
  };

  const slice = React.useSyncExternalStore(
    api.subscribe,
    getSnapshot,
    () => selector(api.getInitialState()),
  );
  React.useDebugValue(slice);
  return slice;
}

export type UseBoundStoreWithEqualityFn<
  TStore extends ReadonlyStoreApi<unknown>,
> = {
  (): ExtractState<TStore>;
  <TSlice>(
    selector: (state: ExtractState<TStore>) => TSlice,
    equalityFn?: EqualityFn<TSlice>,
  ): TSlice;
} & TStore;

type CreateWithEqualityFn = {
  <TState, MutatorsOut extends [StoreMutatorIdentifier, unknown][] = []>(
    initializer: StateCreator<TState, [], MutatorsOut>,
    defaultEqualityFn?: AnyEqualityFn,
  ): UseBoundStoreWithEqualityFn<Mutate<StoreApi<TState>, MutatorsOut>>;
  <TState>(): <
    MutatorsOut extends [StoreMutatorIdentifier, unknown][] = [],
  >(
    initializer: StateCreator<TState, [], MutatorsOut>,
    defaultEqualityFn?: AnyEqualityFn,
  ) => UseBoundStoreWithEqualityFn<Mutate<StoreApi<TState>, MutatorsOut>>;
};

const createWithEqualityFnImpl = <
  TState,
  MutatorsOut extends [StoreMutatorIdentifier, unknown][] = [],
>(
  initializer: StateCreator<TState, [], MutatorsOut>,
  defaultEqualityFn: AnyEqualityFn = objectIs,
): UseBoundStoreWithEqualityFn<Mutate<StoreApi<TState>, MutatorsOut>> => {
  const api = createStore(initializer);

  const useBoundStoreWithEqualityFn = ((
    selector?: (state: TState) => unknown,
    equalityFn: EqualityFn<unknown> = defaultEqualityFn,
  ) =>
    useStoreWithEqualityFn(
      api,
      selector as (state: TState) => unknown,
      equalityFn,
    )) as UseBoundStoreWithEqualityFn<StoreApi<TState>>;

  Object.assign(useBoundStoreWithEqualityFn, api);
  return useBoundStoreWithEqualityFn as UseBoundStoreWithEqualityFn<
    Mutate<StoreApi<TState>, MutatorsOut>
  >;
};

export const createWithEqualityFn = (<TState>(
  initializer?: StateCreator<TState, [], []>,
  defaultEqualityFn?: AnyEqualityFn,
) =>
  initializer
    ? createWithEqualityFnImpl(initializer, defaultEqualityFn)
    : createWithEqualityFnImpl) as CreateWithEqualityFn;
"#;

const ZUSTAND_MIDDLEWARE_TS: &str = r#"import type {
  Listener,
  StateCreator,
  StoreApi,
  StoreMutatorIdentifier,
} from "./vanilla";

export type EqualityChecker<TValue> = (left: TValue, right: TValue) => boolean;

export type SelectorListener<TState, TSlice> = (
  slice: TSlice,
  previousSlice: TSlice,
) => void;

export type Write<TBase, TOverlay> = Omit<TBase, keyof TOverlay> & TOverlay;

export type StoreSubscribeWithSelector<TState> = {
  subscribe: {
    (listener: Listener<TState>): () => void;
    <TSlice>(
      selector: (state: TState) => TSlice,
      listener: SelectorListener<TState, TSlice>,
      options?: {
        equalityFn?: EqualityChecker<TSlice>;
        fireImmediately?: boolean;
      },
    ): () => void;
  };
};

export type SelectorSubscribe<TState> =
  StoreSubscribeWithSelector<TState>["subscribe"];

export type StoreWithSelector<TState> = Write<
  StoreApi<TState>,
  StoreSubscribeWithSelector<TState>
>;

export type WithSelectorSubscribe<TStore> = TStore extends {
  getState: () => infer TState;
}
  ? Write<TStore, StoreSubscribeWithSelector<TState>>
  : never;

declare module "./vanilla" {
  interface StoreMutators<TStore, TPayload> {
    "zustand/subscribeWithSelector": WithSelectorSubscribe<TStore>;
  }
}

export type SubscribeWithSelector = <
  TState,
  MutatorsIn extends [StoreMutatorIdentifier, unknown][] = [],
  MutatorsOut extends [StoreMutatorIdentifier, unknown][] = [],
>(
  initializer: StateCreator<
    TState,
    [...MutatorsIn, ["zustand/subscribeWithSelector", never]],
    MutatorsOut
  >,
) => StateCreator<
  TState,
  MutatorsIn,
  [["zustand/subscribeWithSelector", never], ...MutatorsOut]
>;

const subscribeWithSelectorImpl = <TState>(
  initializer: StateCreator<TState>,
): StateCreator<TState> => (set, get, api) => {
  const baseSubscribe = api.subscribe;
  (api as StoreWithSelector<TState>).subscribe = ((
    selector: Listener<TState> | ((state: TState) => unknown),
    listener?: SelectorListener<TState, unknown>,
    options?: {
      equalityFn?: EqualityChecker<unknown>;
      fireImmediately?: boolean;
    },
  ) => {
    if (!listener) {
      return baseSubscribe(selector as Listener<TState>);
    }

    const equalityFn = options?.equalityFn ?? Object.is;
    let currentSlice = selector(api.getState());
    const unsubscribe = baseSubscribe((state) => {
      const nextSlice = selector(state);
      if (!equalityFn(currentSlice, nextSlice)) {
        const previousSlice = currentSlice;
        currentSlice = nextSlice;
        listener(currentSlice, previousSlice);
      }
    });

    if (options?.fireImmediately) {
      listener(currentSlice, currentSlice);
    }

    return unsubscribe;
  }) as SelectorSubscribe<TState>;

  return initializer(set, get, api);
};

export const subscribeWithSelector =
  subscribeWithSelectorImpl as unknown as SubscribeWithSelector;

export function combine<
  TState extends object,
  TSlice extends object,
  MutatorsIn extends [StoreMutatorIdentifier, unknown][] = [],
  MutatorsOut extends [StoreMutatorIdentifier, unknown][] = [],
>(
  initialState: TState,
  creator: StateCreator<TState, MutatorsIn, MutatorsOut, TSlice>,
): StateCreator<Write<TState, TSlice>, MutatorsIn, MutatorsOut> {
  const creatorWithCombinedState = creator as unknown as StateCreator<
    Write<TState, TSlice>,
    MutatorsIn,
    MutatorsOut,
    TSlice
  >;

  return (set, get, api) =>
    Object.assign({}, initialState, creatorWithCombinedState(set, get, api));
}
"#;

const ZUSTAND_DEVTOOLS_TS: &str = r#"import type { SetState, StateCreator, StoreApi } from "./vanilla";

export type DevtoolsAction =
  | string
  | {
      type: string;
      [key: string]: unknown;
    };

export interface DevtoolsOptions {
  name?: string;
  enabled?: boolean;
  anonymousActionType?: string;
  trace?: boolean;
  traceLimit?: number;
  serialize?: boolean | Record<string, unknown>;
}

export type NamedSet<TState> = (
  partial: Parameters<SetState<TState>>[0],
  replace?: boolean,
  action?: DevtoolsAction,
) => void;

type DevtoolsMessage = {
  type: string;
  payload?: unknown;
  state?: string;
};

type ReduxDevtoolsConnection<TState> = {
  init: (state: TState) => void;
  send: (action: { type: string }, state: TState) => void;
  subscribe?: (listener: (message: DevtoolsMessage) => void) => () => void;
  unsubscribe?: () => void;
};

type ReduxDevtoolsExtension<TState> = {
  connect: (
    options: Omit<DevtoolsOptions, "enabled" | "anonymousActionType">,
  ) => ReduxDevtoolsConnection<TState>;
};

type StoreWithDevtools<TState> = StoreApi<TState> & {
  devtools: {
    cleanup: () => void;
  };
};

type DispatchableStore = {
  dispatch?: (action: { type: string; [key: string]: unknown }) => unknown;
  dispatchFromDevtools?: unknown;
};

declare global {
  interface Window {
    __REDUX_DEVTOOLS_EXTENSION__?: ReduxDevtoolsExtension<unknown>;
  }
}

function hasDispatchFromDevtools(api: unknown): api is DispatchableStore {
  const candidate = api as DispatchableStore;
  return Boolean(candidate.dispatchFromDevtools) &&
    typeof candidate.dispatch === "function";
}

function toDevtoolsAction(
  action: DevtoolsAction | undefined,
  anonymousActionType: string,
) {
  if (!action) {
    return { type: anonymousActionType };
  }
  return typeof action === "string" ? { type: action } : action;
}

function parseDevtoolsState<TState>(message: DevtoolsMessage) {
  if (!message.state) {
    return undefined;
  }
  try {
    return JSON.parse(message.state) as TState;
  } catch {
    return undefined;
  }
}

const devtoolsImpl =
  <TState>(
    initializer: StateCreator<TState>,
    devtoolsOptions: DevtoolsOptions = {},
  ): StateCreator<TState> =>
  (set, get, api) => {
    const {
      enabled = typeof window !== "undefined" &&
        process.env.NODE_ENV !== "production",
      anonymousActionType = "anonymous",
      ...extensionOptions
    } = devtoolsOptions;
    const extension =
      typeof window === "undefined" || !enabled
        ? undefined
        : (window.__REDUX_DEVTOOLS_EXTENSION__ as
            | ReduxDevtoolsExtension<TState>
            | undefined);

    if (!extension) {
      return initializer(set, get, api);
    }

    const connection = extension.connect(extensionOptions);
    let isRecording = true;
    let unsubscribeFromExtension: (() => void) | undefined;

    const namedSet: NamedSet<TState> = (partial, replace, action) => {
      set(partial, replace);
      if (isRecording) {
        connection.send(toDevtoolsAction(action, anonymousActionType), get());
      }
    };

    const baseSetState = api.setState;
    api.setState = ((partial, replace) => {
      baseSetState(partial, replace);
      if (isRecording) {
        connection.send({ type: anonymousActionType }, api.getState());
      }
    }) as StoreApi<TState>["setState"];

    (api as StoreWithDevtools<TState>).devtools = {
      cleanup: () => {
        unsubscribeFromExtension?.();
        connection.unsubscribe?.();
      },
    };

    const initialState = initializer(namedSet, get, api);
    connection.init(initialState);

    unsubscribeFromExtension = connection.subscribe?.((message) => {
      if (message.type === "ACTION" && hasDispatchFromDevtools(api)) {
        let action = message.payload;
        if (typeof message.payload === "string") {
          try {
            action = JSON.parse(message.payload);
          } catch {
            return;
          }
        }
        if (
          action &&
          typeof action === "object" &&
          "type" in action &&
          typeof action.type === "string"
        ) {
          api.dispatch?.(action as { type: string; [key: string]: unknown });
        }
        return;
      }

      if (message.type !== "DISPATCH") {
        return;
      }

      const nextState = parseDevtoolsState<TState>(message);
      if (nextState === undefined) {
        return;
      }

      isRecording = false;
      api.setState(nextState, true);
      isRecording = true;
    });

    return initialState;
  };

export const devtools = devtoolsImpl;
"#;

const ZUSTAND_IMMER_TS: &str = r#"import { produce, type Draft } from "immer";

import type {
  SetState,
  StateCreator,
  StoreMutatorIdentifier,
} from "./vanilla";
import type { Write } from "./middleware";

export type ImmerSetState<TState> = {
  (
    nextStateOrUpdater:
      | TState
      | Partial<TState>
      | ((state: Draft<TState>) => void),
    shouldReplace?: false,
  ): void;
  (
    nextStateOrUpdater: TState | ((state: Draft<TState>) => void),
    shouldReplace: true,
  ): void;
};

export type WithImmer<TStore> = TStore extends {
  getState: () => infer TState;
}
  ? Write<TStore, { setState: ImmerSetState<TState> }>
  : never;

declare module "./vanilla" {
  interface StoreMutators<TStore, TPayload> {
    "zustand/immer": WithImmer<TStore>;
  }
}

export type Immer = <
  TState,
  MutatorsIn extends [StoreMutatorIdentifier, unknown][] = [],
  MutatorsOut extends [StoreMutatorIdentifier, unknown][] = [],
  Result = TState,
>(
  initializer: StateCreator<
    TState,
    [...MutatorsIn, ["zustand/immer", never]],
    MutatorsOut,
    Result
  >,
) => StateCreator<
  TState,
  MutatorsIn,
  [["zustand/immer", never], ...MutatorsOut],
  Result
>;

const immerImpl =
  <TState>(initializer: StateCreator<TState>): StateCreator<TState> =>
  (set, get, api) => {
    const setWithImmer: SetState<TState> = (updater, replace) => {
      const nextState =
        typeof updater === "function"
          ? produce(updater as (state: Draft<TState>) => void)
          : updater;

      set(nextState as Parameters<SetState<TState>>[0], replace);
    };

    api.setState = setWithImmer;
    return initializer(setWithImmer, get, api);
  };

export const immer = immerImpl as unknown as Immer;
"#;

const ZUSTAND_REDUX_TS: &str = r#"import type { StateCreator, StoreApi } from "./vanilla";

export type ReduxAction = { type: string; [key: string]: unknown };

export type ReduxDispatch<TAction extends ReduxAction> = (
  action: TAction,
) => TAction;

export type ReduxState<TAction extends ReduxAction> = {
  dispatch: ReduxDispatch<TAction>;
  dispatchFromDevtools: true;
};

export function redux<TState, TAction extends ReduxAction>(
  reducer: (state: TState, action: TAction) => TState,
  initialState: TState,
): StateCreator<TState & ReduxState<TAction>> {
  return (set, _get, api) => {
    const apiWithDispatch = api as StoreApi<TState & ReduxState<TAction>> &
      ReduxState<TAction>;
    const dispatch: ReduxDispatch<TAction> = (action) => {
      set((state) =>
        reducer(state as TState, action) as Partial<
          TState & ReduxState<TAction>
        >,
      );
      return action;
    };

    apiWithDispatch.dispatch = dispatch;
    apiWithDispatch.dispatchFromDevtools = true;

    return {
      dispatch,
      dispatchFromDevtools: true,
      ...initialState,
    };
  };
}
"#;

const ZUSTAND_SSR_SAFE_TS: &str = r#"import type { StateCreator, SetState, StoreApi } from "./vanilla";

export function ssrSafe<TState>(
  initializer: StateCreator<TState>,
  isSSR: boolean = typeof window === "undefined",
): StateCreator<TState> {
  return (set, get, api) => {
    if (!isSSR) {
      return initializer(set, get, api);
    }

    const ssrSet: SetState<TState> = () => {
      throw new Error("Cannot set state of Zustand store in SSR");
    };
    (api as StoreApi<TState>).setState = ssrSet;

    return initializer(ssrSet, get, api);
  };
}

export const unstable_ssrSafe = ssrSafe;
"#;

const ZUSTAND_SHALLOW_TS: &str = r#"type Entry = [unknown, unknown];

function isObject(value: unknown): value is object {
  return typeof value === "object" && value !== null;
}

function compareEntries(left: Entry[], right: Entry[]) {
  if (left.length !== right.length) {
    return false;
  }

  const rightMap = new Map(right);
  return left.every(([key, value]) => Object.is(value, rightMap.get(key)));
}

export function shallow<TValue>(left: TValue, right: TValue) {
  if (Object.is(left, right)) {
    return true;
  }
  if (!isObject(left) || !isObject(right)) {
    return false;
  }
  if (left instanceof Map && right instanceof Map) {
    return compareEntries([...left.entries()], [...right.entries()]);
  }
  if (left instanceof Set && right instanceof Set) {
    return compareEntries(
      [...left.values()].map((value) => [value, value]),
      [...right.values()].map((value) => [value, value]),
    );
  }

  return compareEntries(
    Object.entries(left as Record<string, unknown>),
    Object.entries(right as Record<string, unknown>),
  );
}

export function useShallow<TState, TSlice>(
  selector: (state: TState) => TSlice,
) {
  let previous: TSlice | undefined;

  return (state: TState) => {
    const next = selector(state);
    if (previous !== undefined && shallow(previous, next)) {
      return previous;
    }
    previous = next;
    return next;
  };
}
"#;

const ZUSTAND_PERSIST_TS: &str = r#"import type {
  StateCreator,
  StoreApi,
  StoreMutatorIdentifier,
} from "./vanilla";

export interface StateStorage {
  getItem: (name: string) => string | null | Promise<string | null>;
  setItem: (name: string, value: string) => void | Promise<void>;
  removeItem: (name: string) => void | Promise<void>;
}

export type StorageValue<TState> = {
  state: TState;
  version?: number;
};

export interface PersistStorage<TState> {
  getItem: (
    name: string,
  ) => StorageValue<TState> | null | Promise<StorageValue<TState> | null>;
  setItem: (name: string, value: StorageValue<TState>) => void | Promise<void>;
  removeItem: (name: string) => void | Promise<void>;
}

export type JsonStorageOptions = {
  reviver?: (key: string, value: unknown) => unknown;
  replacer?: (key: string, value: unknown) => unknown;
};

export type PersistListener<TState> = (state: TState) => void;

export type PersistOptions<TState, TPersistedState = TState> = {
  name: string;
  storage?: PersistStorage<TPersistedState>;
  partialize?: (state: TState) => TPersistedState;
  onRehydrateStorage?: (
    state: TState,
  ) => ((state?: TState, error?: unknown) => void) | void;
  version?: number;
  migrate?: (
    persistedState: unknown,
    version: number,
  ) => TPersistedState | Promise<TPersistedState>;
  merge?: (persistedState: unknown, currentState: TState) => TState;
  skipHydration?: boolean;
};

export type PersistApi<TState, TPersistedState = TState> = {
  persist: {
    setOptions: (
      options: Partial<PersistOptions<TState, TPersistedState>>,
    ) => void;
    getOptions: () => Partial<PersistOptions<TState, TPersistedState>>;
    rehydrate: () => Promise<void>;
    hasHydrated: () => boolean;
    clearStorage: () => void | Promise<void>;
    onHydrate: (listener: PersistListener<TState>) => () => void;
    onFinishHydration: (listener: PersistListener<TState>) => () => void;
  };
};

type WithPersist<TStore, TPersistedState> = TStore extends {
  getState: () => infer TState;
}
  ? Omit<TStore, keyof PersistApi<TState, TPersistedState>> &
      PersistApi<TState, TPersistedState>
  : never;

declare module "./vanilla" {
  interface StoreMutators<TStore, TPayload> {
    "zustand/persist": WithPersist<TStore, TPayload>;
  }
}

export function createJSONStorage<TState>(
  getStorage: () => StateStorage,
  options?: JsonStorageOptions,
): PersistStorage<TState> | undefined {
  let storage: StateStorage;
  try {
    storage = getStorage();
  } catch {
    return undefined;
  }

  return {
    async getItem(name) {
      const value = await storage.getItem(name);
      return value
        ? (JSON.parse(value, options?.reviver) as StorageValue<TState>)
        : null;
    },
    setItem(name, value) {
      return storage.setItem(name, JSON.stringify(value, options?.replacer));
    },
    removeItem(name) {
      return storage.removeItem(name);
    },
  };
}

type Persist = <
  TState,
  MutatorsIn extends [StoreMutatorIdentifier, unknown][] = [],
  MutatorsOut extends [StoreMutatorIdentifier, unknown][] = [],
  TPersistedState = TState,
>(
  initializer: StateCreator<
    TState,
    [...MutatorsIn, ["zustand/persist", unknown]],
    MutatorsOut
  >,
  options: PersistOptions<TState, TPersistedState>,
) => StateCreator<TState, MutatorsIn, [["zustand/persist", TPersistedState], ...MutatorsOut]>;

const persistImpl = <TState, TPersistedState = TState>(
  initializer: StateCreator<TState>,
  options: PersistOptions<TState, TPersistedState>,
): StateCreator<TState> => {
  return (set, get, api) => {
    let optionsWithDefaults = {
      version: 0,
      partialize: (state: TState) => state as unknown as TPersistedState,
      merge: (persistedState: unknown, currentState: TState) =>
        Object.assign({}, currentState, persistedState as object),
      ...options,
    };
    let storage =
      optionsWithDefaults.storage ??
      createJSONStorage<TPersistedState>(() => window.localStorage);
    let hydrated = false;
    const hydrationListeners = new Set<PersistListener<TState>>();
    const finishHydrationListeners = new Set<PersistListener<TState>>();

    const save = () =>
      storage?.setItem(optionsWithDefaults.name, {
        state: optionsWithDefaults.partialize(get()),
        version: optionsWithDefaults.version,
      });
    const baseSetState = api.setState;

    api.setState = (partial, replace) => {
      baseSetState(partial, replace);
      void save();
    };

    const initialState = initializer(
      (partial, replace) => {
        set(partial, replace);
        void save();
      },
      get,
      api,
    );

    const rehydrate = async () => {
      hydrated = false;
      hydrationListeners.forEach((listener) => listener(get()));
      const postRehydrationCallback =
        optionsWithDefaults.onRehydrateStorage?.(get());

      try {
        const stored = await storage?.getItem(optionsWithDefaults.name);
        if (!stored) {
          hydrated = true;
          postRehydrationCallback?.(get(), undefined);
          finishHydrationListeners.forEach((listener) => listener(get()));
          return;
        }

        const storedVersion = stored.version ?? 0;
        const persistedState =
          storedVersion !== optionsWithDefaults.version &&
          optionsWithDefaults.migrate
            ? await optionsWithDefaults.migrate(stored.state, storedVersion)
            : stored.state;

        set(optionsWithDefaults.merge(persistedState, get()), true);
        hydrated = true;
        postRehydrationCallback?.(get(), undefined);
        finishHydrationListeners.forEach((listener) => listener(get()));
      } catch (error) {
        postRehydrationCallback?.(undefined, error);
      }
    };

    (api as StoreApi<TState> & PersistApi<TState>).persist = {
      setOptions: (nextOptions) => {
        optionsWithDefaults = {
          ...optionsWithDefaults,
          ...nextOptions,
        };
        if (nextOptions.storage) {
          storage = nextOptions.storage as PersistStorage<TPersistedState>;
        }
      },
      getOptions: () => optionsWithDefaults,
      rehydrate,
      hasHydrated: () => hydrated,
      clearStorage: () => storage?.removeItem(optionsWithDefaults.name),
      onHydrate: (listener) => {
        hydrationListeners.add(listener);
        return () => {
          hydrationListeners.delete(listener);
        };
      },
      onFinishHydration: (listener) => {
        finishHydrationListeners.add(listener);
        return () => {
          finishHydrationListeners.delete(listener);
        };
      },
    };

    if (!optionsWithDefaults.skipHydration) {
      void rehydrate();
    }

    return initialState;
  };
};

export const persist = persistImpl as unknown as Persist;
"#;

const ZUSTAND_METADATA_TS: &str = r#"export const dxZustandForgePackage = {
  packageId: "state/zustand",
  officialName: "State Management",
  aliases: ["zustand", "npm/zustand", "pmndrs/zustand", "state/zustand-react"],
  upstreamPackage: "zustand",
  upstreamVersion: "5.0.13",
  forgeVersion: "5.0.13-dx.10",
  sourceMirror: "G:\\WWW\\inspirations\\zustand",
  provenance: {
    upstreamReference: "npm:zustand@5.0.13",
    repository: "https://github.com/pmndrs/zustand",
    license: "MIT",
    inspectedSources: [
      "package.json",
      "src/index.ts",
      "src/vanilla.ts",
      "src/react.ts",
      "src/traditional.ts",
      "src/middleware.ts",
      "src/middleware/persist.ts",
      "src/middleware/subscribeWithSelector.ts",
      "src/middleware/devtools.ts",
      "src/middleware/immer.ts",
      "src/middleware/redux.ts",
      "docs/reference/apis/create.md",
      "docs/reference/apis/create-store.md",
      "docs/reference/middlewares/persist.md",
      "docs/reference/middlewares/subscribe-with-selector.md",
    ],
  },
  publicApi: [
    "createStore",
    "create",
    "curried createStore",
    "curried create",
    "Mutate",
    "StoreMutators",
    "StoreMutatorIdentifier",
    "SubscribeWithSelector",
    "StoreSubscribeWithSelector",
    "WithSelectorSubscribe",
    "Write",
    "useStore",
    "useStoreWithEqualityFn",
    "createWithEqualityFn",
    "subscribeWithSelector",
    "combine",
    "devtools",
    "immer",
    "WithImmer",
    "ImmerSetState",
    "redux",
    "unstable_ssrSafe",
    "shallow",
    "useShallow",
    "createJSONStorage",
    "PersistApi",
    "PersistOptions",
    "persist mutator typing",
    "persist",
    "persist.rehydrate",
  ],
  requiredEnv: [],
  appOwnedBoundaries: [
    "selector granularity",
    "custom equality policy",
    "DevTools availability and action taxonomy",
    "SSR data-fetching policy",
    "sensitive-state policy",
    "durable storage",
    "app-owned immer dependency",
    "draft mutation conventions",
    "persistence durability",
    "browser persistence review",
  ],
  exportedFiles: [
    "@/lib/forge/state/zustand/index.ts",
    "@/lib/forge/state/zustand/vanilla.ts",
    "@/lib/forge/state/zustand/react.ts",
    "@/lib/forge/state/zustand/traditional.ts",
    "@/lib/forge/state/zustand/middleware.ts",
    "@/lib/forge/state/zustand/devtools.ts",
    "@/lib/forge/state/zustand/immer.ts",
    "@/lib/forge/state/zustand/redux.ts",
    "@/lib/forge/state/zustand/ssr-safe.ts",
    "@/lib/forge/state/zustand/shallow.ts",
    "@/lib/forge/state/zustand/persist.ts",
    "@/lib/forge/state/zustand/metadata.ts",
  ],
  materializedFiles: [
    "state/zustand/index.ts",
    "state/zustand/vanilla.ts",
    "state/zustand/react.ts",
    "state/zustand/traditional.ts",
    "state/zustand/middleware.ts",
    "state/zustand/devtools.ts",
    "state/zustand/immer.ts",
    "state/zustand/redux.ts",
    "state/zustand/ssr-safe.ts",
    "state/zustand/shallow.ts",
    "state/zustand/persist.ts",
    "state/zustand/metadata.ts",
    "state/zustand/README.md",
  ],
  receiptPaths: [
    "docs/packages/state-zustand.md",
    ".dx/forge/receipts/*-state-zustand.json",
    "examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
    ".dx/forge/docs/state-zustand.md",
    ".dx/forge/source-manifest.json",
  ],
  dxIcon: "state:zustand",
  dashboardUsage: {
    shellBinding: {
      sourceFile: "examples/template/template-shell.tsx",
      materializedFile: "components/template-app/template-shell.tsx",
      component: "LaunchDashboardStateShell",
      marker: 'data-dx-component="launch-dashboard-state-shell"',
    },
    runtimeSurface: {
      id: "launch-runtime-dashboard-state-shell",
      materializedFile: "pages/index.html",
      selector: '[data-dx-component="launch-dashboard-state-shell"]',
      receiptPath: ".dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
    },
    sourceFile: "examples/template/state-zustand-dashboard.tsx",
    visibleComponent: "LaunchDashboardStateControl",
    routeComponent: "launch-zustand-dashboard-route-workflow",
    packageMarker: 'data-dx-package="state/zustand"',
    componentMarker: 'data-dx-component="launch-dashboard-state-workflow"',
    interactions: ["set-dashboard-density", "select-dashboard-focus", "toggle-command-hints", "save-dashboard-settings", "reset-dashboard-settings", "rehydrate-dashboard-settings"],
    persistenceKey: "dx-launch-dashboard-settings",
    actionStateMarkers: ["data-dx-zustand-hydration-event", "data-dx-zustand-rehydrate-state"],
    operatorControls: {
      sourceFile: "examples/template/shadcn-dashboard-controls.tsx",
      materializedFile: "components/launch/shadcn-dashboard-controls.tsx",
      component: "LaunchShadcnDashboardControls",
      marker: 'data-dx-component="shadcn-dashboard-controls"',
      actions: ["set-dashboard-density", "select-dashboard-focus", "toggle-command-hints", "preview-dashboard-receipt"],
    },
    counterProof: {
      sourceFile: "examples/template/state-zustand-counter.tsx",
      visibleComponent: "LaunchCounterControl",
      marker: 'data-dx-component="zustand-state-card"',
      actions: ["increment", "toggle-review-mode", "reset", "rehydrate"],
      persistKey: "dx-launch-counter",
    },
    starterDashboard: {
      sourceFile: "examples/dashboard/src/components/ZustandSettingsPanel.tsx",
      storeFile: "examples/dashboard/src/lib/dashboardSettingsStore.ts",
      component: "ZustandSettingsPanel",
      marker: 'data-dx-component="dashboard-zustand-settings-workflow"',
      persistKey: "dx-dashboard-settings",
      actions: ["toggle-density", "toggle-sidebar", "toggle-command-hints", "rehydrate-settings", "clear-saved-settings", "save-settings"],
      hydrationEvents: ["onHydrate", "onFinishHydration"],
      hydrationMarkers: ["data-dx-zustand-hydration-event", "data-dx-zustand-hydration-count"],
    },
  },
  discovery: {
    dxAdd: "dx add zustand --write",
    importPath: "@/lib/forge/state/zustand",
    immerImportPath: "@/lib/forge/state/zustand/immer",
    templateRole: "launch-state",
  },
} as const;

export type DxZustandForgePackageMetadata = typeof dxZustandForgePackage;
"#;

const ZUSTAND_README_MD: &str = r#"# DX Forge State Management Slice

This package materializes the official DX **State Management** lane as a source-owned Zustand-compatible launch state slice from the upstream `zustand` 5.0.13 public surface. It is meant for DX launch templates that need reliable local state without package-manager lifecycle scripts or `node_modules` materialization.

## Included API

- `createStore`, `StoreApi`, `StateCreator`, `ExtractState`, `Mutate`, `StoreMutators`, and `StoreMutatorIdentifier` from the vanilla store surface, including the upstream curried `createStore<T>()(...)` overload and middleware mutator typing contracts.
- `create` and `useStore` for React through `useSyncExternalStore`, including the upstream curried `create<T>()(...)` overload.
- `useStoreWithEqualityFn` and mutator-aware `createWithEqualityFn` from the traditional equality-selector surface.
- `subscribeWithSelector`, `SubscribeWithSelector`, `StoreSubscribeWithSelector`, `WithSelectorSubscribe`, `Write`, and `combine` from the launch-useful middleware surface, including mutator-aware selector subscription typing.
- `devtools`, `DevtoolsOptions`, and `NamedSet` for the Redux DevTools extension connect/init/send/cleanup path.
- `immer`, `WithImmer`, and `ImmerSetState` for draft-style nested updates through Zustand's upstream `zustand/middleware/immer` mutator contract.
- `redux` for reducer/action dispatch stores, including the upstream `dispatchFromDevtools` compatibility flag.
- `unstable_ssrSafe` for App Router SSR mutation safety around client-owned stores.
- `shallow` and `useShallow` selector helpers.
- `createJSONStorage`, `PersistApi`, `PersistOptions`, and `persist` for small JSON persistence flows with the upstream persist hydration lifecycle and persist mutator typing.
- `metadata.ts` for DX CLI, Zed, and www-template discovery.

## Forge Metadata

- Package id: `state/zustand`.
- Official DX package name: `State Management`.
- Aliases: `zustand`, `npm/zustand`, `pmndrs/zustand`, and `state/zustand-react`.
- Upstream package: `zustand`.
- Source mirror: `G:\WWW\inspirations\zustand`.
- Provenance: `npm:zustand@5.0.13`, `https://github.com/pmndrs/zustand`, MIT license, and inspected upstream source/docs listed in `metadata.ts`.
- Exported files: `@/lib/forge/state/zustand/*` plus `@/lib/forge/state/zustand/metadata.ts`.
- Required env: none.
- Receipt paths: `docs/packages/state-zustand.md`, `.dx/forge/receipts/*-state-zustand.json`, `examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json`, `.dx/forge/docs/state-zustand.md`, and `.dx/forge/source-manifest.json`.
- Dashboard usage: `LaunchDashboardStateShell` binds the composed launch shell to `useLaunchDashboardSettings` with `data-dx-component="launch-dashboard-state-shell"`, density/focus/command-hint markers, explicit `data-dx-zustand-rehydrate-state`, and compact/comfortable dashboard spacing. The runtime-safe generated `/launch` page mirrors that shell with `launch-runtime-dashboard-state-shell` in `public/preview-manifest.json`, a visible `launch-dashboard-state-summary`, and the same `dx-launch-dashboard-settings` persistence markers. `LaunchDashboardStateControl` still exposes `data-dx-package="state/zustand"`, `data-dx-component="launch-dashboard-state-workflow"`, density, focused queue, command hints, save, reset, rehydrate dashboard settings, and `dx-launch-dashboard-settings` persistence markers. The source shell's `LaunchShadcnDashboardControls` also consumes `useLaunchDashboardSettings` so density, focus, command hints, dashboard rehydration state, and receipt previews are driven by the same store instead of local-only component state. The route keeps `LaunchCounterControl` as a small local audit control with `dx-launch-counter` hydration markers. The starter dashboard consumes `ZustandSettingsPanel` through `examples/dashboard/src/lib/dashboardSettingsStore.ts` with `dx-dashboard-settings` persistence, selector subscriptions, shallow equality, rehydrate, clear-saved, save-state actions, `onHydrate`/`onFinishHydration` persist hydration events, and visible hydration event/count markers.

## Boundaries

Curried vanilla and React factory overloads are included for real Zustand-compatible store authoring. Vanilla mutator type contracts, middleware mutator typing for `subscribeWithSelector`/`combine`, persist mutator typing, and Immer middleware are included for source-owned middleware compatibility. Traditional equality selector helpers are included for stable derived client-state selections, the DevTools extension bridge is included for local action/state inspection, Redux middleware is included for reducer-driven launch state, SSR mutation safety is included for App Router pre-render boundaries, and the persist hydration lifecycle is included for reviewed browser persistence. The `immer` npm package dependency remains app-owned because upstream exposes `zustand/middleware/immer` as an optional peer dependency. This is not the full Zustand package: it intentionally excludes upstream multi-store DevTools tracking and package-manager installation. Applications still own selector granularity, custom equality policy, action taxonomy, DevTools availability, persistence durability, SSR data-fetching policy, draft-mutation conventions, and sensitive-state review.

## Template Usage

```tsx
"use client";

import * as React from "react";

import {
  combine,
  createStore,
  createWithEqualityFn,
  devtools,
  persist,
  redux,
  shallow,
  subscribeWithSelector,
  unstable_ssrSafe,
  useStore,
  type Mutate,
  type ReduxState,
  type StoreApi,
} from "@/lib/forge/state/zustand";
import { immer } from "@/lib/forge/state/zustand/immer";

type LaunchCounterModel = {
  count: number;
};

type LaunchCounterAction = { type: "increment" } | { type: "reset" };
type LaunchCounterStore = LaunchCounterModel & ReduxState<LaunchCounterAction>;
type LaunchCounterAuditAction = LaunchCounterAction["type"] | "hydrated";
type LaunchCounterAudit = {
  lastAction: LaunchCounterAuditAction;
  updates: number;
  noteAction: (action: LaunchCounterAuditAction) => void;
};
type LaunchCounterAuditApi = Mutate<
  StoreApi<LaunchCounterAudit>,
  [["zustand/subscribeWithSelector", never], ["zustand/immer", never]]
>;

function launchCounterReducer(
  state: LaunchCounterModel,
  action: LaunchCounterAction,
) {
  switch (action.type) {
    case "increment":
      return { count: state.count + 1 };
    case "reset":
      return { count: 0 };
  }
}

export const useLaunchCounter = createWithEqualityFn<LaunchCounterStore>()(
  devtools(
    unstable_ssrSafe(
      persist(
        redux(launchCounterReducer, {
          count: 0,
        }),
        { name: "dx-launch-counter" },
      ),
    ),
    { name: "DX Launch Counter" },
  ),
  shallow,
);

const launchCounterAuditStore = createStore<LaunchCounterAudit>()(
  subscribeWithSelector(
    immer(
      combine(
        {
          lastAction: "hydrated" as LaunchCounterAuditAction,
          updates: 0,
        },
        (set) => ({
          noteAction: (action) =>
            set((state) => {
              state.lastAction = action;
              state.updates += 1;
            }),
        }),
      ),
    ),
  ),
);
const launchCounterAudit = launchCounterAuditStore as LaunchCounterAuditApi;

export function LaunchCounterAuditLabel() {
  React.useEffect(() => {
    const unsubscribeHydrate = useLaunchCounter.persist.onHydrate(() => {});
    const unsubscribeFinish = useLaunchCounter.persist.onFinishHydration(
      () => {},
    );
    const unsubscribeAudit = launchCounterAudit.subscribe(
      (state) => state.lastAction,
      () => {},
    );

    return () => {
      unsubscribeHydrate();
      unsubscribeFinish();
      unsubscribeAudit();
    };
  }, []);

  const audit = useStore(launchCounterAudit, (state) => ({
    lastAction: state.lastAction,
    updates: state.updates,
  }));

  return <span data-dx-zustand-vanilla-store>{audit.lastAction}</span>;
}
```

Keep server data, authorization, and durable storage in application code. Use this slice for browser-local launch state and reviewed template defaults.
"#;
