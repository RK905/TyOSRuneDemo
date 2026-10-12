# Contributing

This project is in its planning/prototype phase. Read [the MVP scope](docs/PRODUCT.md) and [backlog](docs/BACKLOG.md) before starting implementation. The release license and maintainer identities are still to be established in SUS-001.

## Workflow

1. Select an issue, clarify its acceptance criteria, and identify dependencies.
2. Use a short branch such as `feat/SUS-006-inventory-ledger` or `fix/route-validation`.
3. Keep changes focused. Include tests for stock accounting, authorization, idempotency, routing semantics, and other meaningful behavior changes.
4. Open a pull request using the template; attach verification evidence and UI screenshots when relevant.
5. Obtain a maintainer review before merging. Use squash merges and delete merged branches.

Current baseline verification:

```sh
node --test dev/engine.test.mjs
```

Rust compilation and app startup currently require the missing web asset/build work. Do not report those checks as passing until verified. M0 introduces the full build/test commands and lockfiles.

## Implementation standards

- Keep domain calculations out of presentation components.
- Document quantity units, timezone behavior, and operation retry semantics.
- Require database transactions for stock and delivery state transitions.
- Preserve compatibility deliberately; document schema migrations and API changes.
- Use original implementation and assets. Record sources and licenses for third-party material.
- Use synthetic examples in issues, screenshots, and fixtures.
- Never place credentials or user data in test snapshots or logs.

## Maintainer setup

Before the first public release, establish the repository license, governance, code of conduct, private security reporting channel, and actual CODEOWNERS entries. Do not substitute invented usernames or unmonitored contact addresses.
