import type { NodeParameterDefinition } from "../node-types/types";
import {
  applyN8nDisplayOptions,
  sourceEntriesForArray,
  sourceParametersFromEntries,
} from "./source-description-reader";
import type { DiscordSourceBundle } from "./discord";
import { sourceParametersFromFormFieldsProperties } from "./form-fields";

const sendAndWaitOperation = "sendAndWait";
const sendAndWaitArrayPropertyNames = [
  "approvalOptionsValues",
  "limitWaitTimeProperties",
];

type DiscordSendAndWaitExtractionOptions = {
  sourceBundle: DiscordSourceBundle;
  sharedObjectNames: string[];
};

export function normalizeDiscordSendAndWaitOperationConstant(source: string) {
  return source.replace(/\bSEND_AND_WAIT_OPERATION\b/g, `'${sendAndWaitOperation}'`);
}

function stringPropertyFromSource(
  source: string,
  propertyName: string,
  fallback: string,
) {
  const match = source.match(
    new RegExp(`${propertyName}\\s*:\\s*(['"\`])([\\s\\S]*?)\\1`),
  );

  return match?.[2] ?? fallback;
}

function sourceString(value: string) {
  return JSON.stringify(value);
}

function sendAndWaitSharedSource(sourceBundle: DiscordSourceBundle) {
  return [
    sourceBundle.commonDescriptionSource,
    sourceBundle.sendAndWaitUtilitiesSource,
    sourceBundle.sendAndWaitDescriptionsSource,
    sendAndWaitApprovalOptionsSource(sourceBundle),
  ].join("\n");
}

function sendAndWaitApprovalOptionsSource(sourceBundle: DiscordSourceBundle) {
  const approveLabel = stringPropertyFromSource(
    sourceBundle.messageSendAndWaitOperationSource,
    "defaultApproveLabel",
    "Approve",
  );
  const disapproveLabel = stringPropertyFromSource(
    sourceBundle.messageSendAndWaitOperationSource,
    "defaultDisapproveLabel",
    "Decline",
  );

  return `const approvalOptionsValues = [
{
displayName: 'Type of Approval',
name: 'approvalType',
type: 'options',
placeholder: 'Add option',
default: 'single',
options: [
{ name: 'Approve Only', value: 'single' },
{ name: 'Approve and Disapprove', value: 'double' },
],
},
{
displayName: 'Approve Button Label',
name: 'approveLabel',
type: 'string',
default: ${sourceString(approveLabel)},
displayOptions: { show: { approvalType: ['single', 'double'] } },
},
{
displayName: 'Disapprove Button Label',
name: 'disapproveLabel',
type: 'string',
default: ${sourceString(disapproveLabel)},
displayOptions: { show: { approvalType: ['double'] } },
},
];`;
}

function sendAndWaitPropertiesSource() {
  return `const properties = [
...sendToProperties,
{
displayName: 'Message',
name: 'message',
type: 'string',
default: '',
required: true,
typeOptions: { rows: 4 },
},
{
displayName: 'Response Type',
name: 'responseType',
type: 'options',
default: 'approval',
options: [
{
name: 'Approval',
value: 'approval',
description: 'User can approve/disapprove from within the message',
},
{
name: 'Free Text',
value: 'freeText',
description: 'User can submit a response via a form',
},
{
name: 'Custom Form',
value: 'customForm',
description: 'User can submit a response via a custom form',
},
],
},
{
displayName: 'Approval Options',
name: 'approvalOptions',
type: 'fixedCollection',
placeholder: 'Add option',
default: {},
options: [
{
displayName: 'Values',
name: 'values',
values: approvalOptionsValues,
},
],
displayOptions: { show: { responseType: ['approval'] } },
},
{
displayName: 'Options',
name: 'options',
type: 'collection',
placeholder: 'Add option',
default: {},
options: [limitWaitTimeOption, appendAttributionOption],
displayOptions: { show: { responseType: ['approval'] } },
},
{
displayName: 'Options',
name: 'options',
type: 'collection',
placeholder: 'Add option',
default: {},
options: [
{
displayName: 'Message Button Label',
name: 'messageButtonLabel',
type: 'string',
default: 'Respond',
},
{
displayName: 'Response Form Title',
name: 'responseFormTitle',
description: 'Title of the form that the user can access to provide their response',
type: 'string',
default: '',
},
{
displayName: 'Response Form Description',
description: 'Description of the form that the user can access to provide their response',
name: 'responseFormDescription',
type: 'string',
default: '',
},
{
displayName: 'Response Form Button Label',
name: 'responseFormButtonLabel',
type: 'string',
default: 'Submit',
},
{
displayName: 'Response Form Custom Styling',
name: 'responseFormCustomCss',
type: 'string',
typeOptions: { rows: 10, editor: 'cssEditor' },
default: '',
description: 'Override default styling of the response form with CSS',
},
limitWaitTimeOption,
appendAttributionOption,
],
displayOptions: { show: { responseType: ['freeText', 'customForm'] } },
},
];`;
}

function operationDisplayOptionsBlock() {
  return `{
show: {
resource: ['message'],
operation: ['${sendAndWaitOperation}'],
},
}`;
}

function customFormDisplayOptionsBlock() {
  return `{
show: {
resource: ['message'],
operation: ['${sendAndWaitOperation}'],
responseType: ['customForm'],
},
}`;
}

export function sourceParametersFromDiscordSendAndWaitOperation({
  sourceBundle,
  sharedObjectNames,
}: DiscordSendAndWaitExtractionOptions): NodeParameterDefinition[] {
  const entries = sourceEntriesForArray({
    source: sendAndWaitPropertiesSource(),
    arrayName: "properties",
    sharedSource: sendAndWaitSharedSource(sourceBundle),
    sharedObjectNames: [
      ...sharedObjectNames,
      "appendAttributionOption",
      "limitWaitTimeOption",
    ],
    sharedArraySpreadNames: ["sendToProperties"],
    sharedArrayPropertyNames: sendAndWaitArrayPropertyNames,
  }).map((entry) => applyN8nDisplayOptions(entry, operationDisplayOptionsBlock()));

  return [
    ...sourceParametersFromEntries(entries),
    ...sourceParametersFromFormFieldsProperties({
      sourceBundle,
      displayOptionsBlock: customFormDisplayOptionsBlock(),
    }),
  ];
}
