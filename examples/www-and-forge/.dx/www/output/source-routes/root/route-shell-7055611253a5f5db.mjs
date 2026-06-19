import { dxSourceModule as dxRouteEntryModule } from "./modules/app-page-tsx-d3203b887a53055d.mjs";
const fallbackHtml = "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><link rel=\"icon\" href=\"/public/favicon.svg\" type=\"image/svg+xml\"><link rel=\"apple-touch-icon\" href=\"/public/icon.svg\"><meta name=\"description\" content=\"Orchestrate your code, don't just own it.\"><link rel=\"stylesheet\" href=\"/styles/generated.css\"><link rel=\"stylesheet\" href=\"/styles/globals.css\"><link rel=\"stylesheet\" href=\"/styles/theme.css\"><link rel=\"stylesheet\" href=\"/_dx/styles/react-route-ce40f27a48d53008.css\" data-dx-generated=\"true\"><title>Enhanced Development Experience</title></head><body><main class=\"starter-shell forge-backed-shell\" data-dx-runtime=\"static\" data-dx-output-mode=\"tiny-static\" data-dx-js=\"none\" data-dx-route=\"/\" data-dx-assets=\"/logo.svg\" data-forge-package=\"npm/lodash\"><section class=\"starter-card source-owned-forge-package\" aria-labelledby=\"starter-title\"><img class=\"starter-logo\" src=\"/logo.svg\" alt=\"Dx WWW\"><p class=\"starter-kicker\">Dx WWW</p><h1 id=\"starter-title\">Enhanced Development Experience</h1><p class=\"starter-copy\">Orchestrate your code, don't just own it.</p><p class=\"starter-proof\" data-forge-helper=\"startCase\">Source-owned lodash proof.</p></section></main></body></html>";

export const dxRouteShell = Object.freeze({
  route: "/",
  fallbackHash: "7055611253a5f5db",
  fullReactHydration: false,
  nodeModulesRequired: false,
  sourceModuleEntry: dxRouteEntryModule
});

export function mount(target) {
  const container = typeof target === "string" ? document.querySelector(target) : target;
  if (!container) {
    throw new Error(`DX route shell target not found for ${dxRouteShell.route}`);
  }
  container.innerHTML = fallbackHtml;
  return dxRouteShell;
}

export default dxRouteShell;
