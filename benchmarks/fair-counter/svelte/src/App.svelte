<script>
  let count = 0;
  let cardQuery = "";
  let rowQuery = "";
  const path = window.location.pathname;

  const sections = Array.from({ length: 160 }, (_, index) => {
    const number = index + 1;
    const principle = ["routing", "serialization", "styling", "deployment"][number % 4];
    return {
      number,
      principle,
      copy:
        "This block covers cache behavior, payload shape, routing boundaries, and production maintenance for the same generated content.",
    };
  });

  const cards = Array.from({ length: 180 }, (_, index) => {
    const number = index + 1;
    const category = ["auth", "dashboard", "commerce", "editor", "analytics"][number % 5];
    return {
      number,
      category,
      title: `${category} component ${number}`,
      copy: "Editable source-owned component packaging with predictable upgrade metadata.",
      search: `${category} component ${number}`,
    };
  });

  const rows = Array.from({ length: 1200 }, (_, index) => {
    const number = index + 1;
    const plan = ["Enterprise", "Pro", "Team", "Starter"][number % 4];
    const region = ["APAC", "EU", "NA", "LATAM", "MEA"][number % 5];
    const status = number % 9 === 0 ? "Review" : "Healthy";
    return {
      number,
      account: `Account ${number}`,
      plan,
      region,
      status,
      mrr: 400 + (number % 37) * 91,
      risk: (number * 7) % 100,
      search: `account ${number} ${plan} ${region} ${status}`.toLowerCase(),
    };
  });

  $: visibleCards = cardQuery
    ? cards.filter((card) => card.search.includes(cardQuery.toLowerCase()))
    : cards;
  $: visibleRows = rowQuery ? rows.filter((row) => row.search.includes(rowQuery.toLowerCase())) : rows;
</script>

{#if path === "/medium-docs"}
  <main class="bench-shell">
    <header class="bench-hero">
      <div>
        <div class="eyebrow">Medium route</div>
        <h1>Framework documentation system</h1>
      </div>
      <div class="metric-row">
        <div class="metric"><span>Sections</span><b>{sections.length}</b></div>
        <div class="metric"><span>Runtime</span><b>Svelte</b></div>
      </div>
    </header>
    <section class="doc-grid">
      {#each sections as section}
        <article class="doc-section">
          <h2>Binary web principle {section.number}</h2>
          <p>{section.copy}</p>
          <span class="tag">{section.principle}</span>
        </article>
      {/each}
    </section>
  </main>
{:else if path === "/medium-cards"}
  <main class="bench-shell">
    <header class="bench-hero">
      <div>
        <div class="eyebrow">Medium interactive route</div>
        <h1>Component registry catalog</h1>
      </div>
      <div class="metric-row">
        <div class="metric"><span>Cards</span><b>{cards.length}</b></div>
        <div class="metric"><span>Runtime</span><b>Svelte</b></div>
      </div>
    </header>
    <div class="toolbar">
      <input bind:value={cardQuery} type="search" placeholder="Filter registry cards" aria-label="Filter registry cards" />
    </div>
    <section class="card-grid">
      {#each visibleCards as card}
        <article class="card">
          <h2>{card.title}</h2>
          <p>{card.copy}</p>
          <span class="tag">{card.category}</span>
        </article>
      {/each}
    </section>
  </main>
{:else if path === "/big-dashboard"}
  <main class="bench-shell">
    <header class="bench-hero">
      <div>
        <div class="eyebrow">Big interactive route</div>
        <h1>Revenue operations dashboard</h1>
      </div>
      <div class="metric-row">
        <div class="metric"><span>Rows</span><b>{rows.length}</b></div>
        <div class="metric"><span>Runtime</span><b>Svelte</b></div>
      </div>
    </header>
    <div class="toolbar">
      <input bind:value={rowQuery} type="search" placeholder="Filter customers or status" aria-label="Filter dashboard rows" />
    </div>
    <section class="panel">
      <table>
        <thead>
          <tr><th>Account</th><th>Plan</th><th>Region</th><th>Status</th><th>MRR</th><th>Risk</th></tr>
        </thead>
        <tbody>
          {#each visibleRows as row}
            <tr>
              <td>{row.account}</td>
              <td>{row.plan}</td>
              <td>{row.region}</td>
              <td class={row.status === "Review" ? "status-risk" : "status-ok"}>{row.status}</td>
              <td>${row.mrr}</td>
              <td>{row.risk}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </section>
  </main>
{:else}
  <main class="container">
    <h1>Fair Counter</h1>
    <p>Minimal interactive counter for framework payload testing.</p>

    <div class="counter" aria-live="polite" aria-atomic="true">{count}</div>

    <div role="group" aria-label="Counter controls">
      <button on:click={() => count += 1} type="button">Increment</button>
      <button on:click={() => count -= 1} type="button">Decrement</button>
      <button on:click={() => count = 0} type="button">Reset</button>
    </div>

    <div class="status" role="status">
      <strong>Runtime:</strong> <span>Svelte + Vite</span><br />
      <strong>Status:</strong> <span>Active</span>
    </div>
  </main>
{/if}
