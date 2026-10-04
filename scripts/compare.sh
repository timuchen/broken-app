#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p artifacts

cargo bench --bench baseline | tee artifacts/baseline_after.txt
cargo bench --bench criterion -- --output-format bencher | tee artifacts/criterion_bench.txt

python3 - <<'PY'
import re
from pathlib import Path
from statistics import median

def parse_baseline(path):
    d = {}
    for line in Path(path).read_text().splitlines():
        m = re.match(r'^(sum_even|slow_fib|slow_dedup|normalize):\s*(.+)$', line.strip())
        if not m:
            continue
        label, raw = m.group(1), m.group(2).strip()
        if raw.endswith('ns'):
            ns = float(raw[:-2])
        elif 'µs' in raw or raw.endswith('us'):
            ns = float(raw.replace('µs', 'us')[:-2]) * 1e3
        elif raw.endswith('ms'):
            ns = float(raw[:-2]) * 1e6
        else:
            continue
        d.setdefault(label, []).append(ns)
    return {k: median(v) for k, v in d.items()}

before_path = Path('artifacts/baseline_before.txt')
after_path = Path('artifacts/baseline_after.txt')
if before_path.exists() and after_path.exists():
    before, after = parse_baseline(before_path), parse_baseline(after_path)
    lines = []
    for label in ['sum_even', 'slow_fib', 'slow_dedup', 'normalize']:
        if label in before and label in after and after[label]:
            b, a = before[label], after[label]
            lines.append(f"{label}: before_ns={b:.0f} after_ns={a:.0f} speedup≈{b/a:.1f}x")
    Path('artifacts/bench_comparison.txt').write_text('\n'.join(lines) + '\n')
    print('\n'.join(lines))
else:
    print('skip comparison: need artifacts/baseline_before.txt and baseline_after.txt')
PY
