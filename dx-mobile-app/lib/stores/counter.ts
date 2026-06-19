export type DxStoreDefinition<TStore extends Record<string, unknown>> = TStore;
export type DxStoreAction<TStore, TResult = void> = (store: TStore) => TResult;
export type DxStoreEffect<TStore, TResult = unknown> = (store: TStore) => TResult;
export type DxStoreDerived<TStore, TResult> = (store: TStore) => TResult;

export function store<TStore extends Record<string, unknown>>(
  definition: TStore,
): DxStoreDefinition<TStore> {
  return definition;
}

export function state<TValue>(initial: TValue): TValue {
  return initial;
}

export function derived<TStore, TResult>(
  compute: DxStoreDerived<TStore, TResult>,
): TResult {
  return compute as unknown as TResult;
}

export function action<TStore, TResult = void>(
  handler: DxStoreAction<TStore, TResult>,
): DxStoreAction<TStore, TResult> {
  return handler;
}

export function effect<TStore, TResult = unknown>(
  observe: DxStoreEffect<TStore, TResult>,
): DxStoreEffect<TStore, TResult> {
  return observe;
}

type CounterStoreDraft = {
  count: number;
  label: string;
};

export const counterStore = store({
  count: state(1),
  label: state("Global counter"),
  doubled: derived((store: CounterStoreDraft) => store.count * 2),
  increment: action((store: CounterStoreDraft) => {
    store.count += 1;
  }),
  reset: action((store: CounterStoreDraft) => {
    store.count = 1;
  }),
  announce: effect((store: CounterStoreDraft) => store.label),
});
