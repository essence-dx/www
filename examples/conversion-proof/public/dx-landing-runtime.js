(() => {
        function initLandingRuntime() {
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
          try {
            localStorage.setItem(key, nextTheme);
          } catch (_) {
            root.dataset.theme = nextTheme;
          }
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
          scroller.style.scrollBehavior = "auto";
          scroller.scrollTop = (localY / travel) * max;
          updateScrollbar();
        }

        applyTheme(localStorage.getItem(key) || "dark");
        buttons.forEach((button) => {
          button.addEventListener("click", () => applyTheme(button.dataset.themeChoice));
        });

        function setupCarousel(carousel) {
          if (carousel.dataset.dxCarouselReady === "true") return;
          const track = carousel.querySelector("[data-dx-carousel-track]");
          const slides = Array.from(carousel.querySelectorAll("[data-dx-carousel-slide]"));
          const dots = Array.from(carousel.querySelectorAll("[data-dx-carousel-dot]"));
          const previous = carousel.querySelector("[data-dx-carousel-prev]");
          const next = carousel.querySelector("[data-dx-carousel-next]");
          const autoplay = carousel.dataset.dxCarouselAutoplay === "true";
          const duration = Number(carousel.dataset.dxCarouselInterval || 5200);
          let index = 0;
          let timer = 0;

          if (!track || slides.length < 2) return;
          carousel.dataset.dxCarouselReady = "true";

          function normalize(value) {
            return (value + slides.length) % slides.length;
          }

          function render(value) {
            index = normalize(value);
            carousel.dataset.dxCarouselIndex = String(index);
            track.style.setProperty("--dx-carousel-index", String(index));
            track.style.transform = `translate3d(-${index * 100}%, 0, 0)`;
            slides.forEach((slide, slideIndex) => {
              const active = slideIndex === index;
              slide.classList.toggle("is-active", active);
              slide.setAttribute("aria-hidden", String(!active));
            });
            dots.forEach((dot) => {
              const dotIndex = Number(dot.dataset.dxCarouselIndex || 0);
              const active = dotIndex === index;
              dot.classList.toggle("is-active", active);
              dot.setAttribute("aria-selected", String(active));
              dot.setAttribute("aria-current", active ? "true" : "false");
            });
          }

          function stop() {
            if (!timer) return;
            window.clearInterval(timer);
            timer = 0;
          }

          function start() {
            if (!autoplay || timer) return;
            timer = window.setInterval(() => render(index + 1), duration);
          }

          previous?.addEventListener("click", () => {
            render(index - 1);
            stop();
            window.setTimeout(start, 3200);
          });

          next?.addEventListener("click", () => {
            render(index + 1);
            stop();
            window.setTimeout(start, 3200);
          });

          dots.forEach((dot) => {
            dot.addEventListener("click", () => {
              render(Number(dot.dataset.dxCarouselIndex || 0));
              stop();
              window.setTimeout(start, 3200);
            });
          });

          carousel.addEventListener("focusin", stop);
          carousel.addEventListener("focusout", start);

          render(0);
          start();
        }

        document.querySelectorAll("[data-dx-carousel]").forEach(setupCarousel);

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

          scrollTrack.addEventListener("wheel", (event) => {
            event.preventDefault();
            scroller.style.scrollBehavior = "auto";
            scroller.scrollTop += event.deltaY * 2.25;
            updateScrollbar();
          }, { passive: false });

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
        }

        if (document.readyState === "loading") {
          document.addEventListener("DOMContentLoaded", initLandingRuntime, { once: true });
        } else {
          initLandingRuntime();
        }
      })();
