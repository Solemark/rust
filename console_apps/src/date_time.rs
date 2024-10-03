use chrono::{self, Datelike, Weekday};

#[allow(dead_code)]
pub fn check_time() -> String {
    let datetime = chrono::offset::Local::now();
    format!(
        "the date is {} the {} of {} {}",
        cure_day(datetime.weekday()),
        cure_date(datetime.day()),
        cure_month(datetime.month()),
        datetime.year()
    )
}

pub fn cure_day(day: Weekday) -> String {
    match day {
        Weekday::Mon => String::from("Monday"),
        Weekday::Tue => String::from("Tuesday"),
        Weekday::Wed => String::from("Wednesday"),
        Weekday::Thu => String::from("Thursday"),
        Weekday::Fri => String::from("Friday"),
        Weekday::Sat => String::from("Saturday"),
        Weekday::Sun => String::from("Sunday"),
    }
}

pub fn cure_date(date: u32) -> String {
    match date {
        1 | 21 | 31 => format!("{}{}", date, "st"),
        2 | 22 => format!("{}{}", date, "nd"),
        3 | 23 => format!("{}{}", date, "rd"),
        _ => format!("{}{}", date, "th"),
    }
}

pub fn cure_month(month: u32) -> String {
    match month {
        1 => String::from("January"),
        2 => String::from("February"),
        3 => String::from("March"),
        4 => String::from("April"),
        5 => String::from("May"),
        6 => String::from("June"),
        7 => String::from("July"),
        8 => String::from("August"),
        9 => String::from("September"),
        10 => String::from("October"),
        11 => String::from("November"),
        12 => String::from("December"),
        _ => panic!("Incorrect Month Value!"),
    }
}
