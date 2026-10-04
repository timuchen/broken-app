use broken_app::{algo, normalize, sum_even};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn bench_sum_even(c: &mut Criterion) {
    let data: Vec<i64> = (0..50_000).collect();
    c.bench_function("sum_even", |b| {
        b.iter(|| sum_even(black_box(data.as_slice())))
    });
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("slow_fib_32", |b| {
        b.iter(|| algo::slow_fib(black_box(32)))
    });
}

fn bench_dedup(c: &mut Criterion) {
    let data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    c.bench_function("slow_dedup_sort", |b| {
        b.iter_batched(
            || data.clone(),
            |v| {
                let _ = algo::slow_dedup(black_box(&v));
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("slow_dedup_hashset", |b| {
        b.iter_batched(
            || data.clone(),
            |v| {
                let _ = algo::slow_dedup_hashset(black_box(&v));
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_normalize(c: &mut Criterion) {
    let text = "  Hello\tWorld\n  Foo  Bar  Baz  ".repeat(2_000);
    c.bench_function("normalize", |b| {
        b.iter(|| normalize(black_box(text.as_str())))
    });
}

criterion_group!(
    benches,
    bench_sum_even,
    bench_fib,
    bench_dedup,
    bench_normalize
);
criterion_main!(benches);
