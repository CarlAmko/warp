# Observability

Observability in this fork is local-first. Logs and status surfaces exist to
debug local terminal and agent workflows, not to upload product usage,
telemetry, metrics, crash reports, prompts, credentials, or workspace data.

## Policy

- No automatic telemetry, metrics, crash reporting, Sentry, analytics, or
  upstream log upload should be active in the fork target state.
- Local logs may be written to user-owned paths when they are inspectable and
  documented.
- Networked observability is allowed only when the user explicitly configures a
  destination.
- Secrets must be redacted before log creation, not only at display time.

## Required Event Fields

Use stable structured fields for new local events:

- `timestamp`: ISO 8601 or monotonic timestamp where appropriate.
- `level`: trace, debug, info, warn, or error.
- `component`: subsystem owner such as `harness`, `terminal`, `settings`,
  `ai_provider`, `cli_agent`, or `symphony`.
- `event`: stable event name.
- `correlation_id`: identifier that ties one user action or run together.
- `run_id`: harness, agent, or orchestration run identifier when available.
- `issue_id` and `issue_identifier`: for Symphony or issue-driven work.
- `workspace_path`: local workspace path when needed for operator repair.
- `provider_profile_id`: local AI provider profile id when relevant.
- `provider_request_id`: provider-side request id if returned by the provider.
- `error_class` and `error_message`: stable failure category plus redacted
  human-readable detail.

## Symphony Fields

Future Symphony work should also emit:

- `workflow_path`
- `tracker_kind`
- `poll_tick_id`
- `session_id`
- `attempt`
- `retry_count`
- `status`
- `active_count`
- `max_concurrent_agents`
- `last_tracker_state`

## Redaction Rules

Never log raw values for:

- API keys, bearer tokens, OAuth tokens, session cookies, and private headers.
- Provider request bodies that may contain credentials or private prompts.
- Full environment dumps.
- Secure-storage keys or decrypted secret values.
- Crash payloads, terminal scrollback, command output, or file contents unless
  the user explicitly chooses a local diagnostic export.

Use stable references instead, such as credential key names, provider profile
ids, issue identifiers, workspace paths, and redacted error categories.

## Harness Observability

Harness scripts should print the command being run, fail at the first blocking
error, and return non-zero exit codes. If a prerequisite is missing, print the
missing executable and the setup command or follow-up location.
