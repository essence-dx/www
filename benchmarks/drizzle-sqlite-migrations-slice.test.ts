const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("drizzle sqlite slice exposes the real better-sqlite3 migrator API", () => {
  const drizzle = read("core/src/ecosystem/forge_drizzle.rs");
  const dataStatus = read("examples/template/data-status.tsx");

  assert.match(drizzle, /"js\/db\/drizzle\/migrations\.ts"/);
  assert.match(drizzle, /DRIZZLE_MIGRATIONS_TS/);
  assert.match(drizzle, /from "drizzle-orm\/better-sqlite3\/migrator"/);
  assert.match(drizzle, /from "drizzle-orm\/migrator"/);
  assert.match(drizzle, /applyDxDrizzleMigrations/);
  assert.match(drizzle, /buildDxDrizzleMigrationConfig/);
  assert.match(drizzle, /migrationsFolder/);
  assert.match(drizzle, /dxDrizzleMigrationDefaults/);
  assert.match(drizzle, /migrations: \{/);
  assert.match(drizzle, /appOwnedBoundaries/);
  assert.match(drizzle, /Migration SQL files and Drizzle Kit generation stay app-owned/);

  assert.match(dataStatus, /SQLite migrations/);
  assert.match(dataStatus, /dxDrizzlePackage\.migrations\.helper/);
  assert.match(dataStatus, /dxDrizzlePackage\.migrations\.defaultFolder/);
  assert.match(dataStatus, /key=\{`\$\{surface\.packageId\}:\$\{surface\.label\}`\}/);
});
