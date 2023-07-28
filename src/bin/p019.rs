use chrono::{Datelike, Weekday};

fn main() {
    let mut sundays = 0;

    for year in 1901..=2000 {
        for month in 1..=12 {
            let date = chrono::NaiveDate::from_ymd(year, month, 1);
            if date.weekday() == Weekday::Sun {
                sundays += 1;
            }
        }
    }

    println!("{}", sundays);
}