import { boundsToScreenRect, elementBounds } from "../../lib/whiteboard/render/geometry";
import type { WhiteboardTextElement, WhiteboardViewport } from "../../lib/whiteboard/render/model";

type TextEditorOverlayProps = {
  element: WhiteboardTextElement;
  viewport: WhiteboardViewport;
  onCommit?: (id: WhiteboardTextElement["id"], text: string) => void;
  value?: string;
  className?: string;
  label?: string;
  disabled?: boolean;
};

export function TextEditorOverlay({
  element,
  viewport,
  onCommit,
  value,
  className = "whiteboard-text-editor",
  label = "Edit whiteboard text",
  disabled = false,
}: TextEditorOverlayProps) {
  const rect = boundsToScreenRect(elementBounds(element), viewport);
  const fontSize = element.style.fontSize;
  const commit = (target: HTMLTextAreaElement) => {
    onCommit?.(element.id, target.value);
  };

  return (
    <textarea
      aria-label={label}
      className={className}
      data-whiteboard-text-editor="source-owned"
      data-whiteboard-element-id={element.id}
      data-whiteboard-command="text.commit"
      data-whiteboard-text-commit="blur-mod-enter"
      data-whiteboard-text-field="text"
      defaultValue={value ?? element.text}
      disabled={disabled}
      onBlur={(event) => commit(event.currentTarget)}
      onKeyDown={(event) => {
        if (event.key === "Enter" && (event.ctrlKey || event.metaKey)) {
          event.preventDefault();
          commit(event.currentTarget);
          event.currentTarget.blur();
        }
      }}
      spellCheck={false}
      style={{
        position: "absolute",
        left: `${rect.x}px`,
        top: `${rect.y}px`,
        width: `${Math.max(rect.width, 24)}px`,
        minHeight: `${Math.max(rect.height, fontSize * 1.4)}px`,
        fontSize: `${fontSize * viewport.zoom}px`,
        fontFamily: element.style.fontFamily,
        lineHeight: "1.3",
        transformOrigin: "0 0",
      }}
    />
  );
}
