import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createParameterSchemaForNode } from "../examples/n8n-studio/lib/n8n-studio/node-type-registry";
import { extractSlackMessageSourceParameters } from "../examples/n8n-studio/lib/n8n-studio/source-parameters/slack-message";

const repoRoot = path.resolve(import.meta.dirname, "..");
const slackMessageSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Slack",
  "V2",
  "MessageDescription.ts",
);

function readSlackMessageSource() {
  return fs.readFileSync(slackMessageSourcePath, "utf8");
}

test("n8n Studio models resource locator draft state and list-search boundaries", () => {
  const schema = createParameterSchemaForNode("n8n-nodes-base.slack", {
    resource: "message",
    operation: "post",
    select: "channel",
    channelId: {
      __rl: true,
      mode: "list",
      value: "C123",
      cachedResultName: "general",
    },
  });

  const channel = schema.fields.find((field) => field.name === "channelId");

  assert.equal(channel?.resourceLocatorDraft?.activeMode, "list");
  assert.equal(channel?.resourceLocatorDraft?.selectedValue, "C123");
  assert.equal(channel?.resourceLocatorDraft?.selectedLabel, "general");
  assert.equal(channel?.resourceLocatorDraft?.query, "general");
  assert.equal(channel?.resourceLocatorDraft?.listSearchMethod, "getChannels");
  assert.deepEqual(channel?.resourceLocatorDraft?.listBoundary, {
    status: "source-only",
    providerBoundary: true,
    liveProviderExecution: false,
    issue: "Dynamic channelId list/search requires the n8n editor-session adapter.",
  });

  const urlSchema = createParameterSchemaForNode("n8n-nodes-base.slack", {
    resource: "message",
    operation: "post",
    select: "channel",
    channelId: {
      mode: "url",
      value: "https://app.slack.com/client/T123/C123",
    },
  });
  const urlChannel = urlSchema.fields.find((field) => field.name === "channelId");
  assert.equal(urlChannel?.resourceLocatorDraft?.activeMode, "url");
  assert.equal(
    urlChannel?.resourceLocatorDraft?.selectedValue,
    "https://app.slack.com/client/T123/C123",
  );
});

test("n8n Studio preserves source resource locator URL modes", () => {
  const extraction = extractSlackMessageSourceParameters(readSlackMessageSource());
  const channel = extraction.parameters.find((parameter) => parameter.name === "channelId");

  assert.equal(
    channel?.resourceLocatorModes?.find((mode) => mode.name === "url")?.type,
    "url",
  );
});

test("n8n Studio renders resource locator controls without pretending live search runs", () => {
  const controlSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "parameter-field-control.tsx",
    ),
    "utf8",
  );

  assert.match(controlSource, /resourceLocatorDraft/);
  assert.match(controlSource, /data-resource-locator-mode/);
  assert.match(controlSource, /data-resource-locator-selected-value/);
  assert.match(controlSource, /data-resource-locator-search-boundary/);
  assert.match(controlSource, /liveProviderExecution/);
});
