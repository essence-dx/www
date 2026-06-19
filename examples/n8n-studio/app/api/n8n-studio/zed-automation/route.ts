import { createZedAutomationBridgePacketFromLocalGeneratedSource } from "../../../../server/n8n-studio/zed-automation-bridge";

export function GET() {
  return Response.json(createZedAutomationBridgePacketFromLocalGeneratedSource());
}
