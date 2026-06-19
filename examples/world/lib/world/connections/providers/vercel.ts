import type { WorldConnectionProbe } from "../contracts";
import { runVercelCliProbe } from "../local-cli";
import { httpProbe } from "./shared";

export const vercelConnectionProbes: readonly WorldConnectionProbe[] = [
  httpProbe(
    {
      id: "vercel-api-user",
      providerId: "vercel",
      packageId: "deployment/vercel",
      name: "Vercel REST API",
      category: "Deployment",
      kind: "http",
      endpoint: "https://api.vercel.com/v2/user",
      documentationUrl: "https://vercel.com/docs/rest-api/reference/endpoints/user/get-the-user",
      requiredEnv: ["VERCEL_TOKEN"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: "https://api.vercel.com/v2/user",
      headers: {
        Authorization: `Bearer ${env.VERCEL_TOKEN ?? ""}`,
      },
      expectedStatuses: [200],
      evidence: "authenticated-vercel-user",
    }),
  ),
  {
    id: "vercel-cli-whoami",
    providerId: "vercel",
    packageId: "deployment/vercel",
    name: "Vercel CLI",
    category: "Deployment",
    kind: "cli",
    endpoint: "local-cli:vercel whoami",
    documentationUrl: "https://vercel.com/docs/cli",
    requiredEnv: [],
    optionalEnv: [],
    run: runVercelCliProbe,
  },
];
