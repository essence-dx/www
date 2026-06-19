import { createEditorSessionRequestBatchResponseFromLocalGeneratedSource } from "../../../../server/n8n-studio/editor-session-request";
import { createEditorSessionResponseBatchResponseFromLocalGeneratedSource } from "../../../../server/n8n-studio/editor-session-response";

export async function GET() {
  return Response.json(
    createEditorSessionRequestBatchResponseFromLocalGeneratedSource(),
  );
}

export async function POST(request: Request) {
  let payload: unknown;

  try {
    payload = await request.json();
  } catch {
    payload = { responses: [] };
  }

  return Response.json(
    createEditorSessionResponseBatchResponseFromLocalGeneratedSource(payload),
  );
}
