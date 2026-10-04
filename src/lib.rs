pub mod algo;
pub mod concurrency;

/// Сумма чётных значений без `get_unchecked` и выхода за границу.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов без аллокаций и без утечек.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|b| **b != 0).count()
}

/// Нормализация: убираем все виды пробельных символов и приводим к нижнему регистру.
pub fn normalize(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for part in input.split_whitespace() {
        out.extend(part.chars().flat_map(|c| c.to_lowercase()));
    }
    out
}

/// Усреднение только положительных чисел за один проход.
pub fn average_positive(values: &[i64]) -> f64 {
    let mut sum = 0_i64;
    let mut count = 0_usize;
    for &v in values {
        if v > 0 {
            sum += v;
            count += 1;
        }
    }
    if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

/// Безопасная замена use-after-free: читаем значение до освобождения.
pub fn boxed_value() -> i32 {
    let b = Box::new(42_i32);
    let val = *b;
    val + val
}
