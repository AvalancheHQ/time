use std::hint::black_box;
use std::time::SystemTime;

use criterion::Bencher;
use time::OffsetDateTime;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::{date, datetime, offset, time};

setup_benchmark! {
    "OffsetDateTime",

    fn now_utc(ben: &mut Bencher<'_>) {
        ben.iter(OffsetDateTime::now_utc);
    }

    fn now_local(ben: &mut Bencher<'_>) {
        ben.iter(OffsetDateTime::now_local);
    }

    fn to_offset(ben: &mut Bencher<'_>) {
        iter_all!(ben, [
            || datetime!(2000-01-01 0:00 +11).to_offset(offset!(-5)),
            || datetime!(2000-01-01 0:00 +11).to_offset(offset!(-8)),
        ]);
    }

    fn to_utc(ben: &mut Bencher<'_>) {
        let datetime = black_box(datetime!(2000-01-01 0:00 +11));
        ben.iter(|| datetime.to_utc());
    }

    fn from_unix_timestamp(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || OffsetDateTime::from_unix_timestamp(black_box(0)),
            || OffsetDateTime::from_unix_timestamp(black_box(1_546_300_800)),
        ]);
    }

    fn from_unix_timestamp_nanos(ben: &mut Bencher<'_>) {
        iter_all!(ben, [
            || OffsetDateTime::from_unix_timestamp_nanos(black_box(0)),
            || {
                OffsetDateTime::from_unix_timestamp_nanos(black_box(1_546_300_800_000_000_000))
            },
        ]);
    }

    fn offset(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2019-01-01 0:00 UTC));
        let midnight_plus_one = black_box(datetime!(2019-01-01 0:00 +1));
        let one_am_plus_one = black_box(datetime!(2019-01-01 1:00 +1));
        iter_all_repeated!(ben, [
            || utc.offset(),
            || midnight_plus_one.offset(),
            || one_am_plus_one.offset(),
        ]);
    }

    fn unix_timestamp(ben: &mut Bencher<'_>) {
        let epoch = black_box(OffsetDateTime::UNIX_EPOCH);
        let plus_one = black_box(datetime!(1970-01-01 1:00 +1));
        let minus_one = black_box(datetime!(1970-01-01 0:00 -1));
        iter_all_repeated!(ben, [
            || epoch.unix_timestamp(),
            || plus_one.unix_timestamp(),
            || minus_one.unix_timestamp(),
        ]);
    }

    fn unix_timestamp_nanos(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(1970-01-01 0:00 UTC));
        let plus_one = black_box(datetime!(1970-01-01 1:00 +1));
        let minus_one = black_box(datetime!(1970-01-01 0:00 -1));
        iter_all_repeated!(ben, [
            || utc.unix_timestamp_nanos(),
            || plus_one.unix_timestamp_nanos(),
            || minus_one.unix_timestamp_nanos(),
        ]);
    }

    fn date(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2019-01-01 0:00 UTC));
        let non_utc = black_box(datetime!(2018-12-31 23:00 -1));
        iter_all_repeated!(ben, [
            || utc.date(),
            || non_utc.date(),
        ]);
    }

    fn time(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2019-01-01 0:00 UTC));
        let non_utc = black_box(datetime!(2018-12-31 23:00 -1));
        iter_all_repeated!(ben, [
            || utc.time(),
            || non_utc.time(),
        ]);
    }

    fn year(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2019-01-01 0:00 UTC));
        let non_utc = black_box(datetime!(2018-12-31 23:00 -1));
        iter_all_repeated!(ben, [
            || utc.year(),
            || non_utc.year(),
        ]);
    }

    fn ordinal(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2019-01-01 0:00 UTC));
        let non_utc = black_box(datetime!(2018-12-31 23:00 -1));
        iter_all_repeated!(ben, [
            || utc.ordinal(),
            || non_utc.ordinal(),
        ]);
    }

    fn hour(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2019-01-01 0:00 UTC));
        let non_utc = black_box(datetime!(2018-12-31 23:00 -1));
        iter_all_repeated!(ben, [
            || utc.hour(),
            || non_utc.hour(),
        ]);
    }

    fn minute(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2019-01-01 0:00 UTC));
        let non_utc = black_box(datetime!(2018-12-31 23:00 -1));
        iter_all_repeated!(ben, [
            || utc.minute(),
            || non_utc.minute(),
        ]);
    }

    fn second(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2019-01-01 0:00 UTC));
        let non_utc = black_box(datetime!(2018-12-31 23:00 -1));
        iter_all_repeated!(ben, [
            || utc.second(),
            || non_utc.second(),
        ]);
    }

    fn replace_time(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2020-01-01 5:00 UTC));
        let minus_five = black_box(datetime!(2020-01-01 12:00 -5));
        let plus_one = black_box(datetime!(2020-01-01 0:00 +1));
        iter_all_repeated!(ben, [
            || utc.replace_time(black_box(time!(12:00))),
            || minus_five.replace_time(black_box(time!(7:00))),
            || plus_one.replace_time(black_box(time!(12:00))),
        ]);
    }

    fn replace_date(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2020-01-01 12:00 UTC));
        let plus_one = black_box(datetime!(2020-01-01 0:00 +1));
        iter_all_repeated!(ben, [
            || utc.replace_date(black_box(date!(2020-01-30))),
            || plus_one.replace_date(black_box(date!(2020-01-30))),
        ]);
    }

    fn replace_date_time(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2020-01-01 12:00 UTC));
        let plus_one = black_box(datetime!(2020-01-01 12:00 +1));
        iter_all_repeated!(ben, [
            || utc.replace_date_time(black_box(datetime!(2020-01-30 16:00))),
            || plus_one.replace_date_time(black_box(datetime!(2020-01-30 0:00))),
        ]);
    }

    fn replace_offset(ben: &mut Bencher<'_>) {
        let utc = black_box(datetime!(2020-01-01 0:00 UTC));
        iter_all_repeated!(ben, [
            || utc.replace_offset(black_box(offset!(-5))),
        ]);
    }

    fn partial_eq(ben: &mut Bencher<'_>) {
        ben.iter(|| datetime!(1999-12-31 23:00 -1) == datetime!(2000-01-01 0:00 UTC));
    }

    fn partial_ord(ben: &mut Bencher<'_>) {
        ben.iter(||
            datetime!(2019-01-01 0:00 UTC).partial_cmp(&datetime!(1999-12-31 23:00 -1))
        );
    }

    fn ord(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || datetime!(2019-01-01 0:00 UTC) == datetime!(2018-12-31 23:00 -1),
            || datetime!(2019-01-01 0:00:00.000_000_001 UTC) > datetime!(2019-01-01 0:00 UTC),
        ]);
    }

    fn hash(ben: &mut Bencher<'_>) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        iter_batched_ref!(
            ben,
            DefaultHasher::new,
            [
                |hasher| datetime!(2019-01-01 0:00 UTC).hash(hasher),
                |hasher| datetime!(2018-12-31 23:00 -1).hash(hasher),
            ]
        );
    }

    fn add_duration(ben: &mut Bencher<'_>) {
        let a = 5.days();
        let b = 1.days();
        let c = 2.seconds();
        let d = (-2).seconds();
        let e = 1.hours();

        iter_all!(ben, [
            || datetime!(2019-01-01 0:00 UTC) + a,
            || datetime!(2019-12-31 0:00 UTC) + b,
            || datetime!(2019-12-31 23:59:59 UTC) + c,
            || datetime!(2020-01-01 0:00:01 UTC) + d,
            || datetime!(1999-12-31 23:00 UTC) + e,
        ]);
    }

    fn add_std_duration(ben: &mut Bencher<'_>) {
        let a = 5.std_days();
        let b = 1.std_days();
        let c = 2.std_seconds();

        iter_all!(ben, [
            || datetime!(2019-01-01 0:00 UTC) + a,
            || datetime!(2019-12-31 0:00 UTC) + b,
            || datetime!(2019-12-31 23:59:59 UTC) + c,
        ]);
    }

    fn add_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.days();
        let b = 1.seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00 UTC),
            [
                |datetime| *datetime += a,
                |datetime| *datetime += b,
            ]
        );
    }

    fn add_assign_std_duration(ben: &mut Bencher<'_>) {
        let a = 1.std_days();
        let b = 1.std_seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00 UTC),
            [
                |datetime| *datetime += a,
                |datetime| *datetime += b,
            ]
        );
    }

    fn sub_duration(ben: &mut Bencher<'_>) {
        let a = 5.days();
        let b = 1.days();
        let c = 2.seconds();

        iter_all!(ben, [
            || datetime!(2019-01-06 0:00 UTC) - a,
            || datetime!(2020-01-01 0:00 UTC) - b,
            || datetime!(2020-01-01 0:00:01 UTC) - c,
        ]);
    }

    fn sub_std_duration(ben: &mut Bencher<'_>) {
        let a = 5.std_days();
        let b = 1.std_days();
        let c = 2.std_seconds();

        iter_all!(ben, [
            || datetime!(2019-01-06 0:00 UTC) - a,
            || datetime!(2020-01-01 0:00 UTC) - b,
            || datetime!(2020-01-01 0:00:01 UTC) - c,
        ]);
    }

    fn sub_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.days();
        let b = 1.seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00 UTC),
            [
                |datetime| *datetime -= a,
                |datetime| *datetime -= b,
            ]
        );
    }

    fn sub_assign_std_duration(ben: &mut Bencher<'_>) {
        let a = 1.std_days();
        let b = 1.std_seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00 UTC),
            [
                |datetime| *datetime -= a,
                |datetime| *datetime -= b,
            ]
        );
    }

    fn std_add_duration(ben: &mut Bencher<'_>) {
        let a1 = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let a2 = 0.seconds();
        let b1 = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let b2 = 5.days();
        let c1 = SystemTime::from(datetime!(2019-12-31 0:00 UTC));
        let c2 = 1.days();
        let d1 = SystemTime::from(datetime!(2019-12-31 23:59:59 UTC));
        let d2 = 2.seconds();
        let e1 = SystemTime::from(datetime!(2020-01-01 0:00:01 UTC));
        let e2 = (-2).seconds();
        iter_all_repeated!(ben, [
            || a1 + a2,
            || b1 + b2,
            || c1 + c2,
            || d1 + d2,
            || e1 + e2,
        ]);
    }

    fn std_add_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.days();
        let b = 1.seconds();
        iter_batched_ref!(
            ben,
            || SystemTime::from(datetime!(2019-01-01 0:00 UTC)),
            [
                |datetime| *datetime += a,
                |datetime| *datetime += b,
            ]
        );
    }

    fn std_sub_duration(ben: &mut Bencher<'_>) {
        let a1 = SystemTime::from(datetime!(2019-01-06 0:00 UTC));
        let a2 = 5.days();
        let b1 = SystemTime::from(datetime!(2020-01-01 0:00 UTC));
        let b2 = 1.days();
        let c1 = SystemTime::from(datetime!(2020-01-01 0:00:01 UTC));
        let c2 = 2.seconds();
        let d1 = SystemTime::from(datetime!(2019-12-31 23:59:59 UTC));
        let d2 = (-2).seconds();
        iter_all_repeated!(ben, [
            || a1 - a2,
            || b1 - b2,
            || c1 - c2,
            || d1 - d2,
        ]);
    }

    fn std_sub_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.days();
        let b = 1.seconds();
        iter_batched_ref!(
            ben,
            || SystemTime::from(datetime!(2019-01-01 0:00 UTC)),
            [
                |datetime| *datetime -= a,
                |datetime| *datetime -= b,
            ]
        );
    }

    fn sub_self(ben: &mut Bencher<'_>) {
        iter_all!(ben, [
            || datetime!(2019-01-02 0:00 UTC) - datetime!(2019-01-01 0:00 UTC),
            || datetime!(2019-01-01 0:00 UTC) - datetime!(2019-01-02 0:00 UTC),
            || datetime!(2020-01-01 0:00 UTC) - datetime!(2019-12-31 0:00 UTC),
            || datetime!(2019-12-31 0:00 UTC) - datetime!(2020-01-01 0:00 UTC),
        ]);
    }

    fn std_sub(ben: &mut Bencher<'_>) {
        let a = SystemTime::from(datetime!(2019-01-02 0:00 UTC));
        let b = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let c = SystemTime::from(datetime!(2020-01-01 0:00 UTC));
        let d = SystemTime::from(datetime!(2019-12-31 0:00 UTC));

        iter_all!(ben, [
            || a - datetime!(2019-01-01 0:00 UTC),
            || b - datetime!(2019-01-02 0:00 UTC),
            || c - datetime!(2019-12-31 0:00 UTC),
            || d - datetime!(2020-01-01 0:00 UTC),
        ]);
    }

    fn sub_std(ben: &mut Bencher<'_>) {
        let a = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let b = SystemTime::from(datetime!(2019-01-02 0:00 UTC));
        let c = SystemTime::from(datetime!(2019-12-31 0:00 UTC));
        let d = SystemTime::from(datetime!(2020-01-01 0:00 UTC));

        iter_all!(ben, [
            || datetime!(2019-01-02 0:00 UTC) - a,
            || datetime!(2019-01-01 0:00 UTC) - b,
            || datetime!(2020-01-01 0:00 UTC) - c,
            || datetime!(2019-12-31 0:00 UTC) - d,
        ]);
    }

    fn eq_std(ben: &mut Bencher<'_>) {
        let a = OffsetDateTime::now_utc();
        let b = SystemTime::from(a);
        ben.iter(|| a == b);
    }

    fn std_eq(ben: &mut Bencher<'_>) {
        let a = OffsetDateTime::now_utc();
        let b = SystemTime::from(a);
        ben.iter(|| b == a);
    }

    fn ord_std(ben: &mut Bencher<'_>) {
        let a = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let b = SystemTime::from(datetime!(2020-01-01 0:00 UTC));
        let c = SystemTime::from(datetime!(2019-02-01 0:00 UTC));
        let d = SystemTime::from(datetime!(2019-01-02 0:00 UTC));
        let e = SystemTime::from(datetime!(2019-01-01 1:00:00 UTC));
        let f = SystemTime::from(datetime!(2019-01-01 0:01:00 UTC));
        let g = SystemTime::from(datetime!(2019-01-01 0:00:01 UTC));
        let h = SystemTime::from(datetime!(2019-01-01 0:00:00.001 UTC));
        let i = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let j = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let k = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let l = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let m = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let n = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let o = SystemTime::from(datetime!(2019-01-01 0:00 UTC));

        iter_all!(ben, [
            || datetime!(2019-01-01 0:00 UTC) == a,
            || datetime!(2019-01-01 0:00 UTC) < b,
            || datetime!(2019-01-01 0:00 UTC) < c,
            || datetime!(2019-01-01 0:00 UTC) < d,
            || datetime!(2019-01-01 0:00 UTC) < e,
            || datetime!(2019-01-01 0:00 UTC) < f,
            || datetime!(2019-01-01 0:00 UTC) < g,
            || datetime!(2019-01-01 0:00 UTC) < h,
            || datetime!(2020-01-01 0:00 UTC) > i,
            || datetime!(2019-02-01 0:00 UTC) > j,
            || datetime!(2019-01-02 0:00 UTC) > k,
            || datetime!(2019-01-01 1:00:00 UTC) > l,
            || datetime!(2019-01-01 0:01:00 UTC) > m,
            || datetime!(2019-01-01 0:00:01 UTC) > n,
            || datetime!(2019-01-01 0:00:00.000_000_001 UTC) > o,
        ]);
    }

    fn std_ord(ben: &mut Bencher<'_>) {
        let a = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let b = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let c = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let d = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let e = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let f = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let g = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let h = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
        let i = SystemTime::from(datetime!(2020-01-01 0:00 UTC));
        let j = SystemTime::from(datetime!(2019-02-01 0:00 UTC));
        let k = SystemTime::from(datetime!(2019-01-02 0:00 UTC));
        let l = SystemTime::from(datetime!(2019-01-01 1:00:00 UTC));
        let m = SystemTime::from(datetime!(2019-01-01 0:01:00 UTC));
        let n = SystemTime::from(datetime!(2019-01-01 0:00:01 UTC));
        let o = SystemTime::from(datetime!(2019-01-01 0:00:00.001 UTC));

        iter_all!(ben, [
            || a == datetime!(2019-01-01 0:00 UTC),
            || b < datetime!(2020-01-01 0:00 UTC),
            || c < datetime!(2019-02-01 0:00 UTC),
            || d < datetime!(2019-01-02 0:00 UTC),
            || e < datetime!(2019-01-01 1:00:00 UTC),
            || f < datetime!(2019-01-01 0:01:00 UTC),
            || g < datetime!(2019-01-01 0:00:01 UTC),
            || h < datetime!(2019-01-01 0:00:00.000_000_001 UTC),
            || i > datetime!(2019-01-01 0:00 UTC),
            || j > datetime!(2019-01-01 0:00 UTC),
            || k > datetime!(2019-01-01 0:00 UTC),
            || l > datetime!(2019-01-01 0:00 UTC),
            || m > datetime!(2019-01-01 0:00 UTC),
            || n > datetime!(2019-01-01 0:00 UTC),
            || o > datetime!(2019-01-01 0:00 UTC),
        ]);
    }

    fn from_std(ben: &mut Bencher<'_>) {
        let a = SystemTime::UNIX_EPOCH;
        let b = SystemTime::UNIX_EPOCH - 1.std_days();
        let c = SystemTime::UNIX_EPOCH + 1.std_days();
        iter_all!(ben, [
            || OffsetDateTime::from(a),
            || OffsetDateTime::from(b),
            || OffsetDateTime::from(c),
        ]);
    }

    fn to_std(ben: &mut Bencher<'_>) {
        let a = OffsetDateTime::UNIX_EPOCH;
        let b = OffsetDateTime::UNIX_EPOCH + 1.days();
        let c = OffsetDateTime::UNIX_EPOCH - 1.days();
        iter_all!(ben, [
            || SystemTime::from(a),
            || SystemTime::from(b),
            || SystemTime::from(c),
        ]);
    }
}
