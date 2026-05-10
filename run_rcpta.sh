#!/usr/bin/env bash
# rcpta: compile once, then analyze with any entry func (--analyze-only).
# You must build rcpta yourself (e.g. cd rupta && cargo build).
#
# Usage:
#   ./run_rcpta.sh compile <path_to_file>
#   ./run_rcpta.sh <path_to_file> <entry_func> <output_dir> [--analyze-only]
#
# Examples:
#   # One-time: compile the test program (all deps + target)
#   ./run_rcpta.sh compile rustdsl/classes/tests/animal_hierarchy/main.rs
#
#   # Analyze: run PTA with given entry, write class_pag to output_dir (does compile+analyze)
#   ./run_rcpta.sh rustdsl/classes/tests/animal_hierarchy/main.rs prop_multilevel_upcast_preserves_identity analysis_results/rcpta/out
#
#   # Analyze only: skip full compile, just recompile target crate with PTA (fast, use after "compile")
#   ./run_rcpta.sh rustdsl/classes/tests/animal_hierarchy/main.rs prop_multilevel_upcast_preserves_identity analysis_results/rcpta/out --analyze-only

set -e
set -o pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
MANIFEST="$SCRIPT_DIR/rustdsl/classes/Cargo.toml"
CLASSES_ROOT="$SCRIPT_DIR/rustdsl/classes"

# Subcommand: compile only
if [[ "${1:-}" == "compile" ]]; then
  FILE_PATH="${2:?Usage: $0 compile <path_to_file>}"
  ABS_FILE="$(cd "$SCRIPT_DIR" && realpath -e "$FILE_PATH")"
  [[ "$ABS_FILE" == "$CLASSES_ROOT"* ]] || { echo "Error: $FILE_PATH is not under $CLASSES_ROOT"; exit 1; }
  if [[ "$ABS_FILE" == "$CLASSES_ROOT/tests/"* ]]; then
    REL="${ABS_FILE#$CLASSES_ROOT/tests/}"
    TEST_BINARY="${REL%%/*}"
    echo "Compiling test $TEST_BINARY..."
    (unset RUSTC_WRAPPER; cargo test --no-run --test "$TEST_BINARY" --manifest-path "$MANIFEST" --quiet)
  else
    [[ "$ABS_FILE" == "$CLASSES_ROOT/src/"* ]] || { echo "Error: file must be under tests/ or src/"; exit 1; }
    echo "Compiling lib..."
    (unset RUSTC_WRAPPER; cargo check --lib --manifest-path "$MANIFEST" --quiet)
  fi
  echo "Done. You can now run with --analyze-only and different entry funcs."
  exit 0
fi

# Analyze: path_to_file, entry_func, output_dir [, --analyze-only] [extra pta options...]
FILE_PATH="${1:?Usage: $0 <path_to_file> <entry_func> <output_dir> [--analyze-only] [extra pta options...]}"
ENTRY_FUNC="${2:?Usage: $0 <path_to_file> <entry_func> <output_dir> [--analyze-only] [extra pta options...]}"
OUTPUT_DIR="${3:?Usage: $0 <path_to_file> <entry_func> <output_dir> [--analyze-only] [extra pta options...]}"

shift 3
ANALYZE_ONLY=
if [[ "${1:-}" == "--analyze-only" ]]; then
  ANALYZE_ONLY=1
  shift 1
fi

# Any remaining args are extra PTA options (passed to `pta` after the `--` token).
EXTRA_PTA_ARGS=("$@")

# Resolve output dir to absolute path so pta (running from cargo's cwd, often manifest dir) writes to the right place
ABS_OUTPUT_DIR="$(cd "$SCRIPT_DIR" && realpath -m "$OUTPUT_DIR")"

# Resolve file to absolute path; must be under rustdsl/classes (tests/ or src/)
ABS_FILE="$(cd "$SCRIPT_DIR" && realpath -e "$FILE_PATH")"
REQ_PREFIX="$CLASSES_ROOT"
[[ "$ABS_FILE" == "$REQ_PREFIX"* ]] || { echo "Error: $FILE_PATH is not under $CLASSES_ROOT"; exit 1; }

# Decide --test <name> or --lib from path
if [[ "$ABS_FILE" == "$CLASSES_ROOT/tests/"* ]]; then
  REL="${ABS_FILE#$CLASSES_ROOT/tests/}"
  TEST_BINARY="${REL%%/*}"
  USE_LIB=
else
  [[ "$ABS_FILE" == "$CLASSES_ROOT/src/"* ]] || { echo "Error: file must be under tests/ or src/"; exit 1; }
  USE_LIB=1
  TEST_BINARY=
fi

# rcpta binary
# Note: in some environments `CARGO_TARGET_DIR` is set (e.g. by sandbox tooling),
# so binaries are generated under that dir rather than `rupta/target/`.
DEFAULT_TARGET_DIR="$SCRIPT_DIR/rupta/target"
TARGET_DIR="${CARGO_TARGET_DIR:-$DEFAULT_TARGET_DIR}"

if [[ -z "${CARGO_PTA:-}" ]]; then
  # Prefer the binary under TARGET_DIR/debug if present.
  CANDIDATE_1="$TARGET_DIR/debug/cargo-pta"
  CANDIDATE_2="$SCRIPT_DIR/rupta/target/debug/cargo-pta"
  if [[ -x "$CANDIDATE_1" ]]; then
    CARGO_PTA="$CANDIDATE_1"
  else
    CARGO_PTA="$CANDIDATE_2"
  fi
else
  # Respect user-provided override.
  CARGO_PTA="${CARGO_PTA}"
fi

[[ -x "$CARGO_PTA" ]] || { echo "Error: rcpta not found at $CARGO_PTA (build with: cd rupta && cargo build)"; exit 1; }

mkdir -p "$ABS_OUTPUT_DIR"

echo "file:    $ABS_FILE"
echo "entry:   $ENTRY_FUNC"
echo "out:     $ABS_OUTPUT_DIR"
[[ -n "$ANALYZE_ONLY" ]] && echo "mode:    analyze-only (skip full compile)"
echo "----------------------------------------"

# Phase 1: full compile (skip if --analyze-only)
if [[ -z "$ANALYZE_ONLY" ]]; then
  echo "[1/2] Compiling..."
  if [[ -n "$USE_LIB" ]]; then
    (unset RUSTC_WRAPPER; cargo check --lib --manifest-path "$MANIFEST" --quiet)
  else
    (unset RUSTC_WRAPPER; cargo test --no-run --test "$TEST_BINARY" --manifest-path "$MANIFEST" --quiet)
  fi
  echo "[1/2] Done."
  echo ""
fi

# Phase 2: touch + cargo-pta (recompile target crate with PTA, write class_pag)
echo "[2/2] Analyzing (entry: $ENTRY_FUNC)..."
touch "$ABS_FILE"
mkdir -p "$ABS_OUTPUT_DIR"

# Defensive: avoid inheriting stale PTA_FLAGS from outer shells/sessions.
# cargo-pta should set PTA_FLAGS for its wrapped rustc/pta invocations, but we
# clear and seed it here to prevent accidental old dump paths from leaking in.
unset PTA_FLAGS || true
export PTA_FLAGS='[]'

# If user passed explicit PTA knobs in EXTRA_PTA_ARGS, do not add defaults twice.
has_extra_pta_arg() {
  local needle="$1"
  shift
  local a
  for a in "${EXTRA_PTA_ARGS[@]}"; do
    if [[ "$a" == "$needle" ]]; then
      return 0
    fi
  done
  return 1
}

# Default PTA knobs (can be overridden by EXTRA_PTA_ARGS)
PTA_TYPE_ARGS=(--pta-type cs)
CTX_DEPTH_ARGS=(--context-depth 1)
if has_extra_pta_arg "--pta-type"; then
  PTA_TYPE_ARGS=()
fi
if has_extra_pta_arg "--context-depth"; then
  CTX_DEPTH_ARGS=()
fi

# Optional: skip MIR dump (can reduce rustc load for large entries)
MIR_DUMP_ARGS=(--dump-mir "$ABS_OUTPUT_DIR/mir.txt")
if [[ -n "${RCPTA_SKIP_MIR_DUMP:-}" ]]; then
  MIR_DUMP_ARGS=()
fi

# Avoid stack overflow when analyzing large crates (e.g. vehicle_hierarchy with deep type/call graphs).
# ulimit -s is in KiB; 1048576 = 1 GiB. All child processes (cargo, cargo-pta, pta) inherit this.
# Skip by setting RCPTA_SKIP_STACK_LIMIT=1 if you need the default limit.
if [[ -z "${RCPTA_SKIP_STACK_LIMIT:-}" ]]; then
  ulimit -s 1048576 2>/dev/null || ulimit -s 524288 2>/dev/null || ulimit -s 131072 2>/dev/null || true
fi
# Rust threads get stack from RUST_MIN_STACK (default 2 MiB); raise for deep recursion.
export RUST_MIN_STACK="${RUST_MIN_STACK:-67108864}"   # 64 MiB per thread

if [[ -n "$USE_LIB" ]]; then
  "$CARGO_PTA" pta \
    --manifest-path "$MANIFEST" \
    --lib \
    -- \
    --entry-func "$ENTRY_FUNC" \
    "${PTA_TYPE_ARGS[@]}" \
    "${CTX_DEPTH_ARGS[@]}" \
    --dump-class-pag "$ABS_OUTPUT_DIR/class_pag.txt" \
    --dump-class-pts "$ABS_OUTPUT_DIR/class_pts.txt" \
    --dump-class-call-graph "$ABS_OUTPUT_DIR/class_cg.txt" \
    --dump-type-info "$ABS_OUTPUT_DIR/type-info.txt" \
    --dump-inheritance-graph "$ABS_OUTPUT_DIR/inheritance_graph.txt" \
    --dump-cast-safety-log "$ABS_OUTPUT_DIR/cast_safety.log" \
    "${MIR_DUMP_ARGS[@]}" \
    "${EXTRA_PTA_ARGS[@]}" \
    2>&1 | tee "$ABS_OUTPUT_DIR/analysis.log"
else
  "$CARGO_PTA" pta \
    --manifest-path "$MANIFEST" \
    --test "$TEST_BINARY" \
    -- \
    --entry-func "$ENTRY_FUNC" \
    "${PTA_TYPE_ARGS[@]}" \
    "${CTX_DEPTH_ARGS[@]}" \
    --dump-class-pag "$ABS_OUTPUT_DIR/class_pag.txt" \
    --dump-class-pts "$ABS_OUTPUT_DIR/class_pts.txt" \
    --dump-class-call-graph "$ABS_OUTPUT_DIR/class_cg.txt" \
    --dump-type-info "$ABS_OUTPUT_DIR/type-info.txt" \
    --dump-inheritance-graph "$ABS_OUTPUT_DIR/inheritance_graph.txt" \
    --dump-cast-safety-log "$ABS_OUTPUT_DIR/cast_safety.log" \
    "${MIR_DUMP_ARGS[@]}" \
    "${EXTRA_PTA_ARGS[@]}" \
    2>&1 | tee "$ABS_OUTPUT_DIR/analysis.log"
fi
echo "[2/2] Done."
echo "----------------------------------------"
echo "class_pag:      $ABS_OUTPUT_DIR/class_pag.txt"
echo "class_pts:      $ABS_OUTPUT_DIR/class_pts.txt"
echo "class_cg:       $ABS_OUTPUT_DIR/class_cg.txt"
ls -la "$ABS_OUTPUT_DIR"
