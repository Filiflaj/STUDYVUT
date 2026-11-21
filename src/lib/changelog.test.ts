import { describe, expect, it } from "vitest";
import changelog from "../../CHANGELOG.md?raw";
import pkg from "../../package.json";
import { compareVersions, newSince, parseChangelog } from "./changelog";

const SAMPLE = `# Changelog

## Unreleased

### Opravy

- Oprava překlepu.

## 0.2.0 (2026-12-03)

### Materiály

- BPC-FY1: nová kapitola **Práce a energie**,
  18 nových otázek.

### Aplikace

- Statistiky po předmětech.

## 0.1.0 (2026-11-21)

### Materiály

- První verze.
`;

describe("parseChangelog", () => {
  it("reads sections, groups and wrapped items", () => {
    const log = parseChangelog(SAMPLE);
    expect(log.unreleased).toEqual([{ key: "fixes", items: ["Oprava překlepu."] }]);
    expect(log.releases.map((r) => r.version)).toEqual(["0.2.0", "0.1.0"]);
    expect(log.releases[0].groups).toEqual([
      { key: "materials", items: ["BPC-FY1: nová kapitola **Práce a energie**, 18 nových otázek."] },
      { key: "app", items: ["Statistiky po předmětech."] },
    ]);
    expect(log.releases[0].date).toBe("2026-12-03");
  });

  it("skips what it does not know", () => {
    const log = parseChangelog("# Changelog\n## 1.0.0 (2027-01-01)\n### Jiné\n- x\n### Opravy\n- y\nnoise\n");
    expect(log.releases[0].groups).toEqual([{ key: "fixes", items: ["y"] }]);
  });

  it("matches package.json: the newest release is the app's version, or none yet", () => {
    const log = parseChangelog(changelog);
    const newest = log.releases[0]?.version;
    if (newest !== undefined) expect(newest).toBe(pkg.version);
  });
});

describe("versions", () => {
  it("compare as numbers", () => {
    expect(compareVersions("0.10.0", "0.9.9")).toBe(1);
    expect(compareVersions("1.0.0", "1.0.0")).toBe(0);
    expect(compareVersions("0.1.2", "0.2.0")).toBe(-1);
  });

  it("show what is new once after an update only", () => {
    const log = parseChangelog(SAMPLE);
    expect(newSince(log, "0.1.0", "0.2.0").map((r) => r.version)).toEqual(["0.2.0"]);
    expect(newSince(log, null, "0.2.0")).toEqual([]);
    expect(newSince(log, "0.2.0", "0.1.0")).toEqual([]);
    expect(newSince(log, "0.0.9", "0.2.0").map((r) => r.version)).toEqual(["0.2.0", "0.1.0"]);
  });
});
