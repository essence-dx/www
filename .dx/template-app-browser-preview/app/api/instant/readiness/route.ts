import { createInstantReadinessResponse } from "../../../../server/instant/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createInstantReadinessResponse();
}
