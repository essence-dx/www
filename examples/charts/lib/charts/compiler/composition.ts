import { readField, toLabel, uniqueLabels } from "../format";
import type { ChartScene, SceneElement } from "../scene";
import type { ChartSpec, ViewRegionSpec } from "../spec";
import { emptyScene, scene, sceneId } from "./shared";

type CompileChartSpec = (spec: ChartSpec) => ChartScene;

export function compileFacet(spec: ChartSpec, compile: CompileChartSpec): ChartScene {
  const composition = spec.composition;
  if (!composition || composition.type !== "facet") return emptyScene(spec);

  const groups = uniqueLabels(spec.data.map((datum) => readField(datum, composition.field)));
  if (groups.length === 0) return emptyScene(spec);

  const columns = Math.max(1, Math.min(composition.columns ?? 2, groups.length));
  const rows = Math.ceil(groups.length / columns);
  const cellWidth = spec.width / columns;
  const cellHeight = spec.height / rows;
  const labelHeight = 24;
  const elements: SceneElement[] = [];
  const legends: ChartScene["legend"] = [];

  groups.forEach((group, index) => {
    const column = index % columns;
    const row = Math.floor(index / columns);
    const x = column * cellWidth;
    const y = row * cellHeight;
    const facetSpec: ChartSpec = {
      ...spec,
      id: sceneId(spec.id, "facet", group),
      width: cellWidth,
      height: Math.max(120, cellHeight - labelHeight),
      data: spec.data.filter((datum) => toLabel(readField(datum, composition.field)) === group),
      composition: undefined,
      padding: spec.padding ?? { left: 44, right: 18, top: 20, bottom: 38 },
    };
    const facetScene = compile(facetSpec);

    elements.push({
      kind: "text",
      id: sceneId(spec.id, "facet-label", group),
      x: x + 12,
      y: y + 16,
      text: `${composition.label ?? composition.field}: ${group}`,
      className: "chart-facet-label",
    });

    facetScene.elements.forEach((element) => {
      elements.push({
        ...element,
        id: sceneId(spec.id, group, element.id),
        transform: combineTransforms(`translate(${round(x)} ${round(y + labelHeight)})`, element.transform),
      });
    });

    facetScene.legend.forEach((legend) => {
      if (!legends.some((entry) => entry.label === legend.label)) {
        legends.push(legend);
      }
    });
  });

  return scene(spec, elements, legends);
}

export function compileView(spec: ChartSpec, compile: CompileChartSpec): ChartScene {
  const composition = spec.composition;
  if (!composition || composition.type !== "view") return emptyScene(spec);

  const elements: SceneElement[] = [];
  const legends: ChartScene["legend"] = [];

  composition.children.forEach((child) => {
    const region = normalizeRegion(child.region, spec.width, spec.height);
    const childSpec: ChartSpec = {
      ...child.spec,
      id: sceneId(spec.id, "view", child.id),
      width: Math.max(120, region.width),
      height: Math.max(120, region.height),
    };
    const childScene = compile(childSpec);
    const regionLabel = `${round(region.x)},${round(region.y)},${round(region.width)},${round(region.height)}`;

    elements.push({
      kind: "text",
      id: sceneId(spec.id, "view-label", child.id),
      x: region.x + 12,
      y: region.y + 18,
      text: child.title ?? child.spec.title,
      className: "chart-facet-label",
      viewId: child.id,
      viewTitle: child.title ?? child.spec.title,
      viewRegion: regionLabel,
      coordinateType: child.spec.coordinate?.type,
    });

    childScene.elements.forEach((element) => {
      elements.push({
        ...element,
        id: sceneId(spec.id, child.id, element.id),
        transform: combineTransforms(`translate(${round(region.x)} ${round(region.y + 24)})`, element.transform),
        viewId: child.id,
        viewTitle: child.title ?? child.spec.title,
        viewRegion: regionLabel,
        coordinateType: element.coordinateType ?? child.spec.coordinate?.type,
      });
    });

    childScene.legend.forEach((legend) => {
      if (!legends.some((entry) => entry.label === legend.label)) {
        legends.push(legend);
      }
    });
  });

  return scene(spec, elements, legends);
}

function normalizeRegion(region: ViewRegionSpec, width: number, height: number) {
  return {
    x: region.x <= 1 ? region.x * width : region.x,
    y: region.y <= 1 ? region.y * height : region.y,
    width: region.width <= 1 ? region.width * width : region.width,
    height: region.height <= 1 ? region.height * height : region.height,
  };
}

function combineTransforms(...transforms: Array<string | undefined>) {
  return transforms.filter(Boolean).join(" ");
}

function round(value: number): number {
  return Math.round(value * 100) / 100;
}
