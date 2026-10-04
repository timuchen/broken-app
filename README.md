# broken-app-old

Исходный код до исправлений. Замеры этого каталога.

```bash
cargo check
cargo test
cargo +nightly miri test
cargo bench --bench baseline
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test --test integration --tests -Zbuild-std --target aarch64-apple-darwin
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test --test integration --tests -Zbuild-std --target aarch64-apple-darwin
cargo flamegraph --bin demo -o artifacts/flamegraph.svg
```

Логи в `artifacts/`.
