export type ShadcnDashboardDensity = 'comfortable' | 'compact';
export type ShadcnDashboardAccent = 'account' | 'billing' | 'launch';

export interface ShadcnDashboardPackage {
    id: string;
    label: string;
    publicApi: string;
    role: string;
}

export interface ShadcnDashboardReceipt {
    receiptId: string;
    status: 'local-preview-ready';
    density: ShadcnDashboardDensity;
    accent: ShadcnDashboardAccent;
    filter: string;
    notifications: boolean;
    sourceMirror: string;
    nextAction: string;
}

export const shadcnDashboardPackageMetadata = {
    packageId: 'shadcn/ui/button',
    officialName: 'UI Components',
    aliases: [
        'ui/button',
        'ui/badge',
        'ui/card',
        'ui/field',
        'ui/input',
        'ui/textarea',
        'ui/item',
        'ui/separator',
        'ui/label',
    ],
    upstreamPackage: 'shadcn-ui',
    upstreamPackages: [
        'shadcn-ui@0.0.1',
        '@radix-ui/react-slot@1.2.4',
        '@radix-ui/react-label@2.1.8',
        '@radix-ui/react-separator@1.1.8',
    ],
    upstreamVersion: '0.0.1',
    sourceMirror: 'G:/WWW/inspirations/shadcn-ui',
    sourceMirrors: [
        'G:/WWW/inspirations/shadcn-ui',
        'G:/WWW/inspirations/radix-primitives',
    ],
    docsPath: 'docs/packages/ui-components.md',
    provenance: {
        upstreamRegistry: 'apps/v4/registry/new-york-v4/ui',
        radixPrimitiveSources: [
            'packages/react/slot/src/slot.tsx',
            'packages/react/label/src/label.tsx',
            'packages/react/separator/src/separator.tsx',
        ],
        publicApi: [
            'Button',
            'buttonVariants',
            'Badge',
            'badgeVariants',
            'Card',
            'CardHeader',
            'CardAction',
            'CardContent',
            'Field',
            'FieldGroup',
            'FieldLabel',
            'Input',
            'Label',
            'Textarea',
            'Item',
            'ItemActions',
            'Separator',
        ],
        note: 'DX dashboard controls are composed from the local shadcn/ui v4 radix registry data-slot and variant contracts with Radix primitive provenance kept as metadata.',
    },
    exportedFiles: [
        'js/ui/button.tsx',
        'js/ui/badge.tsx',
        'js/ui/card.tsx',
        'js/ui/field.tsx',
        'js/ui/input.tsx',
        'js/ui/label.tsx',
        'js/ui/textarea.tsx',
        'js/ui/item.tsx',
        'js/ui/separator.tsx',
        'examples/dashboard/src/components/ShadcnDashboardControls.tsx',
        'examples/dashboard/src/lib/shadcnDashboardControls.ts',
    ],
    requiredEnv: [],
    appOwnedBoundaries: [
        'dashboard copy and setting taxonomy',
        'save handler and persistence target',
        'accessibility review for final app controls',
        'registry synchronization beyond the selected launch primitives',
    ],
    receiptPaths: [
        '.dx/forge/receipts/*-shadcn-ui-button.json',
        '.dx/forge/receipts/*-shadcn-ui-card.json',
        '.dx/forge/docs/shadcn-ui-button.md',
        'examples/dashboard/README.md#shadcnui-dashboard-controls',
    ],
    selectedSurfaces: [
        'button',
        'badge',
        'card',
        'label',
        'separator',
        'field',
        'item',
        'input',
        'textarea',
    ],
    dxCheckVisibility: {
        schema: 'dx.forge.package.dx_check_visibility',
        currentStatus: 'present',
        statuses: ['present', 'stale', 'missing receipt', 'blocked', 'unsupported surface'],
        receiptPath:
            'examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json',
    },
    honestyLabel: 'SOURCE-ONLY',
} as const;

export const shadcnDashboardPackages: ShadcnDashboardPackage[] = [
    {
        id: 'shadcn/ui/button',
        label: 'Button',
        publicApi: 'Button + buttonVariants',
        role: 'action controls',
    },
    {
        id: 'shadcn/ui/badge',
        label: 'Badge',
        publicApi: 'Badge + badgeVariants',
        role: 'status chips',
    },
    {
        id: 'shadcn/ui/card',
        label: 'Card',
        publicApi: 'Card, CardHeader, CardAction, CardContent',
        role: 'dashboard shell',
    },
    {
        id: 'shadcn/ui/field',
        label: 'Field',
        publicApi: 'Field, FieldGroup, FieldLabel',
        role: 'settings groups',
    },
    {
        id: 'shadcn/ui/label',
        label: 'Label',
        publicApi: 'Label',
        role: 'accessible names',
    },
    {
        id: 'shadcn/ui/input',
        label: 'Input',
        publicApi: 'Input',
        role: 'filter entry',
    },
    {
        id: 'shadcn/ui/textarea',
        label: 'Textarea',
        publicApi: 'Textarea',
        role: 'review notes',
    },
    {
        id: 'shadcn/ui/item',
        label: 'Item',
        publicApi: 'Item, ItemContent, ItemActions',
        role: 'setting rows',
    },
    {
        id: 'shadcn/ui/separator',
        label: 'Separator',
        publicApi: 'Separator',
        role: 'section rhythm',
    },
];

export const shadcnDashboardDensityOptions: Array<{
    id: ShadcnDashboardDensity;
    label: string;
    description: string;
}> = [
    {
        id: 'comfortable',
        label: 'Comfortable',
        description: 'Roomier dashboard controls for review sessions.',
    },
    {
        id: 'compact',
        label: 'Compact',
        description: 'Denser controls for operators who scan repeatedly.',
    },
];

export const shadcnDashboardAccentOptions: Array<{
    id: ShadcnDashboardAccent;
    label: string;
    packageId: string;
}> = [
    { id: 'account', label: 'Account', packageId: 'shadcn/ui/item' },
    { id: 'billing', label: 'Billing', packageId: 'shadcn/ui/field' },
    { id: 'launch', label: 'Launch', packageId: 'shadcn/ui/button' },
];

export function createShadcnDashboardReceipt(input: {
    density: ShadcnDashboardDensity;
    accent: ShadcnDashboardAccent;
    filter: string;
    notifications: boolean;
}): ShadcnDashboardReceipt {
    const normalizedFilter =
        input.filter.trim().toLowerCase().replace(/[^a-z0-9]+/g, '-') || 'all-controls';

    return {
        receiptId: `dx-shadcn-${input.density}-${input.accent}-${normalizedFilter}`,
        status: 'local-preview-ready',
        density: input.density,
        accent: input.accent,
        filter: input.filter.trim() || 'all controls',
        notifications: input.notifications,
        sourceMirror: shadcnDashboardPackageMetadata.sourceMirror,
        nextAction: 'Wire this receipt to the app-owned settings persistence action.',
    };
}
