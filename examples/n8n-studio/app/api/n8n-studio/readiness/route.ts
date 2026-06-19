import { createReadinessResponseFromLocalGeneratedSource } from "../../../../server/n8n-studio/readiness-response";

export function GET() {
  return Response.json(createReadinessResponseFromLocalGeneratedSource());
}
