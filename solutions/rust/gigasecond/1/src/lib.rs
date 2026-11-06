use time::{PlainDateTime as DateTime, SignedDuration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start
        .checked_add(SignedDuration::seconds(1_000_000_000))
        .unwrap()
}
