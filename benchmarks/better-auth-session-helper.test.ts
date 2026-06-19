const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const registry = fs.readFileSync(
  path.join(root, "core/src/ecosystem/forge_registry.rs"),
  "utf8",
);

test("auth/better-auth exposes a real server-side session helper", () => {
  assert.match(registry, /"js\/auth\/better-auth\/session\.ts"/);
  assert.match(registry, /import \{ headers \} from "next\/headers";/);
  assert.match(registry, /auth\.api\.getSession\(\{\s*headers: await headers\(\),\s*\}\)/);
  assert.match(registry, /export async function getDxBetterAuthSession/);
  assert.match(registry, /export async function requireDxBetterAuthSession/);
  assert.match(registry, /"auth\/better-auth\/session\.ts"/);
  assert.match(registry, /serverSessionHelper: "getDxBetterAuthSession\(auth\)"/);
});

test("auth/better-auth exposes real server-side session management helpers", () => {
  assert.match(registry, /"js\/auth\/better-auth\/session-management\.ts"/);
  assert.match(registry, /auth\.api\.listSessions\(\{\s*headers: await headers\(\),\s*\}\)/);
  assert.match(registry, /auth\.api\.revokeSession\(\{\s*headers: await headers\(\),\s*body: \{\s*token:/);
  assert.match(registry, /auth\.api\.revokeOtherSessions\(\{\s*headers: await headers\(\),\s*\}\)/);
  assert.match(registry, /auth\.api\.revokeSessions\(\{\s*headers: await headers\(\),\s*\}\)/);
  assert.match(registry, /export async function listDxBetterAuthSessions/);
  assert.match(registry, /export async function revokeDxBetterAuthSession/);
  assert.match(registry, /export async function revokeOtherDxBetterAuthSessions/);
  assert.match(registry, /export async function revokeAllDxBetterAuthSessions/);
  assert.match(registry, /sessionManagementHelper: "listDxBetterAuthSessions\(auth\)"/);
});

test("auth/better-auth metadata exposes Forge discovery and dashboard receipts", () => {
  const launchStatus = fs.readFileSync(
    path.join(root, "examples/template/auth-session-status.tsx"),
    "utf8",
  );

  assert.match(registry, /aliases: \[\s*"authentication",\s*"better-auth",\s*"auth\/betterauth",\s*"auth\/better-auth-next",\s*\]/);
  assert.match(registry, /sourceMirror: "G:\\\\WWW\\\\inspirations\\\\better-auth"/);
  assert.match(registry, /repository: "https:\/\/github\.com\/better-auth\/better-auth"/);
  assert.match(registry, /sourceSubpath: "packages\/better-auth"/);
  assert.match(registry, /requiredEnv: \[\s*"BETTER_AUTH_SECRET",\s*"BETTER_AUTH_URL",\s*"BETTER_AUTH_TRUSTED_ORIGINS",\s*"NEXT_PUBLIC_BETTER_AUTH_URL"/);
  assert.match(registry, /appOwnedBoundaries: \[/);
  assert.match(registry, /"Database adapter and migration policy"/);
  assert.match(registry, /receiptPaths: \{/);
  assert.match(registry, /package: "\.dx\/forge\/receipts\/auth-better-auth\.json"/);
  assert.match(registry, /dashboard: "\.dx\/forge\/docs\/launch-companions\/auth-session-status\.md"/);
  assert.match(registry, /exportedFiles: \[/);
  assert.match(registry, /"auth\/better-auth\/README\.md"/);
  assert.match(launchStatus, /import \{ dxBetterAuthForgePackage \} from "@\/auth\/better-auth\/metadata";/);
  assert.match(launchStatus, /data-dx-auth-package-id=\{dxBetterAuthForgePackage\.packageId\}/);
  assert.match(launchStatus, /data-dx-auth-receipt-path=\{dxBetterAuthForgePackage\.receiptPaths\.package\}/);
  assert.match(launchStatus, /data-dx-auth-source-mirror=\{dxBetterAuthForgePackage\.sourceMirror\}/);
});

test("auth/better-auth client exports real session management actions for launch templates", () => {
  const launchStatus = fs.readFileSync(
    path.join(root, "examples/template/auth-session-status.tsx"),
    "utf8",
  );

  assert.match(registry, /export const \{\s*signIn,\s*signUp,\s*signOut,\s*useSession,\s*getSession,\s*listSessions,\s*revokeSession,\s*revokeOtherSessions,\s*revokeSessions,\s*requestPasswordReset,\s*resetPassword,\s*changePassword,\s*sendVerificationEmail,\s*listAccounts,\s*linkSocial,\s*unlinkAccount,\s*getAccessToken,\s*updateUser,\s*changeEmail,\s*deleteUser,\s*\} = authClient;/);
  assert.match(registry, /"authClient\.listSessions\(\) and authClient\.revokeSession\(\) client actions"/);
  assert.match(registry, /clientSessionActions: "listSessions\(\)"/);
  assert.match(launchStatus, /import \{\s*listSessions,\s*revokeOtherSessions,\s*signOut,\s*useSession,\s*\}/);
  assert.match(launchStatus, /await listSessions\(\)/);
  assert.match(launchStatus, /await revokeOtherSessions\(\)/);
  assert.match(launchStatus, /data-auth-session-management="ready"/);
});

test("auth/better-auth launch workflow is visible and locally interactive", () => {
  const launchStatus = fs.readFileSync(
    path.join(root, "examples/template/auth-session-status.tsx"),
    "utf8",
  );
  const route = fs.readFileSync(
    path.join(root, "examples/template/app/page.tsx"),
    "utf8",
  );
  const shell = fs.readFileSync(
    path.join(root, "examples/template/template-shell.tsx"),
    "utf8",
  );
  const cli = fs.readFileSync(
    path.join(root, "dx-www/src/cli/mod.rs"),
    "utf8",
  );

  assert.match(shell, /<LaunchAuthSessionStatus \/>/);
  assert.match(shell, /data-dx-package="auth\/better-auth"/);
  assert.match(launchStatus, /data-dx-package="auth\/better-auth"/);
  assert.match(
    launchStatus,
    /data-dx-component="better-auth-session-status-panel"/,
  );
  assert.match(
    launchStatus,
    /data-dx-parent-component="better-auth-account-dashboard-workflow"/,
  );
  assert.match(launchStatus, /data-dx-component="launch-auth-session-proof"/);
  assert.match(
    launchStatus,
    /data-dx-auth-dashboard-workflow="login-session-account"/,
  );
  assert.match(launchStatus, /data-dx-auth-proof="session-readiness"/);
  assert.match(launchStatus, /data-dx-auth-readiness-card=\{status\.kind\}/);
  assert.match(launchStatus, /data-dx-auth-config=\{authBoundary\.kind\}/);
  assert.match(
    launchStatus,
    /const isAuthConfigured = authBoundary\.kind === "configured";/,
  );
  assert.match(
    launchStatus,
    /data-dx-auth-network-state=\{isAuthConfigured \? "configured" : "missing-config"\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-auth-network-disabled=\{!isAuthConfigured \? "true" : undefined\}/,
  );
  assert.match(
    launchStatus,
    /if \(!isAuthConfigured\) \{\s*previewSafeEmailAction\(readFormValue\(formData, "email"\)\);/s,
  );
  assert.match(launchStatus, /signUpDxBetterAuthEmail/);
  assert.match(
    launchStatus,
    /type EmailAction = "idle" \| "signing-in" \| "signing-up" \| "requesting-reset";/,
  );
  assert.match(launchStatus, /type LaunchAuthBoundaryReview = \{/);
  assert.match(
    launchStatus,
    /useState<LaunchAuthBoundaryReview \| null>\(null\)/,
  );
  assert.match(launchStatus, /function markBoundaryReviewed\(formData\?: FormData\)/);
  assert.match(launchStatus, /function clearBoundaryReview\(\)/);
  assert.match(
    launchStatus,
    /data-dx-auth-interaction="mark-boundary-reviewed"/,
  );
  assert.match(
    launchStatus,
    /data-dx-auth-session-source=\{session\.data \? "better-auth" : "boundary-review"\}/,
  );
  assert.match(
    launchStatus,
    /data-auth-state=\{session\.data \? "signed-in" : "boundary-reviewed"\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-auth-interaction="email-sign-up"/,
  );
  assert.match(
    launchStatus,
    /if \(!isAuthConfigured\) \{\s*previewSafeSignUpAction\(/s,
  );
  assert.match(
    launchStatus,
    /await signUpDxBetterAuthEmail\(\{/,
  );
  assert.match(launchStatus, /function previewBlockedAccountAction\(action: string\)/);
  assert.match(
    launchStatus,
    /if \(!isAuthConfigured\) \{\s*previewBlockedAccountAction\("list sessions"\);/s,
  );
  assert.match(
    launchStatus,
    /if \(!isAuthConfigured\) \{\s*previewBlockedAccountAction\("list linked accounts"\);/s,
  );
  assert.match(
    launchStatus,
    /if \(!isAuthConfigured\) \{\s*previewBlockedAccountAction\(`link \$\{provider\}`\);/s,
  );
  assert.match(
    launchStatus,
    /if \(!isAuthConfigured\) \{\s*previewBlockedAccountAction\("revoke other sessions"\);/s,
  );
  assert.match(
    launchStatus,
    /data-auth-session-management="ready"\s+data-auth-state=\{session\.data \? "signed-in" : "boundary-reviewed"\}\s+data-dx-auth-session-source=\{session\.data \? "better-auth" : "boundary-review"\}/,
  );
  assert.match(
    launchStatus,
    /data-auth-linked-accounts="ready"\s+data-dx-auth-network-state=\{isAuthConfigured \? "configured" : "missing-config"\}/,
  );
  assert.match(launchStatus, /data-dx-auth-provider=\{provider\.provider\}/);
  assert.match(launchStatus, /data-dx-auth-provider-state=\{providerState\}/);
  assert.match(launchStatus, /data-dx-auth-missing-provider-state=/);
  assert.match(launchStatus, /data-dx-auth-interaction="local-readiness-check"/);
  assert.match(launchStatus, /data-dx-auth-interaction="safe-email-preview"/);
  assert.match(launchStatus, /data-dx-auth-boundary-review-state=\{boundaryReviewState\.kind\}/);
  assert.match(launchStatus, /No public Authentication URL is configured/);
  assert.match(launchStatus, /no password or network request was sent/);
  assert.match(cli, /include_str!\("\.\.\/\.\.\/\.\.\/examples\/template\/auth-session-status\.tsx"\)/);
  assert.match(cli, /"components\/template-app\/auth-session-status\.tsx"/);
  assert.match(route, /import \{ TemplateShell \} from "@\/components\/template-app\/template-shell";/);
  assert.match(route, /<TemplateShell \/>/);
  assert.match(shell, /data-dx-component="better-auth-account-dashboard-workflow"/);
  assert.match(shell, /data-dx-dashboard-workflow="login-session-account"/);
  assert.match(shell, /data-dx-product-surface="account-access"/);
  assert.match(cli, /const NEXT_FAMILIAR_LAUNCH_SHELL_TSX: &str =\s*include_str!/);
  assert.match(cli, /include_str!\("\.\.\/\.\.\/\.\.\/examples\/template\/template-shell\.tsx"\)/);
  assert.match(cli, /"components\/template-app\/template-shell\.tsx"/);
});

test("auth/better-auth exposes email password helpers around real client actions", () => {
  const launchStatus = fs.readFileSync(
    path.join(root, "examples/template/auth-session-status.tsx"),
    "utf8",
  );

  assert.match(registry, /"js\/auth\/better-auth\/email-password\.ts"/);
  assert.match(registry, /import \{ signIn, signUp \} from "\.\/client";/);
  assert.match(registry, /export async function signInDxBetterAuthEmail/);
  assert.match(registry, /return signIn\.email\(\{/);
  assert.match(registry, /export async function signUpDxBetterAuthEmail/);
  assert.match(registry, /return signUp\.email\(\{/);
  assert.match(registry, /"authClient\.signIn\.email\(\) and authClient\.signUp\.email\(\) helpers"/);
  assert.match(registry, /emailPasswordHelper: "signInDxBetterAuthEmail\(input\)"/);
  assert.match(launchStatus, /signInDxBetterAuthEmail/);
  assert.match(launchStatus, /signUpDxBetterAuthEmail/);
  assert.match(launchStatus, /data-dx-auth-interaction="email-sign-up"/);
  assert.match(launchStatus, /data-auth-email-password="ready"/);
});

test("auth/better-auth exposes social sign-in helpers around real client actions", () => {
  const launchStatus = fs.readFileSync(
    path.join(root, "examples/template/auth-session-status.tsx"),
    "utf8",
  );

  assert.match(registry, /"js\/auth\/better-auth\/social\.ts"/);
  assert.match(registry, /import \{ signIn \} from "\.\/client";/);
  assert.match(registry, /export type DxBetterAuthSocialProvider = "google";/);
  assert.match(registry, /export async function signInDxBetterAuthSocial/);
  assert.match(registry, /return signIn\.social\(\{/);
  assert.match(registry, /readDxBetterAuthSocialProvider\(input\.provider\)/);
  assert.match(registry, /callbackURL: input\.callbackURL/);
  assert.match(registry, /errorCallbackURL: input\.errorCallbackURL/);
  assert.match(registry, /newUserCallbackURL: input\.newUserCallbackURL/);
  assert.match(registry, /scopes: input\.scopes/);
  assert.match(registry, /requestSignUp: input\.requestSignUp/);
  assert.match(registry, /DX Authentication social provider must be google/);
  assert.match(registry, /"authClient\.signIn\.social\(\) helper for Google provider launches"/);
  assert.match(registry, /"auth\/better-auth\/social\.ts"/);
  assert.match(registry, /socialSignInHelper: "signInDxBetterAuthSocial\(input\)"/);
  assert.match(launchStatus, /signInDxBetterAuthSocial/);
  assert.match(launchStatus, /data-auth-social-provider="ready"/);
});

test("auth/better-auth exposes account linking helpers around real client actions", () => {
  const launchStatus = fs.readFileSync(
    path.join(root, "examples/template/auth-session-status.tsx"),
    "utf8",
  );

  assert.match(registry, /"js\/auth\/better-auth\/accounts\.ts"/);
  assert.match(
    registry,
    /import \{\s*getAccessToken,\s*linkSocial,\s*listAccounts,\s*unlinkAccount,\s*\} from "\.\/client";/,
  );
  assert.match(registry, /export async function listDxBetterAuthAccounts/);
  assert.match(registry, /return listAccounts\(\)/);
  assert.match(registry, /export async function linkDxBetterAuthSocialAccount/);
  assert.match(registry, /return linkSocial\(\{/);
  assert.match(registry, /export async function unlinkDxBetterAuthSocialAccount/);
  assert.match(registry, /return unlinkAccount\(\{/);
  assert.match(registry, /export async function getDxBetterAuthAccessToken/);
  assert.match(registry, /return getAccessToken\(\{/);
  assert.match(
    registry,
    /"authClient\.listAccounts\(\), linkSocial\(\), unlinkAccount\(\), and getAccessToken\(\) helpers"/,
  );
  assert.match(registry, /"auth\/better-auth\/accounts\.ts"/);
  assert.match(registry, /accountLinkingHelper: "linkDxBetterAuthSocialAccount\(input\)"/);
  assert.match(launchStatus, /listDxBetterAuthAccounts/);
  assert.match(launchStatus, /linkDxBetterAuthSocialAccount/);
  assert.match(launchStatus, /data-auth-linked-accounts="ready"/);
});

test("auth/better-auth exposes profile helpers around real user lifecycle actions", () => {
  const launchStatus = fs.readFileSync(
    path.join(root, "examples/template/auth-session-status.tsx"),
    "utf8",
  );

  assert.match(registry, /"js\/auth\/better-auth\/profile\.ts"/);
  assert.match(registry, /updateUser,/);
  assert.match(registry, /changeEmail,/);
  assert.match(registry, /import \{ changeEmail, updateUser \} from "\.\/client";/);
  assert.match(registry, /export async function updateDxBetterAuthUserProfile/);
  assert.match(registry, /return updateUser\(\{/);
  assert.match(registry, /export async function changeDxBetterAuthEmail/);
  assert.match(registry, /return changeEmail\(\{/);
  assert.match(
    registry,
    /"authClient\.updateUser\(\) and authClient\.changeEmail\(\) helpers"/,
  );
  assert.match(registry, /"auth\/better-auth\/profile\.ts"/);
  assert.match(registry, /profileHelper: "updateDxBetterAuthUserProfile\(input\)"/);
  assert.match(launchStatus, /updateDxBetterAuthUserProfile/);
  assert.match(launchStatus, /changeDxBetterAuthEmail/);
  assert.match(launchStatus, /data-auth-profile="ready"/);
});

test("auth/better-auth exposes guarded account deletion around the real deleteUser action", () => {
  const launchStatus = fs.readFileSync(
    path.join(root, "examples/template/auth-session-status.tsx"),
    "utf8",
  );

  assert.match(registry, /"js\/auth\/better-auth\/account-deletion\.ts"/);
  assert.match(registry, /import \{ deleteUser \} from "\.\/client";/);
  assert.match(registry, /DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION/);
  assert.match(registry, /export async function deleteDxBetterAuthAccount/);
  assert.match(registry, /return deleteUser\(\{/);
  assert.match(registry, /password: safePassword/);
  assert.match(registry, /token: safeToken/);
  assert.match(registry, /"authClient\.deleteUser\(\) guarded account deletion helper"/);
  assert.match(registry, /"auth\/better-auth\/account-deletion\.ts"/);
  assert.match(registry, /accountDeletionHelper: "deleteDxBetterAuthAccount\(input\)"/);
  assert.match(launchStatus, /deleteDxBetterAuthAccount/);
  assert.match(launchStatus, /DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION/);
  assert.match(launchStatus, /data-auth-account-deletion="ready"/);
});

test("auth/better-auth exposes account security helpers around real client actions", () => {
  const launchStatus = fs.readFileSync(
    path.join(root, "examples/template/auth-session-status.tsx"),
    "utf8",
  );

  assert.match(registry, /"js\/auth\/better-auth\/account-security\.ts"/);
  assert.match(
    registry,
    /import \{\s*changePassword,\s*requestPasswordReset,\s*resetPassword,\s*sendVerificationEmail,\s*\} from "\.\/client";/,
  );
  assert.match(registry, /export async function requestDxBetterAuthPasswordReset/);
  assert.match(registry, /return requestPasswordReset\(\{/);
  assert.match(registry, /export async function resetDxBetterAuthPassword/);
  assert.match(registry, /return resetPassword\(\{/);
  assert.match(registry, /export async function changeDxBetterAuthPassword/);
  assert.match(registry, /return changePassword\(\{/);
  assert.match(registry, /export async function sendDxBetterAuthVerificationEmail/);
  assert.match(registry, /return sendVerificationEmail\(\{/);
  assert.match(
    registry,
    /accountSecurityHelper: "requestDxBetterAuthPasswordReset\(input\)"/,
  );
  assert.match(launchStatus, /requestDxBetterAuthPasswordReset/);
  assert.match(launchStatus, /data-auth-password-reset="ready"/);
});
