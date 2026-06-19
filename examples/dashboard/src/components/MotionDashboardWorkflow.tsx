import { useState } from 'dx';
import {
    createMotionDashboardReceipt,
    getMotionDashboardStage,
    moveMotionDashboardStage,
    motionDashboardPackage,
    motionDashboardPreferenceStorageKey,
    motionDashboardStages,
    placeMotionDashboardStage,
    type MotionDashboardOrderDirection,
    type MotionDashboardOrderTarget,
    type MotionDashboardReceipt,
    type MotionDashboardStageId,
} from '../lib/motionDashboardWorkflow';

const initialOrder = motionDashboardStages.map((stage) => stage.id);
type MotionDashboardKeyboardEvent = {
    key: string;
    preventDefault: () => void;
};

export function MotionDashboardWorkflow() {
    const [stageId, setStageId] = useState<MotionDashboardStageId>('reveal');
    const [orderedStageIds, setOrderedStageIds] =
        useState<MotionDashboardStageId[]>(initialOrder);
    const [reducedMotionPolicy, setReducedMotionPolicy] = useState<'system' | 'preview'>(
        'system',
    );
    const [receipt, setReceipt] = useState<MotionDashboardReceipt | null>(null);
    const activeStage = getMotionDashboardStage(stageId);
    const activeOrderIndex = orderedStageIds.indexOf(activeStage.id);
    const canMovePrevious = activeOrderIndex > 0;
    const canMoveNext = activeOrderIndex >= 0 && activeOrderIndex < orderedStageIds.length - 1;

    const reverseOrder = () => {
        setOrderedStageIds([...orderedStageIds].reverse());
        setReceipt(null);
    };

    const moveStage = (direction: MotionDashboardOrderDirection) => {
        setOrderedStageIds((current) =>
            moveMotionDashboardStage(current, stageId, direction),
        );
        setReceipt(null);
    };

    const placeStage = (target: MotionDashboardOrderTarget) => {
        setOrderedStageIds((current) =>
            placeMotionDashboardStage(current, stageId, target),
        );
        setReceipt(null);
    };

    const handleKeyboardReorder = (event: MotionDashboardKeyboardEvent) => {
        const target = {
            ArrowLeft: 'previous',
            ArrowUp: 'previous',
            ArrowRight: 'next',
            ArrowDown: 'next',
            Home: 'first',
            End: 'last',
        }[event.key] as MotionDashboardOrderTarget | undefined;

        if (!target) return;
        event.preventDefault();
        placeStage(target);
    };

    const prepareReceipt = () => {
        setReceipt(
            createMotionDashboardReceipt({
                stageId,
                orderedSurfaceIds: orderedStageIds,
                reducedMotionPolicy,
            }),
        );
    };

    return (
        <section
            class="motion-dashboard-panel"
            data-dx-package="animation/motion"
            data-dx-component="dashboard-motion-workflow"
            data-dx-motion-dashboard-workflow="animated-readiness"
            data-dx-motion-stage={activeStage.id}
            data-dx-motion-progress={String(activeStage.progress)}
            data-dx-motion-order={orderedStageIds.join(',')}
            data-dx-motion-keyboard-reorder="arrow-home-end"
            data-dx-motion-preference-storage="local-storage"
            data-dx-motion-storage-key={motionDashboardPreferenceStorageKey}
            data-dx-motion-policy="app-owned-reduced-motion-preview"
            data-dx-motion-reduced={reducedMotionPolicy}
            data-dx-motion-receipt={receipt ? receipt.receiptId : 'idle'}
            data-dx-forge-command={motionDashboardPackage.cliCommand}
            data-dx-source-mirror={motionDashboardPackage.sourceMirror}
            data-dx-style-surface="theme-token-card"
            data-dx-icon-search="motion:animation"
            data-dx-node-modules="forbidden"
        >
            <header class="panel-header">
                <dx-icon name="pack:motion" aria-label="Motion" />
                <div>
                    <h2>Motion & Animation workflow</h2>
                    <p>
                        Preview source-owned animation readiness with real Motion public API
                        boundaries and app-owned policy receipts.
                    </p>
                </div>
            </header>

            <div class="provider-options" data-dx-motion-interaction="stage-picker">
                {motionDashboardStages.map((stage) => (
                    <button
                        key={stage.id}
                        type="button"
                        class={stage.id === activeStage.id ? 'active' : ''}
                        data-dx-motion-action="select-stage"
                        data-dx-motion-stage-option={stage.id}
                        data-dx-motion-selected={stage.id === activeStage.id ? 'true' : 'false'}
                        onClick={() => {
                            setStageId(stage.id);
                            setReceipt(null);
                        }}
                    >
                        {stage.label}
                    </button>
                ))}
            </div>

            <div class="query-box" data-dx-motion-summary={activeStage.id}>
                <strong data-dx-motion-public-api={activeStage.publicApi}>
                    {activeStage.publicApi}
                </strong>
                <span data-dx-motion-export={activeStage.packageExport}>
                    {activeStage.packageExport}
                </span>
                <span data-dx-motion-policy-status={reducedMotionPolicy}>
                    {reducedMotionPolicy === 'preview'
                        ? 'Reduced-motion preview is active for dashboard review.'
                        : 'Motion follows the system preference until the app sets policy.'}
                </span>
            </div>

            <div class="motion-progress-track" data-dx-motion-progress-track="dashboard">
                <span
                    class="motion-progress-bar"
                    data-dx-motion-progress-bar="dashboard"
                    style={{ width: `${activeStage.progress}%` }}
                />
            </div>

            <ol
                class="readiness-list"
                aria-activedescendant={`dashboard-motion-stage-${activeStage.id}`}
                aria-label="Motion dashboard stage order"
                data-dx-motion-interaction="surface-order"
                data-dx-motion-keyboard-reorder="arrow-home-end"
                data-dx-motion-keyboard-state={activeStage.id}
                onKeyDown={handleKeyboardReorder}
                role="listbox"
                tabIndex={0}
            >
                {orderedStageIds.map((id) => {
                    const stage = getMotionDashboardStage(id);

                    return (
                        <li
                            key={stage.id}
                            id={`dashboard-motion-stage-${stage.id}`}
                            aria-selected={stage.id === activeStage.id ? 'true' : 'false'}
                            data-dx-motion-stage-item={stage.id}
                            data-dx-motion-stage-active={
                                stage.id === activeStage.id ? 'true' : 'false'
                            }
                            role="option"
                        >
                            <span>{stage.label}</span>
                            <small>{stage.appBoundary}</small>
                        </li>
                    );
                })}
            </ol>

            <div class="provider-options">
                <button
                    type="button"
                    data-dx-motion-action="reverse-order"
                    onClick={reverseOrder}
                >
                    <dx-icon name="pack:reorder" aria-hidden="true" />
                    Reverse order
                </button>
                <button
                    type="button"
                    disabled={!canMovePrevious}
                    data-dx-motion-action="move-stage-previous"
                    data-dx-motion-order-direction="previous"
                    data-dx-motion-order-available={canMovePrevious ? 'true' : 'false'}
                    onClick={() => moveStage('previous')}
                >
                    <dx-icon name="pack:reorder" aria-hidden="true" />
                    Move earlier
                </button>
                <button
                    type="button"
                    disabled={!canMoveNext}
                    data-dx-motion-action="move-stage-next"
                    data-dx-motion-order-direction="next"
                    data-dx-motion-order-available={canMoveNext ? 'true' : 'false'}
                    onClick={() => moveStage('next')}
                >
                    <dx-icon name="pack:reorder" aria-hidden="true" />
                    Move later
                </button>
                <button
                    type="button"
                    aria-pressed={reducedMotionPolicy === 'preview' ? 'true' : 'false'}
                    data-dx-motion-action="toggle-reduced-motion"
                    data-dx-motion-reduced={reducedMotionPolicy}
                    onClick={() => {
                        setReducedMotionPolicy((current) =>
                            current === 'preview' ? 'system' : 'preview',
                        );
                        setReceipt(null);
                    }}
                >
                    <dx-icon name="pack:motion" aria-hidden="true" />
                    {reducedMotionPolicy === 'preview'
                        ? 'Use system motion'
                        : 'Preview reduced motion'}
                </button>
                <button
                    type="button"
                    class="primary-action"
                    data-dx-motion-action="prepare-motion-receipt"
                    onClick={prepareReceipt}
                >
                    <dx-icon name="pack:receipt" aria-hidden="true" />
                    Prepare motion receipt
                </button>
            </div>

            <p
                class="assistant-receipt"
                data-dx-motion-receipt-state={receipt ? receipt.status : 'idle'}
                data-dx-motion-required-env={motionDashboardPackage.requiredEnv.join(',')}
            >
                {receipt
                    ? `${receipt.receiptId}: ${activeStage.label} is ready for app-owned choreography. ${receipt.nextAction}`
                    : `Source mirror: ${motionDashboardPackage.sourceMirror}. No runtime animation receipt has been prepared yet.`}
            </p>
        </section>
    );
}
