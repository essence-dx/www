"use client";

import { useMemo, useState } from "react";

import { createNodeCreatorState } from "../../lib/n8n-studio/node-creator-actions";
import { n8nNodeTypeRegistry } from "../../lib/n8n-studio/node-type-registry";
import type { NodeTypeDescription } from "../../lib/n8n-studio/node-types/types";
import type { CatalogSummary } from "../../lib/n8n-studio/types";
import { Icon } from "../icons/icon";

export type NodeCatalogPanelProps = {
  catalog: CatalogSummary;
  nodeTypeRegistry?: Record<string, NodeTypeDescription>;
  onAddCatalogNode?: (catalogNodeId: string) => void;
};

export function NodeCatalogPanel({
  catalog,
  nodeTypeRegistry = n8nNodeTypeRegistry,
  onAddCatalogNode,
}: NodeCatalogPanelProps) {
  const [query, setQuery] = useState("");
  const creator = useMemo(
    () => createNodeCreatorState(catalog, query, nodeTypeRegistry),
    [catalog, nodeTypeRegistry, query],
  );

  return (
    <aside
      className="n8ns-catalog-panel"
      aria-label="Node creator and connector catalog"
      data-studio-surface="node-creator"
    >
      <div className="n8ns-panel-header">
        <div>
          <p className="n8ns-eyebrow">Node creator</p>
          <h2>Catalog</h2>
        </div>
        <span className="n8ns-count">{creator.results.length}</span>
      </div>
      <label className="n8ns-search">
        <Icon className="n8ns-icon" name="n8n-studio:search" />
        <input
          aria-label="Search nodes"
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Search connectors"
          value={query}
        />
      </label>
      <div className="n8ns-catalog-stats" aria-label="Source manifest counts">
        <span>{catalog.nodeFolderCount} folders</span>
        <span>{catalog.nodeFileCount} node files</span>
        <span>{catalog.credentialFileCount} credentials</span>
        <span>{creator.addableCount} ready</span>
      </div>
      <div className="n8ns-node-list">
        {creator.results.map(({ addable, node, reason }) => (
          <article
            className="n8ns-node-card"
            data-node-addable={String(addable)}
            data-node-role={node.role}
            key={node.id}
          >
            <div>
              <strong>{node.displayName}</strong>
              <span>{node.category}</span>
            </div>
            <p>{node.description}</p>
            <div className="n8ns-node-card-footer">
              <small>{node.sourcePath}</small>
              <button
                data-node-creator-add={node.id}
                disabled={!addable || !onAddCatalogNode}
                onClick={() => onAddCatalogNode?.(node.id)}
                title={reason}
                type="button"
              >
                <Icon className="n8ns-icon" name="n8n-studio:plus" />
                Add
              </button>
            </div>
          </article>
        ))}
        {creator.results.length === 0 ? (
          <p className="n8ns-muted">No connector matches this search.</p>
        ) : null}
      </div>
    </aside>
  );
}
