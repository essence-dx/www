import { createDatabaseApiReadinessResponse } from "../../../../server/database-api/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createDatabaseApiReadinessResponse();
}
