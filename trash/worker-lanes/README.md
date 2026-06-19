# DX-WWW Worker Lane And Pass Counter

This folder owns the local lane/pass counter for parallel DX-WWW worker chats.

Before changing code, docs, tests, examples, receipts, or generated artifacts,
every worker must read `G:\Dx\www\AGENTS.md` and `G:\Dx\www\.cursorrules`.
`AGENTS.md` is the canonical repository contract. Tool rules, lane prompts,
and status notes are only bridges back to that file.

Use `README.md` for the public overview, `docs/dx-www-developer-contract.md`
for project shape and authoring rules, `docs/api/versioning.md` for public API
stability, and current code, receipts, and command output for numeric or
release claims. Treat `DX.md` as historical launch notes unless a task
explicitly asks for it.

## Current WWW Contract For Workers

- WWW is source-owned, Rust-backed, React/Next-familiar authoring.
- Framework-owned global state uses `store({ state, derived, effect, action })`;
  the Forge package lane named `State Management` is upstream Zustand
  provenance, not the internal WWW runtime.
- Quoted event values such as `onClick="bg-red-500 scale-up"` are
  interaction-class commands; braced event values must lower safely or produce
  a diagnostic.
- DX Style owns `className`, grouped syntax, motion/event classes, generated
  CSS, and the atomic/custom CSS balance. Do not hand-edit generated CSS.
- DX Icon is first-party. Use `<Icon name="pack:check" />` through
  `icons(component=Icon source_tag=icon runtime_tag=dx-icon ...)`; do not
  import npm icon packages in official starters.
- Use focused checks first. Run broad Cargo, `dx build`, dev servers, browser
  automation, deploys, or package installs only when the lane explicitly owns
  that proof or the manager has approved it.
- Keep evidence honest. Do not repeat old scores, branch names, process IDs,
  server URLs, or proof claims unless verified in the current checkout.

Each new worker should run this before doing any repo work:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1
```

The first run claims a lane and assigns pass 1:

```text
AGENT_NUMBER: 1
PASS_NUMBER: 1
WORKER_ID: www-...
```

For pass 2 and pass 3 in the same chat, run the `Next pass command` printed by
the script. It reuses the same lane and increments the pass number:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1 -WorkerId <WORKER_ID>
```

For the final 7-agent / 5-pass closeout round, use the separate scope and
prompt:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1 -MaxLanes 7 -MaxPasses 5 -Scope www-7-agent-5-pass-closeout
```

Prompt:

```text
G:\Dx\www\worker-lanes\WWW_7_AGENT_5_PASS_CLOSEOUT_PROMPT.md
```

Useful manager commands:

```powershell
# See the current counter without claiming a lane.
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1 -Peek

# Reset to zero for a fresh 30-worker round. This archives the old claims log.
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1 -Reset

# Machine-readable output for tooling.
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1 -Json
```

State files live in `G:\Dx\www\worker-lanes\state`:

- `www-30-agent.counter.txt`: current claimed lane number
- `www-30-agent.claims.jsonl`: append-only lane/pass claim log
- `www-30-agent.workers\*.json`: per-worker lane/pass state
- `www-30-agent.lock`: exclusive lock file used to avoid duplicate lane claims
