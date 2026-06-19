const CURRENT_WORKER_CHECKPOINT = Object.freeze({
  kind: "historical-benchmark-snapshot",
  not_current_release_readiness: true,
  current_worker_checkpoint_date: "2026-05-24",
  current_worker_checkpoint_score: "84/100",
  current_worker_checkpoint_blockers: Object.freeze([
    "62 Rust warnings",
    "chaotic worktree",
    "source-guard-heavy evidence",
    "missing browser screenshot proof",
    "missing overlay recovery proof",
    "generated artifact curation",
    "full cargo test and clippy not rerun after curation",
  ]),
});

function buildHistoricalBenchmarkSnapshotStatus() {
  return {
    ...CURRENT_WORKER_CHECKPOINT,
    current_worker_checkpoint_blockers: [...CURRENT_WORKER_CHECKPOINT.current_worker_checkpoint_blockers],
  };
}

function snapshotStatusMarkdownBlock() {
  const status = buildHistoricalBenchmarkSnapshotStatus();
  return [
    "> Historical benchmark snapshot, not the current release-readiness score. The",
    `> current 30-agent worker checkpoint is ${status.current_worker_checkpoint_score} on ${status.current_worker_checkpoint_date} with 62 Rust`,
    "> warnings, generated artifact curation, and missing browser/overlay/full-clippy proof still blocking 95+.",
    "",
  ].join("\n");
}

module.exports = {
  buildHistoricalBenchmarkSnapshotStatus,
  snapshotStatusMarkdownBlock,
};
