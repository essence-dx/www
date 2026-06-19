const { SCHEMAS } = require("./constants.js");
const {
  buildActiveScopeReport,
  buildActiveScopeSummary,
} = require("./active-scope.js");

function buildConsumerSurfacesReport(statusSurface) {
  const activeScope = statusSurface.activeScope ?? buildActiveScopeReport(statusSurface);
  const activeScopeSummary =
    statusSurface.activeScopeSummary ?? buildActiveScopeSummary(activeScope);

  return {
    schema: SCHEMAS.consumerSurfaces,
    status: statusSurface.status,
    source: {
      statusSurface: statusSurface.surface.id,
      owner: statusSurface.surface.owner,
      adapterBoundary: statusSurface.surface.adapterBoundary,
      receiptPaths: [
        statusSurface.evidence.sourceReceipt.path,
        statusSurface.evidence.consumerReceipt.path,
      ],
    },
    dxCheck: statusSurface.dxCheck,
    editorSurfaces: statusSurface.editorSurfaces,
    blockers: statusSurface.blockers,
    unproven: statusSurface.unproven,
    activeScope,
    activeScopeSummary,
  };
}

module.exports = {
  buildConsumerSurfacesReport,
};
