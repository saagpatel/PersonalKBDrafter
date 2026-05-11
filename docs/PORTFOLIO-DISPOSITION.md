# PersonalKBDrafter — Portfolio Disposition

**Status:** Smoke Frozen — the app builds and runs locally, but the
release-readiness checklist requires operator-provided live access to
real Jira and Confluence Data Center instances and a local Ollama
service. Do not surface for routine review until smoke is run.

---

## Why this file exists

Portfolio operating system has been surfacing PersonalKBDrafter as a
row needing review. The codebase is in a healthy, feature-complete
state for v0.1 — recent commits added Confluence base-URL
persistence, Jira ADF content parsing, and settings persistence — but
the row keeps cycling because the disposition is unclear.

This file resolves it.

---

## Why "Smoke Frozen" instead of "Release Frozen"

The other three frozen repos (DesktopPEt, ContentEngine, AIGCCore)
are **signing-frozen**: artifact exists, signing credentials are the
gate. They have similar shape so the operator can batch them through
signing in one session.

PersonalKBDrafter is different. The gate is **smoke validation on
real enterprise systems**:

- Local Ollama service running with a pulled model (e.g., `llama3.2`)
- Jira Data Center URL + PAT with access to a real ticket
- Confluence Data Center URL + PAT with access to a non-production
  space

Until those three are simultaneously available to the operator,
nobody can complete `docs/manual-smoke-walkthrough.md`. Until that
walkthrough is run, the release-readiness checklist
(`docs/release-readiness.md`) cannot be marked complete. The artifact
builds; nobody knows if it works against real systems.

That's a different shape of frozen — and a different unblock
trigger. It does not co-batch with Apple signing.

---

## Current state in one paragraph

PersonalKBDrafter is a Tauri 2 + Rust + React/TypeScript desktop app
for IT support / DevOps / technical writers who want to draft
Confluence knowledge-base articles from Jira tickets using a local
LLM. It pulls ticket content (including ADF-formatted comments and
descriptions), runs a local Ollama model to draft a KB article, and
publishes to Confluence. Credentials persist via Tauri-side storage.
The release-readiness checklist is captured. The manual smoke
walkthrough is captured. Both are unrun.

For full detail:
- `README.md`
- `docs/release-readiness.md`
- `docs/manual-smoke-walkthrough.md`

---

## Portfolio operating system instructions

| Aspect | Posture |
|---|---|
| Portfolio status | `Smoke Frozen` (subcategory of Release Frozen) |
| Review cadence | Suspend overdue counting |
| Resurface conditions | (a) operator gains simultaneous access to Ollama + Jira DC + Confluence DC test space, (b) operator decides to ship without real-system smoke (not currently planned — every release-readiness gate explicitly requires it), or (c) operator dependabot-clears the 5 open PRs (#8–#12) as a hygiene pass |
| Co-batch with | **Not** the signing-frozen cluster (DesktopPEt / ContentEngine / AIGCCore) — different unblock trigger |

---

## Unblock trigger (operator)

When the operator has all three integrations ready:

1. Start Ollama locally with a pulled model.
2. Confirm Jira DC PAT and Confluence DC PAT are valid against the
   chosen test ticket and space.
3. Run `npm run tauri dev` to launch the dev build.
4. Walk every step of `docs/manual-smoke-walkthrough.md`. Record the
   Jira ticket key, Confluence space, Ollama model, and outcome per
   the checklist.
5. If all steps pass, mark the release-readiness checklist complete
   and tag a release. If any step fails, the failure mode determines
   whether the gate is real (functional regression) or environmental
   (token scope, missing API endpoint on DC version).
6. Triage the 5 open dependabot PRs (#8–#12) — most are routine
   transitive bumps that can land together.

Estimated operator time once integrations are in hand: ~45 minutes
for the smoke walkthrough + ~30 minutes for the dependabot triage.

---

## Reactivation procedure (for the next code session)

When portfolio operating system flips this row to `Active`:

1. Note that `legacy-origin` remote points at `saagar210/PersonalKBDrafter`
   (the pre-migration GitHub account) and is preserved for historical
   reference. **Do not push to `legacy-origin`.** Use `origin`
   (`saagpatel/PersonalKBDrafter`) for everything.
2. Delete the stale `codex/*` branches that pre-date the most recent
   meaningful commits (`feb7c08`, `9e6c2bd`, `3cf39ef`). They are
   merged-history artifacts.
3. Re-run `npm ci && npm run lint && npm run build && cd src-tauri
   && cargo test --verbose && cargo clippy -- -D warnings` to
   confirm the toolchain still works after the freeze.
4. Only then proceed to the smoke walkthrough.

---

## Why Smoke Frozen is the right disposition

- **Active** — wrong. The product surface is complete; pushing more
  features now compounds the un-validated surface without addressing
  the validation gate.
- **Cold Storage** — wrong. The code works (recent commits prove
  it), and the readiness docs exist. Calling it "cold" misrepresents
  the state.
- **Archived / Wind-down** — wrong. The release-readiness checklist
  exists specifically because someone meant to ship; nothing has been
  abandoned.
- **Release Frozen (signing)** — wrong. Apple signing is not the
  gate; enterprise-system smoke validation is. Co-batching would
  send the operator looking for the wrong credentials.
- **Smoke Frozen** — correct. The gate is "run the smoke against
  real systems," not "run a packaging script." Distinct enough from
  signing-frozen to warrant its own label.

---

## Last known reference

| Field | Value |
|---|---|
| Last meaningful commit on `main` | `feb7c08` fix(confluence): persist base urls for reconnects |
| Default branch | `main` |
| Build verification status | green (per release-readiness.md gates) |
| Smoke validation status | **Not yet run** |
| Open dependabot PRs | #8 – #12 |
| Blocker | Simultaneous operator access to Ollama + Jira DC + Confluence DC test space |
| Migration note | `legacy-origin` points at the frozen `saagar210` account; do not push there |
