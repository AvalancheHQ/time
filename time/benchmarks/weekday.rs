use std::hint::black_box;

use criterion::Bencher;
use time::Weekday::*;

setup_benchmark! {
    "Weekday",

    fn previous(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Sunday).previous(),
            || black_box(Monday).previous(),
            || black_box(Tuesday).previous(),
            || black_box(Wednesday).previous(),
            || black_box(Thursday).previous(),
            || black_box(Friday).previous(),
            || black_box(Saturday).previous(),
        ]);
    }

    fn next(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Sunday).next(),
            || black_box(Monday).next(),
            || black_box(Tuesday).next(),
            || black_box(Wednesday).next(),
            || black_box(Thursday).next(),
            || black_box(Friday).next(),
            || black_box(Saturday).next(),
        ]);
    }

    fn nth(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Sunday).nth_next(black_box(0)),
            || black_box(Sunday).nth_next(black_box(1)),
            || black_box(Sunday).nth_next(black_box(2)),
            || black_box(Sunday).nth_next(black_box(3)),
            || black_box(Sunday).nth_next(black_box(4)),
            || black_box(Sunday).nth_next(black_box(5)),
            || black_box(Sunday).nth_next(black_box(6)),

            || black_box(Sunday).nth_next(black_box(7)),
            || black_box(Sunday).nth_next(black_box(u8::MAX)),
            || black_box(Monday).nth_next(black_box(7)),
            || black_box(Monday).nth_next(black_box(u8::MAX)),
        ]);
    }

    fn number_from_monday(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Monday).number_from_monday(),
            || black_box(Tuesday).number_from_monday(),
            || black_box(Wednesday).number_from_monday(),
            || black_box(Thursday).number_from_monday(),
            || black_box(Friday).number_from_monday(),
            || black_box(Saturday).number_from_monday(),
            || black_box(Sunday).number_from_monday(),
        ]);
    }

    fn number_from_sunday(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Sunday).number_from_sunday(),
            || black_box(Monday).number_from_sunday(),
            || black_box(Tuesday).number_from_sunday(),
            || black_box(Wednesday).number_from_sunday(),
            || black_box(Thursday).number_from_sunday(),
            || black_box(Friday).number_from_sunday(),
            || black_box(Saturday).number_from_sunday(),
        ]);
    }

    fn number_days_from_monday(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Monday).number_days_from_monday(),
            || black_box(Tuesday).number_days_from_monday(),
            || black_box(Wednesday).number_days_from_monday(),
            || black_box(Thursday).number_days_from_monday(),
            || black_box(Friday).number_days_from_monday(),
            || black_box(Saturday).number_days_from_monday(),
            || black_box(Sunday).number_days_from_monday(),
        ]);
    }

    fn number_days_from_sunday(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(Sunday).number_days_from_sunday(),
            || black_box(Monday).number_days_from_sunday(),
            || black_box(Tuesday).number_days_from_sunday(),
            || black_box(Wednesday).number_days_from_sunday(),
            || black_box(Thursday).number_days_from_sunday(),
            || black_box(Friday).number_days_from_sunday(),
            || black_box(Saturday).number_days_from_sunday(),
        ]);
    }
}
