"use client";

import * as React from "react";

import { DxMarkdown } from "@/components/content/markdown";
import {
  createFumadocsNavigationReceipt,
  dxFumadocsDashboardPages,
  getDxFumadocsDashboardPage,
  type DxFumadocsDashboardPageId,
} from "@/lib/fumadocs/dashboard-workflow";
import { dxFumadocsLLMsContract } from "@/lib/fumadocs/llms";
import { dxFumadocsOpenAPICodeUsageContract } from "@/lib/fumadocs/openapi-code-usage";
import { dxFumadocsOpenAPIContract } from "@/lib/fumadocs/openapi";
import { dxFumadocsNavigationContract } from "@/lib/fumadocs/navigation";
import { dxFumadocsRouteContract } from "@/lib/fumadocs/route-contract";
import { dxFumadocsSearchClientContract } from "@/lib/fumadocs/search-client";
import { dxFumadocsSourcePluginContract } from "@/lib/fumadocs/source-plugins";
import { dxFumadocsTocContract } from "@/lib/fumadocs/toc";

const launchDocMarkdown: Record<DxFumadocsDashboardPageId, string> = {
  overview: [
    "### Launch overview",
    "",
    "DX materializes a Documentation System route from source-owned files.",
    "",
    "- Page-tree navigation is generated from the route contract.",
    "- Search uses the Documentation System route boundary instead of a hidden package card.",
    "- Markdown is rendered with HTML disabled for the launch readiness workflow.",
  ].join("\n"),
  "api-reference": [
    "### API reference",
    "",
    "The OpenAPI package slice owns generated documentation surfaces, not live credential forwarding.",
    "",
    "- Proxy requests read the allowed-origin environment boundary.",
    "- Code usage includes the DX launch fetch generator.",
    "- Production schemas, auth, and rate limits remain application-owned.",
  ].join("\n"),
  "llm-export": [
    "### LLM handoff",
    "",
    "The Documentation System exposes `llms.txt` style exports for tools that need source context.",
    "",
    "- The index route is visible in the generated launch app.",
    "- Full exports are separated from private content policy.",
    "- DX Studio can target this section with stable `data-dx-*` markers.",
  ].join("\n"),
};

const launchDocsChangelog = [
  "Docs route, OpenAPI, search, and LLM handoff surfaced in one panel.",
  "Safe local route receipt added for Web Preview selection.",
  "OpenAPI proxy remains missing-config until allowed origins are app-owned.",
] as const;

export function LaunchDocsStatus() {
  const [activePageId, setActivePageId] =
    React.useState<DxFumadocsDashboardPageId>(dxFumadocsDashboardPages[0].id);
  const [receipt, setReceipt] = React.useState<ReturnType<
    typeof createFumadocsNavigationReceipt
  > | null>(null);
  const activePage = getDxFumadocsDashboardPage(activePageId);
  const activeMarkdown = launchDocMarkdown[activePage.id];

  return (
    <div
      className="grid gap-3"
      data-dx-package="content/fumadocs-next"
      data-dx-component="launch-fumadocs-docs-workflow"
      data-dx-dashboard-workflow="docs-help-changelog"
      data-dx-product-surface="dashboard-help-content"
      data-dx-style-surface="documentation-system"
      data-dx-node-modules="forbidden"
      data-dx-editable="content"
      data-dx-edit-id="launch.docs-help"
      data-dx-edit-kind="dashboard-workflow"
      data-dx-edit-ops="insert_component,move_reorder_section,update_text_content,insert_icon_media"
      data-dx-docs-search-route={dxFumadocsRouteContract.searchRoute}
      data-dx-docs-static-search-route={dxFumadocsRouteContract.staticSearchRoute}
      data-dx-docs-llms-route={dxFumadocsRouteContract.llmsIndexRoute}
      data-dx-docs-llms-full-route={dxFumadocsRouteContract.llmsFullRoute}
      data-dx-docs-navigation={dxFumadocsNavigationContract.surfaces.join(",")}
      data-dx-docs-openapi-code-usage={dxFumadocsOpenAPICodeUsageContract.customGenerators.join(",")}
      data-dx-docs-openapi-proxy={dxFumadocsOpenAPIContract.proxyRoute}
      data-dx-docs-openapi-route={dxFumadocsRouteContract.openApiDocsRoute}
      data-dx-docs-source-plugins={dxFumadocsSourcePluginContract.frontmatterFields.join(",")}
      data-dx-docs-status={dxFumadocsRouteContract.docsRoute}
      data-dx-docs-toc={dxFumadocsTocContract.surfaces.join(",")}
      data-dx-fumadocs-rendered-route={activePage.route}
      data-dx-fumadocs-selected-page={activePage.id}
      data-dx-fumadocs-toc-count={activePage.toc.length}
      data-dx-icon-search="pack:fumadocs"
    >
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="grid gap-2">
          <div className="flex items-center gap-2">
            <dx-icon name="pack:fumadocs" aria-hidden="true" />
            <p
              className="text-sm font-medium"
              data-dx-editable-text="launch-docs-help-title"
            >
              Docs help and changelog
            </p>
          </div>
          <p
            className="max-w-3xl text-xs leading-5 text-muted-foreground"
            data-dx-editable-text="launch-docs-help-copy"
          >
            Documentation System powers the dashboard help panel
            with route selection, OpenAPI readiness, search signals, LLM export
            readiness, and a local receipt action. Content policy, private-doc
            exclusion, runtime search budgets, and the OpenAPI proxy environment
            boundary stay app-owned through{" "}
            {dxFumadocsOpenAPIContract.allowedOriginsEnv}.
          </p>
        </div>
        <div
          className="rounded-md border px-3 py-2 text-xs text-muted-foreground"
          data-dx-fumadocs-missing-config={dxFumadocsOpenAPIContract.allowedOriginsEnv}
        >
          Missing config: {dxFumadocsOpenAPIContract.allowedOriginsEnv}
        </div>
      </div>

      <div
        className="flex flex-wrap gap-2"
        data-dx-fumadocs-interaction="page-tree-selector"
      >
        {dxFumadocsDashboardPages.map((page) => {
          const selected = page.id === activePage.id;

          return (
            <button
              key={page.id}
              className={[
                "rounded-md border px-3 py-2 text-left text-xs transition",
                selected
                  ? "border-foreground bg-foreground text-background"
                  : "border-border bg-background text-muted-foreground hover:text-foreground",
              ].join(" ")}
              type="button"
              onClick={() => {
                setActivePageId(page.id);
                setReceipt(null);
              }}
              data-dx-fumadocs-page-option={page.id}
              data-dx-fumadocs-page-route={page.route}
              data-dx-fumadocs-page-selected={selected ? "true" : "false"}
              aria-pressed={selected}
            >
              <span className="block font-medium">{page.title}</span>
              <span className="block text-[11px] opacity-80">{page.route}</span>
            </button>
          );
        })}
      </div>

      <div className="grid gap-3 lg:grid-cols-[0.74fr_1.26fr]">
        <div
          className="grid content-start gap-3 rounded-md border p-3 text-xs"
          data-dx-fumadocs-navigation-snapshot={activePage.breadcrumb.join("/")}
        >
          <div>
            <p className="font-medium">Page tree</p>
            <p className="mt-1 text-muted-foreground">
              {activePage.description}
            </p>
          </div>
          <div>
            <p className="text-muted-foreground">Breadcrumb</p>
            <ol className="mt-2 flex flex-wrap gap-1">
              {activePage.breadcrumb.map((item) => (
                <li
                  key={item}
                  className="rounded-sm border px-2 py-1"
                  data-dx-fumadocs-breadcrumb={item}
                >
                  {item}
                </li>
              ))}
            </ol>
          </div>
          <div>
            <p className="text-muted-foreground">Peers</p>
            <ul className="mt-2 grid gap-1">
              {activePage.peers.map((peer) => (
                <li key={peer} data-dx-fumadocs-peer={peer}>
                  {peer}
                </li>
              ))}
            </ul>
          </div>
        </div>

        <div
          className="rounded-md border p-3 text-sm"
          data-dx-fumadocs-rendered-markdown="active-page"
        >
          <div className="mb-3 flex flex-wrap items-center justify-between gap-2 text-xs">
            <span className="font-medium">{activePage.title}</span>
            <span
              className="rounded-sm border px-2 py-1 text-muted-foreground"
              data-dx-fumadocs-route-label={activePage.route}
            >
              {activePage.route}
            </span>
          </div>
          <DxMarkdown skipHtml>{activeMarkdown}</DxMarkdown>
        </div>
      </div>

      <div className="grid gap-3 rounded-md border p-3 text-xs">
        <div className="flex flex-wrap items-center justify-between gap-2">
          <p className="font-medium">Documentation System</p>
          <p className="text-muted-foreground">
            {dxFumadocsSearchClientContract.fetchPreset}/
            {dxFumadocsSearchClientContract.staticPreset}
          </p>
        </div>
        <ol
          className="grid gap-1 sm:grid-cols-3"
          data-dx-fumadocs-toc-list={activePage.id}
        >
          {activePage.toc.map((item) => (
            <li key={item} className="rounded-sm border px-2 py-1">
              {item}
            </li>
          ))}
        </ol>
        <div className="grid gap-2 md:grid-cols-[1fr_auto] md:items-end">
          <p className="text-muted-foreground">
            APIs proven here: {dxFumadocsLLMsContract.upstreamApi},{" "}
            {dxFumadocsOpenAPIContract.upstreamApis[0]},{" "}
            {dxFumadocsOpenAPICodeUsageContract.customGenerators[0]},{" "}
            {dxFumadocsSourcePluginContract.frontmatterFields.join(", ")}.
          </p>
          <button
            className="inline-flex items-center gap-2 rounded-md border px-3 py-2 text-sm font-medium"
            type="button"
            data-dx-fumadocs-action="safe-local-route-preview"
            onClick={() =>
              setReceipt(createFumadocsNavigationReceipt({ pageId: activePage.id }))
            }
          >
            <dx-icon name="pack:search" aria-hidden="true" />
            Preview docs receipt
          </button>
        </div>
      </div>

      <div className="grid gap-2 rounded-md border p-3 text-xs">
        <p className="font-medium">Recent docs updates</p>
        <ul className="grid gap-1" data-dx-fumadocs-changelog="launch-docs">
          {launchDocsChangelog.map((item) => (
            <li key={item} data-dx-fumadocs-changelog-item={item}>
              {item}
            </li>
          ))}
        </ul>
      </div>

      <p
        role="status"
        aria-live="polite"
        className="rounded-md border p-3 text-sm text-muted-foreground"
        data-dx-fumadocs-local-response={receipt ? receipt.status : "idle"}
        data-dx-fumadocs-receipt-route={receipt?.route ?? "none"}
      >
        {receipt
          ? `${receipt.packageId} selected ${receipt.route} with ${receipt.tocCount} TOC entries. ${receipt.nextAction}`
          : "Select a docs route, then preview the local Documentation System route receipt."}
      </p>
    </div>
  );
}
