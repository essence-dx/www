import type { DynamicOptionsBoundary } from "../types";
import type { NodeParameterDefinition, NodeTypeDescription } from "./types";

function modelDynamicOptions(responseFilter: string): DynamicOptionsBoundary {
  return {
    source: "n8n-type-options-routing",
    loadMethod: "GET:/v1/models",
    request: {
      method: "GET",
      url: "/v1/models",
    },
    responseFilter,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    issue:
      "OpenAI model options require the DX-owned n8n editor-session adapter before provider calls can run.",
  };
}

const resourceField: NodeParameterDefinition = {
  name: "resource",
  label: "Resource",
  type: "options",
  defaultValue: "text",
  noDataExpression: true,
  options: [
    { name: "Chat", value: "chat" },
    { name: "Image", value: "image" },
    { name: "Text", value: "text" },
  ],
};

const chatOperationField: NodeParameterDefinition = {
  name: "operation",
  label: "Operation",
  type: "options",
  defaultValue: "complete",
  noDataExpression: true,
  options: [
    {
      name: "Complete",
      value: "complete",
      action: "Create a Completion",
      description: "Create one or more completions for a given text",
    },
  ],
  displayOptions: {
    show: {
      resource: ["chat"],
    },
  },
};

const textOperationField: NodeParameterDefinition = {
  name: "operation",
  label: "Operation",
  type: "options",
  defaultValue: "complete",
  noDataExpression: true,
  options: [
    {
      name: "Complete",
      value: "complete",
      action: "Create a Completion",
      description: "Create one or more completions for a given text",
    },
    {
      name: "Edit",
      value: "edit",
      action: "Create an Edit",
      description: "Create an edited version for a given text",
    },
    {
      name: "Moderate",
      value: "moderate",
      action: "Create a Moderation",
      description: "Classify if a text violates OpenAI's content policy",
    },
  ],
  displayOptions: {
    show: {
      resource: ["text"],
    },
  },
};

const imageOperationField: NodeParameterDefinition = {
  name: "operation",
  label: "Operation",
  type: "options",
  defaultValue: "create",
  noDataExpression: true,
  options: [
    {
      name: "Create",
      value: "create",
      action: "Create an Image",
      description: "Create an image for a given text",
    },
  ],
  displayOptions: {
    show: {
      resource: ["image"],
    },
  },
};

const chatFields: NodeParameterDefinition[] = [
  {
    name: "chatModel",
    label: "Model",
    type: "options",
    defaultValue: "gpt-3.5-turbo",
    description: "The model which will generate the completion.",
    dynamicOptions: modelDynamicOptions("gpt-"),
    displayOptions: {
      show: {
        resource: ["chat"],
        operation: ["complete"],
      },
    },
  },
  {
    name: "prompt",
    label: "Prompt",
    type: "fixedCollection",
    defaultValue: {},
    placeholder: "Add Message",
    displayOptions: {
      show: {
        resource: ["chat"],
        operation: ["complete"],
      },
    },
    childParameters: [
      {
        name: "role",
        label: "Role",
        type: "options",
        defaultValue: "user",
        options: [
          { name: "Assistant", value: "assistant" },
          { name: "System", value: "system" },
          { name: "User", value: "user" },
        ],
      },
      {
        name: "content",
        label: "Content",
        type: "string",
        defaultValue: "",
      },
    ],
    renderingBoundary: "complex-source-field",
  },
  {
    name: "simplifyOutput",
    label: "Simplify",
    type: "boolean",
    defaultValue: true,
    description: "Whether to return a simplified version of the response instead of the raw data",
    displayOptions: {
      show: {
        resource: ["chat"],
        operation: ["complete"],
      },
    },
  },
  {
    name: "options",
    label: "Options",
    type: "collection",
    defaultValue: {},
    placeholder: "Add option",
    description: "Additional options to add",
    displayOptions: {
      show: {
        resource: ["chat"],
        operation: ["complete"],
      },
    },
    childParameters: [
      {
        name: "temperature",
        label: "Sampling Temperature",
        type: "number",
        defaultValue: 1,
      },
      {
        name: "topP",
        label: "Top P",
        type: "number",
        defaultValue: 1,
      },
      {
        name: "maxTokens",
        label: "Maximum Number of Tokens",
        type: "number",
        defaultValue: 16,
      },
    ],
    renderingBoundary: "complex-source-field",
  },
];

const textFields: NodeParameterDefinition[] = [
  {
    name: "model",
    label: "Model",
    type: "options",
    defaultValue: "text-davinci-edit-001",
    options: [
      { name: "code-davinci-edit-001", value: "code-davinci-edit-001" },
      { name: "text-davinci-edit-001", value: "text-davinci-edit-001" },
    ],
    displayOptions: {
      show: {
        resource: ["text"],
        operation: ["edit"],
      },
    },
  },
  {
    name: "input",
    label: "Input",
    type: "string",
    defaultValue: "",
    placeholder: "e.g. What day of the wek is it?",
    description: "The input text to be edited",
    displayOptions: {
      show: {
        resource: ["text"],
        operation: ["edit"],
      },
    },
  },
  {
    name: "instruction",
    label: "Instruction",
    type: "string",
    defaultValue: "",
    placeholder: "e.g. Fix the spelling mistakes",
    description: "The instruction that tells the model how to edit the input text",
    displayOptions: {
      show: {
        resource: ["text"],
        operation: ["edit"],
      },
    },
  },
];

const imageFields: NodeParameterDefinition[] = [
  {
    name: "prompt",
    label: "Prompt",
    type: "string",
    defaultValue: "",
    placeholder: "e.g. A cute cat eating a dinosaur",
    description: "A text description of the desired image.",
    displayOptions: {
      show: {
        resource: ["image"],
        operation: ["create"],
      },
    },
  },
  {
    name: "imageModel",
    label: "Model",
    type: "options",
    defaultValue: "dall-e-2",
    description: "The model to use for image generation",
    dynamicOptions: modelDynamicOptions("dall-"),
    displayOptions: {
      show: {
        resource: ["image"],
        operation: ["create"],
      },
    },
  },
  {
    name: "responseFormat",
    label: "Response Format",
    type: "options",
    defaultValue: "binaryData",
    description: "The format in which to return the image.",
    options: [
      { name: "Binary File", value: "binaryData" },
      { name: "Image Url", value: "imageUrl" },
    ],
    displayOptions: {
      show: {
        resource: ["image"],
        operation: ["create"],
      },
    },
  },
  {
    name: "options",
    label: "Options",
    type: "collection",
    defaultValue: {},
    placeholder: "Add option",
    description: "Additional options to add",
    displayOptions: {
      show: {
        resource: ["image"],
        operation: ["create"],
      },
    },
    childParameters: [
      {
        name: "n",
        label: "Number of Images",
        type: "number",
        defaultValue: 1,
      },
      {
        name: "quality",
        label: "Quality",
        type: "options",
        defaultValue: "standard",
        options: [
          { name: "HD", value: "hd" },
          { name: "Standard", value: "standard" },
        ],
      },
      {
        name: "style",
        label: "Style",
        type: "options",
        defaultValue: "vivid",
        options: [
          { name: "Natural", value: "natural" },
          { name: "Vivid", value: "vivid" },
        ],
      },
    ],
    renderingBoundary: "complex-source-field",
  },
];

export const openAiNodeType: NodeTypeDescription = {
  name: "n8n-nodes-base.openAi",
  displayName: "OpenAI",
  sourcePath: "nodes/OpenAi/OpenAi.node.ts",
  version: 1.1,
  credentials: [{ name: "openAiApi", required: true }],
  properties: [
    resourceField,
    chatOperationField,
    imageOperationField,
    textOperationField,
    ...chatFields,
    ...imageFields,
    ...textFields,
  ],
  categories: ["AI"],
  authKinds: ["apiKey"],
  sourceProvenance: "n8n-nodes-base",
  workflowNode: {
    ready: true,
    trigger: false,
    usable_as_tool: true,
    run_mode: "regular",
  },
};
