import { createCatalogResponseFromLocalGeneratedSource } from "../../../../server/n8n-studio/generated-catalog-source";

export function GET() {
  return Response.json(createCatalogResponseFromLocalGeneratedSource());
}
