export type ForgeThemeMode = "light" | "dark" | "system";

export interface ForgeThemeProviderRecipe {
  name: "next-themes-style-theme-provider";
  status: "source-owned";
  attribute: "data-theme";
  defaultMode: ForgeThemeMode;
  availableModes: ForgeThemeMode[];
  storageKey: string;
  scriptless: true;
  runtimeBoundary: string;
}

export function createThemeProvider(defaultMode: ForgeThemeMode = "system"): ForgeThemeProviderRecipe {
  return {
    name: "next-themes-style-theme-provider",
    status: "source-owned",
    attribute: "data-theme",
    defaultMode,
    availableModes: ["light", "dark", "system"],
    storageKey: "dx-www-theme",
    scriptless: true,
    runtimeBoundary: "Persists a DX-WWW theme contract without importing next-themes or running client scripts.",
  };
}
