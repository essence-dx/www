import type { WorldConnectionProbe } from "../contracts";
import { httpProbe } from "./shared";

export const developerToolConnectionProbes: readonly WorldConnectionProbe[] = [
  httpProbe(
    {
      id: "github-authenticated-user",
      providerId: "github",
      packageId: "productivity/github",
      name: "GitHub authenticated user",
      category: "Developer tools",
      kind: "http",
      endpoint: "https://api.github.com/user",
      documentationUrl: "https://docs.github.com/rest/users/users#get-the-authenticated-user",
      requiredEnv: ["GITHUB_TOKEN"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: "https://api.github.com/user",
      headers: {
        Authorization: `Bearer ${env.GITHUB_TOKEN ?? ""}`,
        "X-GitHub-Api-Version": "2022-11-28",
      },
      expectedStatuses: [200],
      evidence: "authenticated-user-readable",
    }),
  ),
  httpProbe(
    {
      id: "linear-viewer",
      providerId: "linear",
      packageId: "productivity/linear",
      name: "Linear viewer",
      category: "Developer tools",
      kind: "http",
      endpoint: "https://api.linear.app/graphql",
      documentationUrl: "https://developers.linear.app/docs/graphql/working-with-the-graphql-api",
      requiredEnv: ["LINEAR_API_KEY"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: "https://api.linear.app/graphql",
      method: "POST",
      headers: {
        Authorization: env.LINEAR_API_KEY ?? "",
        "Content-Type": "application/json",
      },
      body: {
        query: "query DxWorldViewer { viewer { id name } }",
      },
      expectedStatuses: [200],
      evidence: "viewer-readable",
    }),
  ),
  httpProbe(
    {
      id: "notion-current-user",
      providerId: "notion",
      packageId: "productivity/notion",
      name: "Notion current user",
      category: "Developer tools",
      kind: "http",
      endpoint: "https://api.notion.com/v1/users/me",
      documentationUrl: "https://developers.notion.com/reference/get-self",
      requiredEnv: ["NOTION_TOKEN"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: "https://api.notion.com/v1/users/me",
      headers: {
        Authorization: `Bearer ${env.NOTION_TOKEN ?? ""}`,
        "Notion-Version": "2022-06-28",
      },
      expectedStatuses: [200],
      evidence: "current-user-readable",
    }),
  ),
];
