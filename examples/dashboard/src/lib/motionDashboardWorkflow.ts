export type MotionDashboardStageId = 'reveal' | 'measure' | 'reorder';
export type MotionDashboardReducedMotionPolicy = 'system' | 'preview';
export type MotionDashboardOrderDirection = 'previous' | 'next';
export type MotionDashboardOrderTarget = MotionDashboardOrderDirection | 'first' | 'last';
export type MotionDashboardReceiptStatus = 'local-preview-ready';
export type MotionDashboardSelectedSurfaceId =
    | 'provider-policy'
    | 'layout-reorder'
    | 'dashboard-workflow';
export type MotionDashboardDxCheckStatus =
    | 'present'
    | 'stale'
    | 'missing-receipt'
    | 'blocked'
    | 'unsupported-surface';

export interface MotionDashboardStage {
    id: MotionDashboardStageId;
    label: string;
    publicApi: string;
    packageExport: string;
    progress: number;
    appBoundary: string;
}

export interface MotionDashboardReceipt {
    receiptId: string;
    status: MotionDashboardReceiptStatus;
    packageId: 'animation/motion';
    officialPackageName: 'Motion & Animation';
    cliCommand: 'dx add motion-animation --write';
    stageId: MotionDashboardStageId;
    progress: number;
    orderedSurfaceIds: readonly MotionDashboardStageId[];
    selectedSurfaceIds: readonly MotionDashboardSelectedSurfaceId[];
    reducedMotionPolicy: MotionDashboardReducedMotionPolicy;
    sourceMirror: string;
    dxCheckVisibility: MotionDashboardDxCheckVisibility;
    nextAction: string;
}

export interface MotionDashboardPreference {
    orderedSurfaceIds: readonly MotionDashboardStageId[];
    reducedMotionPolicy: MotionDashboardReducedMotionPolicy;
}

export interface MotionDashboardSelectedSurface {
    id: MotionDashboardSelectedSurfaceId;
    files: readonly string[];
    upstreamPublicApis: readonly string[];
    appOwnedBoundary: string;
}

export interface MotionDashboardDxCheckLegendEntry {
    status: MotionDashboardDxCheckStatus;
    meaning: string;
}

export interface MotionDashboardDxCheckSurface {
    id: string;
    status: MotionDashboardDxCheckStatus;
    sourceFile: string;
    materializedFile: string;
    receiptPath: string;
    nextAction: string;
}

export interface MotionDashboardDxCheckVisibility {
    schema: 'dx.forge.package.dx_check_visibility';
    packageId: 'animation/motion';
    officialPackageName: 'Motion & Animation';
    currentStatus: MotionDashboardDxCheckStatus;
    statusLegend: readonly MotionDashboardDxCheckLegendEntry[];
    monitoredSurfaces: readonly MotionDashboardDxCheckSurface[];
}

export const motionDashboardPreferenceStorageKey = 'dx.launch.motion.dashboard';

export const motionDashboardOfficialPackageName = 'Motion & Animation';
export const motionDashboardCliCommand = 'dx add motion-animation --write';

export const motionDashboardInspectedSourceFiles = [
    'packages/motion/src/react.ts',
    'packages/framer-motion/src/index.ts',
    'packages/framer-motion/src/components/AnimatePresence/index.tsx',
    'packages/framer-motion/src/components/Reorder/Group.tsx',
    'packages/framer-motion/src/value/use-scroll.ts',
] as const;

export const motionDashboardSelectedSurfaces = [
    {
        id: 'provider-policy',
        files: ['js/motion/provider.tsx', 'js/motion/reveal.tsx'],
        upstreamPublicApis: ['MotionConfig', 'motion', 'useInView', 'useReducedMotion'],
        appOwnedBoundary: 'app-wide reduced-motion and route reveal policy',
    },
    {
        id: 'layout-reorder',
        files: ['js/motion/layout.tsx', 'js/motion/reorder.tsx', 'js/motion/presence.tsx'],
        upstreamPublicApis: ['LayoutGroup', 'Reorder', 'AnimatePresence', 'useDragControls'],
        appOwnedBoundary: 'persistent dashboard ordering and route-level choreography',
    },
    {
        id: 'dashboard-workflow',
        files: [
            'js/motion/dashboard-workflow.ts',
            'components/launch/motion-interaction-proof.tsx',
        ],
        upstreamPublicApis: ['useMotionValue', 'useScroll', 'useSpring', 'useAnimate'],
        appOwnedBoundary: 'governed runtime/browser proof and animation performance budget',
    },
] as const satisfies readonly MotionDashboardSelectedSurface[];

export const motionDashboardDxCheckVisibility = {
    schema: 'dx.forge.package.dx_check_visibility',
    packageId: 'animation/motion',
    officialPackageName: motionDashboardOfficialPackageName,
    currentStatus: 'present',
    statusLegend: [
        {
            status: 'present',
            meaning: 'selected Motion & Animation surfaces, receipt, and source markers are present',
        },
        {
            status: 'stale',
            meaning: 'materialized Motion & Animation files or hashes no longer match the Forge receipt',
        },
        {
            status: 'missing-receipt',
            meaning: 'selected Motion & Animation surfaces exist without the dashboard workflow receipt',
        },
        {
            status: 'blocked',
            meaning: 'runtime proof or app-owned policy approval is required before claiming more',
        },
        {
            status: 'unsupported-surface',
            meaning: 'a requested Motion & Animation surface is outside the selected upstream-backed set',
        },
    ],
    monitoredSurfaces: [
        {
            id: 'motion-dashboard-workflow',
            status: 'present',
            sourceFile: 'examples/template/launch-shell.tsx',
            materializedFile: 'components/launch/launch-shell.tsx',
            receiptPath:
                'examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json',
            nextAction: 'Run the Motion source guards after editing the dashboard workflow surface.',
        },
        {
            id: 'motion-interaction-proof',
            status: 'present',
            sourceFile: 'examples/template/motion-interaction-proof.tsx',
            materializedFile: 'components/launch/motion-interaction-proof.tsx',
            receiptPath:
                'examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json',
            nextAction: 'Keep Zed/DX Studio selectors aligned with the visible interaction proof.',
        },
    ],
} as const satisfies MotionDashboardDxCheckVisibility;

export const motionDashboardPackage = {
    packageId: 'animation/motion',
    officialPackageName: motionDashboardOfficialPackageName,
    cliCommand: motionDashboardCliCommand,
    aliases: ['motion', 'framer-motion', 'motion/react', 'animation/motion'],
    upstreamPackage: 'motion',
    upstreamVersion: '12.38.0',
    sourceMirror: 'G:/WWW/inspirations/motion',
    inspectedSourceFiles: motionDashboardInspectedSourceFiles,
    selectedSurfaces: motionDashboardSelectedSurfaces,
    dxCheckVisibility: motionDashboardDxCheckVisibility,
    provenance: {
        upstreamPackage: 'motion',
        upstreamVersion: '12.38.0',
        sourcePackages: [
            'packages/motion',
            'packages/framer-motion',
            'packages/motion-dom',
            'packages/motion-utils',
        ],
        publicApi: [
            'motion',
            'm',
            'MotionConfig',
            'LazyMotion',
            'AnimatePresence',
            'LayoutGroup',
            'Reorder',
            'domAnimation',
            'domMax',
            'domMin',
            'useAnimationControls',
            'useAnimation',
            'animationControls',
            'useAnimationFrame',
            'useTime',
            'useCycle',
            'useWillChange',
            'WillChangeMotionValue',
            'usePageInView',
            'useInstantLayoutTransition',
            'useInView',
            'useMotionValue',
            'useTransform',
            'useMotionTemplate',
            'useMotionValueEvent',
            'useVelocity',
            'useScroll',
            'useSpring',
            'useReducedMotion',
            'useAnimate',
            'usePresence',
            'useIsPresent',
            'useDragControls',
            'AnimationPlaybackControlsWithThen',
            'MotionValue',
            'Transition',
            'Variants',
        ],
    },
    exportedFiles: [
        'js/motion/provider.tsx',
        'js/motion/controls.tsx',
        'js/motion/lazy.tsx',
        'js/motion/layout.tsx',
        'js/motion/motion-values.tsx',
        'js/motion/presence.tsx',
        'js/motion/reorder.tsx',
        'js/motion/scroll-progress.tsx',
        'js/motion/dashboard-workflow.ts',
        'js/motion/metadata.ts',
        'examples/dashboard/src/components/MotionDashboardWorkflow.tsx',
        'examples/dashboard/src/lib/motionDashboardWorkflow.ts',
    ],
    requiredEnv: [],
    appOwnedBoundaries: [
        'global motion policy and reduced-motion review',
        'route transition choreography',
        'production preference sync beyond local storage',
        'performance budget for dashboard animation density',
    ],
    receiptPaths: [
        '.dx/forge/receipts/*-animation-motion.json',
        'examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json',
        '.dx/forge/docs/animation-motion.md',
        'examples/dashboard/README.md#motion-dashboard-workflow',
    ],
} as const;

export const motionDashboardStages: readonly MotionDashboardStage[] = [
    {
        id: 'reveal',
        label: 'Reveal',
        publicApi: 'MotionConfig + MotionReveal',
        packageExport: 'motion/provider.tsx + motion/reveal.tsx',
        progress: 34,
        appBoundary: 'choose launch-page reveal timing and reduced-motion policy',
    },
    {
        id: 'measure',
        label: 'Measure',
        publicApi: 'useMotionValue + useScroll + useSpring',
        packageExport: 'motion/motion-values.tsx + motion/scroll-progress.tsx',
        progress: 67,
        appBoundary: 'decide which dashboard metrics deserve animated feedback',
    },
    {
        id: 'reorder',
        label: 'Reorder',
        publicApi: 'LayoutGroup + Reorder + AnimatePresence',
        packageExport: 'motion/layout.tsx + motion/reorder.tsx + motion/presence.tsx',
        progress: 100,
        appBoundary: 'persist user ordering and route-level choreography policy',
    },
] as const;

function isMotionDashboardStageId(value: string): value is MotionDashboardStageId {
    return motionDashboardStages.some((stage) => stage.id === value);
}

function normalizeMotionDashboardOrder(
    value: unknown,
): readonly MotionDashboardStageId[] | null {
    if (!Array.isArray(value)) return null;

    const stageIds = value.filter(
        (stageId): stageId is MotionDashboardStageId =>
            typeof stageId === 'string' && isMotionDashboardStageId(stageId),
    );
    const uniqueStageIds = [...new Set(stageIds)];

    if (uniqueStageIds.length !== motionDashboardStages.length) return null;
    return uniqueStageIds;
}

export function readMotionDashboardPreference(
    storage: Pick<Storage, 'getItem'> | null | undefined,
): MotionDashboardPreference | null {
    if (!storage) return null;

    try {
        const rawPreference = storage.getItem(motionDashboardPreferenceStorageKey);
        if (!rawPreference) return null;

        const preference = JSON.parse(rawPreference) as Partial<MotionDashboardPreference>;
        const orderedSurfaceIds = normalizeMotionDashboardOrder(
            preference.orderedSurfaceIds,
        );
        const reducedMotionPolicy =
            preference.reducedMotionPolicy === 'preview' ? 'preview' : 'system';

        if (!orderedSurfaceIds) return null;

        return {
            orderedSurfaceIds,
            reducedMotionPolicy,
        };
    } catch {
        return null;
    }
}

export function writeMotionDashboardPreference(
    storage: Pick<Storage, 'setItem'> | null | undefined,
    preference: MotionDashboardPreference,
) {
    if (!storage) return;

    try {
        storage.setItem(
            motionDashboardPreferenceStorageKey,
            JSON.stringify({
                orderedSurfaceIds: [...preference.orderedSurfaceIds],
                reducedMotionPolicy: preference.reducedMotionPolicy,
            }),
        );
    } catch {
        // Storage can be unavailable in embedded previews; the visible state remains source-owned.
    }
}

export function getMotionDashboardStage(stageId: MotionDashboardStageId) {
    return (
        motionDashboardStages.find((stage) => stage.id === stageId) ||
        motionDashboardStages[0]
    );
}

export function moveMotionDashboardStage(
    orderedStageIds: readonly MotionDashboardStageId[],
    stageId: MotionDashboardStageId,
    direction: MotionDashboardOrderDirection,
) {
    return placeMotionDashboardStage(orderedStageIds, stageId, direction);
}

export function placeMotionDashboardStage(
    orderedStageIds: readonly MotionDashboardStageId[],
    stageId: MotionDashboardStageId,
    target: MotionDashboardOrderTarget,
) {
    const currentIndex = orderedStageIds.indexOf(stageId);
    if (currentIndex < 0) return [...orderedStageIds];

    const nextIndex = {
        first: 0,
        last: orderedStageIds.length - 1,
        previous: Math.max(0, currentIndex - 1),
        next: Math.min(orderedStageIds.length - 1, currentIndex + 1),
    }[target];

    if (nextIndex === currentIndex) return [...orderedStageIds];

    const nextOrder = [...orderedStageIds];
    const [stage] = nextOrder.splice(currentIndex, 1);
    nextOrder.splice(nextIndex, 0, stage);
    return nextOrder;
}

export function createMotionDashboardReceipt(input: {
    stageId: MotionDashboardStageId;
    orderedSurfaceIds: readonly MotionDashboardStageId[];
    reducedMotionPolicy?: MotionDashboardReducedMotionPolicy;
}): MotionDashboardReceipt {
    const stage = getMotionDashboardStage(input.stageId);
    const reducedMotionPolicy = input.reducedMotionPolicy ?? 'system';

    return {
        receiptId: `dx-motion-dashboard-${stage.id}-${stage.progress}-${reducedMotionPolicy}`,
        status: 'local-preview-ready',
        packageId: motionDashboardPackage.packageId,
        officialPackageName: motionDashboardOfficialPackageName,
        cliCommand: motionDashboardCliCommand,
        stageId: stage.id,
        progress: stage.progress,
        orderedSurfaceIds: [...input.orderedSurfaceIds],
        selectedSurfaceIds: motionDashboardSelectedSurfaces.map((surface) => surface.id),
        reducedMotionPolicy,
        sourceMirror: motionDashboardPackage.sourceMirror,
        dxCheckVisibility: motionDashboardDxCheckVisibility,
        nextAction:
            reducedMotionPolicy === 'preview'
                ? 'Keep reduced-motion preview enabled while app-owned route transitions are reviewed.'
                : 'Wire this receipt to the app-owned route transition and dashboard preference policy.',
    };
}
