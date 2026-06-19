export type WasmBindgenDashboardScenarioId =
    | 'local-add'
    | 'generated-module';

export type WasmBindgenDashboardScenario = {
    id: WasmBindgenDashboardScenarioId;
    label: string;
    publicApi: string;
    boundary: string;
};

export type WasmBindgenDashboardReceipt = {
    packageId: 'wasm/bindgen';
    scenarioId: WasmBindgenDashboardScenarioId;
    status: 'local-readiness-ready' | 'missing-generated-module' | 'error';
    result: number | null;
    nextAction: string;
};

export type WasmBindgenDashboardPackage = {
    officialName: 'WebAssembly Bridge';
    packageId: 'wasm/bindgen';
    aliases: readonly [
        'webassembly-bridge',
        'webassembly/bridge',
        'wasm-bindgen',
        'dx-forge/wasm-bindgen',
    ];
    upstreamPackage: 'wasm-bindgen';
    upstreamVersion: '0.2.121';
    sourceMirror: 'G:/WWW/inspirations/wasm-bindgen';
    provenance: {
        source: 'curated-local-source-mirror';
        upstreamReference: string;
        inspectedFiles: readonly string[];
    };
    requiredEnv: readonly [];
    exportedFiles: readonly string[];
    receiptPaths: readonly string[];
    dxCheckVisibility: {
        schema: 'dx.forge.package.dx_check_visibility';
        currentStatus: 'present';
        statuses: readonly [
            'present',
            'stale',
            'missing-receipt',
            'blocked',
            'unsupported-surface',
        ];
        receiptPath: string;
    };
    launchDashboard: {
        component: 'launch-wasm-compute-dashboard-workflow';
        dashboardCard: 'local-compute';
        workflow: 'local-compute-readiness';
        productSurface: 'launch-dashboard';
        previewManifestSurface: 'launch-runtime-wasm-compute-dashboard';
    };
    appOwnedBoundaries: readonly string[];
};

export const wasmBindgenDashboardPackage = {
    officialName: 'WebAssembly Bridge',
    packageId: 'wasm/bindgen',
    aliases: [
        'webassembly-bridge',
        'webassembly/bridge',
        'wasm-bindgen',
        'dx-forge/wasm-bindgen',
    ],
    upstreamPackage: 'wasm-bindgen',
    upstreamVersion: '0.2.121',
    sourceMirror: 'G:/WWW/inspirations/wasm-bindgen',
    provenance: {
        source: 'curated-local-source-mirror',
        upstreamReference: 'wasm-bindgen@0.2.121 local source mirror',
        inspectedFiles: [
            'Cargo.toml',
            'README.md',
            'src/lib.rs',
            'crates/cli/tests/reference/targets-target-web.js',
            'crates/cli/tests/reference/wasm-export-types.js',
            'crates/cli/tests/reference/web-sys.bg.js',
            'crates/cli/tests/reference/closures.bg.js',
        ],
    },
    requiredEnv: [],
    exportedFiles: [
        'wasm/bindgen/loader.ts',
        'wasm/bindgen/react.tsx',
        'wasm/bindgen/dashboard-workflow.tsx',
        'wasm/bindgen/metadata.ts',
    ],
    receiptPaths: [
        '.dx/forge/receipts/wasm-bindgen.json',
        '.dx/launch/receipts/wasm-bindgen-launch.json',
        'examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json',
    ],
    dxCheckVisibility: {
        schema: 'dx.forge.package.dx_check_visibility',
        currentStatus: 'present',
        statuses: [
            'present',
            'stale',
            'missing-receipt',
            'blocked',
            'unsupported-surface',
        ],
        receiptPath:
            'examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json',
    },
    launchDashboard: {
        component: 'launch-wasm-compute-dashboard-workflow',
        dashboardCard: 'local-compute',
        workflow: 'local-compute-readiness',
        productSurface: 'launch-dashboard',
        previewManifestSurface: 'launch-runtime-wasm-compute-dashboard',
    },
    appOwnedBoundaries: [
        'Rust crate exports marked with #[wasm_bindgen]',
        'wasm32 build artifact',
        'wasm-bindgen CLI output directory',
        'generated JavaScript glue import path',
        'browser security, MIME, CSP, and memory review',
    ],
} as const satisfies WasmBindgenDashboardPackage;

export const wasmBindgenDashboardScenarios: readonly WasmBindgenDashboardScenario[] = [
    {
        id: 'local-add',
        label: 'Local add check',
        publicApi: 'WebAssembly.instantiate + useWasmBindgenModule boundary',
        boundary: 'Runs a safe local WebAssembly module while generated glue is absent.',
    },
    {
        id: 'generated-module',
        label: 'Generated module readiness',
        publicApi: 'WasmBindgenFactory.default(initInput)',
        boundary: 'App must provide the generated wasm-bindgen JS entrypoint and .wasm artifact.',
    },
];

export const localAddWasmBytes = new Uint8Array([
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60,
    0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01,
    0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20,
    0x00, 0x20, 0x01, 0x6a, 0x0b,
]);

export function getWasmBindgenDashboardScenario(
    scenarioId: WasmBindgenDashboardScenarioId,
): WasmBindgenDashboardScenario {
    return (
        wasmBindgenDashboardScenarios.find(
            (scenario) => scenario.id === scenarioId,
        ) || wasmBindgenDashboardScenarios[0]
    );
}

export async function createWasmBindgenDashboardReceipt(
    scenarioId: WasmBindgenDashboardScenarioId,
): Promise<WasmBindgenDashboardReceipt> {
    if (scenarioId === 'generated-module') {
        return {
            packageId: wasmBindgenDashboardPackage.packageId,
            scenarioId,
            status: 'missing-generated-module',
            result: null,
            nextAction:
                'Build the app-owned Rust crate, run wasm-bindgen, and pass the generated module import to the dashboard workflow.',
        };
    }

    if (typeof WebAssembly === 'undefined') {
        return {
            packageId: wasmBindgenDashboardPackage.packageId,
            scenarioId,
            status: 'error',
            result: null,
            nextAction: 'Run this workflow in a browser with WebAssembly support.',
        };
    }

    const instance = await WebAssembly.instantiate(localAddWasmBytes);
    const add = instance.instance.exports.add;
    if (typeof add !== 'function') {
        return {
            packageId: wasmBindgenDashboardPackage.packageId,
            scenarioId,
            status: 'error',
            result: null,
            nextAction: 'The local add WebAssembly fixture did not expose add(a, b).',
        };
    }

    return {
        packageId: wasmBindgenDashboardPackage.packageId,
        scenarioId,
        status: 'local-readiness-ready',
        result: add(2, 3) as number,
        nextAction:
            'Replace the local fixture with the app-owned wasm-bindgen generated module import when the Rust crate is ready.',
    };
}
