use chrono::{Datelike, Duration, TimeZone};
use chrono_tz::US::Eastern;

pub fn get_open_ranges() -> Vec<(i64, i64)> {
    let mut open_ranges = vec![];

    #[cfg(feature = "2023")]
    open_ranges.append(&mut open_ranges_2023());
    #[cfg(feature = "2024")]
    open_ranges.append(&mut open_ranges_2024());
    #[cfg(feature = "2025")]
    open_ranges.append(&mut open_ranges_2025());

    open_ranges
}

#[cfg(feature = "2023")]
pub fn open_ranges_2023() -> Vec<(i64, i64)> {
    // 2023
    // Holiday 2023 Status
    // New Years Day Monday, January 2 Closed
    // Martin Luther King, Jr. Day Monday, January 16 Closed
    // President's Day Monday, February 20 Closed
    // Good Friday Friday, April 7 Closed
    // Memorial Day Monday, May 29 Closed
    // Juneteenth Monday, June 19 Closed
    // Early Close Monday, July 3 1:00 p.m. ET
    // Independence Day Tuesday, July 4 Closed
    // Labor Day Monday, September 4 Closed
    // Thanksgiving Day Thursday, November 23 Closed
    // Early Close Friday, November 24 1:00 p.m. ET
    // Christmas Day Monday, December 25 Closed

    let closed_vec = vec![
        Eastern.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 1, 16, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 2, 20, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 4, 7, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 5, 29, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 6, 19, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 7, 4, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 9, 4, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 11, 23, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 12, 25, 0, 0, 0).unwrap(),
    ];
    let early_close_vec = vec![
        Eastern.with_ymd_and_hms(2023, 7, 3, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2023, 11, 24, 0, 0, 0).unwrap(),
    ];

    let start_date = Eastern.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_date = Eastern.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
    let mut current_date = start_date;

    let mut open_ranges = vec![];
    while current_date <= end_date {
        if current_date.weekday().number_from_monday() <= 5 {
            let mut is_closed = false;
            for closed in closed_vec.iter() {
                if (closed.year() == current_date.year())
                    && (closed.month() == current_date.month())
                    && (closed.day() == current_date.day())
                {
                    is_closed = true;
                    break;
                }
            }
            if !is_closed {
                let start = Eastern
                    .with_ymd_and_hms(
                        current_date.year(),
                        current_date.month(),
                        current_date.day(),
                        9,
                        30,
                        0,
                    )
                    .unwrap();
                let mut is_early_close = false;
                for early_close in early_close_vec.iter() {
                    if (early_close.year() == current_date.year())
                        && (early_close.month() == current_date.month())
                        && (early_close.day() == current_date.day())
                    {
                        is_early_close = true;
                        break;
                    }
                }
                let end = if is_early_close {
                    Eastern
                        .with_ymd_and_hms(
                            current_date.year(),
                            current_date.month(),
                            current_date.day(),
                            13,
                            0,
                            0,
                        )
                        .unwrap()
                } else {
                    Eastern
                        .with_ymd_and_hms(
                            current_date.year(),
                            current_date.month(),
                            current_date.day(),
                            16,
                            0,
                            0,
                        )
                        .unwrap()
                };
                open_ranges.push((start.timestamp_millis(), end.timestamp_millis()));
            }
        }
        current_date += Duration::days(1);
    }
    open_ranges
}

#[cfg(feature = "2024")]
pub fn open_ranges_2024() -> Vec<(i64, i64)> {
    // 2024
    // Holiday 2024 Status
    // New Years Day Monday, January 1 Closed
    // Martin Luther King, Jr. Day Monday, January 15 Closed
    // President's Day Monday, February 19 Closed
    // Good Friday Friday, March 29 Closed
    // Memorial Day Monday, May 27 Closed
    // Juneteenth Wednesday, June 19 Closed
    // Early Close Wednesday, July 3 1:00 p.m. ET
    // Independence Day Thursday, July 4 Closed
    // Labor Day Monday, September 2 Closed
    // Thanksgiving Day Thursday, November 28 Closed
    // Early Close Friday, November 29 1:00 p.m. ET
    // Early Close Tuesday, December 24 1:00 p.m. ET
    // Christmas Day Wednesday, December 25 Closed
    let closed_vec = vec![
        Eastern.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 2, 19, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 3, 29, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 5, 27, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 6, 19, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 7, 4, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 9, 2, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 11, 28, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 12, 25, 0, 0, 0).unwrap(),
    ];

    let early_close_vec = vec![
        Eastern.with_ymd_and_hms(2024, 7, 3, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 11, 29, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2024, 12, 24, 0, 0, 0).unwrap(),
    ];

    let start_date = Eastern.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end_date = Eastern.with_ymd_and_hms(2024, 12, 31, 0, 0, 0).unwrap();
    let mut current_date = start_date;

    let mut open_ranges = vec![];
    while current_date <= end_date {
        if current_date.weekday().number_from_monday() <= 5 {
            let mut is_closed = false;
            for closed in closed_vec.iter() {
                if (closed.year() == current_date.year())
                    && (closed.month() == current_date.month())
                    && (closed.day() == current_date.day())
                {
                    is_closed = true;
                    break;
                }
            }
            if !is_closed {
                let start = Eastern
                    .with_ymd_and_hms(
                        current_date.year(),
                        current_date.month(),
                        current_date.day(),
                        9,
                        30,
                        0,
                    )
                    .unwrap();
                let mut is_early_close = false;
                for early_close in early_close_vec.iter() {
                    if (early_close.year() == current_date.year())
                        && (early_close.month() == current_date.month())
                        && (early_close.day() == current_date.day())
                    {
                        is_early_close = true;
                        break;
                    }
                }
                let end = if is_early_close {
                    Eastern
                        .with_ymd_and_hms(
                            current_date.year(),
                            current_date.month(),
                            current_date.day(),
                            13,
                            0,
                            0,
                        )
                        .unwrap()
                } else {
                    Eastern
                        .with_ymd_and_hms(
                            current_date.year(),
                            current_date.month(),
                            current_date.day(),
                            16,
                            0,
                            0,
                        )
                        .unwrap()
                };
                open_ranges.push((start.timestamp_millis(), end.timestamp_millis()));
            }
        }
        current_date += Duration::days(1);
    }
    open_ranges
}

#[cfg(feature = "2025")]
pub fn open_ranges_2025() -> Vec<(i64, i64)> {
    // 2025
    // Holiday	Date	Market Status
    // New Year's Day	January 1	Closed
    // MLK, Jr. Day	January 20	Closed
    // Presidents Day	February 17	Closed
    // Good Friday	April 18	Closed
    // Memorial Day	May 26	Closed
    // Juneteenth	June 19	Closed
    // Early Close	July 3	1:00 p.m.
    // Independence Day	July 4	Closed
    // Labor Day	September 1	Closed
    // Thanksgiving Day	November 27	Closed
    // Early Close	November 28	1:00 p.m.
    // Early Close	December 24	1:00 p.m.
    // Christmas Day	December 25	Closed

    let closed_vec = vec![
        Eastern.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 1, 20, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 2, 17, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 4, 18, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 5, 26, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 6, 19, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 7, 4, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 9, 1, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 11, 27, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 12, 25, 0, 0, 0).unwrap(),
    ];

    let early_close_vec = vec![
        Eastern.with_ymd_and_hms(2025, 7, 3, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 11, 28, 0, 0, 0).unwrap(),
        Eastern.with_ymd_and_hms(2025, 12, 24, 0, 0, 0).unwrap(),
    ];

    // Iterate over all workdays of 2025
    let start_date = Eastern.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let end_date = Eastern.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
    let mut current_date = start_date;

    let mut open_ranges = vec![];

    while current_date <= end_date {
        if current_date.weekday().number_from_monday() <= 5 {
            let mut is_closed = false;
            for closed in closed_vec.iter() {
                if (closed.year() == current_date.year())
                    && (closed.month() == current_date.month())
                    && (closed.day() == current_date.day())
                {
                    is_closed = true;
                    break;
                }
            }
            if !is_closed {
                let start = Eastern
                    .with_ymd_and_hms(
                        current_date.year(),
                        current_date.month(),
                        current_date.day(),
                        9,
                        30,
                        0,
                    )
                    .unwrap();
                let mut is_early_close = false;
                for early_close in early_close_vec.iter() {
                    if (early_close.year() == current_date.year())
                        && (early_close.month() == current_date.month())
                        && (early_close.day() == current_date.day())
                    {
                        is_early_close = true;
                        break;
                    }
                }
                let end = if is_early_close {
                    Eastern
                        .with_ymd_and_hms(
                            current_date.year(),
                            current_date.month(),
                            current_date.day(),
                            13,
                            0,
                            0,
                        )
                        .unwrap()
                } else {
                    Eastern
                        .with_ymd_and_hms(
                            current_date.year(),
                            current_date.month(),
                            current_date.day(),
                            16,
                            0,
                            0,
                        )
                        .unwrap()
                };
                open_ranges.push((start.timestamp_millis(), end.timestamp_millis()));
            }
        }
        current_date += Duration::days(1);
    }
    open_ranges
}
