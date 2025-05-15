pub struct Date;

impl Date {
    pub fn nanoseconds_to_milliseconds(nanoseconds: u64) -> u64 {
        (nanoseconds / 1000000) as u64
    }
}
