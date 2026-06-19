import { classes } from "../lib/utils";
import { Icon } from "../components/icons/icon";

export default function Page() {
  return (
    <main className={classes("template-shell", "template-shell--tiny")}>
      <section className="template-panel">
        <Icon name="bolt" />
        <p className="template-eyebrow">DX WWW template</p>
        <h1>Tiny development runtime</h1>
        <p>
          This source tree is static, so auto mode can use the lightweight
          may-minihttp-style responder instead of the Axum stack.
        </p>
      </section>
    </main>
  );
}
