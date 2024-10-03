#[cfg(test)]
mod tests {
    use crate::csv::Csv;
    use std::path::Path;

    fn setup(test_number: i8) -> Csv {
        let filename: String = String::from(format!("test{}.csv", test_number));
        let input: Vec<Vec<String>> = vec![
            vec!["H1".to_string(), "H2".to_string(), "H3".to_string()],
            vec!["V1-1".to_string(), "V2-1".to_string(), "V3-1".to_string()],
            vec!["V1-2".to_string(), "V2-2".to_string(), "V3-2".to_string()],
            vec!["V1-3".to_string(), "V2-3".to_string(), "V3-3".to_string()],
        ];
        let csv: Csv = Csv {
            filename,
            data: input,
        };
        match csv.write() {
            Ok(_) => println!("Successfully written"),
            Err(error) => panic!("io::Error, failed to write: {}", error),
        }
        return csv;
    }

    fn teardown(csv: &Csv) {
        match csv.destroy() {
            Ok(_) => println!("Successfully destroyed"),
            Err(error) => panic!("io::Error, failed to destroy: {}", error),
        }
    }

    #[test]
    fn test_csv_exists() {
        let csv: Csv = setup(1);
        assert_eq!(true, Path::new(&csv.filename).exists());
        teardown(&csv);
    }

    #[test]
    fn test_csv_has_data() {
        let csv: Csv = setup(2);
        match Csv::load(csv.filename.clone()) {
            Ok(data) => assert_eq!(csv.data, data.data),
            Err(e) => panic!("Error: {}", e),
        };
        teardown(&csv);
    }

    #[test]
    fn test_csv_was_destroyed() {
        let csv: Csv = setup(3);
        teardown(&csv);
        assert_eq!(false, Path::new(&csv.filename).exists());
    }
}
