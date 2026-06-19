export const automationSummary = {
  "schema": "dx.automations.template_metadata",
  "connectorCount": 536,
  "credentialCount": 396,
  "readyConnectorCount": 112,
  "credentialGatedConnectorCount": 424,
  "triggerCount": 110,
  "toolReadyCount": 360,
  "receiptDir": "G:/Dx/.dx/receipts/automations",
  "receiptSeedPath": "G:/Dx/.dx/receipts/automations/launch-release-notification.json",
  "runReceiptPath": "G:/Dx/.dx/receipts/automations/run-latest.json",
  "commands": {
    "connectors": "dx automations connectors --json",
    "credentials": "dx automations credentials --json",
    "run": "dx automations run --json"
  },
  "sourceProvenance": "n8n-nodes-base"
} as const;

export const automationConnectorHighlights = [
  {
    "id": "n8n-nodes-base.slack",
    "displayName": "Slack",
    "status": "needs_credential",
    "authKinds": [
      "bearer_token",
      "oauth2"
    ],
    "credentials": [
      "slackApi",
      "slackOAuth2Api"
    ],
    "sourceFile": "nodes/Slack/Slack.node.ts",
    "resources": [
      {
        "name": "Channel",
        "value": "channel"
      },
      {
        "name": "File",
        "value": "file"
      },
      {
        "name": "Message",
        "value": "message"
      },
      {
        "name": "Reaction",
        "value": "reaction"
      },
      {
        "name": "Star",
        "value": "star"
      }
    ],
    "operations": [
      {
        "name": "Archive",
        "value": "archive",
        "action": "Archive a channel"
      },
      {
        "name": "Close",
        "value": "close",
        "action": "Close a channel"
      },
      {
        "name": "Create",
        "value": "create",
        "action": "Create a channel"
      },
      {
        "name": "Get",
        "value": "get",
        "action": "Get a channel"
      },
      {
        "name": "Get Many",
        "value": "getAll",
        "action": "Get many channels"
      },
      {
        "name": "History",
        "value": "history",
        "action": "Get the history of a channel"
      }
    ],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": true,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.notion",
    "displayName": "Notion",
    "status": "needs_credential",
    "authKinds": [
      "api_key",
      "oauth2"
    ],
    "credentials": [
      "notionApi",
      "notionOAuth2Api"
    ],
    "sourceFile": "nodes/Notion/Notion.node.ts",
    "resources": [
      {
        "name": "Block",
        "value": "block"
      },
      {
        "name": "Database",
        "value": "database"
      },
      {
        "name": "Database Page",
        "value": "databasePage"
      },
      {
        "name": "Page",
        "value": "page"
      },
      {
        "name": "User",
        "value": "user"
      }
    ],
    "operations": [
      {
        "name": "Append After",
        "value": "append",
        "action": "Append a block"
      },
      {
        "name": "Get",
        "value": "get",
        "action": "Get a database"
      },
      {
        "name": "Get Many",
        "value": "getAll",
        "action": "Get many databases"
      },
      {
        "name": "Search",
        "value": "search",
        "action": "Search a database"
      },
      {
        "name": "Create",
        "value": "create",
        "action": "Create a database page"
      },
      {
        "name": "Update",
        "value": "update",
        "action": "Update a database page"
      }
    ],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": true,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.gmail",
    "displayName": "Gmail",
    "status": "needs_credential",
    "authKinds": [
      "custom",
      "oauth2"
    ],
    "credentials": [
      "googleApi",
      "gmailOAuth2"
    ],
    "sourceFile": "nodes/Google/Gmail/v2/GmailV2.node.ts",
    "resources": [
      {
        "name": "Message",
        "value": "message"
      },
      {
        "name": "Label",
        "value": "label"
      },
      {
        "name": "Draft",
        "value": "draft"
      },
      {
        "name": "Thread",
        "value": "thread"
      }
    ],
    "operations": [
      {
        "name": "Create",
        "value": "create",
        "action": "Create a draft"
      },
      {
        "name": "Delete",
        "value": "delete",
        "action": "Delete a draft"
      },
      {
        "name": "Get",
        "value": "get",
        "action": "Get a draft"
      },
      {
        "name": "Get Many",
        "value": "getAll",
        "action": "Get many drafts"
      },
      {
        "name": "Add Label",
        "value": "addLabels",
        "action": "Add label to message"
      },
      {
        "name": "Mark as Read",
        "value": "markAsRead",
        "action": "Mark a message as read"
      }
    ],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": true,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.googleSheets",
    "displayName": "Google Sheets",
    "status": "needs_credential",
    "authKinds": [
      "oauth2",
      "custom"
    ],
    "credentials": [
      "googleSheetsTriggerOAuth2Api",
      "googleApi",
      "googleSheetsOAuth2Api"
    ],
    "sourceFile": "nodes/Google/Sheet/GoogleSheets.node.ts",
    "resources": [
      {
        "name": "Spreadsheet",
        "value": "spreadsheet"
      },
      {
        "name": "Sheet",
        "value": "sheet"
      },
      {
        "name": "Document",
        "value": "spreadsheet"
      },
      {
        "name": "Sheet Within Document",
        "value": "sheet"
      }
    ],
    "operations": [
      {
        "name": "Append",
        "value": "append",
        "action": "Append data to a sheet"
      },
      {
        "name": "Clear",
        "value": "clear",
        "action": "Clear a sheet"
      },
      {
        "name": "Create",
        "value": "create",
        "action": "Create a sheet"
      },
      {
        "name": "Create or Update",
        "value": "upsert",
        "action": "Create or update a sheet"
      },
      {
        "name": "Delete",
        "value": "delete",
        "action": "Delete a sheet"
      },
      {
        "name": "Lookup",
        "value": "lookup",
        "action": "Look up a column value in a sheet"
      }
    ],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": true,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.github",
    "displayName": "GitHub",
    "status": "needs_credential",
    "authKinds": [
      "bearer_token",
      "oauth2"
    ],
    "credentials": [
      "githubApi",
      "githubOAuth2Api"
    ],
    "sourceFile": "nodes/Github/Github.node.ts",
    "resources": [
      {
        "name": "File",
        "value": "file"
      },
      {
        "name": "Issue",
        "value": "issue"
      },
      {
        "name": "Organization",
        "value": "organization"
      },
      {
        "name": "Release",
        "value": "release"
      },
      {
        "name": "Repository",
        "value": "repository"
      }
    ],
    "operations": [
      {
        "name": "Get Repositories",
        "value": "getRepositories",
        "action": "Get repositories for an organization"
      },
      {
        "name": "Create",
        "value": "create",
        "action": "Create an issue"
      },
      {
        "name": "Create Comment",
        "value": "createComment",
        "action": "Create a comment on an issue"
      },
      {
        "name": "Edit",
        "value": "edit",
        "action": "Edit an issue"
      },
      {
        "name": "Get",
        "value": "get",
        "action": "Get an issue"
      },
      {
        "name": "Lock",
        "value": "lock",
        "action": "Lock an issue"
      }
    ],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": true,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.linear",
    "displayName": "Linear",
    "status": "needs_credential",
    "authKinds": [
      "api_key",
      "oauth2"
    ],
    "credentials": [
      "linearApi",
      "linearOAuth2Api"
    ],
    "sourceFile": "nodes/Linear/Linear.node.ts",
    "resources": [
      {
        "name": "Comment",
        "value": "comment"
      },
      {
        "name": "Issue",
        "value": "issue"
      }
    ],
    "operations": [
      {
        "name": "Add Comment",
        "value": "addComment",
        "action": "Add a comment to an issue"
      },
      {
        "name": "Add Link",
        "value": "addLink",
        "action": "Add a link to an issue"
      },
      {
        "name": "Create",
        "value": "create",
        "action": "Create an issue"
      },
      {
        "name": "Delete",
        "value": "delete",
        "action": "Delete an issue"
      },
      {
        "name": "Get",
        "value": "get",
        "action": "Get an issue"
      },
      {
        "name": "Get Many",
        "value": "getAll",
        "action": "Get many issues"
      }
    ],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": true,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.airtable",
    "displayName": "Airtable",
    "status": "needs_credential",
    "authKinds": [
      "api_key",
      "bearer_token",
      "oauth2"
    ],
    "credentials": [
      "airtableApi",
      "airtableTokenApi",
      "airtableOAuth2Api"
    ],
    "sourceFile": "nodes/Airtable/v1/AirtableV1.node.ts",
    "resources": [],
    "operations": [
      {
        "name": "Append",
        "value": "append",
        "action": "Append data to a table"
      },
      {
        "name": "Delete",
        "value": "delete",
        "action": "Delete data from a table"
      },
      {
        "name": "List",
        "value": "list",
        "action": "List data from a table"
      },
      {
        "name": "Read",
        "value": "read",
        "action": "Read data from a table"
      },
      {
        "name": "Update",
        "value": "update",
        "action": "Update data in a table"
      }
    ],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": false,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.stripe",
    "displayName": "Stripe",
    "status": "needs_credential",
    "authKinds": [
      "bearer_token"
    ],
    "credentials": [
      "stripeApi"
    ],
    "sourceFile": "nodes/Stripe/Stripe.node.ts",
    "resources": [
      {
        "name": "Balance",
        "value": "balance"
      },
      {
        "name": "Charge",
        "value": "charge"
      },
      {
        "name": "Coupon",
        "value": "coupon"
      },
      {
        "name": "Customer",
        "value": "customer"
      },
      {
        "name": "Customer Card",
        "value": "customerCard"
      }
    ],
    "operations": [
      {
        "name": "Get",
        "value": "get",
        "action": "Get a balance"
      },
      {
        "name": "Create",
        "value": "create",
        "action": "Create a charge"
      },
      {
        "name": "Get Many",
        "value": "getAll",
        "action": "Get many charges"
      },
      {
        "name": "Update",
        "value": "update",
        "action": "Update a charge"
      },
      {
        "name": "Add",
        "value": "add",
        "action": "Add a customer card"
      },
      {
        "name": "Remove",
        "value": "remove",
        "action": "Remove a customer card"
      }
    ],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": true,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.telegram",
    "displayName": "Telegram",
    "status": "needs_credential",
    "authKinds": [
      "bearer_token"
    ],
    "credentials": [
      "telegramApi"
    ],
    "sourceFile": "nodes/Telegram/Telegram.node.ts",
    "resources": [
      {
        "name": "Chat",
        "value": "chat"
      },
      {
        "name": "Callback",
        "value": "callback"
      },
      {
        "name": "File",
        "value": "file"
      },
      {
        "name": "Message",
        "value": "message"
      }
    ],
    "operations": [
      {
        "name": "Get",
        "value": "get",
        "action": "Get a chat"
      },
      {
        "name": "Get Administrators",
        "value": "administrators",
        "action": "Get all administrators in a chat"
      },
      {
        "name": "Get Member",
        "value": "member",
        "action": "Get a member in a chat"
      },
      {
        "name": "Leave",
        "value": "leave",
        "action": "Leave a chat"
      },
      {
        "name": "Set Description",
        "value": "setDescription",
        "action": "Set description on a chat"
      },
      {
        "name": "Set Title",
        "value": "setTitle",
        "action": "Set a title on a chat"
      }
    ],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": true,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.webhook",
    "displayName": "Webhook",
    "status": "ready",
    "authKinds": [
      "none"
    ],
    "credentials": [],
    "sourceFile": "nodes/Webhook/Webhook.node.ts",
    "resources": [],
    "operations": [],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": false,
      "run_mode": "metadata-ready"
    }
  },
  {
    "id": "n8n-nodes-base.httpRequest",
    "displayName": "HTTP Request",
    "status": "needs_credential",
    "authKinds": [
      "basic_auth",
      "custom",
      "oauth1",
      "oauth2"
    ],
    "credentials": [
      "httpBasicAuth",
      "httpDigestAuth",
      "httpHeaderAuth",
      "httpQueryAuth",
      "oAuth1Api",
      "oAuth2Api",
      "httpSslAuth"
    ],
    "sourceFile": "nodes/HttpRequest/HttpRequest.node.ts",
    "resources": [],
    "operations": [],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": false,
      "run_mode": "credential-gated"
    }
  },
  {
    "id": "n8n-nodes-base.cron",
    "displayName": "Cron",
    "status": "ready",
    "authKinds": [
      "none"
    ],
    "credentials": [],
    "sourceFile": "nodes/Cron/Cron.node.ts",
    "resources": [],
    "operations": [],
    "workflowNode": {
      "ready": true,
      "trigger": false,
      "usable_as_tool": false,
      "run_mode": "metadata-ready"
    }
  }
] as const;

export const automationCredentialHighlights = [
  {
    "id": "actionNetworkApi",
    "displayName": "Action Network API",
    "authKind": "api_key",
    "fieldCount": 1,
    "secretFields": [
      "apiKey"
    ],
    "sourceFile": "credentials/ActionNetworkApi.credentials.ts"
  },
  {
    "id": "activeCampaignApi",
    "displayName": "ActiveCampaign API",
    "authKind": "api_key",
    "fieldCount": 2,
    "secretFields": [
      "apiKey"
    ],
    "sourceFile": "credentials/ActiveCampaignApi.credentials.ts"
  },
  {
    "id": "acuitySchedulingApi",
    "displayName": "Acuity Scheduling API",
    "authKind": "api_key",
    "fieldCount": 2,
    "secretFields": [
      "apiKey"
    ],
    "sourceFile": "credentials/AcuitySchedulingApi.credentials.ts"
  },
  {
    "id": "acuitySchedulingOAuth2Api",
    "displayName": "AcuityScheduling OAuth2 API",
    "authKind": "oauth2",
    "fieldCount": 6,
    "secretFields": [
      "accessTokenUrl"
    ],
    "sourceFile": "credentials/AcuitySchedulingOAuth2Api.credentials.ts"
  },
  {
    "id": "adaloApi",
    "displayName": "Adalo API",
    "authKind": "bearer_token",
    "fieldCount": 2,
    "secretFields": [
      "apiKey"
    ],
    "sourceFile": "credentials/AdaloApi.credentials.ts"
  },
  {
    "id": "affinityApi",
    "displayName": "Affinity API",
    "authKind": "api_key",
    "fieldCount": 1,
    "secretFields": [
      "apiKey"
    ],
    "sourceFile": "credentials/AffinityApi.credentials.ts"
  },
  {
    "id": "agileCrmApi",
    "displayName": "AgileCRM API",
    "authKind": "api_key",
    "fieldCount": 3,
    "secretFields": [
      "apiKey"
    ],
    "sourceFile": "credentials/AgileCrmApi.credentials.ts"
  },
  {
    "id": "airtableApi",
    "displayName": "Airtable API",
    "authKind": "api_key",
    "fieldCount": 1,
    "secretFields": [
      "apiKey"
    ],
    "sourceFile": "credentials/AirtableApi.credentials.ts"
  },
  {
    "id": "airtableOAuth2Api",
    "displayName": "Airtable OAuth2 API",
    "authKind": "oauth2",
    "fieldCount": 6,
    "secretFields": [
      "accessTokenUrl"
    ],
    "sourceFile": "credentials/AirtableOAuth2Api.credentials.ts"
  },
  {
    "id": "airtableTokenApi",
    "displayName": "Airtable Personal Access Token API",
    "authKind": "bearer_token",
    "fieldCount": 2,
    "secretFields": [
      "accessToken"
    ],
    "sourceFile": "credentials/AirtableTokenApi.credentials.ts"
  }
] as const;

export const automationRoutes = [
  { href: "/automations", label: "Overview" },
  { href: "/automations/connectors", label: "Connectors" },
  { href: "/automations/credentials", label: "Credentials" },
  { href: "/automations/workflows", label: "Workflows" },
] as const;
