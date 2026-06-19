import { betterAuthDashboardPackage } from "../../auth/better-auth/dashboard";

export const dxMobileAuthContract = {
  appName: "DX Mobile App",
  wwwRuntime: "DX WWW App Router",
  nativeRuntime: "DX Native",
  nativeBridge: "dx-native-mobile-auth",
  nativeShell: "tauri-webview",
  nativeConfig: "src-tauri/tauri.conf.json",
  nativeShellReceipt: ".dx/native/native-shell-receipt.json",
  nativeBundleId: "com.essencefromexistence.dx.mobile",
  styleSurface: "dx-mobile-auth-screen",
  checkSurface: "dx-mobile-auth",
  authProviderPackage: "auth/better-auth",
  authOfficialName: betterAuthDashboardPackage.officialName,
  primaryAction: "email-password-sign-in",
  runtimeStatus: "source-ready",
  receiptPath: ".dx/forge/receipts/auth-better-auth.json",
  boundaries: [
    "Database adapter and migrations",
    "Session cookie and trusted-origin policy",
    "OAuth credentials and callback URLs",
  ],
} as const;

export type DxMobileAuthContract = typeof dxMobileAuthContract;
