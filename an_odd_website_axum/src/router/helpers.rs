use std::{fs::File, io::Write};

pub trait Helpers {
    /**
     * json stringify the [`T`]
     * Param: [`&self`]
     * Return: [`String`]
     */
    fn to_json(&self) -> String;
    /**
     * csv stringify the [`T`]
     * Param: [`&self`]
     * Return: [`String`]
     */
    fn to_csv(&self) -> String;
}

/**
 * turn [Vec<T>] into a json formatted string
 * Param: [`Vec<T>`] clients
 * Return: [`String`]
 */
pub(crate) fn to_json_string<T: Helpers>(arr: Vec<T>) -> String {
    let mut output = String::new();
    for item in arr {
        output += &format!("{},", item.to_json());
    }
    format!("[{}]", &output[0..output.len() - 1])
}

/**
 * Write the list [`Vec<T>`] to file [data/name.csv]
 * Param: [`Vec<T>`] arr
 * Param: [`String`] name
 */
pub(crate) fn write_to_file<T: Helpers>(arr: Vec<T>, filename: String) {
    Write::write_all(
        &mut File::create(format!("data/{}.csv", filename))
            .expect(&format!("unable to create file {}.csv", filename)),
        arr.into_iter()
            .map(|item| item.to_csv())
            .collect::<String>()
            .as_bytes(),
    )
    .unwrap_or_default()
}
