import { dxMobileAuthContract } from "../../lib/mobile-auth/auth-contract";

export const dxMobileAuthReadiness = {
  schema: "dx.mobile.auth.readiness",
  app: dxMobileAuthContract.appName,
  runtimeStatus: dxMobileAuthContract.runtimeStatus,
  nativeShellStatus: "source-ready",
  nativeBridge: dxMobileAuthContract.nativeBridge,
  nativeShellReceipt: dxMobileAuthContract.nativeShellReceipt,
  checkSurface: dxMobileAuthContract.checkSurface,
  authProviderPackage: dxMobileAuthContract.authProviderPackage,
  status: "source-ready",
  authRuntimeProven: false,
  nativeReleaseProven: false,
  appOwnedRuntimeBoundaries: dxMobileAuthContract.boundaries,
} as const;

export function readDxMobileAuthReadiness() {
  return dxMobileAuthReadiness;
}
