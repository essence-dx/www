const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const projectRoot = path.join(repoRoot, "examples", "ui-components");

const expectedComponentSlugs = [
  "accordion",
  "alert-dialog",
  "alert",
  "aspect-ratio",
  "avatar",
  "badge",
  "breadcrumb",
  "button-group",
  "button",
  "calendar",
  "card",
  "carousel",
  "chart",
  "checkbox",
  "collapsible",
  "combobox",
  "command",
  "context-menu",
  "dialog",
  "direction",
  "drawer",
  "dropdown-menu",
  "empty",
  "field",
  "form",
  "hover-card",
  "input-group",
  "input-otp",
  "input",
  "item",
  "kbd",
  "label",
  "menubar",
  "native-select",
  "navigation-menu",
  "pagination",
  "popover",
  "progress",
  "radio-group",
  "resizable",
  "scroll-area",
  "select",
  "separator",
  "sheet",
  "sidebar",
  "skeleton",
  "slider",
  "sonner",
  "spinner",
  "switch",
  "table",
  "tabs",
  "textarea",
  "toggle-group",
  "toggle",
  "tooltip",
];

function read(relativePath) {
  return fs.readFileSync(path.join(projectRoot, relativePath), "utf8");
}

function listFiles(relativeRoot) {
  const root = path.join(projectRoot, relativeRoot);
  const files = [];

  function walk(current) {
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        walk(fullPath);
      } else {
        files.push(path.relative(projectRoot, fullPath).replace(/\\/g, "/"));
      }
    }
  }

  walk(root);
  return files;
}

test("ui-components example is a DX WWW project with style icon forge and check", () => {
  const dx = read("dx");

  assert.match(dx, /project\(name="ui-components"/);
  assert.match(dx, /www\(/);
  assert.match(dx, /style\(/);
  assert.match(dx, /icons\(component=Icon source_tag=icon runtime_tag=dx-icon generated_dir=components\/icons\)/);
  assert.match(dx, /forge\(policy=forge-first-no-node-modules\)/);
  assert.match(dx, /check\(score_scale=500 lighthouse=true\)/);
  assert.ok(!fs.existsSync(path.join(projectRoot, "node_modules")));
});

test("ui-components records real shadcn-ui provenance and registry scope", () => {
  const provenance = read("lib/ui-components/provenance.ts");

  assert.match(provenance, /https:\/\/github\.com\/shadcn-ui\/ui\.git/);
  assert.match(provenance, /cd54e0927f3853a777f700a0bbf34507cf697b9c/);
  assert.match(provenance, /registryComponentCount: 56/);
  assert.match(provenance, /DX Charts package, not npm Recharts/);
  assert.match(provenance, /DX Carousel package, not npm Embla/);
});

test("ui-components represents the full current shadcn registry surface", () => {
  const uiFiles = listFiles("components/ui")
    .filter((file) => file.endsWith(".tsx"))
    .map((file) => path.basename(file, ".tsx"))
    .filter((slug) => !["adapter-boundary", "slot", "types"].includes(slug))
    .sort();

  assert.deepEqual(uiFiles, expectedComponentSlugs.slice().sort());

  const catalog = read("lib/ui-components/catalog.ts");
  const index = read("components/ui/index.ts");

  assert.match(catalog, /represented: registryComponents\.length/);
  assert.match(catalog, /adapterBoundaryComponents/);

  for (const slug of expectedComponentSlugs) {
    assert.match(catalog, new RegExp(`shadcn/ui/\\$\\{slug\\}|${slug}\\.tsx|\"${slug}\"`));
    assert.match(index, new RegExp(`export \\* from \"\\./${slug}\"`), `${slug} is exported`);
  }
});

test("ui-components source avoids npm runtime imports", () => {
  const forbidden = [
    "from \"react\"",
    "from 'react'",
    "from \"@radix-ui",
    "from '@radix-ui",
    "from \"lucide-react\"",
    "from 'lucide-react'",
    "from \"class-variance-authority\"",
    "from 'class-variance-authority'",
    "from \"tailwind-merge\"",
    "from 'tailwind-merge'",
    "from \"clsx\"",
    "from 'clsx'",
    "from \"recharts\"",
    "from 'recharts'",
    "from \"embla-carousel",
    "from 'embla-carousel",
    "from \"cmdk\"",
    "from 'cmdk'",
    "from \"sonner\"",
    "from 'sonner'",
    "next/",
    "node_modules",
  ];

  for (const file of listFiles(".")) {
    if (file.startsWith(".dx/") || !/\.(ts|tsx)$/.test(file)) {
      continue;
    }

    const source = read(file);
    for (const marker of forbidden) {
      assert.doesNotMatch(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `${file} contains ${marker}`);
    }
  }
});

test("ui-components uses source-owned primitives and DX icons in app routes", () => {
  const page = read("app/page.tsx");

  for (const marker of [
    "components/home/ui-components-home",
  ]) {
    assert.match(page, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  const home = read("components/home/ui-components-home.tsx");
  for (const marker of [
    "../ui/badge",
    "../ui/button",
    "../ui/card",
    "../gallery/primitive-gallery",
    "../site/registry-map",
    "data-dx-source=\"shadcn-ui\"",
  ]) {
    assert.match(home, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  const header = read("components/site/site-header.tsx");
  assert.match(header, /<Icon name="pack:check"/);

  const routes = [
    "app/docs/components/page.tsx",
    "app/docs/components/primitives/page.tsx",
    "app/registry/page.tsx",
  ];
  for (const route of routes) {
    assert.ok(fs.existsSync(path.join(projectRoot, route)), `${route} exists`);
    assert.match(read(route), /data-dx-route=/);
  }
});

test("ui-components has Forge receipts for the baseline source-owned packages", () => {
  const manifest = JSON.parse(read(".dx/forge/source-.dx/build-cache/manifest.json"));
  const packageIds = manifest.packages.map((item) => item.package_id);

  for (const packageId of [
    "shadcn/ui/button",
    "shadcn/ui/badge",
    "shadcn/ui/card",
    "shadcn/ui/label",
    "shadcn/ui/separator",
    "shadcn/ui/field",
    "shadcn/ui/item",
    "shadcn/ui/input",
    "shadcn/ui/textarea",
    "dx/icon/search",
  ]) {
    assert.ok(packageIds.includes(packageId), `${packageId} is in Forge source manifest`);
  }
});
