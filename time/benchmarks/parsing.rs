use criterion::Bencher;
use time::OffsetDateTime;
use time::format_description::well_known::{Rfc2822, Rfc3339};
use time::format_description::{Component, modifier};
use time::parsing::Parsed;

macro_rules! component {
    ($name:ident {$($field:ident : $value:expr),* $(,)? }) => {{
        const COMPONENT: Component = Component::$name({
            #[allow(unused_mut, reason = "macro-generated code")]
            let mut modifier = modifier::$name::default();
            $(modifier.$field = $value;)*
            modifier
        });
        COMPONENT
    }};
}

setup_benchmark! {
    "Parsing",

    fn parse_component_year(ben: &mut Bencher<'_>) {
        let mut parsed = Parsed::new();
        iter_all!(ben, [
            || {
                parsed.parse_component(b"2021", Component::CalendarYearFullStandardRange(
                    modifier::CalendarYearFullStandardRange::default()
                        .with_padding(modifier::Padding::Zero)
                        .with_sign_is_mandatory(false)
                ))
            },
            || {
                parsed.parse_component(b"21", Component::CalendarYearLastTwo(
                    modifier::CalendarYearLastTwo::default().with_padding(modifier::Padding::Zero)
                ))
            },
            || {
                parsed.parse_component(b"2021", Component::IsoYearFullStandardRange(
                    modifier::IsoYearFullStandardRange::default()
                        .with_padding(modifier::Padding::Zero)
                        .with_sign_is_mandatory(false)
                ))
            },
            || {
                parsed.parse_component(b"21", Component::IsoYearLastTwo(
                    modifier::IsoYearLastTwo::default().with_padding(modifier::Padding::Zero)
                ))
            },
        ]);
    }

    fn parse_component_month(ben: &mut Bencher<'_>) {
        let mut parsed = Parsed::new();
        iter_all!(ben, [
            || {
                parsed.parse_component(b" 1", Component::MonthNumerical(
                    modifier::MonthNumerical::default().with_padding(modifier::Padding::Space)
                ))
            },
            || {
                parsed.parse_component(b"Jan", Component::MonthShort(
                    modifier::MonthShort::default().with_case_sensitive(true)
                ))
            },
            || {
                parsed.parse_component(b"January", Component::MonthLong(
                    modifier::MonthLong::default().with_case_sensitive(true)
                ))
            },
        ]);
    }

    fn parse_component_ordinal(ben: &mut Bencher<'_>) {
        let mut parsed = Parsed::new();
        ben.iter(|| {
            parsed.parse_component(b"012", component!(Ordinal {
                padding: modifier::Padding::Zero,
            }))
        });
    }

    fn parse_component_weekday(ben: &mut Bencher<'_>) {
        let mut parsed = Parsed::new();
        iter_all!(ben, [
            || {
                parsed.parse_component(
                    b"Sun",
                    Component::WeekdayShort(modifier::WeekdayShort::default())
                )
            },
            || {
                parsed.parse_component(
                    b"Sunday",
                    Component::WeekdayLong(modifier::WeekdayLong::default())
                )
            },
            || {
                parsed.parse_component(
                    b"0",
                    Component::WeekdaySunday(
                        modifier::WeekdaySunday::default().with_one_indexed(false)
                    )
                )
            },
            || {
                parsed.parse_component(
                    b"1",
                    Component::WeekdaySunday(
                        modifier::WeekdaySunday::default().with_one_indexed(true)
                    )
                )
            },
            || {
                parsed.parse_component(
                    b"6",
                    Component::WeekdayMonday(
                        modifier::WeekdayMonday::default().with_one_indexed(false)
                    )
                )
            },
            || {
                parsed.parse_component(
                    b"7",
                    Component::WeekdayMonday(
                        modifier::WeekdayMonday::default().with_one_indexed(true)
                    )
                )
            },
        ]);
    }

    fn parse_component_week_number(ben: &mut Bencher<'_>) {
        let mut parsed = Parsed::new();
        iter_all!(ben, [
            || {
                parsed.parse_component(b"2", component!(WeekNumberSunday {
                    padding: modifier::Padding::None,
                }))
            },
            || {
                parsed.parse_component(b"2", component!(WeekNumberMonday {
                    padding: modifier::Padding::None,
                }))
            },
            || {
                parsed.parse_component(b"2", component!(WeekNumberIso {
                    padding: modifier::Padding::None,
                }))
            },
        ]);
    }

    fn parse_component_subsecond(ben: &mut Bencher<'_>) {
        let mut parsed = Parsed::new();
        iter_all!(ben, [
            || {
                parsed.parse_component(b"1", component!(Subsecond {
                    digits: modifier::SubsecondDigits::One,
                }))
            },
            || {
                parsed.parse_component(b"12", component!(Subsecond {
                    digits: modifier::SubsecondDigits::Two,
                }))
            },
            || {
                parsed.parse_component(b"123", component!(Subsecond {
                    digits: modifier::SubsecondDigits::Three,
                }))
            },
            || {
                parsed.parse_component(b"1234", component!(Subsecond {
                    digits: modifier::SubsecondDigits::Four,
                }))
            },
            || {
                parsed.parse_component(b"12345", component!(Subsecond {
                    digits: modifier::SubsecondDigits::Five,
                }))
            },
            || {
                parsed.parse_component(b"123456", component!(Subsecond {
                    digits: modifier::SubsecondDigits::Six,
                }))
            },
            || {
                parsed.parse_component(b"1234567", component!(Subsecond {
                    digits: modifier::SubsecondDigits::Seven,
                }))
            },
            || {
                parsed.parse_component(b"12345678", component!(Subsecond {
                    digits: modifier::SubsecondDigits::Eight,
                }))
            },
            || {
                parsed.parse_component(b"123456789", component!(Subsecond {
                    digits: modifier::SubsecondDigits::Nine,
                }))
            },
            || {
                parsed.parse_component(b"123456789", component!(Subsecond {
                    digits: modifier::SubsecondDigits::OneOrMore,
                }))
            },
        ]);
    }

    fn parse_component_unix_timestamp(ben: &mut Bencher<'_>) {
        let mut parsed = Parsed::new();
        ben.iter(|| parsed.parse_component(std::hint::black_box(b"1234567890"), component!(UnixTimestampSecond {})));
    }

    fn parse_rfc3339(ben: &mut Bencher<'_>) {
        iter_all!(ben, [
            || OffsetDateTime::parse("2021-01-02T03:04:05Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.1Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.12Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.123Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.1234Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.12345Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.123456Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.1234567Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.12345678Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.123456789Z", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.123456789-01:02", &Rfc3339),
            || OffsetDateTime::parse("2021-01-02T03:04:05.123456789+01:02", &Rfc3339),
        ]);
    }

    fn parse_rfc2822(ben: &mut Bencher<'_>) {
        iter_all!(ben, [
            || OffsetDateTime::parse("Sat, 02 Jan 2021 03:04:05 +0000", &Rfc2822),
            || OffsetDateTime::parse("Sat, 02 Jan 2021 03:04:05 +0607", &Rfc2822),
            || OffsetDateTime::parse("Sat, 02 Jan 2021 03:04:05 -0607", &Rfc2822),
        ]);
    }
}
