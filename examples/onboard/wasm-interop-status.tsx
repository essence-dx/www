"use client";

import * as React from "react";

import {
  formatWasmBindgenResponseDiagnostics,
  inspectWasmBindgenResponse,
  type WasmBindgenFactory,
  type WasmBindgenInput,
} from "@/wasm/bindgen/loader";
import { useWasmBindgenModule } from "@/wasm/bindgen/react";

type LaunchWasmModule = {
  add(a: number, b: number): number;
};

export type LaunchWasmInteropStatusProps = {
  input?: WasmBindgenInput;
  importModule?: () => Promise<WasmBindgenFactory<LaunchWasmModule>>;
  wasmResponse?: Response;
};

function WasmPackageIcon() {
  return React.createElement("dx-icon", {
    "aria-hidden": "true",
    className: "size-4 text-primary",
    "data-dx-icon": "pack:wasm-bindgen",
    name: "pack:wasm-bindgen",
  });
}

const localAddWasmBytes = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60,
  0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01,
  0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20,
  0x00, 0x20, 0x01, 0x6a, 0x0b,
]);

async function missingLaunchWasmModule(): Promise<
  WasmBindgenFactory<LaunchWasmModule>
> {
  throw new Error("Provide a wasm-bindgen generated module import.");
}

async function importLocalReadinessWasmBindgenModule(): Promise<
  WasmBindgenFactory<LaunchWasmModule>
> {
  return {
    default: async () => {
      const instance = await WebAssembly.instantiate(localAddWasmBytes);
      const add = instance.instance.exports.add;

      if (typeof add !== "function") {
        throw new Error("Local readiness wasm did not export add(a, b).");
      }

      return {
        add: add as LaunchWasmModule["add"],
      };
    },
  };
}

function createLocalWasmResponse() {
  if (typeof Response === "undefined") {
    return null;
  }

  return new Response(localAddWasmBytes, {
    headers: {
      "Content-Type": "application/wasm",
    },
  });
}

export function LaunchWasmInteropStatus({
  input,
  importModule,
  wasmResponse,
}: LaunchWasmInteropStatusProps) {
  const [localReadinessEnabled, setLocalReadinessEnabled] = React.useState(false);
  const localWasmResponse = React.useMemo(() => createLocalWasmResponse(), []);
  const resolvedImportModule =
    importModule ??
    (localReadinessEnabled ? importLocalReadinessWasmBindgenModule : missingLaunchWasmModule);
  const status = useWasmBindgenModule<LaunchWasmModule>({
    cacheKey: "dx-launch-wasm-status",
    input,
    importModule: resolvedImportModule,
    enabled: Boolean(importModule || localReadinessEnabled),
  });

  const diagnostics = status.diagnostics;
  const memoryPages = diagnostics?.memoryPages ?? null;
  const memoryBytes = diagnostics?.memoryBytes ?? null;
  const externrefTableLength = diagnostics?.externrefTableLength ?? null;
  const responseDiagnostics =
    wasmResponse || localWasmResponse
      ? inspectWasmBindgenResponse(wasmResponse ?? localWasmResponse)
      : null;
  const addResult = status.module?.add(2, 3) ?? null;
  const launchStatus = status.error
    ? "error"
    : status.loading
      ? "loading"
      : status.ready
        ? "ready"
        : "missing-app-module";

  return (
    <div
      className="grid gap-3 text-sm text-muted-foreground"
      data-dx-component="wasm-bindgen-readiness-workflow"
      data-dx-package="wasm/bindgen"
      data-dx-wasm-bindgen-status={launchStatus}
      data-dx-wasm-local-readiness-enabled={localReadinessEnabled ? "true" : "false"}
      data-dx-wasm-node-modules="forbidden"
    >
      <div
        className="rounded-md border border-border bg-card p-3"
        data-dx-wasm-interaction="missing-module-state"
      >
        <p className="flex items-center gap-2 font-medium text-foreground">
          <WasmPackageIcon />
          WebAssembly Bridge interop
        </p>
        <p>
          {importModule
            ? "Using an app-owned wasm-bindgen module import."
            : "No app-owned wasm-bindgen module is configured yet. The launch template can still run a local WebAssembly interop readiness check without node_modules."}
        </p>
      </div>

      <div
        className="grid gap-2 rounded-md border border-border p-3"
        data-dx-wasm-interaction="local-add-readiness"
      >
        <div className="flex flex-wrap items-center justify-between gap-2">
          <span>Local add(a, b) check</span>
          <button
            className="rounded-md border border-border px-3 py-2 text-xs font-medium text-primary transition hover:bg-accent"
            data-dx-wasm-action="run-local-add"
            onClick={() => setLocalReadinessEnabled(true)}
            type="button"
          >
            Run local wasm
          </button>
        </div>
        <p data-dx-wasm-add-result={addResult ?? "idle"}>
          WebAssembly add check: {addResult ?? "waiting for local readiness"}
        </p>
      </div>

      {status.error ? (
        <p className="text-destructive" role="alert">
          {status.error.message}
        </p>
      ) : null}

      {status.loading ? <p>Loading WebAssembly...</p> : null}

      <div
        className="grid gap-1"
        data-dx-wasm-interaction="runtime-diagnostics"
      >
        <p>
          Memory:{" "}
          {memoryPages === null
            ? "not exported"
            : `${memoryPages} page${memoryPages === 1 ? "" : "s"} / ${memoryBytes ?? 0} bytes`}
        </p>
        <p>Start export: {status.start ? "available" : "managed by generated init"}</p>
        <p>
          Text bridge:{" "}
          {status.memory
            ? "UTF-8 encode/decode/allocation/throw helpers ready"
            : "waiting for memory"}
        </p>
        <p>
          Externref table:{" "}
          {status.externrefs
            ? `${externrefTableLength ?? 0} slot${
                externrefTableLength === 1 ? "" : "s"
              }`
            : "not exported"}
        </p>
        <p>
          Exception bridge:{" "}
          {diagnostics?.canStoreException
            ? "externref exception store available"
            : "not exported"}
        </p>
        <p>
          Optional exports: reset{" "}
          {diagnostics?.canResetState ? "available" : "not exported"}, thread
          cleanup {diagnostics?.canDestroyThread ? "available" : "not exported"},
          closure cleanup {diagnostics?.canDestroyClosure ? "available" : "not exported"}
        </p>
        <p>
          Allocator:{" "}
          {diagnostics?.canAllocateBytes
            ? "byte allocation available"
            : "not exported"}
          {diagnostics?.canReallocateBytes ? " / realloc" : ""}
          {diagnostics?.canFreeBytes ? " / free" : ""}
        </p>
        {responseDiagnostics ? (
          <p data-dx-wasm-mime-status={responseDiagnostics.contentType ?? "none"}>
            Wasm MIME: {formatWasmBindgenResponseDiagnostics(responseDiagnostics)}
          </p>
        ) : null}
      </div>
    </div>
  );
}
