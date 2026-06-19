import { Icon } from "../icons/icon";
import type { WhiteboardTool } from "../../lib/whiteboard/model";

export type IconName =
  | "align"
  | "arrow"
  | "copy"
  | "diamond"
  | "download"
  | "ellipse"
  | "eraser"
  | "eye"
  | "eye-off"
  | "fit"
  | "grid"
  | "hand"
  | "image"
  | "library"
  | "line"
  | "lock"
  | "move"
  | "pen"
  | "rectangle"
  | "redo"
  | "select"
  | "snap"
  | "text"
  | "undo"
  | "unlock"
  | "zoom-in"
  | "zoom-out";

export type ToolbarProps = {
  activeTool: WhiteboardTool;
  canRedo: boolean;
  canUndo: boolean;
  zoom: number;
  onRedo: () => void;
  onToolChange: (tool: WhiteboardTool) => void;
  onUndo: () => void;
  onZoomFit: () => void;
  onZoomIn: () => void;
  onZoomOut: () => void;
};

type ToolbarTool = {
  id: WhiteboardTool;
  label: string;
  shortcut: string;
  icon: IconName;
};

const TOOLBAR_TOOLS: readonly ToolbarTool[] = [
  { id: "select", label: "Move", shortcut: "V", icon: "move" },
  { id: "hand", label: "Hand", shortcut: "H", icon: "hand" },
  { id: "freehand", label: "Freehand", shortcut: "F", icon: "pen" },
  { id: "line", label: "Line", shortcut: "L", icon: "line" },
  { id: "arrow", label: "Arrow", shortcut: "A", icon: "arrow" },
  { id: "rectangle", label: "Rectangle", shortcut: "R", icon: "rectangle" },
  { id: "ellipse", label: "Ellipse", shortcut: "E", icon: "ellipse" },
  { id: "diamond", label: "Diamond", shortcut: "D", icon: "diamond" },
  { id: "text", label: "Text", shortcut: "T", icon: "text" },
  { id: "eraser", label: "Eraser", shortcut: "X", icon: "eraser" },
];

export function DxIcon({ name, title, className }: { name: IconName; title?: string; className?: string }) {
  return (
    <Icon
      className={["wb-icon", className].filter(Boolean).join(" ")}
      name={`whiteboard:${name}`}
      title={title}
    />
  );
}

export function Toolbar({
  activeTool,
  canRedo,
  canUndo,
  zoom,
  onRedo,
  onToolChange,
  onUndo,
  onZoomFit,
  onZoomIn,
  onZoomOut,
}: ToolbarProps) {
  return (
    <aside className="wb-toolbar" aria-label="Whiteboard tools">
      <div className="wb-toolbar-group" role="toolbar" aria-label="Drawing tools">
        {TOOLBAR_TOOLS.map((tool) => (
          <button
            aria-label={`${tool.label} tool`}
            aria-pressed={activeTool === tool.id}
            className="wb-icon-button"
            data-active={activeTool === tool.id}
            data-whiteboard-tool-shortcut={tool.shortcut}
            key={tool.id}
            onClick={() => onToolChange(tool.id)}
            title={tool.label}
            type="button"
          >
            <DxIcon name={tool.icon} />
          </button>
        ))}
      </div>
      <IconButton disabled={!canUndo} icon="undo" label="Undo" onClick={onUndo} />
      <IconButton disabled={!canRedo} icon="redo" label="Redo" onClick={onRedo} />
      <div className="wb-toolbar-group" role="toolbar" aria-label="Zoom">
        <IconButton icon="zoom-out" label="Zoom out" onClick={onZoomOut} standalone />
        <button
          aria-label="Fit canvas"
          className="wb-zoom-pill"
          onClick={onZoomFit}
          title="Fit canvas"
          type="button"
        >
          <DxIcon name="fit" />
          <span>{zoom}%</span>
        </button>
        <IconButton icon="zoom-in" label="Zoom in" onClick={onZoomIn} standalone />
      </div>
    </aside>
  );
}

function IconButton({
  disabled,
  icon,
  label,
  onClick,
  standalone = false,
}: {
  disabled?: boolean;
  icon: IconName;
  label: string;
  onClick: () => void;
  standalone?: boolean;
}) {
  const button = (
    <button
      aria-label={label}
      className="wb-icon-button"
      disabled={disabled}
      onClick={onClick}
      title={label}
      type="button"
    >
      <DxIcon name={icon} />
    </button>
  );

  return standalone ? button : (
    <div className="wb-toolbar-group" role="toolbar" aria-label={label}>
      {button}
    </div>
  );
}
