/**
 * CHANGELOG.md, parsed for "Co je nového". The grammar is in
 * docs/versioning.md and enforced by `npm run check:version` (the reference
 * parser is `parseChangelog` in scripts/release.mjs); here it is lenient:
 * unknown lines are skipped, nothing throws.
 */

export type GroupKey = "materials" | "app" | "fixes";

export interface Group {
  key: GroupKey;
  items: string[];
}

export interface Release {
  version: string;
  /** YYYY-MM-DD */
  date: string;
  groups: Group[];
}

export interface Changelog {
  unreleased: Group[];
  /** Newest first. */
  releases: Release[];
}

const GROUPS: Record<string, GroupKey> = { Materiály: "materials", Aplikace: "app", Opravy: "fixes" };

export function parseChangelog(text: string): Changelog {
  const out: Changelog = { unreleased: [], releases: [] };
  let groups: Group[] | null = null;
  let group: Group | null = null;
  let open = false; // the last item may take continuation lines
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trimEnd();
    if (line === "") {
      open = false;
      continue;
    }
    let m: RegExpExecArray | null;
    if (line === "## Unreleased") {
      groups = out.unreleased;
      group = null;
      open = false;
    } else if ((m = /^## (\d+\.\d+\.\d+) \((\d{4}-\d{2}-\d{2})\)$/.exec(line))) {
      const release: Release = { version: m[1], date: m[2], groups: [] };
      out.releases.push(release);
      groups = release.groups;
      group = null;
      open = false;
    } else if (line.startsWith("## ")) {
      groups = null;
      group = null;
    } else if (line.startsWith("### ")) {
      const key = GROUPS[line.slice(4).trim()];
      group = key && groups ? { key, items: [] } : null;
      if (group && groups) groups.push(group);
      open = false;
    } else if (line.startsWith("- ") && group) {
      group.items.push(line.slice(2).trim());
      open = true;
    } else if (/^ {2,}\S/.test(line) && group && open) {
      const items = group.items;
      items[items.length - 1] = `${items[items.length - 1]} ${line.trim()}`;
    }
  }
  for (const r of out.releases) r.groups = r.groups.filter((g) => g.items.length > 0);
  out.unreleased = out.unreleased.filter((g) => g.items.length > 0);
  return out;
}

/** -1, 0 or 1, comparing X.Y.Z versions as numbers. */
export function compareVersions(a: string, b: string): number {
  const pa = a.split(".").map(Number);
  const pb = b.split(".").map(Number);
  for (let i = 0; i < 3; i++) {
    const d = (pa[i] ?? 0) - (pb[i] ?? 0);
    if (d !== 0) return d < 0 ? -1 : 1;
  }
  return 0;
}

/**
 * The releases to show once after an update: those newer than the version
 * last seen, up to the running one. None on a fresh install (no version
 * seen) or after going back to an older build.
 */
export function newSince(log: Changelog, lastSeen: string | null, current: string): Release[] {
  if (!lastSeen || compareVersions(lastSeen, current) >= 0) return [];
  return log.releases.filter(
    (r) => compareVersions(r.version, lastSeen) > 0 && compareVersions(r.version, current) <= 0,
  );
}
