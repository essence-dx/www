import { createDatabaseOrmReadinessResponse } from "../../../../server/database-orm/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createDatabaseOrmReadinessResponse();
}
