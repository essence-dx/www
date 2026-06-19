import { Counter } from "./counter";

export default function Page() {
  return (
    <main className="container">
      <h1>Fair Counter</h1>
      <p>Minimal interactive counter for framework payload testing.</p>
      <Counter />
      <div className="status" role="status">
        <strong>Runtime:</strong> <span>Next.js App Router + React</span>
        <br />
        <strong>Status:</strong> <span>Active</span>
      </div>
    </main>
  );
}
