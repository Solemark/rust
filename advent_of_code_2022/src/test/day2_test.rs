#[cfg(test)]
mod tests {
    use crate::day2::day2;
    use std::collections::HashMap;

    #[test]
    fn test_day_2() {
        let rps: HashMap<String, String> = HashMap::from([
            (String::from("A"), String::from("Rock")),
            (String::from("B"), String::from("Paper")),
            (String::from("C"), String::from("Scissors")),
            (String::from("X"), String::from("Rock")),
            (String::from("Y"), String::from("Paper")),
            (String::from("Z"), String::from("Scissors")),
        ]);

        let poi: HashMap<String, i32> = HashMap::from([
            (String::from("Rock"), 1),
            (String::from("Paper"), 2),
            (String::from("Scissors"), 3),
            (String::from("Loss"), 0),
            (String::from("Draw"), 3),
            (String::from("Win"), 6),
        ]);

        let res: HashMap<String, i32> = HashMap::from([
            (String::from("Rock-Paper"), poi["Win"]),
            (String::from("Paper-Scissors"), poi["Win"]),
            (String::from("Scissors-Rock"), poi["Win"]),
            (String::from("Paper-Rock"), poi["Loss"]),
            (String::from("Scissors-Paper"), poi["Loss"]),
            (String::from("Rock-Scissors"), poi["Loss"]),
            (String::from("Rock-Rock"), poi["Draw"]),
            (String::from("Paper-Paper"), poi["Draw"]),
            (String::from("Scissors-Scissors"), poi["Draw"]),
        ]);
        let data: Vec<Vec<String>> = vec![
            vec![String::from("A"), String::from("Y")],
            vec![String::from("B"), String::from("X")],
            vec![String::from("C"), String::from("Z")],
        ];
        let expect: i32 = 15;
        let result: i32 = day2(data, rps, poi, res);
        assert_eq!(expect, result);
    }
}
