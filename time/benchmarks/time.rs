use std::hint::black_box;

use criterion::Bencher;
use time::Time;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::time;

setup_benchmark! {
    "Time",

    fn from_hms(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || Time::from_hms(black_box(1), black_box(2), black_box(3)),
        ]);
    }

    fn from_hms_milli(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || Time::from_hms_milli(black_box(1), black_box(2), black_box(3), black_box(4)),
        ]);
    }

    fn from_hms_micro(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || Time::from_hms_micro(black_box(1), black_box(2), black_box(3), black_box(4)),
        ]);
    }

    fn from_hms_nano(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || Time::from_hms_nano(black_box(1), black_box(2), black_box(3), black_box(4)),
        ]);
    }

    fn as_hms(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).as_hms(),
        ]);
    }

    fn as_hms_milli(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).as_hms_milli(),
        ]);
    }

    fn as_hms_micro(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).as_hms_micro(),
        ]);
    }

    fn as_hms_nano(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).as_hms_nano(),
        ]);
    }

    fn hour(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).hour(),
        ]);
    }

    fn minute(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).minute(),
        ]);
    }

    fn second(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).second(),
        ]);
    }

    fn millisecond(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).millisecond(),
        ]);
    }

    fn microsecond(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).microsecond(),
        ]);
    }

    fn nanosecond(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT).nanosecond(),
        ]);
    }

    fn add_duration(ben: &mut Bencher<'_>) {
        let a = black_box(1.milliseconds());
        let b = black_box(1.seconds());
        let c = black_box(1.minutes());
        let d = black_box(1.hours());
        let e = black_box(1.days());
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT) + a,
            || black_box(Time::MIDNIGHT) + b,
            || black_box(Time::MIDNIGHT) + c,
            || black_box(Time::MIDNIGHT) + d,
            || black_box(Time::MIDNIGHT) + e,
        ]);
    }

    fn add_assign_duration(ben: &mut Bencher<'_>) {
        let a = black_box(1.milliseconds());
        let b = black_box(1.seconds());
        let c = black_box(1.minutes());
        let d = black_box(1.hours());
        let e = black_box(1.days());
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time += a,
                |time| *time += b,
                |time| *time += c,
                |time| *time += d,
                |time| *time += e,
            ]
        );
    }

    fn sub_duration(ben: &mut Bencher<'_>) {
        let a = black_box(1.milliseconds());
        let b = black_box(1.seconds());
        let c = black_box(1.minutes());
        let d = black_box(1.hours());
        let e = black_box(1.days());
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT) - a,
            || black_box(Time::MIDNIGHT) - b,
            || black_box(Time::MIDNIGHT) - c,
            || black_box(Time::MIDNIGHT) - d,
            || black_box(Time::MIDNIGHT) - e,
        ]);
    }

    fn sub_assign_duration(ben: &mut Bencher<'_>) {
        let a = black_box(1.milliseconds());
        let b = black_box(1.seconds());
        let c = black_box(1.minutes());
        let d = black_box(1.hours());
        let e = black_box(1.days());
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time -= a,
                |time| *time -= b,
                |time| *time -= c,
                |time| *time -= d,
                |time| *time -= e,
            ]
        );
    }

    fn add_std_duration(ben: &mut Bencher<'_>) {
        let a = black_box(1.std_milliseconds());
        let b = black_box(1.std_seconds());
        let c = black_box(1.std_minutes());
        let d = black_box(1.std_hours());
        let e = black_box(1.std_days());
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT) + a,
            || black_box(Time::MIDNIGHT) + b,
            || black_box(Time::MIDNIGHT) + c,
            || black_box(Time::MIDNIGHT) + d,
            || black_box(Time::MIDNIGHT) + e,
        ]);
    }

    fn add_assign_std_duration(ben: &mut Bencher<'_>) {
        let a = black_box(1.std_milliseconds());
        let b = black_box(1.std_seconds());
        let c = black_box(1.std_minutes());
        let d = black_box(1.std_hours());
        let e = black_box(1.std_days());
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time += a,
                |time| *time += b,
                |time| *time += c,
                |time| *time += d,
                |time| *time += e,
            ]
        );
    }

    fn sub_std_duration(ben: &mut Bencher<'_>) {
        let a = black_box(1.std_milliseconds());
        let b = black_box(1.std_seconds());
        let c = black_box(1.std_minutes());
        let d = black_box(1.std_hours());
        let e = black_box(1.std_days());
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT) - a,
            || black_box(Time::MIDNIGHT) - b,
            || black_box(Time::MIDNIGHT) - c,
            || black_box(Time::MIDNIGHT) - d,
            || black_box(Time::MIDNIGHT) - e,
        ]);
    }

    fn sub_assign_std_duration(ben: &mut Bencher<'_>) {
        let a = black_box(1.std_milliseconds());
        let b = black_box(1.std_seconds());
        let c = black_box(1.std_minutes());
        let d = black_box(1.std_hours());
        let e = black_box(1.std_days());
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time -= a,
                |time| *time -= b,
                |time| *time -= c,
                |time| *time -= d,
                |time| *time -= e,
            ]
        );
    }

    fn sub_time(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT) - black_box(time!(0:00:01)),
            || black_box(time!(1:00)) - black_box(Time::MIDNIGHT),
            || black_box(time!(1:00)) - black_box(time!(0:00:01)),
        ]);
    }

    fn ordering(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Time::MIDNIGHT) < black_box(time!(0:00:00.000_000_001)),
            || black_box(Time::MIDNIGHT) < black_box(time!(0:00:01)),
            || black_box(time!(12:00)) > black_box(time!(11:00)),
            || black_box(Time::MIDNIGHT) == black_box(time!(0:00:00.000_000_001)),
        ]);
    }

    fn sort_align_8(ben: &mut Bencher<'_>) {
        ben.iter_batched_ref(
            || {
                #[repr(C,align(8))]
                struct Padder {
                    arr: [Time;4096],
                }
                let mut res = Padder {
                    arr: [Time::MIDNIGHT;4096]
                };
                let mut last = Time::MIDNIGHT;
                let mut last_hour = 0;
                for t in &mut res.arr {
                    *t = last;
                    t.replace_hour(last_hour).expect("failed to replace hour");
                    last += 997.std_milliseconds();
                    last_hour = (last_hour + 5) % 24;
                }
                res.arr.sort_unstable_by_key(|t|
                    (t.nanosecond(),t.second(),t.minute(),t.hour())
                );
                res
            },
            |v| black_box(v).arr.sort_unstable(),
            criterion::BatchSize::SmallInput
        )
    }

    fn sort_align_4(ben: &mut Bencher<'_>) {
        ben.iter_batched_ref(
            || {
                #[repr(C,align(8))]
                struct Padder {
                    pad: u32,
                    arr: [Time;4096],
                }
                let mut res = Padder {
                    pad: 0,
                    arr: [Time::MIDNIGHT;4096]
                };
                let mut last = Time::MIDNIGHT;
                let mut last_hour = 0;
                for t in &mut res.arr {
                    *t = last;
                    t.replace_hour(last_hour).expect("failed to replace hour");
                    last += 997.std_milliseconds();
                    last_hour = (last_hour + 5) % 24;
                }
                res.arr.sort_unstable_by_key(|t|
                    (t.nanosecond(),t.second(),t.minute(),t.hour())
                );
                res
            },
            |v| black_box(v).arr.sort_unstable(),
            criterion::BatchSize::SmallInput
        )
    }

    fn duration_until(ben: &mut Bencher<'_>) {
        let a = black_box(time!(1:02:03.004_005_006));
        let b = black_box(time!(4:05:06.007_008_009));
        iter_all_repeated!(ben, [
            || black_box(a).duration_until(black_box(b)),
        ]);
    }
}
