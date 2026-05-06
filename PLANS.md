# Personal Warp Fork Plans

## Active Phase

Phase 4: Local AI Provider Profiles And Routing.

Phase 0 created deterministic repo-local entrypoints before upstream network,
telemetry, auth, billing, cloud, AI-provider, or Symphony runtime work. Phase 1
neutralized default upstream network roots without deleting broad
cloud/commercial code. Phase 2 centralized no-op/drop behavior for telemetry
and crash reporting. Phase 3 removes cloud/commercial UX from the default local
surface and guards the remaining deeply threaded runtime seams. Phase 4 adds
local AI provider profiles, feeds local model choices from those profiles, and
blocks Warp-hosted AI routing in local/default mode.

## Constraints

- Preserve terminal core, WarpUI, panes, tabs, local settings, secure storage,
  and CLI-agent support.
- Do not delete files, add dependencies, alter schemas, change migrations, or
  change database persisted data formats in Phase 4.
- Preserve unrelated dirty worktree changes.
- Use repo-local commands so agent runs do not depend on hidden manual steps.
- Prefer removal for user-facing cloud/commercial UX where compile impact is
  bounded; use guards/no-ops only for deeply threaded runtime seams.
- Store local AI provider secrets only in secure storage; never in cloud
  managed secrets, telemetry, crash reports, logs, or local profile metadata.
- Keep terminal core, WarpUI, local settings, secure storage, and CLI-agent
  workflows compiling.

## Current Command Surface

- `./script/bootstrap`: platform-specific setup.
- `./script/run`: local Warp run entrypoint.
- `./script/presubmit`: full existing validation path for fmt, clippy,
  clang-format, WGSL formatting, optional PowerShell lint, nextest, completer
  tests, and doc tests.
- `cargo fmt -- --check`: Rust formatting check.
- `cargo clippy --workspace --exclude warp_completer --all-targets --tests -- -D warnings`: primary clippy check.
- `cargo clippy -p warp_completer --all-targets --tests -- -D warnings`: completer clippy check.
- `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2`: workspace test path.
- `cargo nextest run -p warp_completer --features v2`: completer v2 test path.
- `cargo test --doc`: doc test path.
- `cargo check -p warp`: focused app typecheck.

## Known Slow Or Fragile Paths

- `./script/presubmit` and `make ci` can be long-running because they include
  full workspace tests.
- `cargo nextest run --workspace` depends on `cargo-nextest` and can surface
  platform-specific test requirements.
- `./script/run` and platform bundle scripts can require macOS/Linux/Windows
  local tooling and channel config.
- Existing CI workflows include upstream/internal assumptions such as release
  jobs, GCP auth, Sentry upload, Slack notification, large runners, and release
  packaging. Phase 0 adds a separate harness-only workflow instead of editing
  those workflows.

## Phase 0 Checklist

- [x] Add root `Makefile` that includes `Makefile.harness`.
- [x] Add `Makefile.harness` with `smoke`, `lint`, `typecheck`, `test`,
  `check`, and `ci`.
- [x] Add deterministic scripts under `scripts/harness/`.
- [x] Add `scripts/audit_harness.sh`.
- [x] Add compact architecture and observability docs.
- [x] Add harness-only CI workflow.
- [x] Run `make smoke`.
- [x] Run `make check`.
- [x] Run `scripts/audit_harness.sh`.

## Command Results

- `scripts/audit_harness.sh`: passed. All required Phase 0 artifacts,
  executable bits, make targets, docs anchors, and harness CI checks were
  present.
- `cargo fmt -- --check`: passed after Phase 1/2 changes.
- `scripts/audit_harness.sh`: passed after Phase 1/2 changes.
- `make smoke`: passed after `protoc` was installed.
- `make check`: passed after enabling Corepack/Yarn and installing the existing
  Warp cargo test tools (`wgslfmt`, `cargo-nextest`).
- `cargo check -p warp_core`: also failed before checking the target crate
  because the workspace still runs the `warpui` Metal shader build script.
- `make ci`: not run in Phase 3 because it includes long workspace test paths;
  `make smoke`, `make check`, and the harness audit now pass.
- `cargo check -p warp`: passed after Phase 4 local provider profile changes.
- `cargo test -p warp local_provider_profiles --lib`: passed; 2 tests passed,
  3610 filtered out.
- `cargo fmt -- --check`: passed after Phase 4 changes.
- `scripts/audit_harness.sh`: passed after Phase 4 changes.
- `make smoke`: passed after Phase 4 changes.
- `make check`: passed after Phase 4 changes; `pwsh`/PSScriptAnalyzer remains
  skipped because `pwsh` is not installed.
- Environment probe: `xcode-select -p` returned
  `/Applications/Xcode.app/Contents/Developer`; `xcrun -find metal` returned
  `/var/run/com.apple.security.cryptexd/mnt/.../Metal.xctoolchain/usr/bin/metal`;
  `xcrun metal -v` passed with Apple metal version `32023.883`.

## Current Blockers

- No current harness compile blocker after Phase 4 validation.
- Manual acceptance remains pending: launch the app locally, confirm no sign-in
  prompt, confirm settings/nav excludes team/billing/cloud commercial surfaces,
  verify local AI provider configuration appears in AI settings, confirm the
  model selector shows the configured local model, and verify the network log
  shows no startup calls to Firebase, Warp auth, workspace metadata, billing,
  managed secrets, cloud objects, or Warp AI/model metadata.
- Local provider profile metadata and model selection are wired, but the
  OpenAI-compatible streaming runtime adapter is still a follow-up. In
  local/default mode, built-in server-backed agent generation now returns a
  deterministic local-disabled error instead of contacting Warp AI endpoints.

## Next Checkpoint

After Phase 4 validation, report changed files, command results, blockers,
whether Phase 4 is complete, and the recommended next phase. Do not start
Phase 5 until that checkpoint is accepted.

## Phase 1 Progress

Started a narrow Phase 1 slice after the checkpoint:

- Added inert local channel configs in `crates/warp_core/src/channel/config.rs`
  using TEST-NET-1 URLs and no Firebase key/session sharing.
- Switched default `ChannelState::init()` from production Warp server/Oz roots
  to the inert local configs.
- Switched the OSS binary config from production Warp server/Oz roots to the
  inert local configs.
- Left generated first-party `production()` helpers in place for now; no call
  sites remain in the default OSS/channel initialization paths.

Phase 1 validation:

- `cargo fmt -- --check`: passed.
- `scripts/audit_harness.sh`: passed after Phase 1 changes.
- `make smoke`: passed after environment prerequisites were installed.
- `make check`: passed after environment prerequisites were installed.
- `cargo check -p warp_core`: previously blocked by the `warpui` Metal build
  script; rerun after `protoc` is available.
- Targeted search for `WarpServerConfig::production()` and
  `OzConfig::production()` in the touched channel/OSS paths returned no call
  sites.
- Targeted search for `app.warp.dev`, `oz.warp.dev`, `releases.warp.dev`, and
  `warpdotdev` shows remaining active roots are either first-party config
  helpers, tests/comments, package-template/autoupdate Linux debt, or
  commercial/account surfaces deferred to later phases.

## Phase 2 Progress

Implemented the central no-op/drop telemetry and crash-reporting cuts:

- Registered `NoopTelemetryContextProvider` for channels where telemetry config
  is absent; first-party telemetry channels still register
  `AppTelemetryContextProvider`.
- Gated telemetry flush, persisted flush, event send, Rudder batch send, and
  Rudder request send paths on `ChannelState::is_telemetry_available()`.
  Queued telemetry is cleared/dropped locally and returns success when telemetry
  is unavailable.
- Forced AI UGC telemetry collection to `false` when telemetry config is absent.
- Gated crash reporting init and Sentry init on
  `ChannelState::is_crash_reporting_available()`.
- Made Sentry tag setters no-op for channels where crash reporting is
  unavailable. Existing local crash recovery behavior was not removed.

Phase 2 validation:

- `cargo fmt -- --check`: passed.
- `scripts/audit_harness.sh`: passed.
- `make smoke`: passed after environment prerequisites were installed.
- `make check`: passed after environment prerequisites were installed.
- `cargo check -p warp_core`: previously blocked by the `warpui` Metal build
  script; rerun after `protoc` is available.
- Targeted search for `RudderStack`, `rudder`, `Sentry`,
  `crash_reporting::init`, and telemetry context providers shows the default
  runtime path now uses no-op/drop behavior when channel telemetry or crash
  config is absent. Remaining `AppTelemetryContextProvider` matches are tests
  or first-party telemetry-enabled paths.

## Phase 3 Progress

Implemented the first removal-first auth/team/billing/cloud UX pass:

- Added `ChannelState::is_warp_cloud_available()` as the central capability
  check for Warp-hosted auth, teams, billing, cloud objects, cloud preference
  sync, and managed cloud secrets. The default local/OSS config has no Firebase
  key and reports cloud unavailable.
- Local/default auth initialization now returns a logged-out local state before
  API-key auth, `WARP_USER_SECRET`, Firebase persisted credentials, anonymous
  user creation, sign-in URLs, device auth, token refresh, SSO linking, or
  anonymous conversion can start.
- Settings navigation now removes Billing and usage, Teams, Referrals, Shared
  blocks, Warp Drive, and the Cloud platform umbrella in cloud-unavailable
  mode. Attempts to navigate directly to those sections are redirected to
  Account.
- Account settings no longer contains sign-up, settings sync, referrals, plan
  badges, compare-plan links, billing links, upgrade actions, billing portal
  actions, or anonymous conversion controls. The account page is now local
  session/profile information plus local version/logout controls.
- AI settings no longer renders the two active upgrade CTAs when cloud is
  unavailable, and the central `UserWorkspaces::upgrade_link*` helpers return
  an inert URL if any remaining deferred surface asks for an upgrade link.
- Cloud conversation storage privacy controls are hidden and the toggle handler
  is inert when cloud is unavailable.
- Team metadata polling, cloud-object polling, cloud preference sync/retry, and
  managed-secrets GraphQL clients now short-circuit when cloud is unavailable.
- Added a focused settings-section classification test covering the
  cloud/commercial sections removed from local navigation.
- Follow-up deletion-heavy cleanup removed the Account page cloud action enum
  variants and widget implementations rather than only hiding them behind
  guards.
- Follow-up deletion-heavy cleanup stopped registering Billing and usage,
  Teams, Referrals, Shared blocks, Warp Drive, and cloud API key settings pages
  when Warp cloud is unavailable. These pages remain available only for
  first-party cloud-enabled channels.

Phase 3 guarded compatibility seams:

- Commercial/cloud settings page modules, generated GraphQL types, and shared
  action enums still compile but are filtered out or guarded in local/default
  mode. They are not deleted in this phase because that would cascade through
  generated clients and shared settings routing.
- Managed secrets still registers through the existing provider path for
  compatibility, but every ServerApi managed-secrets operation returns empty or
  local-disabled behavior when Warp cloud is unavailable.
- Existing Warp Drive/cloud object model types remain because terminal, agent,
  and local persistence code still share those data structures.

Phase 3 validation:

- `cargo fmt`: passed after Phase 3 edits.
- `cargo fmt -- --check`: passed after Phase 3 edits.
- `scripts/audit_harness.sh`: passed after Phase 3 edits.
- `protoc --version`: passed with `libprotoc 34.1`.
- `corepack enable`: ran to expose Yarn for the `command-signatures-v2` build
  script; `yarn --version` returned `1.22.22`.
- `script/install_cargo_test_deps`: ran existing Warp tool installation and
  installed `wgslfmt` plus `cargo-nextest`.
- `make smoke`: passed after the first Phase 3 pass and again after the
  deletion-heavy Account settings cleanup.
- `make check`: passed after the first Phase 3 pass and again after the
  deletion-heavy Account settings cleanup.
- `xcrun -find metal` and `xcrun metal -v`: passed; Metal is no longer the
  active validation blocker.
- Targeted search confirms the default local guard points exist for
  `is_warp_cloud_available`, auth URL generation, settings nav filtering,
  account page cloud actions, cloud-object/team pollers, cloud preference sync,
  cloud conversation storage, managed secrets, and central upgrade links.

Phase 3 acceptance gaps:

- Manual acceptance remains pending after compile: clean launch without sign-in,
  primary settings/nav without team/billing/cloud surfaces, and network log
  showing no startup calls to Firebase, Warp auth, workspace metadata, billing,
  managed secrets, or cloud objects.
- Remaining broader deletion of dormant cloud/commercial modules is deferred to
  the next pass; target bounded workspace/menu action entrypoints first, not
  generated GraphQL clients or shared model types.

## Phase 4 Progress

Implemented the local-provider baseline:

- Added `LocalAIProviderProfiles` with local profile metadata in private user
  preferences and provider API keys in secure storage. The initial editable
  default profile supports OpenAI-compatible chat completions or responses,
  `display_name`, `base_url`, `model_id`, optional headers, and a local
  credential reference.
- Registered local provider profiles before `LLMPreferences` so local/default
  model choices can be derived without contacting Warp model metadata APIs.
- In cloud-unavailable mode, `LLMPreferences` now uses local provider profiles
  instead of cached/fetched Warp model choices and skips
  `get_feature_model_choices` / `get_free_available_models`.
- Added AI settings UI for the local provider profile and hide the Warp BYOK /
  Bedrock key widgets in local/default mode.
- Local/default AI enablement no longer depends on Warp login state.
- Request construction now excludes Warp-credit fallback in local/default mode
  and bypasses BYOK/team entitlement checks for local API-key inclusion.
- Server-backed multi-agent generation now fails with a deterministic
  local-disabled error when Warp cloud is unavailable, preventing accidental
  Warp AI calls until the local OpenAI-compatible streaming adapter is built.
- Added focused local provider tests for secret separation and local model
  choices without upgrade/quota states.

Phase 4 guarded compatibility seams:

- Generated GraphQL model-choice types and `AIClient` methods remain for
  first-party/cloud-enabled builds and tests.
- Existing `warp_multi_agent_api` request conversion remains in place to keep
  agent panes, conversations, MCP context, permissions, and CLI-agent workflows
  compiling.
- Local provider streaming is not implemented yet; this is the next required
  AI slice before claiming end-to-end built-in AI completion.

Phase 4 validation:

- `cargo check -p warp`: passed.
- `cargo test -p warp local_provider_profiles --lib`: passed.
- `cargo fmt -- --check`: passed.
- `scripts/audit_harness.sh`: passed.
- `make smoke`: passed.
- `make check`: passed.
- Targeted search for `get_feature_model_choices`,
  `get_free_available_models`, `is_byo_api_key_enabled`, `RequiresUpgrade`,
  `OutOfRequests`, `allow_use_of_warp_credits`, `managed secrets`, and
  `Warp Drive context` shows remaining matches are first-party/cloud paths,
  generated/server compatibility, tests, guarded seams, or Phase 5/AI follow-up
  debt.

Phase 4 acceptance gaps:

- Manual launch/network-log acceptance remains pending.
- A configured local provider appears in model choices, but built-in agent
  streaming still returns the local-disabled error until the OpenAI-compatible
  runtime adapter is implemented.
