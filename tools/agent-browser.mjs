#!/usr/bin/env node
import fs from "node:fs/promises";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const { chromium } = require("playwright");

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const stateDir = path.join(rootDir, "artifacts", "agent-browser");
const statePath = path.join(stateDir, "state.json");

async function readState() {
  try {
    return JSON.parse(await fs.readFile(statePath, "utf8"));
  } catch {
    return { url: null, actions: [], refs: [] };
  }
}

async function writeState(state) {
  await fs.mkdir(stateDir, { recursive: true });
  await fs.writeFile(statePath, JSON.stringify(state, null, 2));
}

function usage() {
  console.log(`agent-browser local CLI

Usage:
  agent-browser open <url>
  agent-browser wait --load networkidle
  agent-browser snapshot -i
  agent-browser click <@ref|selector>
  agent-browser eval <javascript expression>
  agent-browser screenshot [--full] [output.png]
  agent-browser verify <url>
  agent-browser close

PowerShell note: quote refs, for example agent-browser click '@e9'.
`);
}

function assertUrl(state) {
  if (!state.url) {
    throw new Error("No URL is open. Run: agent-browser open <url>");
  }
}

async function withPage(state, fn, options = {}) {
  assertUrl(state);

  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({
    viewport: options.viewport || { width: 1440, height: 1000 },
  });
  const consoleErrors = [];
  const pageErrors = [];

  page.on("console", (message) => {
    if (message.type() === "error") {
      consoleErrors.push(message.text());
    }
  });
  page.on("pageerror", (error) => pageErrors.push(error.message));

  try {
    const response = await page.goto(state.url, { waitUntil: "networkidle" });
    for (const selector of state.actions || []) {
      await page.locator(selector).click({ timeout: 5000 });
      await page.waitForLoadState("networkidle", { timeout: 10000 }).catch(() => {});
    }

    return await fn(page, response, { consoleErrors, pageErrors });
  } finally {
    await browser.close();
  }
}

async function openCommand(args) {
  const url = args[0];
  if (!url) {
    throw new Error("Missing URL.");
  }

  const state = { url, actions: [], refs: [] };
  const result = await withPage(state, async (page, response, diagnostics) => ({
    status: response?.status() ?? null,
    title: await page.title(),
    url: page.url(),
    diagnostics,
  }));

  state.url = result.url;
  await writeState(state);
  console.log(JSON.stringify(result, null, 2));
}

async function waitCommand(args) {
  const state = await readState();
  const loadState = args[0] === "--load" ? args[1] || "networkidle" : "networkidle";
  const result = await withPage(state, async (page, response, diagnostics) => {
    await page.waitForLoadState(loadState, { timeout: 15000 });
    return {
      status: response?.status() ?? null,
      loadState,
      title: await page.title(),
      url: page.url(),
      diagnostics,
    };
  });
  console.log(JSON.stringify(result, null, 2));
}

async function snapshotCommand() {
  const state = await readState();
  const result = await withPage(state, async (page) => {
    const refs = await page
      .locator("a, button, input, textarea, select, [role='tab'], [role='button']")
      .evaluateAll((elements) => {
        function cssEscape(value) {
          if (window.CSS && typeof window.CSS.escape === "function") {
            return window.CSS.escape(value);
          }

          return String(value).replace(/[^a-zA-Z0-9_-]/g, "\\$&");
        }

        function selectorForElement(element, index) {
          if (element.id) {
            return `#${cssEscape(element.id)}`;
          }

          const testId = element.getAttribute("data-testid");
          if (testId) {
            return `[data-testid="${cssEscape(testId)}"]`;
          }

          const tag = element.tagName.toLowerCase();
          const text = (element.innerText || element.textContent || "").trim();
          if ((tag === "button" || tag === "a") && text) {
            return `${tag}:has-text("${text.replaceAll('"', '\\"')}")`;
          }

          return `${tag}:nth-of-type(${index + 1})`;
        }

        return elements
          .filter((element) => {
            const rect = element.getBoundingClientRect();
            return rect.width > 0 && rect.height > 0;
          })
          .map((element, index) => ({
            ref: `@e${index + 1}`,
            role:
              element.getAttribute("role") ||
              (element.tagName.toLowerCase() === "a" ? "link" : element.tagName.toLowerCase()),
            name: (element.getAttribute("aria-label") || element.innerText || element.value || "")
              .trim()
              .replace(/\s+/g, " "),
            selector: selectorForElement(element, index),
          }));
      });

    return { refs, title: await page.title(), url: page.url() };
  });

  state.refs = result.refs;
  await writeState(state);
  for (const item of result.refs) {
    console.log(`${item.ref} ${item.role} "${item.name}" ${item.selector}`);
  }
}

async function clickCommand(args) {
  const target = args[0];
  if (!target) {
    throw new Error("Missing click target.");
  }

  const state = await readState();
  const ref = (state.refs || []).find((item) => item.ref === target);
  const selector = ref ? ref.selector : target;
  const result = await withPage(state, async (page, response, diagnostics) => {
    await page.locator(selector).click({ timeout: 5000 });
    await page.waitForLoadState("networkidle", { timeout: 10000 }).catch(() => {});
    return {
      clicked: selector,
      status: response?.status() ?? null,
      title: await page.title(),
      url: page.url(),
      diagnostics,
    };
  });

  state.url = result.url;
  state.actions = [...(state.actions || []), selector];
  await writeState(state);
  console.log(JSON.stringify(result, null, 2));
}

async function evalCommand(args) {
  const source = args.join(" ");
  if (!source) {
    throw new Error("Missing JavaScript expression.");
  }

  const state = await readState();
  const result = await withPage(state, async (page, response, diagnostics) => {
    const value = await page.evaluate((code) => {
      return window.eval(code);
    }, source);

    return {
      status: response?.status() ?? null,
      value,
      diagnostics,
    };
  });
  console.log(JSON.stringify(result, null, 2));
}

async function screenshotCommand(args) {
  const state = await readState();
  const fullPage = args.includes("--full");
  const explicitPath = args.find((arg) => !arg.startsWith("--"));
  const outputPath = path.resolve(rootDir, explicitPath || "artifacts/agent-browser/screenshot.png");

  const result = await withPage(state, async (page) => {
    await fs.mkdir(path.dirname(outputPath), { recursive: true });
    await page.screenshot({ path: outputPath, fullPage });
    return { screenshot: outputPath, fullPage };
  });
  console.log(JSON.stringify(result, null, 2));
}

async function verifyCommand(args) {
  const url = args[0];
  if (!url) {
    throw new Error("Missing URL.");
  }

  const outputDesktop = path.join(rootDir, "artifacts", "agent-browser", "dx-www-desktop.png");
  const outputMobile = path.join(rootDir, "artifacts", "agent-browser", "dx-www-mobile.png");
  const results = [];

  for (const target of [
    { name: "desktop", viewport: { width: 1440, height: 1000 }, screenshot: outputDesktop },
    { name: "mobile", viewport: { width: 390, height: 844 }, screenshot: outputMobile },
  ]) {
    const state = { url, actions: [], refs: [] };
    const result = await withPage(
      state,
      async (page, response, diagnostics) => {
        await page.getByText("Binary web workbench", { exact: true }).waitFor({ timeout: 5000 });
        const styleStatus = page.locator("#style-status");
        const routeStatus = page.locator("#route-status");
        await styleStatus.waitFor({ timeout: 5000 });
        await routeStatus.waitFor({ timeout: 5000 });

        const styleStatusText = await styleStatus.textContent();
        const routeStatusText = await routeStatus.textContent();
        if (styleStatusText !== "ready") {
          throw new Error(`Style status was ${styleStatusText}`);
        }
        if (routeStatusText !== "tiny route online") {
          throw new Error(`Route status was ${routeStatusText}`);
        }

        const dashboard = page.getByRole("tab", { name: "Dashboard", exact: true });
        const dashboardCount = await dashboard.count();
        if (dashboardCount !== 1) {
          throw new Error(`Dashboard tab count was ${dashboardCount}`);
        }

        await dashboard.click({ timeout: 5000 });
        const runtimeBadge = page.locator("[data-runtime]");
        await runtimeBadge.waitFor({ timeout: 5000 });
        const runtimeText = await runtimeBadge.textContent();
        if (runtimeText !== "WASM by merit") {
          throw new Error(`Runtime badge was ${runtimeText}`);
        }
        await fs.mkdir(path.dirname(target.screenshot), { recursive: true });
        await page.screenshot({ path: target.screenshot, fullPage: true });

        return {
          viewport: target.name,
          status: response?.status() ?? null,
          title: await page.title(),
          hasWorkbench: true,
          hasStyleReady: true,
          hasTinyRouteOnline: true,
          dashboardCount,
          runtimeText,
          hasDashboardRuntime: true,
          screenshot: target.screenshot,
          consoleErrors: diagnostics.consoleErrors,
          pageErrors: diagnostics.pageErrors,
        };
      },
      { viewport: target.viewport }
    );
    results.push(result);
  }

  await writeState({ url, actions: [], refs: [] });

  const failed = results.some(
    (result) =>
      result.status !== 200 ||
      result.consoleErrors.length > 0 ||
      result.pageErrors.length > 0 ||
      !result.hasWorkbench ||
      !result.hasStyleReady ||
      !result.hasTinyRouteOnline ||
      !result.hasDashboardRuntime
  );

  console.log(JSON.stringify(results, null, 2));
  if (failed) {
    process.exitCode = 1;
  }
}

async function closeCommand() {
  await writeState({ url: null, actions: [], refs: [] });
  console.log("closed");
}

async function main() {
  const [command, ...args] = process.argv.slice(2);
  switch (command) {
    case "open":
      return openCommand(args);
    case "wait":
      return waitCommand(args);
    case "snapshot":
      return snapshotCommand(args);
    case "click":
      return clickCommand(args);
    case "eval":
      return evalCommand(args);
    case "screenshot":
      return screenshotCommand(args);
    case "verify":
      return verifyCommand(args);
    case "close":
      return closeCommand();
    case "--help":
    case "-h":
    case undefined:
      return usage();
    default:
      throw new Error(`Unknown command: ${command}`);
  }
}

main().catch((error) => {
  console.error(error.message || error);
  process.exit(1);
});
