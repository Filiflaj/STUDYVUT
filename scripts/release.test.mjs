// Tests of the pure parts of release.mjs (and so of check-version.mjs).
// Run: node --test scripts/release.test.mjs
import assert from "node:assert/strict";
import { describe, test } from "node:test";
import {
  applyVersion,
  cargoLockVersions,
  commitMessage,
  compareVersions,
  findProblems,
  localDate,
  nextVersion,
  parseArgs,
  parseChangelog,
  parseVersion,
  releaseChangelog,
  releaseNotes,
  sectionItems,
  setCargoLockVersions,
  setJsonVersions,
  setTomlVersion,
  tomlString,
  tomlVersion,
  WORKSPACE,
  workspaceMembers,
} from "./release.mjs";

const lines = (...l) => `${l.join("\n")}\n`;

describe("versions", () => {
  test("parse X.Y.Z only", () => {
    assert.deepEqual(parseVersion("0.12.3"), [0, 12, 3]);
    for (const bad of ["v1.2.3", "1.2", "1.2.3.4", "01.2.3", "1.2.3-rc.1", "", undefined]) {
      assert.equal(parseVersion(bad), null, String(bad));
    }
  });

  test("compare numerically", () => {
    assert.equal(compareVersions("0.10.0", "0.9.9"), 1);
    assert.equal(compareVersions("1.0.0", "1.0.0"), 0);
    assert.equal(compareVersions("0.2.1", "0.3.0"), -1);
  });

  test("bump a level", () => {
    assert.equal(nextVersion("0.4.2", "patch"), "0.4.3");
    assert.equal(nextVersion("0.4.2", "minor"), "0.5.0");
    assert.equal(nextVersion("0.4.2", "major"), "1.0.0", "major before 1.0 is the 1.0 release");
    assert.equal(nextVersion("1.9.3", "major"), "2.0.0");
  });

  test("an explicit version must be newer", () => {
    assert.equal(nextVersion("0.4.2", "0.6.0"), "0.6.0");
    assert.equal(nextVersion("0.4.2", "v0.4.3"), "0.4.3");
    assert.throws(() => nextVersion("0.4.2", "0.4.2"), /not newer/);
    assert.throws(() => nextVersion("0.4.2", "0.3.9"), /not newer/);
    assert.throws(() => nextVersion("0.4.2", "next"), /neither major, minor, patch/);
  });

  test("the first release names its version, which may be the current one", () => {
    assert.equal(nextVersion("0.1.0", "0.1.0", { firstRelease: true }), "0.1.0");
    assert.equal(nextVersion("0.1.0", "0.2.0", { firstRelease: true }), "0.2.0");
    assert.throws(() => nextVersion("0.1.0", "minor", { firstRelease: true }), /name the first version/);
  });

  test("the local date", () => {
    assert.equal(localDate(new Date(2026, 0, 5, 23, 59)), "2026-01-05");
  });
});

const CHANGELOG = lines(
  "# Changelog",
  "",
  "## Unreleased",
  "",
  "### Materiály",
  "",
  "- BPC-FY1: nová kapitola **Práce a energie** a 18 otázek v kvízu",
  "  Mechanika.",
  "- BPC-MA1: kvíz `limity`.",
  "",
  "### Opravy",
  "",
  "- BPC-FY1: otázka M7 měla špatně označenou správnou odpověď.",
  "",
  "## 0.2.0 (2026-11-28)",
  "",
  "### Aplikace",
  "",
  "- Kvízy: sada **K opakování**.",
  "",
  "## 0.1.0 (2026-11-21)",
  "",
  "### Materiály",
  "",
  "- První verze.",
);

describe("parseChangelog", () => {
  test("reads sections, groups and wrapped items", () => {
    const { unreleased, releases, problems } = parseChangelog(CHANGELOG);
    assert.deepEqual(problems, []);
    assert.deepEqual(
      unreleased.groups.map((g) => [g.title, g.items]),
      [
        [
          "Materiály",
          ["BPC-FY1: nová kapitola **Práce a energie** a 18 otázek v kvízu Mechanika.", "BPC-MA1: kvíz `limity`."],
        ],
        ["Opravy", ["BPC-FY1: otázka M7 měla špatně označenou správnou odpověď."]],
      ],
    );
    assert.deepEqual(
      releases.map((r) => [r.version, r.date, r.line]),
      [
        ["0.2.0", "2026-11-28", 15],
        ["0.1.0", "2026-11-21", 21],
      ],
    );
    assert.equal(sectionItems(releases[0]).length, 1);
  });

  test("an empty Unreleased and CRLF line ends are fine", () => {
    const text = "# Changelog\r\n\r\n## Unreleased\r\n\r\n## 0.1.0 (2026-11-21)\r\n\r\n### Opravy\r\n\r\n- Oprava.\r\n";
    const { unreleased, releases, problems } = parseChangelog(text);
    assert.deepEqual(problems, []);
    assert.deepEqual(sectionItems(unreleased), []);
    assert.equal(releases[0].version, "0.1.0");
  });

  const broken = {
    "the title": ["# Změny", "", "## Unreleased"],
    "an (unreleased) version heading": ["# Changelog", "", "## 0.2.0 (unreleased)", "", "### Opravy", "", "- x"],
    "a heading without a date": ["# Changelog", "", "## 0.2.0", "", "### Opravy", "", "- x"],
    "an impossible date": ["# Changelog", "", "## 0.2.0 (2026-02-30)", "", "### Opravy", "", "- x"],
    "Unreleased after a release": [
      "# Changelog",
      "",
      "## 0.1.0 (2026-11-21)",
      "",
      "### Opravy",
      "",
      "- x",
      "",
      "## Unreleased",
    ],
    "two Unreleased": ["# Changelog", "", "## Unreleased", "", "## Unreleased"],
    "versions not falling": [
      "# Changelog",
      "",
      "## 0.1.0 (2026-11-21)",
      "",
      "### Opravy",
      "",
      "- x",
      "",
      "## 0.2.0 (2026-11-20)",
      "",
      "### Opravy",
      "",
      "- y",
    ],
    "dates rising": [
      "# Changelog",
      "",
      "## 0.2.0 (2026-11-21)",
      "",
      "### Opravy",
      "",
      "- x",
      "",
      "## 0.1.0 (2026-11-22)",
      "",
      "### Opravy",
      "",
      "- y",
    ],
    "an unknown group": ["# Changelog", "", "## Unreleased", "", "### Novinky", "", "- x"],
    "groups out of order": ["# Changelog", "", "## Unreleased", "", "### Opravy", "", "- x", "", "### Materiály", "", "- y"],
    "a group twice": ["# Changelog", "", "## Unreleased", "", "### Opravy", "", "- x", "", "### Opravy", "", "- y"],
    "an empty group": ["# Changelog", "", "## Unreleased", "", "### Opravy", ""],
    "an item outside a group": ["# Changelog", "", "## Unreleased", "", "- x"],
    "an empty item": ["# Changelog", "", "## Unreleased", "", "### Opravy", "", "-"],
    "a nested list": ["# Changelog", "", "## Unreleased", "", "### Opravy", "", "- x", "  - y"],
    "a paragraph inside an item": ["# Changelog", "", "## Unreleased", "", "### Opravy", "", "- x", "", "  y"],
    "text outside an item": ["# Changelog", "", "## Unreleased", "", "Nějaký úvod."],
    "a star bullet": ["# Changelog", "", "## Unreleased", "", "### Opravy", "", "* x"],
    "a fourth-level heading": ["# Changelog", "", "## Unreleased", "", "### Opravy", "", "- x", "", "#### Detail"],
  };
  for (const [name, text] of Object.entries(broken)) {
    test(`reports ${name}`, () => {
      assert.notDeepEqual(parseChangelog(lines(...text)).problems, [], name);
    });
  }
});

describe("releaseChangelog", () => {
  test("dates Unreleased and opens a new one, nothing else changes", () => {
    const out = releaseChangelog(CHANGELOG, "0.3.0", "2026-12-01");
    assert.equal(
      out,
      CHANGELOG.replace("## Unreleased\n", "## Unreleased\n\n## 0.3.0 (2026-12-01)\n"),
    );
    const { unreleased, releases, problems } = parseChangelog(out);
    assert.deepEqual(problems, []);
    assert.deepEqual(sectionItems(unreleased), []);
    assert.deepEqual(
      releases.map((r) => r.version),
      ["0.3.0", "0.2.0", "0.1.0"],
    );
    assert.equal(sectionItems(releases[0]).length, 3);
  });

  test("keeps CRLF line ends", () => {
    const crlf = CHANGELOG.replace(/\n/g, "\r\n");
    const out = releaseChangelog(crlf, "0.3.0", "2026-12-01");
    assert.ok(out.includes("## Unreleased\r\n\r\n## 0.3.0 (2026-12-01)\r\n"));
    assert.ok(!/[^\r]\n/.test(out));
  });

  test("refuses an empty or missing Unreleased", () => {
    const released = releaseChangelog(CHANGELOG, "0.3.0", "2026-12-01");
    assert.throws(() => releaseChangelog(released, "0.3.1", "2026-12-02"), /nothing to release/);
    assert.throws(
      () => releaseChangelog(lines("# Changelog", "", "## 0.1.0 (2026-11-21)", "", "### Opravy", "", "- x"), "0.1.1", "2026-12-02"),
      /no "## Unreleased"/,
    );
  });

  test("refuses TODO placeholders", () => {
    const text = lines("# Changelog", "", "## Unreleased", "", "### Materiály", "", "- TODO: doplnit předměty.");
    assert.throws(() => releaseChangelog(text, "0.1.0", "2026-11-21"), /placeholders/);
  });

  test("refuses a version that is not newer than the last release", () => {
    assert.throws(() => releaseChangelog(CHANGELOG, "0.2.0", "2026-12-01"), /already has 0.2.0/);
  });

  test("refuses a changelog that breaks the grammar", () => {
    const text = CHANGELOG.replace("### Opravy", "### Novinky");
    assert.throws(() => releaseChangelog(text, "0.3.0", "2026-12-01"), /unknown group/);
  });

  test("refuses a bad date", () => {
    assert.throws(() => releaseChangelog(CHANGELOG, "0.3.0", "1. 12. 2026"), /not a date/);
  });
});

test("releaseNotes gives a release's groups and items", () => {
  assert.equal(releaseNotes(CHANGELOG, "0.2.0"), "### Aplikace\n\n- Kvízy: sada **K opakování**.\n");
  assert.equal(releaseNotes(CHANGELOG, "v0.1.0"), "### Materiály\n\n- První verze.\n");
  assert.throws(() => releaseNotes(CHANGELOG, "0.9.0"), /no section/);
});

const WORKSPACE_TOML = lines(
  "[workspace]",
  'resolver = "3"',
  "members = [",
  '    "crates/studyvut-core", # the model',
  '    "src-tauri",',
  "]",
  "",
  "[workspace.package]",
  'version = "0.1.0"',
  'edition = "2024"',
  "",
  "[workspace.dependencies]",
  'studyvut-core = { path = "crates/studyvut-core" }',
  'clap = { version = "4", features = ["derive"] }',
);

const APP_TOML = lines(
  "[package]",
  'name = "studyvut"',
  'version = "0.1.0"',
  "edition.workspace = true",
  "",
  "[dependencies.tauri]",
  'version = "2"',
);

const CORE_TOML = lines("[package]", 'name = "studyvut-core"', "version.workspace = true", "", "[dependencies]", 'serde = "1"');

describe("TOML", () => {
  test("reads the members, names and versions", () => {
    assert.deepEqual(workspaceMembers(WORKSPACE_TOML), ["crates/studyvut-core", "src-tauri"]);
    assert.deepEqual(workspaceMembers('[workspace]\nmembers = ["a", "b/*"]\n'), ["a", "b/*"]);
    assert.equal(tomlVersion(WORKSPACE_TOML, "workspace.package"), "0.1.0");
    assert.equal(tomlVersion(APP_TOML, "package"), "0.1.0");
    assert.equal(tomlVersion(CORE_TOML, "package"), WORKSPACE);
    assert.equal(tomlVersion("[package]\nversion = { workspace = true }\n", "package"), WORKSPACE);
    assert.equal(tomlString(APP_TOML, "package", "name"), "studyvut");
    assert.equal(tomlVersion(APP_TOML, "dependencies.tauri"), "2");
    assert.equal(tomlVersion(APP_TOML, "missing"), undefined);
  });

  test("sets only the version of the given table", () => {
    assert.equal(
      setTomlVersion(WORKSPACE_TOML, "workspace.package", "0.2.0"),
      WORKSPACE_TOML.replace('version = "0.1.0"', 'version = "0.2.0"'),
    );
    const app = setTomlVersion(APP_TOML, "package", "0.2.0");
    assert.equal(app, APP_TOML.replace('version = "0.1.0"', 'version = "0.2.0"'));
    assert.ok(app.includes('[dependencies.tauri]\nversion = "2"'), "a dependency table keeps its version");
  });

  test("refuses an inherited or missing version", () => {
    assert.throws(() => setTomlVersion(CORE_TOML, "package", "0.2.0"), /inherits/);
    assert.throws(() => setTomlVersion('[package]\nname = "x"\n', "package", "0.2.0"), /no version/);
    assert.throws(() => setTomlVersion(CORE_TOML, "workspace.package", "0.2.0"), /no \[workspace.package\]/);
  });
});

const CARGO_LOCK = lines(
  "# This file is automatically @generated by Cargo.",
  "version = 4",
  "",
  "[[package]]",
  'name = "serde"',
  'version = "1.0.228"',
  'source = "registry+https://github.com/rust-lang/crates.io-index"',
  "",
  "[[package]]",
  'name = "studyvut"',
  'version = "0.1.0"',
  "dependencies = [",
  ' "studyvut-core",',
  "]",
  "",
  "[[package]]",
  'name = "studyvut-core"',
  'version = "0.1.0"',
  "dependencies = [",
  ' "serde",',
  "]",
  "",
  "[[package]]",
  'name = "studyvut-core"',
  'version = "0.0.9"',
  'source = "registry+https://github.com/rust-lang/crates.io-index"',
);

describe("Cargo.lock", () => {
  test("reads the workspace packages only", () => {
    const v = cargoLockVersions(CARGO_LOCK, ["studyvut", "studyvut-core", "serde"]);
    assert.deepEqual([...v], [
      ["studyvut", "0.1.0"],
      ["studyvut-core", "0.1.0"],
    ]);
  });

  test("sets the workspace packages, never a registry package of the same name", () => {
    const out = setCargoLockVersions(CARGO_LOCK, ["studyvut", "studyvut-core"], "0.2.0");
    assert.equal(out.match(/version = "0.2.0"/g).length, 2);
    assert.ok(out.includes('version = "1.0.228"'));
    assert.ok(out.includes('version = "0.0.9"'));
    assert.ok(out.startsWith("# This file is automatically @generated by Cargo.\nversion = 4\n"));
    assert.equal(out.split("\n").length, CARGO_LOCK.split("\n").length);
  });

  test("refuses a workspace package that is not locked", () => {
    assert.throws(() => setCargoLockVersions(CARGO_LOCK, ["studyvut-cli"], "0.2.0"), /no workspace package studyvut-cli/);
  });
});

const PACKAGE_JSON = `{
  "name": "studyvut",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "files": ["dist"],
  "scripts": {
    "release": "node scripts/release.mjs"
  }
}
`;

const PACKAGE_LOCK = `{
  "name": "studyvut",
  "version": "0.1.0",
  "lockfileVersion": 3,
  "requires": true,
  "packages": {
    "": {
      "name": "studyvut",
      "version": "0.1.0",
      "dependencies": {
        "katex": "^0.16.0"
      }
    },
    "node_modules/katex": {
      "version": "0.16.25",
      "license": "MIT"
    }
  }
}
`;

describe("JSON", () => {
  test("sets package.json's version and keeps the formatting", () => {
    assert.equal(
      setJsonVersions(PACKAGE_JSON, [["version"]], "0.2.0"),
      PACKAGE_JSON.replace('"version": "0.1.0"', '"version": "0.2.0"'),
    );
  });

  test("sets both versions of package-lock.json, not a dependency's", () => {
    const out = setJsonVersions(PACKAGE_LOCK, [["version"], ["packages", "", "version"]], "0.2.0");
    const lock = JSON.parse(out);
    assert.equal(lock.version, "0.2.0");
    assert.equal(lock.packages[""].version, "0.2.0");
    assert.equal(lock.packages["node_modules/katex"].version, "0.16.25");
    assert.equal(out, PACKAGE_LOCK.replaceAll('"version": "0.1.0"', '"version": "0.2.0"'));
  });

  test("refuses when the first version keys are not the expected ones", () => {
    const odd = '{\n  "engines": { "version": "x" },\n  "version": "0.1.0"\n}\n';
    assert.throws(() => setJsonVersions(odd, [["version"]], "0.2.0", "package.json"), /safely/);
    assert.throws(() => setJsonVersions('{"name": "x"}', [["version"]], "0.2.0", "package.json"), /no version/);
  });
});

describe("commitMessage", () => {
  test("is Release vX.Y.Z, then the trailers", () => {
    assert.equal(commitMessage("0.2.0"), "Release v0.2.0\n");
    assert.equal(
      commitMessage("0.2.0", ["Co-Authored-By: Claude <noreply@anthropic.com>", "Refs: #12"]),
      "Release v0.2.0\n\nCo-Authored-By: Claude <noreply@anthropic.com>\nRefs: #12\n",
    );
  });

  test("refuses what is not a trailer", () => {
    for (const bad of ["no colon", "Key:missing space", ": value", "Key: a\nb", "Bad Key: v"]) {
      assert.throws(() => commitMessage("0.2.0", [bad]), /not "Key: value"/, bad);
    }
  });
});

test("parseArgs", () => {
  assert.deepEqual(parseArgs(["minor", "--dry-run", "--trailer", "A: b", "--trailer=C: d"]), {
    spec: "minor",
    dryRun: true,
    trailers: ["A: b", "C: d"],
    notes: undefined,
    help: false,
  });
  assert.equal(parseArgs(["--notes", "0.2.0"]).notes, "0.2.0");
  assert.throws(() => parseArgs(["minor", "patch"]), /one version or level/);
  assert.throws(() => parseArgs(["--force"]), /unknown option/);
  assert.throws(() => parseArgs(["minor", "--trailer"]), /needs a value/);
});

describe("findProblems and applyVersion", () => {
  const repo = (overrides = {}) => ({
    members: ["crates/studyvut-core", "src-tauri"],
    files: {
      "package.json": PACKAGE_JSON,
      "package-lock.json": PACKAGE_LOCK,
      "Cargo.toml": WORKSPACE_TOML,
      "crates/studyvut-core/Cargo.toml": CORE_TOML,
      "src-tauri/Cargo.toml": APP_TOML,
      "Cargo.lock": CARGO_LOCK,
      "src-tauri/tauri.conf.json": '{\n  "productName": "STUDYVUT"\n}\n',
      "CHANGELOG.md": lines("# Changelog", "", "## Unreleased", "", "### Materiály", "", "- První verze."),
      ...overrides,
    },
  });

  test("a consistent repository before its first release", () => {
    assert.deepEqual(findProblems(repo()), []);
  });

  test("reports every version that differs", () => {
    const problems = findProblems(
      repo({
        "src-tauri/Cargo.toml": APP_TOML.replace("0.1.0", "0.1.1"),
        "Cargo.lock": CARGO_LOCK.replace('name = "studyvut-core"\nversion = "0.1.0"', 'name = "studyvut-core"\nversion = "0.0.1"'),
        "src-tauri/tauri.conf.json": '{ "version": "0.3.0" }',
      }),
    );
    assert.deepEqual(problems, [
      "src-tauri/Cargo.toml 0.1.1 != package.json 0.1.0",
      "Cargo.lock studyvut-core 0.0.1 != package.json 0.1.0",
      "src-tauri/tauri.conf.json 0.3.0 != package.json 0.1.0",
    ]);
  });

  test("reports missing files and a member without a version", () => {
    const r = repo({ "package-lock.json": undefined, "crates/studyvut-core/Cargo.toml": '[package]\nname = "studyvut-core"\n' });
    assert.deepEqual(findProblems(r), [
      "package-lock.json is missing",
      "crates/studyvut-core/Cargo.toml has no version: add version.workspace = true",
    ]);
  });

  test("the newest release must be the version", () => {
    const changelog = lines("# Changelog", "", "## Unreleased", "", "## 0.0.9 (2026-11-01)", "", "### Opravy", "", "- x");
    assert.deepEqual(findProblems(repo({ "CHANGELOG.md": changelog })), [
      "CHANGELOG.md: the newest release is 0.0.9, not 0.1.0",
    ]);
  });

  test("a tag needs its release commit", () => {
    assert.deepEqual(findProblems(repo(), "v0.2.0"), [
      'CHANGELOG.md has no "## 0.1.0 (YYYY-MM-DD)" section for v0.2.0',
      'CHANGELOG.md: "## Unreleased" is not empty, so v0.2.0 is not on its release commit',
      "tag v0.2.0 != package.json 0.1.0",
    ]);
  });

  test("a released TODO is reported", () => {
    const changelog = lines("# Changelog", "", "## Unreleased", "", "## 0.1.0 (2026-11-21)", "", "### Opravy", "", "- TODO: x");
    assert.deepEqual(findProblems(repo({ "CHANGELOG.md": changelog })), ["CHANGELOG.md: 0.1.0 has a TODO item"]);
  });

  test("applyVersion and releaseChangelog give a consistent release commit", () => {
    const r = repo();
    const changed = applyVersion(r, "0.2.0");
    assert.deepEqual(Object.keys(changed).sort(), [
      "Cargo.lock",
      "Cargo.toml",
      "package-lock.json",
      "package.json",
      "src-tauri/Cargo.toml",
    ]);
    changed["CHANGELOG.md"] = releaseChangelog(r.files["CHANGELOG.md"], "0.2.0", "2026-12-01");
    const after = { ...r, files: { ...r.files, ...changed } };
    assert.deepEqual(findProblems(after, "v0.2.0"), []);
    assert.deepEqual(findProblems(after), []);
    assert.deepEqual(findProblems(after, "v0.2.1"), ["tag v0.2.1 != package.json 0.2.0"]);
  });

  test("applyVersion sets a literal tauri.conf.json version and leaves a path alone", () => {
    const literal = applyVersion(repo({ "src-tauri/tauri.conf.json": '{\n  "version": "0.1.0"\n}\n' }), "0.2.0");
    assert.equal(literal["src-tauri/tauri.conf.json"], '{\n  "version": "0.2.0"\n}\n');
    const path = applyVersion(repo({ "src-tauri/tauri.conf.json": '{ "version": "../package.json" }' }), "0.2.0");
    assert.equal(path["src-tauri/tauri.conf.json"], undefined);
  });

  test("the first release changes only the changelog", () => {
    assert.deepEqual(applyVersion(repo(), "0.1.0"), {});
  });
});
