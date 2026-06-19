import {
  launchPackageCatalog,
  launchPackageRoleSummary,
  requiredLaunchEnv,
  type LaunchPackageRole,
} from "./package-catalog";

export type LaunchQueryDashboardPackage = {
  appOwnedBoundaryCount: number;
  command: string;
  displayName: string;
  packageId: string;
  receiptCount: number;
  requiredEnvCount: number;
  role: LaunchPackageRole;
  status: "needs-env" | "ready";
};

export type LaunchQueryDashboardData = {
  appOwnedBoundaryCount: number;
  dashboardPackages: readonly LaunchQueryDashboardPackage[];
  packageCount: number;
  readAt: string;
  requiredEnvCount: number;
  roleCount: number;
  source: "launch-package-catalog";
  status: "ready";
};

const preferredPackageOrder = [
  "tanstack/query",
  "auth/better-auth",
  "payments/stripe-js",
  "state/zustand",
  "validation/zod",
  "api/trpc",
] as const;

export function readLaunchQueryDashboardPackages(): readonly LaunchQueryDashboardPackage[] {
  return preferredPackageOrder.flatMap((packageId) => {
    const item = launchPackageCatalog.find(
      (catalogItem) => catalogItem.packageId === packageId,
    );

    if (!item) return [];

    const requiredEnv = new Set([
      ...item.env,
      ...(item.requiredEnv ?? []),
    ]);

    return [
      {
        appOwnedBoundaryCount: item.appOwnedBoundaries.length,
        command: item.command,
        displayName: item.officialName ?? item.packageId,
        packageId: item.packageId,
        receiptCount: item.receiptPaths?.length ?? 0,
        requiredEnvCount: requiredEnv.size,
        role: item.role,
        status: requiredEnv.size > 0 ? "needs-env" : "ready",
      },
    ];
  });
}

export function readLaunchQueryDashboardData(): LaunchQueryDashboardData {
  return {
    appOwnedBoundaryCount: launchPackageCatalog.reduce(
      (boundaryCount, item) => boundaryCount + item.appOwnedBoundaries.length,
      0,
    ),
    dashboardPackages: readLaunchQueryDashboardPackages(),
    packageCount: launchPackageCatalog.length,
    readAt: new Date().toISOString(),
    requiredEnvCount: requiredLaunchEnv().length,
    roleCount: launchPackageRoleSummary().length,
    source: "launch-package-catalog",
    status: "ready",
  };
}
