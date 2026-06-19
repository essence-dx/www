import {
  DEFAULT_WHITEBOARD_STYLE,
  makeElementId,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardMetadata,
  type WhiteboardPoint,
  type WhiteboardStyle,
} from "./model";
import {
  createDiamondElement,
  createEllipseElement,
  createRectangleElement,
  createTextElement,
} from "./scene";

export type WhiteboardTemplatePresetId =
  | "flowchart-basic"
  | "kanban-board"
  | "retrospective-board"
  | "system-map";

export function createTemplatePresetElements(options: {
  readonly preset: WhiteboardTemplatePresetId;
  readonly origin: WhiteboardPoint;
  readonly idPrefix: string;
  readonly now: string;
  readonly style: Partial<WhiteboardStyle>;
}): readonly WhiteboardElement[] {
  switch (options.preset) {
    case "flowchart-basic":
      return flowchartTemplate(options);
    case "kanban-board":
      return kanbanTemplate(options);
    case "retrospective-board":
      return retrospectiveTemplate(options);
    case "system-map":
      return systemMapTemplate(options);
  }
}

function flowchartTemplate(options: TemplateFactoryOptions): readonly WhiteboardElement[] {
  const { idPrefix: prefix, origin, now, style } = options;
  const startId = `${prefix}-start`;
  const decisionId = `${prefix}-decision`;
  const approveId = `${prefix}-approve`;
  const reviseId = `${prefix}-revise`;

  return [
    box(startId, "Start", origin.x, origin.y, 180, 72, "#ecfeff", "#0891b2", options, "start"),
    label(`${prefix}-start-label`, "Start", origin.x + 40, origin.y + 22, options, "start-label"),
    createDiamondElement({
      id: decisionId,
      role: "shape",
      name: "Decision",
      x: origin.x + 260,
      y: origin.y - 18,
      width: 188,
      height: 112,
      style: templateStyle("#f5f3ff", "#7c3aed", "#2e1065", style),
      createdAt: now,
      updatedAt: now,
      metadata: templateMetadata(options.preset, "decision"),
    }),
    label(`${prefix}-decision-label`, "Ready?", origin.x + 314, origin.y + 22, options, "decision-label"),
    box(`${prefix}-approve`, "Approved path", origin.x + 540, origin.y - 80, 196, 76, "#ecfdf5", "#059669", options, "approved-outcome"),
    label(`${prefix}-approve-label`, "Ship", origin.x + 606, origin.y - 54, options, "approved-label"),
    box(`${prefix}-revise`, "Revision path", origin.x + 540, origin.y + 92, 196, 76, "#fff7ed", "#ea580c", options, "revision-outcome"),
    label(`${prefix}-revise-label`, "Revise", origin.x + 604, origin.y + 118, options, "revision-label"),
    boundArrow(`${prefix}-start-to-decision`, startId, decisionId, origin.x + 180, origin.y + 36, origin.x + 260, origin.y + 38, options),
    boundArrow(`${prefix}-decision-to-approve`, decisionId, approveId, origin.x + 448, origin.y + 28, origin.x + 540, origin.y - 42, options),
    boundArrow(`${prefix}-decision-to-revise`, decisionId, reviseId, origin.x + 448, origin.y + 42, origin.x + 540, origin.y + 130, options),
  ];
}

function kanbanTemplate(options: TemplateFactoryOptions): readonly WhiteboardElement[] {
  const { idPrefix: prefix, origin, now, style } = options;
  const frameId = `${prefix}-frame`;
  const columns = [
    { id: "todo", label: "To do", x: 28, fill: "#eff6ff", stroke: "#2563eb", card: "Backlog item" },
    { id: "doing", label: "Doing", x: 252, fill: "#fefce8", stroke: "#ca8a04", card: "In progress" },
    { id: "done", label: "Done", x: 476, fill: "#f0fdf4", stroke: "#16a34a", card: "Released" },
  ] as const;

  return [
    createRectangleElement({
      id: frameId,
      role: "frame",
      name: "Kanban board",
      x: origin.x,
      y: origin.y,
      width: 704,
      height: 368,
      style: templateStyle("transparent", "#52525b", "#18181b", style, "dashed"),
      createdAt: now,
      updatedAt: now,
      metadata: templateMetadata(options.preset, "frame"),
    }),
    label(`${prefix}-title`, "Kanban", origin.x + 28, origin.y + 20, options, "title", frameId),
    ...columns.flatMap((column) => [
      createRectangleElement({
        id: `${prefix}-${column.id}-column`,
        role: "shape",
        name: `${column.label} column`,
        x: origin.x + column.x,
        y: origin.y + 76,
        width: 200,
        height: 260,
        style: templateStyle(column.fill, column.stroke, "#111827", style),
        createdAt: now,
        updatedAt: now,
        metadata: childMetadata(options.preset, `${column.id}-column`, frameId),
      }),
      label(`${prefix}-${column.id}-header`, column.label, origin.x + column.x + 18, origin.y + 94, options, `${column.id}-header`, frameId),
      stickyCard(`${prefix}-${column.id}-card`, column.card, origin.x + column.x + 18, origin.y + 148, options, `${column.id}-card`, frameId),
    ]),
  ];
}

function retrospectiveTemplate(options: TemplateFactoryOptions): readonly WhiteboardElement[] {
  const { idPrefix: prefix, origin, now, style } = options;
  const frameId = `${prefix}-frame`;
  const quadrants = [
    { id: "went-well", label: "Went well", x: 32, y: 76, fill: "#f0fdf4", stroke: "#16a34a" },
    { id: "improve", label: "Improve", x: 344, y: 76, fill: "#fff7ed", stroke: "#ea580c" },
    { id: "questions", label: "Questions", x: 32, y: 268, fill: "#eff6ff", stroke: "#2563eb" },
    { id: "actions", label: "Actions", x: 344, y: 268, fill: "#f5f3ff", stroke: "#7c3aed" },
  ] as const;

  return [
    createRectangleElement({
      id: frameId,
      role: "frame",
      name: "Retrospective board",
      x: origin.x,
      y: origin.y,
      width: 640,
      height: 468,
      style: templateStyle("transparent", "#52525b", "#18181b", style, "dashed"),
      createdAt: now,
      updatedAt: now,
      metadata: templateMetadata(options.preset, "frame"),
    }),
    label(`${prefix}-title`, "Retrospective", origin.x + 32, origin.y + 24, options, "title", frameId),
    ...quadrants.flatMap((quadrant) => [
      createRectangleElement({
        id: `${prefix}-${quadrant.id}-zone`,
        role: "shape",
        name: quadrant.label,
        x: origin.x + quadrant.x,
        y: origin.y + quadrant.y,
        width: 264,
        height: 156,
        style: templateStyle(quadrant.fill, quadrant.stroke, "#111827", style),
        createdAt: now,
        updatedAt: now,
        metadata: childMetadata(options.preset, `${quadrant.id}-zone`, frameId),
      }),
      label(`${prefix}-${quadrant.id}-label`, quadrant.label, origin.x + quadrant.x + 20, origin.y + quadrant.y + 18, options, `${quadrant.id}-label`, frameId),
      stickyCard(`${prefix}-${quadrant.id}-note`, "Add note", origin.x + quadrant.x + 20, origin.y + quadrant.y + 68, options, `${quadrant.id}-note`, frameId),
    ]),
  ];
}

function systemMapTemplate(options: TemplateFactoryOptions): readonly WhiteboardElement[] {
  const { idPrefix: prefix, origin } = options;
  const clientId = `${prefix}-client`;
  const apiId = `${prefix}-api`;
  const workerId = `${prefix}-worker`;
  const queueId = `${prefix}-queue`;
  const databaseId = `${prefix}-database`;

  return [
    box(clientId, "Client app", origin.x, origin.y + 96, 180, 92, "#eff6ff", "#2563eb", options, "client"),
    label(`${prefix}-client-label`, "Client", origin.x + 54, origin.y + 126, options, "client-label"),
    box(apiId, "API service", origin.x + 280, origin.y + 88, 196, 108, "#f5f3ff", "#7c3aed", options, "api"),
    label(`${prefix}-api-label`, "API", origin.x + 352, origin.y + 126, options, "api-label"),
    box(workerId, "Worker", origin.x + 580, origin.y, 196, 92, "#ecfeff", "#0891b2", options, "worker"),
    label(`${prefix}-worker-label`, "Worker", origin.x + 640, origin.y + 30, options, "worker-label"),
    box(queueId, "Queue", origin.x + 580, origin.y + 178, 196, 92, "#fff7ed", "#ea580c", options, "queue"),
    label(`${prefix}-queue-label`, "Queue", origin.x + 642, origin.y + 208, options, "queue-label"),
    createEllipseElement({
      id: databaseId,
      role: "shape",
      name: "Database",
      x: origin.x + 880,
      y: origin.y + 88,
      width: 180,
      height: 108,
      style: templateStyle("#f0fdf4", "#16a34a", "#064e3b", options.style),
      createdAt: options.now,
      updatedAt: options.now,
      metadata: templateMetadata(options.preset, "database"),
    }),
    label(`${prefix}-database-label`, "Database", origin.x + 922, origin.y + 126, options, "database-label"),
    boundArrow(`${prefix}-client-to-api`, clientId, apiId, origin.x + 180, origin.y + 142, origin.x + 280, origin.y + 142, options),
    boundArrow(`${prefix}-api-to-worker`, apiId, workerId, origin.x + 476, origin.y + 114, origin.x + 580, origin.y + 46, options),
    boundArrow(`${prefix}-api-to-queue`, apiId, queueId, origin.x + 476, origin.y + 170, origin.x + 580, origin.y + 224, options),
    boundArrow(`${prefix}-worker-to-database`, workerId, databaseId, origin.x + 776, origin.y + 46, origin.x + 880, origin.y + 120, options),
    boundArrow(`${prefix}-queue-to-database`, queueId, databaseId, origin.x + 776, origin.y + 224, origin.x + 880, origin.y + 164, options),
  ];
}

type TemplateFactoryOptions = {
  readonly preset: WhiteboardTemplatePresetId;
  readonly origin: WhiteboardPoint;
  readonly idPrefix: string;
  readonly now: string;
  readonly style: Partial<WhiteboardStyle>;
};

function box(
  id: string,
  name: string,
  x: number,
  y: number,
  width: number,
  height: number,
  fill: string,
  stroke: string,
  options: TemplateFactoryOptions,
  templateRole: string,
): WhiteboardElement {
  return createRectangleElement({
    id,
    role: "shape",
    name,
    x,
    y,
    width,
    height,
    style: templateStyle(fill, stroke, "#111827", options.style),
    createdAt: options.now,
    updatedAt: options.now,
    metadata: templateMetadata(options.preset, templateRole),
  });
}

function label(
  id: string,
  text: string,
  x: number,
  y: number,
  options: TemplateFactoryOptions,
  templateRole: string,
  frameId?: string | WhiteboardElementId,
): WhiteboardElement {
  return createTextElement({
    id,
    role: "label",
    name: text,
    x,
    y,
    width: Math.max(96, text.length * 12),
    height: 34,
    text,
    style: {
      fill: "transparent",
      stroke: "transparent",
      textColor: "#111827",
      fontSize: 18,
      ...options.style,
    },
    createdAt: options.now,
    updatedAt: options.now,
    metadata: frameId
      ? childMetadata(options.preset, templateRole, frameId)
      : templateMetadata(options.preset, templateRole),
  });
}

function stickyCard(
  id: string,
  text: string,
  x: number,
  y: number,
  options: TemplateFactoryOptions,
  templateRole: string,
  frameId: string | WhiteboardElementId,
): WhiteboardElement {
  return createTextElement({
    id,
    role: "sticky-note",
    name: text,
    x,
    y,
    width: 156,
    height: 84,
    text,
    style: {
      fill: "#fef3c7",
      stroke: "#f59e0b",
      strokeWidth: 2,
      textColor: "#111827",
      fontSize: 18,
      ...options.style,
    },
    createdAt: options.now,
    updatedAt: options.now,
    metadata: childMetadata(options.preset, templateRole, frameId),
  });
}

function boundArrow(
  id: string,
  startId: string,
  endId: string,
  startX: number,
  startY: number,
  endX: number,
  endY: number,
  options: TemplateFactoryOptions,
): WhiteboardElement {
  return {
    id: makeElementId(id),
    type: "arrow",
    role: "connector",
    points: [
      { x: startX, y: startY },
      { x: endX, y: endY },
    ],
    locked: false,
    hidden: false,
    style: {
      ...DEFAULT_WHITEBOARD_STYLE,
      stroke: "#334155",
      strokeWidth: 3,
      fill: "transparent",
      ...options.style,
    },
    startBinding: { elementId: makeElementId(startId), anchor: "auto" },
    endBinding: { elementId: makeElementId(endId), anchor: "auto" },
    startArrow: "none",
    endArrow: "triangle",
    createdAt: options.now,
    updatedAt: options.now,
    metadata: {
      ...templateMetadata(options.preset, "connector"),
      connectorRoute: "orthogonal",
    },
  };
}

function templateStyle(
  fill: string,
  stroke: string,
  textColor: string,
  style: Partial<WhiteboardStyle>,
  strokeStyle: WhiteboardStyle["strokeStyle"] = "solid",
): Partial<WhiteboardStyle> {
  return {
    fill,
    stroke,
    strokeStyle,
    strokeWidth: 2,
    textColor,
    ...style,
  };
}

function templateMetadata(
  template: WhiteboardTemplatePresetId,
  templateRole: string,
): WhiteboardMetadata {
  return { sourceOwned: true, template, templateRole };
}

function childMetadata(
  template: WhiteboardTemplatePresetId,
  templateRole: string,
  frameId: string | WhiteboardElementId,
): WhiteboardMetadata {
  return {
    ...templateMetadata(template, templateRole),
    frameId: makeElementId(frameId),
  };
}
