import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const read = (relative: string) => fs.readFileSync(path.join(root, relative), "utf8");

test("App Router route CSS is cached or emitted before HTML depends on it", () => {
  const runtime = read("dx-www/src/cli/app_router_runtime_command.rs");
  const build = read("dx-www/src/cli/app_router_build_command.rs");
  const sourceBuild = read("dx-www/src/build/source_engine/route_output.rs");

  assert.match(runtime, /remember_app_route_generated_styles\(cwd, &proof\);/);
  assert.match(build, /write_app_generated_style_assets[\s\S]+std::fs::write\(&html_path/);
  assert.match(sourceBuild, /write_generated_style_assets\(output_dir, &proof\.generated_styles\)\?/);
  assert.match(sourceBuild, /safe_generated_style_output_path/);
});
