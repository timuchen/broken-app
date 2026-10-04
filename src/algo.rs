/// Дедупликация: `sort_unstable` + `dedup` (O(n log n), без `HashSet`).
/// Алгоритмическая оптимизация: было O(n²) и сортировка на каждой вставке.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = values.to_vec();
    out.sort_unstable();
    out.dedup();
    out
}

/// Вариант через `HashSet` + одна сортировка — для сравнения в criterion.
pub fn slow_dedup_hashset(values: &[u64]) -> Vec<u64> {
    use std::collections::HashSet;
    let mut seen = HashSet::with_capacity(values.len());
    let mut out = Vec::with_capacity(values.len());
    for &v in values {
        if seen.insert(v) {
            out.push(v);
        }
    }
    out.sort_unstable();
    out
}

/// Итеративный Фибоначчи O(n) вместо экспоненциальной рекурсии.
/// `u64` переполняется при `n > 93` — тогда `checked_add` паникует.
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0_u64;
            let mut b = 1_u64;
            for _ in 2..=n {
                let next = a
                    .checked_add(b)
                    .expect("slow_fib overflows u64 for n > 93");
                a = b;
                b = next;
            }
            b
        }
    }
}
