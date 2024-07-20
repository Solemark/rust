#[cfg(test)]
mod tests {
    use crate::day3::day3;

    #[test]
    fn test_day_3() {
        let data: Vec<String> = Vec::from([
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ]);
        let expect: i32 = 157;
        let result: i32 = day3(data);
        assert_eq!(expect, result);
    }
}
