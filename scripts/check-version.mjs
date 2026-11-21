// Verify that every version in the repository agrees: package.json,
// package-lock.json, the workspace (core, service, CLI), src-tauri/Cargo.toml,
// the workspace's packages in Cargo.lock, tauri.conf.json if it has one, and
// the newest CHANGELOG.md release, whose grammar the app relies on
// (docs/versioning.md). With a tag, also that the tag is that version and
// "## Unreleased" is empty, which holds only on the release commit.
// Usage: node scripts/check-version.mjs [v1.2.3]
import { findProblems, parseChangelog, readRepo } from "./release.mjs";

// In GitHub Actions the ref is a tag only for releases; a branch name is no version.
const fromEnv = process.env.GITHUB_REF_TYPE === "tag" ? process.env.GITHUB_REF_NAME : undefined;
const tag = process.argv[2] ?? fromEnv;

const repo = readRepo();
const problems = findProblems(repo, tag);
if (problems.length) {
  console.error(problems.join("\n"));
  process.exit(1);
}
const version = JSON.parse(repo.files["package.json"]).version;
const released = parseChangelog(repo.files["CHANGELOG.md"]).releases.length > 0;
console.log(`version ${version} OK${tag ? `, tag ${tag}` : released ? "" : ", not released yet"}`);
