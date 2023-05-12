use chrono::{FixedOffset, Local, NaiveDateTime, TimeZone};
use once_cell::sync::Lazy;

static OFFSET: Lazy<FixedOffset> = Lazy::new(|| {
    let local = Local::now();
    *local.offset()
});

pub fn naive_utc_to_naive_local(utc: &NaiveDateTime) -> NaiveDateTime {
    let local = OFFSET.from_utc_datetime(utc);
    local.naive_local()
}
