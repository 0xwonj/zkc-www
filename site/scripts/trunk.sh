#!/usr/bin/env sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
TRUNK_CONFIG="$ROOT_DIR/Trunk.toml"
LOCK_DIR="$ROOT_DIR/.trunk-lock"
LOCK_PID_FILE="$LOCK_DIR/pid"

DIST_DIR=$(
  awk -F'"' '/^dist[[:space:]]*=/ { print $2; exit }' "$TRUNK_CONFIG" 2>/dev/null || true
)

if [ -z "${DIST_DIR:-}" ]; then
  DIST_DIR="dist"
fi

cleanup_lock() {
  rm -rf "$LOCK_DIR"
}

find_conflicting_trunk_process() {
  for PID in $(pgrep -x trunk 2>/dev/null || true); do
    PROCESS_CWD=$(
      lsof -a -d cwd -p "$PID" -Fn 2>/dev/null | sed -n 's/^n//p' | head -n 1
    )

    if [ "$PROCESS_CWD" = "$ROOT_DIR" ]; then
      COMMAND=$(ps -o command= -p "$PID" 2>/dev/null || true)
      printf >&2 '%s\n' "Another trunk process is already running for this site (pid $PID: $COMMAND). Stop it before starting a new build or serve."
      exit 1
    fi
  done
}

acquire_lock() {
  while :; do
    if mkdir "$LOCK_DIR" 2>/dev/null; then
      printf '%s\n' "$$" > "$LOCK_PID_FILE"
      trap cleanup_lock EXIT HUP INT TERM
      return 0
    fi

    LOCK_OWNER=""
    if [ -f "$LOCK_PID_FILE" ]; then
      LOCK_OWNER=$(cat "$LOCK_PID_FILE" 2>/dev/null || true)
    fi

    if [ -n "$LOCK_OWNER" ] && kill -0 "$LOCK_OWNER" 2>/dev/null; then
      printf >&2 '%s\n' "Another trunk process is already using this site (pid $LOCK_OWNER). Stop it before starting a new build or serve."
      exit 1
    fi

    rm -rf "$LOCK_DIR"
  done
}

mkdir -p "$ROOT_DIR/$DIST_DIR" "$ROOT_DIR/$DIST_DIR/.stage"
find_conflicting_trunk_process
acquire_lock

if [ "${1:-}" = "serve" ]; then
  shift
  env -u NO_COLOR trunk serve \
    --watch src \
    --watch style \
    --watch public \
    --watch index.html \
    --watch Cargo.toml \
    --watch Cargo.lock \
    --watch Trunk.toml \
    --ignore target \
    --ignore dist \
    --ignore output \
    --ignore .trunk-lock \
    "$@"
else
  env -u NO_COLOR trunk "$@"
fi
