use crate::{
    check,
    vec::Vec,
    string::String
};

pub fn decode_hex(s: &str) -> Vec<u8> {
    check(s.len() % 2 == 0, "decod_hex: bad hex string length");
    (0..s.len())
        .step_by(2)
        .map(|i| {
            if let Ok(c) = u8::from_str_radix(&s[i..i + 2], 16) {
                c
            } else {
                check(false, "bad hex characters");
                0u8
            }
        })
        .collect::<Vec<_>>()
}

/// Converts a Unix timestamp (seconds since epoch, UTC) to an ISO 8601 formatted string, e.g. "1970-01-01T00:00:00".
pub fn unix_timestamp_to_iso8601(timestamp: u64) -> String {
    let days = timestamp / 86_400;
    let seconds_in_day = timestamp % 86_400;
    let hours = seconds_in_day / 3600;
    let minutes = (seconds_in_day % 3600) / 60;
    let seconds = seconds_in_day % 60;

    // Decompose the (year, month, day) from days since 1970-01-01.
    let (year, month, day) = compute_date_from_days(days);

    // Manually build the ISO-8601 string (YYYY-MM-DDThh:mm:ssZ).
    let mut result = String::new();
    // Year
    result.push_str(&u64_to_str(year));
    result.push('-');
    // Month
    result.push_str(&zero_pad(month));
    result.push('-');
    // Day
    result.push_str(&zero_pad(day));
    result.push('T');
    // Hour
    result.push_str(&zero_pad(hours));
    result.push(':');
    // Minute
    result.push_str(&zero_pad(minutes));
    result.push(':');
    // Second
    result.push_str(&zero_pad(seconds));

    result
}

/// Decomposes the total number of days since the Unix epoch (1970-01-01)
/// into a (year, month, day) in the Gregorian calendar.
fn compute_date_from_days(mut days: u64) -> (u64, u64, u64) {
    let mut year = 1970_u64;

    // Accumulate years until we find which year 'days' belongs to.
    while days >= days_in_year(year) {
        days -= days_in_year(year);
        year += 1;
    }

    // Now find which month 'days' belongs to.
    let mut month = 1_u64;
    while days >= days_in_month(year, month) {
        days -= days_in_month(year, month);
        month += 1;
    }

    // Days are zero-based internally, so +1 for the actual day of the month.
    let day = days + 1;

    (year, month, day)
}

/// Returns the number of days in the given year (365 or 366).
fn days_in_year(year: u64) -> u64 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// Returns `true` if the given year is a leap year under the Gregorian rules.
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Returns the number of days in a given (year, month).
fn days_in_month(year: u64, month: u64) -> u64 {
    match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        // Should never occur, but just in case
        _ => 0,
    }
}

/// Converts an unsigned 64-bit integer to a `String` without using `std::format`.
fn u64_to_str(mut n: u64) -> String {
    let mut s = String::new();
    if n == 0 {
        s.push('0');
        return s;
    }
    // Allocate enough space for all possible digits of a u64 (up to 20 digits).
    let mut digits = [0u8; 20];
    let mut i = 0;
    while n > 0 {
        digits[i] = (b'0' + (n % 10) as u8) as u8;
        n /= 10;
        i += 1;
    }
    // Reverse the collected digits into the string.
    for &digit in digits[..i].iter().rev() {
        s.push(digit as char);
    }
    s
}

/// Returns a 2-digit zero-padded `String` from a u64 (e.g. 7 -> "07", 12 -> "12").
fn zero_pad(n: u64) -> String {
    let mut s = u64_to_str(n);
    if s.len() < 2 {
        s.insert(0, '0');
    }
    s
}