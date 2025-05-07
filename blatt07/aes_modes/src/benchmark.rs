use std::time::{Duration, Instant};

pub fn time<T, F>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    (f(), start.elapsed())
}

#[macro_export]
macro_rules! time {
    ($expr:expr) => {{
        time(|| $expr)
    }};
}