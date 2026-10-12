# Security

The current code is a local routing prototype, not a supported production logistics service. No production release is supported yet.

Before publishing the repository, maintainers must enable GitHub private vulnerability reporting and document the supported release line. This is tracked in SUS-001 and SUS-025. Until that channel exists, do not post secrets or sensitive reproduction data in public issues; request a private channel from the repository owner.

The v0.1 release requires server-side permissions, secure sessions, request limits, dependency review, audit history, and verified backup/restore. See [the architecture plan](docs/architecture.md). These controls are planned, not claims about the present prototype.
