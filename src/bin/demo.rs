use broken_app::{algo, leak_buffer, normalize, sum_even};
use std::env;
use std::hint::black_box;

fn main() {
    let repeats: u64 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    if repeats <= 1 {
        let nums = [1, 2, 3, 4];
        println!("sum_even: {}", sum_even(&nums));

        let data = [1_u8, 0, 2, 3];
        println!("non-zero bytes: {}", leak_buffer(&data));

        let text = " Hello World ";
        println!("normalize: {}", normalize(text));

        let fib = algo::slow_fib(20);
        println!("fib(20): {}", fib);

        let uniq = algo::slow_dedup(&[1, 2, 2, 3, 1, 4, 4]);
        println!("dedup: {:?}", uniq);
        return;
    }

    let nums: Vec<i64> = (0..50_000).collect();
    let dedup_data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    let bytes = [1_u8, 0, 2, 3];
    let text = " Hello World ";
    for _ in 0..repeats {
        black_box(sum_even(black_box(nums.as_slice())));
        black_box(algo::slow_fib(black_box(32)));
        black_box(algo::slow_dedup(black_box(dedup_data.as_slice())));
        black_box(leak_buffer(black_box(bytes.as_slice())));
        black_box(normalize(black_box(text)));
    }
}
