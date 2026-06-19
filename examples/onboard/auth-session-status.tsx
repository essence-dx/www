"use client";

import { type FormEvent, type ReactNode, useState } from "react";

import {
  linkDxBetterAuthSocialAccount,
  listDxBetterAuthAccounts,
} from "@/auth/better-auth/accounts";
import {
  deleteDxBetterAuthAccount,
  DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION,
} from "@/auth/better-auth/account-deletion";
import { requestDxBetterAuthPasswordReset } from "@/auth/better-auth/account-security";
import {
  signInDxBetterAuthEmail,
  signUpDxBetterAuthEmail,
} from "@/auth/better-auth/email-password";
import {
  changeDxBetterAuthEmail,
  updateDxBetterAuthUserProfile,
} from "@/auth/better-auth/profile";
import {
  signInDxBetterAuthSocial,
  type DxBetterAuthSocialProvider,
} from "@/auth/better-auth/social";
import {
  listSessions,
  revokeOtherSessions,
  signOut,
  useSession,
} from "@/auth/better-auth/client";
import { dxBetterAuthDashboardReadiness } from "@/auth/better-auth/dashboard";
import { dxBetterAuthForgePackage } from "@/auth/better-auth/metadata";

type BetterAuthClientSessionResponse =
  | unknown[]
  | {
      data?: unknown[] | null;
      error?: {
        message?: string;
      } | null;
    };

type BetterAuthClientAccountsResponse = BetterAuthClientSessionResponse;
type SessionAction = "idle" | "listing" | "revoking";
type EmailAction = "idle" | "signing-in" | "signing-up" | "requesting-reset";
type SocialAction = "idle" | DxBetterAuthSocialProvider;
type AccountAction =
  | "idle"
  | "listing"
  | `linking-${DxBetterAuthSocialProvider}`;
type ProfileAction = "idle" | "updating";
type DeleteAction = "idle" | "deleting";

type LaunchAuthBoundaryStatus = {
  kind: "configured" | "same-origin-missing-config";
  message: string;
  clientBaseUrl: string;
};

type LaunchAuthBoundaryReviewState = {
  kind: "idle" | "checked" | "safe-action-preview";
  message: string;
  checkedAt?: string;
};

type LaunchAuthBoundaryReview = {
  email: string;
  name: string;
  reviewedAt: string;
};

const launchAuthProviderReadiness = [
  {
    provider: "google",
    label: "Google",
    requiredEnv: "GOOGLE_CLIENT_ID + GOOGLE_CLIENT_SECRET",
  },
] as const;

const launchAuthServerEnv = dxBetterAuthForgePackage.requiredEnv.filter(
  (key) => key !== "NEXT_PUBLIC_BETTER_AUTH_URL",
);

const launchAuthDashboardMissingConfig =
  dxBetterAuthDashboardReadiness.missingConfig.join(",");

const idleBoundaryReviewState = {
  kind: "idle",
  message:
    "Run the local readiness check or preview a sign-in action without sending credentials.",
} satisfies LaunchAuthBoundaryReviewState;

export function LaunchAuthSessionStatus() {
  const session = useSession();
  const [authBoundary, setAuthBoundary] = useState(
    readLaunchAuthBoundaryStatus,
  );
  const [boundaryReviewState, setBoundaryReviewState] =
    useState<LaunchAuthBoundaryReviewState>(idleBoundaryReviewState);
  const [boundaryReview, setBoundaryReview] =
    useState<LaunchAuthBoundaryReview | null>(null);
  const [activeSessionCount, setActiveSessionCount] = useState<number | null>(null);
  const [sessionAction, setSessionAction] = useState<SessionAction>("idle");
  const [sessionError, setSessionError] = useState<string | null>(null);
  const [linkedAccountCount, setLinkedAccountCount] = useState<number | null>(null);
  const [accountAction, setAccountAction] = useState<AccountAction>("idle");
  const [accountError, setAccountError] = useState<string | null>(null);
  const [emailAction, setEmailAction] = useState<EmailAction>("idle");
  const [emailError, setEmailError] = useState<string | null>(null);
  const [socialAction, setSocialAction] = useState<SocialAction>("idle");
  const [socialError, setSocialError] = useState<string | null>(null);
  const [passwordResetMessage, setPasswordResetMessage] = useState<string | null>(null);
  const [passwordResetError, setPasswordResetError] = useState<string | null>(null);
  const [profileAction, setProfileAction] = useState<ProfileAction>("idle");
  const [profileMessage, setProfileMessage] = useState<string | null>(null);
  const [profileError, setProfileError] = useState<string | null>(null);
  const [deleteAction, setDeleteAction] = useState<DeleteAction>("idle");
  const [deleteMessage, setDeleteMessage] = useState<string | null>(null);
  const [deleteError, setDeleteError] = useState<string | null>(null);
  const isAuthConfigured = authBoundary.kind === "configured";
  const visibleUser = session.data?.user;
  const userLabel =
    visibleUser?.email ?? visibleUser?.name ?? boundaryReview?.email ?? boundaryReview?.name;

  async function refreshActiveSessions() {
    if (!isAuthConfigured) {
      previewBlockedAccountAction("list sessions");
      setSessionError(
        "Session refresh is blocked until Authentication server config exists.",
      );
      setActiveSessionCount(null);
      return;
    }

    setSessionAction("listing");
    setSessionError(null);

    try {
      const response = (await listSessions()) as BetterAuthClientSessionResponse;
      setActiveSessionCount(countBetterAuthSessions(response));
    } catch (error) {
      setSessionError(error instanceof Error ? error.message : "Unable to list sessions");
    } finally {
      setSessionAction("idle");
    }
  }

  async function refreshLinkedAccounts() {
    if (!isAuthConfigured) {
      previewBlockedAccountAction("list linked accounts");
      setAccountError(
        "Linked-account refresh is blocked until Authentication server config exists.",
      );
      setLinkedAccountCount(null);
      return;
    }

    setAccountAction("listing");
    setAccountError(null);

    try {
      const response = (await listDxBetterAuthAccounts()) as BetterAuthClientAccountsResponse;
      setLinkedAccountCount(countBetterAuthSessions(response));
    } catch (error) {
      setAccountError(error instanceof Error ? error.message : "Unable to list accounts");
    } finally {
      setAccountAction("idle");
    }
  }

  async function linkSocialAccount(provider: DxBetterAuthSocialProvider) {
    if (!isAuthConfigured) {
      previewBlockedAccountAction(`link ${provider}`);
      setAccountError(
        `${provider} account linking is blocked until Authentication server config and OAuth credentials exist.`,
      );
      return;
    }

    setAccountAction(`linking-${provider}`);
    setAccountError(null);

    try {
      await linkDxBetterAuthSocialAccount({
        provider,
        callbackURL: "/",
        errorCallbackURL: "/?auth_error=link",
      });
    } catch (error) {
      setAccountError(
        error instanceof Error ? error.message : "Unable to link account",
      );
    } finally {
      setAccountAction("idle");
    }
  }

  async function revokeOtherActiveSessions() {
    if (!isAuthConfigured) {
      previewBlockedAccountAction("revoke other sessions");
      setSessionError(
        "Session revocation is blocked until Authentication server config exists.",
      );
      return;
    }

    setSessionAction("revoking");
    setSessionError(null);

    try {
      await revokeOtherSessions();
      await refreshActiveSessions();
    } catch (error) {
      setSessionError(error instanceof Error ? error.message : "Unable to revoke sessions");
      setSessionAction("idle");
    }
  }

  async function submitEmailSignIn(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);

    if (!isAuthConfigured) {
      previewSafeEmailAction(readFormValue(formData, "email"));
      setEmailError(
        "Authentication server config is missing; previewed the sign-in action without sending credentials.",
      );
      return;
    }

    setEmailAction("signing-in");
    setEmailError(null);

    try {
      await signInDxBetterAuthEmail({
        email: readFormValue(formData, "email"),
        password: readFormValue(formData, "password"),
        rememberMe: true,
        callbackURL: "/",
      });
    } catch (error) {
      setEmailError(error instanceof Error ? error.message : "Unable to sign in");
    } finally {
      setEmailAction("idle");
    }
  }

  async function submitEmailSignUp(formData: FormData) {
    if (!isAuthConfigured) {
      previewSafeSignUpAction(
        readFormValue(formData, "name"),
        readFormValue(formData, "email"),
      );
      setEmailError(
        "Authentication server config is missing; previewed the sign-up action without creating an account.",
      );
      return;
    }

    setEmailAction("signing-up");
    setEmailError(null);

    try {
      await signUpDxBetterAuthEmail({
        email: readFormValue(formData, "email"),
        name: readFormValue(formData, "name"),
        password: readFormValue(formData, "password"),
        callbackURL: "/",
      });
    } catch (error) {
      setEmailError(
        error instanceof Error ? error.message : "Unable to create account",
      );
    } finally {
      setEmailAction("idle");
    }
  }

  async function signInWithSocialProvider(provider: DxBetterAuthSocialProvider) {
    if (!isAuthConfigured) {
      previewSafeProviderAction(provider);
      setSocialError(
        `${provider} sign-in is blocked until Authentication server config and OAuth credentials exist.`,
      );
      return;
    }

    setSocialAction(provider);
    setSocialError(null);

    try {
      await signInDxBetterAuthSocial({
        provider,
        callbackURL: "/",
        errorCallbackURL: "/?auth_error=social",
        newUserCallbackURL: "/?auth=new",
      });
    } catch (error) {
      setSocialError(
        error instanceof Error ? error.message : "Unable to start provider sign-in",
      );
    } finally {
      setSocialAction("idle");
    }
  }

  async function submitPasswordResetRequest(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);

    if (!isAuthConfigured) {
      setPasswordResetError(
        "Password reset is blocked until Authentication server config exists.",
      );
      setPasswordResetMessage(null);
      setBoundaryReviewState({
        kind: "safe-action-preview",
        checkedAt: new Date().toISOString(),
        message:
          "Prepared an Authentication password-reset action locally; no email or network request was sent.",
      });
      return;
    }

    setEmailAction("requesting-reset");
    setPasswordResetError(null);
    setPasswordResetMessage(null);

    try {
      await requestDxBetterAuthPasswordReset({
        email: readFormValue(formData, "email"),
        redirectTo: "/reset-password",
      });
      setPasswordResetMessage("Check your email for the reset link");
    } catch (error) {
      setPasswordResetError(
        error instanceof Error ? error.message : "Unable to request reset",
      );
    } finally {
      setEmailAction("idle");
    }
  }

  async function submitProfileUpdate(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const form = event.currentTarget;
    const formData = new FormData(form);
    const name = readFormValue(formData, "name").trim();
    const newEmail = readFormValue(formData, "newEmail").trim();

    if (!name && !newEmail) {
      setProfileError("Add a display name or new email to update");
      setProfileMessage(null);
      return;
    }

    if (!isAuthConfigured) {
      previewBlockedAccountAction("profile update");
      setProfileError(
        "Profile update is blocked until Authentication server config exists.",
      );
      setProfileMessage(null);
      return;
    }

    setProfileAction("updating");
    setProfileError(null);
    setProfileMessage(null);

    try {
      if (name) {
        await updateDxBetterAuthUserProfile({ name });
      }

      if (newEmail) {
        await changeDxBetterAuthEmail({
          newEmail,
          callbackURL: "/",
        });
      }

      setProfileMessage(
        newEmail
          ? "Profile saved; check your email to confirm the new address"
          : "Profile saved",
      );
      form.reset();
    } catch (error) {
      setProfileError(
        error instanceof Error ? error.message : "Unable to update profile",
      );
    } finally {
      setProfileAction("idle");
    }
  }

  async function submitAccountDeletion(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const form = event.currentTarget;
    const formData = new FormData(form);
    const confirmation = readFormValue(formData, "deleteConfirmation").trim();
    const password = readFormValue(formData, "deletePassword");

    setDeleteAction("deleting");
    setDeleteError(null);
    setDeleteMessage(null);

    try {
      if (confirmation !== DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION) {
        throw new Error(
          `Type "${DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION}" to continue`,
        );
      }

      if (!isAuthConfigured) {
        previewBlockedAccountAction("account deletion");
        setDeleteError(
          "Account deletion is blocked until Authentication server config exists.",
        );
        return;
      }

      await deleteDxBetterAuthAccount({
        confirmation: DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION,
        password,
        callbackURL: "/goodbye",
      });
      setDeleteMessage("Account deletion request accepted");
      form.reset();
    } catch (error) {
      setDeleteError(
        error instanceof Error
          ? error.message
          : "Unable to request account deletion",
      );
    } finally {
      setDeleteAction("idle");
    }
  }

  function refreshLocalReadiness() {
    const nextStatus = readLaunchAuthBoundaryStatus();
    setAuthBoundary(nextStatus);
    setBoundaryReviewState({
      kind: "checked",
      checkedAt: new Date().toISOString(),
      message:
        nextStatus.kind === "configured"
          ? `Client auth URL is configured at ${nextStatus.clientBaseUrl}. Server secrets and adapter wiring still stay app-owned.`
          : "No public Authentication URL is configured, so the template keeps auth on same-origin and does not send credentials.",
    });
  }

  function previewSafeEmailAction(email: string) {
    const target = email.trim() || "an email address";
    setBoundaryReviewState({
      kind: "safe-action-preview",
      checkedAt: new Date().toISOString(),
      message: `Prepared an Authentication email sign-in action for ${target}; no password or network request was sent.`,
    });
  }

  function previewSafeSignUpAction(name: string, email: string) {
    const target = email.trim() || "an email address";
    const label = name.trim() || "a launch user";
    setBoundaryReviewState({
      kind: "safe-action-preview",
      checkedAt: new Date().toISOString(),
      message: `Prepared an Authentication email sign-up action for ${label} at ${target}; no account, password, or network request was created.`,
    });
  }

  function markBoundaryReviewed(formData?: FormData) {
    const reviewedAt = new Date().toISOString();
    const email = readFormValue(formData ?? new FormData(), "email").trim() || "owner@dx.local";
    const name = readFormValue(formData ?? new FormData(), "name").trim() || "Launch owner";

    setBoundaryReview({
      reviewedAt,
      email,
      name,
    });
    setActiveSessionCount(null);
    setLinkedAccountCount(null);
    setBoundaryReviewState({
      kind: "safe-action-preview",
      checkedAt: reviewedAt,
      message:
        `Marked Authentication boundary reviewed for ${email}; no cookie, server session, or account was created.`,
    });
  }

  function clearBoundaryReview() {
    setBoundaryReview(null);
    setActiveSessionCount(null);
    setLinkedAccountCount(null);
    setBoundaryReviewState({
      kind: "checked",
      checkedAt: new Date().toISOString(),
      message: "Cleared the local Authentication boundary review marker.",
    });
  }

  function previewSafeProviderAction(provider: DxBetterAuthSocialProvider) {
    setBoundaryReviewState({
      kind: "safe-action-preview",
      checkedAt: new Date().toISOString(),
      message: `Prepared an Authentication ${provider} sign-in action locally; OAuth redirect was not started.`,
    });
  }

  function previewBlockedAccountAction(action: string) {
    setBoundaryReviewState({
      kind: "safe-action-preview",
      checkedAt: new Date().toISOString(),
      message: `Prepared an Authentication ${action} action locally; no account network request was sent.`,
    });
  }

  let sessionSurface: ReactNode;

  if (session.isPending) {
    sessionSurface = <p data-auth-state="loading">Session loading</p>;
  } else if (!session.data && !boundaryReview) {
    sessionSurface = (
      <div className="grid gap-3" data-auth-state="anonymous">
        <form
          className="grid gap-2"
          data-auth-email-password="ready"
          data-auth-email-sign-up="ready"
          data-dx-auth-email-action-state={emailAction}
          data-dx-auth-network-state={isAuthConfigured ? "configured" : "missing-config"}
          data-dx-auth-network-disabled={!isAuthConfigured ? "true" : undefined}
          onSubmit={(event) => void submitEmailSignIn(event)}
        >
          <p>No launch session</p>
          <input
            aria-label="Name"
            autoComplete="name"
            name="name"
            placeholder="Launch owner"
            type="text"
          />
          <input
            aria-label="Email"
            autoComplete="email"
            name="email"
            placeholder="you@example.com"
            type="email"
          />
          <input
            aria-label="Password"
            autoComplete="current-password"
            name="password"
            placeholder="Password"
            type="password"
          />
          {emailError ? <p data-auth-email-error>{emailError}</p> : null}
          <div className="flex flex-wrap gap-2">
            <button
              aria-busy={emailAction === "signing-in"}
              aria-disabled={!isAuthConfigured}
              disabled={emailAction !== "idle"}
              data-dx-auth-interaction="email-sign-in"
              type="submit"
            >
              Sign in with email
            </button>
            <button
              aria-busy={emailAction === "signing-up"}
              aria-disabled={!isAuthConfigured}
              data-dx-auth-interaction="email-sign-up"
              disabled={emailAction !== "idle"}
              onClick={(event) => {
                const form = event.currentTarget.form;
                if (!form) {
                  return;
                }

                void submitEmailSignUp(new FormData(form));
              }}
              type="button"
            >
              Create account
            </button>
            <button
              data-dx-auth-interaction="safe-email-preview"
              onClick={(event) => {
                const form = event.currentTarget.form;
                const formData = form ? new FormData(form) : new FormData();
                previewSafeEmailAction(readFormValue(formData, "email"));
              }}
              type="button"
            >
              Preview safe action
            </button>
            <button
              data-dx-auth-interaction="mark-boundary-reviewed"
              onClick={(event) => {
                const form = event.currentTarget.form;
                markBoundaryReviewed(form ? new FormData(form) : undefined);
              }}
              type="button"
            >
              Mark boundary reviewed
            </button>
          </div>
        </form>
        <div
          className="flex flex-wrap gap-2"
          data-auth-social-provider="ready"
          data-dx-auth-provider-network-state={
            isAuthConfigured ? "configured" : "missing-config"
          }
        >
          {socialError ? <p data-auth-social-error>{socialError}</p> : null}
          <button
            aria-busy={socialAction === "google"}
            aria-disabled={!isAuthConfigured}
            data-dx-auth-interaction="social-google"
            disabled={socialAction !== "idle"}
            onClick={() => void signInWithSocialProvider("google")}
            type="button"
          >
            Continue with Google
          </button>
        </div>
        <form
          className="grid gap-2"
          data-auth-password-reset="ready"
          data-dx-auth-network-state={isAuthConfigured ? "configured" : "missing-config"}
          onSubmit={(event) => void submitPasswordResetRequest(event)}
        >
          <input
            aria-label="Password reset email"
            autoComplete="email"
            name="email"
            placeholder="you@example.com"
            type="email"
          />
          {passwordResetMessage ? (
            <p data-auth-password-reset-message>{passwordResetMessage}</p>
          ) : null}
          {passwordResetError ? (
            <p data-auth-password-reset-error>{passwordResetError}</p>
          ) : null}
          <button aria-busy={emailAction === "requesting-reset"} type="submit">
            Send reset link
          </button>
        </form>
      </div>
    );
  } else {
    const isBusy = sessionAction !== "idle";
    const isAccountBusy = accountAction !== "idle";
    const isProfileBusy = profileAction !== "idle";
    const isDeleteBusy = deleteAction !== "idle";

    sessionSurface = (
        <div
          className="grid gap-2"
          data-auth-session-management="ready"
          data-auth-state={session.data ? "signed-in" : "boundary-reviewed"}
          data-dx-auth-session-source={session.data ? "better-auth" : "boundary-review"}
          data-dx-auth-boundary-review={boundaryReview?.reviewedAt}
          data-dx-auth-network-state={isAuthConfigured ? "configured" : "missing-config"}
          data-dx-auth-network-disabled={!isAuthConfigured ? "true" : undefined}
        >
          <p>
            {session.data
              ? `Signed in as ${userLabel ?? "launch user"}`
              : `Authentication boundary reviewed for ${userLabel ?? "launch user"}`}
          </p>
          {boundaryReview ? (
            <p data-dx-auth-boundary-review={boundaryReview.reviewedAt}>
              Local review marker only; Authentication did not create a cookie,
              server session, or account.
            </p>
          ) : null}
        <p data-auth-session-count>
          Active sessions: {activeSessionCount ?? "not loaded"}
        </p>
        {sessionError ? <p data-auth-session-error>{sessionError}</p> : null}
        <form
          className="grid gap-2"
          data-auth-profile="ready"
          data-dx-auth-network-state={isAuthConfigured ? "configured" : "missing-config"}
          data-dx-auth-network-disabled={!isAuthConfigured ? "true" : undefined}
          onSubmit={(event) => void submitProfileUpdate(event)}
        >
          <input
            aria-label="Display name"
            autoComplete="name"
            defaultValue={visibleUser?.name ?? ""}
            name="name"
            placeholder="Display name"
            type="text"
          />
          <input
            aria-label="New email"
            autoComplete="email"
            name="newEmail"
            placeholder={visibleUser?.email ?? "new@email.com"}
            type="email"
          />
          {profileMessage ? <p data-auth-profile-message>{profileMessage}</p> : null}
          {profileError ? <p data-auth-profile-error>{profileError}</p> : null}
          <button
            aria-busy={isProfileBusy}
            aria-disabled={!isAuthConfigured}
            disabled={isProfileBusy}
            type="submit"
          >
            Update profile
          </button>
        </form>
        <div
          className="grid gap-2"
          data-auth-linked-accounts="ready"
          data-dx-auth-network-state={isAuthConfigured ? "configured" : "missing-config"}
          data-dx-auth-network-disabled={!isAuthConfigured ? "true" : undefined}
        >
          <p data-auth-linked-account-count>
            Linked accounts: {linkedAccountCount ?? "not loaded"}
          </p>
          {accountError ? <p data-auth-linked-account-error>{accountError}</p> : null}
          <div className="flex flex-wrap gap-2">
            <button
              aria-busy={accountAction === "listing"}
              aria-disabled={!isAuthConfigured}
              disabled={isAccountBusy}
              onClick={() => void refreshLinkedAccounts()}
              type="button"
            >
              Refresh linked accounts
            </button>
            <button
              aria-busy={accountAction === "linking-google"}
              aria-disabled={!isAuthConfigured}
              disabled={isAccountBusy}
              onClick={() => void linkSocialAccount("google")}
              type="button"
            >
              Link Google
            </button>
          </div>
        </div>
        <form
          className="grid gap-2"
          data-auth-account-deletion="ready"
          data-dx-auth-network-state={isAuthConfigured ? "configured" : "missing-config"}
          data-dx-auth-network-disabled={!isAuthConfigured ? "true" : undefined}
          onSubmit={(event) => void submitAccountDeletion(event)}
        >
          <input
            aria-label="Account deletion password"
            autoComplete="current-password"
            name="deletePassword"
            placeholder="Current password"
            type="password"
          />
          <input
            aria-label="Account deletion confirmation"
            name="deleteConfirmation"
            placeholder={DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION}
            type="text"
          />
          {deleteMessage ? (
            <p data-auth-account-deletion-message>{deleteMessage}</p>
          ) : null}
          {deleteError ? (
            <p data-auth-account-deletion-error>{deleteError}</p>
          ) : null}
          <button
            aria-busy={isDeleteBusy}
            aria-disabled={!isAuthConfigured}
            disabled={isDeleteBusy}
            type="submit"
          >
            Delete account
          </button>
        </form>
        <div
          className="flex flex-wrap gap-2"
          data-dx-auth-network-state={isAuthConfigured ? "configured" : "missing-config"}
          data-dx-auth-network-disabled={!isAuthConfigured ? "true" : undefined}
        >
          <button
            aria-busy={sessionAction === "listing"}
            aria-disabled={!isAuthConfigured}
            disabled={isBusy}
            onClick={() => void refreshActiveSessions()}
            type="button"
          >
            Refresh sessions
          </button>
          <button
            aria-busy={sessionAction === "revoking"}
            aria-disabled={!isAuthConfigured}
            disabled={isBusy}
            onClick={() => void revokeOtherActiveSessions()}
            type="button"
          >
            Sign out other sessions
          </button>
          <button
            data-dx-auth-interaction={
              session.data ? "sign-out" : "clear-boundary-review"
            }
            disabled={isBusy}
            onClick={() =>
              session.data ? void signOut() : clearBoundaryReview()
            }
            type="button"
          >
            {session.data ? "Sign out" : "Clear boundary review"}
          </button>
        </div>
      </div>
    );
  }

  const sessionState = session.isPending
    ? "loading"
    : session.data
      ? "signed-in"
      : boundaryReview
        ? "boundary-reviewed"
      : "anonymous";

  return (
    <section
      className="grid gap-4"
      data-dx-package="auth/better-auth"
      data-dx-component="better-auth-session-status-panel"
      data-dx-parent-component="better-auth-account-dashboard-workflow"
      data-dx-legacy-component="launch-auth-session-proof"
      data-dx-auth-dashboard-workflow="login-session-account"
      data-dx-auth-proof="session-readiness"
      data-dx-auth-package-id={dxBetterAuthForgePackage.packageId}
      data-dx-auth-receipt-path={dxBetterAuthForgePackage.receiptPaths.package}
      data-dx-auth-source-mirror={dxBetterAuthForgePackage.sourceMirror}
      data-dx-auth-state={sessionState}
      data-dx-auth-config={authBoundary.kind}
      data-dx-auth-missing-config={launchAuthDashboardMissingConfig}
      data-dx-auth-missing-provider-state={
        authBoundary.kind === "configured" ? "ready" : "missing-config"
      }
      data-dx-product-surface="account-access"
      data-dx-style-surface="authentication-session-status"
      data-dx-icon-search="auth:account"
      data-dx-node-modules="forbidden"
    >
      <span className="sr-only" data-dx-component="launch-auth-session-proof">
        Authentication launch session compatibility marker
      </span>
      <LaunchAuthReadinessCard
        status={authBoundary}
        boundaryReviewState={boundaryReviewState}
        onRefresh={refreshLocalReadiness}
      />
      {sessionSurface}
    </section>
  );
}

function LaunchAuthReadinessCard({
  boundaryReviewState,
  onRefresh,
  status,
}: {
  boundaryReviewState: LaunchAuthBoundaryReviewState;
  onRefresh: () => void;
  status: LaunchAuthBoundaryStatus;
}) {
  const providerState =
    status.kind === "configured" ? "configured" : "missing-config";

  return (
    <div
      className="grid gap-3 rounded-md border p-3"
      data-dx-auth-readiness-card={status.kind}
      data-dx-auth-client-url={status.clientBaseUrl}
    >
      <div className="flex items-start justify-between gap-3">
        <div>
          <p className="flex items-center gap-2 text-sm font-medium">
            <dx-icon name="pack:auth" aria-hidden="true" />
            Authentication readiness
          </p>
          <p className="text-xs leading-5 text-muted-foreground">
            {status.message}
          </p>
        </div>
        <button
          className="rounded-md border px-3 py-2 text-xs font-medium"
          data-dx-auth-interaction="local-readiness-check"
          onClick={onRefresh}
          type="button"
        >
          Check
        </button>
      </div>

      <div className="grid gap-2 sm:grid-cols-2">
        {launchAuthProviderReadiness.map((provider) => (
          <div
            className="rounded-md border p-2"
            data-dx-auth-provider={provider.provider}
            data-dx-auth-provider-state={providerState}
            data-dx-auth-missing-provider={
              providerState === "missing-config" ? provider.provider : undefined
            }
            key={provider.provider}
          >
            <p className="text-xs text-muted-foreground">{provider.label}</p>
            <p className="text-sm font-medium">Provider-ready surface</p>
            <p className="text-xs leading-5 text-muted-foreground">
              Required server env: {provider.requiredEnv}
            </p>
          </div>
        ))}
      </div>

      <p
        className="rounded-md bg-muted px-3 py-2 text-xs leading-5 text-muted-foreground"
        data-dx-auth-boundary-review-state={boundaryReviewState.kind}
        role="status"
      >
        {boundaryReviewState.message}
        {boundaryReviewState.checkedAt ? ` Checked at ${boundaryReviewState.checkedAt}.` : ""}
      </p>
      <p
        className="text-xs leading-5 text-muted-foreground"
        data-dx-auth-required-env={launchAuthServerEnv.join(",")}
      >
        Server-owned env: {launchAuthServerEnv.join(", ")}. OAuth secrets and
        database adapter wiring are intentionally not imported into the browser.
      </p>
      <p
        className="text-xs leading-5 text-muted-foreground"
        data-dx-auth-dashboard-receipt={
          dxBetterAuthForgePackage.receiptPaths.dashboard
        }
      >
        Forge receipt: {dxBetterAuthForgePackage.receiptPaths.package}. Source
        mirror: {dxBetterAuthForgePackage.sourceMirror}.
      </p>
    </div>
  );
}

function countBetterAuthSessions(response: BetterAuthClientSessionResponse): number {
  if (Array.isArray(response)) {
    return response.length;
  }

  if (Array.isArray(response.data)) {
    return response.data.length;
  }

  if (response.error?.message) {
    throw new Error(response.error.message);
  }

  return 0;
}

function readFormValue(formData: FormData, key: string): string {
  const value = formData.get(key);

  if (typeof value !== "string") {
    return "";
  }

  return value;
}

function readLaunchAuthBoundaryStatus(): LaunchAuthBoundaryStatus {
  const publicBaseUrl = readPublicLaunchEnv("NEXT_PUBLIC_BETTER_AUTH_URL");

  if (publicBaseUrl) {
    return {
      kind: "configured",
      clientBaseUrl: publicBaseUrl,
      message:
        "NEXT_PUBLIC_BETTER_AUTH_URL is configured for the browser client.",
    };
  }

  return {
    kind: "same-origin-missing-config",
    clientBaseUrl: "same-origin:/api/auth",
    message:
      "NEXT_PUBLIC_BETTER_AUTH_URL is not set; the generated launch template stays on same-origin and shows safe local auth actions until server config exists.",
  };
}

function readPublicLaunchEnv(key: string): string | null {
  const env = (globalThis as unknown as {
    process?: { env?: Record<string, string | undefined> };
  }).process?.env;
  const value = env?.[key]?.trim();

  return value ? value : null;
}
