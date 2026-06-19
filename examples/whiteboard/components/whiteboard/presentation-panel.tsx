import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import type { WhiteboardDocument, WhiteboardSize } from "../../lib/whiteboard/model";
import {
  createWhiteboardPresentationModel,
  presentationCommandsForSlide,
  type WhiteboardPresentationSlide,
} from "../../lib/whiteboard/presentation";
import { DxIcon } from "./toolbar";

export type PresentationPanelProps = {
  readonly document: WhiteboardDocument;
  readonly stageSize: WhiteboardSize;
};

export function PresentationPanel({ document, stageSize }: PresentationPanelProps) {
  const presentation = createWhiteboardPresentationModel(document, stageSize);

  return (
    <section
      className="wb-panel wb-presentation-panel"
      aria-labelledby="wb-presentation-title"
      data-whiteboard-presentation="frame-navigator"
      data-whiteboard-presentation-current-index={presentation.currentIndex}
      data-whiteboard-presentation-slide-count={presentation.slides.length}
    >
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Frames</p>
          <h2 id="wb-presentation-title">Presentation</h2>
        </div>
        <span className="wb-panel-badge">
          {presentation.slides.length} slides
        </span>
      </div>

      <div className="wb-presentation-controls" role="toolbar" aria-label="Presentation navigation">
        <button
          className="wb-action-button"
          data-whiteboard-command="selection.set viewport.set"
          disabled={!presentation.previousSlide}
          onClick={() => focusSlide(presentation.previousSlide)}
          type="button"
        >
          <DxIcon name="arrow" />
          <span>Previous</span>
        </button>
        <button
          className="wb-action-button"
          data-whiteboard-command="selection.set viewport.set"
          disabled={!presentation.currentSlide}
          onClick={() => focusSlide(presentation.currentSlide)}
          type="button"
        >
          <DxIcon name="fit" />
          <span>Present</span>
        </button>
        <button
          className="wb-action-button"
          data-whiteboard-command="selection.set viewport.set"
          disabled={!presentation.nextSlide}
          onClick={() => focusSlide(presentation.nextSlide)}
          type="button"
        >
          <DxIcon name="arrow" />
          <span>Next</span>
        </button>
      </div>

      {presentation.empty ? (
        <p className="wb-empty-note">Add a frame preset to create presentation slides.</p>
      ) : (
        <div className="wb-presentation-list" role="listbox" aria-label="Frame slides">
          {presentation.slides.map((slide) => (
            <button
              aria-selected={slide.selected}
              className="wb-presentation-slide"
              data-active={slide.selected}
              data-whiteboard-command="selection.set viewport.set"
              data-whiteboard-presentation-child-count={slide.childCount}
              data-whiteboard-presentation-slide-id={slide.id}
              key={slide.id}
              onClick={() => focusSlide(slide)}
              role="option"
              type="button"
            >
              <span>{slide.index + 1}</span>
              <strong>{slide.title}</strong>
              <small>{slide.childCount} items</small>
            </button>
          ))}
        </div>
      )}
    </section>
  );
}

function focusSlide(slide: WhiteboardPresentationSlide | null) {
  if (!slide) return undefined;
  return whiteboardActions.dispatchBatch(presentationCommandsForSlide(slide));
}
