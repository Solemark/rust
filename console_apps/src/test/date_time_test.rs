#[cfg(test)]
mod tests {
    use crate::date_time::{check_time, cure_date, cure_day, cure_month};
    use chrono::Datelike;

    #[test]
    fn test_current_date() {
        let dt = chrono::offset::Local::now();
        let exp = format!(
            "the date is {} the {} of {} {}",
            cure_day(dt.weekday()),
            cure_date(dt.day()),
            cure_month(dt.month()),
            dt.year()
        );
        let res = check_time();
        assert_eq!(exp, res);
    }
}
