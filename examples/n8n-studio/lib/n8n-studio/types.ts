export type StudioSurface =
  | "node-creator"
  | "workflow-canvas"
  | "node-parameters"
  | "expression-editor"
  | "credentials"
  | "resource-locator"
  | "pinned-data"
  | "execution-debug"
  | "ai-tools"
  | "import-export"
  | "receipts";

export type StudioReadinessStatus = "ready" | "blocked" | "configured" | "source-only";

export type N8nSourceNode = {
  id: string;
  name: string;
  displayName: string;
  category: string;
  role: "trigger" | "action" | "ai-tool" | "utility";
  description: string;
  sourcePath: string;
  credentialTypes: string[];
  operations: string[];
  trustStatus: "source-mirrored" | "source-generated" | "dx-owned";
};

export type ParameterOption = {
  name: string;
  value: string;
  description?: string;
  action?: string;
};

export type ParameterDisplayOptions = {
  show?: Record<string, unknown[]>;
  hide?: Record<string, unknown[]>;
};

export type ResourceLocatorMode = {
  name: string;
  displayName: string;
  type: "list" | "string" | "url";
  placeholder?: string;
  searchListMethod?: string;
  searchable?: boolean;
};

export type CatalogSummary = {
  schema: "dx.n8n-studio.catalog";
  sourceManifestPath: string;
  copiedFrom: string;
  nodeFolderCount: number;
  nodeFileCount: number;
  credentialFileCount: number;
  catalogNodes: N8nSourceNode[];
  generatedMetadata?: {
    sourceAvailable: boolean;
    sourcePath: string;
    sourceRecordCount: number;
    nodeTypeCount: number;
    skippedRecordCount: number;
    credentialTypeCount: number;
    generatedAt?: string;
    issue?: string;
  };
};

export type WorkflowPosition = {
  x: number;
  y: number;
};

export type WorkflowNode = {
  id: string;
  name: string;
  type: string;
  typeVersion: number;
  position: WorkflowPosition;
  parameters: Record<string, unknown>;
  credentials?: Record<string, { id: string; name: string }>;
  disabled?: boolean;
  notes?: string;
};

export type WorkflowConnection = {
  id: string;
  sourceNode: string;
  targetNode: string;
  sourceOutput: "main" | "ai_tool";
  targetInput: "main" | "ai_tool";
  index: number;
};

export type WorkflowDocument = {
  schemaVersion: number;
  id: string;
  projectId: string;
  name: string;
  active: boolean;
  nodes: WorkflowNode[];
  connections: WorkflowConnection[];
  tags: string[];
  pinData: Record<string, unknown[]>;
  meta: {
    source: "dx-www-n8n-studio";
    liveProviderExecution: false;
  };
};

export type CanvasProjection = {
  viewport: { x: number; y: number; zoom: number };
  selectedNodeId: string;
  edgeMode: "main" | "ai-tool";
  nodes: WorkflowNode[];
  connections: WorkflowConnection[];
  interaction: CanvasInteractionState;
};

export type CanvasPoint = {
  x: number;
  y: number;
};

export type CanvasKeyboardShortcut = {
  key: string;
  action: string;
};

export type CanvasInteractionMode =
  | "idle"
  | "node-drag"
  | "canvas-pan"
  | "edge-drag"
  | "edge-reconnect"
  | "keyboard";

export type CanvasConnectionEndpoint = "source" | "target";

export type CanvasConnectionDraft = {
  pointerId: number;
  sourceNodeId: string;
  sourceOutput: WorkflowConnection["sourceOutput"];
  origin: CanvasPoint;
  lastPoint: CanvasPoint;
  validEndpointNodeIds: string[];
  edgeId?: string;
  reconnectEndpoint?: CanvasConnectionEndpoint;
  targetNodeId?: string;
  targetInput?: WorkflowConnection["targetInput"];
};

export type CanvasDragState = {
  pointerId: number;
  nodeId?: string;
  origin: CanvasPoint;
  lastPoint: CanvasPoint;
  startPosition?: CanvasPoint;
  startViewport?: CanvasPoint;
  snapToGrid: boolean;
};

export type CanvasBounds = {
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
};

export type CanvasInteractionState = {
  mode: CanvasInteractionMode;
  selectedNodeIds: string[];
  focusedNodeId?: string;
  activeDrag?: CanvasDragState;
  edgeDraft?: CanvasConnectionDraft;
  selectedConnectionId?: string;
  keyboardShortcuts: CanvasKeyboardShortcut[];
  bounds: CanvasBounds;
  canDeleteSelection: boolean;
  issue?: string;
};

export type ParameterValuePath = Array<string | number>;

export type ParameterCollectionItem = {
  key: string;
  label: string;
  itemIndex: number;
  collectionPath: ParameterValuePath;
  valuePath: ParameterValuePath;
  fields: ParameterField[];
};

export type ResourceLocatorDraftState = {
  activeMode: string;
  query: string;
  selectedValue: string;
  selectedLabel?: string;
  listSearchMethod?: string;
  searchable: boolean;
  resolvedQuery?: string;
  resolvedOptions?: ParameterOption[];
  nextPageToken?: string;
  listBoundary: {
    status: "source-only";
    providerBoundary: true;
    liveProviderExecution: false;
    issue: string;
  };
};

export type DynamicOptionsBoundary = {
  source: "n8n-type-options-routing" | "n8n-type-options-load-method";
  loadMethod: string;
  request: {
    method: string;
    url: string;
  };
  responseFilter?: string;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  issue: string;
};

export type ResourceMapperBoundary = {
  resourceMapperMethod: string;
  mode?: string;
  loadOptionsDependsOn: string[];
  fieldWords?: {
    singular?: string;
    plural?: string;
  };
  addAllFields?: boolean;
  multiKeyMatch?: boolean;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  issue: string;
};

export type ResourceMapperSchemaField = {
  id: string;
  displayName: string;
  required?: boolean;
  defaultMatch?: boolean;
  canBeUsedToMatch?: boolean;
  type?: string;
};

export type ResourceMapperSchema = {
  fields: ResourceMapperSchemaField[];
  fieldWords?: {
    singular?: string;
    plural?: string;
  };
  mode?: string;
};

export type ExpressionReferenceKind =
  | "json"
  | "node"
  | "parameter"
  | "workflow"
  | "env"
  | "credentials";

export type ExpressionReference = {
  kind: ExpressionReferenceKind;
  source: string;
  path?: string;
  sensitive: boolean;
};

export type ExpressionDiagnostic = {
  severity: "info" | "warning" | "error";
  code:
    | "secret-reference-blocked"
    | "unclosed-expression"
    | "unsupported-expression-fragment";
  message: string;
};

export type ExpressionAnalysis = {
  mode: "literal" | "full-expression" | "template-expression";
  expression: string;
  expressionBody: string;
  references: ExpressionReference[];
  diagnostics: ExpressionDiagnostic[];
};

export type ExpressionFieldState = {
  fieldName: string;
  fieldLabel: string;
  valuePath: ParameterValuePath;
  expression: string;
  expressionBody: string;
  mode: ExpressionAnalysis["mode"];
  references: ExpressionReference[];
  diagnostics: ExpressionDiagnostic[];
  previewBoundary: {
    status: "source-only";
    providerBoundary: true;
    liveProviderExecution: false;
    secretsIncluded: false;
    issue: string;
  };
};

export type ExpressionEditorState = {
  schema: "dx.n8n-studio.expression-editor";
  selectedNodeId: string;
  selectedNodeName?: string;
  nodeType?: string;
  expressionFieldCount: number;
  diagnosticCount: number;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  fields: ExpressionFieldState[];
  issue: string;
};

export type ParameterField = {
  name: string;
  label: string;
  type:
    | "string"
    | "number"
    | "boolean"
    | "json"
    | "options"
    | "multiOptions"
    | "resourceLocator"
    | "resourceMapper"
    | "expression"
    | "credentialsSelect"
    | "notice"
    | "curlImport"
    | "collection"
    | "fixedCollection"
    | "color";
  required: boolean;
  expressionEnabled: boolean;
  noDataExpression?: boolean;
  value: unknown;
  valuePath?: ParameterValuePath;
  defaultValue?: unknown;
  description?: string;
  placeholder?: string;
  options?: ParameterOption[];
  credentialTypes?: string[];
  displayOptions?: ParameterDisplayOptions;
  dynamicOptions?: DynamicOptionsBoundary;
  resourceMapper?: ResourceMapperBoundary;
  resourceLocatorModes?: ResourceLocatorMode[];
  resourceLocatorDraft?: ResourceLocatorDraftState;
  childFields?: ParameterField[];
  collectionItems?: ParameterCollectionItem[];
  renderingBoundary?: "native" | "complex-source-field";
  expressionState?: ExpressionFieldState;
};

export type CredentialPickerOption = {
  id: string;
  name: string;
  credentialType: string;
  source: "workflow-reference" | "editor-session-placeholder";
  redaction: "secret-values-never-included";
};

export type CredentialPickerBoundary = {
  status: "source-only";
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  issue: string;
};

export type CredentialReadiness = {
  nodeId: string;
  nodeName: string;
  credentialType: string;
  credentialKey: string;
  required: boolean;
  selectedCredentialId?: string;
  selectedCredentialName?: string;
  credentialOptions: CredentialPickerOption[];
  pickerBoundary: CredentialPickerBoundary;
  status: StudioReadinessStatus;
  redaction: "secret-values-never-included";
  issue: string;
};

export type EditorSessionRequestKind =
  | "dynamic-node-parameters"
  | "resource-locator-search"
  | "resource-mapper-schema"
  | "credential-list"
  | "credential-test";

export type CredentialValidationStatus = "valid" | "invalid" | "unknown";

export type EditorSessionRequestPlan = {
  kind: EditorSessionRequestKind;
  nodeId: string;
  nodeName: string;
  nodeType: string;
  status: "blocked" | "configured";
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  redaction: "secret-values-never-included";
  issue: string;
  fieldName?: string;
  fieldLabel?: string;
  loadMethod?: string;
  dynamicLoadBoundary?: string;
  query?: string;
  selectedValue?: string;
  selectedLabel?: string;
  credentialType?: string;
  credentialKey?: string;
  required?: boolean;
  selectedCredentialId?: string;
  selectedCredentialName?: string;
  credentialOptionCount?: number;
  credentialValidationStatus?: CredentialValidationStatus;
  credentialValidatedAt?: string;
  credentialValidationMessage?: string;
  resolvedOptionCount?: number;
  resolvedOptions?: ParameterOption[];
  resolvedQuery?: string;
  nextPageToken?: string;
  resolvedFieldCount?: number;
  resolvedSchema?: ResourceMapperSchema;
  resolvedCredentialOptionCount?: number;
  resolvedCredentialOptions?: CredentialPickerOption[];
};

type EditorSessionTransportResponseBase = {
  nodeId: string;
  nodeType: string;
  fieldName?: string;
  loadMethod?: string;
  dynamicLoadBoundary?: string;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  redaction: "secret-values-never-included";
  issue?: string;
};

export type EditorSessionOptionTransportResponse =
  EditorSessionTransportResponseBase & {
    kind: Extract<
      EditorSessionRequestKind,
      "dynamic-node-parameters" | "resource-locator-search"
    >;
    query?: string;
    nextPageToken?: string;
    options: ParameterOption[];
  };

export type EditorSessionResourceMapperTransportResponse =
  EditorSessionTransportResponseBase & {
    kind: "resource-mapper-schema";
    schema: ResourceMapperSchema;
  };

export type EditorSessionCredentialListTransportResponse =
  EditorSessionTransportResponseBase & {
    kind: "credential-list";
    credentialType: string;
    credentialKey?: string;
    selectedCredentialId?: string;
    selectedCredentialName?: string;
    credentialOptions: CredentialPickerOption[];
  };

export type EditorSessionCredentialTestTransportResponse =
  EditorSessionTransportResponseBase & {
    kind: "credential-test";
    credentialType: string;
    credentialKey?: string;
    selectedCredentialId?: string;
    validationStatus: CredentialValidationStatus;
    validatedAt?: string;
    message?: string;
  };

export type EditorSessionTransportResponse =
  | EditorSessionOptionTransportResponse
  | EditorSessionResourceMapperTransportResponse
  | EditorSessionCredentialListTransportResponse
  | EditorSessionCredentialTestTransportResponse;

export type EditorSessionReadiness = {
  schema: "dx.n8n-studio.editor-session";
  status: StudioReadinessStatus;
  selectedNodeId: string;
  selectedNodeName?: string;
  nodeType?: string;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  redaction: "secret-values-never-included";
  dynamicParameterLoadCount: number;
  resourceLocatorSearchCount: number;
  resourceMapperRequestCount: number;
  credentialRequestCount: number;
  credentialValidationRequestCount: number;
  fulfilledRequestCount: number;
  hiddenFieldCount: number;
  requestPlans: EditorSessionRequestPlan[];
  issue: string;
};

export type ResourceLocatorState = {
  mode: "list" | "id" | "url";
  status: StudioReadinessStatus;
  query: string;
  selectedResourceId?: string;
  nextPageToken?: string;
  issue: string;
};

export type PinnedDataState = {
  nodeName: string;
  itemCount: number;
  status: StudioReadinessStatus;
  sizePolicy: "validated-before-save";
};

export type ExecutionDebugView = "validation" | "runs" | "logs" | "receipts";

export type ExecutionAttemptStatus =
  | "blocked"
  | "queued"
  | "running"
  | "success"
  | "error"
  | "cancelled";

export type ExecutionAttemptSummary = {
  id: string;
  workflowId: string;
  workflowName: string;
  mode: "manual" | "partial" | "webhook";
  status: ExecutionAttemptStatus;
  triggerNodeId?: string;
  selectedNodeId?: string;
  startedAt?: string;
  finishedAt?: string;
  durationMs?: number;
  inputItemCount: number;
  outputItemCount: number;
  receiptPath: string;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  issue: string;
};

export type ExecutionNodeLogStatus =
  | "success"
  | "ready"
  | "blocked"
  | "waiting"
  | "skipped"
  | "error";

export type ExecutionNodeLogRow = {
  id: string;
  attemptId: string;
  nodeId: string;
  nodeName: string;
  nodeType: string;
  status: ExecutionNodeLogStatus;
  inputItemCount: number;
  outputItemCount: number;
  durationMs?: number;
  dataPreviewLabel: string;
  redaction: "secret-values-never-included";
  providerErrorMessage?: string;
  issue: string;
};

export type ExecutionReceiptBoundary = {
  providerBoundary: true;
  liveProviderExecution: false;
  executionReceiptImported: boolean;
  secretsIncluded: false;
  receiptRoot: string;
  receiptPath?: string;
  importedAt?: string;
  issue: string;
};

export type ExecutionReceiptImportIssue = {
  code:
    | "unknown-node-log"
    | "secret-field-stripped"
    | "workflow-id-mismatch"
    | "malformed-node-log";
  severity: "warning" | "blocker";
  message: string;
  nodeName?: string;
};

export type ExecutionReadiness = {
  status: StudioReadinessStatus;
  providerBoundary: true;
  liveProviderExecution: false;
  activeDebugView: ExecutionDebugView;
  debugViews: ExecutionDebugView[];
  availableActions: string[];
  selectedAttemptId: string;
  attempts: ExecutionAttemptSummary[];
  nodeLogs: ExecutionNodeLogRow[];
  receiptBoundary: ExecutionReceiptBoundary;
  receiptIssues: ExecutionReceiptImportIssue[];
  blockedReason: string;
};

export type AiToolState = {
  status: StudioReadinessStatus;
  focusedNodeIds: string[];
  toolLifecycle: Array<"pending" | "running" | "suspended" | "done" | "cancelled" | "error">;
};

export type ImportSource = "clipboard" | "file" | "url";

export type ImportSourceOption = {
  source: ImportSource;
  label: string;
  status: "source-only";
  providerBoundary: true;
  liveProviderExecution: false;
  issue: string;
};

export type ImportExportBoundary = {
  status: "source-only";
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  executableAfterImport: false;
  issue: string;
};

export type ImportPreviewStatus =
  | "awaiting-input"
  | "sanitized"
  | "sanitized-with-issues"
  | "blocked";

export type ImportPreviewIssueRow = {
  code: string;
  message: string;
  nodeName?: string;
  severity: "blocker" | "warning";
  action: string;
};

export type SanitizedImportPreview = {
  status: ImportPreviewStatus;
  source?: ImportSource;
  workflowName?: string;
  keptNodeCount: number;
  droppedIssueCount: number;
  strippedSecretCount: number;
  connectionCount: number;
  pinDataNodeCount: number;
  regeneratedWebhookCount: number;
  sanitizedFields: string[];
  issues: ImportPreviewIssueRow[];
  sanitizedDocument?: WorkflowDocument;
  boundary: ImportExportBoundary;
};

export type ImportDraftStatus =
  | "awaiting-preview"
  | "ready-to-apply"
  | "blocked"
  | "applied"
  | "saved";

export type ImportDraftState = {
  status: ImportDraftStatus;
  canApplyPreview: boolean;
  canSaveDraft: boolean;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  persistedToDisk: false;
  editorSessionOnly: true;
  saveReceiptPath: ".dx/receipts/n8n-studio/import/latest.sr";
  issue: string;
  appliedAt?: string;
  savedAt?: string;
  lastAppliedWorkflowId?: string;
  lastSavedWorkflowId?: string;
};

export type ExportReceiptDetail = {
  schema: "dx.n8n-studio.export.receipt";
  format: "n8n-workflow-json";
  workflowName: string;
  nodeCount: number;
  connectionCount: number;
  pinnedNodeCount: number;
  credentialReferenceCount: number;
  routePath: "/api/n8n-studio/export";
  receiptPath: string;
  downloadName: string;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  redaction: "secret-values-never-included";
  status: "source-only";
  issue: string;
};

export type CurrentWorkflowExportStatus =
  | "idle"
  | "exporting"
  | "exported"
  | "failed";

export type CurrentWorkflowExportState = {
  status: CurrentWorkflowExportStatus;
  routePath: "/api/n8n-studio/export";
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  redaction: "secret-values-never-included";
  issue: string;
  exportedAt?: string;
  responseSchema?: "dx.n8n-studio.export";
  workflowName?: string;
  downloadName?: string;
  nodeCount?: number;
  connectionCount?: number;
  credentialReferenceCount?: number;
  errorMessage?: string;
};

export type ImportExportState = {
  importSources: ImportSourceOption[];
  exportFormat: "n8n-workflow-json";
  sanitizedFields: string[];
  importPreview: SanitizedImportPreview;
  draft: ImportDraftState;
  exportReceipt: ExportReceiptDetail;
  currentExport: CurrentWorkflowExportState;
};

export type ReceiptSummary = {
  schema: "dx.n8n-studio.receipts";
  receiptRoot: string;
  providerBoundary: true;
  liveProviderExecution: false;
  redaction: "secret-values-never-included";
  surfaces: StudioSurface[];
};

export type N8nStudioState = {
  catalog: CatalogSummary;
  document: WorkflowDocument;
  canvas: CanvasProjection;
  parameters: ParameterField[];
  expressionEditor: ExpressionEditorState;
  credentials: CredentialReadiness[];
  editorSession: EditorSessionReadiness;
  resourceLocator: ResourceLocatorState;
  pinnedData: PinnedDataState[];
  execution: ExecutionReadiness;
  aiTools: AiToolState;
  importExport: ImportExportState;
  receipts: ReceiptSummary;
};
