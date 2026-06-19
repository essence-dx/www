const TURBO_TASKS_LANE_SCORE_FOR_SOURCE_ONLY_STATUS = 95;

const TURBO_TASKS_REQUIRED_ACTIONS = Object.freeze([
  "keep-turbo-tasks-as-reference-only-source-study",
  "wire-dx-owned-graph-invalidation-into-dx-build",
  "prove-dx-owned-cache-behavior-before-adding-a-runner",
  "keep-turbo-persistence-as-provenance-only",
]);

module.exports = {
  TURBO_TASKS_LANE_SCORE_FOR_SOURCE_ONLY_STATUS,
  TURBO_TASKS_REQUIRED_ACTIONS,
};
