fn days_in_month(month: u8, year: u32) -> u8 {
    match month {
        2 => {
            if year % 100 == 0 {
                if year % 400 == 0 {
                    29
                } else {
                    28
                }
            } else if year % 4 == 0 {
                29
            } else {
                28
            }
        },
        4 | 6 | 9 | 11 => 30,
        _ => 31
    }
}

pub fn solve() -> u32 {
    let mut res = 0;
    let mut dow: u8 = 0;

    for month in 1..=12 {
        for _ in 1..=days_in_month(month, 1900) {
            dow = (dow + 1) % 7
        }
    }

    for year in 1901..=2000 {
        for month in 1..=12 {
            if dow == 6 {
                res += 1;
            }

            for _ in 1..=days_in_month(month, year) {
                dow = (dow + 1) % 7
            }
        }
    }

    res
}
