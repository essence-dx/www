const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

function createFixtureProject() {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-installed-smoke-"));
  writeFile(root, "dx", [
    'project.name="dx-build-installed-smoke"',
    'project.version="0.0.0"',
    'build.output_dir=".dx/build"',
    'build.optimization_level="release"',
    "",
  ].join("\n"));
  writeFile(root, "next.config.mjs", "export default {};\n");
  writeFile(root, "package.json", [
    '{',
    '  "name": "dx-build-installed-smoke",',
    '  "private": true,',
    '  "dependencies": {',
    '    "next": "16.0.0"',
    "  }",
    "}",
    "",
  ].join("\n"));
  writeFile(root, "app/page.tsx", [
    'import { LaunchCard } from "../components/LaunchCard";',
    'import { launchCopy } from "../server/launch-copy";',
    'import { loadLaunchMetrics } from "../server/loaders";',
    "",
    "export default async function Page() {",
    "  const metrics = await loadLaunchMetrics();",
    "  return <main className=\"hero\">",
    '    <img src="/icons/mark.svg" alt="DX" />',
    "    <LaunchCard title={launchCopy.title} routeHandlers={metrics.routeHandlers} />",
    "  </main>;",
    "}",
    "",
  ].join("\n"));
  writeFile(root, "app/layout.tsx", [
    'import "../styles/app.css";',
    "",
    "export default function Layout({ children }) {",
    '  return <html><body>{children}</body></html>;',
    "}",
    "",
  ].join("\n"));
  writeFile(root, "components/LaunchCard.tsx", [
    'import { launchCopy } from "../server/launch-copy";',
    "",
    "export function LaunchCard({ title, routeHandlers }) {",
    '  return <section className="launch-card"><h1>{title}</h1><p>{launchCopy.body}</p><small>{routeHandlers} route handlers</small></section>;',
    "}",
    "",
  ].join("\n"));
  writeFile(root, "server/launch-copy.ts", [
    "export const launchCopy = {",
    '  title: "DX WWW launch smoke",',
    '  body: "Source-owned app, component, server, style, and asset proof.",',
    "};",
    "",
  ].join("\n"));
  writeFile(root, "server/loaders.ts", [
    "export function loadLaunchMetrics() {",
    "  return {",
    "    routeHandlers: 2,",
    '    serverData: "source-owned-safe-interpreter",',
    "  };",
    "}",
    "",
  ].join("\n"));
  writeFile(root, "app/api/health/route.ts", [
    "export function GET() {",
    '  return { status: 200, ok: true, service: "dx-www" };',
    "}",
    "",
  ].join("\n"));
  writeFile(root, "app/api/checkout/route.ts", [
    "export function POST() {",
    '  return { status: 202, ok: true, mode: "dry-run" };',
    "}",
    "",
  ].join("\n"));
  writeFile(root, "styles/app.css", [
    '@import "../tokens/theme.css";',
    ".hero { display: grid; color: var(--dx-accent); }",
    ".launch-card { border: 1px solid currentColor; }",
    ".unused-card { color: red; }",
    "",
  ].join("\n"));
  writeFile(root, "tokens/theme.css", ":root { --dx-accent: rgb(10 20 30); }\n");
  writeFile(
    root,
    "public/icons/mark.svg",
    '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M1 1h8v8H1z"/></svg>\n',
  );
  return root;
}

function createEmptyProject() {
  return fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-help-smoke-"));
}

function writeFile(root, relative, content) {
  const target = path.join(root, relative);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, content);
}

module.exports = {
  createEmptyProject,
  createFixtureProject,
};
