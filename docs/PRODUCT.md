# Product and MVP plan

Status: proposed implementation plan, October 12, 2026. Audience selected by the project owner: general logistics. Deliverable for this pass: planning and GitHub-ready project files.

## Product concept

Sustain combines supply visibility, demand forecasting, distribution planning, and decision support in one workspace. It is designed to remain useful during intermittent connectivity, with cached information and synchronization when connectivity returns.

The core workflow connects stock observations, anticipated needs, and executable deliveries. Algorithm quality, synchronization guarantees, and performance must be established through implementation and testing.

The design below is an independent proposal with original naming and implementation.

## Positioning

**Sustain helps small logistics teams answer: what do we have, where will it run out, and which delivery should we approve next?**

Initial user: a coordinator managing 3–20 depots, warehouses, or service sites and a small delivery fleet. Secondary users: a site operator recording stock and receipts, and a viewer checking status. Start with a single organization per deployment and synthetic demonstration data.

The value is an explainable, self-hostable workflow that remains useful with intermittent connectivity. Success is completing the entire replenishment loop with auditable quantities, rather than offering many disconnected dashboard features.

## Scope boundary

| Capability | v0.1 MVP | Later |
| --- | --- | --- |
| Inventory | Item/location catalog, ledger, balances, receipts, consumption, reconciliation | Lots, serial numbers, expiry, procurement, ERP connectors |
| Forecast | Explicit daily usage and historical moving average, days of cover, reorder threshold | Seasonality, probabilistic models, model evaluation pipeline |
| Deliveries | One source and destination per delivery, one vehicle, capacity checks, approval and receipt | Multi-stop optimization, fleet scheduling, carrier integrations |
| Network | Bundled synthetic graph, closures, limits, explainable ETA | Real road data, geocoding, live traffic, large-area maps |
| Offline | Cached records; durable queue for observations and receipt claims | Peer-to-peer sync, multiple writable servers, full offline planning |
| Collaboration | Viewer, operator, coordinator, administrator; server audit history | Multi-organization tenancy, enterprise identity integrations |
| AI | No model required; deterministic explanations | Optional read-only assistant with evidence-linked answers |
| Integrations | CSV import/export and documented HTTP API | Barcode scans, telemetry, webhooks, device SDK |

Exclude hardware manufacturing, autonomous vehicle control, personnel tracking, clinical workflows, and procurement/payment execution from this product roadmap. They do not contribute to the chosen general-logistics MVP.

## The demonstration story

Use a seeded scenario with three stock locations, six items, and three vehicles on the existing synthetic transport graph. Graph junctions are transport nodes, not necessarily inventory locations.

1. Open the overview and select a site with 120 cases of water and expected usage of 40 cases/day.
2. Show 3 days of cover. A 2-day lead time plus 2-day safety buffer produces a 160-case reorder point, so a shortage alert appears.
3. Target 7 days of coverage: suggest 160 cases to reach 280 total, subject to source availability and vehicle capacity. Display the assumptions and timestamp.
4. Select an available vehicle, calculate a feasible route, then introduce a bridge closure and compare the changed route and ETA.
5. Approve a versioned delivery. Reserve stock at the source; do not mark it as delivered.
6. Dispatch it. Move reserved stock into in-transit stock. An operator records a partial receipt of 150 cases, leaving 10 in transit until received or explicitly reconciled.
7. Disconnect an already initialized field client, capture a stock observation, reload, reconnect, and reconcile once. Show pending, confirmed, or conflict status throughout.
8. Inspect the audit trail to explain who recorded, approved, dispatched, and received the delivery.

## Screens and interaction model

| Screen | Main content | Primary action |
| --- | --- | --- |
| Overview | Coverage alerts, delivery exceptions, freshness indicators, network view | Investigate a shortage |
| Inventory | Location/item table, on-hand/reserved/available/in-transit, movement history | Record receipt or consumption |
| Replenishment | Ranked alerts with usage window, lead time, target, source candidates | Draft delivery |
| Delivery planner | Cargo, vehicle, route diagram, constraint exclusions, ETA breakdown | Submit for approval |
| Deliveries | Draft/approved/dispatched/partially received/completed/cancelled | Record permitted next event |
| Activity and sync | Audit events, pending local operations, rejected items, conflicts | Retry or resolve |

Use an original, restrained dashboard: neutral surfaces, legible tables, a clear route diagram, and a details panel. Labels accompany status colors. Keyboard access, meaningful empty/error states, and visible timestamps are release requirements. The first map is a diagram of the synthetic graph, never a claimed navigation map.

## Quantitative product rules

- Base quantity units are explicit per item. No implicit conversion between cases, kilograms, and liters.
- Available stock = on-hand minus reserved. In-transit belongs to a delivery and is shown separately.
- Coverage = available quantity / positive daily consumption. Zero usage or insufficient history produces “unknown/no observed use,” never a misleading numeric infinity.
- Historical usage is a trailing 7 complete UTC-day average of accepted consumption events; use a manually entered daily rate when history is insufficient and label the fallback. Store adjustments separately from consumption.
- Reorder threshold = daily consumption × (lead days + safety days).
- Suggested replenishment = max(0, daily consumption × target days − available − confirmed inbound due within the planning horizon). Show expected inbound and its ETA separately; do not increase on-hand before receipt.
- A forecast is a planning estimate. Show observation age, calculation time, data window, and all input assumptions.
- Delivery ETA and cargo feasibility are separate checks. Existing route `weight_tonnes` represents gross vehicle weight; payload adds to vehicle tare weight. Validate payload mass and volume against vehicle capacity before routing.

## MVP acceptance gates

1. A contributor follows the quickstart from a fresh checkout and runs the seeded application without paid accounts or external APIs.
2. The full demonstration above works through the UI and is covered by an end-to-end test.
3. Restarting the service preserves accepted events, delivery states, and audit history.
4. Concurrent approval cannot reserve the same remaining stock twice; identical retry IDs cannot duplicate receipts.
5. Cached views and queued capture survive a browser reload after successful initialization; stale data and conflicts remain visible. Empty first-time offline use has a clear unavailable state.
6. Viewer writes and unauthorized state transitions fail server-side. Test mode does not silently bypass authorization in normal operation.
7. Invalid CSV rows receive row-level feedback and do not partially modify stock unless the user explicitly chooses a validated subset.
8. Proposed performance targets: local API read p95 under 300 ms on a documented reference machine with 10,000 stock events; route calculation under 1 second on the bundled network; initial local UI usable within 3 seconds. Record measured results before claiming them.
9. All release-blocking issues are closed, required CI passes, and backup/restore is demonstrated.

## Decisions and assumptions

Recommended defaults: working name Sustain; Rust service retained; React/TypeScript client; SQLite storage; Apache-2.0 proposed license; one deployment/organization. Final repository owner/name, copyright holder, hosting target, and maintainer identities remain publication decisions. None prevents implementing the local MVP.
