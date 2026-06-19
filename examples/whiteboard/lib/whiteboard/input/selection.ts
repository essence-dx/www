import {
  detectSelectionHandle,
  elementBounds,
  elementsById,
  hitTestDocument,
  normalizeBounds,
  selectionBounds,
} from "../render/geometry";
import { rectHasMinimumExtent } from "../geometry";
import { getAreaSelectionIds } from "../scene";
import type { WhiteboardDocument, WhiteboardPoint, WhiteboardSelection } from "../render/model";
import type { WhiteboardCommand } from "../commands";
import { expandElementIdsForFrames } from "../frames";
import { expandElementIdsForGroups } from "../groups";
import type { ActiveDrag, WhiteboardSelectionAreaDrag } from "./types";

export type SelectionDragResult = {
  readonly drag: ActiveDrag | null;
  readonly commands: WhiteboardCommand[];
};

export function beginSelectionDrag(context: {
  readonly document: WhiteboardDocument;
  readonly selection: WhiteboardSelection;
  readonly pointerId: number;
  readonly worldPoint: WhiteboardPoint;
  readonly extend?: boolean;
}): SelectionDragResult {
  const visibleElements = context.document.elements.filter((element) => !element.hidden);
  const selectedElements = elementsById(visibleElements, context.selection.ids);
  const selectedBounds = selectionBounds(selectedElements);

  if (selectedBounds) {
    const handle = detectSelectionHandle(selectedBounds, context.worldPoint);
    if (handle && selectedElements.length === 1) {
      const element = selectedElements[0];
      if (element.locked) {
        return {
          drag: null,
          commands: [{ type: "selection.set", ids: [element.id] }],
        };
      }

      return {
        drag: {
          kind: "resize",
          pointerId: context.pointerId,
          handle,
          element,
          initialBounds: elementBounds(element),
        },
        commands: [{ type: "selection.set", ids: [element.id] }],
      };
    }
  }

  const hit = hitTestDocument(visibleElements, context.worldPoint);
  if (!hit) {
    return {
      drag: {
        kind: "select-area",
        pointerId: context.pointerId,
        startWorld: context.worldPoint,
        currentWorld: context.worldPoint,
        extend: context.extend,
      },
      commands: [],
    };
  }

  const hitIds = expandElementIdsForFrames(
    context.document,
    expandElementIdsForGroups(context.document, [hit.id]),
  );
  const ids = context.extend ? [...context.selection.ids, ...hitIds] : hitIds;
  const movableElements = elementsById(visibleElements, ids).filter((element) => !element.locked);

  return {
    drag:
      movableElements.length > 0
        ? {
            kind: "move",
            pointerId: context.pointerId,
            startWorld: context.worldPoint,
            elements: movableElements,
          }
        : null,
    commands: [{ type: "selection.set", ids, mode: context.extend ? "extend" : "replace" }],
  };
}

export function selectionAreaBounds(drag: WhiteboardSelectionAreaDrag) {
  return normalizeBounds(drag.startWorld, drag.currentWorld);
}

export function finishSelectionAreaDrag(context: {
  readonly document: WhiteboardDocument;
  readonly selection: WhiteboardSelection;
  readonly drag: WhiteboardSelectionAreaDrag;
}): WhiteboardCommand[] {
  const bounds = selectionAreaBounds(context.drag);
  if (!rectHasMinimumExtent(bounds)) {
    return context.drag.extend ? [] : [{ type: "selection.clear" }];
  }

  const expandedIds = getAreaSelectionIds(context.document, bounds);

  if (expandedIds.length === 0) {
    return context.drag.extend ? [] : [{ type: "selection.clear" }];
  }

  return [
    {
      type: "selection.set",
      ids: context.drag.extend
        ? uniqueIds([...context.selection.ids, ...expandedIds])
        : expandedIds,
      mode: "replace",
    },
  ];
}

function uniqueIds<T>(ids: readonly T[]): readonly T[] {
  return [...new Set(ids)];
}
