import type { NodeTypeDescription } from "./types";

const resourceOptions = [
  { name: "Message", value: "message" },
  { name: "Channel", value: "channel" },
  { name: "File", value: "file" },
  { name: "User", value: "user" },
];

const messageOperationOptions = [
  { name: "Delete", value: "delete", action: "Delete a message" },
  { name: "Get Permalink", value: "getPermalink", action: "Get a message permalink" },
  { name: "Search", value: "search", action: "Search for messages" },
  { name: "Send", value: "post", action: "Send a message" },
  {
    name: "Send and Wait for Response",
    value: "sendAndWait",
    action: "Send message and wait for response",
  },
  { name: "Update", value: "update", action: "Update a message" },
];

const channelModes = [
  {
    displayName: "From List",
    name: "list",
    type: "list" as const,
    placeholder: "Select a channel...",
    searchListMethod: "getChannels",
    searchable: true,
  },
  {
    displayName: "By ID",
    name: "id",
    type: "string" as const,
    placeholder: "C0122KQ70S7E",
  },
  {
    displayName: "By Name",
    name: "name",
    type: "string" as const,
    placeholder: "#general",
  },
  {
    displayName: "By URL",
    name: "url",
    type: "url" as const,
    placeholder: "https://app.slack.com/client/TS9594PZK/B0556F47Z3A",
  },
];

export const slackNodeType: NodeTypeDescription = {
  name: "n8n-nodes-base.slack",
  displayName: "Slack",
  sourcePath: "nodes/Slack/Slack.node.ts",
  version: 2,
  credentials: [
    { name: "SlackApi", required: false },
    { name: "SlackOAuth2Api", required: false },
  ],
  properties: [
    {
      label: "Resource",
      name: "resource",
      type: "options",
      defaultValue: "message",
      noDataExpression: true,
      options: resourceOptions,
    },
    {
      label: "Operation",
      name: "operation",
      type: "options",
      defaultValue: "post",
      noDataExpression: true,
      options: messageOperationOptions,
      displayOptions: {
        show: {
          resource: ["message"],
        },
      },
    },
    {
      label: "Send Message To",
      name: "select",
      type: "options",
      defaultValue: "",
      required: true,
      options: [
        { name: "Channel", value: "channel" },
        { name: "User", value: "user" },
      ],
      displayOptions: {
        show: {
          resource: ["message"],
          operation: ["post"],
        },
      },
    },
    {
      label: "Channel",
      name: "channelId",
      type: "resourceLocator",
      defaultValue: { mode: "list", value: "" },
      required: true,
      placeholder: "Select a channel...",
      resourceLocatorModes: channelModes,
      displayOptions: {
        show: {
          resource: ["message"],
          operation: ["post"],
          select: ["channel"],
        },
      },
    },
  ],
};
