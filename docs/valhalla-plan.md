# Valhalla milestone — proposed, not implemented

The current demo owns graph search and costing in Rust. To match the role's direct Valhalla requirement, the next artifact should be an actual tested C++ patch against a pinned Valhalla commit with a reproducible local tile fixture.

Valhalla separates runtime costing in Sif from path search in Thor. Its costing model distinguishes generalized path cost from elapsed time, which this demo intentionally keeps identical. See the official [dynamic costing design](https://valhalla.github.io/valhalla/sif/dynamic-costing/) and [Thor algorithms](https://valhalla.github.io/valhalla/thor/). Valhalla's existing multimodal algorithm combines pedestrian and transit; transporting a loaded truck over a scheduled ferry requires explicit vehicle/transfer semantics rather than assuming public-transit behavior transfers directly.

## Proposed deliverable

1. Pin a Valhalla release/commit and build a small OSM road fixture with a weight-limited bridge and an alternative road. Preserve data provenance and rebuild commands.
2. Establish truck-routing baseline requests with departure time and weight/height options using the pinned [route API](https://github.com/valhalla/valhalla/blob/master/docs/docs/api/route/api-reference.md). Compare restrictions against known fixture paths.
3. Add an immutable operational overlay indexed by directed graph edge identity, with effective interval, expiry, source, and revision. Resolve external road identifiers to both directed graph edges during ingestion. Never use raw demo edge names as Valhalla graph IDs. Graph-tile rebuilds must invalidate or remap overlays.
4. Extend appropriate access/costing hooks in the pinned C++ implementation to consume closure and weather overlays at predicted traversal times. Inspect that version's signatures before implementing: the design documentation is conceptual, not a stable patch API. Review [DynamicCost source](https://github.com/valhalla/valhalla/blob/master/src/sif/dynamiccost.cc).
5. Add regression tests for bridge restrictions, closure boundaries, missing/stale overlays, reproducibility, and time-dependent forward searches. Decide explicitly whether closures exclude edges or permit waiting; the Rust demo's wait-until-open behavior must not be presumed available in Valhalla.
6. Verify both elapsed-time ETAs and generalized cost. Benchmark expansion count and latency, including regression against default truck costing with no overlays.

For maritime/air transfers, introduce a higher-level transport graph whose ground legs are computed by Valhalla. Scheduled transfers need mode/vehicle state, transfer duration, capacity, and departure calendars. Route-level orchestration is useful but does not replace the direct engine patch needed to demonstrate Valhalla internals experience.

## Interview framing

“I built an original Rust reference engine to make time-dependent routing and transfer semantics testable. It models closures during traversal and treats hard truck limits separately from weather cost. Here is the code and the regression suite. It does not yet integrate with Valhalla; this is the concrete C++ integration milestone I would implement next.”
