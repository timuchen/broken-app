# broken-app-old

Исправления и замеры спринта 5.

Оракул поведения: локальный `../reference-app`, commit `0dbb780b4316a5d1b5cfc14e9fb41a5350a13f60`.

## Коммиты

1. `SPRINT-5 ferst metrics` — исходные баги и первые логи
2. `SPRINT-5 UB refactoring` — `sum_even`, `boxed_value`, `AtomicU64` + bench/flamegraph после UB
3. `SPRINT-5 algorithms and after metrics` — `slow_fib` / `slow_dedup` и логика + after-метрики
4. `SPRINT-5 harness and docs` — общий harness + README

## Сборка и проверки

```bash
cargo check
cargo test --test integration --tests
cargo +nightly miri test --test integration --tests
cargo bench --bench baseline

RUSTFLAGS="-Zsanitizer=address" cargo +nightly test --test integration --tests -Zbuild-std --target aarch64-apple-darwin
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test --test integration --tests -Zbuild-std --target aarch64-apple-darwin

cargo flamegraph --bin demo -o artifacts/flamegraph.svg -- 25000
```

Первый аргумент `demo` — число повторов для профиля.

Valgrind на macOS arm64 нет; лог снят в Linux Docker: `artifacts/valgrind.log`.

## Баги

| Где | Что было | Что сделано |
|-----|----------|-------------|
| `sum_even` | `get_unchecked` за границей | `filter` / `sum` |
| `leak_buffer` | утечка через `Box::into_raw` | подсчёт по срезу |
| `normalize` | только `' '` | `split_whitespace` |
| `average_positive` | среднее по всем | только `v > 0` |
| `use_after_free` | чтение после free | `boxed_value` |
| `race_increment` | `static mut` | `AtomicU64` + `Relaxed`, `join` без глушения |
| `slow_fib` / `slow_dedup` | рекурсия / O(n²) | цикл (`n > 93` → panic); `sort_unstable` + `dedup` |

Регрессии: `tests/integration.rs` (12 tests).

## Скорость

Один harness (`benches/baseline.rs`, `black_box`): `sum_even` 50000, `slow_fib(32)`, `slow_dedup` 5000 пар, `normalize` на `text.repeat(2000)`.

| | до (после UB) | после алгоритмов |
|--|---------------|------------------|
| `sum_even` | ~7.1 µs | ~7.0 µs |
| `slow_fib` | ~5.7 ms | ~42 ns |
| `slow_dedup` | ~7.8 ms | ~12 µs |
| `normalize` | ~122 µs | ~223 µs |

`normalize` после фикса делает больше работы (все whitespace), поэтому не ускоряется.

Логи: `artifacts/baseline_before.txt`, `baseline_after.txt`, `bench_comparison.txt`, `normalize_micro.txt`.

Flamegraph до ускорения: `artifacts/flamegraph.svg` (узкие места — `slow_fib`, `slow_dedup`).  
После: `artifacts/flamegraph_after.svg` (в основном `slow_dedup`, `sum_even`).

## Логи

`artifacts/`: `cargo_check.log`, `cargo_test.log`, `miri_test.log`, `asan_test.log`, `tsan_test.log`, `lldb_average_positive.txt`, `valgrind.log`, flamegraph и bench-файлы выше.
