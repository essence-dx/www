import { classes } from "../lib/utils";
import { Icon } from "../components/icons/icon";

export default function Page() {
  return (
    <main className={classes("template-shell", "template-shell--axum")}>
      <section className="template-panel">
        <Icon name="server" />
        <p className="template-eyebrow">DX WWW template</p>
        <h1>Axum development runtime</h1>
        <p>
          Hot reload, devtools, and route handlers are enabled, so auto mode selects
          the full Axum stack.
        </p>
      </section>
    </main>
  );
}
