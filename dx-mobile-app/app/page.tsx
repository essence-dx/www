import { dxMobileCompanionContract } from "../lib/mobile-companion/companion-contract";
import { readDxMobileCompanionReadiness } from "../server/mobile-companion/readiness";

const readiness = readDxMobileCompanionReadiness();

export const metadata = {
  title: "DX Mobile App",
  description:
    "DX Mobile App is a native companion for Zed and DX Agents, built with DX WWW, DX Native, Forge, Check, Style, and Icon.",
} as const;

export function DxMobileCompanionScreen() {
  return (
    <main
      className="native-companion-shell"
      data-dx-route="/"
      data-dx-mobile-app="companion"
      data-dx-www-framework="app-router"
      data-dx-native-target="tauri-webview"
      data-dx-native-bridge="dx-native-mobile-companion"
      data-dx-native-bundle="com.essencefromexistence.dx.mobile"
      data-dx-style-surface="dx-mobile-companion-screen"
      data-dx-check-surface="dx-mobile-companion"
      data-dx-forge-package="auth/better-auth"
      data-dx-agents-gateway="dx-agents"
      data-dx-pairing-contract="dx.mobile.pairing"
      data-dx-chat-contract="dx.mobile.chat_handoff"
      data-dx-chat-transport="/ws/chat"
      data-dx-acp-transport="/acp"
      data-dx-session-key="session_key"
    >
      <section className="companion-stage" aria-labelledby="dx-mobile-title">
        <header className="companion-hero">
          <a className="companion-brand" href="/" aria-label="DX Mobile App">
            <img src="/logo.svg" alt="" />
            <span>
              <strong>DX Mobile App</strong>
              <small>{dxMobileCompanionContract.nativeRuntime}</small>
            </span>
          </a>

          <div className="companion-heading">
            <h1 id="dx-mobile-title">Pair with Zed</h1>
            <p>Zed chats, DX Agents sessions, and approvals from your phone.</p>
          </div>
        </header>

        <section className="companion-pairing-panel" aria-labelledby="dx-mobile-pairing-title">
          <div className="companion-panel-heading">
            <span className="companion-kicker">Secure pairing</span>
            <h2 id="dx-mobile-pairing-title">QR handoff</h2>
          </div>

          <form
            className="dx-mobile-pairing-form"
            data-dx-pairing-interaction="qr-session-handoff"
            data-dx-pairing-endpoint="/api/pair"
            data-dx-legacy-pairing-endpoint="/pair"
            data-dx-device-registry-endpoint="/api/pair"
            data-dx-native-command="dx_mobile_parse_pairing_payload"
          >
            <label className="companion-field" htmlFor="dx-mobile-gateway-origin">
              <span>Gateway</span>
              <input
                id="dx-mobile-gateway-origin"
                name="gateway_origin"
                type="url"
                inputMode="url"
                placeholder="http://127.0.0.1:42617"
                autoComplete="url"
              />
            </label>

            <label className="companion-field" htmlFor="dx-mobile-pairing-code">
              <span>Pairing code</span>
              <input
                id="dx-mobile-pairing-code"
                name="pairing_code"
                type="text"
                inputMode="numeric"
                autoComplete="one-time-code"
                placeholder="000000"
              />
            </label>

            <label className="companion-field" htmlFor="dx-mobile-agent-alias">
              <span>Agent</span>
              <input
                id="dx-mobile-agent-alias"
                name="agent_alias"
                type="text"
                autoComplete="off"
                placeholder={dxMobileCompanionContract.suggestedAgentAlias}
              />
            </label>

            <label className="companion-field companion-field-wide" htmlFor="dx-mobile-pairing-payload">
              <span>QR payload</span>
              <textarea
                id="dx-mobile-pairing-payload"
                name="pairing_payload"
                rows={4}
                spellCheck={false}
                placeholder={`${dxMobileCompanionContract.pairingPayload.preferredScheme}?gateway=...&code=...`}
              ></textarea>
            </label>

            <div className="companion-actions">
              <button
                className="companion-secondary-action"
                type="button"
                data-dx-native-command="dx_mobile_scan_pairing_qr"
              >
                Scan QR
              </button>
              <button className="companion-primary-action" type="submit">
                <span>Connect</span>
                <span
                  aria-hidden="true"
                  className="companion-icon companion-icon-check"
                  data-dx-icon="status:check"
                  data-icon-source="dx-icons"
                ></span>
              </button>
            </div>
          </form>

          <output
            className="companion-runtime-status"
            id="dx-mobile-runtime-status"
            data-dx-mobile-runtime-status="source-ready"
            data-dx-native-store-command="dx_mobile_store_device_credential"
            data-dx-native-load-command="dx_mobile_load_device_credential"
            data-dx-native-clear-command="dx_mobile_clear_device_credential"
          >
            Waiting for desktop pairing
          </output>
        </section>

        <section className="companion-chat-shell" aria-labelledby="dx-mobile-chat-title">
          <div className="companion-panel-heading">
            <span className="companion-kicker">Zed chats</span>
            <h2 id="dx-mobile-chat-title">Sessions</h2>
          </div>

          <div className="dx-mobile-chat-list" data-dx-chat-list-endpoint="/api/sessions">
            <button className="companion-session-row is-selected" type="button" data-dx-session-key="pending">
              <span>
                <strong>Pair a desktop</strong>
                <small>No active session</small>
              </span>
              <span
                aria-hidden="true"
                className="companion-icon companion-icon-muted"
                data-dx-icon="status:check"
                data-icon-source="dx-icons"
              ></span>
            </button>
          </div>

          <div
            className="dx-mobile-transcript"
            data-dx-session-messages-endpoint="/api/sessions/:session_key/messages"
            aria-live="polite"
          >
            <p>Messages will appear after pairing.</p>
          </div>

          <form className="dx-mobile-message-form" data-dx-chat-send="/ws/chat">
            <label className="companion-field companion-message-field" htmlFor="dx-mobile-message">
              <span>Message</span>
              <textarea id="dx-mobile-message" name="message" rows={3} placeholder="Ask Friday in Zed"></textarea>
            </label>
            <button className="companion-primary-action" type="submit" disabled aria-disabled="true">
              <span>Send</span>
              <span
                aria-hidden="true"
                className="companion-icon companion-icon-check"
                data-dx-icon="status:check"
                data-icon-source="dx-icons"
              ></span>
            </button>
          </form>
        </section>

        <aside className="companion-status-panel" aria-label="Connection status">
          <div className="companion-status-row">
            <span
              aria-hidden="true"
              className="companion-icon companion-icon-check"
              data-dx-icon="status:check"
              data-icon-source="dx-icons"
            ></span>
            <span>DX Agents gateway</span>
          </div>
          <div className="companion-status-row">
            <span
              aria-hidden="true"
              className="companion-icon companion-icon-check"
              data-dx-icon="status:check"
              data-icon-source="dx-icons"
            ></span>
            <span>ACP session identity</span>
          </div>
          <div className="companion-status-row">
            <span
              aria-hidden="true"
              className="companion-icon companion-icon-check"
              data-dx-icon="status:check"
              data-icon-source="dx-icons"
            ></span>
            <span>Native credential bridge</span>
          </div>
          <button
            className="companion-forget-action"
            type="button"
            data-dx-forget-connection
            data-dx-native-command="dx_mobile_clear_device_credential"
          >
            Forget connection
          </button>
        </aside>

        <div
          className="companion-contract"
          data-dx-contract="dx-mobile-companion"
          data-dx-pairing-contract={dxMobileCompanionContract.pairingContract}
          data-dx-chat-contract={dxMobileCompanionContract.chatContract}
          data-dx-native-runtime={dxMobileCompanionContract.nativeRuntime}
          data-dx-native-shell-receipt={dxMobileCompanionContract.nativeShellReceipt}
          data-dx-pairing-runtime-proven={readiness.pairingRuntimeProven}
          data-dx-zed-chat-runtime-proven={readiness.zedChatRuntimeProven}
          hidden
        />
      </section>
      <script src="/mobile-companion-runtime.js" defer />
    </main>
  );
}

export default function HomePage() {
  return <DxMobileCompanionScreen />;
}
