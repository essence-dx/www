import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import type { WhiteboardDocument, WhiteboardSize } from "../../lib/whiteboard/model";
import {
  createWhiteboardOutlineModel,
  outlineCommandsForItem,
  type WhiteboardOutlineItem,
} from "../../lib/whiteboard/outline";
import { DxIcon } from "./toolbar";

export type OutlinePanelProps = {
  readonly document: WhiteboardDocument;
  readonly stageSize: WhiteboardSize;
};

export function OutlinePanel({ document, stageSize }: OutlinePanelProps) {
  const outline = createWhiteboardOutlineModel(document, stageSize);

  return (
    <section
      className="wb-panel wb-outline-panel"
      aria-labelledby="wb-outline-title"
      data-whiteboard-outline="frame-grouped"
      data-whiteboard-outline-section-count={outline.sections.length}
      data-whiteboard-outline-selected-count={outline.selectedItemCount}
    >
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Navigate</p>
          <h2 id="wb-outline-title">Outline</h2>
        </div>
        <span className="wb-panel-badge">
          {outline.itemCount} items
        </span>
      </div>

      {outline.empty ? (
        <p className="wb-empty-note">Draw or insert elements to populate the board outline.</p>
      ) : (
        <div className="wb-outline-sections">
          {outline.sections.map((section) => (
            <section
              aria-labelledby={`wb-outline-section-${section.id}`}
              className="wb-outline-section"
              data-whiteboard-outline-frame-id={section.frameId ?? "none"}
              data-whiteboard-outline-section={section.id}
              data-whiteboard-outline-section-count={section.itemCount}
              key={section.id}
            >
              <div className="wb-outline-section-heading">
                <h3 id={`wb-outline-section-${section.id}`}>{section.title}</h3>
                <span>{section.itemCount}</span>
              </div>

              <div className="wb-outline-list" role="listbox" aria-label={`${section.title} outline items`}>
                {section.items.map((item) => (
                  <button
                    aria-selected={item.selected}
                    className="wb-outline-item"
                    data-active={item.selected}
                    data-hidden={item.hidden}
                    data-locked={item.locked}
                    data-whiteboard-command="selection.set viewport.set"
                    data-whiteboard-outline-frame-id={item.frameId ?? "none"}
                    data-whiteboard-outline-item-id={item.id}
                    data-whiteboard-outline-item-role={item.role}
                    data-whiteboard-outline-item-type={item.type}
                    key={item.id}
                    onClick={() => focusOutlineItem(item)}
                    role="option"
                    type="button"
                  >
                    <span className="wb-outline-mark" data-element-type={item.type} />
                    <strong>{item.label}</strong>
                    <small>{outlineItemStatus(item)}</small>
                  </button>
                ))}
              </div>
            </section>
          ))}
        </div>
      )}
    </section>
  );
}

function focusOutlineItem(item: WhiteboardOutlineItem) {
  return whiteboardActions.dispatchBatch(outlineCommandsForItem(item));
}

function outlineItemStatus(item: WhiteboardOutlineItem): string {
  if (item.hidden) return "hidden";
  if (item.locked) return "locked";
  if (item.role === "frame") return "frame";
  return item.role;
}
