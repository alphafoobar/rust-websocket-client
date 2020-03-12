use chrono::{DateTime, Utc};

pub fn parse_datetime(s: &str) -> DateTime<Utc> {
    return DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc);
}