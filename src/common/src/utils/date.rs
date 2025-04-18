pub struct Date;

impl Date {
    pub fn nanoseconds_to_milliseconds(nanoseconds: u64) -> u32 {
        (nanoseconds / 1_000_000) as u32
    }
}
