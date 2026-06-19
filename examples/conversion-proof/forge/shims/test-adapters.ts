import { findLaunchRuntimeBoundary, launchRuntimeBoundaries } from "./runtime-boundaries";

export interface LaunchShimResult {
  id: string;
  ok: false;
  skipped: true;
  status: string;
  reason: string;
  future_work: string;
}

export function createLaunchShimResult(id: string): LaunchShimResult {
  const boundary = findLaunchRuntimeBoundary(id);
  if (!boundary) {
    return {
      id,
      ok: false,
      skipped: true,
      status: "missing-runtime",
      reason: "Unknown boundary id.",
      future_work: "Add the boundary to forge/shims/runtime-boundaries.ts before claiming coverage.",
    };
  }

  return {
    id: boundary.id,
    ok: false,
    skipped: true,
    status: boundary.status,
    reason: boundary.reason,
    future_work: boundary.future_work,
  };
}

export function listLaunchShimResults(): LaunchShimResult[] {
  return launchRuntimeBoundaries.map((boundary) => createLaunchShimResult(boundary.id));
}
