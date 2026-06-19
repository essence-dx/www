const fs = require("node:fs");

function summarizeRootRouteOutput(root, manifest, route, sourcePath) {
  const outputs = Array.isArray(manifest.route_outputs)
    ? manifest.route_outputs.filter((item) => item.route === route && item.source_path === sourcePath)
    : [];
  const output = outputs[0] || null;
  const parts = {
    entryModuleChunk: emitted(root, output?.entry_module_chunk_output),
    html: emitted(root, output?.html_output),
    packet: emitted(root, output?.packet_output),
    pageGraph: emitted(root, output?.page_graph_output),
    serverData: emitted(root, output?.server_data_output),
  };
  return {
    present: Object.values(parts).every((part) => part.present),
    route: output?.route || null,
    sourcePath: output?.source_path || null,
    duplicateCount: Math.max(0, outputs.length - 1),
    ...parts,
  };
}

function routeOutputFailures(output) {
  const failures = [];
  if (output.duplicateCount > 0) {
    failures.push(`source-build manifest has duplicate root route outputs for ${routeOutputLabel(output)}`);
  }
  return failures;
}

function routeOutputLabel(output) {
  return [output.route, output.sourcePath].filter(Boolean).join(" ") || "<unknown-route-output>";
}

function emitted(root, outputPath) {
  return {
    path: outputPath || null,
    present: outputPath ? fs.existsSync(`${root}/${outputPath}`) : false,
  };
}

module.exports = {
  routeOutputFailures,
  summarizeRootRouteOutput,
};
