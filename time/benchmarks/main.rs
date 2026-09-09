//! Benchmarks for `time`.
//!
//! These benchmarks are not very precise when run locally, but they're good enough to catch major
//! performance regressions. Run them if you think that may be the case.
//!
//! CI runs these benchmarks on every push and pull request through CodSpeed, which measures them
//! with CPU simulation instead of wall time.

#![allow(
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::alloc_instead_of_core,
    reason = "irrelevant for benchmarks"
)]
#![allow(
    clippy::missing_docs_in_private_items,
    reason = "may be removed in the future"
)]

use std::hint::black_box;

#[cfg(not(all(
    feature = "default",
    feature = "alloc",
    feature = "formatting",
    feature = "large-dates",
    feature = "local-offset",
    feature = "macros",
    feature = "parsing",
    feature = "quickcheck",
    feature = "serde-human-readable",
    feature = "serde-well-known",
    feature = "std",
    feature = "rand",
    feature = "serde",
    bench,
)))]
compile_error!("benchmarks must be run as `RUSTFLAGS=\"--cfg bench\" cargo bench --all-features`");

macro_rules! setup_benchmark {
    (
        $group_prefix:literal,
        $(
            $(#[$fn_attr:meta])*
            fn $fn_name:ident ($bencher:ident : $bencher_type:ty)
            $code:block
        )*
    ) => {
        $(
            $(#[$fn_attr])*
            fn $fn_name(
                c: &mut ::criterion::Criterion
            ) {
                c.bench_function(
                    concat!($group_prefix, ": ", stringify!($fn_name)),
                    |$bencher: $bencher_type| $code
                );
            }
        )*

        ::criterion::criterion_group! {
            name = benches;
            config = ::criterion::Criterion::default()
                // Set a stricter statistical significance threshold ("p-value")
                // for deciding what's an actual performance change vs. noise.
                // The more benchmarks, the lower this needs to be in order to
                // not get lots of false positives.
                .significance_level(0.0001)
                // Ignore any performance change less than this (0.05 = 5%) as
                // noise, regardless of statistical significance.
                .noise_threshold(0.05)
                // Reduce the time taken to run each benchmark
                .warm_up_time(::std::time::Duration::from_millis(100))
                .measurement_time(::std::time::Duration::from_millis(500));
            targets = $($fn_name,)*
        }
    };
}

/// How many times the routines of a nano-benchmark are run within a single measurement.
///
/// CodSpeed measures a single iteration of each benchmark, so a routine that compiles down to a
/// handful of instructions is smaller than the fixed cost of taking a measurement (and may not be
/// resolvable at all). Running the routine repeatedly amortizes that cost.
const ROUNDS: usize = 64;

/// Benchmark a group of routines as a single measurement.
///
/// Calling `Bencher::iter` more than once in the same benchmark reports several values under the
/// same name, of which only the last is kept. Grouping the routines instead measures all of them.
macro_rules! iter_all {
    ($ben:ident, [$($routine:expr),+ $(,)?]) => {
        $ben.iter(|| {
            $(crate::run_opaque($routine);)+
        })
    };
}

/// Benchmark a group of routines as a single measurement, running each of them [`ROUNDS`] times.
///
/// This is used for the benchmarks that are too small to be measured on their own.
macro_rules! iter_all_repeated {
    ($ben:ident, [$($routine:expr),+ $(,)?]) => {
        $ben.iter(|| {
            for _ in 0..crate::ROUNDS {
                $(crate::run_opaque($routine);)+
            }
        })
    };
}

/// Benchmark a group of routines mutating a freshly initialized value as a single measurement.
///
/// As with [`iter_all`], the routines are grouped so that they are all measured under the
/// benchmark's name. They are applied to the same value, one after the other.
macro_rules! iter_batched_ref {
    ($ben:ident, $initializer:expr,[$($routine:expr),+ $(,)?]) => {
        $ben.iter_batched_ref(
            $initializer,
            |value| {
                $(crate::run_opaque_ref(value, $routine);)+
            },
            ::criterion::BatchSize::SmallInput,
        );
    };
}

macro_rules! mods {
    ($(mod $mod:ident;)+) => {
        $(mod $mod;)+
        ::criterion::criterion_main!($($mod::benches),+);
    }
}

mods![
    mod date;
    mod duration;
    mod formatting;
    mod instant;
    mod month;
    mod offset_date_time;
    mod parse_format_description;
    mod parsing;
    mod plain_date_time;
    mod rand08;
    mod rand09;
    mod time;
    mod utc_date_time;
    mod utc_offset;
    mod util;
    mod weekday;
];

/// Run a routine once, discarding its result.
///
/// The routine is called through an opaque pointer, which keeps the compiler from hoisting the
/// call out of the surrounding loop or from replacing it with a constant.
fn run_opaque<O>(mut routine: impl FnMut() -> O) {
    let routine = black_box(&mut routine);
    drop(black_box(routine()));
}

/// Run a routine once on a mutable value, discarding its result.
///
/// This is the equivalent of [`run_opaque`] for the routines of [`iter_batched_ref`].
fn run_opaque_ref<I, O>(value: &mut I, mut routine: impl FnMut(&mut I) -> O) {
    let routine = black_box(&mut routine);
    let value = black_box(value);
    drop(black_box(routine(value)));
}

/// Shuffle a slice in a random but deterministic manner.
fn shuffle<T, const N: usize>(mut slice: [T; N]) -> [T; N] {
    use ::rand09::prelude::*;

    let mut seed = SmallRng::seed_from_u64(0);
    slice.shuffle(&mut seed);
    slice
}
