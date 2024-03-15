use chrono::{self, DateTime, Datelike, Local, Weekday};

#[allow(dead_code)]
pub fn check_time() -> String {
    let datetime: DateTime<Local> = chrono::offset::Local::now();
    let day: Weekday = Datelike::weekday(&datetime);
    let date: u32 = Datelike::day(&datetime);
    let month: u32 = Datelike::month(&datetime);
    let year: i32 = Datelike::year(&datetime);
    format!(
        "the date is {} the {} of {} {}",
        cure_day(day),
        cure_date(date),
        cure_month(month),
        year
    )
}

fn cure_day(day: Weekday) -> String {
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

fn cure_date(date: u32) -> String {
    match date {
        1 => format!("{}{}", date, "st"),
        2 => format!("{}{}", date, "nd"),
        3 => format!("{}{}", date, "rd"),
        21 => format!("{}{}", date, "st"),
        22 => format!("{}{}", date, "nd"),
        23 => format!("{}{}", date, "rd"),
        31 => format!("{}{}", date, "st"),
        _ => format!("{}{}", date, "th"),
    }
}

fn cure_month(month: u32) -> String {
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
