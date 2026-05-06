# Personal Warp Fork Agent Guide

This repository is now a personal fork of Warp. Treat it as a solo-dev,
local-first terminal and agent workbench, not as a commercial SaaS client.

For implementation handoff on this fork revision, read
`specs/personal-warp-fork/PRODUCT.md` and
`specs/personal-warp-fork/TECH.md` before changing code. `AGENTS.md` defines
the philosophy and operating rules; the specs define the concrete target state,
phase order, module hotspots, and acceptance checks.

This fork should also implement OpenAI Symphony in accordance with the upstream
specification: https://github.com/openai/symphony/blob/main/SPEC.md. Treat
Symphony as the local issue-to-agent orchestration layer for long-running solo
development work, not as an upstream Warp/Oz cloud feature.

Convert this repository to follow the Harness Engineering Playbook at
`/Users/carl/.codex/skills/harness-engineering-playbook/SKILL.md`. The fork
should have deterministic agent-run entrypoints, compact command-first docs,
clear architecture boundaries, local observability, `PLANS.md` for durable
multi-step work, repo-local harness scripts, static checks, and entropy audits.

## Fork Philosophy

- The product is for one developer first. Optimize for Carl's daily terminal,
  repo, AI-agent, and local-automation workflows before broad multi-user needs.
- Prefer local-first behavior, inspectable state, transparent settings, and
  user-owned data. Network access should be explicit, understandable, and easy
  to disable.
- Assume there should be no upstream dependency on `warp.dev` for auth, model
  routing, telemetry, metrics, updates, billing, teams, cloud sync, or product
  entitlement. Any remaining upstream call path is technical debt unless the
  user explicitly asks to keep it.
- Commercial/team surfaces are removal targets: billing, plans, enterprise
  policy, team invites, shared workspaces, managed seats, Warp Drive cloud
  storage, Oz/cloud-agent infrastructure, and upstream account workflows.
- Preserve the parts that make the app valuable as a terminal: speed, panes,
  tabs, command workflows, code review ergonomics, CLI-agent integration,
  keyboard-first navigation, themes, accessibility, and local persistence.

## Implementation Priorities

1. Remove outbound upstream connections and data exhaust first. Search for and
   remove or neutralize `warp.dev`, `warpdotdev`, telemetry, analytics, metrics,
   crash reporting, Sentry, Segment/Amplitude-style reporting, remote config,
   entitlement checks, and update/download endpoints before polishing features.
2. Remove commercial collaboration layers next. Team, billing, enterprise,
   cloud sync, managed workspace, invite, shared object, and Warp Drive code
   should not be preserved for compatibility unless the user explicitly says so.
3. Replace Warp-hosted AI assumptions with local/user-owned configuration.
   Built-in AI should route to an external provider selected by the user, not a
   Warp proxy or upstream model service.
4. Keep CLI-agent workflows strong. Claude Code, Codex, OpenCode, Gemini-style
   local agents, code review comment routing, agent panes, notifications, and
   task handoff history are core surfaces for this fork.
5. Favor small, verifiable cuts. When ripping out a commercial subsystem, leave
   the app compiling and keep behavior local by default rather than replacing
   everything in one risky sweep.
6. Before large implementation passes, install or adapt the Harness Engineering
   artifacts: `PLANS.md`, `docs/ARCHITECTURE.md`, `docs/OBSERVABILITY.md`,
   `Makefile.harness`, `scripts/harness/*.sh`, `scripts/audit_harness.sh`,
   and CI checks for `make smoke`, `make check`, and `make ci`.

## AI Provider Direction

- Remove Warp-hosted model provider assumptions and any model proxying through
  upstream Warp services.
- Add user-configured provider profiles with at least:
  - provider name
  - endpoint/base URL
  - API key or local credential reference
  - model id
  - optional provider-specific headers or compatibility mode
- Default the first implementation target to OpenAI-compatible HTTP APIs because
  that covers OpenAI, many local servers, NanoGPT-style gateways, and common
  proxy providers. Keep the abstraction flexible enough for Anthropic and other
  non-OpenAI wire shapes.
- Store secrets locally only. Do not sync provider keys, model history, prompts,
  usage, or credentials to cloud services.
- Provider settings should be visible and editable in the app. Avoid hidden
  magic defaults that silently reintroduce upstream Warp services.

## Uniquely Carl Product Ideas

Use these as future-work guidance when looking for high-leverage improvements:

- Build an agent command center that understands Codex, Claude Code, OpenCode,
  GPT Pro, Vibe Kanban, Taskmaster, GitHub, Slack, and Google Calendar as first
  class tools in the terminal workflow.
- Implement Symphony-compatible issue orchestration: repo-owned `WORKFLOW.md`,
  Linear issue polling, bounded concurrent agent runs, isolated per-issue
  workspaces, retries/reconciliation, and an operator-visible status surface.
- Add the Harness Engineering command layer so repeated autonomous runs have
  stable setup, smoke, lint, typecheck, test, audit, and CI paths.
- Add a local MCP/tool health panel: configured servers, reachable status,
  auth state, recent failures, available tools, and quick links to repair
  common config issues.
- Add a provider/model switcher that can jump between OpenAI, NanoGPT,
  Anthropic, local OpenAI-compatible servers, and per-repo defaults without
  digging through settings.
- Add a prompt/profile library for repeated workflows: code review, PR cleanup,
  issue triage, landing-page critique, GTM research, market scans, repo memory
  refresh, and implementation planning.
- Surface repo memory shortcuts: recent project context, known commands,
  prior decisions, accepted architecture docs, and "read this first" files.
- Improve heavy-agent accessibility: low-noise attention inbox, keyboard-first
  notification triage, clear blocked/running/done states, reduced visual churn,
  and stable panes for long-running agent work.
- Make handoffs explicit. A task should show what agent/tool touched it, what
  files changed, what commands ran, what remains unresolved, and where to resume.

## Repo Operating Rules

<!-- codebase-memory-mcp:start -->
# Codebase Knowledge Graph (codebase-memory-mcp)

This project uses codebase-memory-mcp to maintain a knowledge graph of the
codebase. ALWAYS prefer MCP graph tools over grep/glob/file-search for code
discovery when the graph has this checkout indexed.

## Priority Order

1. `search_graph` - find functions, classes, routes, variables by pattern
2. `trace_path` - trace who calls a function or what it calls
3. `get_code_snippet` - read specific function/class source code
4. `query_graph` - run Cypher queries for complex patterns
5. `get_architecture` - high-level project summary

## When to fall back to grep/glob

- Searching for string literals, error messages, URLs, telemetry names, config
  values, or docs
- Searching non-code files such as scripts, manifests, packaging templates,
  Markdown, TOML, YAML, JSON, GraphQL schemas, and generated specs
- When MCP tools do not have this checkout indexed or return insufficient
  results

## Examples

- Find a handler: `search_graph(name_pattern=".*OrderHandler.*")`
- Who calls it: `trace_path(function_name="OrderHandler", direction="inbound")`
- Read source: `get_code_snippet(qualified_name="pkg/orders.OrderHandler")`
<!-- codebase-memory-mcp:end -->

@/Users/carl/.codex/RTK.md

- Prefix shell commands with `rtk` whenever possible.
- For sufficiently large work, look for safe opportunities to parallelize.
- When using `spawn_agent`, default `agent_type="worker"` and
  `agent_type="explorer"` subagents to `model="gpt-5.5"` with low reasoning
  unless the user explicitly requests a different model or the task clearly
  requires a different model.
- Use subagents only for concrete, bounded work that can run safely in parallel.
  Do not delegate work that blocks the immediate next local step.

## Warp Engineering Rules

- Read `WARP.md` before non-trivial code changes. It contains current build,
  test, architecture, and style guidance for this Rust/WarpUI codebase.
- For UI work, read `.agents/skills/warp-ui-guidelines/SKILL.md` before
  editing UI code. Reuse established WarpUI components, themes, and patterns.
- Be careful around terminal model locking. Avoid nested `model.lock()` calls;
  keep lock scopes short and pass locked references down when needed.
- Prefer focused tests for behavior changes. For broad refactors or subsystem
  removals, run the narrowest relevant checks first, then expand verification.
- Preserve unrelated user changes. This repo may have dirty worktree state; do
  not revert files you did not intentionally edit.



<claude-mem-context>
# Memory Context

# [warp] recent context, 2026-05-06 7:15am MDT

No previous sessions found.
</claude-mem-context>
