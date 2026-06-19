const launchNav = [
  { label: "Auth", href: "#auth" },
  { label: "Data", href: "#data" },
  { label: "State", href: "#state" },
  { label: "Query", href: "#query" },
  { label: "Forms", href: "#forms" },
  { label: "Payments", href: "#payments" },
  { label: "AI", href: "#ai" },
  { label: "Docs", href: "#docs" },
  { label: "WASM", href: "#wasm" },
] as const;

const launchCommands = [
  { label: "Open receipts", href: "#docs" },
  { label: "Review env boundary", href: "#payments" },
  { label: "Capture launch lead", href: "#forms" },
] as const;

export function LaunchDashboardSidebar({ envCount }: { envCount: number }) {
  return (
    <aside
      className="rounded-md border border-border bg-card p-4"
      data-dx-component="launch-dashboard-sidebar"
      data-dx-editable="navigation"
      data-dx-edit-id="launch.sidebar"
      data-dx-edit-kind="navigation"
      data-dx-edit-ops="update_text_content,update_design_token"
    >
      <div className="flex items-center gap-3">
        <div className="grid h-9 w-9 place-items-center rounded-md border border-border bg-muted text-lg font-semibold">
          d
        </div>
        <div>
          <p className="text-sm font-semibold">DX WWW</p>
          <p className="text-xs text-muted-foreground">Forge launch</p>
        </div>
      </div>
      <nav
        className="mt-6 grid gap-1"
        aria-label="Launch sections"
        data-dx-insert-slot="launch-section-nav"
      >
        {launchNav.map((item) => (
          <a
            key={item.href}
            className="rounded-md px-3 py-2 text-sm text-muted-foreground transition hover:bg-muted hover:text-foreground"
            href={item.href}
            data-dx-content-key={`launch.nav.${item.href.slice(1)}`}
            data-dx-editable-text={item.href.slice(1)}
          >
            {item.label}
          </a>
        ))}
      </nav>
      <div className="mt-6 rounded-md border border-border p-3">
        <p className="text-xs text-muted-foreground">Environment contracts</p>
        <p className="mt-1 text-2xl font-semibold">{envCount}</p>
      </div>
    </aside>
  );
}

export function LaunchCommandBar() {
  return (
    <div
      className="flex flex-wrap gap-2"
      aria-label="Launch commands"
      data-dx-component="launch-command-bar"
      data-dx-insert-slot="launch-command-bar"
    >
      {launchCommands.map((command) => (
        <a
          key={command.href}
          className="rounded-md border px-3 py-2 text-sm font-medium"
          href={command.href}
          data-dx-content-key={`launch.command.${command.href.slice(1)}`}
          data-dx-editable-text={command.href.slice(1)}
        >
          {command.label}
        </a>
      ))}
    </div>
  );
}

export function CapabilityBadge({ label }: { label: string }) {
  return (
    <span className="rounded-md border px-2.5 py-1 text-xs text-muted-foreground">
      {label}
    </span>
  );
}
