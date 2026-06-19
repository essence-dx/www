type SceneMetadata = {
  transform?: string;
  className?: string;
  coordinateType?: string;
  coordinateOptions?: string;
  viewId?: string;
  viewTitle?: string;
  viewRegion?: string;
  graphNodeId?: string;
  graphEdgeId?: string;
  graphComboId?: string;
  graphLayout?: string;
  graphBehaviors?: string;
  graphPlugins?: string;
  graphFocusNodeId?: string;
  graphRelationState?: string;
  chordSource?: string;
  chordTarget?: string;
  chordWeight?: string;
  chordNodeId?: string;
  tableCellId?: string;
  tableRowKey?: string;
  tableColumnKey?: string;
  tableValueField?: string;
  tableSection?: string;
  tableHierarchy?: string;
  tableInteractions?: string;
  tableTotals?: string;
  tableSortState?: string;
  tableDrillPath?: string;
  mapLayerId?: string;
  mapLayerType?: string;
  mapFeatureId?: string;
  mapProjection?: string;
  mapViewport?: string;
  mapInteractions?: string;
  mapLayerZoom?: string;
  mapLayerAutoFit?: string;
  mapLayerBlend?: string;
  mapLayerState?: string;
  mapLegend?: string;
  adviceRuleId?: string;
  adviceReason?: string;
  adviceConfidence?: string;
  dataSetStageId?: string;
  dataSetStageName?: string;
  dataSetTransform?: string;
  dataSetRowCount?: string;
  mobileViewport?: string;
  mobileGestures?: string;
  mobilePixelRatio?: string;
  mobileSafeArea?: string;
  wordCloudTerm?: string;
  wordCloudWeight?: string;
  diagramNodeId?: string;
  diagramPortId?: string;
  diagramEdgeId?: string;
  diagramRouter?: string;
  diagramConnector?: string;
  diagramInteractions?: string;
  diagramPortState?: string;
  diagramPortGroup?: string;
  diagramPortPosition?: string;
};

type RectElement = SceneMetadata & {
  kind: "rect";
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  fill?: string;
  stroke?: string;
  opacity?: number;
  radius?: number;
  label?: string;
};

type LineElement = SceneMetadata & {
  kind: "line";
  id: string;
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  stroke?: string;
  strokeWidth?: number;
  opacity?: number;
  label?: string;
};

type PathElement = SceneMetadata & {
  kind: "path";
  id: string;
  d: string;
  fill?: string;
  stroke?: string;
  strokeWidth?: number;
  opacity?: number;
  label?: string;
};

type CircleElement = SceneMetadata & {
  kind: "circle";
  id: string;
  cx: number;
  cy: number;
  r: number;
  fill?: string;
  stroke?: string;
  opacity?: number;
  label?: string;
};

type TextElement = SceneMetadata & {
  kind: "text";
  id: string;
  x: number;
  y: number;
  text: string;
  anchor?: "start" | "middle" | "end";
  fill?: string;
  fontSize?: number;
  fontWeight?: number;
  label?: string;
};

type PolygonElement = SceneMetadata & {
  kind: "polygon";
  id: string;
  points: string;
  fill?: string;
  stroke?: string;
  opacity?: number;
  label?: string;
};

export type SceneElement = RectElement | LineElement | PathElement | CircleElement | TextElement | PolygonElement;

export interface LegendItem {
  label: string;
  color: string;
}

export interface ChartScene {
  id: string;
  title: string;
  description: string;
  width: number;
  height: number;
  elements: SceneElement[];
  legend: LegendItem[];
  summary: string;
}
