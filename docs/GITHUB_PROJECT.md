# GitHub repository and Project setup

This is a publish-ready specification, not a claim that a remote repository or board exists. The workspace was not a Git checkout at assessment time. Do not overwrite the existing routing prototype when initializing it.

## Repository proposal

- Name: `sustain` (working proposal; availability not checked).
- Description: “Open-source logistics workspace for inventory, shortages, and delivery coordination.”
- Topics: `logistics`, `inventory`, `supply-chain`, `rust`, `offline-first`, `self-hosted`.
- Default branch: `main`; short-lived feature branches and squash merges.
- Visibility: public after license/provenance and secret review in SUS-001.
- License recommendation: Apache-2.0. Its official text includes copyright and patent grants; see [Apache's license and application instructions](https://www.apache.org/licenses/LICENSE-2.0). Apply the license to code only after confirming ownership and third-party notices.
- Enable Issues, Discussions, private security reporting, dependency alerts, and a repository Project where supported by the selected account.
- Add real maintainers to CODEOWNERS before release; no guessed account handles are included.

## Board definition

[project.json](../planning/project.json) specifies labels, milestones, custom fields, and views. It is our configuration format, **not a native GitHub Project import format**.

| Field | Type / values |
| --- | --- |
| Status | Backlog → Ready → In progress → In review → Done; Blocked when waiting on an identified dependency |
| Priority | P0 critical/prerequisite, P1 required MVP, P2 follow-up, P3 discovery |
| Area | inventory, delivery, forecast, routing, offline, web, security, and supporting areas |
| Planning ID | Text, e.g. SUS-015; preserve even after GitHub assigns an issue number |
| Estimate days | Number; focused engineer-days, not a due-date promise |
| Iteration | Two-week iterations, starting when implementation begins |
| Target release | v0.0.1, v0.1.0, v0.2.0, v0.3.0 |
| Milestone / Assignee | GitHub issue fields |

Use an MVP Kanban board, current-iteration table, milestone roadmap, blocked-work table, and later-work table. GitHub supports custom-length iteration fields and roadmap views; see [GitHub's iteration documentation](https://docs.github.com/en/issues/planning-and-tracking-with-projects/understanding-fields/about-iteration-fields). No fixed calendar dates are assigned until there is an implementation start date and an owner for each milestone.

Ready requires clear acceptance criteria and completed dependencies. In review requires a linked PR and evidence. Done requires merged code, passing relevant checks, and satisfied acceptance criteria. A closed issue alone must not imply a verified release.

## Prepared backlog

There are 29 issues: 25 for M0–M3, totaling 36 focused engineering days, and four later discovery/pilot issues. The 8–10 week schedule includes integration and contingency. Dependencies are stored by planning ID in [backlog.json](../planning/backlog.json); link them to actual GitHub issues after publication.

Generate individual Markdown issue bodies locally:

```sh
node scripts/check-planning.mjs
node scripts/export-issues.mjs
```

Output goes to `planning/generated/`. The exporter only writes local files; it makes no GitHub API calls. Rerunning it regenerates those files from the backlog. `manifest.json` contains titles, body filenames, label names, milestone titles, and dependency IDs.

For a selected repository, a maintainer can use each body with GitHub's new-issue UI or the GitHub CLI. For example, after replacing `OWNER/REPO` and creating labels/milestones:

```sh
gh issue create --repo OWNER/REPO --title "[SUS-001] Establish project identity and open-source release policy" --body-file planning/generated/SUS-001.md --label "area:foundation" --label "priority:P1" --milestone "M0 — Runnable foundation"
```

This command is an example, not executed. Do not bulk-run it twice without checking existing Planning IDs: GitHub issue creation is not intrinsically idempotent. Record resulting issue URLs/numbers against the stable IDs, then link dependency relationships.

## Publication sequence

1. Resolve owner/name and code provenance; adopt the chosen license, attribution, and maintainer policies.
2. Initialize Git in this workspace, inspect tracked files and ignores, and create the repository with the chosen visibility. Keep `.tools`, secrets, generated builds, and live databases out of commits.
3. Push the initial plan/prototype, create M0–M5 milestones and labels from `project.json`, then create issues from the generated bodies.
4. Create the Project and fields/views above, add issues, map Planning IDs to issue numbers, and assign milestone owners. Do not assign contributors without their agreement.
5. Configure a branch ruleset: require PR review, resolved conversations, and relevant CI; disallow force pushes/deletion on main. Availability depends on the selected repository/account.
6. Initially require `routing-reference` from the Baseline workflow. After SUS-004 lands, require the actual Rust/web/parity job names too. Do not require nonexistent jobs or mark future checks as already passing.
7. Configure auto-add for new repository issues and move merged-and-accepted work to Done. Keep product acceptance review explicit.
8. Create release notes from closed milestone work and the release checklist; tag v0.1 only when the MVP gate passes.

## Current automation coverage

The included workflow executes the existing JavaScript routing tests and validates planning data. It does not currently compile Rust, build a frontend, check deployment, or establish production security. SUS-004 adds those gates after the missing frontend build is repaired. Use pinned dependency lockfiles and review action versions/pins as part of that work.
