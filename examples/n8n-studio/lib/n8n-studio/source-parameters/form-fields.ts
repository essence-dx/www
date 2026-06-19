import type { NodeParameterDefinition } from "../node-types/types";
import {
  applyN8nDisplayOptions,
  sourceEntriesForArray,
  sourceParametersFromEntries,
} from "./source-description-reader";

export type FormFieldsSourceBundle = {
  formNodeSource: string;
  formCommonDescriptionSource: string;
};

type FormFieldsExtractionOptions = {
  sourceBundle: FormFieldsSourceBundle;
  displayOptionsBlock: string;
};

const formFieldObjectNames = ["formFields", "formFieldsDynamic"];
const formFieldArrayPropertyNames = ["formOptions", "formElementTypes"];

export function sourceParametersFromFormFieldsProperties({
  sourceBundle,
  displayOptionsBlock,
}: FormFieldsExtractionOptions): NodeParameterDefinition[] {
  const entries = sourceEntriesForArray({
    source: sourceBundle.formNodeSource,
    arrayName: "formFieldsProperties",
    sharedSource: sourceBundle.formCommonDescriptionSource,
    sharedObjectNames: formFieldObjectNames,
    sharedArrayPropertyNames: formFieldArrayPropertyNames,
  }).map((entry) => applyN8nDisplayOptions(entry, displayOptionsBlock));

  return sourceParametersFromEntries(entries);
}
