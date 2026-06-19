import { dxMobileCompanionContract } from "../../lib/mobile-companion/companion-contract";

export function readDxMobileCompanionReadiness() {
  const nativeArtifacts = dxMobileCompanionContract.nativeArtifacts;

  return {
    app: dxMobileCompanionContract.appName,
    nativeShellStatus: "source-ready",
    pairingRuntimeProven: false,
    zedChatRuntimeProven: false,
    nativeSecureStorageProven: false,
    nativeSecureStorageStatus: "adapter-required-no-plaintext-fallback",
    nativeWwwOutputCurrent: false,
    nativeWwwOutputStatus: "stale-build-required",
    nativeWwwOutputRefreshCommand: nativeArtifacts.refreshCommand,
    nativeWwwOutputSourceRuntime: nativeArtifacts.sourceRuntime,
    nativeWwwOutputArtifactRuntime: nativeArtifacts.packagedRuntime,
    qrScannerStatus: "adapter-required",
    androidDebugApkSourceCurrent: false,
    androidDebugApkStatus: "stale-source-build-required",
    androidReleaseProven: false,
    iosReleaseProven: false,
    gatewayOriginPolicy: dxMobileCompanionContract.gatewayOriginPolicy,
    credentialTokenPolicy: dxMobileCompanionContract.credentialTokenPolicy,
    pairingPayloadPolicy: dxMobileCompanionContract.pairingPayload,
    identityValidationPolicy: dxMobileCompanionContract.identityValidationPolicy,
    identityValidationRuntimeProven: true,
    requiredGatewayEndpoints: dxMobileCompanionContract.gateway,
    requiredNativeCommands: dxMobileCompanionContract.nativeCommands,
    requiredNativeArtifacts: nativeArtifacts,
    securityBoundaries: dxMobileCompanionContract.securityBoundaries,
  } as const;
}
