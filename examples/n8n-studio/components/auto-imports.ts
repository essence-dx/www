/* Source-owned imports seed. Regenerate with dx imports sync after imports adoption. */
export * from "./icons/icon";
export * from "./n8n-studio/n8n-studio-app";

export const dxAutoImportMap = {
  "components": [
    {
      "kind": "component",
      "name": "N8nStudioApp",
      "source": "components/n8n-studio/n8n-studio-app.tsx",
      "import_path": "@/components/n8n-studio/n8n-studio-app",
      "barrel_export": "./n8n-studio/n8n-studio-app",
      "exports": ["N8nStudioApp", "N8nStudioAppProps"],
      "default_export_name": null,
      "used_exports": ["N8nStudioApp"],
      "unused_exports": ["N8nStudioAppProps"],
      "type_exports": ["N8nStudioAppProps"]
    },
    {
      "kind": "dx-icon",
      "name": "Icon",
      "source": "components/icons/icon.tsx",
      "import_path": "@/components/icons/icon",
      "barrel_export": "./icons/icon",
      "exports": ["Icon", "IconProps"],
      "default_export_name": null,
      "used_exports": ["Icon", "IconProps"],
      "unused_exports": [],
      "type_exports": ["IconProps"]
    }
  ],
  "composables": [],
  "utilities": [],
  "forgePackages": [
    {
      "packageId": "automations/n8n-studio",
      "source": "forge/automations/n8n-studio",
      "importPath": "@/forge/automations/n8n-studio",
      "exports": []
    }
  ],
  "styleHelpers": [
    {
      "name": "generated.css",
      "source": "styles/generated.css",
      "importPath": "@/styles/generated.css"
    },
    {
      "name": "theme.css",
      "source": "styles/theme.css",
      "importPath": "@/styles/theme.css"
    }
  ],
  "sourceHash": "pending-dx-imports-sync"
} as const;

export type DxAutoImportComponent = typeof dxAutoImportMap.components[number]["name"];
export type DxAutoImportComposable = typeof dxAutoImportMap.composables[number]["name"];
export type DxAutoImportUtility = typeof dxAutoImportMap.utilities[number]["name"];
export type DxAutoImportForgePackage = typeof dxAutoImportMap.forgePackages[number]["packageId"];
export type DxAutoImportStyleHelper = typeof dxAutoImportMap.styleHelpers[number]["name"];
