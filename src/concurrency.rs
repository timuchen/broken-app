use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Потокобезопасный инкремент через `AtomicU64` (без data race).
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    COUNTER.store(0, Ordering::Relaxed);
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                COUNTER.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().expect("worker thread panicked");
    }
    COUNTER.load(Ordering::Relaxed)
}

/// Текущее значение счётчика.
pub fn read_counter() -> u64 {
    COUNTER.load(Ordering::Relaxed)
}

/// Сброс счётчика.
pub fn reset_counter() {
    COUNTER.store(0, Ordering::Relaxed);
}
