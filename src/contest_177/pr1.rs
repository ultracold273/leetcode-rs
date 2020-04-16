
fn days_since_epoch(date: String) -> i32 {
    let d = date.split('-').map(|a| i32::from_str_radix(a, 10).unwrap()).collect::<Vec<_>>();
    let year = d[0];
    let month = d[1];
    let day = d[2];
    let mut days = 0;
    days += 365 * (year - 1971) + (year - 1971) / 4;
    if (year - 1971) % 4 > 1 { days += 1; }
    for m in 0..month {
        days += match m {
            0 => {0},
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {31},
            4 | 6 | 9 | 11 => {30},
            2 => {28},
            _ => {0},
        };
    }
    if year % 4 == 0 && year != 2100 && month > 2 { days += 1; }
    days += day;
    days
}

fn days_between_dates(date1: String, date2: String) -> i32 {
    (days_since_epoch(date2) - days_since_epoch(date1)).abs()
}