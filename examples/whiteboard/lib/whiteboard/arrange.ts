import { getElementBounds, translateElement } from "./geometry";
import { rerouteBoundConnectors } from "./connectors";
import { expandElementIdsForFrames } from "./frames";
import {
  duplicateElement,
  existingGroups,
  makeCopyId,
  nextGroupId,
  patchElements,
  selectedEditableElements,
  targetPointForAlignment,
  unionElementBounds,
  type WhiteboardAlignment,
} from "./arrange-helpers";
import {
  makeElementId,
  makeGroupId,
  type WhiteboardDocument,
  type WhiteboardElementId,
  type WhiteboardGroup,
  type WhiteboardGroupId,
  type WhiteboardMetadata,
  type WhiteboardPoint,
} from "./model";
import { insertElements } from "./scene";

export type WhiteboardDistribution = "horizontal" | "vertical";
export type { WhiteboardAlignment } from "./arrange-helpers";

export interface CreateElementGroupOptions {
  readonly id?: string | WhiteboardGroupId;
  readonly name?: string;
  readonly metadata?: WhiteboardMetadata;
  readonly now?: string;
}

export interface RemoveElementGroupOptions {
  readonly groupIds?: readonly (string | WhiteboardGroupId)[];
  readonly elementIds?: readonly (string | WhiteboardElementId)[];
  readonly now?: string;
}

export interface SetElementGroupMetadataOptions {
  readonly id: string | WhiteboardGroupId;
  readonly metadata: WhiteboardMetadata;
  readonly now?: string;
}

export function setDocumentMetadata(
  document: WhiteboardDocument,
  metadata: WhiteboardMetadata,
  now = new Date().toISOString(),
): WhiteboardDocument {
  const nextMetadata = { ...(document.metadata ?? {}), ...metadata };
  if (JSON.stringify(nextMetadata) === JSON.stringify(document.metadata ?? {})) return document;

  return {
    ...document,
    metadata: nextMetadata,
    updatedAt: now,
  };
}

export function duplicateElements(
  document: WhiteboardDocument,
  ids: readonly WhiteboardElementId[],
  options: {
    readonly offset?: WhiteboardPoint;
    readonly now?: string;
  } = {},
): WhiteboardDocument {
  const sourceIds = new Set(expandElementIdsForFrames(document, ids));
  if (sourceIds.size === 0) return document;

  const offset = options.offset ?? { x: 32, y: 32 };
  const now = options.now ?? new Date().toISOString();
  const existingIds = new Set(document.elements.map((element) => element.id));
  const sourceElements = document.elements.filter((element) => sourceIds.has(element.id));
  const copiedIdBySource = new Map(
    sourceElements.map((element) => [element.id, makeCopyId(element.id, existingIds)]),
  );
  const copies = document.elements
    .filter((element) => sourceIds.has(element.id))
    .map((element) =>
      duplicateElement(
        element,
        copiedIdBySource.get(element.id) ?? makeCopyId(element.id, existingIds),
        offset,
        now,
        copiedIdBySource,
      ),
    );

  return copies.length === 0
    ? document
    : rerouteBoundConnectors(
        insertElements(document, copies, { select: true, now }),
        copies.map((copy) => copy.id),
        now,
      );
}

export function createElementGroup(
  document: WhiteboardDocument,
  ids: readonly (string | WhiteboardElementId)[],
  options: CreateElementGroupOptions = {},
): WhiteboardDocument {
  const elementIds = selectedEditableElements(document, ids.map((id) => makeElementId(id))).map(
    (element) => element.id,
  );

  if (elementIds.length < 2) return document;

  const now = options.now ?? new Date().toISOString();
  const groupId = options.id ? makeGroupId(options.id) : nextGroupId(document);
  const group: WhiteboardGroup = {
    id: groupId,
    name: options.name?.trim() || `Group ${existingGroups(document).length + 1}`,
    elementIds,
    createdAt: now,
    updatedAt: now,
    metadata: options.metadata,
  };

  const nextElements = document.elements.map((element) =>
    elementIds.includes(element.id) ? { ...element, groupId, updatedAt: now } : element,
  );
  const nextGroups = [...existingGroups(document).filter((item) => item.id !== groupId), group];

  return {
    ...document,
    elements: nextElements,
    groups: nextGroups,
    selection: elementIds,
    updatedAt: now,
  };
}

export function setElementGroupMetadata(
  document: WhiteboardDocument,
  options: SetElementGroupMetadataOptions,
): WhiteboardDocument {
  const groupId = makeGroupId(options.id);
  const groups = existingGroups(document);
  const now = options.now ?? new Date().toISOString();
  let changed = false;
  const nextGroups = groups.map((group) => {
    if (group.id !== groupId) return group;
    changed = true;
    return {
      ...group,
      metadata: options.metadata,
      updatedAt: now,
    };
  });

  return changed ? { ...document, groups: nextGroups, updatedAt: now } : document;
}

export function removeElementGroups(
  document: WhiteboardDocument,
  options: RemoveElementGroupOptions = {},
): WhiteboardDocument {
  const groups = existingGroups(document);
  if (groups.length === 0) return document;

  const explicitGroupIds = new Set((options.groupIds ?? []).map((id) => makeGroupId(id)));
  const elementIds = new Set((options.elementIds ?? []).map((id) => makeElementId(id)));
  const removedGroupIds = new Set<WhiteboardGroupId>(explicitGroupIds);

  if (elementIds.size > 0) {
    for (const group of groups) {
      if (group.elementIds.some((id) => elementIds.has(id))) {
        removedGroupIds.add(group.id);
      }
    }
  }

  if (removedGroupIds.size === 0) return document;

  const now = options.now ?? new Date().toISOString();
  const nextGroups = groups.filter((group) => !removedGroupIds.has(group.id));
  const nextElements = document.elements.map((element) =>
    element.groupId && removedGroupIds.has(element.groupId)
      ? { ...element, groupId: undefined, updatedAt: now }
      : element,
  );

  return {
    ...document,
    elements: nextElements,
    groups: nextGroups.length > 0 ? nextGroups : undefined,
    updatedAt: now,
  };
}

export function setElementsLocked(
  document: WhiteboardDocument,
  ids: readonly WhiteboardElementId[],
  locked: boolean,
  now = new Date().toISOString(),
): WhiteboardDocument {
  return patchElements(document, ids, (element) => ({ ...element, locked, updatedAt: now }), now);
}

export function setElementsHidden(
  document: WhiteboardDocument,
  ids: readonly WhiteboardElementId[],
  hidden: boolean,
  now = new Date().toISOString(),
): WhiteboardDocument {
  const next = patchElements(document, ids, (element) => ({ ...element, hidden, updatedAt: now }), now);
  const rerouted = rerouteBoundConnectors(next, ids, now);
  return hidden
    ? {
        ...rerouted,
        selection: rerouted.selection.filter((id) => !ids.includes(id)),
      }
    : rerouted;
}

export function alignElements(
  document: WhiteboardDocument,
  ids: readonly WhiteboardElementId[],
  alignment: WhiteboardAlignment,
  now = new Date().toISOString(),
): WhiteboardDocument {
  const selected = selectedEditableElements(document, ids);
  if (selected.length < 2) return document;

  const boundsById = new Map(selected.map((element) => [element.id, getElementBounds(element)]));
  const sceneBounds = unionElementBounds(selected);

  return rerouteBoundConnectors(
    patchElements(
      document,
      selected.map((element) => element.id),
      (element) => {
        const bounds = boundsById.get(element.id);
        if (!bounds) return element;

        const target = targetPointForAlignment(sceneBounds, bounds, alignment);
        return translateElement(element, { x: target.x - bounds.x, y: target.y - bounds.y }, now);
      },
      now,
    ),
    selected.map((element) => element.id),
    now,
  );
}

export function distributeElements(
  document: WhiteboardDocument,
  ids: readonly WhiteboardElementId[],
  distribution: WhiteboardDistribution,
  now = new Date().toISOString(),
): WhiteboardDocument {
  const selected = selectedEditableElements(document, ids);
  if (selected.length < 3) return document;

  const boundsById = new Map(selected.map((element) => [element.id, getElementBounds(element)]));
  const sorted = [...selected].sort((left, right) => {
    const leftBounds = boundsById.get(left.id);
    const rightBounds = boundsById.get(right.id);
    if (!leftBounds || !rightBounds) return 0;
    return distribution === "horizontal" ? leftBounds.x - rightBounds.x : leftBounds.y - rightBounds.y;
  });

  const firstBounds = boundsById.get(sorted[0].id);
  const lastBounds = boundsById.get(sorted[sorted.length - 1].id);
  if (!firstBounds || !lastBounds) return document;

  const totalSize = sorted.reduce((sum, element) => {
    const bounds = boundsById.get(element.id);
    return sum + (distribution === "horizontal" ? bounds?.width ?? 0 : bounds?.height ?? 0);
  }, 0);
  const span =
    distribution === "horizontal"
      ? lastBounds.x + lastBounds.width - firstBounds.x
      : lastBounds.y + lastBounds.height - firstBounds.y;
  const gap = (span - totalSize) / (sorted.length - 1);
  let cursor = distribution === "horizontal" ? firstBounds.x : firstBounds.y;
  const targetById = new Map<WhiteboardElementId, number>();

  for (const element of sorted) {
    const bounds = boundsById.get(element.id);
    if (!bounds) continue;
    targetById.set(element.id, cursor);
    cursor += (distribution === "horizontal" ? bounds.width : bounds.height) + gap;
  }

  return rerouteBoundConnectors(
    patchElements(
      document,
      sorted.map((element) => element.id),
      (element) => {
        const bounds = boundsById.get(element.id);
        const target = targetById.get(element.id);
        if (!bounds || target === undefined) return element;

        const delta =
          distribution === "horizontal"
            ? { x: target - bounds.x, y: 0 }
            : { x: 0, y: target - bounds.y };
        return translateElement(element, delta, now);
      },
      now,
    ),
    sorted.map((element) => element.id),
    now,
  );
}
