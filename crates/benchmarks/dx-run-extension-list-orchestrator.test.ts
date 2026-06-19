const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("dx run defaults to the dx extension list orchestrator", () => {
  const orchestratorPath = "dx-www/src/cli/extension_orchestrator.rs";
  assert.ok(fs.existsSync(path.join(root, orchestratorPath)), "extension orchestrator module missing");

  const scriptRunner = read("dx-www/src/cli/script_runner.rs");
  const cliMod = read("dx-www/src/cli/mod.rs");
  const orchestrator = read(orchestratorPath);
  const artifacts = read("dx-www/src/cli/serializer_artifacts.rs");
  const webTools = read("dx-www/src/cli/public_framework_tools.rs");
  const serializerOutput = read("../serializer/src/llm/serializer_output.rs");

  assert.match(cliMod, /mod extension_orchestrator;/);
  assert.match(cliMod, /mod serializer_artifacts;/);
  assert.match(scriptRunner, /run_dx_extension_orchestrator\(cwd, args\)/);
  assert.match(orchestrator, /DX_EXTENSION_LIST_SCHEMA/);
  assert.match(orchestrator, /ensure_dx_extension_list/);
  assert.match(orchestrator, /default_dx_project_config\(project_name\)/);
  assert.doesNotMatch(orchestrator, /www-or-generic/);
  assert.match(orchestrator, /read_dx_machine_document/);
  assert.match(orchestrator, /run_dx_icons\(project, &\["sync"/);
  assert.match(orchestrator, /run_dx_imports\(project, &\["sync"/);
  assert.match(orchestrator, /run_dx_style\(project, &\["build"/);
  assert.match(orchestrator, /run_dx_packages_check\(project, &\["run"/);
  assert.match(orchestrator, /run_dx_web_perf_check\(project, &web_perf_args/);
  assert.match(orchestrator, /score_scale": 500/);
  assert.match(orchestrator, /lighthouse_score_source/);
  assert.match(orchestrator, /\.dx\/run\/orchestrator\.sr/);
  assert.match(orchestrator, /write_sr_artifact/);
  assert.match(webTools, /"--lighthouse"/);
  assert.match(webTools, /run_lighthouse_measurement/);
  assert.match(webTools, /\.dx\/check\/500-points-lighthouse\.sr/);
  assert.match(artifacts, /write_sr_artifact/);
  assert.match(artifacts, /read_dx_machine_document/);
  assert.match(artifacts, /machine_to_document/);
  assert.match(artifacts, /SerializerOutputConfig::new\(\)/);
  assert.match(serializerOutput, /flatten_serializer_output_stem/);
  assert.match(serializerOutput, /forge-data\.machine/);
});

test("default www template declares the extension list inside dx", () => {
  const newCommand = read("dx-www/src/cli/new_command.rs");
  const templateDx = read("examples/template/dx");
  const templateClassUtils = read("examples/template/lib/utils.ts");
  const config = read("dx-www/src/config.rs");
  const configSource = read("dx-www/src/config_source.rs");

  for (const source of [newCommand, templateDx]) {
    assert.match(source, /project\(name=.*version=0\.1\.0 kind=www-app\)/);
    assert.match(source, /www\(\s*app_dir=app\s+output_dir=\.dx\/www\/output\s*\)/s);
    assert.doesNotMatch(source, /runtime\(/);
    assert.doesNotMatch(source, /dx-www-html|dx-www-js|dx-www-wasm|dx-www-protocol/);
    assert.doesNotMatch(source, /client=|client-tiny/);
    assert.match(source, /style\(\s*(?:mode=generated-css\s+)?tokens=styles\/theme\.css\s+generated_css=styles\/generated\.css\s*\)/s);
    assert.match(
      source,
      /icons\(component=Icon\s+(?:source_tag=icon\s+runtime_tag=dx-icon\s+)?generated_dir=components\/icons\)/,
    );
    assert.match(source, /forge\(policy=forge-first-no-node-modules\)/);
    assert.doesNotMatch(source, /wwtsx|tseq/);
    assert.match(source, /check\(score_scale=500 lighthouse=true\)/);
    assert.match(source, /docs\(\s*route=\/docs\s+content=content\/docs\s+openapi=openapi\/dx-www\.yaml\s*\)/s);
    assert.doesNotMatch(source, /tools\[name command enabled output\]\(/);
    assert.doesNotMatch(source, /watch\[tool extensions\]\(/);
    assert.doesNotMatch(source, /packages\[id version source surfaces\]\(/);
    assert.doesNotMatch(source, /classnames\(/);
    assert.doesNotMatch(source, /ui\(/);
    assert.doesNotMatch(source, /biome\(/);
    assert.doesNotMatch(source, /dx extension file/i);
    assert.doesNotMatch(source, /project\.kind=/);
    assert.doesNotMatch(source, /tooling\./);
    assert.doesNotMatch(source, /extensions\./);
  }

  assert.match(config, /pub generated_dir: String/);
  assert.match(config, /pub ui: UiToolingConfig/);
  assert.match(config, /pub classnames: ClassnamesToolingConfig/);
  assert.match(configSource, /icons\.generated_dir/);
  assert.match(configSource, /ui\.components_dir/);
  assert.match(configSource, /classnames\.helper/);
  assert.match(configSource, /document\.get_path\(key\)/);
  assert.match(templateClassUtils, /export function classes/);
  assert.match(templateClassUtils, /export const dxClass = classes/);
  assert.match(templateClassUtils, /export const cn = classes/);
});
