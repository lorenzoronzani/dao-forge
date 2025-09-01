use super::*;

#[test]
fn ns_to_ms_truncates() {
    assert_eq!(Date::nanoseconds_to_milliseconds(1_999_999), 1);
    assert_eq!(Date::nanoseconds_to_milliseconds(2_000_000), 2);
}

#[test]
fn ms_to_ns_exact() {
    assert_eq!(Date::milliseconds_to_nanoseconds(123), 123_000_000);
}

#[test]
fn timestamp_to_date_epoch() {
    assert_eq!(Date::timestamp_to_date(0), "01.01.1970");
}

#[test]
fn timestamp_to_date_leap_day() {
    let ns: u64 = 1_582_934_400_000_000_000;
    assert_eq!(Date::timestamp_to_date(ns), "29.02.2020");
}

#[test]
fn ms_ns_roundtrip() {
    let ms = 987_654;
    let ns = Date::milliseconds_to_nanoseconds(ms);
    assert_eq!(Date::nanoseconds_to_milliseconds(ns), ms);
}
