import {
  makeElementId,
  type WhiteboardDocument,
  type WhiteboardElementId,
  type WhiteboardGroupId,
} from "./model";

export function expandElementIdsForGroups(
  document: WhiteboardDocument,
  ids: readonly (string | WhiteboardElementId)[],
): readonly WhiteboardElementId[] {
  const requestedIds = new Set(ids.map((id) => makeElementId(id)));
  const groupIds = new Set<WhiteboardGroupId>();

  for (const element of document.elements) {
    if (requestedIds.has(element.id) && element.groupId) {
      groupIds.add(element.groupId);
    }
  }

  if (groupIds.size === 0) {
    return document.elements.filter((element) => requestedIds.has(element.id)).map((element) => element.id);
  }

  const expandedIds = new Set(requestedIds);
  for (const group of document.groups ?? []) {
    if (!groupIds.has(group.id)) continue;
    for (const elementId of group.elementIds) {
      expandedIds.add(elementId);
    }
  }

  for (const element of document.elements) {
    if (element.groupId && groupIds.has(element.groupId)) {
      expandedIds.add(element.id);
    }
  }

  return document.elements.filter((element) => expandedIds.has(element.id)).map((element) => element.id);
}
