use std::hint::black_box;

use criterion::Bencher;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::{datetime, offset};

setup_benchmark! {
    "PlainDateTime",

    // All getters are trivially dispatched to the relevant field, and do not need to be benchmarked
    // a second time.

    fn assume_offset(ben: &mut Bencher<'_>) {
        let datetime = black_box(datetime!(2019-01-01 0:00));
        iter_all_repeated!(ben, [
            || datetime.assume_offset(black_box(offset!(UTC))),
            || datetime.assume_offset(black_box(offset!(-1))),
        ]);
    }

    fn assume_utc(ben: &mut Bencher<'_>) {
        let datetime = black_box(datetime!(2019-01-01 0:00));
        iter_all_repeated!(ben, [
            || datetime.assume_utc(),
        ]);
    }

    fn add_duration(ben: &mut Bencher<'_>) {
        let a = 5.days();
        let b = 1.days();
        let c = 2.seconds();
        let d = (-2).seconds();
        let e = 1.hours();

        iter_all!(ben, [
            || datetime!(2019-01-01 0:00) + a,
            || datetime!(2019-12-31 0:00) + b,
            || datetime!(2019-12-31 23:59:59) + c,
            || datetime!(2020-01-01 0:00:01) + d,
            || datetime!(1999-12-31 23:00) + e,
        ]);
    }

    fn add_std_duration(ben: &mut Bencher<'_>) {
        let a = 5.std_days();
        let b = 1.std_days();
        let c = 2.std_seconds();

        iter_all!(ben, [
            || datetime!(2019-01-01 0:00) + a,
            || datetime!(2019-12-31 0:00) + b,
            || datetime!(2019-12-31 23:59:59) + c,
        ]);
    }

    fn add_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.days();
        let b = 1.seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00),
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
            || datetime!(2019-01-01 0:00),
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
        let d = (-2).seconds();
        let e = (-1).hours();

        iter_all!(ben, [
            || datetime!(2019-01-06 0:00) - a,
            || datetime!(2020-01-01 0:00) - b,
            || datetime!(2020-01-01 0:00:01) - c,
            || datetime!(2019-12-31 23:59:59) - d,
            || datetime!(1999-12-31 23:00) - e,
        ]);
    }

    fn sub_std_duration(ben: &mut Bencher<'_>) {
        let a = 5.std_days();
        let b = 1.std_days();
        let c = 2.std_seconds();

        iter_all!(ben, [
            || datetime!(2019-01-06 0:00) - a,
            || datetime!(2020-01-01 0:00) - b,
            || datetime!(2020-01-01 0:00:01) - c,
        ]);
    }

    fn sub_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.days();
        let b = 1.seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00),
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
            || datetime!(2019-01-01 0:00),
            [
                |datetime| *datetime -= a,
                |datetime| *datetime -= b,
            ]
        );
    }

    fn sub_datetime(ben: &mut Bencher<'_>) {
        let jan_1 = black_box(datetime!(2019-01-01 0:00));
        let jan_2 = black_box(datetime!(2019-01-02 0:00));
        let dec_31 = black_box(datetime!(2019-12-31 0:00));
        let next_jan_1 = black_box(datetime!(2020-01-01 0:00));

        iter_all_repeated!(ben, [
            || jan_2 - jan_1,
            || jan_1 - jan_2,
            || next_jan_1 - dec_31,
            || dec_31 - next_jan_1,
        ]);
    }

    fn ord(ben: &mut Bencher<'_>) {
        let base = black_box(datetime!(2019-01-01 0:00));
        let year = black_box(datetime!(2020-01-01 0:00));
        let month = black_box(datetime!(2019-02-01 0:00));
        let day = black_box(datetime!(2019-01-02 0:00));
        let hour = black_box(datetime!(2019-01-01 1:00));
        let minute = black_box(datetime!(2019-01-01 0:01));
        let second = black_box(datetime!(2019-01-01 0:00:01));
        let nanosecond = black_box(datetime!(2019-01-01 0:00:00.000_000_001));

        iter_all_repeated!(ben, [
            || base.partial_cmp(&base),
            || base.partial_cmp(&year),
            || base.partial_cmp(&month),
            || base.partial_cmp(&day),
            || base.partial_cmp(&hour),
            || base.partial_cmp(&minute),
            || base.partial_cmp(&second),
            || base.partial_cmp(&nanosecond),
            || year.partial_cmp(&base),
            || month.partial_cmp(&base),
            || day.partial_cmp(&base),
            || hour.partial_cmp(&base),
            || minute.partial_cmp(&base),
            || second.partial_cmp(&base),
            || nanosecond.partial_cmp(&base),
        ]);
    }
}
