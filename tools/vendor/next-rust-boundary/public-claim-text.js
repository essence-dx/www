const fs = require("node:fs");
const path = require("node:path");
const { readText } = require("./paths.js");

const PUBLIC_CLAIM_TEXT_PATHS = [
  "vendor/next-rust/README.md",
  "docs/next-rust-merge-checkpoint.md",
  "DX.md",
  "TODO.md",
  "CHANGELOG.md",
];

const FORBIDDEN_PUBLIC_CLAIMS = [
  {
    claim: "dx-www-next-renamed",
    pattern: /\bDX-WWW\s+is\s+Next(?:\.js)?\s+renamed\b/i,
    reason: "DX-WWW must not be presented as Next.js renamed",
  },
  {
    claim: "full-next-parity",
    pattern: /\bfull\s+Next(?:\.js)?\s+parity\b|\bNext(?:\.js)?\s+parity\s+(?:is\s+)?(?:complete|done|proven|shipped|ready)\b/i,
    reason: "Full Next.js parity is unproven and must stay explicitly false or blocked",
  },
  {
    claim: "next-runtime-takeover",
    pattern: /\bNext(?:\.js)?\s+runtime\s+(?:takeover|(?:is\s+)?(?:the\s+)?(?:core|foundation|required|authoritative))\b/i,
    reason: "Next.js runtime must not be described as the DX-WWW runtime foundation",
  },
  {
    claim: "react-rsc-core",
    pattern: /\bReact\s*\/\s*RSC\b.*\b(required|core|foundation)\b/i,
    reason: "React/RSC must not be described as a required DX-WWW core model",
  },
  {
    claim: "node-napi-foundation",
    pattern: /\bNode\/NAPI\s+(?:is\s+)?(?:the\s+)?(?:default\s+)?foundation\b|\bNode\b.*\bNAPI\b.*\b(default|foundation|required)\b/i,
    reason: "Node/NAPI must not be described as the default DX-WWW foundation",
  },
  {
    claim: "node-modules-default",
    pattern: /\bnode_modules\s+(?:is\s+)?(?:the\s+)?default\b|\bdefault\s+node_modules\b/i,
    reason: "node_modules must not be described as the default DX-WWW foundation",
  },
  {
    claim: "turbopack-public-architecture",
    pattern: /\bTurbopack\s+(?:is\s+)?(?:the\s+)?public\s+architecture\b|\bpublic\s+Turbopack\s+architecture\b/i,
    reason: "Turbopack must stay a wrapped build-layer reference until public architecture is proven",
  },
  {
    claim: "next-devtools-clone-target",
    pattern:
      /\bNext(?:\.js)?\s+DevTools\b.*\b(?:clone|parity|target|goal|replacement)\b|\b(?:clone|copy|copied|parity)\b.*\bNext(?:\.js)?\s+DevTools\b|\bnext-devtools\b/i,
    reason: "Next.js DevTools clone/parity is out of scope; keep only DX-owned dev feedback",
  },
  {
    claim: "dx-devtools-removed-target",
    pattern:
      /\bDX-WWW\s+DevTools\b|\/_dx\/devtools\b|\bDevTools\/build diagnostics\b|\bsource-safe\s+DevTools\s+code frames\b|\bexternal\s+DevTools\s+runtime\b/i,
    reason: "Removed DevTools clone routes and surfaces must not remain public DX-WWW targets",
    honestBoundaryAllowed: false,
  },
  {
    claim: "turbopack-runtime-build-adoption",
    pattern:
      /\bTurbopack\b.*\b(?:powers|powering|powered|drives|driving|backs|backing|executes|executing)\b.*\b(?:dx\s+)?(?:build|dev)\b|\b(?:dx\s+)?(?:build|dev)\b.*\b(?:powered|driven|backed|executed)\b.*\bTurbopack\b|\breal\s+Turbopack\s+(?:runtime\s*\/\s*build|runtime|build)\s+adoption\b|\bTurbopack\s+(?:runtime\s*\/\s*build|runtime|build)\s+adoption\b.*\b(?:target|goal|scope|in\s+scope)\b/i,
    reason: "Real Turbopack runtime/build adoption is out of scope for DX-WWW; keep reference/provenance only",
  },
  {
    claim: "external-bundler-execution-proof-target",
    pattern:
      /(?:\bTurbopack\b|\bexternal\s+bundler\b)[^\n.]*\b(?:runtime\s+)?(?:execution|adoption)\b[^\n.]*\b(?:proof|remain(?:s)?\s+unclaimed)\b/i,
    reason: "External bundler execution proof targets are removed from DX-WWW scope",
    honestBoundaryAllowed: false,
  },
];

const HONEST_BOUNDARY_LANGUAGE =
  /\b(not|no|false|blocked|remove(?:d|s|ing)?|unclaimed|unproven|unimplemented|unsupported|out of scope|outside scope|out of (?:the )?core path|without|does not claim|do not claim|not a|remain(?:s)? false|stay(?:s)? false|stays visibly false)\b/i;

function filePath(repoRoot, relativePath) {
  return path.join(repoRoot, ...relativePath.split("/"));
}

function publicClaimFindings(relativePath, text) {
  const lines = text.split(/\r?\n/);
  return lines.flatMap((line, index) => {
    const localContext = [lines[index - 1], line, lines[index + 1]].filter(Boolean).join(" ");
    const hasHonestBoundary = HONEST_BOUNDARY_LANGUAGE.test(localContext);
    return FORBIDDEN_PUBLIC_CLAIMS.filter(({ pattern, honestBoundaryAllowed = true }) => {
      if (honestBoundaryAllowed && hasHonestBoundary) {
        return false;
      }
      return pattern.test(line);
    }).map(
      ({ claim, reason }) => ({
        file: relativePath,
        line: index + 1,
        claim,
        reason,
        text: line.trim(),
      }),
    );
  });
}

function collectPublicClaimText(repoRoot) {
  const checkedFiles = [];
  const forbiddenClaims = [];

  for (const relativePath of PUBLIC_CLAIM_TEXT_PATHS) {
    const absolutePath = filePath(repoRoot, relativePath);
    if (!fs.existsSync(absolutePath)) {
      continue;
    }
    checkedFiles.push(relativePath);
    forbiddenClaims.push(...publicClaimFindings(relativePath, readText(absolutePath)));
  }

  return {
    checkedFiles,
    forbiddenClaims,
  };
}

module.exports = {
  collectPublicClaimText,
};
