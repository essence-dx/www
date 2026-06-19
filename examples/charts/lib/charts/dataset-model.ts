import type { DataSetStageSpec, DataSetTransformStepSpec, DataSetViewSpec, Datum, TransformSpec } from "./spec";
import { applyTransforms } from "./transforms";

export const DATASET_STAGE_SEPARATOR = " -> ";

export type DataSetViewModel = {
  id: string;
  label: string;
  stages: DataSetStageSpec[];
  outputRows: Datum[];
};

export function createDataSetView(spec: DataSetViewSpec): DataSetViewModel {
  const stages: DataSetStageSpec[] = [
    {
      id: "source",
      label: spec.sourceLabel,
      rowCount: spec.sourceRows.length,
      rows: cloneRows(spec.sourceRows),
    },
  ];

  let currentRows = cloneRows(spec.sourceRows);
  spec.steps.forEach((step) => {
    currentRows = applyTransforms(currentRows, [step.transform]);
    stages.push(stageFromStep(step, currentRows));
  });

  return {
    id: spec.id,
    label: spec.label,
    stages,
    outputRows: cloneRows(currentRows),
  };
}

export function runDataSetPipeline(spec: DataSetViewSpec): Datum[] {
  return createDataSetView(spec).outputRows;
}

export function materializeDataSetFlowRows(view: DataSetViewModel): Datum[] {
  return view.stages.slice(1).map((stage, index) => {
    const previous = view.stages[index];
    const transformType = stage.transform?.type ?? "source";
    return {
      source: previous.label,
      target: stage.label,
      value: Math.max(1, stage.rowCount),
      stageId: stage.id,
      stageName: stage.label,
      transform: transformType,
      rowCount: stage.rowCount,
      lineage: [previous.label, stage.label].join(DATASET_STAGE_SEPARATOR),
    };
  });
}

export function summarizeDataSetPipeline(view: DataSetViewModel): string {
  const source = view.stages[0];
  const output = view.stages[view.stages.length - 1] ?? source;
  return `${view.label}: ${source.rowCount} source rows to ${output.rowCount} output rows across ${Math.max(0, view.stages.length - 1)} transforms.`;
}

function stageFromStep(step: DataSetTransformStepSpec, rows: Datum[]): DataSetStageSpec {
  return {
    id: step.id,
    label: step.label,
    rowCount: rows.length,
    transform: cloneTransform(step.transform),
    rows: cloneRows(rows),
  };
}

function cloneRows(rows: Datum[]): Datum[] {
  return rows.map((row) => ({ ...row }));
}

function cloneTransform(transform: TransformSpec): TransformSpec {
  return { ...transform };
}
