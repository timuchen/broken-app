use broken_app::{algo, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn sums_even_empty_and_boundary() {
    assert_eq!(sum_even(&[]), 0);
    assert_eq!(sum_even(&[1]), 0);
    assert_eq!(sum_even(&[2]), 2);
    assert_eq!(sum_even(&[-2, -1, 0, 1, 2]), 0);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn leak_buffer_empty_and_all_zero() {
    assert_eq!(leak_buffer(&[]), 0);
    assert_eq!(leak_buffer(&[0, 0, 0]), 0);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]);
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn normalize_tabs_and_newlines() {
    assert_eq!(normalize(" Hello\tWorld\n"), "helloworld");
    assert_eq!(normalize("A  B\t\tC"), "abc");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn average_positive_edge_cases() {
    assert_eq!(broken_app::average_positive(&[]), 0.0);
    assert_eq!(broken_app::average_positive(&[-1, -2, 0]), 0.0);
    assert!((broken_app::average_positive(&[3, -1, 6]) - 4.5).abs() < f64::EPSILON);
}

#[test]
fn boxed_value_no_use_after_free() {
    assert_eq!(broken_app::boxed_value(), 84);
}

#[test]
fn race_increment_is_correct() {
    let total = broken_app::concurrency::race_increment(1_000, 4);
    assert_eq!(total, 4_000);
    assert_eq!(broken_app::concurrency::read_counter(), 4_000);
    broken_app::concurrency::reset_counter();
    assert_eq!(broken_app::concurrency::read_counter(), 0);
}
