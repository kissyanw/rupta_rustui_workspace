# RCPTA Session Snapshot (2026-05-04)

## Current objective
- Continue RCPTA optimization on `vehicle_suites`.
- Focus target: `entry5` in `vehicle_hierarchy`:
  - test/property: `prop_drivable_interface_polymorphism`
  - command pattern: `./run_rcpta.sh ... --analyze-only --context-depth 1`
- Requirement from user:
  - fix entry5 stack overflow with minimal side effects;
  - avoid expensive full-suite regressions;
  - validate using selected complex entries only.

## What happened in this session
1. We attempted to continue entry5 fix work.
2. Environment issue blocked normal sandbox execution:
   - sandbox commands failed with:
   - `failed to open synthetic bubblewrap mount registry lock /tmp/codex-bwrap-synthetic-mount-targets/lock: Permission denied (os error 13)`
3. Because of that, escalation prompts became frequent (required by execution policy when key commands fail in sandbox).
4. User requested full rollback of entry5-investigation-related code/logging changes and asked to produce this snapshot for next conversation.

## Rollback completed in this session
The following source files were restored to repository state (no local modifications now):
- `rupta/src/bin/cargo-pta.rs`
- `rupta/src/bin/pta.rs`
- `rupta/src/builder/fpag_builder.rs`
- `rupta/src/pta/andersen.rs`
- `rupta/src/pta/context_sensitive.rs`
- `rupta/src/util/class/analysis.rs`
- `rupta/src/util/results_dumper.rs`

Verification:
- `git status --short <above files>` returned empty output.

## Important context to carry into next chat
- Entry3/Entry4 previously analyzed successfully.
- Entry5 failure symptom remains unresolved after rollback:
  - crash location around RCPTA analysis init (`initialize()` stage)
  - runtime error: rustc thread stack overflow.
- `always_encode_mir` was previously tested and is likely not root cause.
- Preferred strategy (user-approved):
  - localize issue in entry5 first;
  - keep changes minimal;
  - run targeted vehicle entries for regression (not full animal/vehicle sweeps each iteration).

## Environment issue to resolve first in next chat
Potential blocker:
- `/tmp/codex-bwrap-synthetic-mount-targets/lock` permission problem in sandbox.

Suggested first steps next chat:
1. Check/repair `/tmp/codex-bwrap-synthetic-mount-targets` ownership/permissions.
2. Re-run a minimal read-only command in sandbox to confirm normal operation.
3. Resume entry5 staged isolation inside `context_sensitive::initialize()` with minimal instrumentation.
