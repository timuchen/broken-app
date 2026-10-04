#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p artifacts

REPEATS="${1:-25000}"
OUT="${2:-artifacts/flamegraph.svg}"

cargo build --release --bin demo
if command -v cargo-flamegraph >/dev/null 2>&1; then
  cargo flamegraph --bin demo -o "$OUT" -- "$REPEATS" || {
    echo "cargo flamegraph failed; falling back to sample(1)" >&2
    ./target/release/demo "$REPEATS" &
    pid=$!
    sample "$pid" 8 -mayDie -file /tmp/broken-app-sample.txt || true
    wait "$pid" || true
    if [[ -f /tmp/FlameGraph/stackcollapse-sample.awk ]]; then
      awk -f /tmp/FlameGraph/stackcollapse-sample.awk /tmp/broken-app-sample.txt \
        | perl /tmp/FlameGraph/flamegraph.pl > "$OUT"
    fi
  }
else
  echo "install cargo-flamegraph or use sample/FlameGraph manually" >&2
  exit 1
fi

ls -l "$OUT"
