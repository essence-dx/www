const sceneHost = document.querySelector(".onboard-spline-layer[data-spline-frame]");

if (sceneHost && !sceneHost.querySelector("iframe")) {
  const frame = document.createElement("iframe");
  frame.className = "onboard-spline-frame";
  frame.src = sceneHost.getAttribute("data-spline-frame") || "";
  frame.title = "DX Spline scene";
  frame.width = "100%";
  frame.height = "100%";
  frame.frameBorder = "0";
  frame.allow = "autoplay; fullscreen; xr-spatial-tracking";
  sceneHost.appendChild(frame);
}
