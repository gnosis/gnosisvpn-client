use humantime::format_duration;
use serde::ser::Serialize;
use std::time::SystemTime;

pub fn serialize<T>(v: &T) -> String
where
    T: ?Sized + Serialize,
{
    match serde_json::to_string(&v) {
        Ok(s) => s,
        Err(e) => format!("serializion error: {}", e),
    }
}

pub fn elapsed(timestamp: &SystemTime) -> String {
    match timestamp.elapsed() {
        Ok(elapsed) => truncate_after_second_space(format_duration(elapsed).to_string().as_str()).to_string(),
        Err(e) => format!("error displaying duration: {}", e),
    }
}

fn truncate_after_second_space(s: &str) -> &str {
    let spaces = s.match_indices(' ').take(2);
    if let Some((index, _)) = spaces.last() {
        &s[..index]
    } else {
        s
    }
}
