import {
    createDxDashboardSettingsReceipt,
    dxDashboardSettingsExample,
    dxDashboardSettingsInvalidExample,
    formatDxDashboardSettingsIssues,
    safeParseDxDashboardSettingsForm,
    type DxDashboardSettingsFormInput,
    type DxDashboardSettingsIssue,
    type DxDashboardSettingsReceipt,
    type DxDashboardSettingsResult,
    type DxDashboardSettings,
} from './forge/validation/zod/dashboard-settings';

export type {
    DxDashboardSettings,
    DxDashboardSettingsFormInput,
    DxDashboardSettingsIssue,
    DxDashboardSettingsReceipt,
    DxDashboardSettingsResult,
} from './forge/validation/zod/dashboard-settings';

export const zodDashboardPackageMetadata = {
    packageId: 'validation/zod',
    officialName: 'Validation & Schemas',
    aliases: ['zod', 'zod/v4', 'schema/zod', 'validation/zod/v4'],
    upstreamPackage: 'zod',
    upstreamVersion: '4.4.3',
    sourceMirror: 'G:/WWW/inspirations/zod',
    provenance: {
        upstreamFiles: [
            'packages/zod/src/v4/classic/external.ts',
            'packages/zod/src/v4/classic/schemas.ts',
        ],
        publicApi: [
            'safeParse',
            'strictObject',
            'flattenError',
            'meta',
            'readonly',
        ],
    },
    exportedFiles: [
        'js/validation/zod/dashboard-settings.ts',
        'examples/dashboard/src/lib/forge/validation/zod/dashboard-settings.ts',
        'examples/dashboard/src/components/ZodSettingsValidator.tsx',
    ],
    requiredEnv: [],
    appOwnedBoundaries: [
        'settings persistence target',
        'authorization before account changes',
        'final product taxonomy and copy',
        'full arbitrary schema composition beyond this launch slice',
    ],
    receiptPaths: [
        'examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json',
        '.dx/forge/receipts/*-validation-zod.json',
        '.dx/forge/docs/validation-zod.md',
        'examples/dashboard/README.md#zod-settings-validation',
    ],
    dxCheckVisibility: {
        schema: 'dx.forge.package.dx_check_visibility',
        receiptPath:
            'examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json',
        currentStatus: 'present',
        statuses: [
            'present',
            'stale',
            'missing receipt',
            'blocked',
            'unsupported surface',
        ],
        monitoredSurfaces: [
            'dashboard-settings-validation',
            'launch-package-catalog',
            'starter-dashboard-settings-validator',
        ],
    },
    dashboardUsage: {
        route: '/launch',
        component: 'launch-settings-validation-summary',
        starterComponent: 'dashboard-zod-settings-validator',
        workflow: 'settings-validation',
        markers: [
            'data-dx-package="validation/zod"',
            'data-dx-component="launch-settings-validation-summary"',
            'data-dx-component="dashboard-zod-settings-validator"',
            'data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"',
            'data-dx-zod-dashboard-field-state',
            'data-dx-zod-dashboard-field-error',
        ],
    },
    iconName: 'pack:validation-zod',
} as const;

export const zodDashboardDraftValid = dxDashboardSettingsExample;
export const zodDashboardDraftInvalid = dxDashboardSettingsInvalidExample;

export function validateZodDashboardSettingsDraft(
    input: DxDashboardSettingsFormInput,
): {
    result: DxDashboardSettingsResult;
    issues: DxDashboardSettingsIssue[];
    receipt: DxDashboardSettingsReceipt;
    fieldErrors: Record<string, string[]>;
    settings: DxDashboardSettings | null;
} {
    const result = safeParseDxDashboardSettingsForm(input);
    const receipt = createDxDashboardSettingsReceipt(input);

    return {
        result,
        issues: formatDxDashboardSettingsIssues(input),
        receipt,
        fieldErrors: receipt.fieldErrors,
        settings: result.success ? result.data : null,
    };
}

export {
    createDxDashboardSettingsReceipt,
    formatDxDashboardSettingsIssues,
    safeParseDxDashboardSettingsForm,
};
