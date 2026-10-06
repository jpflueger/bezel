# Security policy

Bezel apps are untrusted code. The host and the DOM runtime are the trusted computing base. Anything that lets an app read files, reach hosts, observe input or run code outside what its `bezel.toml` declares is a security bug, as is anything that lets an app freeze or crash the host UI thread. See ADR-0007 and `docs/architecture` §10 for the threat model.

Report privately to the maintainer (see repository profile) with a minimal app that demonstrates the issue. Expect an acknowledgement within 72 hours and a fix or mitigation plan within 14 days for confirmed issues. Please allow a coordinated disclosure window of 30 days.

No usage telemetry is collected by hosts or the CLI.
