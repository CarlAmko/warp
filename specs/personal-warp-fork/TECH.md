# Personal Warp Fork Revision - Tech Spec

## Context

`AGENTS.md` describes the fork philosophy, but it is not enough for a weaker
implementation model. It states priorities, not module boundaries, sequencing,
or acceptance checks. Use this spec as the implementation handoff for the first
revision of the fork.

This revision must also implement OpenAI Symphony in accordance with the
upstream specification:
https://github.com/openai/symphony/blob/main/SPEC.md. The implementer should
read that spec before building the Symphony slice. In this fork, Symphony is a
local orchestration layer for issue-driven coding-agent work, not a Warp/Oz
cloud replacement.

Before the fork work scales up, convert the repo to follow the Harness
Engineering Playbook:
`/Users/carl/.codex/skills/harness-engineering-playbook/SKILL.md`. The
implementer should read that skill plus its `references/openai-harness-practices.md`
and `references/rollout-checklist.md` before creating harness artifacts.

Important current code facts:

- `crates/warp_core/src/channel/config.rs:15` defines server, Oz, telemetry,
  autoupdate, crash-reporting, and MCP static config in one channel config.
- `crates/warp_core/src/channel/config.rs:44` hardcodes production upstream
  endpoints: `https://app.warp.dev`, `wss://rtc.app.warp.dev/graphql/v2`,
  `wss://sessions.app.warp.dev`, Firebase auth, and `https://oz.warp.dev`.
- `crates/warp_core/src/channel/state.rs:38` initializes OSS builds with
  production server and Oz config even though telemetry, updates, and crash
  reporting are `None`.
- `app/src/lib.rs:1010` extracts API-key auth, initializes `AuthState`, then
  registers `ServerApiProvider`, `AuthManager`, and
  `AppTelemetryContextProvider`.
- `app/src/server/server_api.rs:1` centralizes server clients for AI, auth,
  block, harness support, integrations, managed secrets, objects, referrals,
  teams, and workspaces.
- `app/src/settings/ai.rs:710` defines many AI settings. Many still use
  `SyncToCloud::Globally`, which conflicts with the fork's local-only default.
- `app/src/settings/ai.rs:1499` disables AI for anonymous/logged-out users and
  remote-session org policy. The fork should decouple AI from Warp login and
  org policy.
- `app/src/ai/llms.rs:26` treats BYOK as a workspace entitlement and checks
  provider-specific keys through `ApiKeyManager`.
- `app/src/ai/llms.rs:498` stores available model metadata in
  `LLMPreferences`, currently refreshed from auth/network/workspace events.
- `crates/graphql/src/api/workspace.rs:6` couples workspace metadata to teams,
  billing, available LLMs, request usage, and provider routing.
- `app/src/terminal/input/models/data_source.rs:494` renders upgrade/BYOK copy
  based on team entitlement and provider allowlists.
- `app/src/settings_view/privacy_page.rs:1676` still exposes cloud conversation
  storage behind feature/AI/org settings.
- `app/src/crash_reporting/mod.rs:183` initializes Sentry when feature and
  privacy settings allow it. This subsystem should be removed or permanently
  unreachable for the fork.

## Proposed Changes

### Phase 0: Harness Engineering Baseline

- Record the current Warp command surface in `PLANS.md`: `./script/bootstrap`,
  `./script/run`, `./script/presubmit`, `cargo fmt`, `cargo clippy`,
  `cargo nextest`, focused package checks, and any known long-running/flaky
  paths.
- Add or adapt the playbook artifacts without overwriting user-authored content:
  `PLANS.md`, `docs/ARCHITECTURE.md`, `docs/OBSERVABILITY.md`,
  `Makefile.harness`, `scripts/audit_harness.sh`,
  `scripts/harness/smoke.sh`, `scripts/harness/test.sh`,
  `scripts/harness/lint.sh`, and `scripts/harness/typecheck.sh`.
- If practical, use the playbook initializer:
  `python3 scripts/harness_wizard.py init /Users/carl/Documents/GitHub/warp --profile control`.
  If that script does not exist in this repo yet, create equivalent artifacts
  manually from the playbook templates and document the deviation in `PLANS.md`.
- Ensure the stable command layer exists:
  - `make smoke` for a cheap app/repo sanity check.
  - `make check` for lint/typecheck/format-fast checks.
  - `make ci` for the local equivalent of the full required validation path.
- Define strict module boundaries in `docs/ARCHITECTURE.md`, including current
  boundaries for terminal core, WarpUI, settings, server/cloud clients, AI/LLM
  routing, local CLI agents, Symphony, and harness scripts.
- Define observability expectations in `docs/OBSERVABILITY.md`: correlation
  IDs, structured log fields, Symphony run identifiers, provider request
  identifiers, privacy rules, and local-only log sinks.
- Add entropy controls: `scripts/audit_harness.sh` and a CI workflow or CI TODO
  that checks harness files, command availability, docs drift, and stale scripts.
- Run the harness audit path from the playbook:
  `python3 scripts/harness_wizard.py audit /Users/carl/Documents/GitHub/warp`
  when available. Treat `MISSING` and `FAIL` as blocking unless documented in
  `PLANS.md` with an exact follow-up.

### Phase 1: Disable Upstream Network Roots

- Replace the default `ChannelState` server/Oz initialization with fork-local
  inert defaults. Do not initialize production Warp server URLs in OSS/local
  builds.
- Make `SERVER_ROOT_URL`, `WS_SERVER_URL`, and similar overrides development
  diagnostics only. They must not silently point back to upstream Warp.
- Remove or neutralize autoupdate/release-repo wiring that references Warp
  release servers.
- Keep the in-app network log useful as a verification tool. Use it to prove
  the app is not calling upstream domains.

### Phase 2: Remove Telemetry And Crash Reporting

- Make telemetry macros and context providers no-op for this fork, or remove
  call sites incrementally while preserving compileability.
- Remove RudderStack/telemetry queue behavior and AI UGC telemetry collection.
- Remove Sentry/crash-reporting initialization and user identity tagging.
- Keep local logs for debugging, but they must never upload automatically.

### Phase 3: Remove Auth, Teams, Billing, And Cloud State From Core UX

- Let the app run as a local user without `AuthState` requiring Firebase/Warp
  credentials.
- Hide or delete settings/navigation for teams, billing, workspace invites,
  upgrade flows, enterprise policy, cloud conversation storage, Warp Drive
  sharing, and managed cloud objects.
- Remove GraphQL team/billing/workspace metadata as a runtime dependency for
  core terminal and local agent use. Generated GraphQL types can remain during
  early cuts if no runtime code depends on them.
- Any setting that persists provider, agent, prompt, accessibility, or local
  workflow state should be `SyncToCloud::Never` or use a local-only store.

### Phase 4: Replace Warp AI Routing With Local Provider Profiles

- Introduce a local provider profile model, preferably alongside existing AI
  settings initially to reduce blast radius.
- Minimum profile schema:
  - `id`
  - `display_name`
  - `base_url`
  - `api_key_storage_key` or explicit local credential reference
  - `model_id`
  - `compatibility` such as `openai_chat_completions`, `openai_responses`, or
    `anthropic_messages`
  - optional headers
- Store profile metadata in local settings and secrets in `secure_storage` or
  a clearly documented local auth file. Do not use cloud managed secrets.
- Convert `LLMPreferences` to read from local provider profiles instead of
  fetching model choices from Warp workspace metadata.
- Update model selector UI to show local profiles/models and remove upgrade,
  quota, discount, billing, and team-admin disabled states.
- Route built-in AI request paths through the selected profile. Start with the
  OpenAI-compatible request shape because it covers OpenAI, local servers, and
  many gateway providers.

### Phase 5: Preserve And Improve Local Agent Workflows

- Preserve CLI-agent detection, panes, rich input, code review comment routing,
  context chips, notifications, and terminal handoff behavior.
- Remove Oz/cloud-agent code paths from default UX. If a local equivalent is
  needed, implement it as a local agent runner instead of a Warp cloud task.
- Add follow-up TODOs for the Carl-specific command center: MCP/tool health,
  provider/model switcher, prompt/profile library, repo memory shortcuts,
  low-noise attention inbox, and explicit task handoff history.

### Phase 6: Implement Symphony Local Orchestration

- Implement Symphony against the upstream spec:
  `https://github.com/openai/symphony/blob/main/SPEC.md`.
- Keep the implementation local-first and independent of Warp cloud, Oz, Warp
  Drive, Warp auth, and Warp-hosted model routing.
- Add a repository workflow loader that reads `WORKFLOW.md`, parses optional
  YAML front matter, and returns typed config plus prompt body.
- Add a typed config layer for tracker, polling, workspace, hooks, agent, and
  Codex/runtime settings. Resolve environment-variable indirection without
  logging secrets.
- Add an issue tracker adapter beginning with Linear, matching the spec's
  normalized issue model and active/terminal-state filtering.
- Add an orchestrator that owns polling, bounded concurrency, claimed/running
  issue state, retry/backoff, reconciliation, and graceful stop behavior when
  issue eligibility changes.
- Add a workspace manager that maps issue identifiers to deterministic sanitized
  workspace paths, creates/preserves per-issue workspaces, runs configured
  lifecycle hooks, and cleans terminal-state workspaces only according to the
  documented policy.
- Add an agent runner that launches the configured coding-agent command in the
  per-issue workspace, renders the issue plus `WORKFLOW.md` prompt template,
  streams progress back to the orchestrator, and records run/session metadata.
- Add an operator status surface. A terminal/status pane is acceptable for the
  first pass; a rich dashboard is not required. It must show running, claimed,
  retrying, completed/handed-off, last event, workspace path, and recent error
  state.
- Document every implementation-defined Symphony policy: workspace cleanup,
  hook execution, sandbox/approval posture, tracker state transitions, retry
  limits, and whether config changes apply dynamically or require restart.

### Phase 7: Documentation And Packaging Cleanup

- Rewrite README/support/contributing/package templates so they no longer point
  to upstream Warp docs, Slack, careers, billing, release repos, support email,
  Oz, or `warpdotdev` repositories as active fork surfaces.
- Keep legal license files intact unless the user explicitly asks for licensing
  changes.
- Leave historical comments/tests only when removing them would add risk; mark
  high-volume residue with follow-up TODOs if needed.

## Execution Notes For A Weaker LLM

- Do not attempt the whole fork in one patch. Make one phase compile before
  starting the next.
- Do not delete terminal, renderer, panes, tabs, settings infrastructure,
  secure storage, local CLI-agent support, or WarpUI components just because
  they mention AI or Warp.
- Prefer hiding/dead-ending commercial UI first when full deletion would create
  a large compile cascade.
- When a server-dependent type is deeply threaded through local code, introduce
  a local adapter or no-op implementation before deleting the old client.
- Every time a cloud setting is touched, check whether it is stored with
  `SyncToCloud::Globally`; fork-owned settings should be local-only.
- After each phase, run a focused search for upstream domains and commercial
  words, then inspect only the matches that can execute at runtime.
- Keep `PLANS.md` current for multi-step work: scope, constraints, active
  phase, commands run, failures, next checkpoint, and acceptance gaps.
- Do not bypass harness commands with one-off manual steps unless the harness is
  missing. If it is missing, patch the harness first or document the gap.

## Parallelization

Safe parallel slices:

- Harness docs/command wrappers/CI audit.
- Network roots and packaging/docs cleanup.
- Telemetry/crash-reporting no-op work.
- Team/billing/cloud UI removal.
- AI provider profile model and local credential storage.
- Model selector/AI settings UI cleanup.
- Symphony workflow loader/config/tracker adapter.
- Symphony orchestrator/workspace manager/agent runner/status surface.
- CLI-agent regression validation.

Avoid parallel edits to `app/src/lib.rs`, `app/src/settings/ai.rs`,
`app/src/ai/llms.rs`, and `app/src/server/server_api.rs` unless ownership is
assigned clearly; those files are likely integration hotspots.
Avoid parallel edits to `Makefile.harness`, `PLANS.md`, and harness scripts
unless one worker owns the harness slice.

## Testing And Validation

- Run `rtk cargo check -p warp` after each phase that touches Rust code.
- Run `make smoke`, `make check`, and `make ci` once the harness layer exists.
- Run targeted tests for changed crates or modules before broad checks.
- For UI changes, run the smallest available WarpUI/app integration path and
  manually inspect the affected settings/model selector surfaces.
- Run `scripts/audit_harness.sh` and the playbook audit command when available.
- Use the in-app network log or an external proxy/firewall to verify no clean
  startup calls go to upstream Warp domains.
- Run:
  - `rtk rg -n "warp\\.dev|warpdotdev|oz\\.warp\\.dev|app\\.warp\\.dev|releases\\.warp\\.dev" app crates resources script README.md WARP.md CONTRIBUTING.md SECURITY.md about.toml`
  - `rtk rg -n "telemetry|RudderStack|Sentry|crash reporting|billing|enterprise|team invite|Warp Drive|cloud conversation|upgrade" app/src crates README.md WARP.md`
- Final acceptance should include a clean local launch, no sign-in requirement,
  no unexpected upstream network calls, and a successful AI request through a
  user-configured external provider profile.
- Symphony acceptance should include a local dry-run or controlled run that
  loads `WORKFLOW.md`, fetches or stubs an eligible Linear issue, creates the
  deterministic workspace path, dispatches one coding-agent handoff, records
  status/log output, and demonstrates retry or reconciliation behavior.

## Known Follow-Ups

- Rebrand product names and assets after functional upstream removal is stable.
- Expand the harness from control profile to full profile once the fork's core
  command surface stabilizes, including nightly entropy checks if useful.
- Decide whether local sync/export should replace any removed Warp Drive
  workflows.
- Design richer Symphony UI and Carl-specific command-center affordances after
  the spec-compliant local orchestration path is stable.
