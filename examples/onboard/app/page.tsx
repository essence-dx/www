export const metadata = {
  title: "DX Onboard",
  description: "HelloGlow and Friday running inside DX WWW.",
} as const;

export default function OnboardPage() {
  return (
    <main className="dx-home-page" data-dx-route="/" data-dx-template="onboard">
      {/*
      <script
        type="module"
        src="/public/onboard-spline.js"
        data-onboard-spline-loader="source-owned"
      ></script>
      */}
      <section
        className="onboard-effects-only"
        aria-label="HelloGlow and Friday controls"
        data-dx-component="dx-onboard-effects"
      >
        {/*
        <div
          className="onboard-spline-layer"
          data-spline-frame="https://my.spline.design/authentication-pCrGxnULOxeQ1U46tami7m1e/"
          aria-hidden="true"
        ></div>
        */}

        <div className="dx-ref-hello onboard-hello-only" aria-hidden="true">
          <span className="start">0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span>0</span>
          <span className="end">0</span>
        </div>

        <div className="onboard-friday-control">
          <input className="dx-ref-friday-toggle" id="dx-ref-friday-active" type="checkbox" />
          <label className="dx-ref-friday-button" htmlFor="dx-ref-friday-active" role="button">
            <span className="dx-ref-friday-label-active">Active</span>
            <span className="dx-ref-friday-label-disabled">Disabled</span>
          </label>
        </div>
      </section>
    </main>
  );
}
