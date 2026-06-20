    #[test]
    fn migration_static_site_materializes_honest_source_owned_example() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(
            canonical_package_id("wordpress/static-site"),
            "migration/static-site"
        );

        let package = source_package_for_project("migration/static-site", dir.path())
            .expect("migration package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let readme = package
            .files
            .iter()
            .find(|file| file.path == "migrations/static-site/README.md")
            .and_then(|file| file.content.as_deref())
            .expect("migration readme");

        assert_eq!(package.package_id, "migration/static-site");
        assert_eq!(package.upstream_name, "@dx/forge-migrations");
        assert!(paths.contains(&"migrations/static-site/content.ts"));
        assert!(paths.contains(&"migrations/static-site/page.tsx"));
        assert!(paths.contains(&"migrations/static-site/sample-wordpress-export.json"));
        assert!(paths.contains(&"migrations/static-site/README.md"));
        assert!(readme.contains("not a full WordPress plugin or theme migration"));
        assert!(readme.contains("No package install is required"));
        assert_eq!(paths.len(), 4);
    }

    #[test]
    fn shadcn_button_materializes_support_files() {
        let dir = tempdir().expect("tempdir");
        let package = source_package_for_project("shadcn/ui/button", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert!(paths.contains(&"components/ui/button.tsx"));
        assert!(paths.contains(&"components/ui/slot.tsx"));
        assert!(paths.contains(&"lib/utils.ts"));
    }

    #[test]
    fn icon_search_materializes_only_selected_icon_and_helper() {
        let dir = tempdir().expect("tempdir");
        let package = source_package_for_project("icon/search", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert!(paths.contains(&"components/icons/search.tsx"));
        assert!(paths.contains(&"components/icons/README.md"));
        assert!(paths.contains(&"lib/icons.ts"));
        assert_eq!(
            paths.iter().filter(|path| path.ends_with(".tsx")).count(),
            1
        );
        assert!(!paths.contains(&"components/ui/button.tsx"));
    }

    #[test]
    fn better_auth_materializes_real_next_auth_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("authentication"), "auth/better-auth");
        assert_eq!(canonical_package_id("better-auth"), "auth/better-auth");
        let package = source_package_for_project("authentication", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "auth/better-auth");
        assert_eq!(package.version, "1.6.11-dx.9");
        assert!(paths.contains(&"auth/better-auth/options.ts"));
        assert!(paths.contains(&"auth/better-auth/server.ts"));
        assert!(paths.contains(&"auth/better-auth/client.ts"));
        assert!(paths.contains(&"auth/better-auth/email-password.ts"));
        assert!(paths.contains(&"auth/better-auth/social.ts"));
        assert!(paths.contains(&"auth/better-auth/accounts.ts"));
        assert!(paths.contains(&"auth/better-auth/profile.ts"));
        assert!(paths.contains(&"auth/better-auth/account-deletion.ts"));
        assert!(paths.contains(&"auth/better-auth/account-security.ts"));
        assert!(paths.contains(&"auth/better-auth/route.ts"));
        assert!(paths.contains(&"auth/better-auth/session.ts"));
        assert!(paths.contains(&"auth/better-auth/session-management.ts"));
        assert!(paths.contains(&"auth/better-auth/metadata.ts"));
        assert!(paths.contains(&"auth/better-auth/.env.example"));
        assert!(paths.contains(&"auth/better-auth/README.md"));

        let server = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/server.ts")
            .and_then(|file| file.content.as_deref())
            .expect("server source");
        assert!(
            server.contains(r#"import { betterAuth, type BetterAuthOptions } from "better-auth";"#)
        );
        assert!(server.contains(r#"import { nextCookies } from "better-auth/next-js";"#));
        assert!(server.contains("plugins: [...plugins, nextCookies()]"));

        let route = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("route source");
        assert!(route.contains("toNextJsHandler(auth)"));

        let client = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/client.ts")
            .and_then(|file| file.content.as_deref())
            .expect("client source");
        assert!(client.contains("createAuthClient"));
        assert!(client.contains("useSession"));
        assert!(client.contains("signOut"));
        assert!(client.contains("listSessions"));
        assert!(client.contains("revokeOtherSessions"));
        assert!(client.contains("revokeSessions"));
        assert!(client.contains("requestPasswordReset"));
        assert!(client.contains("resetPassword"));
        assert!(client.contains("changePassword"));
        assert!(client.contains("sendVerificationEmail"));
        assert!(client.contains("listAccounts"));
        assert!(client.contains("linkSocial"));
        assert!(client.contains("unlinkAccount"));
        assert!(client.contains("getAccessToken"));
        assert!(client.contains("updateUser"));
        assert!(client.contains("changeEmail"));
        assert!(client.contains("deleteUser"));

        let email_password = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/email-password.ts")
            .and_then(|file| file.content.as_deref())
            .expect("email password source");
        assert!(email_password.contains("signIn.email"));
        assert!(email_password.contains("signUp.email"));
        assert!(email_password.contains("signInDxBetterAuthEmail"));
        assert!(email_password.contains("signUpDxBetterAuthEmail"));

        let social = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/social.ts")
            .and_then(|file| file.content.as_deref())
            .expect("social source");
        assert!(social.contains("signIn.social"));
        assert!(social.contains("signInDxBetterAuthSocial"));
        assert!(social.contains("DxBetterAuthSocialProvider"));

        let accounts = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/accounts.ts")
            .and_then(|file| file.content.as_deref())
            .expect("accounts source");
        assert!(accounts.contains("listAccounts"));
        assert!(accounts.contains("linkSocial"));
        assert!(accounts.contains("unlinkAccount"));
        assert!(accounts.contains("getAccessToken"));
        assert!(accounts.contains("linkDxBetterAuthSocialAccount"));

        let profile = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/profile.ts")
            .and_then(|file| file.content.as_deref())
            .expect("profile source");
        assert!(profile.contains("updateUser"));
        assert!(profile.contains("changeEmail"));
        assert!(profile.contains("updateDxBetterAuthUserProfile"));
        assert!(profile.contains("changeDxBetterAuthEmail"));
        assert!(profile.contains("normalizeDxBetterAuthEmail"));

        let account_deletion = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/account-deletion.ts")
            .and_then(|file| file.content.as_deref())
            .expect("account deletion source");
        assert!(account_deletion.contains("deleteUser"));
        assert!(account_deletion.contains("deleteDxBetterAuthAccount"));
        assert!(account_deletion.contains("DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION"));
        assert!(account_deletion.contains("allowFreshSessionDelete"));

        let account_security = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/account-security.ts")
            .and_then(|file| file.content.as_deref())
            .expect("account security source");
        assert!(account_security.contains("requestPasswordReset"));
        assert!(account_security.contains("resetPassword"));
        assert!(account_security.contains("changePassword"));
        assert!(account_security.contains("sendVerificationEmail"));
        assert!(account_security.contains("requestDxBetterAuthPasswordReset"));
        assert!(account_security.contains("changeDxBetterAuthPassword"));

        let session = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/session.ts")
            .and_then(|file| file.content.as_deref())
            .expect("session source");
        assert!(session.contains(r#"import { headers } from "next/headers";"#));
        assert!(session.contains("auth.api.getSession"));
        assert!(session.contains("getDxBetterAuthSession"));

        let session_management = package
            .files
            .iter()
            .find(|file| file.path == "auth/better-auth/session-management.ts")
            .and_then(|file| file.content.as_deref())
            .expect("session management source");
        assert!(session_management.contains("auth.api.listSessions"));
        assert!(session_management.contains("auth.api.revokeSession"));
        assert!(session_management.contains("auth.api.revokeOtherSessions"));
        assert!(session_management.contains("auth.api.revokeSessions"));
    }

    #[test]
    fn motion_react_alias_materializes_launch_animation_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("motion"), "animation/motion");
        assert_eq!(canonical_package_id("motion-animation"), "animation/motion");
        assert_eq!(
            canonical_package_id("motion-and-animation"),
            "animation/motion"
        );
        assert_eq!(canonical_package_id("motion/react"), "animation/motion");
        assert_eq!(canonical_package_id("framer-motion"), "animation/motion");

        let package = source_package_for_project("motion-animation", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let reveal = package
            .files
            .iter()
            .find(|file| file.path == "motion/reveal.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion reveal source");
        let controls = package
            .files
            .iter()
            .find(|file| file.path == "motion/controls.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion controls source");
        let frame = package
            .files
            .iter()
            .find(|file| file.path == "motion/frame.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion frame source");
        let will_change = package
            .files
            .iter()
            .find(|file| file.path == "motion/will-change.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion will-change source");
        let page_visibility = package
            .files
            .iter()
            .find(|file| file.path == "motion/page-visibility.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion page visibility source");
        let provider = package
            .files
            .iter()
            .find(|file| file.path == "motion/provider.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion provider source");
        let layout = package
            .files
            .iter()
            .find(|file| file.path == "motion/layout.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion layout source");
        let lazy = package
            .files
            .iter()
            .find(|file| file.path == "motion/lazy.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion lazy source");
        let presence = package
            .files
            .iter()
            .find(|file| file.path == "motion/presence.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion presence source");
        let reorder = package
            .files
            .iter()
            .find(|file| file.path == "motion/reorder.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion reorder source");
        let scoped_animate = package
            .files
            .iter()
            .find(|file| file.path == "motion/scoped-animate.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion scoped animate source");
        let motion_values = package
            .files
            .iter()
            .find(|file| file.path == "motion/motion-values.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion values source");
        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "motion/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("motion metadata");

        assert_eq!(package.package_id, "animation/motion");
        assert_eq!(package.upstream_name, "motion");
        assert_eq!(package.version, "12.38.0-dx.12");
        assert!(paths.contains(&"motion/presets.ts"));
        assert!(paths.contains(&"motion/controls.tsx"));
        assert!(paths.contains(&"motion/frame.tsx"));
        assert!(paths.contains(&"motion/layout.tsx"));
        assert!(paths.contains(&"motion/lazy.tsx"));
        assert!(paths.contains(&"motion/motion-values.tsx"));
        assert!(paths.contains(&"motion/page-visibility.tsx"));
        assert!(paths.contains(&"motion/provider.tsx"));
        assert!(paths.contains(&"motion/presence.tsx"));
        assert!(paths.contains(&"motion/reorder.tsx"));
        assert!(paths.contains(&"motion/reveal.tsx"));
        assert!(paths.contains(&"motion/scoped-animate.tsx"));
        assert!(paths.contains(&"motion/scroll-progress.tsx"));
        assert!(paths.contains(&"motion/will-change.tsx"));
        assert!(paths.contains(&"motion/dashboard-workflow.ts"));
        assert!(paths.contains(&"motion/metadata.ts"));
        assert!(paths.contains(&"motion/README.md"));
        assert_eq!(paths.len(), 17);
        assert!(controls.contains("useAnimationControls"));
        assert!(controls.contains("useAnimation"));
        assert!(controls.contains("animationControls"));
        assert!(controls.contains("LegacyAnimationControls"));
        assert!(controls.contains("AnimationDefinition"));
        assert!(controls.contains("MotionControlledStatus"));
        assert!(controls.contains("createDxAnimationControls"));
        assert!(controls.contains(r#"data-dx-motion="controlled-status""#));
        assert!(frame.contains("useAnimationFrame"));
        assert!(frame.contains("useTime"));
        assert!(frame.contains("useCycle"));
        assert!(frame.contains("CycleState"));
        assert!(frame.contains("useDxFrameClock"));
        assert!(frame.contains("MotionFrameTicker"));
        assert!(frame.contains(r#"data-dx-motion="frame-ticker""#));
        assert!(will_change.contains("useWillChange"));
        assert!(will_change.contains("WillChangeMotionValue"));
        assert!(will_change.contains("dxWillChangeProperties"));
        assert!(will_change.contains("createDxWillChangeValue"));
        assert!(will_change.contains("useDxWillChange"));
        assert!(will_change.contains("MotionWillChangeBox"));
        assert!(will_change.contains(r#"data-dx-motion="will-change-box""#));
        assert!(page_visibility.contains("usePageInView"));
        assert!(page_visibility.contains("useDxPageVisibility"));
        assert!(page_visibility.contains("MotionPageVisibilityBadge"));
        assert!(page_visibility.contains(r#"data-dx-motion="page-visibility-badge""#));
        assert!(layout.contains("LayoutGroup"));
        assert!(layout.contains("useInstantLayoutTransition"));
        assert!(layout.contains("DxMotionLayoutGroup"));
        assert!(layout.contains("MotionLayoutItem"));
        assert!(layout.contains("useDxInstantLayoutTransition"));
        assert!(layout.contains("dxMotionLayoutId"));
        assert!(layout.contains("layoutDependency"));
        assert!(layout.contains("layoutRoot"));
        assert!(lazy.contains("LazyMotion"));
        assert!(lazy.contains("domAnimation"));
        assert!(lazy.contains("domMax"));
        assert!(lazy.contains("domMin"));
        assert!(lazy.contains("DxLazyMotionProvider"));
        assert!(lazy.contains("MotionLazyBox"));
        assert!(lazy.contains("dxLazyMotionFeatures"));
        assert!(lazy.contains("dxLazyMotion = m"));
        assert!(lazy.contains(r#"data-dx-motion="lazy-motion-box""#));
        assert!(motion_values.contains("useMotionValue"));
        assert!(motion_values.contains("useTransform"));
        assert!(motion_values.contains("useMotionTemplate"));
        assert!(motion_values.contains("useMotionValueEvent"));
        assert!(motion_values.contains("useVelocity"));
        assert!(motion_values.contains("useDxMotionValueMeter"));
        assert!(motion_values.contains("MotionValueMeter"));
        assert!(motion_values.contains(r#"data-dx-motion="motion-value-meter""#));
        assert!(motion_values.contains(r#"data-dx-motion="motion-value-meter-indicator""#));
        assert!(provider.contains("MotionConfig"));
        assert!(provider.contains("MotionConfigProps"));
        assert!(provider.contains("dxMotionConfigDefaults"));
        assert!(provider.contains("DxMotionProvider"));
        assert!(provider.contains(r#"reducedMotion: "user""#));
        assert!(presence.contains("AnimatePresence"));
        assert!(presence.contains("AnimatePresenceProps"));
        assert!(presence.contains("LayoutGroup"));
        assert!(presence.contains("usePresence"));
        assert!(presence.contains("useIsPresent"));
        assert!(presence.contains("DxMotionPresence"));
        assert!(presence.contains("MotionPresenceItem"));
        assert!(presence.contains(r#"mode: "popLayout""#));
        assert!(reorder.contains("Reorder"));
        assert!(reorder.contains("useDragControls"));
        assert!(reorder.contains("DragControls"));
        assert!(reorder.contains("DxReorderGroup"));
        assert!(reorder.contains("DxReorderItem"));
        assert!(reorder.contains("useDxReorderControls"));
        assert!(reorder.contains("dragListener: false"));
        assert!(reveal.contains(r#"from "motion/react""#));
        assert!(reveal.contains("useReducedMotion"));
        assert!(reveal.contains("MotionReveal"));
        assert!(scoped_animate.contains("useAnimate"));
        assert!(scoped_animate.contains("useDxMotionPressFeedback"));
        let scroll_progress = package
            .files
            .iter()
            .find(|file| file.path == "motion/scroll-progress.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("motion scroll progress source");
        assert!(scroll_progress.contains("useScroll"));
        assert!(scroll_progress.contains("useSpring"));
        assert!(scroll_progress.contains("MotionScrollProgress"));
        assert!(metadata.contains(r#"packageId: "animation/motion""#));
        assert!(metadata.contains(r#"officialPackageName: "Motion & Animation""#));
        assert!(metadata.contains(r#"importPath: "motion/react""#));
        assert!(metadata.contains("dxCheckVisibility"));
    }

    #[test]
    fn wasm_bindgen_materializes_real_launch_loader_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("webassembly-bridge"), "wasm/bindgen");
        assert_eq!(canonical_package_id("webassembly/bridge"), "wasm/bindgen");
        assert_eq!(canonical_package_id("wasm-bindgen"), "wasm/bindgen");
        assert_eq!(canonical_package_id("rust/wasm-bindgen"), "wasm/bindgen");

        let package =
            source_package_for_project("wasm-bindgen", dir.path()).expect("wasm-bindgen package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let loader = package
            .files
            .iter()
            .find(|file| file.path == "wasm/bindgen/loader.ts")
            .and_then(|file| file.content.as_deref())
            .expect("loader source");
        let react = package
            .files
            .iter()
            .find(|file| file.path == "wasm/bindgen/react.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("react source");
        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "wasm/bindgen/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata source");

        assert_eq!(package.package_id, "wasm/bindgen");
        assert_eq!(package.upstream_name, "wasm-bindgen");
        assert_eq!(package.version, "0.2.121-dx.0");
        assert!(paths.contains(&"wasm/bindgen/loader.ts"));
        assert!(paths.contains(&"wasm/bindgen/react.tsx"));
        assert!(paths.contains(&"wasm/bindgen/example.tsx"));
        assert!(paths.contains(&"wasm/bindgen/dashboard-workflow.tsx"));
        assert!(paths.contains(&"wasm/bindgen/metadata.ts"));
        assert!(paths.contains(&"wasm/bindgen/README.md"));
        assert_eq!(paths.len(), 6);
        assert!(loader.contains("loadWasmBindgenModule"));
        assert!(loader.contains("init(input)"));
        assert!(react.contains("useWasmBindgenModule"));
        assert!(metadata.contains("wasm-bindgen 0.2.121"));
        assert!(metadata.contains("bundler"));
        assert!(metadata.contains("web"));
        assert!(metadata.contains("nodejs"));
        assert!(metadata.contains("no-modules"));
    }

    #[test]
    fn three_scene_alias_materializes_web_preview_scene_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("three-scene"), "3d/launch-scene");
        assert_eq!(
            canonical_package_id("@react-three/fiber"),
            "3d/launch-scene"
        );
        assert_eq!(canonical_package_id("three"), "three");

        let package = source_package_for_project("three-scene", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let component = package
            .files
            .iter()
            .find(|file| file.path == "components/scene/launch-scene.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("scene component");
        let preset = package
            .files
            .iter()
            .find(|file| file.path == "lib/scene/preset.ts")
            .and_then(|file| file.content.as_deref())
            .expect("scene preset");
        let index = package
            .files
            .iter()
            .find(|file| file.path == "lib/scene/index.ts")
            .and_then(|file| file.content.as_deref())
            .expect("scene package index");
        let source_alias = package
            .files
            .iter()
            .find(|file| file.path == "three/index.ts")
            .and_then(|file| file.content.as_deref())
            .expect("three source alias");
        let runtime = package
            .files
            .iter()
            .find(|file| file.path == "lib/scene/webgl-runtime.ts")
            .and_then(|file| file.content.as_deref())
            .expect("webgl runtime");
        let readiness = package
            .files
            .iter()
            .find(|file| file.path == "lib/scene/preview-readiness.ts")
            .and_then(|file| file.content.as_deref())
            .expect("scene preview readiness");
        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "lib/scene/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("scene metadata");

        assert_eq!(package.package_id, "3d/launch-scene");
        assert_eq!(package.upstream_name, "three");
        assert_eq!(package.version, THREE_SCENE_VERSION);
        assert!(paths.contains(&"components/scene/launch-scene.tsx"));
        assert!(paths.contains(&"lib/scene/index.ts"));
        assert!(paths.contains(&"three/index.ts"));
        assert!(paths.contains(&"lib/scene/types.ts"));
        assert!(paths.contains(&"lib/scene/preset.ts"));
        assert!(paths.contains(&"lib/scene/interaction.ts"));
        assert!(paths.contains(&"lib/scene/preview-readiness.ts"));
        assert!(paths.contains(&"lib/scene/performance-monitor.ts"));
        assert!(paths.contains(&"lib/scene/renderer-handoff.ts"));
        assert!(paths.contains(&"lib/scene/r3f-renderer-adapter.ts"));
        assert!(paths.contains(&"lib/scene/webgl-runtime.ts"));
        assert!(paths.contains(&"lib/scene/metadata.ts"));
        assert!(paths.contains(&"lib/scene/README.md"));
        assert_eq!(paths.len(), three_scene_templates().len());
        assert!(component.contains("createDxLaunchScenePreset"));
        assert!(component.contains("createDxScenePreviewReadiness"));
        assert!(component.contains("mountDxSceneWithRenderer"));
        assert!(component.contains(r#"data-dx-style-surface="launch-scene""#));
        assert!(component.contains(r#"<dx-icon name="pack:three-scene" aria-hidden="true" />"#));
        assert!(index.contains("createDxLaunchScenePreset"));
        assert!(index.contains("createDxSceneRenderBudget"));
        assert!(index.contains("mountDxWebGLScene"));
        assert!(index.contains("createDxSceneRendererHandoff"));
        assert!(index.contains("createDxSceneR3FDreiRendererAdapter"));
        assert!(index.contains("DxLaunchScenePreset"));
        assert!(source_alias.contains(r#"export * from "../lib/scene/index";"#));
        assert!(!source_alias.contains(r#"from "three""#));
        assert!(!source_alias.contains("node_modules"));
        assert!(preset.contains("splineLikeLayers"));
        assert!(readiness.contains("createDxScenePreviewReadiness"));
        assert!(readiness.contains(r#"requiresPackageInstall: false"#));
        assert!(runtime.contains("createDxSceneRenderBudget"));
        assert!(runtime.contains("maxDevicePixelRatio"));
        assert!(runtime.contains("prefers-reduced-motion"));
        assert!(metadata.contains(r#"packageId: "3d/launch-scene""#));
        assert!(metadata.contains("aliases"));
        assert!(metadata.contains("sourceMirrors"));
        assert!(metadata.contains("exportedFiles"));
        assert!(metadata.contains("receiptPaths"));
        assert!(metadata.contains(r#"name: "@react-three/fiber""#));
        assert!(metadata.contains(r#"name: "@react-three/drei""#));
    }

    #[test]
    fn tanstack_query_alias_materializes_launch_query_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("tanstack-query"), "tanstack/query");
        assert_eq!(
            canonical_package_id("data-fetching-cache"),
            "tanstack/query"
        );
        assert_eq!(
            canonical_package_id("@tanstack/react-query"),
            "tanstack/query"
        );

        let package =
            source_package_for_project("data-fetching-cache", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "tanstack/query");
        assert_eq!(package.upstream_name, "@tanstack/react-query");
        assert_eq!(package.version, "5.100.10-dx.0");
        assert!(paths.contains(&"lib/query/client.ts"));
        assert!(paths.contains(&"lib/query/provider.tsx"));
        assert!(paths.contains(&"lib/query/prefetch.tsx"));
        assert!(paths.contains(&"lib/query/metadata.ts"));
        assert!(paths.contains(&"lib/query/README.md"));

        let client = package
            .files
            .iter()
            .find(|file| file.path == "lib/query/client.ts")
            .and_then(|file| file.content.as_deref())
            .expect("client source");
        assert!(client.contains(r#"from "@tanstack/react-query";"#));
        assert!(client.contains("QueryClient"));
        assert!(client.contains("createDxQueryClient"));
        assert!(client.contains("DX_QUERY_DEFAULT_STALE_TIME_MS = 60_000"));
        assert!(client.contains("staleTime: config.staleTimeMs ?? DX_QUERY_DEFAULT_STALE_TIME_MS"));

        let provider = package
            .files
            .iter()
            .find(|file| file.path == "lib/query/provider.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("provider source");
        assert!(provider.contains(r#"from "@tanstack/react-query";"#));
        assert!(provider.contains("QueryClientProvider"));
        assert!(provider.contains("DxQueryProvider"));

        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "lib/query/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata source");
        assert!(metadata.contains(r#"packageId: "tanstack/query""#));
        assert!(metadata.contains(r#"upstreamPackage: "@tanstack/react-query""#));
        assert!(metadata.contains("QueryClientProvider"));
        assert!(metadata.contains("HydrationBoundary"));
    }

    #[test]
    fn next_intl_alias_materializes_app_router_i18n_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("next-intl"), "i18n/next-intl");
        assert_eq!(canonical_package_id("next-intl/routing"), "i18n/next-intl");

        let package = source_package_for_project("next-intl", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "i18n/next-intl");
        assert_eq!(package.upstream_name, "next-intl");
        assert_eq!(package.version, "4.12.0-dx.0");
        assert!(paths.contains(&"i18n/routing.ts"));
        assert!(paths.contains(&"i18n/navigation.ts"));
        assert!(paths.contains(&"i18n/request.ts"));
        assert!(paths.contains(&"i18n/middleware.ts"));
        assert!(paths.contains(&"i18n/provider.tsx"));
        assert!(paths.contains(&"i18n/metadata.ts"));
        assert!(paths.contains(&"i18n/messages/en.json"));
        assert!(paths.contains(&"i18n/messages/bn.json"));

        let routing = package
            .files
            .iter()
            .find(|file| file.path == "i18n/routing.ts")
            .and_then(|file| file.content.as_deref())
            .expect("routing source");
        assert!(routing.contains(r#"import { defineRouting } from "next-intl/routing";"#));
        assert!(routing.contains("defineDxRouting"));

        let request = package
            .files
            .iter()
            .find(|file| file.path == "i18n/request.ts")
            .and_then(|file| file.content.as_deref())
            .expect("request source");
        assert!(request.contains(r#"import { getRequestConfig } from "next-intl/server";"#));
        assert!(request.contains("createDxRequestConfig"));

        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "i18n/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata source");
        assert!(metadata.contains(r#"packageId: "i18n/next-intl""#));
        assert!(metadata.contains("NextIntlClientProvider"));
        assert!(metadata.contains("createMiddleware"));

        let messages = package
            .files
            .iter()
            .find(|file| file.path == "i18n/messages/en.json")
            .and_then(|file| file.content.as_deref())
            .expect("english message source");
        assert!(messages.contains(r#""phaseStatus""#));
        assert!(messages.contains("{phase}"));
    }

    #[test]
    fn fumadocs_alias_materializes_launch_docs_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("fumadocs"), "content/fumadocs-next");
        assert_eq!(
            canonical_package_id("docs/fumadocs"),
            "content/fumadocs-next"
        );

        let package = source_package_for_project("fumadocs", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "content/fumadocs-next");
        assert_eq!(package.upstream_name, "fumadocs");
        assert_eq!(package.version, "16.8.12-dx.11");
        assert!(!paths.contains(&"next.config.mjs"));
        assert!(!paths.contains(&"source.config.ts"));
        assert!(paths.contains(&"lib/fumadocs/source.ts"));
        assert!(paths.contains(&"lib/fumadocs/layout.tsx"));
        assert!(paths.contains(&"lib/fumadocs/llms.ts"));
        assert!(paths.contains(&"lib/fumadocs/openapi.ts"));
        assert!(paths.contains(&"lib/fumadocs/openapi-code-usage.ts"));
        assert!(paths.contains(&"lib/fumadocs/source-plugins.tsx"));
        assert!(paths.contains(&"lib/fumadocs/navigation.ts"));
        assert!(paths.contains(&"lib/fumadocs/toc.ts"));
        assert!(paths.contains(&"lib/fumadocs/search.ts"));
        assert!(paths.contains(&"lib/fumadocs/search-client.ts"));
        assert!(paths.contains(&"lib/fumadocs/readiness.ts"));
        assert!(paths.contains(&"lib/fumadocs/metadata.ts"));
        assert!(paths.contains(&"lib/fumadocs/route-contract.ts"));
        assert!(paths.contains(&"components/mdx.tsx"));
        assert!(paths.contains(&"components/api-page.tsx"));
        assert!(paths.contains(&"components/api-page.client.tsx"));
        assert!(paths.contains(&"app/docs/layout.tsx"));
        assert!(paths.contains(&"app/docs/[[...slug]]/page.tsx"));
        assert!(paths.contains(&"app/docs/readiness/route.ts"));
        assert!(paths.contains(&"app/llms.txt/route.ts"));
        assert!(paths.contains(&"app/llms-full.txt/route.ts"));
        assert!(paths.contains(&"app/llms.mdx/docs/[[...slug]]/route.ts"));
        assert!(paths.contains(&"app/api/search/route.ts"));
        assert!(paths.contains(&"app/api/search-static/route.ts"));
        assert!(paths.contains(&"app/api/openapi/proxy/route.ts"));
        assert!(paths.contains(&"content/docs/meta.json"));
        assert!(paths.contains(&"content/docs/index.mdx"));
        assert!(paths.contains(&"openapi/dx-launch.yaml"));

        let page = package
            .files
            .iter()
            .find(|file| file.path == "app/docs/[[...slug]]/page.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("page source");
        assert!(page.contains("DocsPage"));
        assert!(page.contains(r#"import { APIPage } from "@/components/api-page";"#));
        assert!(page.contains(r#"page.type === "openapi""#));
        assert!(page.contains("page.data.getAPIPageProps()"));
        assert!(page.contains("getDxFumadocsPageToc"));
        assert!(page.contains("toc={getDxFumadocsPageToc(page)}"));
        assert!(page.contains("createRelativeLink"));
        assert!(page.contains("generateStaticParams"));

        let toc = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/toc.ts")
            .and_then(|file| file.content.as_deref())
            .expect("toc source");
        assert!(toc.contains(r#"import { getTableOfContents } from "fumadocs-core/content/toc";"#));
        assert!(toc.contains(r#"import type { TOCItemType } from "fumadocs-core/toc";"#));
        assert!(toc.contains("dxFumadocsTocContract"));
        assert!(toc.contains("getDxFumadocsMarkdownToc"));
        assert!(toc.contains("getDxFumadocsPageToc"));
        assert!(toc.contains("getDxFumadocsPageTocSummary"));

        let llms = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/llms.ts")
            .and_then(|file| file.content.as_deref())
            .expect("llms source");
        assert!(llms.contains(r#"import { llms } from "fumadocs-core/source";"#));
        assert!(llms.contains("createDxFumadocsLLMsIndex"));
        assert!(llms.contains(r#"page.data.getText("processed")"#));
        assert!(llms.contains(r#"page.type === "openapi""#));
        assert!(llms.contains("page.data.getSchema().bundled"));

        let openapi = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/openapi.ts")
            .and_then(|file| file.content.as_deref())
            .expect("openapi source");
        assert!(openapi.contains(r#"import { createOpenAPI } from "fumadocs-openapi/server";"#));
        assert!(openapi.contains("dxFumadocsOpenAPIContract"));
        assert!(openapi.contains(r#"schemaFile: "openapi/dx-launch.yaml""#));
        assert!(openapi.contains(r#"proxyRoute: "/api/openapi/proxy""#));
        assert!(openapi.contains(r#"allowedOriginsEnv: "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS""#));
        assert!(openapi.contains("createAPIPage from fumadocs-openapi/ui"));
        assert!(openapi.contains("openapi.createProxy from fumadocs-openapi/server"));
        assert!(openapi.contains("dxFumadocsOpenAPI = createOpenAPI"));
        assert!(openapi.contains("proxyUrl: dxFumadocsOpenAPIContract.proxyRoute"));
        assert!(openapi.contains("readDxFumadocsOpenAPIAllowedOrigins"));
        assert!(openapi.contains("process.env"));

        let openapi_code_usage = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/openapi-code-usage.ts")
            .and_then(|file| file.content.as_deref())
            .expect("openapi code usage source");
        assert!(openapi_code_usage.contains("createCodeUsageGeneratorRegistry"));
        assert!(openapi_code_usage.contains("registerDefault(dxFumadocsOpenAPICodeUsages)"));
        assert!(
            openapi_code_usage.contains(r#"dxFumadocsOpenAPICodeUsages.add("dx-launch-fetch""#)
        );
        assert!(openapi_code_usage.contains("dxFumadocsOpenAPICodeUsageContract"));

        let api_page = package
            .files
            .iter()
            .find(|file| file.path == "components/api-page.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("api page source");
        assert!(api_page.contains(r#"import { createAPIPage } from "fumadocs-openapi/ui";"#));
        assert!(api_page.contains("client: dxFumadocsOpenAPIClientConfig"));
        assert!(api_page.contains("codeUsages: dxFumadocsOpenAPICodeUsages"));

        let api_page_client = package
            .files
            .iter()
            .find(|file| file.path == "components/api-page.client.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("api page client source");
        assert!(api_page_client.contains(r#""use client""#));
        assert!(api_page_client.contains("defineClientConfig"));
        assert!(api_page_client.contains("codeUsages: dxFumadocsOpenAPICodeUsages"));

        let source_plugins = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/source-plugins.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("source plugins source");
        assert!(!source_plugins.contains("lucideIconsPlugin"));
        assert!(source_plugins.contains("iconPlugin"));
        assert!(source_plugins.contains("dxFumadocsIconPlugin"));
        assert!(source_plugins.contains("data-dx-docs-icon-source"));
        assert!(source_plugins.contains("statusBadgesPlugin"));
        assert!(source_plugins.contains("slugsFromData"));
        assert!(source_plugins.contains("dxFumadocsSourcePluginContract"));
        assert!(source_plugins.contains("dxFumadocsSourcePlugins"));
        assert!(source_plugins.contains("dxFumadocsSlugFn"));

        let navigation = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/navigation.ts")
            .and_then(|file| file.content.as_deref())
            .expect("navigation source");
        assert!(
            navigation
                .contains(r#"import { getBreadcrumbItems } from "fumadocs-core/breadcrumb";"#)
        );
        assert!(
            navigation
                .contains(r#"import { findNeighbour, flattenTree, getPageTreePeers } from "fumadocs-core/page-tree";"#)
        );
        assert!(navigation.contains("dxFumadocsNavigationContract"));
        assert!(navigation.contains("getDxFumadocsPageBreadcrumbs"));
        assert!(navigation.contains("getDxFumadocsNavigationSnapshot"));
        assert!(navigation.contains("source.getPageTree(locale)"));
        assert!(navigation.contains("flattenTree(tree.children)"));
        assert!(navigation.contains("findNeighbour(tree, url)"));
        assert!(navigation.contains("getPageTreePeers(tree, url)"));

        let source = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/source.ts")
            .and_then(|file| file.content.as_deref())
            .expect("source loader");
        assert!(source.contains("dxFumadocsOpenAPI.staticSource"));
        assert!(source.contains(r#"baseDir: dxFumadocsOpenAPIContract.docsBaseDir"#));
        assert!(source.contains("dxFumadocsSourcePlugins"));
        assert!(source.contains(
            "plugins: [...dxFumadocsSourcePlugins, ...dxFumadocsOpenAPILoaderPluginConfig.plugins]"
        ));
        assert!(source.contains("slugs: dxFumadocsSlugFn"));

        let openapi_schema = package
            .files
            .iter()
            .find(|file| file.path == "openapi/dx-launch.yaml")
            .and_then(|file| file.content.as_deref())
            .expect("openapi schema");
        assert!(openapi_schema.contains("openapi: 3.1.0"));
        assert!(openapi_schema.contains("/api/search"));
        assert!(openapi_schema.contains("/api/openapi/proxy"));

        let llms_index_route = package
            .files
            .iter()
            .find(|file| file.path == "app/llms.txt/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("llms index route");
        assert!(llms_index_route.contains("createDxFumadocsLLMsIndex().index()"));

        let llms_full_route = package
            .files
            .iter()
            .find(|file| file.path == "app/llms-full.txt/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("llms full route");
        assert!(llms_full_route.contains("source.getPages().map(getDxFumadocsLLMText)"));

        let llms_mdx_route = package
            .files
            .iter()
            .find(|file| file.path == "app/llms.mdx/docs/[[...slug]]/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("llms mdx route");
        assert!(llms_mdx_route.contains("getDxFumadocsPageMarkdownUrl"));
        assert!(llms_mdx_route.contains("RouteContext"));

        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata source");
        assert!(metadata.contains(r#"packageId: "content/fumadocs-next""#));
        assert!(metadata.contains(r#"name: "fumadocs-core""#));
        assert!(metadata.contains(r#"name: "fumadocs-openapi""#));
        assert!(metadata.contains("llms from fumadocs-core/source"));
        assert!(metadata.contains("page.data.getText"));
        assert!(metadata.contains("createOpenAPI from fumadocs-openapi/server"));
        assert!(metadata.contains("openapi.createProxy from fumadocs-openapi/server"));
        assert!(metadata.contains("createAPIPage from fumadocs-openapi/ui"));
        assert!(metadata.contains("createCodeUsageGeneratorRegistry"));
        assert!(metadata.contains("registerDefault from fumadocs-openapi/requests/generators/all"));
        assert!(metadata.contains("page.data.getAPIPageProps()"));
        assert!(metadata.contains("createFromSource from fumadocs-core/search/server"));
        assert!(metadata.contains("useDocsSearch from fumadocs-core/search/client"));
        assert!(!metadata.contains("lucideIconsPlugin from fumadocs-core/source/lucide-icons"));
        assert!(metadata.contains("DX Icon component from @/components/icons/icon"));
        assert!(metadata.contains("typed DxConfig extracts framework.fumadocs.* values"));
        assert!(metadata.contains("statusBadgesPlugin from fumadocs-core/source/status-badges"));
        assert!(metadata.contains("slugsFromData from fumadocs-core/source/slugs"));
        assert!(metadata.contains("getBreadcrumbItems from fumadocs-core/breadcrumb"));
        assert!(metadata.contains("findNeighbour"));
        assert!(metadata.contains("getTableOfContents from fumadocs-core/content/toc"));
        assert!(metadata.contains("TOCItemType from fumadocs-core/toc"));
        assert!(metadata.contains("dx add content/fumadocs-next --write"));
        assert!(metadata.contains("dxFumadocsRouteContract"));
        assert!(metadata.contains(r#"routeContractFile: "lib/fumadocs/route-contract.ts""#));
        assert!(metadata.contains(r#"sourcePluginFile: "lib/fumadocs/source-plugins.tsx""#));
        assert!(metadata.contains(r#"sourcePluginFrontmatterFields: ["icon", "status", "slug"]"#));
        assert!(metadata.contains(r#"navigationFile: "lib/fumadocs/navigation.ts""#));
        assert!(metadata.contains(
            r#"navigationSurfaces: ["breadcrumb", "previous-next", "sidebar-count", "peers"]"#
        ));
        assert!(metadata.contains(r#"tocFile: "lib/fumadocs/toc.ts""#));
        assert!(
            metadata.contains(
                r#"tocSurfaces: ["docs-page-outline", "markdown-outline", "toc-summary"]"#
            )
        );
        assert!(metadata.contains(r#"llmsIndexRoute: "/llms.txt""#));
        assert!(metadata.contains(r#"llmsConfigFile: "lib/fumadocs/llms.ts""#));
        assert!(metadata.contains(r#"readinessRoute: "/docs/readiness""#));
        assert!(metadata.contains(r#"readinessRouteFile: "app/docs/readiness/route.ts""#));
        assert!(metadata.contains(r#"readinessSourceFile: "lib/fumadocs/readiness.ts""#));
        assert!(metadata.contains(r#"openApiDocsRoute: "/docs/api""#));
        assert!(metadata.contains(r#"openApiConfigFile: "lib/fumadocs/openapi.ts""#));
        assert!(metadata.contains(r#"openApiCodeUsageFile: "lib/fumadocs/openapi-code-usage.ts""#));
        assert!(metadata.contains(r#"openApiProxyRoute: "/api/openapi/proxy""#));
        assert!(metadata.contains(r#"openApiProxyRouteFile: "app/api/openapi/proxy/route.ts""#));
        assert!(
            metadata.contains(
                r#"openApiProxyAllowedOriginsEnv: "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS""#
            )
        );
        assert!(metadata.contains(r#"openApiSchemaFile: "openapi/dx-launch.yaml""#));
        assert!(metadata.contains(r#"openApiClientConfigFile: "components/api-page.client.tsx""#));
        assert!(metadata.contains(r#"searchRoute: "/api/search""#));
        assert!(metadata.contains(r#"staticSearchRoute: "/api/search-static""#));
        assert!(metadata.contains(r#"searchClientFile: "lib/fumadocs/search-client.ts""#));

        let search = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/search.ts")
            .and_then(|file| file.content.as_deref())
            .expect("search source");
        assert!(
            search.contains(r#"import { createFromSource } from "fumadocs-core/search/server";"#)
        );
        assert!(search.contains("createDxFumadocsSearchApi"));
        assert!(search.contains("staticGET from Fumadocs SearchAPI"));
        assert!(search.contains(r#"queryParam: "query""#));

        let search_client = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/search-client.ts")
            .and_then(|file| file.content.as_deref())
            .expect("search client source");
        assert!(
            search_client
                .contains(r#"import type { ClientPreset } from "fumadocs-core/search/client";"#)
        );
        assert!(search_client.contains("useDocsSearch from fumadocs-core/search/client"));
        assert!(search_client.contains("dxFumadocsFetchSearchClient"));
        assert!(search_client.contains("dxFumadocsStaticSearchClient"));

        let search_route = package
            .files
            .iter()
            .find(|file| file.path == "app/api/search/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("search route source");
        assert!(search_route.contains("createDxFumadocsSearchApi"));
        assert!(search_route.contains("export const GET = searchApi.GET"));

        let static_search_route = package
            .files
            .iter()
            .find(|file| file.path == "app/api/search-static/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("static search route source");
        assert!(static_search_route.contains("export const revalidate = false"));
        assert!(static_search_route.contains("export const GET = searchApi.staticGET"));

        let openapi_proxy_route = package
            .files
            .iter()
            .find(|file| file.path == "app/api/openapi/proxy/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("openapi proxy route source");
        assert!(openapi_proxy_route.contains("readDxFumadocsOpenAPIAllowedOrigins"));
        assert!(openapi_proxy_route.contains("dxFumadocsOpenAPI.createProxy"));
        assert!(openapi_proxy_route.contains("allowedOrigins,"));
        assert!(openapi_proxy_route.contains("filterRequest(request)"));
        assert!(openapi_proxy_route.contains(r#"request.url.startsWith("https://")"#));
        assert!(openapi_proxy_route.contains("GET, HEAD, PUT, POST, PATCH, DELETE"));

        let route_contract = package
            .files
            .iter()
            .find(|file| file.path == "lib/fumadocs/route-contract.ts")
            .and_then(|file| file.content.as_deref())
            .expect("route contract source");
        assert!(route_contract.contains(r#"docsRoute: "/docs""#));
        assert!(route_contract.contains(r#"readinessRoute: "/docs/readiness""#));
        assert!(route_contract.contains(r#"llmsIndexRoute: "/llms.txt""#));
        assert!(route_contract.contains(r#"llmsFullRoute: "/llms-full.txt""#));
        assert!(route_contract.contains(r#"llmsPageMarkdownRoute: "/llms.mdx/docs/[[...slug]]""#));
        assert!(route_contract.contains(r#"openApiDocsRoute: "/docs/api""#));
        assert!(route_contract.contains(r#"openApiProxyRoute: "/api/openapi/proxy""#));
        assert!(
            route_contract.contains(
                r#"openApiProxyAllowedOriginsEnv: "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS""#
            )
        );
        assert!(route_contract.contains(r#"searchRoute: "/api/search""#));
        assert!(route_contract.contains(r#"staticSearchRoute: "/api/search-static""#));
        assert!(route_contract.contains(r#"contentDir: "content/docs""#));
        assert!(route_contract.contains(r#"sourcePluginFile: "lib/fumadocs/source-plugins.tsx""#));
        assert!(
            route_contract.contains(r#"sourcePluginFrontmatterFields: ["icon", "status", "slug"]"#)
        );
        assert!(route_contract.contains(r#"navigationFile: "lib/fumadocs/navigation.ts""#));
        assert!(route_contract.contains(
            r#"navigationSurfaces: ["breadcrumb", "previous-next", "sidebar-count", "peers"]"#
        ));
        assert!(route_contract.contains(r#"tocFile: "lib/fumadocs/toc.ts""#));
        assert!(
            route_contract.contains(
                r#"tocSurfaces: ["docs-page-outline", "markdown-outline", "toc-summary"]"#
            )
        );
        assert!(route_contract.contains(r#"openApiSchemaFile: "openapi/dx-launch.yaml""#));
        assert!(route_contract.contains(r#"wwwConfigOwner: "framework.www.config_owner_file""#));
        assert!(route_contract.contains(r#"docsRoute: "framework.fumadocs.docs_route""#));
        assert!(
            route_contract.contains(r#"generatedRoutes: "framework.fumadocs.generated_routes""#)
        );
        assert!(route_contract.contains(r#"llmsConfigFile: "lib/fumadocs/llms.ts""#));
        assert!(route_contract.contains(r#"openApiConfigFile: "lib/fumadocs/openapi.ts""#));
        assert!(
            route_contract
                .contains(r#"openApiCodeUsageFile: "lib/fumadocs/openapi-code-usage.ts""#)
        );
        assert!(route_contract.contains(r#"openApiPageComponentFile: "components/api-page.tsx""#));
        assert!(
            route_contract.contains(r#"openApiClientConfigFile: "components/api-page.client.tsx""#)
        );
        assert!(
            route_contract.contains(r#"openApiProxyRouteFile: "app/api/openapi/proxy/route.ts""#)
        );
        assert!(route_contract.contains(r#"searchConfigFile: "lib/fumadocs/search.ts""#));
        assert!(route_contract.contains(r#"searchClientFile: "lib/fumadocs/search-client.ts""#));
        assert!(route_contract.contains(r#"searchRouteFile: "app/api/search/route.ts""#));
        assert!(
            route_contract.contains(r#"staticSearchRouteFile: "app/api/search-static/route.ts""#)
        );
        assert!(route_contract.contains(r#"docsPageFile: "app/docs/[[...slug]]/page.tsx""#));
        assert!(route_contract.contains(r#"readinessRouteFile: "app/docs/readiness/route.ts""#));
        assert!(route_contract.contains("sourceGuard"));
        assert!(route_contract.contains("pending-app-owned-next-runtime"));
    }

    #[test]
    fn zod_alias_materializes_launch_validation_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("zod"), "validation/zod");
        assert_eq!(canonical_package_id("zod/v4"), "validation/zod");

        let package = source_package_for_project("zod", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "validation/zod");
        assert_eq!(package.upstream_name, "zod");
        assert_eq!(package.version, "4.4.3-dx.12");
        assert!(paths.contains(&"lib/validation/zod/schemas.ts"));
        assert!(paths.contains(&"lib/validation/zod/objects.ts"));
        assert!(paths.contains(&"lib/validation/zod/parse.ts"));
        assert!(paths.contains(&"lib/validation/zod/errors.ts"));
        assert!(paths.contains(&"lib/validation/zod/json-schema.ts"));
        assert!(paths.contains(&"lib/validation/zod/json-schema-import.ts"));
        assert!(paths.contains(&"lib/validation/zod/codecs.ts"));
        assert!(paths.contains(&"lib/validation/zod/coerce.ts"));
        assert!(paths.contains(&"lib/validation/zod/env.ts"));
        assert!(paths.contains(&"lib/validation/zod/files.ts"));
        assert!(paths.contains(&"lib/validation/zod/transforms.ts"));
        assert!(paths.contains(&"lib/validation/zod/catalog.ts"));
        assert!(paths.contains(&"lib/validation/zod/refinements.ts"));
        assert!(paths.contains(&"lib/validation/zod/patterns.ts"));
        assert!(paths.contains(&"lib/validation/zod/registry.ts"));
        assert!(paths.contains(&"lib/validation/zod/example.ts"));
        assert!(paths.contains(&"lib/validation/zod/metadata.ts"));
        assert!(paths.contains(&"lib/validation/zod/README.md"));

        let parse = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/parse.ts")
            .and_then(|file| file.content.as_deref())
            .expect("parse source");
        assert!(parse.contains(r#"import { z } from "zod";"#));
        assert!(parse.contains("schema.safeParse"));
        assert!(parse.contains("z.treeifyError"));
        assert!(parse.contains("z.flattenError"));

        let objects = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/objects.ts")
            .and_then(|file| file.content.as_deref())
            .expect("objects source");
        assert!(objects.contains(".partial("));
        assert!(objects.contains(".required("));
        assert!(objects.contains(".safeExtend("));
        assert!(objects.contains(".pick("));
        assert!(objects.contains(".omit("));
        assert!(objects.contains("safeParseDxLaunchSignupSubmission"));

        let errors = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/errors.ts")
            .and_then(|file| file.content.as_deref())
            .expect("errors source");
        assert!(errors.contains("z.config"));
        assert!(errors.contains("z.locales.en"));
        assert!(errors.contains("z.ZodErrorMap"));
        assert!(errors.contains("{ error: dxLaunchErrorMap }"));
        assert!(errors.contains("safeParseDxLaunchSignupForDisplay"));

        let json_schema = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/json-schema.ts")
            .and_then(|file| file.content.as_deref())
            .expect("json schema source");
        assert!(json_schema.contains("z.toJSONSchema"));
        assert!(json_schema.contains("draft-2020-12"));

        let json_schema_import = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/json-schema-import.ts")
            .and_then(|file| file.content.as_deref())
            .expect("json schema import source");
        assert!(json_schema_import.contains("z.fromJSONSchema"));
        assert!(json_schema_import.contains(r#"defaultTarget: "draft-2020-12""#));
        assert!(json_schema_import.contains("safeParseDxLaunchExternalPackage"));

        let codecs = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/codecs.ts")
            .and_then(|file| file.content.as_deref())
            .expect("codecs source");
        assert!(codecs.contains("z.codec"));
        assert!(codecs.contains("z.decode"));
        assert!(codecs.contains("z.encode"));
        assert!(codecs.contains("z.safeDecode"));
        assert!(codecs.contains("z.safeEncode"));

        let coerce = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/coerce.ts")
            .and_then(|file| file.content.as_deref())
            .expect("coerce source");
        assert!(coerce.contains("z.coerce.string"));
        assert!(coerce.contains("z.coerce.number"));
        assert!(coerce.contains("z.coerce.date"));
        assert!(coerce.contains("z.input"));
        assert!(coerce.contains("parseDxLaunchSearchParams"));

        let env = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/env.ts")
            .and_then(|file| file.content.as_deref())
            .expect("env source");
        assert!(env.contains("z.stringbool"));
        assert!(env.contains("parseDxLaunchEnvFlags"));
        assert!(env.contains("DX_ENABLE_RUNTIME_PREVIEW"));

        let files = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/files.ts")
            .and_then(|file| file.content.as_deref())
            .expect("files source");
        assert!(files.contains("z.file()"));
        assert!(files.contains(".min(1"));
        assert!(files.contains(".max("));
        assert!(files.contains(".mime("));
        assert!(files.contains("safeParseDxLaunchAssetFile"));
        assert!(files.contains("dxLaunchAssetFileJsonSchema"));

        let transforms = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/transforms.ts")
            .and_then(|file| file.content.as_deref())
            .expect("transforms source");
        assert!(transforms.contains("z.preprocess"));
        assert!(transforms.contains(".transform("));
        assert!(transforms.contains(".pipe("));
        assert!(transforms.contains(".prefault("));
        assert!(transforms.contains(".catch("));
        assert!(transforms.contains("parseDxLaunchScoreInput"));

        let catalog = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/catalog.ts")
            .and_then(|file| file.content.as_deref())
            .expect("catalog source");
        assert!(catalog.contains("z.strictObject"));
        assert!(catalog.contains("z.record("));
        assert!(catalog.contains("z.partialRecord"));
        assert!(catalog.contains(".catchall("));
        assert!(catalog.contains(".readonly("));
        assert!(catalog.contains("parseDxLaunchPackageCatalog"));
        assert!(catalog.contains("summarizeDxLaunchPackageCatalog"));

        let refinements = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/refinements.ts")
            .and_then(|file| file.content.as_deref())
            .expect("refinements source");
        assert!(refinements.contains(".refine("));
        assert!(refinements.contains(".superRefine("));
        assert!(refinements.contains(".check("));
        assert!(refinements.contains("z.refine("));
        assert!(refinements.contains("ctx.addIssue"));
        assert!(refinements.contains("parseDxLaunchApprovalGate"));
        assert!(refinements.contains("safeParseDxLaunchApprovalGate"));

        let patterns = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/patterns.ts")
            .and_then(|file| file.content.as_deref())
            .expect("patterns source");
        assert!(patterns.contains("z.templateLiteral"));
        assert!(patterns.contains("dxLaunchRoutePathSchema"));
        assert!(patterns.contains("parseDxLaunchRoutePath"));
        assert!(patterns.contains(".dx/forge/"));

        let registry = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/registry.ts")
            .and_then(|file| file.content.as_deref())
            .expect("registry source");
        assert!(registry.contains("z.registry"));
        assert!(registry.contains("z.globalRegistry"));
        assert!(registry.contains(".register("));
        assert!(registry.contains(".meta("));

        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "lib/validation/zod/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata source");
        assert!(metadata.contains(r#"packageId: "validation/zod""#));
        assert!(metadata.contains(r#"upstreamPackage: "zod""#));
        assert!(metadata.contains("safeParse"));
        assert!(metadata.contains("schema.safeExtend"));
        assert!(metadata.contains("object.partial"));
        assert!(metadata.contains("toJSONSchema"));
        assert!(metadata.contains("z.codec"));
        assert!(metadata.contains("z.coerce.number"));
        assert!(metadata.contains("z.registry"));
        assert!(metadata.contains("z.stringbool"));
        assert!(metadata.contains("z.templateLiteral"));
        assert!(metadata.contains("z.file"));
        assert!(metadata.contains("z.preprocess"));
        assert!(metadata.contains("schema.transform"));
        assert!(metadata.contains("z.strictObject"));
        assert!(metadata.contains("z.record"));
        assert!(metadata.contains("z.partialRecord"));
        assert!(metadata.contains("schema.refine"));
        assert!(metadata.contains("schema.superRefine"));
        assert!(metadata.contains("z.config"));
        assert!(metadata.contains("z.locales"));
        assert!(metadata.contains("schema.safeParse(errorMap)"));
        assert!(metadata.contains("z.fromJSONSchema"));
    }

    #[test]
    fn react_hook_form_materializes_launch_form_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(
            canonical_package_id("react-hook-form"),
            "forms/react-hook-form"
        );
        assert_eq!(canonical_package_id("forms"), "forms/react-hook-form");
        assert_eq!(canonical_package_id("rhf"), "forms/react-hook-form");

        let package = source_package_for_project("react-hook-form", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "forms/react-hook-form");
        assert_eq!(package.upstream_name, "react-hook-form");
        assert_eq!(package.version, "7.75.0-dx.0");
        assert!(paths.contains(&"lib/forms/react-hook-form/form.tsx"));
        assert!(paths.contains(&"lib/forms/react-hook-form/fields.tsx"));
        assert!(paths.contains(&"lib/forms/react-hook-form/resolver.ts"));
        assert!(paths.contains(&"lib/forms/react-hook-form/metadata.ts"));
        assert!(paths.contains(&"lib/forms/react-hook-form/README.md"));

        let form = package
            .files
            .iter()
            .find(|file| file.path == "lib/forms/react-hook-form/form.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("form source");
        assert!(form.contains(
            r#"import { FormProvider, useForm, useFormContext } from "react-hook-form";"#
        ));
        assert!(form.contains("methods.handleSubmit(onSubmit, onInvalid)"));

        let fields = package
            .files
            .iter()
            .find(|file| file.path == "lib/forms/react-hook-form/fields.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("fields source");
        assert!(fields.contains("UseFormRegister"));
        assert!(fields.contains("getDxFieldErrorMessage"));

        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "lib/forms/react-hook-form/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata source");
        assert!(metadata.contains(r#"packageId: "forms/react-hook-form""#));
        assert!(metadata.contains("useFieldArray"));
    }

    #[test]
    fn react_markdown_alias_materializes_launch_content_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(
            canonical_package_id("react-markdown"),
            "content/react-markdown"
        );
        assert_eq!(
            canonical_package_id("markdown/react"),
            "content/react-markdown"
        );
        assert_eq!(
            canonical_package_id("markdown-mdx-content"),
            "content/react-markdown"
        );
        assert_eq!(
            canonical_package_id("mdx/content"),
            "content/react-markdown"
        );

        let package = source_package_for_project("react-markdown", dir.path()).expect("package");
        assert_eq!(package.package_id, "content/react-markdown");
        assert_eq!(package.upstream_name, "react-markdown");
        assert_eq!(package.version, "10.1.0-dx.2");

        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        assert!(paths.contains(&"components/content/markdown.tsx"));
        assert!(paths.contains(&"components/content/markdown-components.tsx"));
        assert!(paths.contains(&"components/content/markdown-metadata.ts"));
        assert!(paths.contains(&"components/content/mdx-provider.tsx"));
        assert!(paths.contains(&"components/content/README.md"));
        assert!(paths.contains(&"server/content/mdx.ts"));
        assert!(paths.contains(&"components/markdown.tsx"));
        assert!(paths.contains(&"components/markdown-client.tsx"));
        assert!(paths.contains(&"lib/react-markdown/metadata.ts"));
        assert!(paths.contains(&"lib/react-markdown/README.md"));
        assert!(paths.contains(&"lib/mdx/metadata.ts"));
        assert!(paths.contains(&"lib/mdx/README.md"));

        let renderer = package
            .files
            .iter()
            .find(|file| file.path == "components/content/markdown.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("react markdown renderer");
        assert!(renderer.contains(r#"import ReactMarkdown"#));
        assert!(renderer.contains("skipHtml"));
        assert!(renderer.contains("defaultUrlTransform"));
        assert!(renderer.contains("resolveDxMarkdownAllowedElements(options)"));
        assert!(renderer.contains("if (options.disallowedElements) return undefined;"));

        let compat_renderer = package
            .files
            .iter()
            .find(|file| file.path == "components/markdown.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("react markdown compat renderer");
        assert!(compat_renderer.contains("resolveDxMarkdownAllowedElements"));
        assert!(!compat_renderer.contains("allowedElements = dxMarkdownAllowedElements"));

        let client_renderer = package
            .files
            .iter()
            .find(|file| file.path == "components/markdown-client.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("react markdown client renderer");
        assert!(client_renderer.contains("MarkdownHooks"));
        assert!(client_renderer.contains("resolveDxMarkdownAllowedElements"));

        let mdx_provider = package
            .files
            .iter()
            .find(|file| file.path == "components/content/mdx-provider.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("mdx provider source");
        assert!(mdx_provider.contains("MDXProvider"));
        assert!(mdx_provider.contains("useMDXComponents"));
        assert!(mdx_provider.contains(r#"data-dx-component="dx-mdx-provider""#));
        assert!(mdx_provider.contains(r#"data-dx-style-surface="markdown-mdx-content""#));

        let mdx_compile = package
            .files
            .iter()
            .find(|file| file.path == "server/content/mdx.ts")
            .and_then(|file| file.content.as_deref())
            .expect("mdx compile source");
        assert!(mdx_compile.contains(r#"from "@mdx-js/mdx""#));
        assert!(mdx_compile.contains("compileDxMdxContent"));
        assert!(mdx_compile.contains("createDxMdxProcessor"));

        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "components/content/markdown-metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("react markdown metadata");
        assert!(metadata.contains(r#"officialDxPackageName: "Markdown & MDX Content""#));
        assert!(metadata.contains(r#"packageId: "content/react-markdown""#));
        assert!(metadata.contains(r#"upstreamPackage: "react-markdown""#));
        assert!(metadata.contains(r#""@mdx-js/mdx""#));
        assert!(metadata.contains(r#""@mdx-js/react""#));
        assert!(metadata.contains("MarkdownAsync"));
        assert!(metadata.contains("dxCheckVisibility"));
    }

    #[test]
    fn react_hook_form_respects_custom_forms_path_mapping() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx"),
            r#"forge.paths.js_forms = "src/forms""#,
        )
        .expect("dx config");

        let package =
            source_package_for_project("forms/react-hook-form", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert!(paths.contains(&"src/forms/react-hook-form/form.tsx"));
        assert!(paths.contains(&"src/forms/react-hook-form/fields.tsx"));
        assert!(paths.contains(&"src/forms/react-hook-form/resolver.ts"));
    }

    #[test]
    fn stripe_js_alias_materializes_launch_payment_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("payments"), "payments/stripe-js");
        assert_eq!(canonical_package_id("stripe-js"), "payments/stripe-js");
        assert_eq!(
            canonical_package_id("@stripe/stripe-js"),
            "payments/stripe-js"
        );

        let package = source_package_for_project("payments", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "payments/stripe-js");
        assert_eq!(package.upstream_name, "@stripe/stripe-js");
        assert_eq!(package.version, "9.6.0-dx.1");
        assert!(paths.contains(&"lib/payments/stripe-js/config.ts"));
        assert!(paths.contains(&"lib/payments/stripe-js/client.ts"));
        assert!(paths.contains(&"lib/payments/stripe-js/payment.ts"));
        assert!(paths.contains(&"lib/payments/stripe-js/checkout.ts"));
        assert!(paths.contains(&"lib/payments/stripe-js/server.ts"));
        assert!(paths.contains(&"lib/payments/stripe-js/metadata.ts"));
        assert!(paths.contains(&"lib/payments/stripe-js/README.md"));

        let config = package
            .files
            .iter()
            .find(|file| file.path == "lib/payments/stripe-js/config.ts")
            .and_then(|file| file.content.as_deref())
            .expect("stripe config source");
        assert!(config.contains("NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY"));
        assert!(config.contains("assertNoPublicStripeSecrets"));

        let client = package
            .files
            .iter()
            .find(|file| file.path == "lib/payments/stripe-js/client.ts")
            .and_then(|file| file.content.as_deref())
            .expect("stripe client source");
        assert!(client.contains(r#"from "@stripe/stripe-js/pure""#));
        assert!(client.contains("loadStripe"));

        let payment = package
            .files
            .iter()
            .find(|file| file.path == "lib/payments/stripe-js/payment.ts")
            .and_then(|file| file.content.as_deref())
            .expect("stripe payment source");
        assert!(payment.contains("confirmPayment"));
        assert!(payment.contains("return_url"));

        let checkout = package
            .files
            .iter()
            .find(|file| file.path == "lib/payments/stripe-js/checkout.ts")
            .and_then(|file| file.content.as_deref())
            .expect("stripe checkout source");
        assert!(checkout.contains("validateDxStripeCheckoutContact"));
        assert!(checkout.contains("submitDxStripeCheckoutContact"));

        let server = package
            .files
            .iter()
            .find(|file| file.path == "lib/payments/stripe-js/server.ts")
            .and_then(|file| file.content.as_deref())
            .expect("stripe server source");
        assert!(server.contains(r#"import "server-only";"#));
        assert!(server.contains("STRIPE_SECRET_KEY"));
        assert!(server.contains("createDxStripeCheckoutSession"));

        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "lib/payments/stripe-js/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata source");
        assert!(metadata.contains(r#"packageId: "payments/stripe-js""#));
        assert!(metadata.contains(r#"officialPackageName: "Payments""#));
        assert!(metadata.contains(r#"upstreamPackage: "@stripe/stripe-js""#));
        assert!(metadata.contains("forms/react-hook-form"));
    }

    #[test]
    fn vercel_ai_alias_materializes_launch_ai_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("vercel-ai"), "ai/vercel-ai");
        assert_eq!(canonical_package_id("ai-sdk"), "ai/vercel-ai");
        assert_eq!(canonical_package_id("npm/ai"), "ai/vercel-ai");

        let package = source_package_for_project("vercel-ai", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "ai/vercel-ai");
        assert_eq!(package.upstream_name, "ai");
        assert_eq!(package.version, "7.0.0-canary.146-dx.0");
        assert!(paths.contains(&"lib/ai/model.ts"));
        assert!(paths.contains(&"lib/ai/tools.ts"));
        assert!(paths.contains(&"lib/ai/chat-route.ts"));
        assert!(paths.contains(&"lib/ai/client-chat.tsx"));
        assert!(paths.contains(&"lib/ai/metadata.ts"));
        assert!(paths.contains(&"app/api/ai/chat/route.ts"));
        assert!(paths.contains(&"lib/ai/README.md"));

        let chat_route = package
            .files
            .iter()
            .find(|file| file.path == "lib/ai/chat-route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("chat route source");
        assert!(
            chat_route
                .contains(r#"import { convertToModelMessages, streamText, tool } from "ai";"#)
        );
        assert!(chat_route.contains("createDxAIChatRoute"));
        assert!(chat_route.contains("result.toUIMessageStreamResponse()"));

        let tools = package
            .files
            .iter()
            .find(|file| file.path == "lib/ai/tools.ts")
            .and_then(|file| file.content.as_deref())
            .expect("tools source");
        assert!(tools.contains(r#"import { tool } from "ai";"#));
        assert!(tools.contains("z.object"));
        assert!(tools.contains("createDxLaunchTools"));

        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "lib/ai/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata source");
        assert!(metadata.contains(r#"packageId: "ai/vercel-ai""#));
        assert!(metadata.contains(r#"upstreamPackage: "ai""#));
        assert!(metadata.contains("streamText"));
        assert!(metadata.contains("DefaultChatTransport"));

        let template_route = package
            .files
            .iter()
            .find(|file| file.path == "app/api/ai/chat/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("template route source");
        assert!(template_route.contains(r#"from "@/lib/ai/chat-route";"#));
        assert!(template_route.contains("POST"));
    }

    #[test]
    fn shadcn_button_rewrites_utils_import_for_custom_paths() {
        let dir = tempdir().expect("tempdir");
        fs::write(
            dir.path().join("dx"),
            "forge.paths.js_ui=src/ui\nforge.paths.js_lib=src/shared/lib\n",
        )
        .expect("write config");

        let package = source_package_for_project("shadcn/ui/button", dir.path()).expect("package");
        let button = package
            .files
            .iter()
            .find(|file| file.path == "src/ui/button.tsx")
            .expect("button file");

        assert!(
            button
                .content
                .as_deref()
                .expect("button content")
                .contains(r#"import { cn } from "../shared/lib/utils";"#)
        );
    }

    #[test]
    fn icon_search_rewrites_helper_import_for_custom_paths() {
        let dir = tempdir().expect("tempdir");
        fs::write(
            dir.path().join("dx"),
            "forge.paths.js_icons=src/icons\nforge.paths.js_lib=src/shared/lib\n",
        )
        .expect("write config");

        let package = source_package_for_project("icon/search", dir.path()).expect("package");
        let icon = package
            .files
            .iter()
            .find(|file| file.path == "src/icons/search.tsx")
            .expect("icon file");

        assert!(
            icon.content
                .as_deref()
                .expect("icon content")
                .contains(r#"from "../shared/lib/icons";"#)
        );
    }

    #[test]
    fn source_package_variant_materializes_into_isolated_paths() {
        let dir = tempdir().expect("tempdir");

        let package = source_package_for_project_variant("ui/button", dir.path(), "marketing")
            .expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let button = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/variants/marketing/button.tsx")
            .expect("variant button");

        assert_eq!(package.package_id, "shadcn/ui/button");
        assert_eq!(package.variant, "marketing");
        assert!(paths.contains(&"components/ui/variants/marketing/button.tsx"));
        assert!(paths.contains(&"components/ui/variants/marketing/slot.tsx"));
        assert!(paths.contains(&"lib/forge/variants/marketing/utils.ts"));
        assert!(
            button
                .content
                .as_deref()
                .expect("button content")
                .contains(r#"from "../../../../lib/forge/variants/marketing/utils";"#)
        );
    }

    #[test]
    fn unsafe_source_package_variant_names_are_rejected() {
        assert!(default_source_package_variant("ui/button", "brand-a").is_ok());
        assert!(default_source_package_variant("ui/button", "../brand").is_err());
        assert!(default_source_package_variant("ui/button", "brand/name").is_err());
        assert!(default_source_package_variant("ui/button", "brand name").is_err());
    }

    #[test]
    fn local_registry_init_writes_index_manifest_and_blobs() {
        let dir = tempdir().expect("tempdir");

        let report = init_local_registry(dir.path()).expect("init");

        assert!(dir.path().join("index.json").exists());
        assert!(
            dir.path()
                .join("packages/js/shadcn/ui/button/0.1.0/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/shadcn/ui/badge/0.1.0/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/shadcn/ui/label/0.1.0/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/dx/icon/search/0.1.0/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/auth/better-auth/1.6.11-dx.9/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/validation/zod/4.4.3-dx.12/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/forms/react-hook-form/7.75.0-dx.0/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/payments/stripe-js/9.6.0-dx.1/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/content/fumadocs-next/16.8.12-dx.11/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/db/drizzle-sqlite/0.1.0/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(
            dir.path()
                .join("packages/js/migration/static-site/0.1.0/.dx/build-cache/manifest.json")
                .exists()
        );
        assert!(report.objects.len() >= 8);
    }

    #[test]
    fn r2_dry_run_object_urls_handle_missing_bucket_without_secrets() {
        let package = registry_package("shadcn/ui/button").expect("package");
        let status = DxForgeR2Status {
            configured: false,
            setup_status: "missing-config".to_string(),
            missing_config: vec![
                "account_id_or_endpoint".to_string(),
                "access_key_id".to_string(),
                "secret_access_key".to_string(),
                "bucket".to_string(),
            ],
            account_id_set: false,
            access_key_id_set: false,
            secret_access_key_set: false,
            bucket_set: false,
            endpoint_set: false,
            public_base_url_set: false,
            bucket: None,
            endpoint: None,
            public_base_url: None,
            prefix: DEFAULT_R2_PREFIX.to_string(),
        };
        let objects = registry_object_keys(&status.prefix, &package, true)
            .into_iter()
            .map(|key| object_url_from_status(&status, &key))
            .collect::<Vec<_>>();

        assert!(
            objects
                .iter()
                .any(|object| object.contains("packages/js/shadcn/ui/button/0.1.0/.dx/build-cache/manifest.json"))
        );
        assert!(
            objects
                .iter()
                .all(|object| object.starts_with("r2://<unconfigured-bucket>/"))
        );
    }

    #[test]
    fn r2_dry_run_object_urls_redact_configured_bucket_values() {
        let package = registry_package("shadcn/ui/button").expect("package");
        let status = DxForgeR2Status {
            configured: true,
            setup_status: "configured".to_string(),
            missing_config: Vec::new(),
            account_id_set: true,
            access_key_id_set: true,
            secret_access_key_set: true,
            bucket_set: true,
            endpoint_set: true,
            public_base_url_set: true,
            bucket: Some("secret-bucket".to_string()),
            endpoint: Some("https://secret-account.r2.cloudflarestorage.com".to_string()),
            public_base_url: Some("https://public.example.invalid/private".to_string()),
            prefix: DEFAULT_R2_PREFIX.to_string(),
        };
        let objects = registry_object_keys(&status.prefix, &package, true)
            .into_iter()
            .map(|key| object_url_from_status(&status, &key))
            .collect::<Vec<_>>();
        let serialized = serde_json::to_string(&status).expect("status json");
        let object_plan = objects.join("\n");

        assert!(
            objects
                .iter()
                .all(|object| object.starts_with("r2://<configured-bucket>/"))
        );
        assert!(!serialized.contains("secret-bucket"));
        assert!(!serialized.contains("secret-account"));
        assert!(!serialized.contains("public.example.invalid"));
        assert!(!object_plan.contains("secret-bucket"));
        assert!(!object_plan.contains("secret-account"));
        assert!(!object_plan.contains("public.example.invalid"));
    }
