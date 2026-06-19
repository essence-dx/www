export const metadata = {
  title: "Enhanced Development Experience",
  description: "Orchestrate your code, don't just own it.",
} as const;

export default function HomePage() {
  return (
    <main className="starter-shell" data-dx-route="/" data-dx-template="minimal">
      <section className="starter-card" aria-labelledby="starter-title">
        <img className="starter-logo" src="/logo.svg" alt="Dx WWW" />
        <p className="starter-kicker">Dx WWW</p>
        <h1 id="starter-title">Enhanced Development Experience</h1>
        <p className="starter-copy">Orchestrate your code, don't just own it.</p>
      </section>
    </main>
  );
}
