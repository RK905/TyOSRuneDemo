# Sustain

An open-source logistics workspace for understanding stock, anticipating shortages, and coordinating deliveries. **Sustain is a working name.**

## Project status

Planning and routing-engine prototype. The complete MVP is **not implemented**. This project is an independent implementation of general logistics workflows.

The current workspace contains a runnable Rust/Axum routing demo, a JavaScript reference server, a complete diagnostic UI, and a synthetic transport network. A workspace-local Rust toolchain is installed without changing system PATH. The routing demo is implemented; the broader logistics MVP described below remains proposed.

**Run the demo:** `.\scripts\rust.ps1 run` (workspace-local Rust), `cargo run` (normal Rust installation), or `npm start` (JavaScript reference). Open http://127.0.0.1:8080. See the [routing demo guide](docs/routing-demo.md) for scenarios, API details, screenshots, and validation. Rust compilation, 10 Rust tests, 13 JavaScript tests, and strict Clippy checks passed on October 12, 2026.

Verified on October 12, 2026: all 13 existing JavaScript routing tests pass:

```sh
node --test dev/engine.test.mjs
```

Node.js 20 or newer is required by the existing package. No npm dependencies are required for these tests. CI uses Node.js 24. Do not interpret passing routing tests as application readiness.

## The proposed MVP

A logistics coordinator sees stock at several locations, identifies an approaching shortage, prepares a feasible replenishment delivery, approves it, and records its receipt. Field staff can inspect cached records and queue observations while disconnected.

- Inventory ledger, locations, item catalog, and validated CSV import.
- Explainable consumption rates, coverage estimates, and shortage alerts.
- Vehicle-aware delivery planning with explicit route constraints.
- Approval, dispatch, receipt, and attributable event history.
- Cached views and offline capture with explicit reconciliation.
- A reproducible synthetic demo that needs no external services or AI keys.

## Start with the plan

| Document | Purpose |
| --- | --- |
| [Product and MVP](docs/PRODUCT.md) | Reference review, scope, workflows, screens, and acceptance criteria |
| [Architecture](docs/architecture.md) | Stack, data model, APIs, stock accounting, and offline behavior |
| [Roadmap](docs/ROADMAP.md) | Milestones, dependencies, estimates, and release gates |
| [GitHub project setup](docs/GITHUB_PROJECT.md) | Repository settings, project board, labels, and publication sequence |
| [Issue backlog](docs/BACKLOG.md) | Actionable work with dependencies and completion criteria |
| [Machine-readable backlog](planning/backlog.json) | Stable planning IDs and structured issue data |
| [Project configuration](planning/project.json) | Proposed milestones, labels, fields, and views |
| [Contributing](CONTRIBUTING.md) | Development and review expectations |

## Existing layout

```text
src/                 Rust API and routing implementation
dev/                 JavaScript reference server, engine, and tests
data/                Synthetic transport network
docs/                Product and engineering plans
planning/            GitHub issue and board definitions
.github/             Issue forms, PR template, and baseline CI
```

The target application layout is described in the architecture document; future directories are not presented here as implemented code.

## Open-source release

Apache-2.0 is the proposed license. The repository owner must establish provenance of the existing code and adopt a license before publishing the first open-source release. No license has been applied by this planning change. Use original branding, interfaces, documentation, and fixtures; reference product screenshots are not bundled.

No GitHub repository, issues, or Project board have been created by this planning pass. Local planning IDs such as `SUS-001` are not GitHub issue numbers.
