use std::hint::black_box;
use std::time::Duration as StdDuration;

use criterion::Bencher;
use time::SignedDuration;
use time::ext::{NumericalDuration, NumericalStdDuration};

setup_benchmark! {
    "SignedDuration",

    fn is_zero(ben: &mut Bencher<'_>) {
        let a = black_box((-1).nanoseconds());
        let b = black_box(0.seconds());
        let c = black_box(1.nanoseconds());
        iter_all_repeated!(ben, [
            || a.is_zero(),
            || b.is_zero(),
            || c.is_zero(),
        ]);
    }

    fn is_negative(ben: &mut Bencher<'_>) {
        let a = black_box((-1).seconds());
        let b = black_box(0.seconds());
        let c = black_box(1.seconds());
        iter_all_repeated!(ben, [
            || a.is_negative(),
            || b.is_negative(),
            || c.is_negative(),
        ]);
    }

    fn is_positive(ben: &mut Bencher<'_>) {
        let a = black_box((-1).seconds());
        let b = black_box(0.seconds());
        let c = black_box(1.seconds());
        iter_all_repeated!(ben, [
            || a.is_positive(),
            || b.is_positive(),
            || c.is_positive(),
        ]);
    }

    fn abs(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(0.seconds());
        let c = black_box((-1).seconds());
        iter_all_repeated!(ben, [
            || a.abs(),
            || b.abs(),
            || c.abs(),
        ]);
    }

    fn unsigned_abs(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(0.seconds());
        let c = black_box((-1).seconds());
        iter_all_repeated!(ben, [
            || a.unsigned_abs(),
            || b.unsigned_abs(),
            || c.unsigned_abs(),
        ]);
    }

    fn new(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::new(black_box(1), black_box(0)),
            || SignedDuration::new(black_box(-1), black_box(0)),
            || SignedDuration::new(black_box(1), black_box(2_000_000_000)),

            || SignedDuration::new(black_box(0), black_box(0)),
            || SignedDuration::new(black_box(0), black_box(1_000_000_000)),
            || SignedDuration::new(black_box(-1), black_box(1_000_000_000)),
            || SignedDuration::new(black_box(-2), black_box(1_000_000_000)),

            || SignedDuration::new(black_box(1), black_box(-1)),
            || SignedDuration::new(black_box(-1), black_box(1)),
            || SignedDuration::new(black_box(1), black_box(1)),
            || SignedDuration::new(black_box(-1), black_box(-1)),
            || SignedDuration::new(black_box(0), black_box(1)),
            || SignedDuration::new(black_box(0), black_box(-1)),

            || SignedDuration::new(black_box(-1), black_box(1_400_000_000)),
            || SignedDuration::new(black_box(-2), black_box(1_400_000_000)),
            || SignedDuration::new(black_box(-3), black_box(1_400_000_000)),
            || SignedDuration::new(black_box(1), black_box(-1_400_000_000)),
            || SignedDuration::new(black_box(2), black_box(-1_400_000_000)),
            || SignedDuration::new(black_box(3), black_box(-1_400_000_000)),
        ]);
    }

    fn weeks(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::weeks(black_box(1)),
            || SignedDuration::weeks(black_box(2)),
            || SignedDuration::weeks(black_box(-1)),
            || SignedDuration::weeks(black_box(-2)),
        ]);
    }

    fn days(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::days(black_box(1)),
            || SignedDuration::days(black_box(2)),
            || SignedDuration::days(black_box(-1)),
            || SignedDuration::days(black_box(-2)),
        ]);
    }

    fn hours(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::hours(black_box(1)),
            || SignedDuration::hours(black_box(2)),
            || SignedDuration::hours(black_box(-1)),
            || SignedDuration::hours(black_box(-2)),
        ]);
    }

    fn minutes(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::minutes(black_box(1)),
            || SignedDuration::minutes(black_box(2)),
            || SignedDuration::minutes(black_box(-1)),
            || SignedDuration::minutes(black_box(-2)),
        ]);
    }

    fn seconds(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::seconds(black_box(1)),
            || SignedDuration::seconds(black_box(2)),
            || SignedDuration::seconds(black_box(-1)),
            || SignedDuration::seconds(black_box(-2)),
        ]);
    }

    fn seconds_f64(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::seconds_f64(black_box(0.5)),
            || SignedDuration::seconds_f64(black_box(-0.5)),
        ]);
    }

    fn seconds_f32(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::seconds_f32(black_box(0.5)),
            || SignedDuration::seconds_f32(black_box(-0.5)),
        ]);
    }

    fn saturating_seconds_f64(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::saturating_seconds_f64(black_box(0.5)),
            || SignedDuration::saturating_seconds_f64(black_box(-0.5)),
        ]);
    }

    fn saturating_seconds_f32(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::saturating_seconds_f32(black_box(0.5)),
            || SignedDuration::saturating_seconds_f32(black_box(-0.5)),
        ]);
    }

    fn checked_seconds_f64(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::checked_seconds_f64(black_box(0.5)),
            || SignedDuration::checked_seconds_f64(black_box(-0.5)),
        ]);
    }

    fn checked_seconds_f32(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::checked_seconds_f32(black_box(0.5)),
            || SignedDuration::checked_seconds_f32(black_box(-0.5)),
        ]);
    }

    fn milliseconds(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::milliseconds(black_box(1)),
            || SignedDuration::milliseconds(black_box(-1)),
        ]);
    }

    fn microseconds(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::microseconds(black_box(1)),
            || SignedDuration::microseconds(black_box(-1)),
        ]);
    }

    fn nanoseconds(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || SignedDuration::nanoseconds(black_box(1)),
            || SignedDuration::nanoseconds(black_box(-1)),
        ]);
    }

    fn whole_weeks(ben: &mut Bencher<'_>) {
        let a = black_box(SignedDuration::weeks(1));
        let b = black_box(SignedDuration::weeks(-1));
        let c = black_box(SignedDuration::days(6));
        let d = black_box(SignedDuration::days(-6));
        iter_all_repeated!(ben, [
            || a.whole_weeks(),
            || b.whole_weeks(),
            || c.whole_weeks(),
            || d.whole_weeks(),
        ]);
    }

    fn whole_days(ben: &mut Bencher<'_>) {
        let a = black_box(SignedDuration::days(1));
        let b = black_box(SignedDuration::days(-1));
        let c = black_box(SignedDuration::hours(23));
        let d = black_box(SignedDuration::hours(-23));
        iter_all_repeated!(ben, [
            || a.whole_days(),
            || b.whole_days(),
            || c.whole_days(),
            || d.whole_days(),
        ]);
    }

    fn whole_hours(ben: &mut Bencher<'_>) {
        let a = black_box(SignedDuration::hours(1));
        let b = black_box(SignedDuration::hours(-1));
        let c = black_box(SignedDuration::minutes(59));
        let d = black_box(SignedDuration::minutes(-59));
        iter_all_repeated!(ben, [
            || a.whole_hours(),
            || b.whole_hours(),
            || c.whole_hours(),
            || d.whole_hours(),
        ]);
    }

    fn whole_minutes(ben: &mut Bencher<'_>) {
        let a = black_box(1.minutes());
        let b = black_box((-1).minutes());
        let c = black_box(59.seconds());
        let d = black_box((-59).seconds());
        iter_all_repeated!(ben, [
            || a.whole_minutes(),
            || b.whole_minutes(),
            || c.whole_minutes(),
            || d.whole_minutes(),
        ]);
    }

    fn whole_seconds(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box((-1).seconds());
        let c = black_box(1.minutes());
        let d = black_box((-1).minutes());
        iter_all_repeated!(ben, [
            || a.whole_seconds(),
            || b.whole_seconds(),
            || c.whole_seconds(),
            || d.whole_seconds(),
        ]);
    }

    fn as_seconds_f64(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box((-1).seconds());
        let c = black_box(1.minutes());
        let d = black_box((-1).minutes());
        let e = black_box(1.5.seconds());
        let f = black_box((-1.5).seconds());
        iter_all_repeated!(ben, [
            || a.as_seconds_f64(),
            || b.as_seconds_f64(),
            || c.as_seconds_f64(),
            || d.as_seconds_f64(),
            || e.as_seconds_f64(),
            || f.as_seconds_f64(),
        ]);
    }

    fn as_seconds_f32(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box((-1).seconds());
        let c = black_box(1.minutes());
        let d = black_box((-1).minutes());
        let e = black_box(1.5.seconds());
        let f = black_box((-1.5).seconds());
        iter_all_repeated!(ben, [
            || a.as_seconds_f32(),
            || b.as_seconds_f32(),
            || c.as_seconds_f32(),
            || d.as_seconds_f32(),
            || e.as_seconds_f32(),
            || f.as_seconds_f32(),
        ]);
    }

    fn whole_milliseconds(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box((-1).seconds());
        let c = black_box(1.milliseconds());
        let d = black_box((-1).milliseconds());
        iter_all_repeated!(ben, [
            || a.whole_milliseconds(),
            || b.whole_milliseconds(),
            || c.whole_milliseconds(),
            || d.whole_milliseconds(),
        ]);
    }

    fn subsec_milliseconds(ben: &mut Bencher<'_>) {
        let a = black_box(1.4.seconds());
        let b = black_box((-1.4).seconds());
        iter_all_repeated!(ben, [
            || a.subsec_milliseconds(),
            || b.subsec_milliseconds(),
        ]);
    }

    fn whole_microseconds(ben: &mut Bencher<'_>) {
        let a = black_box(1.milliseconds());
        let b = black_box((-1).milliseconds());
        let c = black_box(1.microseconds());
        let d = black_box((-1).microseconds());
        iter_all_repeated!(ben, [
            || a.whole_microseconds(),
            || b.whole_microseconds(),
            || c.whole_microseconds(),
            || d.whole_microseconds(),
        ]);
    }

    fn subsec_microseconds(ben: &mut Bencher<'_>) {
        let a = black_box(1.0004.seconds());
        let b = black_box((-1.0004).seconds());
        iter_all_repeated!(ben, [
            || a.subsec_microseconds(),
            || b.subsec_microseconds(),
        ]);
    }

    fn whole_nanoseconds(ben: &mut Bencher<'_>) {
        let a = black_box(1.microseconds());
        let b = black_box((-1).microseconds());
        let c = black_box(1.nanoseconds());
        let d = black_box((-1).nanoseconds());
        iter_all_repeated!(ben, [
            || a.whole_nanoseconds(),
            || b.whole_nanoseconds(),
            || c.whole_nanoseconds(),
            || d.whole_nanoseconds(),
        ]);
    }

    fn subsec_nanoseconds(ben: &mut Bencher<'_>) {
        let a = black_box(1.000_000_4.seconds());
        let b = black_box((-1.000_000_4).seconds());
        iter_all_repeated!(ben, [
            || a.subsec_nanoseconds(),
            || b.subsec_nanoseconds(),
        ]);
    }

    fn checked_add(ben: &mut Bencher<'_>) {
        let a = black_box(5.seconds());
        let b = black_box(SignedDuration::MAX);
        let c = black_box((-5).seconds());

        let a2 = black_box(5.seconds());
        let b2 = black_box(1.nanoseconds());
        let c2 = black_box(5.seconds());

        iter_all_repeated!(ben, [
            || a.checked_add(a2),
            || b.checked_add(b2),
            || c.checked_add(c2),
        ]);
    }

    fn checked_sub(ben: &mut Bencher<'_>) {
        let a = black_box(5.seconds());
        let b = black_box(SignedDuration::MIN);
        let c = black_box(5.seconds());

        let a2 = black_box(5.seconds());
        let b2 = black_box(1.nanoseconds());
        let c2 = black_box(10.seconds());

        iter_all_repeated!(ben, [
            || a.checked_sub(a2),
            || b.checked_sub(b2),
            || c.checked_sub(c2),
        ]);
    }

    fn checked_mul(ben: &mut Bencher<'_>) {
        let a = black_box(5.seconds());
        let b = black_box(SignedDuration::MAX);
        iter_all_repeated!(ben, [
            || a.checked_mul(black_box(2)),
            || b.checked_mul(black_box(2)),
        ]);
    }

    fn checked_div(ben: &mut Bencher<'_>) {
        let a = black_box(10.seconds());
        iter_all_repeated!(ben, [
            || a.checked_div(black_box(2)),
            || a.checked_div(black_box(0)),
        ]);
    }

    fn saturating_add(ben: &mut Bencher<'_>) {
        let a = black_box(5.seconds());
        let b = black_box(SignedDuration::MAX);
        let c = black_box(SignedDuration::MIN);
        let d = black_box((-5).seconds());

        let a2 = black_box(5.seconds());
        let b2 = black_box(1.nanoseconds());
        let c2 = black_box((-1).nanoseconds());
        let d2 = black_box(5.seconds());

        iter_all_repeated!(ben, [
            || a.saturating_add(a2),
            || b.saturating_add(b2),
            || c.saturating_add(c2),
            || d.saturating_add(d2),
        ]);
    }

    fn saturating_sub(ben: &mut Bencher<'_>) {
        let a = black_box(5.seconds());
        let b = black_box(SignedDuration::MIN);
        let c = black_box(SignedDuration::MAX);
        let d = black_box(5.seconds());

        let a2 = black_box(5.seconds());
        let b2 = black_box(1.nanoseconds());
        let c2 = black_box((-1).nanoseconds());
        let d2 = black_box(10.seconds());

        iter_all_repeated!(ben, [
            || a.saturating_sub(a2),
            || b.saturating_sub(b2),
            || c.saturating_sub(c2),
            || d.saturating_sub(d2),
        ]);
    }

    fn saturating_mul(ben: &mut Bencher<'_>) {
        let a = black_box(5.seconds());
        let b = black_box(5.seconds());
        let c = black_box(5.seconds());
        let d = black_box(SignedDuration::MAX);
        let e = black_box(SignedDuration::MIN);
        let f = black_box(SignedDuration::MAX);
        let g = black_box(SignedDuration::MIN);

        iter_all_repeated!(ben, [
            || a.saturating_mul(black_box(2)),
            || b.saturating_mul(black_box(-2)),
            || c.saturating_mul(black_box(0)),
            || d.saturating_mul(black_box(2)),
            || e.saturating_mul(black_box(2)),
            || f.saturating_mul(black_box(-2)),
            || g.saturating_mul(black_box(-2)),
        ]);
    }

    fn try_from_std_duration(ben: &mut Bencher<'_>) {
        let a = black_box(0.std_seconds());
        let b = black_box(1.std_seconds());
        iter_all_repeated!(ben, [
            || SignedDuration::try_from(black_box(a)),
            || SignedDuration::try_from(black_box(b)),
        ]);
    }

    fn try_to_std_duration(ben: &mut Bencher<'_>) {
        let a = black_box(0.seconds());
        let b = black_box(1.seconds());
        let c = black_box((-1).seconds());
        iter_all_repeated!(ben, [
            || StdDuration::try_from(a),
            || StdDuration::try_from(b),
            || StdDuration::try_from(c),
        ]);
    }

    fn add(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(2.seconds());
        let c = black_box(500.milliseconds());
        let d = black_box((-1).seconds());
        ben.iter(|| a + b + c + d);
    }

    fn add_std(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(2.std_seconds());
        iter_all_repeated!(ben, [
            || a + b,
        ]);
    }

    fn std_add(ben: &mut Bencher<'_>) {
        let a = black_box(1.std_seconds());
        let b = black_box(2.seconds());
        iter_all_repeated!(ben, [
            || a + b,
        ]);
    }

    fn add_assign(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(500.milliseconds());
        let c = black_box((-1).seconds());
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration += a,
                |duration| *duration += b,
                |duration| *duration += c,
            ]
        );
    }

    fn add_assign_std(ben: &mut Bencher<'_>) {
        let a = black_box(1.std_seconds());
        let b = black_box(500.std_milliseconds());
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration += a,
                |duration| *duration += b,
            ]
        );
    }

    fn neg(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box((-1).seconds());
        let c = black_box(0.seconds());
        iter_all_repeated!(ben, [
            || -a,
            || -b,
            || -c,
        ]);
    }

    fn sub(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(1.seconds());
        let c = black_box(1_500.milliseconds());
        let d = black_box(500.milliseconds());
        let e = black_box(1.seconds());
        let f = black_box((-1).seconds());
        iter_all_repeated!(ben, [
            || a - b,
            || b - c,
            || c - d,
            || d - e,
            || e - f,
            || f - a,
        ]);
    }

    fn sub_std(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(2.std_seconds());
        iter_all_repeated!(ben, [
            || black_box(a) - black_box(b),
        ]);
    }

    fn std_sub(ben: &mut Bencher<'_>) {
        let a = black_box(1.std_seconds());
        let b = black_box(2.seconds());
        iter_all_repeated!(ben, [
            || a - b,
        ]);
    }

    fn sub_assign(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(500.milliseconds());
        let c = black_box((-1).seconds());
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration -= a,
                |duration| *duration -= b,
                |duration| *duration -= c,
            ]
        );
    }

    fn mul_int(ben: &mut Bencher<'_>) {
        let d = black_box(1.seconds());
        iter_all_repeated!(ben, [
            || d * black_box(2),
            || d * black_box(-2),
        ]);
    }

    fn mul_int_assign(ben: &mut Bencher<'_>) {
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration *= 2,
                |duration| *duration *= -2,
            ]
        );
    }

    fn int_mul(ben: &mut Bencher<'_>) {
        let d = black_box(1.seconds());
        iter_all_repeated!(ben, [
            || black_box(2) * d,
            || black_box(-2) * d,
        ]);
    }

    fn div_int(ben: &mut Bencher<'_>) {
        let d = black_box(1.seconds());
        iter_all_repeated!(ben, [
            || d / black_box(2),
            || d / black_box(-2),
        ]);
    }

    fn div_int_assign(ben: &mut Bencher<'_>) {
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration /= 2,
                |duration| *duration /= -2,
            ]
        );
    }

    fn div(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(0.5.seconds());
        iter_all_repeated!(ben, [
            || a / b,
        ]);
    }

    fn mul_float(ben: &mut Bencher<'_>) {
        let d = black_box(1.seconds());
        iter_all_repeated!(ben, [
            || d * black_box(1.5_f32),
            || d * black_box(2.5_f32),
            || d * black_box(-1.5_f32),
            || d * black_box(0_f32),
            || d * black_box(1.5_f64),
            || d * black_box(2.5_f64),
            || d * black_box(-1.5_f64),
            || d * black_box(0_f64),
        ]);
    }

    fn float_mul(ben: &mut Bencher<'_>) {
        let d = black_box(1.seconds());
        iter_all_repeated!(ben, [
            || black_box(1.5_f32) * d,
            || black_box(2.5_f32) * d,
            || black_box(-1.5_f32) * d,
            || black_box(0_f32) * d,
            || black_box(1.5_f64) * d,
            || black_box(2.5_f64) * d,
            || black_box(-1.5_f64) * d,
            || black_box(0_f64) * d,
        ]);
    }

    fn mul_float_assign(ben: &mut Bencher<'_>) {
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration *= 1.5_f32,
                |duration| *duration *= 2.5_f32,
                |duration| *duration *= -1.5_f32,
                |duration| *duration *= 3.15_f32,
                |duration| *duration *= 1.5_f64,
                |duration| *duration *= 2.5_f64,
                |duration| *duration *= -1.5_f64,
                |duration| *duration *= 0_f64,
            ]
        );
    }

    fn div_float(ben: &mut Bencher<'_>) {
        let d = black_box(1.seconds());
        iter_all!(ben, [
            || d / black_box(1_f32),
            || d / black_box(2_f32),
            || d / black_box(-1_f32),
            || d / black_box(1_f64),
            || d / black_box(2_f64),
            || d / black_box(-1_f64),
        ]);
    }

    fn div_float_assign(ben: &mut Bencher<'_>) {
        iter_batched_ref!(
            ben,
            || 10.seconds(),
            [
                |duration| *duration /= 1_f32,
                |duration| *duration /= 2_f32,
                |duration| *duration /= -1_f32,
                |duration| *duration /= 1_f64,
                |duration| *duration /= 2_f64,
                |duration| *duration /= -1_f64,
            ]
        );
    }

    fn partial_eq(ben: &mut Bencher<'_>) {
        let a = black_box(1.minutes());
        let b = black_box((-1).minutes());
        let c = black_box(40.seconds());
        iter_all_repeated!(ben, [
            || a == b,
            || c == a,
        ]);
    }

    fn partial_eq_std(ben: &mut Bencher<'_>) {
        let a = black_box((-1).seconds());
        let b = black_box(1.std_seconds());
        let c = black_box((-1).minutes());
        let d = black_box(1.std_minutes());
        let e = black_box(40.seconds());
        iter_all_repeated!(ben, [
            || a == b,
            || c == d,
            || e == d,
        ]);
    }

    fn std_partial_eq(ben: &mut Bencher<'_>) {
        let a = black_box(1.std_seconds());
        let b = black_box((-1).seconds());
        let c = black_box(1.std_minutes());
        let d = black_box((-1).minutes());
        let e = black_box(40.std_seconds());
        let f = black_box(1.minutes());
        iter_all_repeated!(ben, [
            || a == b,
            || c == d,
            || e == f,
        ]);
    }

    fn partial_ord(ben: &mut Bencher<'_>) {
        let a = black_box(0.seconds());
        let b = black_box(1.seconds());
        let c = black_box((-1).seconds());
        let d = black_box(1.minutes());
        let e = black_box((-1).minutes());
        iter_all_repeated!(ben, [
            || a.partial_cmp(&a),
            || b.partial_cmp(&a),
            || b.partial_cmp(&c),
            || c.partial_cmp(&b),
            || a.partial_cmp(&c),
            || a.partial_cmp(&b),
            || c.partial_cmp(&a),
            || d.partial_cmp(&b),
            || e.partial_cmp(&c),
        ]);
    }

    fn partial_ord_std(ben: &mut Bencher<'_>) {
        let a = black_box(0.seconds());
        let b = black_box(0.std_seconds());
        let c = black_box(1.seconds());
        let d = black_box((-1).seconds());
        let e = black_box(1.std_seconds());
        let f = black_box(1.minutes());
        let g = black_box(u64::MAX.std_seconds());
        iter_all_repeated!(ben, [
            || a.partial_cmp(&b),
            || c.partial_cmp(&b),
            || d.partial_cmp(&e),
            || a.partial_cmp(&e),
            || d.partial_cmp(&b),
            || f.partial_cmp(&e),
            || a.partial_cmp(&g),
        ]);
    }

    fn std_partial_ord(ben: &mut Bencher<'_>) {
        let a = black_box(0.std_seconds());
        let b = black_box(0.seconds());
        let c = black_box(1.std_seconds());
        let d = black_box((-1).seconds());
        let e = black_box(1.seconds());
        let f = black_box(1.std_minutes());
        iter_all_repeated!(ben, [
            || a.partial_cmp(&b),
            || c.partial_cmp(&b),
            || c.partial_cmp(&d),
            || a.partial_cmp(&d),
            || a.partial_cmp(&e),
            || f.partial_cmp(&e),
        ]);
    }

    fn ord(ben: &mut Bencher<'_>) {
        let a = black_box(1.seconds());
        let b = black_box(0.seconds());
        let c = black_box((-1).seconds());
        let d = black_box(1.minutes());
        let e = black_box((-1).minutes());
        iter_all_repeated!(ben, [
            || a > b,
            || a > c,
            || c < a,
            || b > c,
            || b < a,
            || c < b,
            || d > a,
            || e < c,
        ]);
    }
}
