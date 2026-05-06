# Architecture Boundaries

This fork keeps Warp useful as a local-first terminal and agent workbench while
removing upstream SaaS dependencies in later phases. Keep changes inside the
owning boundary unless a plan explicitly says otherwise.

## Command Layer

- `make smoke`: cheap repo and app sanity check.
- `make check`: static checks before long tests.
- `make ci`: local equivalent of the required full validation path.
- `scripts/harness/*`: deterministic wrappers around existing Warp commands.

## Boundaries

- Terminal core and renderer: terminal model, panes, tabs, shell integration,
  rendering, input, and local terminal workflows. Do not remove or replace this
  while cutting cloud surfaces.
- WarpUI: custom UI framework, actions, themes, accessibility, and view/model
  handles. UI changes should reuse existing WarpUI patterns.
- Settings and local persistence: user preferences, local state, secure
  storage, and SQLite persistence. Fork-owned settings should be local-only.
- Server and cloud clients: Warp API, GraphQL, auth, workspaces, teams,
  managed objects, managed secrets, telemetry, update, Oz, and release clients.
  These are removal or adapter boundaries for later phases.
- AI and LLM routing: model metadata, request routing, provider settings, and
  credentials. Later phases should route through user-owned provider profiles,
  not Warp-hosted model services.
- Local CLI agents: Codex, Claude Code, OpenCode, Gemini-style CLIs, code
  review routing, agent panes, notifications, context chips, and handoff
  history. Preserve these workflows.
- Symphony local orchestration: future local issue-to-agent scheduler. It must
  load repo-owned `WORKFLOW.md`, use typed config, talk to Linear first, create
  deterministic per-issue workspaces, run bounded local agents, and expose
  operator-visible status and logs.
- Harness scripts: shell-only command wrappers and audits. They may call
  existing project scripts but should not mutate source during checks.

## Data Flow Rules

- Parse and validate external data at boundaries: tracker payloads, provider
  configs, environment variables, server responses, and workflow front matter.
- Keep secrets in local credential stores or explicit local references. Never
  log API keys, tokens, provider headers, prompts with secrets, or crash data.
- Treat any call path to `warp.dev`, `warpdotdev`, Oz, telemetry, crash
  reporting, release servers, billing, teams, or cloud sync as technical debt
  unless the user explicitly asks to keep it.

## Phase Notes

- Phase 0 changes only harness artifacts and docs.
- Phase 1 starts at upstream network roots and channel configuration.
- Phase 6 Symphony work must follow the upstream OpenAI Symphony service
  specification and remain independent of Warp cloud, Oz, auth, Drive, and
  hosted model routing.
