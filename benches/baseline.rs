use broken_app::{algo, normalize, sum_even};
use std::hint::black_box;
use std::time::Instant;

fn time_it(label: &str, mut f: impl FnMut()) {
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    println!("{label}: {:?}", elapsed);
}

fn main() {
    let data: Vec<i64> = (0..50_000).collect();
    let fib_n = 32;
    let dedup_data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    let text = "  Hello\tWorld\n  Foo  Bar  Baz  ".repeat(2_000);

    for _ in 0..3 {
        time_it("sum_even", || {
            black_box(sum_even(black_box(data.as_slice())));
        });

        time_it("slow_fib", || {
            black_box(algo::slow_fib(black_box(fib_n)));
        });

        time_it("slow_dedup", || {
            black_box(algo::slow_dedup(black_box(dedup_data.as_slice())));
        });

        time_it("normalize", || {
            black_box(normalize(black_box(text.as_str())));
        });
    }
}
