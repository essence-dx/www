import { useState } from 'dx';
import {
    validateZodDashboardSettingsDraft,
    zodDashboardDraftInvalid,
    zodDashboardDraftValid,
    zodDashboardPackageMetadata as metadata,
    type DxDashboardSettingsFormInput,
    type DxDashboardSettingsIssue,
    type DxDashboardSettingsReceipt,
    type DxDashboardSettings,
} from '../lib/zodDashboardSettings';

type FieldName = keyof DxDashboardSettingsFormInput;

function cloneDraft(input: DxDashboardSettingsFormInput): DxDashboardSettingsFormInput {
    return { ...input };
}

export function ZodSettingsValidator() {
    const [draft, setDraft] = useState<DxDashboardSettingsFormInput>(
        cloneDraft(zodDashboardDraftInvalid),
    );
    const [issues, setIssues] = useState<DxDashboardSettingsIssue[]>([]);
    const [fieldErrors, setFieldErrors] = useState<Record<string, string[]>>({});
    const [settings, setSettings] = useState<DxDashboardSettings | null>(null);
    const [receipt, setReceipt] = useState<DxDashboardSettingsReceipt | null>(null);

    const updateDraft = (field: FieldName, value: unknown) => {
        setDraft({ ...draft, [field]: value });
        setIssues([]);
        setFieldErrors({});
        setSettings(null);
        setReceipt(null);
    };

    const loadDraft = (nextDraft: DxDashboardSettingsFormInput) => {
        setDraft(cloneDraft(nextDraft));
        setIssues([]);
        setFieldErrors({});
        setSettings(null);
        setReceipt(null);
    };

    const validateDraft = () => {
        const validation = validateZodDashboardSettingsDraft(draft);
        setIssues(validation.issues);
        setFieldErrors(validation.fieldErrors);
        setSettings(validation.settings);
        setReceipt(validation.receipt);
    };

    function hasFieldError(field: FieldName) {
        return Boolean(fieldErrors[field]?.length);
    }

    function fieldState(field: FieldName) {
        if (!receipt) return 'idle';
        return hasFieldError(field) ? 'invalid' : 'valid';
    }

    function firstFieldError(field: FieldName) {
        return fieldErrors[field]?.[0] ?? '';
    }

    return (
        <section
            class="settings-section"
            data-dx-package="validation/zod"
            data-dx-component="dashboard-zod-settings-validator"
            data-dx-dashboard-workflow="settings-validation"
            data-dx-source-mirror={metadata.sourceMirror}
            data-dx-node-modules="forbidden"
            data-dx-style-surface="validation-schemas"
            data-dx-theme-surface="theme-token-card"
            data-dx-token-scope="validation/zod"
            data-dx-zod-dashboard-state={receipt ? receipt.status : 'idle'}
            data-dx-zod-dashboard-receipt={receipt ? receipt.receiptId : 'none'}
        >
            <div class="section-title">
                <dx-icon name="pack:validation-zod" aria-label="Validation" />
                <h2>Zod settings validation</h2>
            </div>
            <p>
                Validate dashboard settings with the source-owned Zod v4 safeParse,
                strictObject, and flattened issue contracts before an app-owned save action.
            </p>

            <dl class="readiness-list" data-dx-zod-dashboard-readiness="source-owned">
                <div>
                    <dt>Package</dt>
                    <dd data-dx-zod-dashboard-package-id={metadata.packageId}>
                        {metadata.packageId}
                    </dd>
                </div>
                <div>
                    <dt>Source mirror</dt>
                    <dd data-dx-zod-dashboard-source-mirror={metadata.sourceMirror}>
                        {metadata.sourceMirror}
                    </dd>
                </div>
            </dl>

            <div class="form-grid" data-dx-zod-dashboard-form="settings">
                <label data-dx-zod-dashboard-field="workspaceName">
                    Workspace name
                    <input
                        aria-invalid={hasFieldError('workspaceName')}
                        data-dx-zod-dashboard-field-state={fieldState('workspaceName')}
                        data-dx-zod-dashboard-field-error={firstFieldError('workspaceName')}
                        value={String(draft.workspaceName ?? '')}
                        onChange={(event) =>
                            updateDraft(
                                'workspaceName',
                                (event.target as HTMLInputElement).value,
                            )
                        }
                    />
                </label>
                <label data-dx-zod-dashboard-field="contactEmail">
                    Contact email
                    <input
                        aria-invalid={hasFieldError('contactEmail')}
                        data-dx-zod-dashboard-field-state={fieldState('contactEmail')}
                        data-dx-zod-dashboard-field-error={firstFieldError('contactEmail')}
                        value={String(draft.contactEmail ?? '')}
                        autocomplete="email"
                        onChange={(event) =>
                            updateDraft(
                                'contactEmail',
                                (event.target as HTMLInputElement).value,
                            )
                        }
                    />
                </label>
                <label data-dx-zod-dashboard-field="defaultLocale">
                    Locale
                    <select
                        aria-invalid={hasFieldError('defaultLocale')}
                        data-dx-zod-dashboard-field-state={fieldState('defaultLocale')}
                        data-dx-zod-dashboard-field-error={firstFieldError('defaultLocale')}
                        value={String(draft.defaultLocale ?? 'en')}
                        onChange={(event) =>
                            updateDraft(
                                'defaultLocale',
                                (event.target as HTMLSelectElement).value,
                            )
                        }
                    >
                        <option value="en">English</option>
                        <option value="bn">Bangla</option>
                        <option value="hi">Hindi</option>
                    </select>
                </label>
                <label data-dx-zod-dashboard-field="theme">
                    Theme
                    <select
                        aria-invalid={hasFieldError('theme')}
                        data-dx-zod-dashboard-field-state={fieldState('theme')}
                        data-dx-zod-dashboard-field-error={firstFieldError('theme')}
                        value={String(draft.theme ?? 'system')}
                        onChange={(event) =>
                            updateDraft('theme', (event.target as HTMLSelectElement).value)
                        }
                    >
                        <option value="system">System</option>
                        <option value="light">Light</option>
                        <option value="dark">Dark</option>
                    </select>
                </label>
                <label data-dx-zod-dashboard-field="previewMode">
                    Preview mode
                    <select
                        aria-invalid={hasFieldError('previewMode')}
                        data-dx-zod-dashboard-field-state={fieldState('previewMode')}
                        data-dx-zod-dashboard-field-error={firstFieldError('previewMode')}
                        value={String(draft.previewMode ?? 'preview')}
                        onChange={(event) =>
                            updateDraft(
                                'previewMode',
                                (event.target as HTMLSelectElement).value,
                            )
                        }
                    >
                        <option value="stable">Stable</option>
                        <option value="preview">Preview</option>
                    </select>
                </label>
                <label data-dx-zod-dashboard-field="launchScoreTarget">
                    Launch score target
                    <input
                        type="number"
                        min="70"
                        max="100"
                        aria-invalid={hasFieldError('launchScoreTarget')}
                        data-dx-zod-dashboard-field-state={fieldState('launchScoreTarget')}
                        data-dx-zod-dashboard-field-error={firstFieldError('launchScoreTarget')}
                        value={String(draft.launchScoreTarget ?? 90)}
                        onChange={(event) =>
                            updateDraft(
                                'launchScoreTarget',
                                (event.target as HTMLInputElement).value,
                            )
                        }
                    />
                </label>
                <label class="checkbox" data-dx-zod-dashboard-field="packageReceiptsRequired">
                    <input
                        type="checkbox"
                        aria-invalid={hasFieldError('packageReceiptsRequired')}
                        data-dx-zod-dashboard-field-state={fieldState('packageReceiptsRequired')}
                        data-dx-zod-dashboard-field-error={firstFieldError('packageReceiptsRequired')}
                        checked={Boolean(draft.packageReceiptsRequired)}
                        onChange={(event) =>
                            updateDraft(
                                'packageReceiptsRequired',
                                (event.target as HTMLInputElement).checked,
                            )
                        }
                    />
                    Require Forge package receipts
                </label>
            </div>

            <div class="settings-actions">
                <button
                    type="button"
                    data-dx-zod-dashboard-action="load-invalid"
                    onClick={() => loadDraft(zodDashboardDraftInvalid)}
                >
                    Load invalid
                </button>
                <button
                    type="button"
                    data-dx-zod-dashboard-action="load-valid"
                    onClick={() => loadDraft(zodDashboardDraftValid)}
                >
                    Load valid
                </button>
                <button
                    type="button"
                    class="save-btn"
                    data-dx-zod-dashboard-action="validate"
                    onClick={validateDraft}
                >
                    Validate settings
                </button>
            </div>

            <div
                class="query-box"
                data-dx-zod-dashboard-issues={issues.length > 0 ? 'visible' : 'clear'}
                data-dx-zod-dashboard-field-errors={Object.keys(fieldErrors).join(',') || 'none'}
                role={issues.length > 0 ? 'alert' : undefined}
            >
                {issues.length > 0 ? (
                    <ul>
                        {issues.map(issue => (
                            <li key={`${issue.path}:${issue.code}`}>
                                {issue.path}: {issue.message}
                            </li>
                        ))}
                    </ul>
                ) : (
                    <span>Issues appear here after validation.</span>
                )}
            </div>

            <pre
                class="query-box"
                data-dx-zod-dashboard-settings-json={settings ? 'parsed' : 'empty'}
            >
                {settings
                    ? JSON.stringify(settings, null, 2)
                    : JSON.stringify(fieldErrors, null, 2)}
            </pre>

            <p
                class="assistant-receipt"
                data-dx-zod-dashboard-receipt={receipt ? receipt.receiptId : 'none'}
                data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"
                data-dx-zod-dashboard-status={receipt ? receipt.status : 'idle'}
            >
                {receipt
                    ? `${receipt.receiptId}: ${receipt.status}.`
                    : 'Use the validator before saving profile settings.'}
            </p>
        </section>
    );
}
