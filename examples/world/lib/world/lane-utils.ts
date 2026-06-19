import type { WorldCategory, WorldIntegration } from "./contracts";

type IntegrationInput = Omit<
  WorldIntegration,
  "categoryId" | "category" | "packageId" | "supportMode" | "routeHandlers" | "nextAction"
> &
  Partial<Pick<WorldIntegration, "packageId" | "supportMode" | "routeHandlers" | "nextAction">>;

export function category(
  id: string,
  title: string,
  purpose: string,
  providers: readonly IntegrationInput[],
): WorldCategory {
  return {
    id,
    title,
    purpose,
    providers: providers.map((provider) => ({
      ...provider,
      categoryId: id,
      category: title,
      packageId: provider.packageId ?? `${id}/${provider.id}`,
      supportMode: provider.supportMode ?? "source-owned-preview",
      routeHandlers: provider.routeHandlers ?? [
        `GET /api/world/provider?provider=${provider.id}`,
      ],
      nextAction:
        provider.nextAction ??
        "Add Env Firewall values, run a read-only provider check, then import a redacted receipt.",
    })),
  };
}
