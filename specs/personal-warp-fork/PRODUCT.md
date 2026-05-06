# Personal Warp Fork Revision - Product Spec

## Goal

Turn this Warp fork into a solo-developer, local-first terminal and agent
workbench with no dependency on upstream `warp.dev` services. The fork should
keep the strongest local terminal and CLI-agent workflows while removing
commercial SaaS coupling: upstream auth, telemetry, metrics, crash reporting,
billing, team administration, managed cloud features, Warp-hosted AI, and
Warp Drive/cloud sync.

This revision also adds a local implementation of OpenAI Symphony, following
the upstream service specification:
https://github.com/openai/symphony/blob/main/SPEC.md.

The repo must also be converted to follow the Harness Engineering Playbook:
`/Users/carl/.codex/skills/harness-engineering-playbook/SKILL.md`.

## Target User

Carl is the only target user for this revision. Optimize for daily use across
local repos, Codex, Claude Code, OpenCode, GPT Pro, GitHub, Vibe Kanban,
Taskmaster, Slack, Google Calendar, MCP tools, and long-running agent work.

## Product Principles

1. Local-first is the default. The app can use network services only when the
   user configured that service explicitly.
2. No silent upstream dependency. A clean run must not contact `warp.dev`,
   `warpdotdev`, Warp release servers, Warp auth, Warp telemetry, Warp crash
   reporting, Warp Drive, Oz, or Warp model routing.
3. AI is user-owned. Built-in AI routes to locally stored provider profiles,
   not a Warp-hosted model proxy.
4. Solo-dev beats team support. Remove team, billing, enterprise policy,
   managed workspace, invite, and shared-object UX unless a local replacement
   is explicitly needed.
5. Preserve terminal quality. Do not break panes, tabs, shell integration,
   code review, local CLI-agent panes, themes, accessibility, or local settings
   while removing commercial features.
6. Agent runs should be reproducible. Hard setup, smoke, lint, typecheck, test,
   audit, and CI flows should be one command away and documented in repo-local
   harness artifacts.

## Required Behavior

1. First launch does not require sign-in, account creation, a team, a workspace,
   or a Warp API key.
2. The default app state has telemetry, metrics, crash reporting, usage
   tracking, and cloud conversation storage disabled or removed.
3. The app exposes AI provider profiles in local settings. A profile includes
   provider name, endpoint/base URL, API key or local credential reference,
   model id, and optional compatibility/headers.
4. API keys and provider credentials are stored locally only. They are never
   synced, uploaded, logged, or included in telemetry/crash data.
5. The first AI-provider wire target is OpenAI-compatible HTTP. The design must
   allow non-OpenAI providers later without rewriting all agent UI.
6. Model selection shows user-configured/local models without upgrade prompts,
   team policy gates, request quotas, or Warp billing copy.
7. Local CLI-agent workflows remain first class: Codex, Claude Code, OpenCode,
   Gemini-style CLIs, code review comment routing, agent panes, notifications,
   context chips, and handoff history.
8. Cloud/Oz/ambient-agent features are either removed from the UI or replaced
   by local-only equivalents. Any remaining cloud-agent string or action must
   be visibly unreachable or explicitly marked as follow-up debt.
9. Warp Drive, cloud sync, shared sessions, team sharing, invites, and remote
   object permissions are removed from default navigation and settings.
10. Documentation and packaging no longer point users to upstream Warp docs,
    Slack, careers, release repos, support addresses, billing pages, or Oz
    platform material as if they are part of this fork.
11. The app supports a Symphony-compatible local orchestration mode for coding
    work. It reads repo-owned `WORKFLOW.md`, polls Linear or another configured
    issue tracker, creates isolated per-issue workspaces, runs bounded
    concurrent coding-agent sessions, handles retries/reconciliation, and shows
    operator-visible status/logs.
12. Symphony integration is local-first. It must not depend on Warp cloud, Oz,
    Warp Drive, Warp auth, or upstream Warp model routing.
13. The repository includes Harness Engineering artifacts: `PLANS.md`,
    `docs/ARCHITECTURE.md`, `docs/OBSERVABILITY.md`, `Makefile.harness`,
    deterministic `scripts/harness/` command wrappers, harness audit tooling,
    and CI coverage for the harness.
14. `make smoke`, `make check`, and `make ci` are stable entrypoints. They may
    call existing Warp scripts internally, but agents should not need to know
    hidden setup steps.

## Out Of Scope For The First Pass

- Rebranding every visual asset and product string.
- Rewriting the terminal emulator, renderer, shell integration, panes, tabs, or
  core WarpUI architecture.
- Building every future "uniquely Carl" QoL feature. The first pass should
  preserve hooks and document follow-ups.
- Providing a hosted replacement for Warp services. Prefer local removal or
  local configuration over building new servers.
- Extending Symphony beyond the upstream spec's required behavior. Start with a
  faithful local implementation before adding richer Warp-native UI.
- Perfecting the final harness in one pass. The first pass should create the
  control profile and close blocking audit gaps; later entropy work can refine
  coverage.

## Acceptance Criteria

- `rg -n "warp\\.dev|warpdotdev|oz\\.warp\\.dev|app\\.warp\\.dev|releases\\.warp\\.dev"` returns only documented historical references, test fixtures that are explicitly justified, or follow-up TODOs.
- A clean app run can be observed without unexpected network calls to upstream
  Warp domains.
- AI requests use the selected local provider profile and local credential
  source.
- The app can be used as a normal terminal without signing in.
- Team/billing/cloud settings do not appear in primary settings/navigation.
- Local CLI-agent workflows still work after commercial features are removed.
- Symphony can run one issue through the local workflow loop from tracker
  eligibility to per-issue workspace creation and agent handoff/status.
- Harness audit passes, or any remaining gaps are documented in `PLANS.md` with
  exact owners and commands. `make smoke`, `make check`, and `make ci` exist and
  fail deterministically when prerequisites are missing.
