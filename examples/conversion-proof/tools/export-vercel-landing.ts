const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const sourcePath = path.join(root, "pages", "index.html");
const outputDir = path.join(root, ".dx", "vercel-landing");

function copyDirectory(source, target) {
  if (!fs.existsSync(source)) {
    return;
  }

  fs.mkdirSync(target, { recursive: true });

  for (const entry of fs.readdirSync(source, { withFileTypes: true })) {
    const sourceEntry = path.join(source, entry.name);
    const targetEntry = path.join(target, entry.name);

    if (entry.isDirectory()) {
      copyDirectory(sourceEntry, targetEntry);
    } else if (entry.isFile()) {
      fs.copyFileSync(sourceEntry, targetEntry);
    }
  }
}

function readLandingBody() {
  const source = fs.readFileSync(sourcePath, "utf8");
  const template = source.match(/<template>\s*([\s\S]*?)\s*<\/template>/);

  if (!template) {
    throw new Error(`Missing <template> wrapper in ${sourcePath}`);
  }

  return processDxIcons(template[1].replace(
    /\s*<link rel="stylesheet" href="\/styles\/dx-landing.css(?:\?v=[^"]+)?">\s*/,
    "\n",
  ));
}

function extractAttr(attrs, name) {
  const match = attrs.match(new RegExp(`${name}="([^"]*)"`));
  return match ? match[1] : undefined;
}

function processDxIcons(html) {
  const paths = {
    android: "M7 8h10v9a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2V8zm2-4-2-2m8 2 2-2M5 9v7m14-7v7m-9-5h.01M14 11h.01",
    apple: "M16.5 13.5c-.8 2.35-2.38 4.5-4.08 4.5-.8 0-1.24-.36-2.08-.36-.88 0-1.4.36-2.14.36-1.72 0-3.7-3.2-3.7-6.12 0-2.46 1.55-4.18 3.48-4.18.94 0 1.62.44 2.18.44.54 0 1.42-.52 2.54-.44.48.02 1.86.2 2.76 1.48-2.42 1.46-2.02 4.06 1.04 4.32zM12.7 4c.54-.66.92-1.58.82-2.5-.78.04-1.72.52-2.28 1.18-.5.58-.94 1.52-.82 2.42.86.08 1.74-.44 2.28-1.1z",
    binary: "M10 4H6v16h4V4zm8 6h-4v10h4V10z",
    firefoxbrowser: "M7.2 5.1C9.2 3.5 12.2 3 14.9 4.2c-1.1.2-1.7.7-2.1 1.3 2.6-.1 5 1.6 5.7 4.1.8 2.8-.4 5.9-2.9 7.4-3.1 1.9-7.2.9-9.1-2.2-1.7-2.8-.9-6.4 1.8-8.3-.6-.2-1-.7-1.1-1.4z",
    database: "M12 2C6.48 2 2 4.02 2 6.5V17.5C2 19.98 6.48 22 12 22S22 19.98 22 17.5V6.5C22 4.02 17.52 2 12 2zM12 4C16.42 4 20 5.35 20 6.5S16.42 9 12 9 4 7.65 4 6.5 7.58 4 12 4zM4 17.5V14.87C5.68 15.94 8.63 16.5 12 16.5S18.32 15.94 20 14.87V17.5C20 18.65 16.42 20 12 20S4 18.65 4 17.5z",
    globe: "M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z M2 12h20 M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z",
    googlechrome: "M21 12a9 9 0 0 1-13.5 7.8l4.5-7.8h9zM3 12a9 9 0 0 1 4.5-7.8l4.5 7.8-4.5 7.8A8.98 8.98 0 0 1 3 12zm4.5-7.8A9 9 0 0 1 21 12h-9L7.5 4.2zM12 8a4 4 0 1 1 0 8 4 4 0 0 1 0-8z",
    layers: "M12 4 4 8l8 4 8-4-8-4zM4 12l8 4 8-4M4 16l8 4 8-4",
    linux: "M12 3c2.2 0 4 2.3 4 5.2 0 1.1.2 2 .8 2.8 1 1.2 1.7 2.8 1.7 4.6 0 2.2-2.1 3.4-6.5 3.4s-6.5-1.2-6.5-3.4c0-1.8.7-3.4 1.7-4.6.6-.8.8-1.7.8-2.8C8 5.3 9.8 3 12 3zm-1.2 5.4h.01M13.2 8.4h.01M9.5 13.5c1.2.8 3.8.8 5 0",
    monitor: "M4 5h16v12H4zM8 21h8M12 17v4",
    palette: "M12 2C6.49 2 2 6.49 2 12s4.49 10 10 10c1.38 0 2.5-1.12 2.5-2.5 0-.61-.23-1.2-.64-1.67-.08-.1-.13-.21-.13-.33 0-.28.22-.5.5-.5H16c3.31 0 6-2.69 6-6 0-4.96-4.49-9-10-9zm-5.5 9c-.83 0-1.5-.67-1.5-1.5S5.67 8 6.5 8 8 8.67 8 9.5 7.33 11 6.5 11zm3-4C8.67 7 8 6.33 8 5.5S8.67 4 9.5 4s1.5.67 1.5 1.5S10.33 7 9.5 7zm5 0c-.83 0-1.5-.67-1.5-1.5S13.67 4 14.5 4s1.5.67 1.5 1.5S15.33 7 14.5 7zm3 4c-.83 0-1.5-.67-1.5-1.5S16.67 8 17.5 8s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
    package: "M12 3 4 7v10l8 4 8-4V7l-8-4zM4 7l8 4 8-4M12 11v10",
    play: "M8 5v14l11-7zM4 5v14",
    rust: "M12 6a6 6 0 1 1 0 12 6 6 0 0 1 0-12zm0-3v3m0 12v3M3 12h3m12 0h3M5.6 5.6l2.1 2.1m8.6 8.6 2.1 2.1m0-12.8-2.1 2.1m-8.6 8.6-2.1 2.1",
    send: "M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z",
    shield: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z",
    sparkles: "M12 3 14.35 8.35 20 10.55 14.35 12.75 12 18 9.65 12.75 4 10.55 9.65 8.35 12 3ZM18.35 15.35 19.15 17.05 21 17.85 19.15 18.65 18.35 20.35 17.55 18.65 15.7 17.85 17.55 17.05 18.35 15.35Z",
    split: "M12 3v18M5 8h14M7 16h10",
    star: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z",
    users: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2M9 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8zm14 10v-2a4 4 0 0 0-3-3.87m-4-12a4 4 0 0 1 0 7.75",
    vercel: "M12 4 22 20H2L12 4z",
    windows: "M3 5.5 10.5 4v7H3V5.5zm8.5-1.7L21 2v9h-9.5V3.8zM3 13h7.5v7L3 18.5V13zm8.5 0H21v9l-9.5-1.8V13z",
    zap: "M13 10V3L4 14h7v7l9-11h-7z",
  };

  return html.replace(/<dx-icon\s+([^>]*)\/>/g, (_, attrs) => {
    const name = extractAttr(attrs, "name") || "zap";
    const className = extractAttr(attrs, "class") || "platform-icon";
    const iconKey = name.includes(":") ? name.split(":").pop() : name;
    const pathData = paths[name] || paths[iconKey] || paths.zap;
    return `<svg class="${className}" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true" data-icon-source="dx-icons" data-dx-icon="${name}"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="${pathData}"></path></svg>`;
  });
}

fs.rmSync(outputDir, { force: true, recursive: true });
fs.mkdirSync(outputDir, { recursive: true });

copyDirectory(path.join(root, "public"), outputDir);
copyDirectory(path.join(root, "styles"), path.join(outputDir, "styles"));

const body = readLandingBody().replaceAll('src="/public/', 'src="/');
const html = `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="color-scheme" content="light dark">
    <meta name="theme-color" content="white" media="(prefers-color-scheme: light)">
    <meta name="theme-color" content="black" media="(prefers-color-scheme: dark)">
    <meta name="description" content="WWW is the Rust-first DX web framework with Style, Forge, Check, local AI, browser automation, and source-owned packages.">
    <meta name="robots" content="index,follow">
    <meta property="og:title" content="WWW - Enhanced Development Experience">
    <meta property="og:description" content="A React-familiar, Rust-powered web framework with Style, Forge, Check, static export, and no hidden dependency surface.">
    <meta property="og:type" content="website">
    <meta name="twitter:card" content="summary_large_image">
    <link rel="canonical" href="https://dx.dev/">
    <link rel="icon" href="/favicon.svg" type="image/svg+xml">
    <link rel="shortcut icon" href="/favicon.svg" type="image/svg+xml">
    <title>WWW - Enhanced Development Experience</title>
    <link rel="stylesheet" href="/styles/dx-landing.css?v=20260521-theme">
  </head>
  <body>
${body}
  </body>
</html>
`;

fs.writeFileSync(path.join(outputDir, "index.html"), html);
fs.writeFileSync(
  path.join(outputDir, "vercel.json"),
  `${JSON.stringify(
    {
      cleanUrls: true,
      trailingSlash: false,
      headers: [
        {
          source: "/(.*)",
          headers: [
            {
              key: "X-DX-Framework",
              value: "www",
            },
          ],
        },
      ],
    },
    null,
    2,
  )}\n`,
);

console.log(`Exported WWW landing to ${outputDir}`);
