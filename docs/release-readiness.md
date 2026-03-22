# Release Readiness Checklist

Use this checklist before calling the desktop app release-ready on a local machine.

## Local setup

- `npm run check:prereqs` succeeds.
- `npm ci` succeeds from a clean checkout.
- `npm run lint` succeeds.
- `npm run build` succeeds.
- `cd src-tauri && cargo test --verbose` succeeds.
- `cd src-tauri && cargo clippy -- -D warnings` succeeds.
- `cd src-tauri && cargo audit --deny warnings` succeeds.
- `npm run tauri build` succeeds.

## Smoke walkthrough

- Complete the checklist in `docs/manual-smoke-walkthrough.md`.

## Known release gates

- No open P0 or P1 issues in install, launch, drafting, save/load, or publish flows.
- Any remaining P2 issues are documented and do not block the core workflow.
- Documentation matches the actual shortcuts, package manager, and supported integration scope.
- The live smoke report records the Jira ticket key, Confluence space, Ollama model, and exact failures if any step does not pass.
