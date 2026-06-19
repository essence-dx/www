(() => {
        const key = "dx-landing-theme-v2";
        const root = document.documentElement;
        const scroller = document.querySelector(".dx-landing");
        const scrollTrack = document.querySelector(".dx-scrollbar");
        const scrollThumb = document.querySelector(".dx-scrollbar span");
        const buttons = Array.from(document.querySelectorAll("[data-theme-choice]"));
        const revealTargets = Array.from(document.querySelectorAll(
          ".landing-section, .landing-cta, .dx-footer, .platform-card, .feature-banner, .card-grid article, .story-grid article, .playbook-grid article, .theme-proof-grid article, .hero-image-grid figure, .hero-video-carousel"
        ));

        function applyTheme(theme) {
          const nextTheme = theme === "light" || theme === "dark" ? theme : "system";
          root.dataset.theme = nextTheme;
          localStorage.setItem(key, nextTheme);
          buttons.forEach((button) => {
            button.setAttribute("aria-pressed", String(button.dataset.themeChoice === nextTheme));
          });
        }

        function updateScrollbar() {
          if (!scroller || !scrollThumb) return;
          const max = Math.max(1, scroller.scrollHeight - scroller.clientHeight);
          const ratio = scroller.clientHeight / Math.max(scroller.scrollHeight, scroller.clientHeight);
          const track = Math.max(72, scroller.clientHeight - 56);
          const thumbHeight = Math.max(44, Math.round(track * ratio));
          const thumbTravel = Math.max(0, track - thumbHeight);
          const y = Math.round((scroller.scrollTop / max) * thumbTravel);
          scrollThumb.style.height = `${thumbHeight}px`;
          scrollThumb.style.transform = `translate3d(0, ${y}px, 0)`;
        }

        function scrollToTrackPoint(clientY) {
          if (!scroller || !scrollTrack || !scrollThumb) return;
          const rect = scrollTrack.getBoundingClientRect();
          const trackStyle = getComputedStyle(scrollTrack);
          const trackInset = parseFloat(trackStyle.paddingTop) || 0;
          const max = Math.max(1, scroller.scrollHeight - scroller.clientHeight);
          const track = Math.max(1, rect.height - trackInset * 2);
          const thumbHeight = scrollThumb.offsetHeight || 44;
          const travel = Math.max(1, track - thumbHeight);
          const localY = Math.min(Math.max(clientY - rect.top - trackInset - thumbHeight / 2, 0), travel);
          scroller.scrollTop = (localY / travel) * max;
          updateScrollbar();
        }

        applyTheme(localStorage.getItem(key) || "dark");
        buttons.forEach((button) => {
          button.addEventListener("click", () => applyTheme(button.dataset.themeChoice));
        });

        if (scroller) {
          scroller.addEventListener("scroll", updateScrollbar, { passive: true });
          window.addEventListener("resize", updateScrollbar);
          updateScrollbar();
        }

        if (scrollTrack && scroller) {
          let isDragging = false;

          scrollTrack.addEventListener("pointerdown", (event) => {
            event.preventDefault();
            isDragging = true;
            scrollTrack.classList.add("is-dragging");
            scrollTrack.setPointerCapture?.(event.pointerId);
            scrollToTrackPoint(event.clientY);
          });

          scrollTrack.addEventListener("pointermove", (event) => {
            if (!isDragging) return;
            event.preventDefault();
            scrollToTrackPoint(event.clientY);
          });

          const stopDragging = (event) => {
            if (!isDragging) return;
            isDragging = false;
            scrollTrack.classList.remove("is-dragging");
            scrollTrack.releasePointerCapture?.(event.pointerId);
          };

          scrollTrack.addEventListener("pointerup", stopDragging);
          scrollTrack.addEventListener("pointercancel", stopDragging);
          scrollTrack.addEventListener("lostpointercapture", () => {
            isDragging = false;
            scrollTrack.classList.remove("is-dragging");
          });
        }

        if ("IntersectionObserver" in window) {
          scroller?.setAttribute("data-reveal-ready", "true");
          const observer = new IntersectionObserver((entries) => {
            entries.forEach((entry) => {
              if (entry.isIntersecting) {
                entry.target.classList.add("is-visible");
                observer.unobserve(entry.target);
              }
            });
          }, { root: scroller, threshold: 0.12, rootMargin: "0px 0px -8% 0px" });
          revealTargets.forEach((target) => observer.observe(target));
        } else {
          revealTargets.forEach((target) => target.classList.add("is-visible"));
        }

        window.addEventListener("resize", updateScrollbar, { passive: true });
      })();
