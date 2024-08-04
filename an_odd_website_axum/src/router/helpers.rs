use std::{fs::File, io::Write};

pub trait Helpers {
    fn to_json(&self) -> String;
    fn to_csv(&self) -> String;
}

pub(crate) fn to_json_string<T: Helpers>(arr: Vec<T>) -> String {
    let mut output = String::new();
    for item in arr {
        output += &format!("{},", item.to_json());
    }
    format!("[{}]", &output[0..output.len() - 1])
}

pub(crate) fn write_to_file<T: Helpers>(arr: Vec<T>, filename: String) {
    Write::write_all(
        &mut File::create(format!("data/{}.csv", filename))
            .unwrap_or_else(|_| panic!("unable to create file {}.csv", filename)),
        arr.into_iter()
            .map(|item| item.to_csv())
            .collect::<String>()
            .as_bytes(),
    )
    .unwrap_or_default()
}
