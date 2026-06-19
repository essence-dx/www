import { Icon } from "@/components/icons/icon";

export function LandingProofGrid() {
  return (
    <div className="grid gap-3 md:grid-cols-3" aria-label="DX ecosystem proof points">
      <article className="rounded-lg border border-border bg-card p-5" data-dx-proof-point="www">
        <div className="mb-5 flex items-center justify-between">
          <Icon name="pack:motion" className="size-5 text-primary" />
          <span className="text-xs font-black uppercase text-muted-foreground">WWW</span>
        </div>
        <h2 className="text-xl font-black text-foreground">TSX-first route</h2>
        <p className="mt-3 text-sm leading-6 text-muted-foreground">
          The public landing now has a WWW App Router source instead of only a static export.
        </p>
      </article>

      <article className="rounded-lg border border-border bg-card p-5" data-dx-proof-point="style">
        <div className="mb-5 flex items-center justify-between">
          <Icon name="pack:ui-components" className="size-5 text-primary" />
          <span className="text-xs font-black uppercase text-muted-foreground">Style</span>
        </div>
        <h2 className="text-xl font-black text-foreground">Style surface</h2>
        <p className="mt-3 text-sm leading-6 text-muted-foreground">
          Layout, typography, color, spacing, and motion are expressed through Style classes.
        </p>
      </article>

      <article className="rounded-lg border border-border bg-card p-5" data-dx-proof-point="forge">
        <div className="mb-5 flex items-center justify-between">
          <Icon name="pack:state" className="size-5 text-primary" />
          <span className="text-xs font-black uppercase text-muted-foreground">Forge</span>
        </div>
        <h2 className="text-xl font-black text-foreground">Source-owned identity</h2>
        <p className="mt-3 text-sm leading-6 text-muted-foreground">
          Footer copy, proof points, and package metadata live in a Forge-owned source slice.
        </p>
      </article>
    </div>
  );
}

export function LandingIdentityFooter() {
  return (
    <footer
      className="grid min-h-screen items-end overflow-hidden border-t border-border px-6 py-10 md:px-10"
      id="dx-footer"
      data-dx-section="footer"
      data-dx-style-motion="big-footer-timeframe"
      data-forge-package="www/landing-identity"
    >
      <div className="grid w-full gap-8">
        <div className="flex flex-wrap items-center justify-between gap-4">
          <p className="text-sm font-black uppercase text-muted-foreground">
            Enhanced Development Experience
          </p>
          <p className="text-sm font-black uppercase text-muted-foreground">
            Forge-owned identity package
          </p>
        </div>

        <div className="grid gap-2" aria-label="DX footer identity animation">
          <strong
            className="block text-6xl font-black uppercase leading-none text-foreground md:text-9xl animate-slide-up-1s-delay-200ms-both"
            data-dx-timeframe-step="01"
            data-dx-motion-source="dx-style"
            motion="animate-slide-up-1s-delay-200ms-both"
          >
            Enhanced Development Experience
          </strong>
          <strong
            className="block text-6xl font-black uppercase leading-none text-foreground md:text-9xl animate-slide-up-1s-delay-700ms-both"
            data-dx-timeframe-step="02"
            data-dx-motion-source="dx-style"
            motion="animate-slide-up-1s-delay-700ms-both"
          >
            I use Dx BTW
          </strong>
          <strong
            className="block text-6xl font-black uppercase leading-none text-foreground md:text-9xl animate-slide-up-1s-delay-1000ms-both"
            data-dx-timeframe-step="03"
            data-dx-motion-source="dx-style"
            motion="animate-slide-up-1s-delay-1000ms-both"
          >
            WWW
          </strong>
          <strong
            className="block text-6xl font-black uppercase leading-none text-foreground md:text-9xl animate-bounce-2s-ease-in-out"
            data-dx-timeframe-step="04"
            data-dx-motion-source="dx-style"
            motion="animate-bounce-2s-ease-in-out"
          >
            I use Dx BTW
          </strong>
        </div>
      </div>
    </footer>
  );
}
