import { Icon } from "../icons/icon";

const concepts = [
  { title: "Package parity", text: "Each AntV reference clone is mapped to a DX-owned status, source proof, interaction proof, and task-linked gallery surface before runtime claims are made." },
  { title: "Spec", text: "A chart is a typed object with data, marks, encodings, axes, legend, theme, and dimensions." },
  { title: "Data", text: "Rows stay as plain records. Field access, formatting, and null handling are owned by DX code." },
  { title: "Transform", text: "Filter, sort, group, bin, stackY, normalizeY, and dodgeX run before mark compilation so presets can consume raw rows." },
  { title: "Preset", text: "G2Plot-style configs lower into the same typed grammar through local adapter functions." },
  { title: "Scale", text: "Linear, band, point, and ordinal color scales map data values into plot coordinates and tokens." },
  { title: "Mark", text: "Bars, lines, areas, points, rules, heatmaps, pies, radar, gauges, boxplots, bullets, sunbursts, graphs, maps, and table cells compile into SVG scene nodes." },
  { title: "Composition", text: "Facet composition splits one spec into small compiled sub-scenes without npm chart runtimes." },
  { title: "Scene", text: "The compiler emits stable scene nodes with sanitized ids, labels, legend items, transforms, and accessibility metadata." },
  { title: "Interaction", text: "The public runtime reads chart data attributes, clamps tooltips, and supports pointer plus keyboard selection." },
  { title: "Check", text: "DX Style, DX Icon, imports, Forge receipts, and project contract checks remain the verification boundary." },
] as const;

export function DocsOverview() {
  return (
    <div className="charts-page-stack" data-dx-route="docs">
      <header className="charts-page-heading">
        <p className="charts-kicker">Grammar docs</p>
        <h1>Small kernel, serious boundaries.</h1>
        <p className="charts-lead">
          G2’s useful idea is grammar-first composition. DX Charts keeps that idea and expresses it through focused
          modules for data, scales, marks, scenes, interaction, and verification.
        </p>
      </header>
      <section className="charts-doc-grid" aria-label="DX Charts concepts">
        {concepts.map((concept) => (
          <article className="charts-doc-card" key={concept.title}>
            <Icon name="charts:check" />
            <h2>{concept.title}</h2>
            <p>{concept.text}</p>
          </article>
        ))}
      </section>
      <section className="charts-code-panel" aria-label="Package parity status contract">
        <p className="charts-kicker">Package parity</p>
        <pre>
          <code>{`{
  packageName: "@antv/g2plot",
  coverageStatus: "Implemented",
  sourceProof: "Local preset adapters and compiler output.",
  interactionProof: "Keyboard-selectable marks, tooltips, and scene metadata."
}`}</code>
        </pre>
      </section>
      <section className="charts-code-panel" aria-label="Preset adapter spec">
        <p className="charts-kicker">G2Plot adapter</p>
        <pre>
          <code>{`g2plotCartesian({
  id: "stacked-release-work",
  title: "Release work stack",
  description: "Work hours stacked by lane.",
  task: "composition",
  data: releaseWorkMix,
  preset: "column",
  xField: "week",
  yField: "hours",
  seriesField: "lane",
  isStack: true
})`}</code>
        </pre>
      </section>
      <section className="charts-code-panel" aria-label="Facet composition spec">
        <p className="charts-kicker">Facet composition</p>
        <pre>
          <code>{`g2plotFacet({
  id: "faceted-runtime-proof",
  title: "Runtime proof facets",
  description: "Complexity and proof by family.",
  task: "relation",
  data: runtimeProofs,
  preset: "facet",
  field: "family",
  columns: 3,
  child: {
    preset: "scatter",
    xField: "complexity",
    yField: "proof",
    colorField: "surface"
  }
})`}</code>
        </pre>
      </section>
    </div>
  );
}
