export type BetterAuthDashboardActionId =
    | 'read-session'
    | 'update-profile'
    | 'link-provider'
    | 'revoke-other-sessions';

export type BetterAuthDashboardProvider = 'google';

export type BetterAuthDashboardUser = {
    id: string;
    email: string;
    name: string;
    role: string;
};

export type BetterAuthDashboardProfileDraft = {
    name: string;
    email: string;
    provider: BetterAuthDashboardProvider;
};

export type BetterAuthDashboardSessionSnapshot = {
    status: 'signed-in' | 'signed-out';
    userLabel: string;
    role: string;
    publicApi: readonly string[];
};

export type BetterAuthDashboardProfileRequest = {
    action: 'update-profile';
    method: 'client';
    publicApi: readonly string[];
    payload: {
        name: string;
        email: string;
    };
};

export type BetterAuthDashboardActionReceipt = {
    receiptId: string;
    status: 'ready' | 'missing-config';
    message: string;
    action: BetterAuthDashboardActionId;
    requiredEnv: readonly string[];
    publicApi: readonly string[];
    appOwnedBoundary: string;
};

export const betterAuthDashboardPackage = {
    packageId: 'auth/better-auth',
    officialName: 'Authentication',
    aliases: ['better-auth', 'auth/betterauth', 'auth/better-auth-next'],
    upstreamPackage: 'better-auth',
    sourceMirror: 'G:/WWW/inspirations/better-auth',
    docsPath: 'docs/packages/authentication.md',
    provenance: {
        repository: 'https://github.com/better-auth/better-auth',
        sourceSubpath: 'packages/better-auth',
        version: '1.6.11',
    },
    exportedFiles: [
        'auth/better-auth/client.ts',
        'auth/better-auth/profile.ts',
        'auth/better-auth/social.ts',
        'auth/better-auth/accounts.ts',
        'auth/better-auth/session.ts',
        'auth/better-auth/session-management.ts',
        'auth/better-auth/dashboard.ts',
        'auth/better-auth/metadata.ts',
    ],
    requiredEnv: [
        'BETTER_AUTH_SECRET',
        'BETTER_AUTH_URL',
        'BETTER_AUTH_TRUSTED_ORIGINS',
        'NEXT_PUBLIC_BETTER_AUTH_URL',
    ],
    appOwnedBoundaries: [
        'database adapter and migrations',
        'session lifetime, cookie, and trusted-origin policy',
        'OAuth provider credentials and callback URLs',
        'profile-field authorization and email verification policy',
        'account linking, token storage, and session revocation UX',
    ],
    receiptPaths: [
        '.dx/forge/receipts/auth-better-auth.json',
        '.dx/forge/docs/launch-companions/auth-session-status.md',
        '.dx/forge/template-readiness/launch-readiness-bundle.json',
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
        receiptPath: 'examples/template/.dx/forge/receipts/auth-better-auth.json',
        monitoredSurfaces: [
            'authentication-account-workflow',
            'authentication-session-status',
        ],
    },
} as const;

export const dxBetterAuthDashboardActions = [
    {
        id: 'read-session',
        label: 'Read session',
        publicApi: ['useSession()', 'auth.api.getSession({ headers })'],
        appOwnedBoundary: 'server request headers and cookie refresh policy',
    },
    {
        id: 'update-profile',
        label: 'Update profile',
        publicApi: ['authClient.updateUser()', 'authClient.changeEmail()'],
        appOwnedBoundary: 'profile-field authorization and email verification policy',
    },
    {
        id: 'link-provider',
        label: 'Link provider',
        publicApi: [
            'authClient.signIn.social()',
            'authClient.listAccounts()',
            'authClient.linkSocial()',
        ],
        appOwnedBoundary: 'OAuth credentials, callback URLs, scopes, and consent copy',
    },
    {
        id: 'revoke-other-sessions',
        label: 'Revoke other sessions',
        publicApi: ['authClient.listSessions()', 'authClient.revokeOtherSessions()'],
        appOwnedBoundary: 'session revocation UX and audit logging',
    },
] as const satisfies readonly {
    id: BetterAuthDashboardActionId;
    label: string;
    publicApi: readonly string[];
    appOwnedBoundary: string;
}[];

export function createDxBetterAuthDashboardSessionSnapshot(
    user: BetterAuthDashboardUser | null | undefined,
): BetterAuthDashboardSessionSnapshot {
    if (!user) {
        return {
            status: 'signed-out',
            userLabel: 'No local dashboard session',
            role: 'guest',
            publicApi: ['useSession()', 'auth.api.getSession({ headers })'],
        };
    }

    return {
        status: 'signed-in',
        userLabel: `${user.name} <${user.email}>`,
        role: user.role,
        publicApi: ['useSession()', 'auth.api.getSession({ headers })'],
    };
}

export function createDxBetterAuthDashboardProfileRequest(
    draft: BetterAuthDashboardProfileDraft,
): BetterAuthDashboardProfileRequest {
    const name = readDashboardName(draft.name);
    const email = readDashboardEmail(draft.email);

    return {
        action: 'update-profile',
        method: 'client',
        publicApi: ['authClient.updateUser()', 'authClient.changeEmail()'],
        payload: {
            name,
            email,
        },
    };
}

export function createDxBetterAuthDashboardActionReceipt({
    actionId,
    draft,
}: {
    actionId: BetterAuthDashboardActionId;
    draft: BetterAuthDashboardProfileDraft;
}): BetterAuthDashboardActionReceipt {
    const action = readDashboardAction(actionId);
    const status = action.id === 'read-session' ? 'ready' : 'missing-config';
    const target = action.id === 'link-provider' ? `-${draft.provider}` : '';

    return {
        receiptId: `better-auth-dashboard-${action.id}${target}`,
        status,
        message:
            status === 'ready'
                ? 'Local dashboard session snapshot is ready; server cookies still belong to the app runtime.'
                : `${action.label} is prepared, but Authentication credentials, routes, database policy, and production UX remain app-owned.`,
        action: action.id,
        requiredEnv: betterAuthDashboardPackage.requiredEnv,
        publicApi: action.publicApi,
        appOwnedBoundary: action.appOwnedBoundary,
    };
}

function readDashboardAction(actionId: BetterAuthDashboardActionId) {
    const action = dxBetterAuthDashboardActions.find((item) => item.id === actionId);
    if (!action) {
        throw new Error('Choose a supported Authentication dashboard action.');
    }
    return action;
}

function readDashboardName(value: string) {
    const name = value.trim();
    if (name.length < 2) {
        throw new Error('Enter a profile name with at least 2 characters.');
    }
    return name;
}

function readDashboardEmail(value: string) {
    const email = value.trim().toLowerCase();
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
        throw new Error('Enter a valid Authentication account email.');
    }
    return email;
}
