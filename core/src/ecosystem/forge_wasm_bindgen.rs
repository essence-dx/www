pub(crate) const WASM_BINDGEN_VERSION: &str = "0.2.121-dx.0";

pub(crate) fn wasm_bindgen_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/wasm/bindgen/loader.ts", WASM_BINDGEN_LOADER_TS),
        ("js/wasm/bindgen/react.tsx", WASM_BINDGEN_REACT_TSX),
        ("js/wasm/bindgen/example.tsx", WASM_BINDGEN_EXAMPLE_TSX),
        (
            "js/wasm/bindgen/dashboard-workflow.tsx",
            WASM_BINDGEN_DASHBOARD_WORKFLOW_TSX,
        ),
        ("js/wasm/bindgen/metadata.ts", WASM_BINDGEN_METADATA_TS),
        ("js/wasm/bindgen/README.md", WASM_BINDGEN_README_MD),
    ]
}

const WASM_BINDGEN_LOADER_TS: &str = r#"export type WasmBindgenBytes = ArrayBuffer | ArrayBufferView;

export type WasmBindgenRawInput =
  | RequestInfo
  | URL
  | WebAssembly.Module
  | WasmBindgenBytes
  | Response;

export type WasmBindgenInput =
  | WasmBindgenRawInput
  | Promise<WasmBindgenRawInput>
  | {
      module_or_path: WasmBindgenRawInput | Promise<WasmBindgenRawInput>;
      memory?: WebAssembly.Memory;
      thread_stack_size?: number;
    };

export type WasmBindgenRawSyncInput = WebAssembly.Module | WasmBindgenBytes;

export type WasmBindgenSyncInput =
  | WasmBindgenRawSyncInput
  | { module: WasmBindgenRawSyncInput; memory?: WebAssembly.Memory; thread_stack_size?: number };

export type WasmBindgenInit<TModule extends object> = (
  input?: WasmBindgenInput,
  memory?: WebAssembly.Memory,
) => PromiseLike<TModule> | TModule;

export type WasmBindgenSyncInit<TModule extends object> = (
  input: WasmBindgenSyncInput,
  memory?: WebAssembly.Memory,
) => TModule;

export type WasmBindgenFactory<TModule extends object> = {
  default: WasmBindgenInit<TModule>;
  initSync?: WasmBindgenSyncInit<TModule>;
} & Partial<TModule>;

export type WasmBindgenResettableModule = {
  __wbg_reset_state?: () => void;
};

export type WasmBindgenMemoryModule = {
  memory?: WebAssembly.Memory;
};

export type WasmBindgenAllocatorModule = {
  __wbindgen_malloc?: (size: number, align: number) => number;
  __wbindgen_realloc?: (ptr: number, oldSize: number, align: number, newSize: number) => number;
  __wbindgen_free?: (ptr: number, size: number, align: number) => void;
};

export type WasmBindgenExternrefModule = {
  __wbindgen_externrefs?: WebAssembly.Table;
};

export type WasmBindgenExceptionModule = {
  __externref_table_alloc?: () => number;
  __externref_table_dealloc?: (idx: number) => void;
  __wbindgen_exn_store?: (idx: number) => void;
};

export type WasmBindgenThreadModule = {
  __wbindgen_thread_destroy?: (a?: number, b?: number, c?: number) => void;
};

export type WasmBindgenClosureModule = {
  __wbindgen_destroy_closure?: (a: number, b: number) => void;
};

export type WasmBindgenClosureState = {
  a: number;
  b: number;
};

export type WasmBindgenStartModule = {
  __wbindgen_start?: (threadStackSize?: number) => void;
};

export type WasmBindgenResponseDiagnostics = {
  ok: boolean;
  responseType: Response["type"];
  contentType: string | null;
  instantiateStreamingSupported: boolean;
  expectedResponseType: boolean;
  shouldWarnAboutMime: boolean;
};

export type WasmBindgenModuleDiagnostics = {
  memory: WebAssembly.Memory | null;
  memoryPages: number | null;
  memoryBytes: number | null;
  externrefs: WebAssembly.Table | null;
  externrefTableLength: number | null;
  start: ((threadStackSize?: number) => void) | null;
  canInitializeExternrefs: boolean;
  canResetState: boolean;
  canDestroyThread: boolean;
  canDestroyClosure: boolean;
  canAllocateBytes: boolean;
  canReallocateBytes: boolean;
  canFreeBytes: boolean;
  canAllocateExternref: boolean;
  canDeallocateExternref: boolean;
  canStoreException: boolean;
};

export type WasmBindgenMemoryViews = {
  bytes: Uint8Array | null;
  dataView: DataView | null;
};

export type WasmBindgenEncodedString = {
  bytes: Uint8Array;
  length: number;
};

export type WasmBindgenAllocation = {
  ptr: number;
  length: number;
  align: number;
};

export type WasmBindgenStringAllocation = WasmBindgenAllocation & {
  encodedLength: number;
};

export type WasmBindgenLoadOptions<TModule extends object> = {
  input?: WasmBindgenInput;
  importModule: () => Promise<WasmBindgenFactory<TModule>>;
  memory?: WebAssembly.Memory;
  cache?: boolean;
  timeoutMs?: number;
  signal?: AbortSignal;
};

export type WasmBindgenSyncLoadOptions<TModule extends object> = {
  input: WasmBindgenSyncInput;
  factory: WasmBindgenFactory<TModule>;
  memory?: WebAssembly.Memory;
  cache?: boolean;
};

const moduleCache = new Map<string, Promise<object>>();
const syncModuleCache = new Map<string, object>();

export async function loadWasmBindgenModule<TModule extends object>(
  cacheKey: string,
  options: WasmBindgenLoadOptions<TModule>,
): Promise<TModule> {
  const resolvedOptions = assertWasmBindgenLoadOptions(options);
  if (typeof window === "undefined" && hasWasmBindgenFetchInput(resolvedOptions.input)) {
    throw new Error("Fetch-backed wasm-bindgen loading must run in a browser context.");
  }
  throwIfWasmBindgenAborted(resolvedOptions.signal);
  assertWasmBindgenImportModule(resolvedOptions.importModule);

  const shouldCache = resolvedOptions.cache ?? true;
  const resolvedCacheKey = assertWasmBindgenCacheKey(cacheKey);
  if (shouldCache) {
    const cachedSync = syncModuleCache.get(resolvedCacheKey);
    if (cachedSync) {
      return cachedSync as TModule;
    }

    const cached = moduleCache.get(resolvedCacheKey);
    if (cached) {
      return cached as Promise<TModule>;
    }
  }

  const pending = initializeWasmBindgenModule(resolvedOptions).catch((error: unknown) => {
    if (shouldCache) {
      deleteCachedWasmBindgenModule(resolvedCacheKey);
    }
    throw error;
  });
  if (shouldCache) {
    moduleCache.set(resolvedCacheKey, pending as Promise<object>);
  }

  return pending;
}

export async function preloadWasmBindgenModule<TModule extends object>(
  cacheKey: string,
  options: WasmBindgenLoadOptions<TModule>,
): Promise<void> {
  await loadWasmBindgenModule(cacheKey, options);
}

export async function reloadWasmBindgenModule<TModule extends object>(
  cacheKey: string,
  options: WasmBindgenLoadOptions<TModule>,
): Promise<TModule> {
  const resolvedCacheKey = assertWasmBindgenCacheKey(cacheKey);
  const resolvedOptions = assertWasmBindgenLoadOptions(options);
  throwIfWasmBindgenAborted(resolvedOptions.signal);
  assertWasmBindgenImportModule(resolvedOptions.importModule);
  deleteCachedWasmBindgenModule(resolvedCacheKey);
  syncModuleCache.delete(resolvedCacheKey);
  return loadWasmBindgenModule(resolvedCacheKey, resolvedOptions);
}

export function loadWasmBindgenModuleSync<TModule extends object>(
  cacheKey: string,
  options: WasmBindgenSyncLoadOptions<TModule>,
): TModule {
  const resolvedCacheKey = assertWasmBindgenCacheKey(cacheKey);
  const resolvedOptions = assertWasmBindgenSyncLoadOptions(options);
  const shouldCache = resolvedOptions.cache ?? true;
  if (shouldCache) {
    const cached = syncModuleCache.get(resolvedCacheKey);
    if (cached) {
      return cached as TModule;
    }
  }

  const factory = assertWasmBindgenFactory(resolvedOptions.factory);
  if (typeof factory.initSync !== "function") {
    throw new Error("Expected a wasm-bindgen initSync(input) export.");
  }

  const module = assertWasmBindgenInitializedModule(
    factory.initSync(resolvedOptions.input, resolvedOptions.memory),
  );
  if (shouldCache) {
    syncModuleCache.set(resolvedCacheKey, module);
    moduleCache.set(resolvedCacheKey, Promise.resolve(module));
  }

  return module;
}

export function reloadWasmBindgenModuleSync<TModule extends object>(
  cacheKey: string,
  options: WasmBindgenSyncLoadOptions<TModule>,
): TModule {
  const resolvedCacheKey = assertWasmBindgenCacheKey(cacheKey);
  const resolvedOptions = assertWasmBindgenSyncLoadOptions(options);
  deleteCachedWasmBindgenModule(resolvedCacheKey);
  syncModuleCache.delete(resolvedCacheKey);
  return loadWasmBindgenModuleSync(resolvedCacheKey, resolvedOptions);
}

async function initializeWasmBindgenModule<TModule extends object>(
  options: WasmBindgenLoadOptions<TModule>,
): Promise<TModule> {
  const factory = assertWasmBindgenFactory(
    await withWasmBindgenAsyncBoundaries(Promise.resolve(options.importModule()), options),
  );
  const init = factory.default;
  if (typeof init !== "function") {
    throw new Error("Expected a wasm-bindgen default init(input) export.");
  }

  const initialized = init(options.input, options.memory);
  if (isWasmBindgenPromiseLike(initialized)) {
    return withWasmBindgenAsyncBoundaries(
      Promise.resolve(initialized).then(assertWasmBindgenInitializedModule),
      options,
    );
  }

  return assertWasmBindgenInitializedModule(initialized);
}

export function clearWasmBindgenModuleCache(cacheKey?: string) {
  if (cacheKey !== undefined) {
    const resolvedCacheKey = assertWasmBindgenCacheKey(cacheKey);
    deleteCachedWasmBindgenModule(resolvedCacheKey);
    syncModuleCache.delete(resolvedCacheKey);
    return;
  }

  moduleCache.clear();
  syncModuleCache.clear();
}

export function hasCachedWasmBindgenModule(cacheKey: string): boolean {
  const resolvedCacheKey = assertWasmBindgenCacheKey(cacheKey);
  return moduleCache.has(resolvedCacheKey) || syncModuleCache.has(resolvedCacheKey);
}

export function cachedWasmBindgenModuleKeys(): string[] {
  return [...new Set([...syncModuleCache.keys(), ...moduleCache.keys()])];
}

export function wasmBindgenModuleCacheSize(): number {
  return cachedWasmBindgenModuleKeys().length;
}

export function resetWasmBindgenModuleState(module: object | null | undefined): boolean {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenResettableModule;
  const resetState = resolvedModule.__wbg_reset_state;
  if (resetState === undefined) {
    return false;
  }

  if (typeof resetState !== "function") {
    throw new Error("Expected wasm-bindgen __wbg_reset_state export to be a function.");
  }

  resetState();
  return true;
}

export function getWasmBindgenModuleMemory(
  module: object | null | undefined,
): WebAssembly.Memory | null {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenMemoryModule;
  const memory = resolvedModule.memory;
  if (memory === undefined) {
    return null;
  }

  if (!(memory instanceof WebAssembly.Memory)) {
    throw new Error("Expected wasm-bindgen memory export to be WebAssembly.Memory.");
  }

  return memory;
}

export function getWasmBindgenUint8Memory(
  module: object | null | undefined,
): Uint8Array | null {
  const memory = getWasmBindgenModuleMemory(module);
  return memory === null ? null : new Uint8Array(memory.buffer);
}

export function getWasmBindgenDataViewMemory(
  module: object | null | undefined,
): DataView | null {
  const memory = getWasmBindgenModuleMemory(module);
  return memory === null ? null : new DataView(memory.buffer);
}

export function getWasmBindgenMemoryViews(
  module: object | null | undefined,
): WasmBindgenMemoryViews {
  return {
    bytes: getWasmBindgenUint8Memory(module),
    dataView: getWasmBindgenDataViewMemory(module),
  };
}

export function encodeWasmBindgenString(value: string): WasmBindgenEncodedString {
  if (typeof value !== "string") {
    throw new Error("wasm-bindgen string encoding input must be a string.");
  }

  const bytes = new TextEncoder().encode(value);
  return {
    bytes,
    length: bytes.length,
  };
}

export function decodeWasmBindgenString(
  module: object | null | undefined,
  ptr: number,
  len: number,
): string {
  const bytes = getWasmBindgenUint8Memory(module);
  if (bytes === null) {
    throw new Error("Expected wasm-bindgen memory export before decoding a string.");
  }

  assertWasmBindgenMemoryRange(ptr, len, bytes.byteLength);

  return new TextDecoder("utf-8", { ignoreBOM: true, fatal: true }).decode(
    bytes.subarray(ptr, ptr + len),
  );
}

export function throwWasmBindgenError(
  module: object | null | undefined,
  ptr: number,
  len: number,
): never {
  throw new Error(decodeWasmBindgenString(module, ptr, len));
}

export function allocateWasmBindgenBytes(
  module: object | null | undefined,
  bytes: WasmBindgenBytes,
  align = 1,
): WasmBindgenAllocation {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenAllocatorModule;
  const malloc = assertWasmBindgenMalloc(resolvedModule.__wbindgen_malloc);
  const source = toWasmBindgenUint8Array(bytes);
  const resolvedAlign = assertWasmBindgenAlignment(align);
  const length = assertWasmBindgenAllocationSize(source.byteLength);
  const ptr = malloc(length, resolvedAlign);
  const memoryBytes = getWasmBindgenUint8Memory(resolvedModule);
  if (memoryBytes === null) {
    throw new Error("Expected wasm-bindgen memory export after allocating bytes.");
  }

  assertWasmBindgenMemoryRange(ptr, length, memoryBytes.byteLength);
  memoryBytes.set(source, ptr);

  return { ptr, length, align: resolvedAlign };
}

export function allocateWasmBindgenString(
  module: object | null | undefined,
  value: string,
): WasmBindgenStringAllocation {
  const encoded = encodeWasmBindgenString(value);
  const allocation = allocateWasmBindgenBytes(module, encoded.bytes, 1);
  return {
    ...allocation,
    encodedLength: encoded.length,
  };
}

export function reallocateWasmBindgenAllocation(
  module: object | null | undefined,
  allocation: WasmBindgenAllocation,
  nextLength: number,
): WasmBindgenAllocation {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenAllocatorModule;
  const realloc = assertWasmBindgenRealloc(resolvedModule.__wbindgen_realloc);
  const nextSize = assertWasmBindgenAllocationSize(nextLength);
  const align = assertWasmBindgenAlignment(allocation.align);
  assertWasmBindgenMemoryRange(allocation.ptr, allocation.length, Number.MAX_SAFE_INTEGER);
  const ptr = realloc(allocation.ptr, allocation.length, align, nextSize);
  const memoryBytes = getWasmBindgenUint8Memory(resolvedModule);
  if (memoryBytes === null) {
    throw new Error("Expected wasm-bindgen memory export after reallocating bytes.");
  }

  assertWasmBindgenMemoryRange(ptr, nextSize, memoryBytes.byteLength);

  return { ptr, length: nextSize, align };
}

export function freeWasmBindgenAllocation(
  module: object | null | undefined,
  allocation: WasmBindgenAllocation,
): void {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenAllocatorModule;
  const free = assertWasmBindgenFree(resolvedModule.__wbindgen_free);
  assertWasmBindgenMemoryRange(allocation.ptr, allocation.length, Number.MAX_SAFE_INTEGER);
  const align = assertWasmBindgenAlignment(allocation.align);
  free(allocation.ptr, allocation.length, align);
}

export function getWasmBindgenExternrefTable(
  module: object | null | undefined,
): WebAssembly.Table | null {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenExternrefModule;
  const externrefs = resolvedModule.__wbindgen_externrefs;
  if (externrefs === undefined) {
    return null;
  }

  if (!(externrefs instanceof WebAssembly.Table)) {
    throw new Error("Expected wasm-bindgen __wbindgen_externrefs export to be WebAssembly.Table.");
  }

  return externrefs;
}

export function initializeWasmBindgenExternrefTable(
  module: object | null | undefined,
): boolean {
  const externrefs = getWasmBindgenExternrefTable(module);
  if (externrefs === null) {
    return false;
  }

  const offset = externrefs.grow(4);
  externrefs.set(0, undefined);
  externrefs.set(offset + 0, undefined);
  externrefs.set(offset + 1, null);
  externrefs.set(offset + 2, true);
  externrefs.set(offset + 3, false);
  return true;
}

export function allocateWasmBindgenExternref(
  module: object | null | undefined,
  value: unknown,
): number {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenExceptionModule &
    WasmBindgenExternrefModule;
  const allocate = assertWasmBindgenExternrefAllocator(resolvedModule.__externref_table_alloc);
  const externrefs = getWasmBindgenExternrefTable(resolvedModule);
  if (externrefs === null) {
    throw new Error("Expected wasm-bindgen __wbindgen_externrefs export before allocating externref.");
  }

  const index = assertWasmBindgenExternrefIndex(allocate());
  externrefs.set(index, value);
  return index;
}

export function deallocateWasmBindgenExternref(
  module: object | null | undefined,
  index: number,
): boolean {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenExceptionModule;
  const deallocate = resolvedModule.__externref_table_dealloc;
  if (deallocate === undefined) {
    return false;
  }

  const resolvedIndex = assertWasmBindgenExternrefIndex(index);
  assertWasmBindgenExternrefDeallocator(deallocate)(resolvedIndex);
  return true;
}

export function storeWasmBindgenException(
  module: object | null | undefined,
  value: unknown,
): number {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenExceptionModule &
    WasmBindgenExternrefModule;
  const storeException = assertWasmBindgenExceptionStore(resolvedModule.__wbindgen_exn_store);
  const index = allocateWasmBindgenExternref(resolvedModule, value);
  storeException(index);
  return index;
}

export function destroyWasmBindgenThread(
  module: object | null | undefined,
  a?: number,
  b?: number,
  c?: number,
): boolean {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenThreadModule;
  const destroyThread = resolvedModule.__wbindgen_thread_destroy;
  if (destroyThread === undefined) {
    return false;
  }

  if (typeof destroyThread !== "function") {
    throw new Error("Expected wasm-bindgen __wbindgen_thread_destroy export to be a function.");
  }

  destroyThread(a, b, c);
  return true;
}

export function destroyWasmBindgenClosure(
  module: object | null | undefined,
  state: WasmBindgenClosureState,
): boolean {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenClosureModule;
  const destroyClosure = resolvedModule.__wbindgen_destroy_closure;
  if (destroyClosure === undefined) {
    return false;
  }

  if (typeof destroyClosure !== "function") {
    throw new Error("Expected wasm-bindgen __wbindgen_destroy_closure export to be a function.");
  }

  assertWasmBindgenClosureState(state);
  destroyClosure(state.a, state.b);
  state.a = 0;
  return true;
}

export function getWasmBindgenStartFunction(
  module: object | null | undefined,
): ((threadStackSize?: number) => void) | null {
  const resolvedModule = assertWasmBindgenInitializedModule(module) as WasmBindgenStartModule;
  const start = resolvedModule.__wbindgen_start;
  if (start === undefined) {
    return null;
  }

  if (typeof start !== "function") {
    throw new Error("Expected wasm-bindgen __wbindgen_start export to be a function.");
  }

  return start;
}

export function inspectWasmBindgenModule(
  module: object | null | undefined,
): WasmBindgenModuleDiagnostics {
  const resolvedModule = assertWasmBindgenInitializedModule(module);
  const memory = getWasmBindgenModuleMemory(resolvedModule);
  const externrefs = getWasmBindgenExternrefTable(resolvedModule);
  const start = getWasmBindgenStartFunction(resolvedModule);
  const resetState = (resolvedModule as WasmBindgenResettableModule).__wbg_reset_state;
  const destroyThread = (resolvedModule as WasmBindgenThreadModule).__wbindgen_thread_destroy;
  const destroyClosure =
    (resolvedModule as WasmBindgenClosureModule).__wbindgen_destroy_closure;
  const allocator = resolvedModule as WasmBindgenAllocatorModule;
  const exceptions = resolvedModule as WasmBindgenExceptionModule;

  assertOptionalWasmBindgenFunction(resetState, "__wbg_reset_state");
  assertOptionalWasmBindgenFunction(destroyThread, "__wbindgen_thread_destroy");
  assertOptionalWasmBindgenFunction(destroyClosure, "__wbindgen_destroy_closure");
  assertOptionalWasmBindgenFunction(allocator.__wbindgen_malloc, "__wbindgen_malloc");
  assertOptionalWasmBindgenFunction(allocator.__wbindgen_realloc, "__wbindgen_realloc");
  assertOptionalWasmBindgenFunction(allocator.__wbindgen_free, "__wbindgen_free");
  assertOptionalWasmBindgenFunction(exceptions.__externref_table_alloc, "__externref_table_alloc");
  assertOptionalWasmBindgenFunction(
    exceptions.__externref_table_dealloc,
    "__externref_table_dealloc",
  );
  assertOptionalWasmBindgenFunction(exceptions.__wbindgen_exn_store, "__wbindgen_exn_store");

  return {
    memory,
    memoryPages: memory === null ? null : memory.buffer.byteLength / 65536,
    memoryBytes: memory === null ? null : memory.buffer.byteLength,
    externrefs,
    externrefTableLength: externrefs === null ? null : externrefs.length,
    start,
    canInitializeExternrefs: externrefs !== null,
    canResetState: resetState !== undefined,
    canDestroyThread: destroyThread !== undefined,
    canDestroyClosure: destroyClosure !== undefined,
    canAllocateBytes: allocator.__wbindgen_malloc !== undefined,
    canReallocateBytes: allocator.__wbindgen_realloc !== undefined,
    canFreeBytes: allocator.__wbindgen_free !== undefined,
    canAllocateExternref: exceptions.__externref_table_alloc !== undefined,
    canDeallocateExternref: exceptions.__externref_table_dealloc !== undefined,
    canStoreException: exceptions.__wbindgen_exn_store !== undefined,
  };
}

export function inspectWasmBindgenResponse(response: Response): WasmBindgenResponseDiagnostics {
  const resolvedResponse = assertWasmBindgenResponse(response);
  const contentType = resolvedResponse.headers.get("Content-Type");
  const expectedResponseType = isWasmBindgenExpectedResponseType(resolvedResponse.type);
  return {
    ok: resolvedResponse.ok,
    responseType: resolvedResponse.type,
    contentType,
    instantiateStreamingSupported: typeof WebAssembly.instantiateStreaming === "function",
    expectedResponseType,
    shouldWarnAboutMime:
      resolvedResponse.ok && expectedResponseType && contentType !== "application/wasm",
  };
}

export function formatWasmBindgenResponseDiagnostics(
  diagnostics: WasmBindgenResponseDiagnostics,
): string {
  const resolvedDiagnostics = assertWasmBindgenResponseDiagnostics(diagnostics);

  if (resolvedDiagnostics.shouldWarnAboutMime) {
    return "Serve Wasm as application/wasm to keep instantiateStreaming fast.";
  }

  if (!resolvedDiagnostics.instantiateStreamingSupported) {
    return "WebAssembly.instantiateStreaming is unavailable; wasm-bindgen will fall back to instantiate.";
  }

  return resolvedDiagnostics.contentType ?? "No Wasm Content-Type provided.";
}

function deleteCachedWasmBindgenModule(cacheKey: string) {
  moduleCache.delete(cacheKey);
}

export function assertWasmBindgenCacheKey(cacheKey: string): string {
  const resolvedCacheKey = cacheKey.trim();
  if (!resolvedCacheKey) {
    throw new Error("wasm-bindgen cacheKey must be a non-empty string.");
  }

  return resolvedCacheKey;
}

export function assertWasmBindgenTimeout(timeoutMs: number): number {
  if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) {
    throw new Error("wasm-bindgen timeoutMs must be a positive finite number.");
  }

  return timeoutMs;
}

export function assertWasmBindgenResponseDiagnostics(
  diagnostics: WasmBindgenResponseDiagnostics | null | undefined,
): WasmBindgenResponseDiagnostics {
  if (diagnostics === null || diagnostics === undefined || typeof diagnostics !== "object") {
    throw new Error("wasm-bindgen response diagnostics must be an object from inspectWasmBindgenResponse.");
  }

  if (typeof diagnostics.ok !== "boolean") {
    throw new Error("wasm-bindgen response diagnostics ok must be a boolean.");
  }

  if (typeof diagnostics.responseType !== "string") {
    throw new Error("wasm-bindgen response diagnostics responseType must be a string.");
  }

  if (diagnostics.contentType !== null && typeof diagnostics.contentType !== "string") {
    throw new Error("wasm-bindgen response diagnostics contentType must be a string or null.");
  }

  if (typeof diagnostics.instantiateStreamingSupported !== "boolean") {
    throw new Error("wasm-bindgen response diagnostics instantiateStreamingSupported must be a boolean.");
  }

  if (typeof diagnostics.expectedResponseType !== "boolean") {
    throw new Error("wasm-bindgen response diagnostics expectedResponseType must be a boolean.");
  }

  if (typeof diagnostics.shouldWarnAboutMime !== "boolean") {
    throw new Error("wasm-bindgen response diagnostics shouldWarnAboutMime must be a boolean.");
  }

  return diagnostics;
}

export function assertWasmBindgenFactory<TModule extends object>(
  factory: WasmBindgenFactory<TModule> | null | undefined,
): WasmBindgenFactory<TModule> {
  if (factory === null || factory === undefined || typeof factory !== "object") {
    throw new Error("Expected importModule() to resolve a wasm-bindgen module object.");
  }

  return factory;
}

export function assertWasmBindgenImportModule<TModule extends object>(
  importModule: WasmBindgenLoadOptions<TModule>["importModule"],
): WasmBindgenLoadOptions<TModule>["importModule"] {
  if (typeof importModule !== "function") {
    throw new Error("Expected importModule to be a function returning wasm-bindgen glue.");
  }

  return importModule;
}

export function assertWasmBindgenLoadOptions<TModule extends object>(
  options: WasmBindgenLoadOptions<TModule> | null | undefined,
): WasmBindgenLoadOptions<TModule> {
  if (options === null || options === undefined || typeof options !== "object") {
    throw new Error("Expected wasm-bindgen load options to be an object.");
  }

  assertWasmBindgenAsyncMemory(options.memory);
  assertWasmBindgenAsyncInputOptions(options.input);

  return options;
}

export function assertWasmBindgenSyncLoadOptions<TModule extends object>(
  options: WasmBindgenSyncLoadOptions<TModule> | null | undefined,
): WasmBindgenSyncLoadOptions<TModule> {
  if (options === null || options === undefined || typeof options !== "object") {
    throw new Error("Expected wasm-bindgen sync load options to be an object.");
  }

  assertWasmBindgenSyncInput(options.input);
  assertWasmBindgenSyncMemory(options.memory);

  return options;
}

export function assertWasmBindgenSyncInput(input: WasmBindgenSyncInput | undefined) {
  if (input === undefined) {
    throw new Error("Expected wasm-bindgen initSync(input) input.");
  }

  if (input instanceof WebAssembly.Module || isWasmBindgenBufferSource(input)) {
    return;
  }

  if (
    typeof input === "object" &&
    input !== null &&
    "module" in input &&
    (input.module instanceof WebAssembly.Module || isWasmBindgenBufferSource(input.module))
  ) {
    const syncOptions = input as {
      module: WasmBindgenRawSyncInput;
      memory?: WebAssembly.Memory;
      thread_stack_size?: number;
    };
    assertWasmBindgenSyncMemory(syncOptions.memory);
    assertWasmBindgenThreadStackSize(syncOptions.thread_stack_size);
    return;
  }

  throw new Error("Expected wasm-bindgen initSync(input) to receive a module, bytes, or { module }.");
}

export function assertWasmBindgenAsyncInputOptions(input: WasmBindgenInput | undefined) {
  if (typeof input === "object" && input !== null && "module_or_path" in input) {
    const asyncOptions = input as {
      module_or_path: WasmBindgenRawInput | Promise<WasmBindgenRawInput>;
      memory?: WebAssembly.Memory;
      thread_stack_size?: number;
    };
    assertWasmBindgenAsyncMemory(asyncOptions.memory);
    assertWasmBindgenThreadStackSize(asyncOptions.thread_stack_size);
  }
}

export function assertWasmBindgenSyncMemory(memory: WebAssembly.Memory | undefined) {
  if (memory !== undefined && !(memory instanceof WebAssembly.Memory)) {
    throw new Error("Expected wasm-bindgen initSync(input, memory) memory to be WebAssembly.Memory.");
  }
}

export function assertWasmBindgenAsyncMemory(memory: WebAssembly.Memory | undefined) {
  if (memory !== undefined && !(memory instanceof WebAssembly.Memory)) {
    throw new Error("Expected wasm-bindgen init(input, memory) memory to be WebAssembly.Memory.");
  }
}

export function assertWasmBindgenThreadStackSize(threadStackSize: number | undefined) {
  if (
    threadStackSize !== undefined &&
    (typeof threadStackSize !== "number" ||
      !Number.isFinite(threadStackSize) ||
      threadStackSize === 0 ||
      threadStackSize % 65536 !== 0)
  ) {
    throw new Error("Expected wasm-bindgen thread_stack_size to be a non-zero multiple of 65536.");
  }
}

export function assertWasmBindgenAlignment(align: number): number {
  if (!Number.isInteger(align) || align <= 0) {
    throw new Error("wasm-bindgen allocation alignment must be a positive integer.");
  }

  return align;
}

export function assertWasmBindgenAllocationSize(size: number): number {
  if (!Number.isInteger(size) || size < 0) {
    throw new Error("wasm-bindgen allocation size must be a non-negative integer.");
  }

  return size;
}

export function assertWasmBindgenExternrefIndex(index: number): number {
  if (!Number.isInteger(index) || index < 0) {
    throw new Error("wasm-bindgen externref index must be a non-negative integer.");
  }

  return index;
}

export function assertWasmBindgenClosureState(
  state: WasmBindgenClosureState | null | undefined,
): WasmBindgenClosureState {
  if (state === null || state === undefined || typeof state !== "object") {
    throw new Error("wasm-bindgen closure state must be an object.");
  }

  if (!Number.isInteger(state.a) || state.a < 0) {
    throw new Error("wasm-bindgen closure state field a must be a non-negative integer.");
  }

  if (!Number.isInteger(state.b) || state.b < 0) {
    throw new Error("wasm-bindgen closure state field b must be a non-negative integer.");
  }

  return state;
}

export function assertWasmBindgenMemoryRange(ptr: number, len: number, byteLength: number) {
  if (!Number.isInteger(ptr) || ptr < 0) {
    throw new Error("wasm-bindgen memory pointer must be a non-negative integer.");
  }

  if (!Number.isInteger(len) || len < 0) {
    throw new Error("wasm-bindgen memory length must be a non-negative integer.");
  }

  if (!Number.isInteger(byteLength) || byteLength < 0) {
    throw new Error("wasm-bindgen memory byteLength must be a non-negative integer.");
  }

  if (ptr + len > byteLength) {
    throw new Error("wasm-bindgen memory range is outside the exported memory buffer.");
  }
}

export function assertWasmBindgenMalloc(
  malloc: WasmBindgenAllocatorModule["__wbindgen_malloc"],
): NonNullable<WasmBindgenAllocatorModule["__wbindgen_malloc"]> {
  if (typeof malloc !== "function") {
    throw new Error("Expected wasm-bindgen __wbindgen_malloc export to be a function.");
  }

  return malloc;
}

export function assertWasmBindgenRealloc(
  realloc: WasmBindgenAllocatorModule["__wbindgen_realloc"],
): NonNullable<WasmBindgenAllocatorModule["__wbindgen_realloc"]> {
  if (typeof realloc !== "function") {
    throw new Error("Expected wasm-bindgen __wbindgen_realloc export to be a function.");
  }

  return realloc;
}

export function assertWasmBindgenFree(
  free: WasmBindgenAllocatorModule["__wbindgen_free"],
): NonNullable<WasmBindgenAllocatorModule["__wbindgen_free"]> {
  if (typeof free !== "function") {
    throw new Error("Expected wasm-bindgen __wbindgen_free export to be a function.");
  }

  return free;
}

export function assertWasmBindgenExternrefAllocator(
  allocate: WasmBindgenExceptionModule["__externref_table_alloc"],
): NonNullable<WasmBindgenExceptionModule["__externref_table_alloc"]> {
  if (typeof allocate !== "function") {
    throw new Error("Expected wasm-bindgen __externref_table_alloc export to be a function.");
  }

  return allocate;
}

export function assertWasmBindgenExternrefDeallocator(
  deallocate: WasmBindgenExceptionModule["__externref_table_dealloc"],
): NonNullable<WasmBindgenExceptionModule["__externref_table_dealloc"]> {
  if (typeof deallocate !== "function") {
    throw new Error("Expected wasm-bindgen __externref_table_dealloc export to be a function.");
  }

  return deallocate;
}

export function assertWasmBindgenExceptionStore(
  storeException: WasmBindgenExceptionModule["__wbindgen_exn_store"],
): NonNullable<WasmBindgenExceptionModule["__wbindgen_exn_store"]> {
  if (typeof storeException !== "function") {
    throw new Error("Expected wasm-bindgen __wbindgen_exn_store export to be a function.");
  }

  return storeException;
}

export function assertWasmBindgenResponse(response: Response | null | undefined): Response {
  if (typeof Response !== "function" || !(response instanceof Response)) {
    throw new Error("Expected wasm-bindgen Response diagnostics input to be a Response.");
  }

  return response;
}

export function assertWasmBindgenInitializedModule<TModule extends object>(
  module: TModule | null | undefined,
): TModule {
  if (module === null || module === undefined || typeof module !== "object") {
    throw new Error("Expected wasm-bindgen init(input) to return a module object.");
  }

  return module;
}

function isWasmBindgenPromiseLike<TModule extends object>(
  value: PromiseLike<TModule> | TModule,
): value is PromiseLike<TModule> {
  return (
    (typeof value === "object" || typeof value === "function") &&
    value !== null &&
    typeof (value as PromiseLike<TModule>).then === "function"
  );
}

function isWasmBindgenBufferSource(value: unknown): value is WasmBindgenBytes {
  return value instanceof ArrayBuffer || ArrayBuffer.isView(value);
}

function toWasmBindgenUint8Array(bytes: WasmBindgenBytes): Uint8Array {
  if (bytes instanceof ArrayBuffer) {
    return new Uint8Array(bytes);
  }

  return new Uint8Array(bytes.buffer, bytes.byteOffset, bytes.byteLength);
}

function isWasmBindgenFetchInput(input: unknown): input is RequestInfo | URL {
  return (
    typeof input === "string" ||
    (typeof Request === "function" && input instanceof Request) ||
    (typeof URL === "function" && input instanceof URL)
  );
}

function isWasmBindgenExpectedResponseType(type: Response["type"]): boolean {
  return type === "basic" || type === "cors" || type === "default";
}

function assertOptionalWasmBindgenFunction(value: unknown, exportName: string) {
  if (value !== undefined && typeof value !== "function") {
    throw new Error(`Expected wasm-bindgen ${exportName} export to be a function.`);
  }
}

function hasWasmBindgenFetchInput(input: WasmBindgenInput | undefined): boolean {
  if (isWasmBindgenFetchInput(input)) {
    return true;
  }

  return (
    typeof input === "object" &&
    input !== null &&
    "module_or_path" in input &&
    !isWasmBindgenPromiseLike(input.module_or_path) &&
    isWasmBindgenFetchInput(input.module_or_path)
  );
}

function withWasmBindgenAsyncBoundaries<TModule extends object>(
  pending: Promise<TModule>,
  options: WasmBindgenLoadOptions<TModule>,
): Promise<TModule> {
  throwIfWasmBindgenAborted(options.signal);
  if (options.timeoutMs === undefined && options.signal === undefined) {
    return pending;
  }

  const races: Array<Promise<TModule> | Promise<never>> = [pending];
  let timeout: ReturnType<typeof setTimeout> | undefined;
  let removeAbortListener: (() => void) | undefined;

  if (options.timeoutMs !== undefined) {
    const resolvedTimeoutMs = assertWasmBindgenTimeout(options.timeoutMs);
    races.push(
      new Promise<never>((_, reject) => {
        timeout = setTimeout(() => {
          reject(new Error(`wasm-bindgen init(input) timed out after ${resolvedTimeoutMs}ms.`));
        }, resolvedTimeoutMs);
      }),
    );
  }

  if (options.signal !== undefined) {
    races.push(
      new Promise<never>((_, reject) => {
        const abort = () => reject(new Error("wasm-bindgen init(input) was aborted."));
        if (options.signal?.aborted) {
          abort();
          return;
        }
        options.signal.addEventListener("abort", abort, { once: true });
        removeAbortListener = () => options.signal?.removeEventListener("abort", abort);
      }),
    );
  }

  return Promise.race(races).finally(() => {
    if (timeout !== undefined) {
      clearTimeout(timeout);
    }
    removeAbortListener?.();
  });
}

function throwIfWasmBindgenAborted(signal?: AbortSignal) {
  if (signal?.aborted) {
    throw new Error("wasm-bindgen init(input) was aborted.");
  }
}
"#;

const WASM_BINDGEN_REACT_TSX: &str = r#""use client";

import * as React from "react";

import {
  assertWasmBindgenCacheKey,
  clearWasmBindgenModuleCache,
  destroyWasmBindgenThread,
  hasCachedWasmBindgenModule,
  initializeWasmBindgenExternrefTable,
  inspectWasmBindgenModule,
  loadWasmBindgenModule,
  preloadWasmBindgenModule,
  reloadWasmBindgenModule,
  resetWasmBindgenModuleState,
  type WasmBindgenInput,
  type WasmBindgenFactory,
  type WasmBindgenModuleDiagnostics,
} from "./loader";

export type WasmBindgenHookStatus =
  | "idle"
  | "loading"
  | "preloading"
  | "reloading"
  | "error";

export type WasmBindgenHookState<TModule extends object> = {
  module: TModule | null;
  cacheKey: string;
  status: WasmBindgenHookStatus;
  loading: boolean;
  error: Error | null;
  cached: boolean;
  ready: boolean;
  memory: WebAssembly.Memory | null;
  externrefs: WebAssembly.Table | null;
  start: ((threadStackSize?: number) => void) | null;
  diagnostics: WasmBindgenModuleDiagnostics | null;
  cancel: () => void;
  clear: () => void;
  destroyThread: (a?: number, b?: number, c?: number) => boolean;
  initializeExternrefs: () => boolean;
  preload: () => Promise<void>;
  reload: () => Promise<void>;
  reset: () => boolean;
};

export type UseWasmBindgenModuleOptions<TModule extends object> = {
  cacheKey: string;
  input?: WasmBindgenInput;
  importModule: () => Promise<WasmBindgenFactory<TModule>>;
  cache?: boolean;
  timeoutMs?: number;
  signal?: AbortSignal;
  enabled?: boolean;
};

export function useWasmBindgenModule<TModule extends object>(
  options: UseWasmBindgenModuleOptions<TModule>,
): WasmBindgenHookState<TModule> {
  const resolvedCacheKey = React.useMemo(
    () => assertWasmBindgenCacheKey(options.cacheKey),
    [options.cacheKey],
  );
  const [state, setState] = React.useState<
    Omit<
      WasmBindgenHookState<TModule>,
      | "ready"
      | "memory"
      | "externrefs"
      | "start"
      | "diagnostics"
      | "cancel"
      | "clear"
      | "destroyThread"
      | "initializeExternrefs"
      | "preload"
      | "reload"
      | "reset"
    >
  >({
    module: null,
    cacheKey: resolvedCacheKey,
    status: options.enabled === false ? "idle" : "loading",
    loading: Boolean(options.enabled ?? true),
    error: null,
    cached: hasCachedWasmBindgenModule(resolvedCacheKey),
  });
  const mountedRef = React.useRef(true);
  const loadAbortRef = React.useRef<AbortController | null>(null);
  const preloadAbortRef = React.useRef<AbortController | null>(null);
  const reloadAbortRef = React.useRef<AbortController | null>(null);
  const requestVersionRef = React.useRef(0);

  React.useEffect(() => {
    mountedRef.current = true;
    return () => {
      mountedRef.current = false;
      requestVersionRef.current += 1;
      loadAbortRef.current?.abort();
      loadAbortRef.current = null;
      preloadAbortRef.current?.abort();
      preloadAbortRef.current = null;
      reloadAbortRef.current?.abort();
      reloadAbortRef.current = null;
    };
  }, []);

  const cancel = React.useCallback(() => {
    requestVersionRef.current += 1;
    loadAbortRef.current?.abort();
    loadAbortRef.current = null;
    preloadAbortRef.current?.abort();
    preloadAbortRef.current = null;
    reloadAbortRef.current?.abort();
    reloadAbortRef.current = null;
    if (mountedRef.current) {
      setState((current) => ({
        ...current,
        cacheKey: resolvedCacheKey,
        status: "idle",
        loading: false,
        error: null,
        cached: hasCachedWasmBindgenModule(resolvedCacheKey),
      }));
    }
  }, [resolvedCacheKey]);

  const clear = React.useCallback(() => {
    requestVersionRef.current += 1;
    loadAbortRef.current?.abort();
    loadAbortRef.current = null;
    preloadAbortRef.current?.abort();
    preloadAbortRef.current = null;
    reloadAbortRef.current?.abort();
    reloadAbortRef.current = null;
    clearWasmBindgenModuleCache(resolvedCacheKey);
    if (mountedRef.current) {
      setState({
        module: null,
        cacheKey: resolvedCacheKey,
        status: "idle",
        loading: false,
        error: null,
        cached: false,
      });
    }
  }, [resolvedCacheKey]);

  const reset = React.useCallback(() => {
    if (state.module === null) {
      return false;
    }

    return resetWasmBindgenModuleState(state.module);
  }, [state.module]);

  const destroyThread = React.useCallback(
    (a?: number, b?: number, c?: number) => {
      if (state.module === null) {
        return false;
      }

      return destroyWasmBindgenThread(state.module, a, b, c);
    },
    [state.module],
  );

  const initializeExternrefs = React.useCallback(() => {
    if (state.module === null) {
      return false;
    }

    return initializeWasmBindgenExternrefTable(state.module);
  }, [state.module]);

  const preload = React.useCallback(async () => {
    const requestVersion = (requestVersionRef.current += 1);
    loadAbortRef.current?.abort();
    loadAbortRef.current = null;
    preloadAbortRef.current?.abort();
    if (options.enabled === false) {
      preloadAbortRef.current = null;
      if (mountedRef.current) {
        setState({
          module: null,
          cacheKey: resolvedCacheKey,
          status: "idle",
          loading: false,
          error: null,
          cached: hasCachedWasmBindgenModule(resolvedCacheKey),
        });
      }
      return;
    }

    const abortController = new AbortController();
    preloadAbortRef.current = abortController;
    const abortSignal = composeWasmBindgenSignals(options.signal, abortController.signal);

    if (mountedRef.current) {
      setState((current) => ({
        ...current,
        cacheKey: resolvedCacheKey,
        status: "preloading",
        loading: true,
        error: null,
      }));
    }

    try {
      await preloadWasmBindgenModule<TModule>(resolvedCacheKey, {
        ...options,
        signal: abortSignal.signal,
      });
      if (
        mountedRef.current &&
        preloadAbortRef.current === abortController &&
        requestVersionRef.current === requestVersion
      ) {
        setState((current) => ({
          ...current,
          cacheKey: resolvedCacheKey,
          status: "idle",
          loading: false,
          error: null,
          cached: hasCachedWasmBindgenModule(resolvedCacheKey),
        }));
      }
    } catch (error: unknown) {
      if (
        mountedRef.current &&
        preloadAbortRef.current === abortController &&
        requestVersionRef.current === requestVersion
      ) {
        setState((current) => ({
          ...current,
          cacheKey: resolvedCacheKey,
          status: "error",
          loading: false,
          error: error instanceof Error ? error : new Error(String(error)),
          cached: hasCachedWasmBindgenModule(resolvedCacheKey),
        }));
      }
    } finally {
      abortSignal.dispose();
      if (preloadAbortRef.current === abortController) {
        preloadAbortRef.current = null;
      }
    }
  }, [
    resolvedCacheKey,
    options.input,
    options.importModule,
    options.cache,
    options.timeoutMs,
    options.signal,
    options.enabled,
  ]);

  const reload = React.useCallback(async () => {
    const requestVersion = (requestVersionRef.current += 1);
    if (options.enabled === false) {
      if (mountedRef.current) {
        setState({
          module: null,
          cacheKey: resolvedCacheKey,
          status: "idle",
          loading: false,
          error: null,
          cached: hasCachedWasmBindgenModule(resolvedCacheKey),
        });
      }
      return;
    }

    loadAbortRef.current?.abort();
    loadAbortRef.current = null;
    preloadAbortRef.current?.abort();
    preloadAbortRef.current = null;
    reloadAbortRef.current?.abort();
    const abortController = new AbortController();
    reloadAbortRef.current = abortController;
    const abortSignal = composeWasmBindgenSignals(options.signal, abortController.signal);

    if (mountedRef.current) {
      setState({
        module: null,
        cacheKey: resolvedCacheKey,
        status: "reloading",
        loading: true,
        error: null,
        cached: false,
      });
    }

    try {
      const module = await reloadWasmBindgenModule<TModule>(resolvedCacheKey, {
        ...options,
        signal: abortSignal.signal,
      });
      if (
        mountedRef.current &&
        reloadAbortRef.current === abortController &&
        requestVersionRef.current === requestVersion
      ) {
        setState({
          module,
          cacheKey: resolvedCacheKey,
          status: "idle",
          loading: false,
          error: null,
          cached: hasCachedWasmBindgenModule(resolvedCacheKey),
        });
      }
    } catch (error: unknown) {
      if (
        mountedRef.current &&
        reloadAbortRef.current === abortController &&
        requestVersionRef.current === requestVersion
      ) {
        setState({
          module: null,
          cacheKey: resolvedCacheKey,
          status: "error",
          loading: false,
          error: error instanceof Error ? error : new Error(String(error)),
          cached: hasCachedWasmBindgenModule(resolvedCacheKey),
        });
      }
    } finally {
      abortSignal.dispose();
      if (reloadAbortRef.current === abortController) {
        reloadAbortRef.current = null;
      }
    }
  }, [
    resolvedCacheKey,
    options.input,
    options.importModule,
    options.cache,
    options.timeoutMs,
    options.signal,
    options.enabled,
  ]);

  React.useEffect(() => {
    const requestVersion = (requestVersionRef.current += 1);
    if (options.enabled === false) {
      setState({
        module: null,
        cacheKey: resolvedCacheKey,
        status: "idle",
        loading: false,
        error: null,
        cached: hasCachedWasmBindgenModule(resolvedCacheKey),
      });
      return;
    }

    let cancelled = false;
    const abortController = new AbortController();
    loadAbortRef.current = abortController;
    const abortSignal = composeWasmBindgenSignals(options.signal, abortController.signal);
    setState((current) => ({
      ...current,
      cacheKey: resolvedCacheKey,
      status: "loading",
      loading: true,
      error: null,
    }));

    loadWasmBindgenModule<TModule>(resolvedCacheKey, {
      ...options,
      signal: abortSignal.signal,
    })
      .finally(() => {
        if (loadAbortRef.current === abortController) {
          loadAbortRef.current = null;
        }
        abortSignal.dispose();
      })
      .then((module) => {
        if (!cancelled && requestVersionRef.current === requestVersion) {
          setState({
            module,
            cacheKey: resolvedCacheKey,
            status: "idle",
            loading: false,
            error: null,
            cached: hasCachedWasmBindgenModule(resolvedCacheKey),
          });
        }
      })
      .catch((error: unknown) => {
        if (!cancelled && requestVersionRef.current === requestVersion) {
          setState({
            module: null,
            cacheKey: resolvedCacheKey,
            status: "error",
            loading: false,
            error: error instanceof Error ? error : new Error(String(error)),
            cached: hasCachedWasmBindgenModule(resolvedCacheKey),
          });
        }
      });

    return () => {
      cancelled = true;
      requestVersionRef.current += 1;
      abortController.abort();
      if (loadAbortRef.current === abortController) {
        loadAbortRef.current = null;
      }
      abortSignal.dispose();
    };
  }, [
    resolvedCacheKey,
    options.input,
    options.importModule,
    options.cache,
    options.timeoutMs,
    options.signal,
    options.enabled,
  ]);

  const ready = state.module !== null && state.status === "idle" && state.error === null;
  const diagnostics = state.module === null ? null : inspectWasmBindgenModule(state.module);
  const memory = diagnostics?.memory ?? null;
  const externrefs = diagnostics?.externrefs ?? null;
  const start = diagnostics?.start ?? null;

  return {
    ...state,
    ready,
    memory,
    externrefs,
    start,
    diagnostics,
    cancel,
    clear,
    destroyThread,
    initializeExternrefs,
    preload,
    reload,
    reset,
  };
}

function composeWasmBindgenSignals(
  externalSignal: AbortSignal | undefined,
  internalSignal: AbortSignal,
): { signal: AbortSignal; dispose: () => void } {
  if (!externalSignal) {
    return { signal: internalSignal, dispose: () => {} };
  }

  if (externalSignal.aborted) {
    return { signal: externalSignal, dispose: () => {} };
  }

  const controller = new AbortController();
  const abort = () => controller.abort();
  externalSignal.addEventListener("abort", abort, { once: true });
  internalSignal.addEventListener("abort", abort, { once: true });
  return {
    signal: controller.signal,
    dispose: () => {
      externalSignal.removeEventListener("abort", abort);
      internalSignal.removeEventListener("abort", abort);
    },
  };
}
"#;

const WASM_BINDGEN_EXAMPLE_TSX: &str = r#""use client";

import { useWasmBindgenModule } from "./react";

type CounterWasm = {
  add(a: number, b: number): number;
};

export function WasmBindgenCounterExample() {
  const { module, ready, loading, error } = useWasmBindgenModule<CounterWasm>({
    cacheKey: "launch-counter-wasm",
    importModule: () => import("@/wasm/launch_counter"),
  });

  if (loading) {
    return <p>Loading WebAssembly...</p>;
  }

  if (error) {
    return <p role="alert">{error.message}</p>;
  }

  return <p>2 + 3 = {ready ? module.add(2, 3) : "unavailable"}</p>;
}
"#;

const WASM_BINDGEN_DASHBOARD_WORKFLOW_TSX: &str = r#""use client";

import * as React from "react";

import type { WasmBindgenFactory } from "./loader";
import { useWasmBindgenModule } from "./react";

type DashboardWasmModule = {
  add(a: number, b: number): number;
};

export type WasmBindgenDashboardWorkflowProps = {
  importModule?: () => Promise<WasmBindgenFactory<DashboardWasmModule>>;
};

const localAddWasmBytes = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60,
  0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01,
  0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20,
  0x00, 0x20, 0x01, 0x6a, 0x0b,
]);

async function importLocalAddModule(): Promise<WasmBindgenFactory<DashboardWasmModule>> {
  return {
    default: async () => {
      const instance = await WebAssembly.instantiate(localAddWasmBytes);
      const add = instance.instance.exports.add;

      if (typeof add !== "function") {
        throw new Error("Local dashboard wasm did not export add(a, b).");
      }

      return { add: add as DashboardWasmModule["add"] };
    },
  };
}

export function WasmBindgenDashboardWorkflow({
  importModule,
}: WasmBindgenDashboardWorkflowProps) {
  const [enabled, setEnabled] = React.useState(false);
  const status = useWasmBindgenModule<DashboardWasmModule>({
    cacheKey: "dx-dashboard-wasm-bindgen",
    importModule: importModule ?? importLocalAddModule,
    enabled,
  });
  const result = status.module?.add(2, 3) ?? null;

  return (
    <section
      className="grid gap-3 rounded-md border border-border bg-card p-4 text-card-foreground"
      data-dx-component="dashboard-wasm-bindgen-workflow"
      data-dx-package="wasm/bindgen"
      data-dx-dashboard-workflow="wasm-interop"
      data-dx-wasm-dashboard-status={
        status.error ? "error" : status.ready ? "ready" : enabled ? "loading" : "idle"
      }
      data-dx-style-surface="theme-token-card"
      data-dx-icon-search="wasm:bindgen"
      data-dx-node-modules="forbidden"
    >
      <header className="flex items-start gap-3">
        {React.createElement("dx-icon", {
          "aria-hidden": "true",
          name: "pack:wasm-bindgen",
        })}
        <div>
          <h2 className="text-base font-semibold">WebAssembly Bridge workflow</h2>
          <p className="text-sm text-muted-foreground">
            Run a local WebAssembly proof while app-generated wasm-bindgen glue
            stays explicit.
          </p>
        </div>
      </header>
      <button
        className="rounded-md border border-border px-3 py-2 text-sm text-primary hover:bg-accent"
        data-dx-wasm-dashboard-action="run-local-add"
        onClick={() => setEnabled(true)}
        type="button"
      >
        Run WebAssembly workflow
      </button>
      <p data-dx-wasm-add-result={result ?? "idle"}>
        {status.error
          ? status.error.message
          : result === null
            ? "No WebAssembly result yet."
            : `WebAssembly add(2, 3) = ${result}.`}
      </p>
    </section>
  );
}
"#;

const WASM_BINDGEN_METADATA_TS: &str = r###"export const dxWasmBindgenForgePackage = {
  officialName: "WebAssembly Bridge",
  packageId: "wasm/bindgen",
  aliases: [
    "webassembly-bridge",
    "webassembly/bridge",
    "wasm-bindgen",
    "dx-forge/wasm-bindgen",
  ],
  upstreamPackage: "wasm-bindgen",
  upstreamVersion: "0.2.121",
  basedOn: "wasm-bindgen 0.2.121",
  sourceMirror: "G:\\WWW\\inspirations\\wasm-bindgen",
  honestyLabel: "ADAPTER-BOUNDARY",
  provenance: {
    source: "curated-local-source-mirror",
    upstreamReference: "wasm-bindgen@0.2.121 local source mirror",
    inspectedFiles: [
      "Cargo.toml",
      "README.md",
      "src/lib.rs",
      "crates/cli/tests/reference/targets-target-web.js",
      "crates/cli/tests/reference/targets-target-web-atomics.d.ts",
      "crates/cli/tests/reference/wasm-export-types.js",
      "crates/cli/tests/reference/wasm-export-types.d.ts",
      "crates/cli/tests/reference/web-sys.bg.js",
      "crates/cli/tests/reference/closures.bg.js",
    ],
    verified: false,
    note:
      "DX mirrors the generated JavaScript and TypeScript glue contracts from the local wasm-bindgen source; this is curated source evidence, not live upstream provenance.",
  },
  sourceSurface: [
    "#[wasm_bindgen] macro exported through wasm_bindgen::prelude::*",
    "JsValue runtime handle",
    "Closure callback bridge",
    "wasm-bindgen-cli targets: bundler, web, nodejs, no-modules, deno, module",
    "generated InitInput and SyncInitInput BufferSource support",
    "generated fetch-backed init(input) support for string, Request, and URL inputs",
    "generated Response instantiateStreaming MIME fallback warning",
    "generated cached Uint8Array and DataView memory helper pattern",
    "generated TextEncoder and fatal UTF-8 TextDecoder string bridge pattern",
    "generated __wbg___wbindgen_throw import bridge pattern",
    "generated default init(input) JavaScript entrypoint",
    "generated initSync(input) JavaScript entrypoint",
    "generated InitOutput.memory WebAssembly.Memory export",
    "generated InitOutput.__wbindgen_malloc/__wbindgen_realloc/__wbindgen_free allocator exports",
    "generated InitOutput.__wbindgen_externrefs WebAssembly.Table export",
    "generated __wbindgen_init_externref_table import seeding for undefined, null, true, and false",
    "generated __externref_table_alloc/__externref_table_dealloc and __wbindgen_exn_store exception bridge exports",
    "generated InitOutput.__wbindgen_start() export",
    "generated InitOutput.__wbindgen_thread_destroy() export for atomics targets",
    "generated __wbindgen_destroy_closure export used by closure FinalizationRegistry cleanup",
    "generated __wbg_reset_state() JavaScript export when reset-state support is enabled",
    "validated load options, validated async memory option, validated sync load options, validated sync init input shape, validated BufferSource bytes input, validated browser-only fetch-backed init input, validated Response diagnostics input, validated Response diagnostics formatter input, Response MIME diagnostics, formatted Response diagnostics, module export diagnostics, validated sync memory option, validated thread_stack_size option, validated memory export access, source-owned Uint8Array memory view, source-owned DataView memory view, source-owned UTF-8 string encoding, source-owned UTF-8 string decoding, source-owned UTF-8 string allocation, source-owned throw bridge, validated memory string ranges, source-owned byte allocation, source-owned byte reallocation, source-owned allocation free helper, validated allocator alignment, validated allocation size, validated closure state cleanup, memory byte diagnostics, allocator export diagnostics, validated externref table access, externref table length diagnostics, source-mirrored externref table seeding, source-owned externref allocation, source-owned externref deallocation, source-owned exception storage bridge, validated externref indices, exception bridge diagnostics, validated start export access, validated thread-destroy export access, validated closure-destroy export access, validated cache keys, import function validation, module-shape validation, initialized module validation, optional reset-state export invocation, optional atomics thread-destroy invocation, optional closure-destroy invocation, async default init(input, memory), sync initSync(input, memory), promise-like init handling, import/init timeout boundaries, retry-safe cache eviction, preload warmup, cache reload, sync cache reload, cache inspection, cache-key diagnostics, cache-size diagnostics, hook cache-key normalization, hook resolved cache-key state, hook cache status, typed hook status, derived hook readiness, derived hook memory, derived hook externrefs, derived hook start function, derived hook diagnostics, disabled preload guard, hook cancel action, hook preload action, hook clear action, hook reload action, hook reset action, hook externref table initialization action, hook thread-destroy action, stale hook request suppression, hook reload cancellation, hook cleanup cancellation, and AbortSignal cancellation",
  ],
  selectedSurfaces: [
    "generated-module-loader",
    "react-hook",
    "dashboard-workflow",
    "launch-local-compute-dashboard",
    "response-mime-diagnostics",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: [
      "present",
      "stale",
      "missing-receipt",
      "blocked",
      "unsupported-surface",
    ],
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
    monitoredSurfaces: [
      "dashboard-wasm-bindgen-workflow",
      "launch-wasm-compute-dashboard-workflow",
      "wasm-bindgen-readiness-workflow",
    ],
  },
  generatedFiles: [
    "wasm/bindgen/loader.ts",
    "wasm/bindgen/react.tsx",
    "wasm/bindgen/example.tsx",
    "wasm/bindgen/dashboard-workflow.tsx",
    "wasm/bindgen/metadata.ts",
    "wasm/bindgen/README.md",
  ],
  exportedFiles: {
    loader: "wasm/bindgen/loader.ts",
    react: "wasm/bindgen/react.tsx",
    example: "wasm/bindgen/example.tsx",
    dashboardWorkflow: "wasm/bindgen/dashboard-workflow.tsx",
    metadata: "wasm/bindgen/metadata.ts",
    docs: "wasm/bindgen/README.md",
  },
  requiredEnv: [],
  appOwnedBoundaries: [
    "Rust crate source and #[wasm_bindgen] export design",
    "wasm32-unknown-unknown build profile and generated .wasm artifact",
    "wasm-bindgen CLI install/version pin and target selection",
    "Generated JavaScript glue import path and cache key ownership",
    "Browser security, CSP, MIME serving, memory growth, and performance review",
  ],
  receiptPaths: {
    package: ".dx/forge/receipts/wasm-bindgen.json",
    launch: ".dx/launch/receipts/wasm-bindgen-launch.json",
    dashboardWorkflow:
      "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
    sourceGuard: "benchmarks/wasm-bindgen-slice.test.ts",
  },
  dashboardUsage: {
    route: "/launch",
    sourceFile: "examples/dashboard/src/components/WasmBindgenWorkflow.tsx",
    launchSourceFile: "examples/template/wasm-interop-status.tsx",
    component: "WasmBindgenWorkflow",
    launchComponent: "LaunchWasmInteropStatus",
    launchDashboardComponent: "launch-wasm-compute-dashboard-workflow",
    previewManifestSurface: "launch-runtime-wasm-compute-dashboard",
    productSurface: "launch-dashboard",
    visibleWorkflow: "local WebAssembly add(a, b) readiness check plus app-owned generated-module readiness",
    markers: [
      'data-dx-package="wasm/bindgen"',
      'data-dx-component="launch-wasm-compute-dashboard-workflow"',
      'data-dx-dashboard-card="local-compute"',
      'data-dx-dashboard-workflow="local-compute-readiness"',
      'data-dx-component="wasm-bindgen-readiness-workflow"',
      'data-dx-wasm-action="run-local-add"',
      "data-dx-wasm-add-result",
    ],
  },
  icons: {
    source: "dx-icons",
    names: ["pack:wasm-bindgen"],
  },
  commands: {
    cargoBuild: "cargo build --target wasm32-unknown-unknown",
    bindgen: "wasm-bindgen target/wasm32-unknown-unknown/release/<crate>.wasm --target bundler --out-dir src/wasm",
    dxAdd: "dx add webassembly-bridge --write",
  },
  launchBoundary:
    "DX owns the loader and React integration source. Your Rust crate, wasm-bindgen CLI install, generated .wasm, and generated JS remain explicit project inputs.",
} as const;
"###;

const WASM_BINDGEN_README_MD: &str = r#"# DX Forge WebAssembly Bridge

This package gives DX-WWW launch templates a small source-owned WebAssembly Bridge for modules generated by `wasm-bindgen`.

Official DX package name: `WebAssembly Bridge`
Package id: `wasm/bindgen`
Upstream package: `wasm-bindgen`
Upstream version: `0.2.121`
Honesty label: `ADAPTER-BOUNDARY`

It is based on the real upstream surface: Rust exports are marked with `#[wasm_bindgen]`, the CLI emits JavaScript glue for targets such as `bundler`, `web`, `nodejs`, `no-modules`, `deno`, and `module`, and the generated JavaScript exposes a default `init(input)` function.

## Owned files

- `wasm/bindgen/loader.ts` loads, preloads, reloads, inspects, clears, and caches a generated wasm-bindgen module with load-options validation, default `init(input, memory)` support for `{ module_or_path }` generated glue, full `BufferSource` byte input support, browser-only fetch-backed string/`Request`/`URL` input guards, validated and formatted Response MIME diagnostics for the generated `instantiateStreaming` fallback warning, module export diagnostics for generated memory, start, externref, reset, allocator, closure, exception, and thread helpers, `initSync(input, memory)` support for already-imported generated glue, sync reload support, raw or `{ module }` sync input validation, optional `WebAssembly.Memory` validation, `thread_stack_size` validation for object-form async and sync init, typed access to the generated `memory` export, source-owned `Uint8Array` and `DataView` memory views that mirror generated glue helper patterns, source-owned UTF-8 string encode/decode/allocation/throw helpers that mirror generated `TextEncoder`, fatal `TextDecoder`, `passStringToWasm0`, and `__wbg___wbindgen_throw` boundaries, source-owned byte allocation, reallocation, and free helpers for generated `__wbindgen_malloc`, `__wbindgen_realloc`, and `__wbindgen_free`, typed access to the generated `__wbindgen_externrefs` table, source-mirrored seeding for the generated `__wbindgen_init_externref_table` import, source-owned externref allocation/deallocation and exception storage helpers for generated `__externref_table_alloc`, `__externref_table_dealloc`, and `__wbindgen_exn_store`, typed access to the generated `__wbindgen_start` export, optional `__wbg_reset_state()` invocation for generated reset-state glue, optional `__wbindgen_thread_destroy()` invocation for atomics glue, optional `__wbindgen_destroy_closure()` invocation for generated closure cleanup, validated cache keys, import-function checks, module-shape checks, initialized module validation, promise-like init handling, import/init timeout boundaries, cache-key/size diagnostics, and `AbortSignal` cancellation.
- `wasm/bindgen/react.tsx` exposes a Client Component hook for launch UI, normalizes cache keys through the loader validator, reports the resolved cache key, typed load/preload/reload/error status, derived readiness, derived `WebAssembly.Memory`, derived externref table, derived start function, module diagnostics, and whether the current module key is cached, includes cache-safe `cancel()`, `preload()`, `clear()`, `reload()`, optional generated-state `reset()`, optional externref table `initializeExternrefs()`, and optional atomics `destroyThread()` actions, respects disabled preload gates, suppresses stale load/reload completions, aborts replaced operations, and aborts pending async initialization during unmounts.
- `wasm/bindgen/example.tsx` shows a tiny typed usage path.
- `wasm/bindgen/dashboard-workflow.tsx` is the starter dashboard workflow. It uses the source-owned hook and loader boundary, runs a safe local WebAssembly `add(a, b)` proof, exposes `data-dx-package="wasm/bindgen"` / `data-dx-component="dashboard-wasm-bindgen-workflow"` markers, and keeps the app-generated wasm-bindgen module import explicit.
- `wasm/bindgen/metadata.ts` records the package id, aliases, local source mirror, inspected upstream reference files, exported Forge files, required environment values, app-owned boundaries, receipt paths, DX icon names, dashboard usage markers, upstream API surface, and launch boundary.

## Dashboard usage

The launch dashboard consumes this package through `LaunchWasmInteropStatus`, and the starter dashboard consumes it through `WasmBindgenWorkflow`. They expose `data-dx-package="wasm/bindgen"`, `data-dx-component="launch-wasm-compute-dashboard-workflow"`, `data-dx-component="wasm-bindgen-readiness-workflow"`, or `data-dx-component="dashboard-wasm-bindgen-workflow"`, a `data-dx-wasm-action="run-local-add"` / `data-dx-wasm-dashboard-action="run-local-add"` button, and a `data-dx-wasm-add-result` status marker. The `/launch` surface is also source-owned by the DX Studio edit contract as `wasm-compute-dashboard-workflow`. When the app has not supplied a generated wasm-bindgen module import, the dashboard shows that missing-config boundary and still offers a safe local WebAssembly add check with no template-local `node_modules` workflow.

The visible icon uses DX-owned icon syntax: `<dx-icon name="pack:wasm-bindgen" />`.

## Boundary

This is not a replacement for the Rust crate, the procedural macro, or the CLI. Build the Rust crate to `wasm32-unknown-unknown`, run `wasm-bindgen`, then point the loader at the generated JavaScript entrypoint your project owns.
"#;
