use exponential_backoff::Backoff;
use std::time;

pub fn open_session() -> Backoff {
    let attempts = 3;
    let min = time::Duration::from_secs(1);
    let max = time::Duration::from_secs(5);
    Backoff::new(attempts, min, max)
}

pub fn close_session() -> Backoff {
    let attempts = 2;
    let min = time::Duration::from_secs(1);
    let max = time::Duration::from_secs(3);
    Backoff::new(attempts, min, max)
}

pub fn get_addresses() -> Backoff {
    let attempts = 10;
    let min = time::Duration::from_secs(1);
    let max = time::Duration::from_secs(60);
    Backoff::new(attempts, min, max)
}

pub fn list_sessions() -> Backoff {
    let attempts = 3;
    let min = time::Duration::from_secs(1);
    let max = time::Duration::from_secs(5);
    Backoff::new(attempts, min, max)
}

pub trait FromIteratorToSeries {
    fn to_vec(&self) -> Vec<time::Duration>;
}

impl FromIteratorToSeries for Backoff {
    // Convert the Backoff struct into a Vec<time::Duration>
    // reversing order so that smallest durations are first
    fn to_vec(&self) -> Vec<time::Duration> {
        let mut vec = self.into_iter().fold(Vec::new(), |mut acc, e| {
            if let Some(dur) = e {
                acc.push(dur);
            }
            acc
        });
        vec.reverse();
        vec
    }
}
