import type { NodeParameterDefinition, NodeTypeDescription } from "../node-types/types";

export type SourceParameterExtraction = {
  sourcePath: string;
  generatedFrom: "source-parameter-description";
  parameters: NodeParameterDefinition[];
  unsupportedParameterNames: string[];
};

export type SourceNodeTypeFactory = (source: string) => NodeTypeDescription;
