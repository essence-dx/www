import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import {
  WHITEBOARD_LIBRARY_PRESETS,
  WHITEBOARD_LIBRARY_TEMPLATE_PRESETS,
  type WhiteboardLibraryPreset,
  type WhiteboardLibraryPresetCategory,
  type WhiteboardLibraryPresetId,
} from "../../lib/whiteboard/library";
import { DxIcon } from "./toolbar";

type WhiteboardLibraryGroup = {
  readonly id: WhiteboardLibraryPresetCategory;
  readonly label: string;
  readonly presets: readonly WhiteboardLibraryPreset[];
};

const WHITEBOARD_LIBRARY_GROUPS: readonly WhiteboardLibraryGroup[] = [
  {
    id: "single",
    label: "Presets",
    presets: WHITEBOARD_LIBRARY_PRESETS.filter((preset) => preset.category === "single"),
  },
  {
    id: "template",
    label: "Templates",
    presets: WHITEBOARD_LIBRARY_TEMPLATE_PRESETS,
  },
];

export function LibraryPanel() {
  return (
    <section
      className="wb-panel wb-library-panel"
      aria-labelledby="wb-library-title"
      data-whiteboard-library="command-backed"
      data-whiteboard-library-shell="grouped-presets"
      data-whiteboard-library-template-count={WHITEBOARD_LIBRARY_TEMPLATE_PRESETS.length}
    >
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Insert</p>
          <h2 id="wb-library-title">Library</h2>
        </div>
        <span className="wb-panel-badge">
          {WHITEBOARD_LIBRARY_PRESETS.length} presets
        </span>
      </div>

      <div
        className="wb-library-groups"
        data-whiteboard-library-presets={WHITEBOARD_LIBRARY_PRESETS.length}
      >
        {WHITEBOARD_LIBRARY_GROUPS.map((group) => (
          <section
            aria-labelledby={`wb-library-${group.id}-title`}
            className="wb-library-group"
            data-whiteboard-library-group={group.id}
            data-whiteboard-library-group-count={group.presets.length}
            key={group.id}
          >
            <div className="wb-library-group-heading">
              <h3 id={`wb-library-${group.id}-title`}>{group.label}</h3>
              <span>{group.presets.length}</span>
            </div>

            <div className="wb-library-grid" data-whiteboard-library-grid={group.id}>
              {group.presets.map((preset, groupIndex) => {
                const presetIndex = WHITEBOARD_LIBRARY_PRESETS.indexOf(preset);

                return (
                  <button
                    aria-label={`Insert ${preset.label}`}
                    className="wb-library-card"
                    data-whiteboard-command="library.insert"
                    data-whiteboard-element-count={preset.elementCount}
                    data-whiteboard-preset={preset.id}
                    data-whiteboard-preset-category={preset.category}
                    data-whiteboard-template={preset.category === "template" ? "source-owned" : undefined}
                    key={preset.id}
                    onClick={() => insertPreset(preset.id, presetIndex >= 0 ? presetIndex : groupIndex)}
                    type="button"
                  >
                    <DxIcon name="library" />
                    <span>{preset.label}</span>
                    <small>{preset.description}</small>
                  </button>
                );
              })}
            </div>
          </section>
        ))}
      </div>
    </section>
  );
}

function insertPreset(preset: WhiteboardLibraryPresetId, index: number) {
  return whiteboardActions.insertLibraryPreset(preset, {
    x: 96 + index * 28,
    y: 96 + index * 24,
  });
}
