"use client";

import { useMemo, useState } from "react";
import { cards } from "../bench-data";

const allCards = cards();

export function CardCatalog() {
  const [query, setQuery] = useState("");
  const visibleCards = useMemo(() => {
    const normalized = query.trim().toLowerCase();
    if (!normalized) {
      return allCards;
    }
    return allCards.filter((card) => `${card.category} ${card.title}`.includes(normalized));
  }, [query]);

  return (
    <main className="bench-shell">
      <header className="bench-hero">
        <div>
          <div className="eyebrow">Medium interactive route</div>
          <h1>Component registry catalog</h1>
        </div>
        <div className="metric-row">
          <Metric label="Cards" value={allCards.length} />
          <Metric label="Runtime" value="Next.js" />
        </div>
      </header>
      <div className="toolbar">
        <input
          aria-label="Filter registry cards"
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Filter registry cards"
          type="search"
          value={query}
        />
      </div>
      <section className="card-grid">
        {visibleCards.map((card) => (
          <article className="card" key={card.number}>
            <h2>{card.title}</h2>
            <p>{card.copy}</p>
            <span className="tag">{card.category}</span>
          </article>
        ))}
      </section>
    </main>
  );
}

function Metric({ label, value }) {
  return (
    <div className="metric">
      <span>{label}</span>
      <b>{value}</b>
    </div>
  );
}
