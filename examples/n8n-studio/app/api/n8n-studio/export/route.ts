import {
  createExportResponseFromLocalGeneratedSource,
  createExportResponseFromPayload,
} from "../../../../server/n8n-studio/export-response";

export function GET() {
  return Response.json(createExportResponseFromLocalGeneratedSource());
}

export async function POST(request: Request) {
  let payload: unknown;

  try {
    payload = await request.json();
  } catch {
    payload = {};
  }

  return Response.json(createExportResponseFromPayload(payload));
}
