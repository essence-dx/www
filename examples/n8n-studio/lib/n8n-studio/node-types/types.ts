import type { ParameterDisplayOptions, ParameterField, ParameterOption, ResourceLocatorMode } from "../types";

export type NodeTypeDescription = {
  name: string;
  displayName: string;
  sourcePath: string;
  version: number;
  credentials: Array<{ name: string; required: boolean }>;
  properties: NodeParameterDefinition[];
  categories?: string[];
  aliases?: string[];
  authKinds?: string[];
  sourceProvenance?: string;
  workflowNode?: {
    ready: boolean;
    trigger: boolean;
    usable_as_tool: boolean;
    run_mode: string;
  };
};

export type NodeParameterDefinition = {
  name: string;
  label: string;
  type: ParameterField["type"];
  defaultValue: unknown;
  required?: boolean;
  noDataExpression?: boolean;
  description?: string;
  placeholder?: string;
  options?: ParameterOption[];
  credentialTypes?: string[];
  displayOptions?: ParameterDisplayOptions;
  dynamicOptions?: ParameterField["dynamicOptions"];
  resourceMapper?: ParameterField["resourceMapper"];
  resourceLocatorModes?: ResourceLocatorMode[];
  childParameters?: NodeParameterDefinition[];
  renderingBoundary?: "native" | "complex-source-field";
};

export type NodeParameterSchema = {
  nodeType: string;
  fields: ParameterField[];
  dynamicLoadBoundaries: string[];
  hiddenFields: string[];
};
