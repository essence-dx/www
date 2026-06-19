import assert from "node:assert/strict";
import test from "node:test";

import {
  applyN8nDisplayOptions,
  credentialsFromVersionDescription,
  sourceEntriesForArray,
  sourceParametersFromEntries,
  sourceParametersFromVersionDescription,
} from "../examples/n8n-studio/lib/n8n-studio/source-parameters/source-description-reader";

const sharedSource = `
export const tableRLC = {
  displayName: 'Table',
  name: 'table',
  type: 'resourceLocator',
  default: { mode: 'list', value: '' },
  modes: [
    {
      displayName: 'From List',
      name: 'list',
      type: 'list',
      typeOptions: {
        searchListMethod: 'tableSearch',
        searchable: true,
      },
    },
  ],
};

export const operatorOptions = [
  {
    name: 'Equal',
    value: 'equal',
  },
];

export const whereFixedCollection = {
  displayName: 'Select Rows',
  name: 'where',
  type: 'fixedCollection',
  default: {},
  options: [
    {
      displayName: 'Values',
      name: 'values',
      values: [
        {
          displayName: 'Operator',
          name: 'condition',
          type: 'options',
          options: operatorOptions,
          default: 'equal',
        },
      ],
    },
  ],
};
`;

test("n8n source description reader expands shared objects and shared option arrays", () => {
  const entries = sourceEntriesForArray({
    source: `
export const description = [
  {
    ...tableRLC,
    displayOptions: {
      hide: {
        operation: ['executeQuery'],
      },
    },
  },
  whereFixedCollection,
];
`,
    arrayName: "description",
    sharedSource,
    sharedObjectNames: ["tableRLC", "whereFixedCollection"],
    sharedArrayPropertyNames: ["operatorOptions"],
  });
  const parameters = sourceParametersFromEntries(entries);
  const table = parameters.find((parameter) => parameter.name === "table");
  const where = parameters.find((parameter) => parameter.name === "where");

  assert.equal(table?.type, "resourceLocator");
  assert.equal(
    table?.resourceLocatorModes?.some(
      (mode) => mode.searchListMethod === "tableSearch",
    ),
    true,
  );
  assert.equal(where?.type, "fixedCollection");
  assert.equal(
    where?.childParameters?.some(
      (field) =>
        field.name === "condition" &&
        field.options?.some((option) => option.value === "equal"),
    ),
    true,
  );
});

test("n8n source description reader expands same-source array spreads", () => {
  const entries = sourceEntriesForArray({
    source: `
export const commonFields = [
  {
    displayName: 'Model',
    name: 'model',
    type: 'options',
    default: 'gpt-4.1',
    options: [
      {
        name: 'GPT-4.1',
        value: 'gpt-4.1',
      },
    ],
  },
];

export const description = [
  /* local model fields */
  ...commonFields,
  {
    displayName: 'Prompt',
    name: 'prompt',
    type: 'string',
    default: '',
  },
];
`,
    arrayName: "description",
    localArraySpreadNames: ["commonFields"],
  });
  const parameters = sourceParametersFromEntries(entries);

  assert.deepEqual(
    parameters.map((parameter) => parameter.name),
    ["model", "prompt"],
  );
});

test("n8n source description reader expands local and nested shared object spreads", () => {
  const entries = sourceEntriesForArray({
    source: `
export const readFilter = {
  displayName: 'Filters',
  name: 'filtersUI',
  type: 'fixedCollection',
  default: {},
  options: [
    {
      displayName: 'Filter',
      name: 'values',
      values: [
        {
          displayName: 'Column',
          name: 'lookupColumn',
          type: 'options',
          typeOptions: {
            loadOptionsMethod: 'getSheetHeaderRowWithGeneratedColumnNames',
          },
          default: '',
        },
      ],
    },
  ],
};

export const description = [
  {
    ...readFilter,
    displayOptions: {
      show: {
        resource: ['sheet'],
        operation: ['read'],
      },
    },
  },
  {
    displayName: 'Options',
    name: 'options',
    type: 'collection',
    default: {},
    options: [
      {
        ...handlingExtraData,
        displayOptions: {
          show: {
            '/columns.mappingMode': ['autoMapInputData'],
          },
        },
      },
    ],
  },
];
`,
    arrayName: "description",
    localObjectNames: ["readFilter"],
    sharedSource: `
export const handlingExtraData = {
  displayName: 'When Input Data Has Columns Not Present in Sheet',
  name: 'handlingExtraData',
  type: 'options',
  default: 'insertInNewColumn',
  options: [
    {
      name: 'Insert in New Column',
      value: 'insertInNewColumn',
    },
  ],
};
`,
    sharedObjectNames: ["handlingExtraData"],
  });
  const parameters = sourceParametersFromEntries(entries);
  const filters = parameters.find((parameter) => parameter.name === "filtersUI");
  const options = parameters.find((parameter) => parameter.name === "options");
  const handlingExtraData = options?.childParameters?.find(
    (parameter) => parameter.name === "handlingExtraData",
  );

  assert.equal(filters?.type, "fixedCollection");
  assert.equal(
    filters?.childParameters?.some(
      (parameter) =>
        parameter.name === "lookupColumn" &&
        parameter.dynamicOptions?.loadMethod ===
          "getSheetHeaderRowWithGeneratedColumnNames",
    ),
    true,
  );
  assert.equal(handlingExtraData?.type, "options");
  assert.deepEqual(
    handlingExtraData?.displayOptions?.show?.["/columns.mappingMode"],
    ["autoMapInputData"],
  );
});

test("n8n source description reader applies updateDisplayOptions merge semantics", () => {
  const entry = applyN8nDisplayOptions(
    `{
      displayName: 'Return All',
      name: 'returnAll',
      type: 'boolean',
      default: false,
      displayOptions: {
        show: {
          resource: ['event'],
          returnAll: [false],
        },
      },
    }`,
    `{
      show: {
        resource: ['database'],
        operation: ['select'],
      },
      hide: {
        table: [''],
      },
    }`,
  );
  const [parameter] = sourceParametersFromEntries([entry]);

  assert.equal(parameter.name, "returnAll");
  assert.deepEqual(parameter.displayOptions?.show?.resource, ["database"]);
  assert.deepEqual(parameter.displayOptions?.show?.operation, ["select"]);
  assert.deepEqual(parameter.displayOptions?.show?.returnAll, [false]);
  assert.deepEqual(parameter.displayOptions?.hide?.table, [""]);
});

test("n8n source description reader extracts version properties and credentials", () => {
  const versionSource = `
export const versionDescription = {
  credentials: [
    {
      name: 'postgres',
      required: true,
    },
  ],
  properties: [
    {
      displayName: 'Resource',
      name: 'resource',
      type: 'hidden',
      default: 'database',
    },
  ],
};
`;

  const [resource] = sourceParametersFromVersionDescription(versionSource);
  const credentials = credentialsFromVersionDescription(versionSource);

  assert.equal(resource.name, "resource");
  assert.equal(resource.type, "string");
  assert.deepEqual(credentials, [{ name: "postgres", required: true }]);
});

test("n8n source description reader preserves leading const parameters", () => {
  const versionSource = `
export const authentication = {
  displayName: 'Authentication',
  name: 'authentication',
  type: 'options',
  options: [
    {
      name: 'OAuth2',
      value: 'oAuth2',
    },
  ],
  default: 'oAuth2',
};

export const versionDescription = {
  properties: [
    {
      displayName: 'Resource',
      name: 'resource',
      type: 'hidden',
      default: 'sheet',
    },
  ],
};
`;

  const parameters = sourceParametersFromVersionDescription(versionSource, {
    leadingConstObjectNames: ["authentication"],
  });

  assert.deepEqual(
    parameters.map((parameter) => parameter.name),
    ["authentication", "resource"],
  );
  assert.equal(parameters[0]?.type, "options");
});
