export type ChartPrimitive = string | number | boolean | Date | null;

export type Datum = Record<string, ChartPrimitive>;

export type FieldType = "quantitative" | "temporal" | "ordinal" | "nominal";

export type ChartTask =
  | "comparison"
  | "trend"
  | "distribution"
  | "proportion"
  | "relation"
  | "flow"
  | "map"
  | "table"
  | "mobile"
  | "ai"
  | "composition";

export type ChartFamily =
  | "G2"
  | "G2Plot"
  | "F2"
  | "S2"
  | "G6"
  | "X6"
  | "L7"
  | "L7Plot"
  | "Graphin"
  | "G"
  | "DataSet"
  | "AVA"
  | "GPTVis"
  | "MCP"
  | "ChartSkills"
  | "AntDesignCharts"
  | "AntDesignPlots"
  | "AntDesignGraphs";

export type Channel =
  | "x"
  | "y"
  | "color"
  | "series"
  | "size"
  | "theta"
  | "low"
  | "q1"
  | "median"
  | "q3"
  | "high"
  | "source"
  | "target"
  | "parent"
  | "label"
  | "tooltip";

export type MarkKind =
  | "bar"
  | "line"
  | "area"
  | "point"
  | "rule"
  | "heatmap"
  | "pie"
  | "radar"
  | "gauge"
  | "boxplot"
  | "bullet"
  | "funnel"
  | "waterfall"
  | "treemap"
  | "sunburst"
  | "sankey"
  | "chord"
  | "graph"
  | "map"
  | "pivot"
  | "wordcloud";

export type ScaleKind = "linear" | "band" | "point" | "time" | "ordinal";

export type ValueFormat = "number" | "compact" | "percent" | "currency" | "date";

export interface FieldEncoding {
  field: string;
  type: FieldType;
  label?: string;
  format?: ValueFormat;
  scale?: ScaleKind;
}

export interface MarkStyle {
  fill?: string;
  stroke?: string;
  opacity?: number;
  strokeWidth?: number;
  radius?: number;
}

export interface WaterfallMarkOptions {
  totalField?: string;
}

export type ChordNodeSort = "name-asc" | "weight-asc" | "weight-desc";

export interface ChordMarkOptions {
  nodePaddingRatio?: number;
  nodeWidthRatio?: number;
  nodeSort?: ChordNodeSort;
}

export interface MarkSpec {
  id: string;
  type: MarkKind;
  encoding: Partial<Record<Channel, FieldEncoding>>;
  transforms?: TransformSpec[];
  style?: MarkStyle;
  stacked?: boolean;
  waterfall?: WaterfallMarkOptions;
  chord?: ChordMarkOptions;
}

export type ReducerKind = "count" | "sum" | "mean" | "min" | "max";

export type TransformSpec =
  | {
      type: "filter";
      field: string;
      equals?: ChartPrimitive;
      min?: number;
      max?: number;
    }
  | {
      type: "sort";
      field: string;
      order?: "asc" | "desc";
    }
  | {
      type: "group";
      groupBy: string[];
      field?: string;
      as: string;
      reducer: ReducerKind;
    }
  | {
      type: "bin";
      field: string;
      as: string;
      valueAs: string;
      count?: number;
    }
  | {
      type: "stackY";
      x: string;
      y: string;
      series?: string;
    }
  | {
      type: "normalizeY";
      x: string;
      y: string;
      series?: string;
    }
  | {
      type: "dodgeX";
      x: string;
      series: string;
    };

export interface DataSetTransformStepSpec {
  id: string;
  label: string;
  transform: TransformSpec;
  description?: string;
}

export interface DataSetStageSpec {
  id: string;
  label: string;
  rowCount: number;
  transform?: TransformSpec;
  rows: Datum[];
}

export interface DataSetViewSpec {
  id: string;
  label: string;
  sourceLabel: string;
  sourceRows: Datum[];
  steps: DataSetTransformStepSpec[];
}

export type MobileGestureKind = "tap-tooltip" | "pan-x" | "pinch-zoom" | "tap-select";

export interface MobileViewportSpec {
  width: number;
  height: number;
  pixelRatio?: number;
  safeArea?: "none" | "compact" | "ios";
}

export interface MobileGestureSpec {
  type: MobileGestureKind;
  enabled?: boolean;
}

export interface MobileChartSpec {
  viewport: MobileViewportSpec;
  gestures?: MobileGestureSpec[];
  snap?: "nearest" | "none";
}

export type DiagramPortGroup = "input" | "output" | "control";
export type DiagramPortPosition = "left" | "right" | "top" | "bottom";
export type DiagramPortStateKind = "available" | "connected" | "active" | "disabled";
export type DiagramRouterKind = "normal" | "orth" | "manhattan";
export type DiagramConnectorKind = "normal" | "smooth" | "rounded" | "jumpover";

export interface DiagramPortSpec {
  id: string;
  group: DiagramPortGroup;
  label?: string;
  position?: DiagramPortPosition;
  state?: DiagramPortStateKind;
}

export interface DiagramNodeSpec {
  id: string;
  label: string;
  shape?: "rect" | "rounded-rect";
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  ports?: DiagramPortSpec[];
}

export interface DiagramTerminalSpec {
  cell: string;
  port?: string;
}

export interface DiagramRoutingSpec {
  name: DiagramRouterKind;
  padding?: number;
}

export interface DiagramConnectorSpec {
  name: DiagramConnectorKind;
  radius?: number;
}

export interface DiagramEdgeSpec {
  id: string;
  source: DiagramTerminalSpec;
  target: DiagramTerminalSpec;
  label?: string;
  vertices?: Array<{ x: number; y: number }>;
  router?: DiagramRoutingSpec;
  connector?: DiagramConnectorSpec;
}

export interface DiagramInteractionSpec {
  type: "select-node" | "connect-port" | "drag-node" | "pan-canvas" | "zoom-canvas";
  enabled?: boolean;
}

export interface DiagramModelSpec {
  nodes: DiagramNodeSpec[];
  edges: DiagramEdgeSpec[];
  interactions?: DiagramInteractionSpec[];
}

export type CoordinateKind = "cartesian" | "transpose" | "polar" | "theta";

export interface CoordinateSpec {
  type: CoordinateKind;
  innerRadius?: number;
  startAngle?: number;
  endAngle?: number;
}

export interface ViewRegionSpec {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface ViewCompositionChildSpec {
  id: string;
  title?: string;
  region: ViewRegionSpec;
  spec: ChartSpec;
}

export type FacetCompositionSpec = {
  type: "facet";
  field: string;
  columns?: number;
  label?: string;
};

export type ViewCompositionSpec = {
  type: "view";
  children: ViewCompositionChildSpec[];
};

export type CompositionSpec = FacetCompositionSpec | ViewCompositionSpec;

export type TableHierarchyKind = "grid" | "tree";

export interface TableFieldSpec {
  field: string;
  label?: string;
  type?: FieldType;
  width?: number;
}

export interface TableMeasureSpec {
  field: string;
  label?: string;
  format?: ValueFormat;
  reducer?: ReducerKind;
}

export interface TableTotalsSpec {
  row?: "none" | "right";
  column?: "none" | "bottom";
  grand?: boolean;
}

export type TableSortTarget = "row" | "column";

export interface TableSortSpec {
  target: TableSortTarget;
  field?: string;
  valueField?: string;
  order?: "asc" | "desc";
}

export interface TableDrillSpec {
  field: string;
  value: ChartPrimitive;
  path: ChartPrimitive[];
  expanded?: boolean;
}

export interface TableFilterSpec {
  field: string;
  equals?: ChartPrimitive;
  min?: number;
  max?: number;
}

export interface TableInteractionSpec {
  type: "cell-hover" | "brush-selection" | "sort-header" | "drill-down";
  enabled?: boolean;
  sort?: TableSortSpec;
  drill?: TableDrillSpec;
}

export interface TableSheetSpec {
  rows: TableFieldSpec[];
  columns: TableFieldSpec[];
  values: TableMeasureSpec[];
  hierarchyType?: TableHierarchyKind;
  totals?: TableTotalsSpec;
  interactions?: TableInteractionSpec[];
  sort?: TableSortSpec[];
  drillDown?: TableDrillSpec[];
  filterRows?: TableFilterSpec[];
}

export type GeoProjectionKind = "equirectangular" | "mercator-lite";

export interface GeoViewportSpec {
  center: [number, number];
  zoom: number;
  minZoom?: number;
  maxZoom?: number;
  pitch?: number;
  bearing?: number;
}

export interface GeoBasemapSpec {
  type: "none" | "token-land" | "graticule";
  labels?: boolean;
}

export interface GeoLayerStyleSpec {
  fill?: string;
  stroke?: string;
  opacity?: number;
  strokeWidth?: number;
  radiusScale?: number;
  className?: string;
}

export type GeoLayerBlendMode = "normal" | "additive" | "subtractive" | "max";

export interface GeoLayerStateSpec {
  active?: boolean;
  selected?: boolean;
}

export interface GeoLegendSpec {
  channel: "layer" | "color" | "size";
  title?: string;
  position?: "top-left" | "top-right" | "bottom-left" | "bottom-right";
  enabled?: boolean;
}

export interface GeoLayerSpec {
  id: string;
  type: "point" | "bubble" | "heatmap" | "region";
  encoding?: Partial<Record<Channel, FieldEncoding>>;
  regionField?: string;
  style?: GeoLayerStyleSpec;
  interactions?: GeoInteractionSpec[];
  visible?: boolean;
  minZoom?: number;
  maxZoom?: number;
  autoFit?: boolean;
  blend?: GeoLayerBlendMode;
  state?: GeoLayerStateSpec;
  legend?: GeoLegendSpec;
  zIndex?: number;
}

export type GeoLayerRuntimeSpec = Pick<GeoLayerSpec, "minZoom" | "maxZoom" | "autoFit" | "blend" | "state" | "legend">;

export interface GeoRegionSpec {
  id: string;
  label?: string;
  points: Array<[number, number]>;
}

export interface GeoInteractionSpec {
  type: "pan" | "zoom" | "tooltip" | "select-feature";
  enabled?: boolean;
}

export interface GeoMapSpec {
  projection?: GeoProjectionKind;
  viewport?: GeoViewportSpec;
  basemap?: GeoBasemapSpec;
  regions?: GeoRegionSpec[];
  layers?: GeoLayerSpec[];
  legends?: GeoLegendSpec[];
  interactions?: GeoInteractionSpec[];
}

export interface ChartAdviceIntentSpec {
  task: ChartTask;
  recordCount: number;
  dimensions: string[];
  measures: string[];
  hasTime?: boolean;
  hasGeo?: boolean;
  hasNetwork?: boolean;
  hasHierarchy?: boolean;
}

export interface ChartAdviceCandidateSpec {
  id: string;
  label: string;
  mark: MarkKind;
  strengths: ChartTask[];
}

export interface ChartAdviceRecommendationSpec {
  choice: string;
  confidence: number;
  reason: string;
  ruleId: string;
  rationale: string;
}

export interface ChartAdviceModelSpec {
  intent: ChartAdviceIntentSpec;
  candidates?: ChartAdviceCandidateSpec[];
  maxRecommendations?: number;
}

export type ChartPromptFieldRole = "dimension" | "measure" | "time" | "geo" | "network" | "hierarchy";

export interface ChartPromptFieldSpec {
  name: string;
  role: ChartPromptFieldRole;
  label?: string;
}

export interface ChartPromptRequestSpec {
  id: string;
  prompt: string;
  fields: ChartPromptFieldSpec[];
  recordCount: number;
  sampleRows?: Datum[];
  task?: ChartTask;
  outputFamily?: ChartFamily;
  maxRecommendations?: number;
}

export interface PromptChartMaterializationSpec {
  request: ChartPromptRequestSpec;
  intent: ChartAdviceIntentSpec;
  recommendations: ChartAdviceRecommendationSpec[];
}

export type GPTVisChartType =
  | "area"
  | "bar"
  | "boxplot"
  | "column"
  | "funnel"
  | "histogram"
  | "line"
  | "network-graph"
  | "pie"
  | "radar"
  | "chord"
  | "sankey"
  | "scatter"
  | "table"
  | "treemap"
  | "waterfall"
  | "word-cloud";

export type GptVisChartType = GPTVisChartType;

export interface ChartToolSpec {
  id: string;
  label: string;
  chartType: GPTVisChartType;
  mark: MarkKind;
  family: "GPTVis";
  tasks: ChartTask[];
  requiredRoles?: ChartPromptFieldRole[];
  preferredRoles?: ChartPromptFieldRole[];
  aliases?: string[];
  supported: boolean;
}

export interface ChartToolRecommendationSpec {
  toolId: string;
  choice: string;
  chartType: GPTVisChartType;
  mark: MarkKind;
  confidence: number;
  reason: string;
  ruleId: string;
  rationale: string;
}

export interface ChartToolRouteSpec {
  requestId: string;
  prompt: string;
  intent: ChartAdviceIntentSpec;
  selected: ChartToolRecommendationSpec;
  recommendations: ChartToolRecommendationSpec[];
  generatedSpecId: string;
  syntax: string;
}

export interface AgentChartRequestSpec {
  id: string;
  prompt: string;
  task: ChartTask;
  inputFields: string[];
  outputFamily?: ChartFamily;
}

export interface ChartSkillSpec {
  id: string;
  label: string;
  task: ChartTask;
  family: ChartFamily;
  produces: MarkKind[];
  dependsOn?: string[];
}

export interface AgentChartWorkflowSpec {
  request: AgentChartRequestSpec;
  skills: ChartSkillSpec[];
}

export type GraphLayoutKind = "circular" | "grid" | "radial" | "dagre-lite" | "combo-cluster";

export interface GraphNodeSpec {
  id: string;
  label?: string;
  combo?: string;
  type?: string;
  value?: number;
}

export interface GraphEdgeSpec {
  id?: string;
  source: string;
  target: string;
  label?: string;
  relation?: string;
  weight?: number;
}

export interface GraphComboSpec {
  id: string;
  label?: string;
  parent?: string;
}

export interface GraphLayoutSpec {
  type: GraphLayoutKind;
  rankDirection?: "TB" | "LR";
  radius?: number;
}

export interface GraphFocusSpec {
  nodeId: string;
  relationDepth?: number;
}

export interface GraphRelationActivationSpec {
  nodeId?: string;
  edgeIds?: string[];
  relation?: string;
}

export interface GraphBehaviorSpec {
  type: "drag-node" | "drag-canvas" | "zoom-canvas" | "focus-node" | "activate-relations";
  enabled?: boolean;
  focus?: GraphFocusSpec;
  activation?: GraphRelationActivationSpec;
}

export interface GraphPluginSpec {
  type: "legend" | "minimap" | "tooltip" | "context-panel";
  enabled?: boolean;
  position?: "top-left" | "top-right" | "bottom-left" | "bottom-right";
  target?: "canvas" | "selection";
}

export interface GraphModelSpec {
  nodes: GraphNodeSpec[];
  edges: GraphEdgeSpec[];
  combos?: GraphComboSpec[];
  layout?: GraphLayoutSpec;
  behaviors?: GraphBehaviorSpec[];
  plugins?: GraphPluginSpec[];
}

export interface AxisSpec {
  channel: "x" | "y";
  title?: string;
  grid?: boolean;
  ticks?: number;
}

export interface LegendSpec {
  channel: "color" | "series";
  title?: string;
}

export interface ChartPadding {
  top: number;
  right: number;
  bottom: number;
  left: number;
}

export interface ChartSpec {
  id: string;
  title: string;
  description: string;
  task: ChartTask;
  family: ChartFamily;
  width: number;
  height: number;
  data: Datum[];
  marks: MarkSpec[];
  axes?: AxisSpec[];
  legend?: LegendSpec;
  composition?: CompositionSpec;
  coordinate?: CoordinateSpec;
  table?: TableSheetSpec;
  map?: GeoMapSpec;
  graph?: GraphModelSpec;
  advice?: ChartAdviceModelSpec;
  router?: ChartToolRouteSpec;
  dataset?: DataSetViewSpec;
  mobile?: MobileChartSpec;
  diagram?: DiagramModelSpec;
  padding?: Partial<ChartPadding>;
}

export interface ChartCatalogItem {
  slug: string;
  title: string;
  task: ChartTask;
  family: ChartFamily;
  summary: string;
  whenToUse: string;
  avoidWhen: string;
  dataShape: string;
  encodingNotes: string[];
  spec: ChartSpec;
}

export const DEFAULT_PADDING: ChartPadding = {
  top: 30,
  right: 28,
  bottom: 46,
  left: 54,
};
