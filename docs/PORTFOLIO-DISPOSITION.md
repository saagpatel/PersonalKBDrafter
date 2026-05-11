# PersonalKBDrafter — Portfolio Disposition

**Status:** Active — the canonical `origin/main` (saagpatel) is the
working state. The product surface is real but minimal at HEAD;
substantive functional work that previously existed under the
`saagar210` account never propagated to canonical and lives only on
side branches. Disposition is **not** Release Frozen / Smoke Frozen
yet.

> **Audience:** anyone resuming PersonalKBDrafter work, or wondering
> why the prior version of this file cited commits that are not on
> `origin/main`.

---

## Correction notice

A previous version of this doc (shipped in PR #15) claimed:

- Recent meaningful commits `feb7c08`, `9e6c2bd`, `3cf39ef`
  (Confluence persistence, Jira ADF parsing, settings persistence)
  were on `main`.
- A `docs/release-readiness.md` and `docs/manual-smoke-walkthrough.md`
  defined the release gates.
- The repo was **Smoke Frozen** waiting on operator access to real
  Jira / Confluence / Ollama.

All three claims were based on reading the wrong remote.

The cited commits are on `legacy-origin/main` (the frozen `saagar210`
account's `main`), not on `origin/main` (`saagpatel`). The cited
docs are not on `origin/main` either — they exist only on the
`codex/chore/bootstrap-codex-os` feature branch. The "Smoke Frozen"
disposition assumed those release-gate docs were canonical; they're
not.

This file replaces the prior disposition. **The Smoke Frozen
label was wrong.** The repo is in an earlier state than that
disposition implied.

---

## What is actually on `origin/main`

Confirmed via `git ls-tree -r origin/main` and
`git log --oneline origin/main`:

- Tauri 2 + Rust + React/TypeScript source code, including
  `src-tauri/src/commands/articles.rs`, `confluence.rs`,
  `drafting.rs`, `jira.rs`, and friends.
- TypeScript bindings (`Article.ts`, `ArticleStatus.ts`,
  `FlaggedSection.ts`, `QualityScore.ts`, `Template.ts`).
- Standard portfolio chore scaffolding: `LICENSE`, `Makefile`,
  `CHANGELOG.md`, `.codex/`, `.github/`, `.husky/`, etc.
- `docs/PORTFOLIO-DISPOSITION.md` (this file, replacing the prior).
- **Not** present on `origin/main`: `docs/release-readiness.md`,
  `docs/manual-smoke-walkthrough.md`, or the Confluence-base-URL /
  Jira-ADF-parsing / settings-persistence functional commits cited
  in the prior disposition.

The product surface that exists on `origin/main` is real — it's a
working Tauri shell with Jira/Confluence command scaffolding — but
it's earlier than the prior disposition implied.

---

## What's on `legacy-origin/main` and the codex branches

`legacy-origin/main` is the frozen saagar210 account's `main`. It
diverged from `origin/main`:

- 3 codex bootstrap commits are on `legacy-origin/main` that are not
  on `origin/main`.
- The `codex/chore/bootstrap-codex-os` feature branch (which exists
  on both `legacy-origin` and `origin`) is where the
  `release-readiness.md` and `manual-smoke-walkthrough.md` docs
  live.
- The cited functional commits `feb7c08`, `9e6c2bd`, `3cf39ef` are
  on a saagar210-side branch and were never brought to canonical
  `origin/main`.

**Operator action recommended (not auto-resolved by Claude Code):**
audit what work exists on the legacy-origin side and decide whether
to bring it forward or accept the loss. Same pattern as the
FreeLanceInvoice correction — the saagar210 → saagpatel migration
may have left work behind in multiple repos.

---

## Why "Active" instead of other dispositions

- **Active** — correct. The canonical `origin/main` has a working
  product shell but is missing the polish work that gives it its
  release-candidate framing. The next move is operator decision-time:
  audit legacy-origin, bring forward what should land, then re-assess.
- **Release Frozen (signing)** — wrong. Apple signing is not the gate;
  the gate is "decide what should be on `main` first."
- **Smoke Frozen** — wrong. The smoke walkthrough doc isn't on `main`;
  the prior disposition's whole premise was misread.
- **Cold Storage / Archived** — wrong. The product surface is real and
  there's no decision to stop.

---

## Portfolio operating system instructions

| Aspect               | Posture                                                                                                                                                                  |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Portfolio status     | `Active`                                                                                                                                                                 |
| Critical follow-up   | Audit `legacy-origin/main` and the `codex/*` branches; decide what to bring forward to `origin/main`                                                                     |
| Review cadence       | Resume normal cadence — this row needs decision-time                                                                                                                     |
| Resurface conditions | (a) operator completes the legacy-origin audit and the canonical state on `origin/main` matches intent, (b) only then re-assess for Release Frozen / Smoke Frozen / etc. |
| Co-batch with        | The legacy-origin sweep itself, which may need to span multiple repos                                                                                                    |

---

## Why this mistake happened (general lesson)

The local clone of this repo had `main` tracking `legacy-origin/main`,
not `origin/main`. When the prior disposition analysis ran
`git log`, `git ls-tree`, etc., the implicit reference resolution
returned `legacy-origin/main` results, but the commands' output was
written into the doc as if it described `origin/main`. The
disposition's specific commit/file citations were therefore wrong,
even though the high-level product story was largely accurate.

This is the **second** disposition in this session affected by the
same legacy-origin / origin misread (the first was FreeLanceInvoice,
corrected in its own PR). Worth treating as a session-level pattern:

- Any repo with both `origin` and `legacy-origin` remotes is at
  risk of disposition mismatch.
- The fix in operator-side tooling is to **always use the literal
  `origin/<branch>` form** in disposition analysis.
- Verified-correct so far: Relay (commit `ab85e88` confirmed on
  `origin/main`).
- Still to verify: DeepTank, Nexus, OrbitForge stale dup, the
  stale IncidentWorkbench dup at `ITPRJsViaClaude/`. For each:
  check whether commits/files cited in their dispositions are
  actually on `origin/<branch>` rather than `legacy-origin/<branch>`.

---

## Last known reference

| Field                                         | Value                                                                                                                   |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Last commit on `origin/main`                  | `d49c1b1` docs: portfolio disposition + esbuild pnpm workspace fix (#15) — note this file replaces what that PR shipped |
| Functional code on `origin/main`              | Yes (Tauri shell + Jira/Confluence command scaffolding)                                                                 |
| Release-readiness docs on `origin/main`       | No (on codex branch only)                                                                                               |
| `legacy-origin/main` diverged                 | Yes — 3 codex bootstrap commits not yet on canonical                                                                    |
| Functional commits cited in prior disposition | On legacy-origin or codex branch, **not on canonical `origin/main`**                                                    |
| Migration note                                | Local clone may track `legacy-origin/main` by accident — verify with `git branch -vv` before relying on branch state    |
