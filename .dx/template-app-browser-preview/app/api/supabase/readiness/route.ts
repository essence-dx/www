import { createSupabaseReadinessResponse } from "../../../../server/supabase/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createSupabaseReadinessResponse();
}
