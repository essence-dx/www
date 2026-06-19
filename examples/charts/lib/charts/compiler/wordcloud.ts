import { round } from "../geometry";
import { extent, formatValue, readField, toLabel, toNumber } from "../format";
import { CHART_PALETTE, createLinearScale } from "../scales";
import type { ChartSpec } from "../spec";
import type { SceneElement } from "../scene";
import { applyMarkTransforms } from "../transforms";
import { emptyScene, scene, sceneId } from "./shared";

type WordBox = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export function compileWordCloud(spec: ChartSpec) {
  const mark = spec.marks[0];
  const label = mark.encoding.label ?? mark.encoding.x;
  const weight = mark.encoding.size ?? mark.encoding.y;
  if (!label || !weight) return emptyScene(spec);

  const rows = applyMarkTransforms(spec.data, mark)
    .map((datum) => ({ label: toLabel(readField(datum, label.field)), value: toNumber(readField(datum, weight.field)) }))
    .filter((word) => word.label && word.value > 0)
    .sort((left, right) => right.value - left.value);

  const scale = createLinearScale(extent(rows.map((word) => word.value)), [14, Math.min(44, spec.height * 0.13)]);
  const placed: WordBox[] = [];
  const elements: SceneElement[] = [];

  rows.forEach((word, index) => {
    const fontSize = round(scale.map(word.value));
    const box = placeWord(spec.width, spec.height, word.label, fontSize, placed, index);
    placed.push(box);
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "word", index),
      x: round(box.x + box.width / 2),
      y: round(box.y + box.height * 0.76),
      text: word.label,
      anchor: "middle",
      fill: CHART_PALETTE[index % CHART_PALETTE.length],
      fontSize,
      fontWeight: index < 3 ? 800 : 650,
      label: `${word.label}: ${formatValue(word.value, weight.format)}`,
      className: "chart-mark chart-mark-wordcloud",
      wordCloudTerm: word.label,
      wordCloudWeight: formatValue(word.value, weight.format),
    });
  });

  return scene(spec, elements, []);
}

export function placeWord(width: number, height: number, text: string, fontSize: number, placed: WordBox[], index: number): WordBox {
  const boxWidth = Math.max(fontSize * 2.2, text.length * fontSize * 0.56);
  const boxHeight = fontSize * 1.18;
  const centerX = width / 2 - boxWidth / 2;
  const centerY = height / 2 - boxHeight / 2;
  const maxSteps = 160;

  for (let step = 0; step < maxSteps; step += 1) {
    const angle = step * 0.48 + index * 0.92;
    const radius = step * 2.6;
    const candidate = {
      x: centerX + Math.cos(angle) * radius,
      y: centerY + Math.sin(angle) * radius * 0.68,
      width: boxWidth,
      height: boxHeight,
    };

    if (inside(candidate, width, height) && placed.every((box) => !intersects(candidate, box))) {
      return candidate;
    }
  }

  return {
    x: Math.max(8, Math.min(width - boxWidth - 8, centerX + (index % 5) * 10 - 20)),
    y: Math.max(18, Math.min(height - boxHeight - 8, centerY + Math.floor(index / 5) * (boxHeight + 4))),
    width: boxWidth,
    height: boxHeight,
  };
}

function inside(box: WordBox, width: number, height: number): boolean {
  return box.x >= 8 && box.y >= 18 && box.x + box.width <= width - 8 && box.y + box.height <= height - 8;
}

function intersects(left: WordBox, right: WordBox): boolean {
  return left.x < right.x + right.width && left.x + left.width > right.x && left.y < right.y + right.height && left.y + left.height > right.y;
}
