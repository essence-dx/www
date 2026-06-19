import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import type {
  WhiteboardElementPatch,
  WhiteboardTextElement,
} from "../../lib/whiteboard/model";

export type TextControlsProps = {
  readonly element: WhiteboardTextElement;
};

export function TextControls({ element }: TextControlsProps) {
  return (
    <div className="wb-control-stack" data-whiteboard-text-controls="command-backed">
      <label className="wb-input-row">
        <span>Text</span>
        <textarea
          className="wb-text-field"
          data-whiteboard-command="text.commit"
          data-whiteboard-text-field="text"
          defaultValue={element.text}
          onBlur={(event) => whiteboardActions.commitText(element.id, event.currentTarget.value)}
          onKeyDown={(event) => {
            if (event.key === "Enter" && (event.ctrlKey || event.metaKey)) {
              event.preventDefault();
              whiteboardActions.commitText(element.id, event.currentTarget.value);
              event.currentTarget.blur();
            }
          }}
          rows={3}
        />
      </label>
      <label className="wb-select-row">
        <span>Text align</span>
        <select
          onInput={(event) =>
            whiteboardActions.dispatch({
              type: "element.update",
              id: element.id,
              patch: {
                textAlign: event.currentTarget.value as WhiteboardElementPatch["textAlign"],
              },
            })
          }
          value={element.textAlign}
        >
          <option value="left">Left</option>
          <option value="center">Center</option>
          <option value="right">Right</option>
        </select>
      </label>
      <label className="wb-select-row">
        <span>Vertical align</span>
        <select
          onInput={(event) =>
            whiteboardActions.dispatch({
              type: "element.update",
              id: element.id,
              patch: {
                verticalAlign: event.currentTarget.value as WhiteboardElementPatch["verticalAlign"],
              },
            })
          }
          value={element.verticalAlign}
        >
          <option value="top">Top</option>
          <option value="middle">Middle</option>
          <option value="bottom">Bottom</option>
        </select>
      </label>
    </div>
  );
}
