import { docSections } from "../bench-data";

export const metadata = {
  title: "Next Medium Docs Benchmark",
};

export default function MediumDocsPage() {
  const sections = docSections();

  return (
    <main className="bench-shell">
      <header className="bench-hero">
        <div>
          <div className="eyebrow">Medium route</div>
          <h1>Framework documentation system</h1>
        </div>
        <div className="metric-row">
          <Metric label="Sections" value={sections.length} />
          <Metric label="Runtime" value="Next.js" />
        </div>
      </header>
      <section className="doc-grid">
        {sections.map((section) => (
          <article className="doc-section" key={section.number}>
            <h2>Binary web principle {section.number}</h2>
            <p>{section.copy}</p>
            <span className="tag">{section.principle}</span>
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
