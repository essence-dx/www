(() => {
  const tooltip = document.querySelector("[data-dx-chart-tooltip]");

  const clampPosition = (value, max) => Math.max(12, Math.min(value, max - 12));

  const show = (event, label) => {
    if (!tooltip || !label) return;
    tooltip.textContent = label;
    tooltip.setAttribute("data-visible", "true");
    const pointer = "clientX" in event ? event : { clientX: 24, clientY: 24 };
    const x = clampPosition(pointer.clientX + 14, window.innerWidth || 320);
    const y = clampPosition(pointer.clientY + 14, window.innerHeight || 240);
    tooltip.style.transform = `translate(${x}px, ${y}px)`;
  };

  const hide = () => {
    if (!tooltip) return;
    tooltip.removeAttribute("data-visible");
    tooltip.textContent = "";
  };

  const chartTarget = (event) => event.target instanceof Element ? event.target.closest("[data-dx-chart-hit]") : null;

  const syncSelectedCount = (target) => {
    const frame = target.closest("[data-dx-chart-id]");
    if (frame) {
      frame.setAttribute("data-selected-count", String(frame.querySelectorAll("[data-selected]").length));
    }
  };

  const toggleSelection = (target) => {
    const selected = !target.hasAttribute("data-selected");
    target.toggleAttribute("data-selected", selected);
    target.setAttribute("aria-pressed", selected ? "true" : "false");
    syncSelectedCount(target);
  };

  document.addEventListener("pointermove", (event) => {
    const target = chartTarget(event);
    if (!target) {
      hide();
      return;
    }
    show(event, target.getAttribute("data-dx-chart-hit") || "");
  });

  document.addEventListener("focusin", (event) => {
    const target = chartTarget(event);
    if (target) show({ clientX: 24, clientY: 24 }, target.getAttribute("data-dx-chart-hit") || "");
  });

  document.addEventListener("focusout", hide);

  document.addEventListener("click", (event) => {
    const target = chartTarget(event);
    if (!target) return;
    toggleSelection(target);
    show(event, target.getAttribute("data-dx-chart-hit") || "");
  });

  document.addEventListener("keydown", (event) => {
    const target = chartTarget(event);
    if (!target) return;

    if (event.key === "Escape") {
      target.removeAttribute("data-selected");
      target.setAttribute("aria-pressed", "false");
      syncSelectedCount(target);
      hide();
      return;
    }

    if (event.key !== "Enter" && event.key !== " ") return;
    event.preventDefault();
    toggleSelection(target);
    show(event, target.getAttribute("data-dx-chart-hit") || "");
  });
})();
