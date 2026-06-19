export type Listener<TState> = (
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

export interface StoreApi<TState> {
    setState: SetState<TState>;
    getState: () => TState;
    getInitialState: () => TState;
    subscribe: Subscribe<TState>;
}

export type StateCreator<TState> = (
    set: SetState<TState>,
    get: () => TState,
    store: StoreApi<TState>,
) => TState;

export type EqualityChecker<TValue> = (
    left: TValue,
    right: TValue,
) => boolean;

export type SelectorListener<TSlice> = (
    slice: TSlice,
    previousSlice: TSlice,
) => void;

export type Subscribe<TState> = {
    (listener: Listener<TState>): () => void;
    <TSlice>(
        selector: (state: TState) => TSlice,
        listener: SelectorListener<TSlice>,
        options?: {
            equalityFn?: EqualityChecker<TSlice>;
            fireImmediately?: boolean;
        },
    ): () => void;
};

export interface StateStorage<R = void> {
    getItem: (name: string) => string | null;
    setItem: (name: string, value: string) => R;
    removeItem: (name: string) => R;
}

export interface StorageValue<TState> {
    state: TState;
    version?: number;
}

export interface PersistStorage<TState, R = void> {
    getItem: (name: string) => StorageValue<TState> | null;
    setItem: (name: string, value: StorageValue<TState>) => R;
    removeItem: (name: string) => R;
}

export type PersistListener<TState> = (state: TState) => void;

export interface PersistOptions<TState, TPersistedState = Partial<TState>> {
    name: string;
    storage?: PersistStorage<TPersistedState>;
    partialize?: (state: TState) => TPersistedState;
    version?: number;
    merge?: (persistedState: TPersistedState, currentState: TState) => TState;
    skipHydration?: boolean;
}

export interface PersistApi<TState> {
    clearStorage: () => void;
    setOptions: (options: Partial<PersistOptions<TState, unknown>>) => void;
    getOptions: () => PersistOptions<TState, unknown>;
    hasHydrated: () => boolean;
    rehydrate: () => void;
    onHydrate: (listener: PersistListener<TState>) => () => void;
    onFinishHydration: (listener: PersistListener<TState>) => () => void;
}

export type PersistStoreApi<TState> = StoreApi<TState> & {
    persist: PersistApi<TState>;
};

type CreateStore = {
    <TState>(initializer: StateCreator<TState>): StoreApi<TState>;
    <TState>(): (initializer: StateCreator<TState>) => StoreApi<TState>;
};

function createStoreImpl<TState>(
    initializer: StateCreator<TState>,
): StoreApi<TState> {
    let state: TState;
    let initialState: TState;
    const listeners = new Set<Listener<TState>>();

    const setState: SetState<TState> = (partial, replace) => {
        const nextState =
            typeof partial === 'function'
                ? (partial as (state: TState) => TState | Partial<TState>)(state)
                : partial;

        if (Object.is(nextState, state)) return;

        const previousState = state;
        state =
            replace || typeof nextState !== 'object' || nextState === null
                ? (nextState as TState)
                : Object.assign({}, state, nextState);
        listeners.forEach(listener => listener(state, previousState));
    };

    const subscribe = ((listener: Listener<TState>) => {
        listeners.add(listener);
        return () => listeners.delete(listener);
    }) as Subscribe<TState>;

    const api: StoreApi<TState> = {
        setState,
        getState: () => state,
        getInitialState: () => initialState,
        subscribe,
    };

    initialState = state = initializer(setState, api.getState, api);
    return api;
}

export const createStore = ((initializer?: StateCreator<unknown>) =>
    initializer ? createStoreImpl(initializer) : createStoreImpl) as CreateStore;

export function createJSONStorage<TState, R = void>(
    getStorage: () => StateStorage<R>,
): PersistStorage<TState, R> | undefined {
    let storage: StateStorage<R>;

    try {
        storage = getStorage();
    } catch {
        return undefined;
    }

    return {
        getItem: name => {
            const value = storage.getItem(name);
            return value === null ? null : JSON.parse(value);
        },
        setItem: (name, value) =>
            storage.setItem(name, JSON.stringify(value)),
        removeItem: name => storage.removeItem(name),
    };
}

function browserStorage<TState>() {
    return createJSONStorage<TState>(() => {
        const storage = (globalThis as { localStorage?: StateStorage }).localStorage;
        if (!storage) throw new Error('localStorage is unavailable');
        return storage;
    });
}

export function persist<TState extends object, TPersistedState = Partial<TState>>(
    initializer: StateCreator<TState>,
    options: PersistOptions<TState, TPersistedState>,
): StateCreator<TState> {
    return (set, get, api) => {
        let optionsWithDefaults = {
            version: 0,
            partialize: (state: TState) => state as unknown as TPersistedState,
            merge: (persistedState: TPersistedState, currentState: TState) => ({
                ...currentState,
                ...(persistedState as object),
            }),
            ...options,
        };
        let storage =
            optionsWithDefaults.storage ?? browserStorage<TPersistedState>();
        let hasHydrated = false;
        const hydrationListeners = new Set<PersistListener<TState>>();
        const finishHydrationListeners = new Set<PersistListener<TState>>();

        const writeStorage = () => {
            if (!storage) return;
            storage.setItem(optionsWithDefaults.name, {
                state: optionsWithDefaults.partialize(get()),
                version: optionsWithDefaults.version,
            });
        };

        const setAndPersist: SetState<TState> = (partial, replace) => {
            set(partial, replace);
            writeStorage();
        };
        const baseSetState = api.setState;

        api.setState = (partial, replace) => {
            baseSetState(partial, replace);
            writeStorage();
        };

        const readPersistedState = (currentState: TState) => {
            const value = storage?.getItem(optionsWithDefaults.name);

            if (!value || value.version !== optionsWithDefaults.version) {
                return currentState;
            }

            return optionsWithDefaults.merge(value.state, currentState);
        };

        const finishHydration = () => {
            hasHydrated = true;
            finishHydrationListeners.forEach(listener => listener(get()));
        };

        (api as PersistStoreApi<TState>).persist = {
            clearStorage: () => storage?.removeItem(optionsWithDefaults.name),
            setOptions: nextOptions => {
                optionsWithDefaults = {
                    ...optionsWithDefaults,
                    ...(nextOptions as Partial<
                        PersistOptions<TState, TPersistedState>
                    >),
                };
                if (nextOptions.storage) {
                    storage = nextOptions.storage as PersistStorage<TPersistedState>;
                }
            },
            getOptions: () => optionsWithDefaults as PersistOptions<TState, unknown>,
            hasHydrated: () => hasHydrated,
            rehydrate: () => {
                hasHydrated = false;
                hydrationListeners.forEach(listener => listener(get()));
                const nextState = readPersistedState(get());
                set(nextState, true);
                finishHydration();
            },
            onHydrate: listener => {
                hydrationListeners.add(listener);
                return () => {
                    hydrationListeners.delete(listener);
                };
            },
            onFinishHydration: listener => {
                finishHydrationListeners.add(listener);
                return () => {
                    finishHydrationListeners.delete(listener);
                };
            },
        };

        const initialState = initializer(setAndPersist, get, api);
        if (optionsWithDefaults.skipHydration) {
            return initialState;
        }

        const hydratedState = readPersistedState(initialState);
        hasHydrated = true;
        return hydratedState;
    };
}

function subscribeWithSelectorImpl<TState>(
    initializer: StateCreator<TState>,
): StateCreator<TState> {
    return (set, get, api) => {
        const baseSubscribe = api.subscribe as (
            listener: Listener<TState>,
        ) => () => void;

        api.subscribe = ((selectorOrListener: unknown, listener?: unknown, options?: {
            equalityFn?: EqualityChecker<unknown>;
            fireImmediately?: boolean;
        }) => {
            if (typeof listener !== 'function') {
                return baseSubscribe(selectorOrListener as Listener<TState>);
            }

            const selector = selectorOrListener as (state: TState) => unknown;
            const onSlice = listener as SelectorListener<unknown>;
            const equalityFn = options?.equalityFn ?? Object.is;
            let currentSlice = selector(api.getState());
            const unsubscribe = baseSubscribe(state => {
                const nextSlice = selector(state);
                if (equalityFn(currentSlice, nextSlice)) return;
                const previousSlice = currentSlice;
                currentSlice = nextSlice;
                onSlice(currentSlice, previousSlice);
            });

            if (options?.fireImmediately) {
                onSlice(currentSlice, currentSlice);
            }

            return unsubscribe;
        }) as Subscribe<TState>;

        return initializer(set, get, api);
    };
}

export const subscribeWithSelector = subscribeWithSelectorImpl;

export function shallow<TValue>(left: TValue, right: TValue) {
    if (Object.is(left, right)) return true;
    if (
        typeof left !== 'object' ||
        left === null ||
        typeof right !== 'object' ||
        right === null
    ) {
        return false;
    }

    const leftEntries = Object.entries(left as Record<string, unknown>);
    const rightEntries = Object.entries(right as Record<string, unknown>);
    if (leftEntries.length !== rightEntries.length) return false;

    const rightMap = new Map(rightEntries);
    return leftEntries.every(([key, value]) => Object.is(value, rightMap.get(key)));
}
