import { betterAuthDashboardPackage } from "../../auth/better-auth/dashboard";
import { dxMobileAuthContract } from "../../lib/mobile-auth/auth-contract";
import { Icon } from "../icons/icon";

export function DxMobileAuthScreen() {
  return (
    <main
      className="native-auth-shell"
      data-dx-route="/"
      data-dx-mobile-app="auth"
      data-dx-www-framework="app-router"
      data-dx-native-target="tauri-webview"
      data-dx-native-bridge="dx-native-mobile-auth"
      data-dx-style-surface="dx-mobile-auth-screen"
      data-dx-check-surface="dx-mobile-auth"
      data-dx-forge-package="auth/better-auth"
      data-dx-auth-runtime-status={dxMobileAuthContract.runtimeStatus}
    >
      <section className="native-auth-stage" aria-labelledby="mobile-auth-title">
        <div className="native-auth-copy">
          <a className="native-auth-brand" href="/" aria-label="DX Mobile App">
            <img src="/logo.svg" alt="" />
            <span>
              <strong>DX Mobile App</strong>
              <small>{dxMobileAuthContract.nativeRuntime}</small>
            </span>
          </a>

          <div className="native-auth-heading">
            <h1 id="mobile-auth-title">Sign in to DX Mobile</h1>
            <p>
              Continue to your DX workspace with the same account you use across
              Friday, WWW, and the native app.
            </p>
          </div>
        </div>

        <form
          className="auth-form"
          action="/api/auth/sign-in/email"
          method="post"
          data-dx-auth-interaction="email-sign-in"
          data-dx-auth-provider={betterAuthDashboardPackage.officialName}
          data-dx-native-bridge={dxMobileAuthContract.nativeBridge}
        >
          <div className="auth-form-heading">
            <h2>Welcome back</h2>
            <p>Enter your email and password to continue.</p>
          </div>

          <label className="auth-field" htmlFor="dx-mobile-email">
            <span>Email</span>
            <input
              id="dx-mobile-email"
              name="email"
              type="email"
              inputMode="email"
              autoComplete="email"
              placeholder="essence@dx.local"
              required
            />
          </label>

          <label className="auth-field" htmlFor="dx-mobile-password">
            <span>Password</span>
            <input
              id="dx-mobile-password"
              name="password"
              type="password"
              autoComplete="current-password"
              placeholder="Enter password"
              required
            />
          </label>

          <button className="auth-submit" type="submit">
            <span>Continue</span>
            <Icon name="pack:arrow-right" className="auth-icon auth-icon-arrow" />
          </button>

          <button className="auth-secondary" type="button" disabled aria-disabled="true">
            Passkey
          </button>
        </form>

        <aside className="native-auth-panel" aria-label="Native session status">
          <div>
            <span className="native-auth-panel-label">Session</span>
            <strong>Secure sign-in</strong>
          </div>
          <div className="native-auth-panel-row">
            <Icon name="pack:check" className="auth-icon auth-icon-check" />
            <span>Email and password</span>
          </div>
          <div className="native-auth-panel-row">
            <Icon name="pack:check" className="auth-icon auth-icon-check" />
            <span>Native WebView bridge</span>
          </div>
        </aside>

        <div
          className="native-auth-contract"
          data-dx-contract="dx-mobile-auth"
          data-dx-forge-auth="auth/better-auth"
          data-dx-native-runtime="DX Native"
          data-dx-native-bundle="com.essencefromexistence.dx.mobile"
          data-dx-native-shell-receipt={dxMobileAuthContract.nativeShellReceipt}
          hidden
        />
      </section>
    </main>
  );
}
