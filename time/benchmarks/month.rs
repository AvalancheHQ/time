use std::hint::black_box;

use criterion::Bencher;
use time::Month::*;

setup_benchmark! {
    "Month",

    fn previous(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(January).previous(),
            || black_box(February).previous(),
            || black_box(March).previous(),
            || black_box(April).previous(),
            || black_box(May).previous(),
            || black_box(June).previous(),
            || black_box(July).previous(),
            || black_box(August).previous(),
            || black_box(September).previous(),
            || black_box(October).previous(),
            || black_box(November).previous(),
            || black_box(December).previous(),
        ]);
    }

    fn next(ben: &mut Bencher<'_>) {
        iter_all_repeated!(ben, [
            || black_box(January).next(),
            || black_box(February).next(),
            || black_box(March).next(),
            || black_box(April).next(),
            || black_box(May).next(),
            || black_box(June).next(),
            || black_box(July).next(),
            || black_box(August).next(),
            || black_box(September).next(),
            || black_box(October).next(),
            || black_box(November).next(),
            || black_box(December).next(),
        ]);
    }

    fn length(ben: &mut Bencher<'_>) {
        // Common year
        iter_all_repeated!(ben, [
            || black_box(January).length(black_box(2019)),
            || black_box(February).length(black_box(2019)),
            || black_box(March).length(black_box(2019)),
            || black_box(April).length(black_box(2019)),
            || black_box(May).length(black_box(2019)),
            || black_box(June).length(black_box(2019)),
            || black_box(July).length(black_box(2019)),
            || black_box(August).length(black_box(2019)),
            || black_box(September).length(black_box(2019)),
            || black_box(October).length(black_box(2019)),
            || black_box(November).length(black_box(2019)),
            || black_box(December).length(black_box(2019)),

            // Leap year
            || black_box(January).length(black_box(2020)),
            || black_box(February).length(black_box(2020)),
            || black_box(March).length(black_box(2020)),
            || black_box(April).length(black_box(2020)),
            || black_box(May).length(black_box(2020)),
            || black_box(June).length(black_box(2020)),
            || black_box(July).length(black_box(2020)),
            || black_box(August).length(black_box(2020)),
            || black_box(September).length(black_box(2020)),
            || black_box(October).length(black_box(2020)),
            || black_box(November).length(black_box(2020)),
            || black_box(December).length(black_box(2020)),
        ]);
    }
}
