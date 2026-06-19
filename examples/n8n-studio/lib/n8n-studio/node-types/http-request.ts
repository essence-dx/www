import type { NodeTypeDescription } from "./types";

const methodOptions = [
  { name: "DELETE", value: "DELETE" },
  { name: "GET", value: "GET" },
  { name: "HEAD", value: "HEAD" },
  { name: "OPTIONS", value: "OPTIONS" },
  { name: "PATCH", value: "PATCH" },
  { name: "POST", value: "POST" },
  { name: "PUT", value: "PUT" },
];

const authenticationOptions = [
  { name: "None", value: "none" },
  {
    name: "Predefined Credential Type",
    value: "predefinedCredentialType",
    description: "Use one of the credential types implemented by the provider.",
  },
  {
    name: "Generic Credential Type",
    value: "genericCredentialType",
    description: "Choose from basic, bearer, header, OAuth, and other generic auth forms.",
  },
];

export const httpRequestNodeType: NodeTypeDescription = {
  name: "n8n-nodes-base.httpRequest",
  displayName: "HTTP Request",
  sourcePath: "nodes/HttpRequest/HttpRequest.node.ts",
  version: 4,
  credentials: [
    { name: "HttpBasicAuth", required: false },
    { name: "HttpBearerAuth", required: false },
    { name: "HttpDigestAuth", required: false },
    { name: "HttpCustomAuth", required: false },
  ],
  properties: [
    {
      label: "",
      name: "curlImport",
      type: "curlImport",
      defaultValue: "",
    },
    {
      label: "Method",
      name: "method",
      type: "options",
      defaultValue: "GET",
      description: "The request method to use",
      options: methodOptions,
    },
    {
      label: "URL",
      name: "url",
      type: "string",
      defaultValue: "",
      placeholder: "http://example.com/index.html",
      description: "The URL to make the request to",
      required: true,
    },
    {
      label: "Authentication",
      name: "authentication",
      type: "options",
      defaultValue: "none",
      noDataExpression: true,
      options: authenticationOptions,
    },
    {
      label: "Credential Type",
      name: "nodeCredentialType",
      type: "credentialsSelect",
      defaultValue: "",
      required: true,
      noDataExpression: true,
      credentialTypes: ["extends:oAuth2Api", "extends:oAuth1Api", "has:authenticate"],
      displayOptions: {
        show: {
          authentication: ["predefinedCredentialType"],
        },
      },
    },
    {
      label: "Generic Auth Type",
      name: "genericAuthType",
      type: "credentialsSelect",
      defaultValue: "",
      required: true,
      credentialTypes: ["has:genericAuth"],
      displayOptions: {
        show: {
          authentication: ["genericCredentialType"],
        },
      },
    },
    {
      label: "SSL Certificates",
      name: "provideSslCertificates",
      type: "boolean",
      defaultValue: false,
    },
  ],
};
