import { z } from 'zod';

export const dxDashboardThemeSchema = z.enum(['system', 'light', 'dark']);
export const dxDashboardLocaleSchema = z.enum(['en', 'bn', 'hi']);
export const dxDashboardPreviewModeSchema = z.enum(['stable', 'preview']);

export const dxDashboardSettingsFormSchema = z.strictObject({
    workspaceName: z.string().trim().min(3).max(80),
    contactEmail: z.email().trim().toLowerCase(),
    defaultLocale: dxDashboardLocaleSchema.default('en'),
    theme: dxDashboardThemeSchema.default('system'),
    previewMode: dxDashboardPreviewModeSchema.default('preview'),
    packageReceiptsRequired: z.boolean().default(true),
    launchScoreTarget: z.coerce.number().int().min(70).max(100).default(90),
});

export const dxDashboardSettingsSchema = dxDashboardSettingsFormSchema
    .safeExtend({
        updatedAt: z.iso.datetime().optional(),
    })
    .readonly()
    .meta({
        title: 'DX dashboard settings',
        description:
            'Source-owned Zod schema for validating the starter dashboard settings workflow.',
        sourceMirror: 'G:/WWW/inspirations/zod',
        packageId: 'validation/zod',
    });

export type DxDashboardTheme = z.infer<typeof dxDashboardThemeSchema>;
export type DxDashboardLocale = z.infer<typeof dxDashboardLocaleSchema>;
export type DxDashboardPreviewMode = z.infer<typeof dxDashboardPreviewModeSchema>;
export type DxDashboardSettingsFormInput = z.input<typeof dxDashboardSettingsFormSchema>;
export type DxDashboardSettings = z.infer<typeof dxDashboardSettingsSchema>;
export type DxDashboardSettingsResult = ReturnType<typeof safeParseDxDashboardSettingsForm>;

export interface DxDashboardSettingsIssue {
    code: string;
    path: string;
    message: string;
}

export interface DxDashboardSettingsReceipt {
    receiptId: string;
    packageId: 'validation/zod';
    status: 'valid-settings' | 'invalid-settings';
    sourceMirror: string;
    fieldErrors: Record<string, string[]>;
    settings?: DxDashboardSettings;
}

const sourceMirror = 'G:/WWW/inspirations/zod';

export const dxDashboardSettingsExample: DxDashboardSettingsFormInput = {
    workspaceName: 'DX Launch Dashboard',
    contactEmail: 'operator@example.com',
    defaultLocale: 'en',
    theme: 'system',
    previewMode: 'preview',
    packageReceiptsRequired: true,
    launchScoreTarget: 94,
};

export const dxDashboardSettingsInvalidExample: DxDashboardSettingsFormInput = {
    workspaceName: 'DX',
    contactEmail: 'not-an-email',
    defaultLocale: 'en',
    theme: 'system',
    previewMode: 'preview',
    packageReceiptsRequired: true,
    launchScoreTarget: 42,
};

export function safeParseDxDashboardSettingsForm(input: DxDashboardSettingsFormInput) {
    return dxDashboardSettingsSchema.safeParse(input);
}

export function parseDxDashboardSettings(input: DxDashboardSettingsFormInput) {
    return dxDashboardSettingsSchema.parse(input);
}

export function formatDxDashboardSettingsIssues(
    input: DxDashboardSettingsFormInput,
): DxDashboardSettingsIssue[] {
    const result = safeParseDxDashboardSettingsForm(input);
    return result.success
        ? []
        : result.error.issues.map(issue => ({
            code: issue.code,
            path: issue.path.join('.') || 'settings',
            message: issue.message,
        }));
}

export function createDxDashboardSettingsReceipt(
    input: DxDashboardSettingsFormInput,
): DxDashboardSettingsReceipt {
    const result = safeParseDxDashboardSettingsForm(input);
    if (!result.success) {
        return {
            receiptId: 'zod-dashboard-settings-invalid',
            packageId: 'validation/zod',
            status: 'invalid-settings',
            sourceMirror,
            fieldErrors: z.flattenError(result.error).fieldErrors,
        };
    }

    return {
        receiptId: `zod-dashboard-settings-${slugify(result.data.workspaceName)}`,
        packageId: 'validation/zod',
        status: 'valid-settings',
        sourceMirror,
        fieldErrors: {},
        settings: {
            ...result.data,
            updatedAt: result.data.updatedAt || new Date().toISOString(),
        },
    };
}

function slugify(value: string) {
    return value.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '') || 'settings';
}
