#[cfg(test)]
mod tests {
    use crate::day1::day1;

    #[test]
    fn test_day_1() {
        let data: Vec<String> = Vec::from([
            String::from("1000"), // 1
            String::from("2000"),
            String::from("3000"),
            String::from(""),
            String::from("4000"), // 2
            String::from(""),
            String::from("5000"), // 3
            String::from("6000"),
            String::from(""),
            String::from("7000"), // 4
            String::from("8000"),
            String::from("9000"),
            String::from(""),
            String::from("10000"), // 5
        ]);
        let expect: String = String::from("Elf 4 has the most calories");
        let result: String = day1(data);
        assert_eq!(expect, result);
    }
}
