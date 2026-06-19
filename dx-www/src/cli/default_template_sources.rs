#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DefaultTemplateAppRouteSource {
    pub(crate) route: &'static str,
    pub(crate) aliases: &'static [&'static str],
    pub(crate) source_file: &'static str,
    pub(crate) materialized_file: &'static str,
    pub(crate) role: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DefaultTemplateSourceFile {
    pub(crate) source_file: &'static str,
    pub(crate) materialized_file: &'static str,
    pub(crate) role: &'static str,
}

pub(crate) const DEFAULT_TEMPLATE_HOME_ROUTE_SOURCE_FILE: &str = "examples/template/app/page.tsx";

pub(crate) const DEFAULT_TEMPLATE_APP_ROUTE_SOURCES: &[DefaultTemplateAppRouteSource] =
    &[DefaultTemplateAppRouteSource {
        route: "/",
        aliases: &[],
        source_file: DEFAULT_TEMPLATE_HOME_ROUTE_SOURCE_FILE,
        materialized_file: "app/page.tsx",
        role: "minimal-www-home-page",
    }];

pub(crate) const DEFAULT_TEMPLATE_CORE_SOURCE_FILES: &[DefaultTemplateSourceFile] = &[
    DefaultTemplateSourceFile {
        source_file: DEFAULT_TEMPLATE_HOME_ROUTE_SOURCE_FILE,
        materialized_file: "app/page.tsx",
        role: "minimal-www-home-page",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/app/layout.tsx",
        materialized_file: "app/layout.tsx",
        role: "minimal-www-root-layout",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/styles/theme.css",
        materialized_file: "styles/theme.css",
        role: "minimal-www-theme-tokens",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/styles/generated.css",
        materialized_file: "styles/generated.css",
        role: "minimal-www-generated-css",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/styles/globals.css",
        materialized_file: "styles/globals.css",
        role: "minimal-www-global-css",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/components/icons/icon.tsx",
        materialized_file: "components/icons/icon.tsx",
        role: "minimal-www-source-owned-icon-component",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/lib/utils.ts",
        materialized_file: "lib/utils.ts",
        role: "minimal-www-source-owned-utility-helpers",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/lib/stores/counter.ts",
        materialized_file: "lib/stores/counter.ts",
        role: "minimal-www-global-store-convention",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/public/logo.svg",
        materialized_file: "public/logo.svg",
        role: "minimal-www-logo-asset",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/public/icon.svg",
        materialized_file: "public/icon.svg",
        role: "minimal-www-icon-asset",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/public/favicon.svg",
        materialized_file: "public/favicon.svg",
        role: "minimal-www-favicon-asset",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/vercel.json",
        materialized_file: "vercel.json",
        role: "minimal-www-vercel-deploy-policy",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/.gitignore",
        materialized_file: ".gitignore",
        role: "minimal-www-gitignore-policy",
    },
    DefaultTemplateSourceFile {
        source_file: "examples/template/README.md",
        materialized_file: "README.md",
        role: "minimal-www-readme",
    },
];
