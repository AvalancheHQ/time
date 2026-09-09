use std::hint::black_box;

use criterion::Bencher;
use time::{OffsetDateTime, UtcOffset};

setup_benchmark! {
    "UtcOffset",

    fn from_hms(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || UtcOffset::from_hms(black_box(0), black_box(0), black_box(0)),
        ]);
    }

    fn from_whole_seconds(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || UtcOffset::from_whole_seconds(black_box(0)),
        ]);
    }

    fn as_hms(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(UtcOffset::UTC).as_hms(),
        ]);
    }

    fn whole_hours(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(UtcOffset::UTC).whole_hours(),
        ]);
    }

    fn whole_minutes(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(UtcOffset::UTC).whole_minutes(),
        ]);
    }

    fn minutes_past_hour(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(UtcOffset::UTC).minutes_past_hour(),
        ]);
    }

    fn whole_seconds(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(UtcOffset::UTC).whole_seconds(),
        ]);
    }

    fn seconds_past_minute(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(UtcOffset::UTC).seconds_past_minute(),
        ]);
    }

    fn is_utc(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(UtcOffset::UTC).is_utc(),
        ]);
    }

    fn is_positive(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(UtcOffset::UTC).is_positive(),
        ]);
    }

    fn is_negative(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(UtcOffset::UTC).is_negative(),
        ]);
    }

    fn local_offset_at(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::local_offset_at(black_box(OffsetDateTime::UNIX_EPOCH)));
    }

    fn current_local_offset(ben: &mut Bencher<'_>) {
        ben.iter(UtcOffset::current_local_offset);
    }
}
