"use client";

import { useMemo, useState } from "react";
import { dashboardRows } from "../bench-data";

const allRows = dashboardRows();

export function RevenueDashboard() {
  const [query, setQuery] = useState("");
  const visibleRows = useMemo(() => {
    const normalized = query.trim().toLowerCase();
    if (!normalized) {
      return allRows;
    }
    return allRows.filter((row) => row.search.includes(normalized));
  }, [query]);

  return (
    <main className="bench-shell">
      <header className="bench-hero">
        <div>
          <div className="eyebrow">Big interactive route</div>
          <h1>Revenue operations dashboard</h1>
        </div>
        <div className="metric-row">
          <Metric label="Rows" value={allRows.length} />
          <Metric label="Runtime" value="Next.js" />
        </div>
      </header>
      <div className="toolbar">
        <input
          aria-label="Filter dashboard rows"
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Filter customers or status"
          type="search"
          value={query}
        />
      </div>
      <section className="panel">
        <table>
          <thead>
            <tr>
              <th>Account</th>
              <th>Plan</th>
              <th>Region</th>
              <th>Status</th>
              <th>MRR</th>
              <th>Risk</th>
            </tr>
          </thead>
          <tbody>
            {visibleRows.map((row) => (
              <tr key={row.number}>
                <td>{row.account}</td>
                <td>{row.plan}</td>
                <td>{row.region}</td>
                <td className={row.status === "Review" ? "status-risk" : "status-ok"}>{row.status}</td>
                <td>${row.mrr}</td>
                <td>{row.risk}</td>
              </tr>
            ))}
          </tbody>
        </table>
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
