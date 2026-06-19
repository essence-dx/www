import { N8nStudioApp } from "../components/n8n-studio/n8n-studio-app";
import { createStudioBootFromLocalGeneratedSource } from "../server/n8n-studio/generated-catalog-source";

export default function N8nStudioPage() {
  const boot = createStudioBootFromLocalGeneratedSource();

  return (
    <N8nStudioApp
      nodeTypeRegistry={boot.nodeTypeRegistry}
      state={boot.state}
    />
  );
}
