export type N8nConnectorStatus = 'metadata-ready' | 'missing-config';

export type N8nDashboardConnector = {
    id: string;
    displayName: string;
    description: string;
    sourceFile: string;
    upstreamTypes: string[];
    resources: string[];
    operations: string[];
    credentialTypes: string[];
    secretFields: string[];
    requiredEnv: string[];
    status: N8nConnectorStatus;
    trigger: boolean;
    usableAsTool: boolean;
};

export type N8nWorkflowReadiness = {
    connector: N8nDashboardConnector;
    status: N8nConnectorStatus;
    missingEnv: string[];
    canDraftLocally: boolean;
    canRunLive: boolean;
    boundary: string;
};

export type N8nRedactedReceipt = {
    schema: 'dx.automations.zed.run_receipt';
    packageId: 'automations/n8n';
    workflowId: string;
    workflowIntent: string;
    connectorId: string;
    connectorName: string;
    status: N8nConnectorStatus;
    mode: 'dashboard-local-draft';
    receiptPath: string;
    redacted: true;
    command: 'dx automations run --json';
    nextAction: string;
};

export const n8nAutomationForgeMetadata = {
    packageId: "automations/n8n",
    officialName: "Automation Connectors",
    officialPackageName: "Automation Connectors",
    aliases: ["automation-connectors", "@n8n/nodes-base", "n8n-nodes-base", "workflows/n8n"],
    upstreamPackage: "n8n-nodes-base",
    upstreamVersion: "2.22.0",
    sourceMirror: "G:\\WWW\\inspirations\\n8n\\packages\\nodes-base",
    provenance: [
        "packages/nodes-base/package.json",
        "packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts",
        "packages/nodes-base/nodes/Slack/Slack.node.ts",
        "packages/nodes-base/nodes/Notion/Notion.node.ts",
        "packages/nodes-base/credentials/SlackApi.credentials.ts",
        "packages/nodes-base/credentials/SlackOAuth2Api.credentials.ts",
        "packages/nodes-base/credentials/NotionApi.credentials.ts",
    ],
    upstreamPublicApis: [
        "INodeTypeDescription",
        "INodeType",
        "IVersionedNodeType",
        "ICredentialType",
        "IAuthenticateGeneric",
        "ICredentialTestRequest",
    ],
    exportedFiles: [
        "examples/dashboard/src/lib/n8nAutomationBridge.ts",
        "examples/dashboard/src/components/AutomationWorkflowPanel.tsx",
        "examples/template/automations/automations-metadata.ts",
        "js/lib/automations/n8n/metadata.ts",
        "js/lib/automations/n8n/catalog.ts",
        "js/lib/automations/n8n/readiness.ts",
        "js/lib/automations/n8n/receipt.ts",
        "js/lib/automations/n8n/bridge.ts",
    ],
    requiredEnv: [
        "SLACK_BOT_TOKEN",
        "SLACK_SIGNING_SECRET",
        "NOTION_API_KEY",
        "DX_AUTOMATIONS_OPERATOR_APPROVAL",
    ],
    appOwnedBoundaries: [
        "Credential storage, encryption, and rotation",
        "Live workflow execution approval",
        "Provider-specific OAuth callback setup",
        "Receipt persistence under G:\\Dx\\.dx\\receipts\\automations",
    ],
    receiptPaths: ["G:\\Dx\\.dx\\receipts\\automations"],
    selectedSurfaces: [
        "connector-catalog",
        "credential-readiness",
        "redacted-run-receipt",
        "starter-dashboard-workflow",
    ],
    dxCheckVisibility: {
        schema: "dx.forge.package.dx_check_visibility",
        currentStatus: "present",
        statuses: ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
        receiptPath: "G:\\Dx\\.dx\\receipts\\automations\\launch-release-notification.json",
    },
    honestyLabel: "ADAPTER-BOUNDARY",
    commands: {
        connectors: 'dx automations connectors --json',
        credentials: 'dx automations credentials --json',
        run: 'dx automations run --json',
    },
} as const;

export const n8nDashboardConnectors: N8nDashboardConnector[] = [
    {
        id: 'n8n-nodes-base.manualTrigger',
        displayName: 'Manual Trigger',
        description: 'Starts a workflow from an explicit operator action.',
        sourceFile: 'packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts',
        upstreamTypes: ['INodeTypeDescription', 'INodeType', 'ITriggerFunctions'],
        resources: ['workflow'],
        operations: ['trigger'],
        credentialTypes: [],
        secretFields: [],
        requiredEnv: [],
        status: 'metadata-ready',
        trigger: true,
        usableAsTool: false,
    },
    {
        id: 'n8n-nodes-base.slack',
        displayName: 'Slack',
        description: 'Consumes the Slack API through bearer-token or OAuth credentials.',
        sourceFile: 'packages/nodes-base/nodes/Slack/Slack.node.ts',
        upstreamTypes: ['IVersionedNodeType', 'ICredentialType', 'IAuthenticateGeneric'],
        resources: ['channel', 'message', 'file', 'reaction'],
        operations: ['post', 'get', 'getAll', 'archive'],
        credentialTypes: ['slackApi', 'slackOAuth2Api'],
        secretFields: ["accessToken", "signatureSecret"],
        requiredEnv: ['SLACK_BOT_TOKEN', 'SLACK_SIGNING_SECRET'],
        status: 'missing-config',
        trigger: false,
        usableAsTool: true,
    },
    {
        id: 'n8n-nodes-base.notion',
        displayName: 'Notion',
        description: 'Consumes the Notion API through API-key or OAuth credentials.',
        sourceFile: 'packages/nodes-base/nodes/Notion/Notion.node.ts',
        upstreamTypes: ['IVersionedNodeType', 'ICredentialType'],
        resources: ['database', 'page', 'block', 'user'],
        operations: ['search', 'get', 'create', 'update'],
        credentialTypes: ['notionApi', 'notionOAuth2Api'],
        secretFields: ['apiKey', 'accessToken'],
        requiredEnv: ['NOTION_API_KEY'],
        status: 'missing-config',
        trigger: false,
        usableAsTool: true,
    },
];

export function selectN8nConnector(connectorId: string): N8nDashboardConnector {
    return (
        n8nDashboardConnectors.find((connector) => connector.id === connectorId) ||
        n8nDashboardConnectors[0]
    );
}

export function formatN8nCredentialBoundary(connector: N8nDashboardConnector): string {
    if (connector.credentialTypes.length === 0) {
        return 'No credential type is required for this local workflow draft.';
    }

    return `${connector.credentialTypes.join(', ')} require app-owned secret storage; redacted fields: ${connector.secretFields.join(', ')}.`;
}

export function buildN8nWorkflowReadiness(connectorId: string): N8nWorkflowReadiness {
    const connector = selectN8nConnector(connectorId);
    const missingEnv = connector.requiredEnv;

    return {
        connector,
        status: missingEnv.length > 0 ? 'missing-config' : 'metadata-ready',
        missingEnv,
        canDraftLocally: true,
        canRunLive: missingEnv.length === 0,
        boundary: formatN8nCredentialBoundary(connector),
    };
}

export function createN8nDashboardWorkflow(connectorId: string) {
    const readiness = buildN8nWorkflowReadiness(connectorId);

    return {
        workflowId: `dashboard-${readiness.connector.id.replace('n8n-nodes-base.', '')}`,
        packageId: n8nAutomationForgeMetadata.packageId,
        connector: readiness.connector,
        readiness,
        sourceMirror: n8nAutomationForgeMetadata.sourceMirror,
        command: n8nAutomationForgeMetadata.commands.run,
    };
}

export function createRedactedN8nReceipt({
    connectorId,
    intent,
}: {
    connectorId: string;
    intent: string;
}): N8nRedactedReceipt {
    const workflow = createN8nDashboardWorkflow(connectorId);
    const receiptPath = `${n8nAutomationForgeMetadata.receiptPaths[0]}\\${workflow.workflowId}.json`;
    const trimmedIntent = intent.trim();

    return {
        schema: 'dx.automations.zed.run_receipt',
        packageId: 'automations/n8n',
        workflowId: workflow.workflowId,
        workflowIntent: trimmedIntent,
        connectorId: workflow.connector.id,
        connectorName: workflow.connector.displayName,
        status: workflow.readiness.status,
        mode: 'dashboard-local-draft',
        receiptPath,
        redacted: true,
        command: 'dx automations run --json',
        nextAction:
            workflow.readiness.status === 'metadata-ready'
                ? `Review local draft intent: ${trimmedIntent || 'No intent entered'}.`
                : `Configure ${workflow.readiness.missingEnv.join(', ')} before live execution.`,
    };
}
