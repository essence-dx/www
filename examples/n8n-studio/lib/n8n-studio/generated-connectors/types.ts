import type { CatalogSummary } from "../types";
import type { NodeTypeDescription } from "../node-types/types";

export type GeneratedConnectorOption = {
  name: string;
  value: string;
  action?: string;
};

export type GeneratedConnectorRecord = {
  id: string;
  display_name: string;
  node_name: string;
  source_file: string;
  folder: string;
  categories: string[];
  aliases: string[];
  description: string;
  credential_type_names: string[];
  auth_kinds: string[];
  resources: GeneratedConnectorOption[];
  operations: GeneratedConnectorOption[];
  actions: GeneratedConnectorOption[];
  status: string;
  workflow_node: {
    ready: boolean;
    trigger: boolean;
    usable_as_tool: boolean;
    run_mode: string;
  };
  risk: string;
  source_provenance: string;
};

export type GeneratedConnectorCatalog = {
  schema: string;
  generated_at?: string;
  connectors: GeneratedConnectorRecord[];
};

export type GeneratedConnectorCoverage =
  NonNullable<CatalogSummary["generatedMetadata"]>;

export type GeneratedNodeTypeRegistry = {
  registry: Record<string, NodeTypeDescription>;
  coverage: GeneratedConnectorCoverage;
};
