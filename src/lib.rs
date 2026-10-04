pub mod algo;
pub mod concurrency;

/// Сумма чётных значений без `get_unchecked` и выхода за границу.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = input.len();
    let raw = Box::into_raw(boxed) as *mut u8;

    let mut count = 0;
    unsafe {
        for i in 0..len {
            if *raw.add(i) != 0_u8 {
                count += 1;
            }
        }
        // утечка: не вызываем Box::from_raw(raw);
    }
    count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let sum: i64 = values.iter().sum();
    if values.is_empty() {
        return 0.0;
    }
    sum as f64 / values.len() as f64
}

/// Безопасная замена use-after-free: читаем значение до освобождения.
pub fn boxed_value() -> i32 {
    let b = Box::new(42_i32);
    let val = *b;
    val + val
}
