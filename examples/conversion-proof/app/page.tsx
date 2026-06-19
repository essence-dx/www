import { LandingPageContent } from "@/lib/forge/www/landing-page";

export const metadata = {
  title: "WWW - Enhanced Development Experience",
  description: "The source-owned web framework for the DX ecosystem.",
} as const;

export default function LandingPage() {
  return <LandingPageContent />;
}
