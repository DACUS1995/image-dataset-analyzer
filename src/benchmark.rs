use std::time::{self, Duration};

pub fn timeit<F: Fn() -> T, T>(f: F) -> (T, Duration) {
    let start = time::SystemTime::now();
    let result = f();
    let end = time::SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    (result, duration)
}
