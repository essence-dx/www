const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const supabaseForgePath = path.join(
  root,
  "core",
  "src",
  "ecosystem",
  "forge_supabase.rs",
);

function readSupabaseForgeSource() {
  return fs.readFileSync(supabaseForgePath, "utf8");
}

function readSupabaseTemplateSource(templatePath) {
  const source = readSupabaseForgeSource();
  const escapedTemplatePath = templatePath.replace(
    /[.*+?^${}()|[\]\\]/g,
    "\\$&",
  );
  const match = source.match(
    new RegExp(`"${escapedTemplatePath}",\\s*r#"[\\s\\S]*?"#,`),
  );

  assert.ok(match, `${templatePath} template should exist`);
  return match[0];
}

test("supabase forge env template rejects invalid public config", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /function assertSupabasePublicUrl/);
  assert.match(source, /function assertSupabasePublishableKey/);
  assert.match(source, /new URL\(value\)/);
  assert.match(source, /https?:/);
  assert.match(source, /service[_-]?role|secret/i);
  assert.match(source, /NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY/);
  assert.doesNotMatch(source, /SUPABASE_SERVICE_ROLE_KEY=/);
});

test("supabase server template uses the same public config validation", () => {
  const source = readSupabaseForgeSource();

  assert.match(
    source,
    /"js\/supabase\/server\.ts"[\s\S]*readSupabasePublicConfig,[\s\S]*type DxSupabaseEnv,/,
  );
  assert.match(
    source,
    /const config = readSupabasePublicConfig\(env\);[\s\S]*createServerClient<Database>\(\s*config\.url,\s*config\.publishableKey,/,
  );
  assert.doesNotMatch(source, /const sourceEnv = env \?\? defaultSupabaseEnv\(\);/);
});

test("supabase auth actions keep redirects app-local", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /function assertSupabaseLocalRedirectPath/);
  assert.match(source, /function readSupabaseAuthActionPaths/);
  assert.match(source, /const paths = readSupabaseAuthActionPaths\(merged\);/);
  assert.match(source, /redirect\(withMessage\(paths\.failurePath, error\.message\)\)/);
  assert.match(source, /revalidatePath\(paths\.revalidate, "layout"\)/);
  assert.match(source, /redirect\(paths\.successPath\)/);
  assert.match(source, /path\.startsWith\("\/\/"\)/);
  assert.match(source, /new URL\(path, "http:\/\/dx\.local"\)/);
  assert.doesNotMatch(source, /redirect\(merged\.successPath\)/);
});

test("supabase auth callback exchanges codes with local next redirects", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/auth-callback\.ts"/);
  assert.match(source, /import \{ NextResponse, type NextRequest \} from "next\/server";/);
  assert.match(source, /export async function handleDxSupabaseAuthCallback/);
  assert.match(source, /const code = requestUrl\.searchParams\.get\("code"\);/);
  assert.match(source, /await supabase\.auth\.exchangeCodeForSession\(code\)/);
  assert.match(source, /requestUrl\.searchParams\.get\(options\.nextParam\)/);
  assert.match(source, /function isSupabaseLocalCallbackPath/);
  assert.match(source, /path\.startsWith\("\/\/"\)/);
  assert.match(source, /NextResponse\.redirect\(new URL\(paths\.successPath, requestUrl\.origin\)\)/);
  assert.match(source, /"lib\/supabase\/auth-callback\.ts"/);
  assert.match(source, /callbackHandler: "handleDxSupabaseAuthCallback\(request\)"/);
  assert.doesNotMatch(source, /x-forwarded-host/);
});

test("supabase oauth helper starts real browser redirects safely", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/auth-oauth\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /import type \{ Provider \} from "@supabase\/supabase-js";/);
  assert.match(source, /export async function signInWithDxSupabaseOAuth/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /supabase\.auth\.signInWithOAuth\(\{/);
  assert.match(source, /provider: options\.provider/);
  assert.match(source, /redirectTo: buildDxSupabaseOAuthRedirectTo\(options\)/);
  assert.match(source, /export function buildDxSupabaseOAuthRedirectTo/);
  assert.match(source, /window\.location\.origin/);
  assert.match(source, /url\.searchParams\.set\(nextParam, nextPath\)/);
  assert.match(source, /function assertSupabaseLocalOAuthPath/);
  assert.match(source, /path\.startsWith\("\/\/"\)/);
  assert.match(source, /"lib\/supabase\/auth-oauth\.ts"/);
  assert.match(source, /oauthSignIn: "signInWithDxSupabaseOAuth\(\{ provider: \\"github\\" \}\)"/);
});

test("supabase profile helpers consume the shipped profiles table", () => {
  const source = readSupabaseForgeSource();
  const profileSource = readSupabaseTemplateSource("js/supabase/profiles.ts");

  assert.match(source, /"js\/supabase\/profiles\.ts"/);
  assert.match(source, /export type DxSupabaseProfilesDatabase/);
  assert.match(source, /Relationships: \[\]/);
  assert.match(source, /Views: Record<string, never>/);
  assert.match(source, /export async function getDxSupabaseCurrentProfile/);
  assert.match(source, /supabase\.auth\.getUser\(\)/);
  assert.match(source, /export async function getDxSupabaseProfile/);
  assert.match(source, /\.from\("profiles"\)/);
  assert.match(source, /\.select\(DX_SUPABASE_PROFILE_SELECT\)/);
  assert.match(source, /\.eq\("id", userId\)/);
  assert.match(source, /status !== 406/);
  assert.match(source, /export async function upsertDxSupabaseProfile/);
  assert.match(source, /\.upsert\(toDxSupabaseProfileUpsert\(userId, input\)\)/);
  assert.match(source, /updated_at: new Date\(\)\.toISOString\(\)/);
  assert.match(source, /fullName: row\.full_name/);
  assert.match(source, /avatarUrl: row\.avatar_url/);
  assert.match(source, /"lib\/supabase\/profiles\.ts"/);
  assert.match(source, /profileHelper: "getDxSupabaseCurrentProfile\(\)"/);
  assert.doesNotMatch(profileSource, /auth\.getSession\(\)/);
});

test("supabase proxy helper refreshes SSR auth cookies", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/proxy\.ts"/);
  assert.match(source, /import \{ createServerClient \} from "@supabase\/ssr";/);
  assert.match(source, /import \{ NextResponse, type NextRequest \} from "next\/server";/);
  assert.match(source, /type DxSupabaseEnv/);
  assert.match(source, /export type DxSupabaseSessionProxyOptions/);
  assert.match(source, /export async function updateDxSupabaseSession/);
  assert.match(source, /let supabaseResponse = NextResponse\.next\(\{\s*request,\s*\}\);/);
  assert.match(source, /const config = readSupabasePublicConfig\(options\.env\);/);
  assert.match(source, /createServerClient\(\s*config\.url,\s*config\.publishableKey,/);
  assert.match(source, /return request\.cookies\.getAll\(\)/);
  assert.match(source, /request\.cookies\.set\(name, value\)/);
  assert.match(source, /supabaseResponse\.cookies\.set\(name, value, options\)/);
  assert.match(source, /supabaseResponse\.headers\.set\(key, value\)/);
  assert.match(source, /await supabase\.auth\.getClaims\(\)/);
  assert.match(source, /dxSupabaseSessionProxyConfig/);
  assert.match(source, /_next\/static\|_next\/image\|favicon\.ico/);
  assert.match(source, /"lib\/supabase\/proxy\.ts"/);
  assert.match(source, /sessionProxy: "updateDxSupabaseSession\(request\)"/);
  assert.doesNotMatch(source, /process\.env\.NEXT_PUBLIC_SUPABASE_URL!/);
  assert.doesNotMatch(source, /process\.env\.NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY!/);
});

test("supabase auth guard follows upstream protected-page getUser pattern", () => {
  const source = readSupabaseForgeSource();
  const guardSource = readSupabaseTemplateSource("js/supabase/auth-guard.ts");

  assert.match(source, /"js\/supabase\/auth-guard\.ts"/);
  assert.match(guardSource, /import \{ redirect \} from "next\/navigation";/);
  assert.match(guardSource, /import type \{ AuthError, SupabaseClient, User \} from "@supabase\/supabase-js";/);
  assert.match(guardSource, /export const DX_SUPABASE_DEFAULT_LOGIN_PATH = "\/login"/);
  assert.match(guardSource, /export type DxSupabaseAuthGuardOptions/);
  assert.match(guardSource, /export type DxSupabaseServerUserResult/);
  assert.match(guardSource, /export type DxSupabaseAuthenticatedUser/);
  assert.match(guardSource, /export async function getDxSupabaseServerUser/);
  assert.match(guardSource, /createDxSupabaseServerClient<Database>\(\)/);
  assert.match(guardSource, /supabase\.auth\.getUser\(\)/);
  assert.match(guardSource, /authenticated: true/);
  assert.match(guardSource, /authenticated: false/);
  assert.match(guardSource, /export async function requireDxSupabaseServerUser/);
  assert.match(guardSource, /redirect\(readDxSupabaseLoginRedirect\(options\)\)/);
  assert.match(guardSource, /export function readDxSupabaseLoginRedirect/);
  assert.match(guardSource, /function assertDxSupabaseLocalLoginPath/);
  assert.match(guardSource, /value\.startsWith\("\/\/"\)/);
  assert.match(guardSource, /value\.includes\(":\/\/"\)/);
  assert.doesNotMatch(guardSource, /auth\.getSession\(\)/);
  assert.match(source, /"lib\/supabase\/auth-guard\.ts"/);
  assert.match(source, /auth\.getUser\(\) protected-page server guard/);
  assert.match(source, /authGuard: "requireDxSupabaseServerUser\(\)"/);
});

test("supabase avatar storage helper owns the user-management storage path", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/avatar-storage\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /export const DX_SUPABASE_AVATAR_BUCKET = "avatars"/);
  assert.match(source, /export type DxSupabaseAvatarUploadOptions/);
  assert.match(source, /export async function uploadDxSupabaseAvatar/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.upload\(path, file,/);
  assert.match(source, /cacheControl: options\.cacheControl \?\? "3600"/);
  assert.match(source, /upsert: options\.upsert \?\? false/);
  assert.match(source, /export async function downloadDxSupabaseAvatarUrl/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.download\(safePath\)/);
  assert.match(source, /URL\.createObjectURL\(data\)/);
  assert.match(source, /export function getDxSupabaseAvatarPublicUrl/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.getPublicUrl\(safePath\)/);
  assert.match(source, /export function revokeDxSupabaseAvatarUrl/);
  assert.match(source, /URL\.revokeObjectURL\(url\)/);
  assert.match(source, /function assertDxSupabaseAvatarPath/);
  assert.match(source, /path\.includes\("\.\."\)/);
  assert.match(source, /insert into storage\.buckets \(id, name\)/);
  assert.match(source, /values \('avatars', 'avatars'\)/);
  assert.match(source, /on conflict \(id\) do nothing/);
  assert.match(source, /on storage\.objects/);
  assert.match(source, /bucket_id = 'avatars'/);
  assert.match(source, /"lib\/supabase\/avatar-storage\.ts"/);
  assert.match(source, /avatarUpload: "uploadDxSupabaseAvatar\(\{ userId, file \}\)"/);
});

test("supabase password recovery helper follows upstream reset and update APIs", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/password-recovery\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /export type DxSupabasePasswordResetOptions/);
  assert.match(source, /export type DxSupabasePasswordUpdateOptions/);
  assert.match(source, /export async function requestDxSupabasePasswordReset/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /supabase\.auth\.resetPasswordForEmail\(email, \{/);
  assert.match(source, /redirectTo: buildDxSupabasePasswordResetRedirectTo\(options\)/);
  assert.match(source, /export async function updateDxSupabasePassword/);
  assert.match(source, /supabase\.auth\.updateUser\(\{ password \}\)/);
  assert.match(source, /export function buildDxSupabasePasswordResetRedirectTo/);
  assert.match(source, /window\.location\.origin/);
  assert.match(source, /function assertSupabaseLocalPasswordRecoveryPath/);
  assert.match(source, /path\.startsWith\("\/\/"\)/);
  assert.match(source, /"lib\/supabase\/password-recovery\.ts"/);
  assert.match(source, /passwordReset: "requestDxSupabasePasswordReset\(\{ email \}\)"/);
  assert.match(source, /passwordUpdate: "updateDxSupabasePassword\(\{ password \}\)"/);
});

test("supabase auth confirm helper verifies token hashes with local redirects", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/auth-confirm\.ts"/);
  assert.match(source, /import type \{ EmailOtpType \} from "@supabase\/supabase-js";/);
  assert.match(source, /import \{ NextResponse, type NextRequest \} from "next\/server";/);
  assert.match(source, /export async function handleDxSupabaseAuthConfirm/);
  assert.match(source, /const tokenHash = requestUrl\.searchParams\.get\("token_hash"\);/);
  assert.match(source, /readDxSupabaseEmailOtpType\(requestUrl\.searchParams\.get\("type"\)\)/);
  assert.match(source, /await supabase\.auth\.verifyOtp\(\{\s*type,\s*token_hash: tokenHash,/);
  assert.match(source, /requestUrl\.searchParams\.get\(options\.nextParam\)/);
  assert.match(source, /function isSupabaseLocalConfirmPath/);
  assert.match(source, /path\.startsWith\("\/\/"\)/);
  assert.match(source, /NextResponse\.redirect\(new URL\(paths\.successPath, requestUrl\.origin\)\)/);
  assert.match(source, /"lib\/supabase\/auth-confirm\.ts"/);
  assert.match(source, /confirmHandler: "handleDxSupabaseAuthConfirm\(request\)"/);
  assert.doesNotMatch(source, /x-forwarded-host/);
});

test("supabase otp helper starts passwordless email sign-ins safely", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/auth-otp\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /export type DxSupabaseOtpSignInOptions/);
  assert.match(source, /export async function signInWithDxSupabaseOtp/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /supabase\.auth\.signInWithOtp\(\{/);
  assert.match(source, /email,/);
  assert.match(source, /emailRedirectTo: buildDxSupabaseOtpRedirectTo\(options\)/);
  assert.match(source, /shouldCreateUser: options\.shouldCreateUser/);
  assert.match(source, /export function buildDxSupabaseOtpRedirectTo/);
  assert.match(source, /window\.location\.origin/);
  assert.match(source, /url\.searchParams\.set\(nextParam, nextPath\)/);
  assert.match(source, /function assertSupabaseLocalOtpPath/);
  assert.match(source, /path\.startsWith\("\/\/"\)/);
  assert.match(source, /"lib\/supabase\/auth-otp\.ts"/);
  assert.match(source, /otpSignIn: "signInWithDxSupabaseOtp\(\{ email \}\)"/);
});

test("supabase mfa helper wraps browser auth mfa APIs safely", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/auth-mfa\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /Factor/);
  assert.match(source, /MFAChallengeAndVerifyParams/);
  assert.match(source, /MFAEnrollParams/);
  assert.match(source, /MFAUnenrollParams/);
  assert.match(source, /export type DxSupabaseMfaFactor = Factor/);
  assert.match(source, /export type DxSupabaseMfaAssuranceLevel = "aal1" \| "aal2"/);
  assert.match(source, /export async function listDxSupabaseMfaFactors/);
  assert.match(source, /\.auth\.mfa\.listFactors\(\)/);
  assert.match(source, /export async function getDxSupabaseMfaAssuranceLevel/);
  assert.match(source, /\.auth\.mfa\.getAuthenticatorAssuranceLevel\(\)/);
  assert.match(source, /export async function enrollDxSupabaseTotpFactor/);
  assert.match(source, /factorType: "totp"/);
  assert.match(source, /export async function enrollDxSupabasePhoneFactor/);
  assert.match(source, /factorType: "phone"/);
  assert.match(source, /phone: assertDxSupabaseMfaPhone\(options\.phone\)/);
  assert.match(source, /export async function challengeDxSupabaseMfa/);
  assert.match(source, /\.auth\.mfa\.challenge\(\{ factorId \}\)/);
  assert.match(source, /export async function verifyDxSupabaseMfa/);
  assert.match(source, /\.auth\.mfa\.verify\(\{/);
  assert.match(source, /challengeId,/);
  assert.match(source, /code,/);
  assert.match(source, /export async function challengeAndVerifyDxSupabaseMfa/);
  assert.match(source, /\.auth\.mfa\.challengeAndVerify\(\{/);
  assert.match(source, /export async function unenrollDxSupabaseMfaFactor/);
  assert.match(source, /\.auth\.mfa\.unenroll\(\{ factorId \}\)/);
  assert.match(source, /function assertDxSupabaseMfaFactorId/);
  assert.match(source, /function assertDxSupabaseMfaCode/);
  assert.match(source, /function assertDxSupabaseMfaPhone/);
  assert.match(source, /"lib\/supabase\/auth-mfa\.ts"/);
  assert.match(source, /auth\.mfa\.enroll, listFactors, challenge, verify, challengeAndVerify, unenroll, and getAuthenticatorAssuranceLevel/);
  assert.match(source, /mfaFactors: "listDxSupabaseMfaFactors\(\)"/);
});

test("supabase identity helper wraps linked auth identities safely", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/auth-identities\.ts"/);
  assert.match(source, /UserIdentity/);
  assert.match(source, /Provider/);
  assert.match(source, /SignInWithOAuthCredentials/);
  assert.match(source, /SignInWithIdTokenCredentials/);
  assert.match(source, /export type DxSupabaseUserIdentity = UserIdentity/);
  assert.match(source, /export async function listDxSupabaseUserIdentities/);
  assert.match(source, /\.auth\.getUserIdentities\(\)/);
  assert.match(source, /return data\.identities \?\? \[\]/);
  assert.match(source, /export async function getDxSupabaseIdentityByProvider/);
  assert.match(source, /identity\.provider === provider/);
  assert.match(source, /export async function linkDxSupabaseOAuthIdentity/);
  assert.match(source, /\.auth\.linkIdentity\(credentials\)/);
  assert.match(source, /redirectTo: buildDxSupabaseIdentityRedirectTo\(options\)/);
  assert.match(source, /export async function linkDxSupabaseIdTokenIdentity/);
  assert.match(source, /token: assertDxSupabaseIdentityToken\(options\.token\)/);
  assert.match(source, /export async function unlinkDxSupabaseIdentity/);
  assert.match(source, /\.auth\.unlinkIdentity\(assertDxSupabaseUserIdentity\(identity\)\)/);
  assert.match(source, /\.auth\.refreshSession\(\)/);
  assert.match(source, /export async function unlinkDxSupabaseIdentityByProvider/);
  assert.match(source, /Supabase identity .* is not linked/);
  assert.match(source, /function assertDxSupabaseIdentityProvider/);
  assert.match(source, /function assertDxSupabaseIdentityToken/);
  assert.match(source, /function assertSupabaseLocalIdentityPath/);
  assert.match(source, /"lib\/supabase\/auth-identities\.ts"/);
  assert.match(source, /auth\.getUserIdentities, auth\.linkIdentity, auth\.unlinkIdentity, and auth\.refreshSession/);
  assert.match(source, /linkedIdentities: "listDxSupabaseUserIdentities\(\)"/);
});

test("supabase auth session helper wraps browser session APIs safely", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/auth-session\.ts"/);
  assert.match(source, /AuthChangeEvent/);
  assert.match(source, /Session/);
  assert.match(source, /User/);
  assert.match(source, /export type DxSupabaseAuthSnapshot/);
  assert.match(source, /export type DxSupabaseAuthChange/);
  assert.match(source, /export type DxSupabaseAuthSubscription/);
  assert.match(source, /export async function getDxSupabaseBrowserSession/);
  assert.match(source, /\.auth\.getSession\(\)/);
  assert.match(source, /return data\.session \?\? null/);
  assert.match(source, /export async function getDxSupabaseTrustedUser/);
  assert.match(source, /\.auth\.getUser\(\)/);
  assert.match(source, /return data\.user \?\? null/);
  assert.match(source, /export async function getDxSupabaseVerifiedClaims/);
  assert.match(source, /\.auth\.getClaims\(\)/);
  assert.match(source, /return data\.claims \?\? null/);
  assert.match(source, /export async function refreshDxSupabaseBrowserSession/);
  assert.match(source, /\.auth\.refreshSession\(\)/);
  assert.match(source, /export async function readDxSupabaseAuthSnapshot/);
  assert.match(source, /readDxSupabaseAccessToken\(session\)/);
  assert.match(source, /export function subscribeToDxSupabaseAuthState/);
  assert.match(source, /\.auth\.onAuthStateChange\(\(event, session\) =>/);
  assert.match(source, /const defer = options\.defer \?\? deferDxSupabaseAuthChange/);
  assert.match(source, /defer\(\(\) => options\.onChange\(\{ event, session \}\)\)/);
  assert.match(source, /setTimeout\(callback, 0\)/);
  assert.match(source, /data\.subscription\.unsubscribe\(\)/);
  assert.match(source, /export function readDxSupabaseAccessToken/);
  assert.match(source, /session\?\.access_token \?\? null/);
  assert.match(source, /"lib\/supabase\/auth-session\.ts"/);
  assert.match(
    source,
    /auth\.getSession, auth\.getUser, auth\.getClaims, auth\.refreshSession, and auth\.onAuthStateChange/,
  );
  assert.match(source, /authSnapshot: "readDxSupabaseAuthSnapshot\(\)"/);
});

test("supabase anonymous auth helper wraps real anonymous sign-in and upgrade APIs", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/auth-anonymous\.ts"/);
  assert.match(source, /export type DxSupabaseAnonymousSignInOptions/);
  assert.match(source, /export type DxSupabaseAnonymousUpgradeOptions/);
  assert.match(source, /export async function signInDxSupabaseAnonymously/);
  assert.match(source, /\.auth\.signInAnonymously\(credentials\)/);
  assert.match(source, /data\.user/);
  assert.match(source, /readDxSupabaseAnonymousUserFlag\(data\.user\)/);
  assert.match(source, /export async function upgradeDxSupabaseAnonymousUser/);
  assert.match(source, /\.auth\.updateUser\(\s*attributes,\s*updateOptions,\s*\)/);
  assert.match(source, /emailRedirectTo: buildDxSupabaseAnonymousRedirectTo/);
  assert.match(source, /export function buildDxSupabaseAnonymousCredentials/);
  assert.match(source, /if \(Object\.keys\(authOptions\)\.length === 0\)/);
  assert.match(source, /return \{\};/);
  assert.match(source, /return \{\s*options: authOptions,/);
  assert.match(source, /function buildDxSupabaseAnonymousUpgradeAttributes/);
  assert.match(source, /At least one anonymous user upgrade field is required/);
  assert.match(source, /function assertSupabaseLocalAnonymousPath/);
  assert.match(source, /path\.startsWith\("\/\/"\)/);
  assert.match(source, /window\.location\.origin/);
  assert.match(source, /"lib\/supabase\/auth-anonymous\.ts"/);
  assert.match(source, /auth\.signInAnonymously and auth\.updateUser for anonymous auth upgrade flows/);
  assert.match(source, /anonymousSignIn: "signInDxSupabaseAnonymously\(\{ data \}\)"/);
});

test("supabase realtime auth helper applies access tokens for private channels", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/realtime-auth\.ts"/);
  assert.match(source, /import type \{ Session, SupabaseClient \} from "@supabase\/supabase-js";/);
  assert.match(source, /getDxSupabaseBrowserSession,/);
  assert.match(source, /readDxSupabaseAccessToken,/);
  assert.match(source, /export type DxSupabaseRealtimeAuthOptions/);
  assert.match(source, /export type DxSupabaseRealtimeAuthResult/);
  assert.match(source, /export type DxSupabasePrivateRealtimeChannelOptions/);
  assert.match(source, /export async function applyDxSupabaseRealtimeAuth/);
  assert.match(source, /const accessToken = await resolveDxSupabaseRealtimeAccessToken\(options\)/);
  assert.match(source, /\.realtime\.setAuth\(accessToken\)/);
  assert.match(source, /authenticated: true/);
  assert.match(source, /Supabase Realtime private channels require an authenticated session/);
  assert.match(source, /export async function resolveDxSupabaseRealtimeAccessToken/);
  assert.match(source, /options\.accessToken/);
  assert.match(source, /const providedSession = options\.session \?\? null/);
  assert.match(source, /readDxSupabaseAccessToken\(providedSession\)/);
  assert.match(source, /await getDxSupabaseBrowserSession\(\)/);
  assert.match(source, /export function buildDxSupabasePrivateRealtimeChannelConfig/);
  assert.match(source, /private: true/);
  assert.match(source, /broadcast: \{/);
  assert.match(source, /self: options\.self \?\? true/);
  assert.match(source, /ack: options\.ack \?\? true/);
  assert.match(source, /if \(options\.presenceKey\)/);
  assert.match(source, /config\.presence = \{/);
  assert.match(source, /function assertDxSupabaseRealtimeAccessToken/);
  assert.match(source, /function assertDxSupabaseRealtimePresenceKey/);
  assert.match(source, /"lib\/supabase\/realtime-auth\.ts"/);
  assert.match(source, /realtime\.setAuth\(accessToken\) for private Realtime channel authorization/);
  assert.match(source, /realtimeAuth: "applyDxSupabaseRealtimeAuth\(\)"/);
});

test("supabase realtime helper subscribes to postgres changes safely", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/realtime-postgres\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(
    source,
    /import type \{\s*RealtimeChannel,\s*RealtimePostgresChangesPayload,\s*\} from "@supabase\/supabase-js";/,
  );
  assert.match(source, /export type DxSupabaseRealtimePostgresEvent/);
  assert.match(source, /export type DxSupabaseRealtimePostgresOptions/);
  assert.match(source, /export type DxSupabaseRealtimePostgresSubscription/);
  assert.match(source, /export function subscribeToDxSupabasePostgresChanges/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /supabase\.channel\(channelName\)/);
  assert.match(source, /\.on\(\s*"postgres_changes",/);
  assert.match(source, /event: options\.event \?\? "\*"/);
  assert.match(source, /schema: options\.schema \?\? "public"/);
  assert.match(source, /table: assertDxSupabaseRealtimeTable\(options\.table\)/);
  assert.match(source, /filter: options\.filter/);
  assert.match(
    source,
    /options\.onChange\(payload as RealtimePostgresChangesPayload<Row>\)/,
  );
  assert.match(source, /\.subscribe\(\(status, error\) =>/);
  assert.match(source, /return supabase\.removeChannel\(channel\)/);
  assert.match(source, /function assertDxSupabaseRealtimeChannel/);
  assert.match(source, /value === "realtime"/);
  assert.match(source, /"lib\/supabase\/realtime-postgres\.ts"/);
  assert.match(
    source,
    /realtimePostgres: "subscribeToDxSupabasePostgresChanges\(\{ table: \\"profiles\\", onChange \}\)"/,
  );
});

test("supabase realtime broadcast helper sends and receives channel messages", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/realtime-broadcast\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(
    source,
    /import type \{ RealtimeChannel \} from "@supabase\/supabase-js";/,
  );
  assert.match(source, /export type DxSupabaseBroadcastPayload/);
  assert.match(source, /export type DxSupabaseRealtimeBroadcastConfig/);
  assert.match(source, /export type DxSupabaseRealtimeBroadcastOptions/);
  assert.match(source, /export type DxSupabaseRealtimeBroadcastSubscription/);
  assert.match(source, /export function subscribeToDxSupabaseBroadcast/);
  assert.match(source, /export async function sendDxSupabaseBroadcast/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /supabase\s*\.channel\(channelName, \{/);
  assert.match(source, /private: options\.config\?\.private \?\? true/);
  assert.match(source, /self: options\.config\?\.self \?\? true/);
  assert.match(source, /ack: options\.config\?\.ack \?\? true/);
  assert.match(source, /\.on\(\s*"broadcast",\s*\{\s*event,?\s*\}/);
  assert.match(source, /\.subscribe\(\(status, error\) =>/);
  assert.match(source, /return channel\.send\(\{/);
  assert.match(source, /type: "broadcast"/);
  assert.match(source, /payload,/);
  assert.match(source, /return supabase\.removeChannel\(channel\)/);
  assert.match(source, /options\.channel\.send\(\{/);
  assert.match(source, /function assertDxSupabaseBroadcastChannel/);
  assert.match(source, /function assertDxSupabaseBroadcastEvent/);
  assert.match(source, /value === "realtime"/);
  assert.match(source, /"lib\/supabase\/realtime-broadcast\.ts"/);
  assert.match(
    source,
    /realtimeBroadcast: "subscribeToDxSupabaseBroadcast\(\{ channel: \\"room:launch:messages\\", event: \\"message_sent\\", onMessage \}\)"/,
  );
});

test("supabase realtime presence helper tracks user state safely", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/realtime-presence\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /import type \{ RealtimeChannel \} from "@supabase\/supabase-js";/);
  assert.match(source, /export type DxSupabasePresencePayload/);
  assert.match(source, /export type DxSupabasePresenceState/);
  assert.match(source, /export type DxSupabaseRealtimePresenceOptions/);
  assert.match(source, /export type DxSupabaseRealtimePresenceSubscription/);
  assert.match(source, /export function subscribeToDxSupabasePresence/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /const channelOptions = options\.key/);
  assert.match(source, /supabase\s*\.channel\(channelName, channelOptions\)/);
  assert.match(
    source,
    /presence: \{\s*key: assertDxSupabasePresenceKey\(options\.key\),?\s*\}/,
  );
  assert.match(source, /\.on\(\s*"presence",\s*\{\s*event: "sync",?\s*\}/);
  assert.match(source, /channel\.presenceState\(\)/);
  assert.match(source, /\.on\(\s*"presence",\s*\{\s*event: "join",?\s*\}/);
  assert.match(source, /newPresences: event\.newPresences as Presence\[\]/);
  assert.match(source, /\.on\(\s*"presence",\s*\{\s*event: "leave",?\s*\}/);
  assert.match(source, /leftPresences: event\.leftPresences as Presence\[\]/);
  assert.match(source, /status === "SUBSCRIBED" && options\.initialPresence/);
  assert.match(source, /void channel\.track\(options\.initialPresence\)/);
  assert.match(source, /presenceState\(\)/);
  assert.match(source, /return channel\.track\(presence\)/);
  assert.match(source, /return channel\.untrack\(\)/);
  assert.match(source, /return supabase\.removeChannel\(channel\)/);
  assert.match(source, /function assertDxSupabasePresenceChannel/);
  assert.match(source, /function assertDxSupabasePresenceKey/);
  assert.match(source, /value === "realtime"/);
  assert.match(source, /"lib\/supabase\/realtime-presence\.ts"/);
  assert.match(
    source,
    /realtimePresence: "subscribeToDxSupabasePresence\(\{ channel: \\"room:launch:presence\\", key: userId, initialPresence \}\)"/,
  );
});

test("supabase edge function helper invokes real functions with typed errors", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/edge-functions\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /FunctionRegion/);
  assert.match(source, /FunctionsHttpError/);
  assert.match(source, /FunctionsRelayError/);
  assert.match(source, /FunctionsFetchError/);
  assert.match(source, /export type DxSupabaseFunctionInvokeOptions/);
  assert.match(source, /export type DxSupabaseFunctionErrorKind/);
  assert.match(source, /export class DxSupabaseFunctionInvokeError extends Error/);
  assert.match(source, /export async function invokeDxSupabaseFunction/);
  assert.match(source, /export async function tryInvokeDxSupabaseFunction/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /\.functions\.invoke<TResponse>\(/);
  assert.match(source, /assertDxSupabaseFunctionName\(name\)/);
  assert.match(source, /body: options\.body/);
  assert.match(source, /headers: options\.headers/);
  assert.match(source, /method: options\.method/);
  assert.match(source, /region: options\.region/);
  assert.match(source, /error instanceof FunctionsHttpError/);
  assert.match(source, /error instanceof FunctionsRelayError/);
  assert.match(source, /error instanceof FunctionsFetchError/);
  assert.match(source, /function assertDxSupabaseFunctionName/);
  assert.match(source, /\^\[A-Za-z0-9_-\]\+\$/);
  assert.match(source, /"lib\/supabase\/edge-functions\.ts"/);
  assert.match(
    source,
    /edgeFunction: "invokeDxSupabaseFunction\(\\"hello-world\\", \{ body \}\)"/,
  );
});

test("supabase signed storage helper supports private file URLs and uploads", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/signed-storage\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /export type DxSupabaseSignedDownloadOptions/);
  assert.match(source, /export type DxSupabaseSignedDownloadBatchOptions/);
  assert.match(source, /export type DxSupabaseSignedDownloadBatchItem/);
  assert.match(source, /export type DxSupabaseSignedUploadOptions/);
  assert.match(source, /export type DxSupabaseSignedUploadResult/);
  assert.match(source, /export async function createDxSupabaseSignedDownloadUrl/);
  assert.match(source, /export async function createDxSupabaseSignedDownloadUrls/);
  assert.match(source, /export async function createDxSupabaseSignedUploadUrl/);
  assert.match(source, /export async function uploadDxSupabaseToSignedUrl/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.createSignedUrl\(/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.createSignedUrls\(/);
  assert.match(source, /assertDxSupabaseSignedStoragePath\(options\.path\)/);
  assert.match(source, /assertDxSupabaseSignedStoragePaths\(options\.paths\)/);
  assert.match(source, /assertDxSupabaseSignedStorageExpiresIn/);
  assert.match(source, /download: options\.download/);
  assert.match(source, /transform: options\.transform/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.createSignedUploadUrl\(/);
  assert.match(source, /upsert: options\.upsert/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.uploadToSignedUrl\(/);
  assert.match(source, /token: data\.token/);
  assert.match(source, /signedUrl: data\.signedUrl/);
  assert.match(source, /function assertDxSupabaseSignedStorageBucket/);
  assert.match(source, /function assertDxSupabaseSignedStoragePath/);
  assert.match(source, /function assertDxSupabaseSignedStoragePaths/);
  assert.match(source, /function toDxSupabaseSignedDownloadBatchItem/);
  assert.match(source, /function readDxSupabaseSignedStorageItemError/);
  assert.match(source, /path\.includes\("\.\."\)/);
  assert.match(source, /"lib\/supabase\/signed-storage\.ts"/);
  assert.match(
    source,
    /signedDownloads: "createDxSupabaseSignedDownloadUrls\(\{ bucket: \\"uploads\\", paths \}\)"/,
  );
  assert.match(
    source,
    /signedUpload: "createDxSupabaseSignedUploadUrl\(\{ bucket: \\"uploads\\", path \}\)"/,
  );
});

test("supabase storage object helper manages bucket objects safely", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/storage-objects\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /export type DxSupabaseStorageObject/);
  assert.match(source, /export type DxSupabaseStorageListOptions/);
  assert.match(source, /export type DxSupabaseStorageDownloadOptions/);
  assert.match(source, /export type DxSupabaseStorageTransferOptions/);
  assert.match(source, /export async function listDxSupabaseStorageObjects/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.list\(prefix, \{/);
  assert.match(
    source,
    /limit: assertDxSupabaseStorageListLimit\(options\.limit \?\? 100\)/,
  );
  assert.match(
    source,
    /offset: assertDxSupabaseStorageOffset\(options\.offset \?\? 0\)/,
  );
  assert.match(source, /sortBy: options\.sortBy/);
  assert.match(source, /search: options\.search/);
  assert.match(source, /export async function downloadDxSupabaseStorageObject/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.download\(path, \{/);
  assert.match(source, /download: options\.download/);
  assert.match(source, /transform: options\.transform/);
  assert.match(source, /export async function removeDxSupabaseStorageObjects/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.remove\(paths\)/);
  assert.match(source, /paths\.length > 1000/);
  assert.match(source, /export async function copyDxSupabaseStorageObject/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.copy\(fromPath, toPath, \{/);
  assert.match(source, /export async function moveDxSupabaseStorageObject/);
  assert.match(source, /\.storage\s*\.from\(bucket\)\s*\.move\(fromPath, toPath, \{/);
  assert.match(source, /const destinationBucket = options\.destinationBucket/);
  assert.match(source, /destinationBucket,/);
  assert.match(source, /upsert: options\.upsert/);
  assert.match(source, /function assertDxSupabaseStorageBucket/);
  assert.match(source, /function assertDxSupabaseStoragePath/);
  assert.match(source, /function assertDxSupabaseStoragePrefix/);
  assert.match(source, /path\.includes\("\.\."\)/);
  assert.match(source, /"lib\/supabase\/storage-objects\.ts"/);
  assert.match(
    source,
    /storageObjects: "listDxSupabaseStorageObjects\(\{ bucket: \\"uploads\\" \}\)"/,
  );
});

test("supabase database row helper wraps table CRUD safely", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/database-rows\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /export type DxSupabaseRowValue/);
  assert.match(source, /export type DxSupabaseRowData/);
  assert.match(source, /export type DxSupabaseRowFilter/);
  assert.match(source, /export type DxSupabaseRowsResult/);
  assert.match(source, /export async function selectDxSupabaseRows/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /\.from\(table\)\.select\(columns, \{/);
  assert.match(source, /count: options\.count/);
  assert.match(source, /function applyDxSupabaseRowFilters/);
  assert.match(source, /\.order\(assertDxSupabaseRowColumn\(options\.order\.column\), \{/);
  assert.match(source, /\.range\(range\.from, range\.to\)/);
  assert.match(source, /export async function insertDxSupabaseRows/);
  assert.match(source, /\.from\(table\)\.insert\(rows\)\.select\(columns\)/);
  assert.match(source, /export async function upsertDxSupabaseRows/);
  assert.match(source, /\.from\(table\)\.upsert\(rows, \{/);
  assert.match(source, /onConflict: options\.onConflict/);
  assert.match(source, /ignoreDuplicates: options\.ignoreDuplicates/);
  assert.match(source, /export async function updateDxSupabaseRows/);
  assert.match(source, /\.from\(table\)\.update\(values\)/);
  assert.match(source, /assertDxSupabaseRequiredFilters\(options\.filters, "update"\)/);
  assert.match(source, /export async function deleteDxSupabaseRows/);
  assert.match(source, /\.from\(table\)\.delete\(\)/);
  assert.match(source, /assertDxSupabaseRequiredFilters\(options\.filters, "delete"\)/);
  assert.match(source, /function assertDxSupabaseRowTable/);
  assert.match(source, /function assertDxSupabaseRowColumn/);
  assert.match(source, /function assertDxSupabaseSelectColumns/);
  assert.match(source, /"lib\/supabase\/database-rows\.ts"/);
  assert.match(
    source,
    /databaseRows: "selectDxSupabaseRows\(\{ table: \\"instruments\\" \}\)"/,
  );
});

test("supabase database row helper supports postgrest filters and single-row reads", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /export type DxSupabaseRowFilterOperator/);
  assert.match(source, /operator\?: DxSupabaseRowFilterOperator/);
  assert.match(source, /export type DxSupabaseRowOrFilter/);
  assert.match(source, /limit\?: number/);
  assert.match(source, /export async function selectSingleDxSupabaseRow/);
  assert.match(source, /query = applyDxSupabaseRowFilters\(query, options\.filters \?\? \[\]\)/);
  assert.match(source, /query = applyDxSupabaseRowOrFilters\(query, options\.or \?\? \[\]\)/);
  assert.match(source, /\.limit\(assertDxSupabaseRowLimit\(options\.limit\)\)/);
  assert.match(source, /case "neq":\s+return query\.neq\(column, filter\.value\)/);
  assert.match(source, /case "gt":\s+return query\.gt\(column, filter\.value\)/);
  assert.match(source, /case "gte":\s+return query\.gte\(column, filter\.value\)/);
  assert.match(source, /case "lt":\s+return query\.lt\(column, filter\.value\)/);
  assert.match(source, /case "lte":\s+return query\.lte\(column, filter\.value\)/);
  assert.match(source, /case "like":\s+return query\.like\(column, assertDxSupabasePatternFilterValue\(filter\.value, "like"\)\)/);
  assert.match(source, /case "ilike":\s+return query\.ilike\(column, assertDxSupabasePatternFilterValue\(filter\.value, "ilike"\)\)/);
  assert.match(source, /case "is":\s+return query\.is\(column, assertDxSupabaseIsFilterValue\(filter\.value\)\)/);
  assert.match(source, /case "in":\s+return query\.in\(column, assertDxSupabaseRowArrayFilter\(filter\.value, "in"\)\)/);
  assert.match(source, /case "contains":\s+return query\.contains\(column, filter\.value\)/);
  assert.match(source, /case "containedBy":\s+return query\.containedBy\(column, filter\.value\)/);
  assert.match(source, /\.or\(assertDxSupabaseOrFilter\(filter\.expression\), \{/);
  assert.match(source, /options\.required \? query\.single\(\) : query\.maybeSingle\(\)/);
  assert.match(source, /function assertDxSupabaseRowLimit/);
  assert.match(source, /function assertDxSupabaseOrFilter/);
  assert.match(
    source,
    /from\(table\)\.select, eq, neq, gt, gte, lt, lte, like, ilike, is, in, contains, containedBy, or, limit, single, and maybeSingle/,
  );
  assert.match(
    source,
    /rowFilters: "selectDxSupabaseRows\(\{ table: \\"players\\", filters: \[\{ column: \\"age\\", operator: \\"gte\\", value: 20 \}\] \}\)"/,
  );
});

test("supabase rpc helper calls postgres functions with guarded names", () => {
  const source = readSupabaseForgeSource();

  assert.match(source, /"js\/supabase\/rpc\.ts"/);
  assert.match(source, /"use client";/);
  assert.match(source, /export type DxSupabaseRpcArgs/);
  assert.match(source, /export type DxSupabaseRpcOptions/);
  assert.match(source, /export type DxSupabaseRpcResult/);
  assert.match(source, /export async function callDxSupabaseRpc/);
  assert.match(source, /createDxSupabaseBrowserClient\(\)/);
  assert.match(source, /\.rpc\(/);
  assert.match(source, /assertDxSupabaseRpcName\(options\.name\)/);
  assert.match(source, /options\.args \?\? \(\{\} as Args\)/);
  assert.match(source, /get: options\.get/);
  assert.match(source, /head: options\.head/);
  assert.match(source, /count: options\.count/);
  assert.match(source, /count: count \?\? null/);
  assert.match(source, /function assertDxSupabaseRpcName/);
  assert.match(source, /\^\[A-Za-z_\]\[A-Za-z0-9_\]\*\$/);
  assert.match(source, /"lib\/supabase\/rpc\.ts"/);
  assert.match(
    source,
    /rpcCall: "callDxSupabaseRpc\(\{ name: \\"match_documents\\", args \}\)"/,
  );
});
