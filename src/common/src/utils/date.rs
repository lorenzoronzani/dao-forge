use time::OffsetDateTime;

pub struct Date;

impl Date {
    pub fn nanoseconds_to_milliseconds(nanoseconds: u64) -> u64 {
        (nanoseconds / 1000000) as u64
    }

    pub fn milliseconds_to_nanoseconds(milliseconds: u64) -> u64 {
        milliseconds * 1000000
    }

    pub fn timestamp_to_date(nanoseconds: u64) -> String {
        let timestamp_s = nanoseconds / 1_000_000_000;
        let date = OffsetDateTime::from_unix_timestamp(timestamp_s as i64).unwrap();
        format!(
            "{:02}.{:02}.{:04}",
            date.day(),
            date.month() as u32,
            date.year()
        )
    }
}
