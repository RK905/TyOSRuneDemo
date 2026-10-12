# Issue backlog

These are local planning IDs, not published GitHub issue numbers. All work starts in Backlog. Estimates are focused engineering days, excluding integration contingency; null estimates mark post-MVP discovery. Source of truth: [backlog.json](../planning/backlog.json).

## SUS-001 — Establish project identity and open-source release policy

M0 · P1 · foundation · 0.5 days

Dependencies: none.

Record repository owner, original working name and provenance of existing code; adopt the chosen license with correct attribution.

Acceptance criteria:

- [ ] License and attribution committed before public release
- [ ] real maintainers assigned
- [ ] private security reporting and contribution policy documented.

## SUS-002 — Restore a runnable web application and routing demo

M0 · P0 · web · 2 days

Dependencies: none.

Introduce React/TypeScript build and replace missing web asset references in both development and Rust startup paths.

Acceptance criteria:

- [ ] Fresh checkout can build and open the UI
- [ ] bundled network renders
- [ ] route request and no-feasible-route errors display
- [ ] startup commands documented.

## SUS-003 — Add SQLite migrations and deterministic seed data

M0 · P0 · data · 1.5 days

Dependencies: none.

Create versioned schema with operation IDs, entity versions, actors, item units, ledger, reservations, deliveries, and audit.

Acceptance criteria:

- [ ] Empty database migrates and seeds repeatably
- [ ] synthetic fixture covers three stock sites/six items/three vehicles
- [ ] restart preserves data.

## SUS-004 — Expand CI to Rust, web, and route contract parity

M0 · P1 · quality · 1 days

Dependencies: SUS-002.

Add locked builds, cargo tests, frontend checks, and shared request/result fixtures for the Rust and JavaScript solvers.

Acceptance criteria:

- [ ] CI compiles the application and compares successful, invalid, closure, and ferry cases
- [ ] invalid graph tests included
- [ ] local commands documented.

## SUS-005 — Build catalog APIs and inventory workspace

M1 · P1 · inventory · 1.5 days

Dependencies: SUS-002, SUS-003.

Create location/item/vehicle forms and a searchable inventory table with explicit units and transport-node mapping.

Acceptance criteria:

- [ ] Create/edit validation works through API and UI
- [ ] duplicate SKU and unknown nodes fail clearly
- [ ] keyboard and empty states supported.

## SUS-006 — Implement transactional stock ledger and balances

M1 · P0 · inventory · 2 days

Dependencies: SUS-003.

Implement receipt, consumption, and compensating adjustment events with scaled quantities and materialized balances.

Acceptance criteria:

- [ ] Restart-safe balances equal ledger totals
- [ ] negative stock is rejected
- [ ] concurrent writes remain consistent
- [ ] adjustments are excluded from consumption rates.

## SUS-007 — Add stock observations and versioned reconciliation

M1 · P1 · inventory · 1 days

Dependencies: SUS-005, SUS-006.

Capture physical counts as observations and apply reviewed differences as explicit ledger adjustments.

Acceptance criteria:

- [ ] Count with stale balance version becomes conflict
- [ ] operator cannot silently overwrite stock
- [ ] coordinator correction records reason and actor.

## SUS-008 — Add CSV preview, validation, commit, and export

M1 · P1 · inventory · 1 days

Dependencies: SUS-005, SUS-006.

Support documented item/stock import templates and versioned previews.

Acceptance criteria:

- [ ] Invalid rows show row-level errors
- [ ] repeated commit is idempotent
- [ ] malformed units fail
- [ ] export neutralizes spreadsheet formula injection.

## SUS-009 — Implement users, sessions, and role enforcement

M1 · P0 · security · 2 days

Dependencies: SUS-003, SUS-002.

Add Viewer/Operator/Coordinator/Admin roles with secure sessions and permissions on all domain mutations.

Acceptance criteria:

- [ ] Viewer writes denied by API
- [ ] secure password hashing, CSRF defense, expiry and login limits verified
- [ ] demo mode explicit
- [ ] role changes invalidate unauthorized actions.

## SUS-010 — Expose audit history and freshness indicators

M1 · P1 · inventory · 1 days

Dependencies: SUS-006, SUS-009.

Write audit events in domain transactions and display entity timelines plus last-confirmed timestamps.

Acceptance criteria:

- [ ] Events show server actor/time/correlation
- [ ] failed writes do not appear successful
- [ ] stale/missing observation labels persist in UI.

## SUS-011 — Implement coverage and replenishment estimates

M2 · P0 · forecast · 1.5 days

Dependencies: SUS-006, SUS-005.

Calculate trailing usage, manual fallback, days of cover, reorder threshold, and target replenishment.

Acceptance criteria:

- [ ] Independent 120/40 example produces 3 days cover and 160 suggested cases
- [ ] zero/insufficient usage explicit
- [ ] units, rounding and inbound horizon tested.

## SUS-012 — Build shortage queue with traceable calculations

M2 · P1 · forecast · 1 days

Dependencies: SUS-011, SUS-010.

Rank shortage alerts and expose calculation inputs, policy settings, and freshness.

Acceptance criteria:

- [ ] User can inspect and edit rate/lead/safety/target policy by role
- [ ] changed policy recalculates
- [ ] no unsupported confidence percentage is shown.

## SUS-013 — Validate cargo and draft delivery proposals

M2 · P0 · delivery · 1.5 days

Dependencies: SUS-005, SUS-006, SUS-012, SUS-009.

Create one-source/one-destination drafts with line items and payload mass/volume totals.

Acceptance criteria:

- [ ] Unknown units or missing cargo conversion block approval
- [ ] vehicle payload/volume capacity checked
- [ ] route weight includes vehicle tare plus payload.

## SUS-014 — Connect route planning and scenario comparison

M2 · P1 · routing · 1.5 days

Dependencies: SUS-002, SUS-013.

Connect draft delivery to existing routing engine and display route, constraints, waits, revision, and ETA.

Acceptance criteria:

- [ ] Bridge closure/overweight/ferry scenarios update visibly
- [ ] impossible route prevents approval
- [ ] synthetic graph and ETA assumptions clearly labeled.

## SUS-015 — Approve deliveries with atomic reservations

M2 · P0 · delivery · 2 days

Dependencies: SUS-013, SUS-014, SUS-009, SUS-010.

Approve against current stock, graph, delivery, and vehicle versions in one transaction.

Acceptance criteria:

- [ ] Competing approvals cannot over-reserve stock or vehicle interval
- [ ] stale proposals conflict
- [ ] identical operation retry returns same result
- [ ] cancellation releases reservations.

## SUS-016 — Dispatch and receive deliveries with conserved quantities

M2 · P0 · delivery · 2 days

Dependencies: SUS-015.

Implement dispatch, partial receipts, completion, returns, and discrepancy reconciliation.

Acceptance criteria:

- [ ] Dispatch removes source stock/adds transit
- [ ] partial receipt adds only received amount
- [ ] retry/excess receipt blocked
- [ ] dispatched cancellation cannot erase inventory.

## SUS-017 — Build delivery execution and exception screens

M2 · P1 · web · 1.5 days

Dependencies: SUS-016, SUS-010.

Expose permitted transitions with role-aware controls and clear exception state.

Acceptance criteria:

- [ ] 120/40 replenishment scenario can be completed entirely through UI
- [ ] partial receipt and discrepancy visible
- [ ] rejected transitions show actionable errors.

## SUS-018 — Add browser app shell and bounded data cache

M3 · P1 · offline · 1.5 days

Dependencies: SUS-002, SUS-009, SUS-010.

Cache application assets and authorized snapshot in service worker/IndexedDB.

Acceptance criteria:

- [ ] Initialized client opens after disconnect/reload
- [ ] first-time offline state explained
- [ ] freshness visible
- [ ] logout/account switch isolates cached data
- [ ] quota errors handled.

## SUS-019 — Implement durable outbox and idempotent sync API

M3 · P0 · offline · 2 days

Dependencies: SUS-007, SUS-016, SUS-018.

Queue field observations/receipt claims and synchronize by stable IDs and server cursor.

Acceptance criteria:

- [ ] Disconnect/reload/reconnect accepts a claim once
- [ ] changed payload with reused ID conflicts
- [ ] interrupted acknowledgements replay safely
- [ ] per-operation errors retained.

## SUS-020 — Build sync status and conflict resolution workflow

M3 · P0 · offline · 1.5 days

Dependencies: SUS-019.

Show queued, accepted, rejected, and conflicting operations with permitted resolution actions.

Acceptance criteria:

- [ ] Two-device stale count requires reviewed adjustment
- [ ] expired auth blocks uploads
- [ ] manual retry works without Background Sync
- [ ] failed claims remain inspectable.

## SUS-021 — Verify the complete replenishment and outage scenario

M3 · P0 · quality · 2 days

Dependencies: SUS-017, SUS-020.

Add browser and API integration coverage for the complete demo and core failure paths.

Acceptance criteria:

- [ ] Clean fixture completes forecast-to-receipt
- [ ] concurrency, replay, partial receipts, graph changes, restart and offline conflict cases pass with recorded evidence.

## SUS-022 — Package deployment, migrations, backup, and restore

M3 · P1 · operations · 1.5 days

Dependencies: SUS-016, SUS-009.

Provide single-service container and persistent database volume with local quickstart and remote TLS guidance.

Acceptance criteria:

- [ ] Fresh install/seed works
- [ ] backup restores to separate instance
- [ ] upgrade migration tested
- [ ] live databases and secrets excluded
- [ ] readiness checks database.

## SUS-023 — Complete accessibility and performance review

M3 · P1 · web · 1 days

Dependencies: SUS-017, SUS-020.

Review keyboard operation, labels, contrast, narrow layouts and measured performance targets.

Acceptance criteria:

- [ ] Core loop usable with keyboard
- [ ] statuses not color-only
- [ ] agreed reference-machine measurements documented against PRODUCT targets
- [ ] regressions resolved.

## SUS-024 — Publish API and contributor documentation

M3 · P1 · docs · 1 days

Dependencies: SUS-019, SUS-022.

Document API schemas, units, state transitions, conflict responses, installation, demo, and contribution workflow.

Acceptance criteria:

- [ ] Examples work against running API
- [ ] fresh checkout instructions tested by another person where available
- [ ] limitations and supported browsers documented.

## SUS-025 — Prepare v0.1 release and security gate

M3 · P0 · security · 1 days

Dependencies: SUS-001, SUS-004, SUS-021, SUS-022, SUS-023, SUS-024.

Audit dependencies and secrets, verify release checklist, tag source, package build outputs and changelog.

Acceptance criteria:

- [ ] No unresolved P0/P1 data-integrity/core defects
- [ ] license and security reporting active
- [ ] acceptance evidence attached
- [ ] release notes match implemented features.

## SUS-026 — Prototype optional read-only logistics assistant

M4 · P2 · assistant · TBD days

Dependencies: SUS-025.

Design provider-neutral assistant over authorized inventory/forecast/delivery reads, disabled by default.

Acceptance criteria:

- [ ] Answers cite data and timestamps
- [ ] tool allowlist read-only
- [ ] unknown data stated
- [ ] offline/unconfigured mode still usable
- [ ] evaluation fixtures reject fabricated stock values.

## SUS-027 — Pilot CSV/API integration and barcode capture

M4 · P2 · integration · TBD days

Dependencies: SUS-025.

Choose one real pilot workflow and one adapter; explore camera/barcode or external stock feed with fixture-backed contracts.

Acceptance criteria:

- [ ] Adapter boundaries documented
- [ ] duplicate/late events tested
- [ ] user feedback collected
- [ ] no unreviewed quantity overwrite.

## SUS-028 — Evaluate real-road maps and routing data

M4 · P2 · routing · TBD days

Dependencies: SUS-025.

Assess map/routing providers, data licenses, offline availability and vehicle-limit completeness using representative pilot routes.

Acceptance criteria:

- [ ] Record coverage/cost/license matrix and benchmark
- [ ] distinguish display maps from routing graph
- [ ] retain synthetic demo without external dependencies.

## SUS-029 — Design multi-stop fleet planning and improved forecasts

M5 · P3 · research · TBD days

Dependencies: SUS-027, SUS-028.

Use pilot data to scope time windows, stop sequencing, fleet scheduling, seasonal demand and database scaling.

Acceptance criteria:

- [ ] ADR compares solver and storage alternatives with benchmarks
- [ ] forecast evaluation uses held-out history
- [ ] implementation tickets estimated before commitment.
