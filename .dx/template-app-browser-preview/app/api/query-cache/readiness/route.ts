import {
  createDataFetchingCacheActionResponse,
  createDataFetchingCacheReadinessResponse,
} from "@/server/query-cache/readiness";

export const dynamic = "force-dynamic";

export function GET(request: Request) {
  return createDataFetchingCacheReadinessResponse(request);
}

export async function POST(request: Request) {
  return createDataFetchingCacheActionResponse(request);
}
