// Cut a release: set every version field, date the changelog's Unreleased
// section, commit "Release vX.Y.Z" and tag vX.Y.Z. Never pushes.
//
//   node scripts/release.mjs <major|minor|patch|X.Y.Z> [--dry-run] [--trailer "Key: value"]...
//   node scripts/release.mjs --notes X.Y.Z    prints that version's changelog section
//
// The policy is in docs/versioning.md. Everything that does not touch the disk
// or git is an exported function, shared with check-version.mjs and tested by
// release.test.mjs. Node built-ins only; runs on Node 22 and later.
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, realpathSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

export const ROOT = fileURLToPath(new URL("..", import.meta.url));
export const TAURI_CONF = "src-tauri/tauri.conf.json";
/** The changelog's groups, in their order. The app maps them to its own labels. */
export const GROUPS = ["Materiály", "Aplikace", "Opravy"];
/** tomlVersion's answer for `version.workspace = true`. */
export const WORKSPACE = "workspace";

// ---------------------------------------------------------------- versions

const VERSION = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/;

/** [major, minor, patch] of "X.Y.Z", or null. No "v", no pre-release. */
export function parseVersion(text) {
  const m = VERSION.exec(typeof text === "string" ? text : "");
  return m ? m.slice(1).map(Number) : null;
}

/** -1, 0 or 1. */
export function compareVersions(a, b) {
  const x = parseVersion(a);
  const y = parseVersion(b);
  if (!x || !y) throw new Error(`cannot compare ${a} and ${b}: not X.Y.Z`);
  for (let i = 0; i < 3; i++) if (x[i] !== y[i]) return x[i] < y[i] ? -1 : 1;
  return 0;
}

/**
 * The version that follows `current` for `spec`: "major", "minor", "patch" or
 * an explicit "X.Y.Z" (a leading "v" is allowed), which must be newer.
 * `firstRelease` (nothing released yet): the explicit version may equal
 * `current`, and a level is refused, since it would skip the version the
 * files already carry.
 */
export function nextVersion(current, spec, { firstRelease = false } = {}) {
  const cur = parseVersion(current);
  if (!cur) throw new Error(`the current version ${current} is not X.Y.Z`);
  const [major, minor, patch] = cur;
  if (spec === "major" || spec === "minor" || spec === "patch") {
    if (firstRelease) {
      throw new Error(`nothing is released yet: name the first version, e.g. \`npm run release -- ${current}\``);
    }
    if (spec === "major") return `${major + 1}.0.0`;
    if (spec === "minor") return `${major}.${minor + 1}.0`;
    return `${major}.${minor}.${patch + 1}`;
  }
  const v = String(spec ?? "").replace(/^v/, "");
  if (!parseVersion(v)) throw new Error(`"${spec}" is neither major, minor, patch nor a version X.Y.Z`);
  const order = compareVersions(v, current);
  if (order < 0 || (order === 0 && !firstRelease)) throw new Error(`${v} is not newer than ${current}`);
  return v;
}

/** YYYY-MM-DD of `date` in the local time zone. */
export function localDate(date) {
  const pad = (n) => String(n).padStart(2, "0");
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
}

function isDate(text) {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(text)) return false;
  const d = new Date(`${text}T00:00:00Z`);
  return !Number.isNaN(d.getTime()) && d.toISOString().slice(0, 10) === text;
}

// ---------------------------------------------------------------- changelog

/**
 * Parses CHANGELOG.md and checks it against the grammar in docs/versioning.md.
 * Returns { unreleased, releases, problems }: `unreleased` is the
 * "## Unreleased" section or null, `releases` the dated sections newest first;
 * a section is { version, date, line, groups: [{ title, items }] } (`line`:
 * the 1-based line of its heading; Unreleased has version and date null),
 * an item is its text with wrapped lines joined by one space.
 */
export function parseChangelog(text) {
  const lines = String(text ?? "").split(/\r?\n/);
  const problems = [];
  const releases = [];
  let unreleased = null;
  let section = null;
  let group = null;
  let item = null; // index of the open item in group.items
  const problem = (i, message) => problems.push(`CHANGELOG.md:${i + 1}: ${message}`);

  if (lines[0]?.trimEnd() !== "# Changelog") problem(0, 'the first line must be "# Changelog"');
  for (let i = 1; i < lines.length; i++) {
    const line = lines[i].trimEnd();
    if (line === "") {
      item = null;
      continue;
    }
    let m;
    if (line.startsWith("## ")) {
      group = null;
      item = null;
      // A section with an error is parsed on, detached, so that its lines raise no more problems.
      section = { version: null, date: null, line: i + 1, groups: [] };
      if (line === "## Unreleased") {
        if (unreleased) problem(i, 'a second "## Unreleased"');
        else if (releases.length) problem(i, '"## Unreleased" must come before every release');
        else unreleased = section;
      } else if (/^## \S+ \(unreleased\)$/i.test(line)) {
        problem(i, `"${line}": collect the lines under "## Unreleased"; the release names and dates the version`);
      } else if ((m = /^## (\S+) \((\S+)\)$/.exec(line))) {
        const newer = releases.at(-1);
        if (!parseVersion(m[1])) problem(i, `"${m[1]}" is not a version X.Y.Z`);
        else if (!isDate(m[2])) problem(i, `"${m[2]}" is not a date YYYY-MM-DD`);
        else if (newer && compareVersions(m[1], newer.version) >= 0) {
          problem(i, `${m[1]} must be older than ${newer.version} above it`);
        } else if (newer && m[2] > newer.date) problem(i, `${m[1]} is dated after ${newer.version} above it`);
        else {
          section.version = m[1];
          section.date = m[2];
          releases.push(section);
        }
      } else {
        problem(i, `"${line}" is neither "## Unreleased" nor "## X.Y.Z (YYYY-MM-DD)"`);
      }
    } else if (line.startsWith("### ")) {
      item = null;
      const title = line.slice(4).trim();
      const last = section?.groups.at(-1);
      // As with sections: a group with an error is parsed on, detached.
      group = { title, items: [], line: i + 1 };
      if (!section) problem(i, `"${line}" before the first "## " section`);
      else if (!GROUPS.includes(title)) {
        problem(i, `unknown group "${title}": use ${GROUPS.map((g) => `"### ${g}"`).join(", ")}`);
      } else if (section.groups.some((g) => g.title === title)) problem(i, `a second "### ${title}" in one section`);
      else if (last && GROUPS.indexOf(title) < GROUPS.indexOf(last.title)) {
        problem(i, `"### ${title}" must come before "### ${last.title}" (order: ${GROUPS.join(", ")})`);
      } else section.groups.push(group);
    } else if (line === "-" || line.startsWith("- ") || /^[*+] /.test(line)) {
      item = null;
      if (group) group.attempted = true;
      const textOfItem = line.slice(1).trim();
      if (!group) problem(i, `an item outside a group: put it under ${GROUPS.map((g) => `"### ${g}"`).join(", ")}`);
      else if (!line.startsWith("-")) problem(i, 'items start with "- "');
      else if (!textOfItem) problem(i, "an empty item");
      else {
        group.items.push(textOfItem);
        item = group.items.length - 1;
      }
    } else if (/^ {2,}\S/.test(line)) {
      const more = line.trim();
      if (item === null) problem(i, "an indented line that continues no item (no blank lines inside an item)");
      else if (/^([-*+]|\d+[.)])\s/.test(more)) problem(i, "nested lists are not supported: make it an item of its own");
      else group.items[item] += ` ${more}`;
    } else if (/^#/.test(line)) {
      problem(i, `"${line}": only "## " version headings and "### " groups`);
    } else {
      problem(i, 'text outside an item: every change is a "- " item, wrapped lines indented by two spaces');
    }
  }
  for (const s of [unreleased, ...releases]) {
    for (const g of s?.groups ?? []) {
      if (!g.items.length && !g.attempted) problem(g.line - 1, `"### ${g.title}" has no items`);
    }
  }
  return { unreleased, releases, problems };
}

/** All items of a section, in order. */
export function sectionItems(section) {
  return section ? section.groups.flatMap((g) => g.items) : [];
}

/**
 * Turns "## Unreleased" into "## <version> (<date>)" and opens a new empty
 * "## Unreleased" above it; nothing else changes. Refuses a changelog that
 * breaks the grammar, has no or an empty Unreleased section, still has a
 * "TODO" item there, or already has <version> or a newer release.
 */
export function releaseChangelog(text, version, date) {
  if (!parseVersion(version)) throw new Error(`${version} is not X.Y.Z`);
  if (!isDate(date)) throw new Error(`${date} is not a date YYYY-MM-DD`);
  const { unreleased, releases, problems } = parseChangelog(text);
  if (problems.length) throw new Error(problems.join("\n"));
  if (!unreleased) throw new Error('CHANGELOG.md has no "## Unreleased" section');
  const items = sectionItems(unreleased);
  if (!items.length) throw new Error('nothing to release: "## Unreleased" in CHANGELOG.md lists no changes');
  const todo = items.filter((t) => /^TODO\b/.test(t));
  if (todo.length) {
    throw new Error(`"## Unreleased" still has placeholders:\n${todo.map((t) => `- ${t}`).join("\n")}`);
  }
  if (releases.length && compareVersions(version, releases[0].version) <= 0) {
    throw new Error(`CHANGELOG.md already has ${releases[0].version}, not older than ${version}`);
  }
  const eol = text.includes("\r\n") ? "\r\n" : "\n";
  const lines = text.split(/\r?\n/);
  lines.splice(unreleased.line - 1, 1, "## Unreleased", "", `## ${version} (${date})`);
  return lines.join(eol);
}

/** The body of a release's section (groups and items) as Markdown, e.g. for the GitHub release. */
export function releaseNotes(text, version) {
  const { releases } = parseChangelog(text);
  const release = releases.find((r) => r.version === String(version).replace(/^v/, ""));
  if (!release) throw new Error(`CHANGELOG.md has no section for ${version}`);
  const lines = text.split(/\r?\n/);
  const next = releases[releases.indexOf(release) + 1];
  const body = lines.slice(release.line, next ? next.line - 1 : lines.length);
  return `${body.join("\n").trim()}\n`;
}

// ---------------------------------------------------------------- TOML (just enough for Cargo files)

const TABLE_HEADER = /^\[\[?\s*([^\]]+?)\s*\]\]?\s*(#.*)?$/;

/** Line indexes [start, end) of the body of table `name`, or null. */
function tomlTable(lines, name) {
  let start = -1;
  for (let i = 0; i < lines.length; i++) {
    const m = TABLE_HEADER.exec(lines[i]);
    if (!m) continue;
    if (start >= 0) return [start, i];
    if (m[1] === name && !lines[i].startsWith("[[")) start = i + 1;
  }
  return start >= 0 ? [start, lines.length] : null;
}

/** A string value `key = "..."` of table `name`, or undefined. */
export function tomlString(text, name, key) {
  const lines = String(text ?? "").split(/\r?\n/);
  const range = tomlTable(lines, name);
  if (!range) return undefined;
  const re = new RegExp(`^\\s*${key}\\s*=\\s*"([^"]*)"`);
  for (let i = range[0]; i < range[1]; i++) {
    const m = re.exec(lines[i]);
    if (m) return m[1];
  }
  return undefined;
}

const INHERITED = /^\s*version\s*(\.\s*workspace\s*=\s*true|=\s*\{\s*workspace\s*=\s*true\s*\})/;

/** `version` of table `name`: the string, WORKSPACE when inherited, or undefined. */
export function tomlVersion(text, name) {
  const lines = String(text ?? "").split(/\r?\n/);
  const range = tomlTable(lines, name);
  if (!range) return undefined;
  for (let i = range[0]; i < range[1]; i++) {
    if (INHERITED.test(lines[i])) return WORKSPACE;
  }
  return tomlString(text, name, "version");
}

/** Sets the literal `version = "..."` of table `name`. */
export function setTomlVersion(text, name, version) {
  const lines = text.split(/\r?\n/);
  const range = tomlTable(lines, name);
  if (!range) throw new Error(`no [${name}] table`);
  for (let i = range[0]; i < range[1]; i++) {
    if (INHERITED.test(lines[i])) throw new Error(`[${name}] inherits its version from the workspace`);
    const m = /^(\s*version\s*=\s*")[^"]*(".*)$/.exec(lines[i]);
    if (m) {
      lines[i] = `${m[1]}${version}${m[2]}`;
      return lines.join(text.includes("\r\n") ? "\r\n" : "\n");
    }
  }
  throw new Error(`[${name}] has no version`);
}

/** The `members` of the root Cargo.toml's [workspace], as written (globs too). */
export function workspaceMembers(text) {
  const lines = String(text ?? "").split(/\r?\n/);
  const range = tomlTable(lines, "workspace");
  if (!range) return [];
  const body = lines
    .slice(...range)
    .map((l) => l.replace(/#.*$/, ""))
    .join("\n");
  const m = /^\s*members\s*=\s*\[([^\]]*)\]/m.exec(body);
  return m ? [...m[1].matchAll(/"([^"]+)"/g)].map((x) => x[1]) : [];
}

// ---------------------------------------------------------------- Cargo.lock

function lockPackages(lines) {
  const packages = [];
  let current = null;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (line.startsWith("[")) {
      current = line.trim() === "[[package]]" ? { name: undefined, versionLine: -1, source: false } : null;
      if (current) packages.push(current);
      continue;
    }
    if (!current) continue;
    let m;
    if ((m = /^name = "([^"]*)"/.exec(line))) current.name = m[1];
    else if (/^version = "/.test(line)) current.versionLine = i;
    else if (/^source = /.test(line)) current.source = true;
  }
  return packages;
}

/** Map of name to version of the workspace's own packages (no `source`) among `names`. */
export function cargoLockVersions(text, names) {
  const lines = String(text ?? "").split(/\r?\n/);
  const found = new Map();
  for (const p of lockPackages(lines)) {
    if (!p.source && names.includes(p.name) && p.versionLine >= 0 && !found.has(p.name)) {
      found.set(p.name, /^version = "([^"]*)"/.exec(lines[p.versionLine])[1]);
    }
  }
  return found;
}

/** Sets the version of the workspace packages `names` in Cargo.lock; registry and git packages stay. */
export function setCargoLockVersions(text, names, version) {
  const lines = text.split(/\r?\n/);
  const seen = new Set();
  for (const p of lockPackages(lines)) {
    if (p.source || !names.includes(p.name) || p.versionLine < 0) continue;
    lines[p.versionLine] = `version = "${version}"`;
    seen.add(p.name);
  }
  const missing = names.filter((n) => !seen.has(n));
  if (missing.length) {
    throw new Error(`Cargo.lock has no workspace package ${missing.join(", ")}: build once (cargo build) to add it`);
  }
  return lines.join(text.includes("\r\n") ? "\r\n" : "\n");
}

// ---------------------------------------------------------------- JSON

/**
 * Sets the string at each of `paths` (e.g. [["version"], ["packages", "", "version"]])
 * to `version`, changing only those characters: the first paths.length
 * `"version": "..."` pairs are replaced and the result is verified against
 * the parsed file, so formatting stays exactly as npm (or a person) left it.
 */
export function setJsonVersions(text, paths, version, file = "JSON") {
  const expected = JSON.parse(text);
  for (const path of paths) {
    let at = expected;
    for (const key of path.slice(0, -1)) at = at?.[key];
    if (typeof at?.[path.at(-1)] !== "string") throw new Error(`${file} has no ${path.join(".")}`);
    at[path.at(-1)] = version;
  }
  let left = paths.length;
  const out = text.replace(/("version"\s*:\s*")[^"]*(")/g, (all, a, b) => (left-- > 0 ? `${a}${version}${b}` : all));
  if (JSON.stringify(JSON.parse(out)) !== JSON.stringify(expected)) {
    throw new Error(`cannot set the version in ${file} safely: set ${paths.map((p) => p.join(".")).join(" and ")} by hand`);
  }
  return out;
}

/** tauri.conf.json's own version when it is a literal X.Y.Z, else undefined (absent, or a path to package.json). */
export function tauriConfVersion(text) {
  const v = JSON.parse(text).version;
  return typeof v === "string" && !v.endsWith(".json") ? v : undefined;
}

// ---------------------------------------------------------------- the repository

/** Reads every file that carries the version. A missing file is undefined. */
export function readRepo(root = ROOT) {
  const read = (path) => {
    try {
      return readFileSync(join(root, path), "utf8");
    } catch {
      return undefined;
    }
  };
  const files = {};
  for (const path of ["package.json", "package-lock.json", "Cargo.toml", "Cargo.lock", "CHANGELOG.md", TAURI_CONF]) {
    files[path] = read(path);
  }
  const members = [];
  for (const member of workspaceMembers(files["Cargo.toml"])) {
    if (member.endsWith("/*")) {
      const dir = member.slice(0, -2);
      const entries = existsSync(join(root, dir)) ? readdirSync(join(root, dir), { withFileTypes: true }) : [];
      for (const e of entries.filter((e) => e.isDirectory()).sort((a, b) => a.name.localeCompare(b.name))) {
        if (existsSync(join(root, dir, e.name, "Cargo.toml"))) members.push(`${dir}/${e.name}`);
      }
    } else {
      members.push(member.replace(/\/$/, ""));
    }
  }
  for (const m of members) files[`${m}/Cargo.toml`] = read(`${m}/Cargo.toml`);
  return { files, members };
}

/**
 * What is inconsistent: every version must equal package.json's, the
 * changelog must follow the grammar and its newest release must be that
 * version (or there is none yet). With a tag (v1.2.3): the tag must be that
 * version, the changelog must have its dated section, and "## Unreleased"
 * must be empty, as it is in the release commit.
 */
export function findProblems({ files, members }, tag) {
  const problems = [];
  const need = (path) => {
    if (files[path] === undefined) problems.push(`${path} is missing`);
    return files[path];
  };
  const json = (path) => {
    const text = need(path);
    if (text === undefined) return undefined;
    try {
      return JSON.parse(text);
    } catch (e) {
      problems.push(`${path}: ${e.message}`);
      return undefined;
    }
  };
  const found = []; // [where, version]

  const version = json("package.json")?.version;
  if (!parseVersion(version)) problems.push(`package.json: version ${version} is not X.Y.Z`);
  const lock = json("package-lock.json");
  if (lock) found.push(["package-lock.json", lock.version], ['package-lock.json packages[""]', lock.packages?.[""]?.version]);

  const cargo = need("Cargo.toml");
  if (cargo !== undefined) {
    const v = tomlVersion(cargo, "workspace.package");
    found.push(["Cargo.toml [workspace.package]", v === WORKSPACE ? undefined : v]);
  }
  const names = [];
  for (const m of members) {
    const path = `${m}/Cargo.toml`;
    const text = need(path);
    if (text === undefined) continue;
    const name = tomlString(text, "package", "name");
    if (!name) problems.push(`${path} has no [package] name`);
    else names.push(name);
    const v = tomlVersion(text, "package");
    if (v === undefined) problems.push(`${path} has no version: add version.workspace = true`);
    else if (v !== WORKSPACE) found.push([path, v]);
  }
  const cargoLock = need("Cargo.lock");
  if (cargoLock !== undefined) {
    const locked = cargoLockVersions(cargoLock, names);
    for (const name of names) {
      if (!locked.has(name)) problems.push(`Cargo.lock has no workspace package ${name} (cargo build adds it)`);
      else found.push([`Cargo.lock ${name}`, locked.get(name)]);
    }
  }
  if (files[TAURI_CONF] !== undefined) {
    try {
      const v = tauriConfVersion(files[TAURI_CONF]);
      if (v !== undefined) found.push([TAURI_CONF, v]);
    } catch (e) {
      problems.push(`${TAURI_CONF}: ${e.message}`);
    }
  }
  if (version) {
    for (const [where, v] of found) if (v !== version) problems.push(`${where} ${v} != package.json ${version}`);
  }

  const changelog = need("CHANGELOG.md");
  if (changelog !== undefined) {
    const { unreleased, releases, problems: grammar } = parseChangelog(changelog);
    problems.push(...grammar);
    const newest = releases[0];
    if (newest && newest.version !== version) {
      problems.push(`CHANGELOG.md: the newest release is ${newest.version}, not ${version}`);
    }
    for (const r of releases) {
      if (sectionItems(r).some((t) => /^TODO\b/.test(t))) problems.push(`CHANGELOG.md: ${r.version} has a TODO item`);
      if (!sectionItems(r).length) problems.push(`CHANGELOG.md: ${r.version} lists no changes`);
    }
    if (tag) {
      if (!newest) problems.push(`CHANGELOG.md has no "## ${version} (YYYY-MM-DD)" section for ${tag}`);
      if (sectionItems(unreleased).length) {
        problems.push(`CHANGELOG.md: "## Unreleased" is not empty, so ${tag} is not on its release commit`);
      }
    }
  }
  if (tag && tag.replace(/^v/, "") !== version) problems.push(`tag ${tag} != package.json ${version}`);
  return problems;
}

/** The changed files (path -> text) that set every version field to `version`. CHANGELOG.md is releaseChangelog's. */
export function applyVersion({ files, members }, version) {
  const out = {
    "package.json": setJsonVersions(files["package.json"], [["version"]], version, "package.json"),
    "package-lock.json": setJsonVersions(
      files["package-lock.json"],
      [["version"], ["packages", "", "version"]],
      version,
      "package-lock.json",
    ),
    "Cargo.toml": setTomlVersion(files["Cargo.toml"], "workspace.package", version),
  };
  const names = [];
  for (const m of members) {
    const path = `${m}/Cargo.toml`;
    names.push(tomlString(files[path], "package", "name"));
    if (tomlVersion(files[path], "package") !== WORKSPACE) out[path] = setTomlVersion(files[path], "package", version);
  }
  out["Cargo.lock"] = setCargoLockVersions(files["Cargo.lock"], names, version);
  if (files[TAURI_CONF] !== undefined && tauriConfVersion(files[TAURI_CONF]) !== undefined) {
    out[TAURI_CONF] = setJsonVersions(files[TAURI_CONF], [["version"]], version, TAURI_CONF);
  }
  return Object.fromEntries(Object.entries(out).filter(([path, text]) => text !== files[path]));
}

/** "Release vX.Y.Z", then the trailers ("Key: value") in a paragraph of their own. */
export function commitMessage(version, trailers = []) {
  for (const t of trailers) {
    if (!/^[A-Za-z0-9][A-Za-z0-9-]*: \S.*$/.test(t) || t.includes("\n")) {
      throw new Error(`trailer "${t}" is not "Key: value"`);
    }
  }
  return `Release v${version}\n${trailers.length ? `\n${trailers.join("\n")}\n` : ""}`;
}

// ---------------------------------------------------------------- command line

const USAGE = `usage: node scripts/release.mjs <major|minor|patch|X.Y.Z> [--dry-run] [--trailer "Key: value"]...
       node scripts/release.mjs --notes X.Y.Z
Through npm, put the arguments after --: npm run release -- minor --dry-run`;

export function parseArgs(argv) {
  const args = { spec: undefined, dryRun: false, trailers: [], notes: undefined, help: false };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    const value = (flag) => {
      if (a.startsWith(`${flag}=`)) return a.slice(flag.length + 1);
      if (i + 1 >= argv.length) throw new Error(`${flag} needs a value`);
      return argv[++i];
    };
    if (a === "--dry-run" || a === "-n") args.dryRun = true;
    else if (a === "--help" || a === "-h") args.help = true;
    else if (a === "--trailer" || a.startsWith("--trailer=")) args.trailers.push(value("--trailer"));
    else if (a === "--notes" || a.startsWith("--notes=")) args.notes = value("--notes");
    else if (a.startsWith("-")) throw new Error(`unknown option ${a}`);
    else if (args.spec === undefined) args.spec = a;
    else throw new Error(`one version or level only, got ${args.spec} and ${a}`);
  }
  return args;
}

function git(args, options = {}) {
  return execFileSync("git", args, { cwd: ROOT, encoding: "utf8", stdio: ["pipe", "pipe", "pipe"], ...options });
}

function gitSucceeds(args) {
  return spawnSync("git", args, { cwd: ROOT, stdio: "ignore" }).status === 0;
}

function main(argv) {
  // `npm run release minor --dry-run --trailer "A: b"` (no --) gives the flags
  // to npm, which sets npm_config_*: a lost --dry-run still runs dry, a lost
  // --trailer stops the release.
  if (process.env.npm_config_trailer !== undefined) {
    throw new Error(`npm took --trailer for itself; put the arguments after --:\n${USAGE}`);
  }
  const args = parseArgs(argv);
  if (process.env.npm_config_dry_run === "true") args.dryRun = true;
  if (args.help) {
    console.log(USAGE);
    return 0;
  }
  if (args.notes !== undefined) {
    process.stdout.write(releaseNotes(readFileSync(join(ROOT, "CHANGELOG.md"), "utf8"), args.notes));
    return 0;
  }
  if (args.spec === undefined) throw new Error(USAGE);

  const repo = readRepo();
  const before = findProblems(repo);
  if (before.length) throw new Error(`the versions are not consistent now; fix this first:\n${before.join("\n")}`);
  const current = JSON.parse(repo.files["package.json"]).version;
  const firstRelease = parseChangelog(repo.files["CHANGELOG.md"]).releases.length === 0;
  const version = nextVersion(current, args.spec, { firstRelease });
  const tag = `v${version}`;
  const date = localDate(new Date());
  const changed = {
    ...applyVersion(repo, version),
    "CHANGELOG.md": releaseChangelog(repo.files["CHANGELOG.md"], version, date),
  };
  const after = findProblems({ ...repo, files: { ...repo.files, ...changed } }, tag);
  if (after.length) throw new Error(`the release would leave inconsistent versions (a bug in this script):\n${after.join("\n")}`);
  const message = commitMessage(version, args.trailers);

  const refusals = [];
  const dirty = git(["status", "--porcelain"]).trimEnd();
  if (dirty) refusals.push(`the working tree is not clean; commit or stash first:\n${dirty}`);
  if (gitSucceeds(["rev-parse", "-q", "--verify", `refs/tags/${tag}`])) refusals.push(`the tag ${tag} exists already`);
  const branch = spawnSync("git", ["symbolic-ref", "-q", "--short", "HEAD"], { cwd: ROOT, encoding: "utf8" });
  if (branch.status !== 0) refusals.push("HEAD is detached: check out the branch to release from");
  if (refusals.length && !args.dryRun) throw new Error(refusals.join("\n"));

  console.log(`${current} -> ${version} (${date})${args.dryRun ? ", dry run" : ""}`);
  console.log(`files:  ${Object.keys(changed).join(", ")}`);
  console.log(`commit: ${message.trimEnd().replace(/\n/g, "\n        ")}`);
  console.log(`tag:    ${tag} (annotated)`);
  if (args.dryRun) {
    if (refusals.length) {
      console.error(`\nthe real run would refuse:\n${refusals.join("\n")}`);
      return 1;
    }
    console.log("\ndry run: nothing written");
    return 0;
  }

  for (const [path, text] of Object.entries(changed)) writeFileSync(join(ROOT, path), text);
  const check = spawnSync(process.execPath, [join(ROOT, "scripts", "check-version.mjs"), tag], {
    cwd: ROOT,
    stdio: "inherit",
  });
  if (check.status !== 0) {
    for (const path of Object.keys(changed)) writeFileSync(join(ROOT, path), repo.files[path]);
    throw new Error("check-version failed; the files are restored");
  }
  git(["add", "--", ...Object.keys(changed)]);
  git(["commit", "--quiet", "--file", "-"], { input: message, stdio: ["pipe", "inherit", "inherit"] });
  git(["tag", "--annotate", tag, "--message", `Release ${tag}`], { stdio: ["pipe", "inherit", "inherit"] });
  console.log(`\nCommitted and tagged ${tag}. Nothing is pushed. To publish, push both at once:
  git push --atomic origin ${branch.stdout.trim()} ${tag}
To undo before pushing:
  git tag -d ${tag} && git reset --keep HEAD~1`);
  return 0;
}

const invoked = process.argv[1] && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url));
if (invoked) {
  try {
    process.exitCode = main(process.argv.slice(2));
  } catch (e) {
    console.error(`release: ${e.message}`);
    process.exitCode = 1;
  }
}
