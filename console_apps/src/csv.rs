use std::{
    fs::{self, File},
    io::{self, prelude::*},
};
/**
 * library for performing io based csv operations
 * @property filename: String
 * @property data: Vec<Vec<String>>
 * filename includes the path to the file, if no path is included file will be added to root directory
 * data is a nested Vec of data to be written:
 *      vec![ //nested Vec's must be type Vec<String>
 *          vec!["header_1", "header_2", "header_3"], //Header Row
 *          vec!["data_1-1", "data_2-1", "data_3-1"], //First row of data
 *          vec!["data_1-2", "data_2-2", "data_3-2"], //Second row of data
 *          vec!["data_1-3", "data_2-3", "data_3-3"], //Third row of data
 *          //etc
 *      ];
 */
pub struct Csv {
    pub filename: String,
    pub data: Vec<Vec<String>>,
}
#[allow(dead_code)]
impl Csv {
    /**
     * @TODO - seperate from CSV functions
     * loads csv with the input filename
     * @uses filename: String
     * @uses data: Vec<Vec<String>>
     * @returns Result<CSV, io:Error>
     */
    pub fn load(filename: &String) -> Result<Csv, io::Error> {
        let mut output: Csv = Csv {
            filename: filename.to_string(),
            data: Vec::new(),
        };
        let result: String = fs::read_to_string(&output.filename)?;
        for row in result.lines() {
            let mut row_items: Vec<String> = Vec::new();
            let items: Vec<&str> = row.split(',').collect();
            for item in items {
                row_items.push(item.to_string());
            }
            output.data.push(row_items);
        }
        Ok(output)
    }

    /**
     * writes the current csv to the root directory
     * @uses filename: String
     * @uses data: Vec<Vec<String>>
     * @returns Result<bool, io:Error>
     */
    pub fn write(&self) -> Result<bool, io::Error> {
        let mut file: File = File::create(&self.filename)?;
        let mut output: String = String::new();
        let mut i: usize = 0;
        for row in &self.data {
            for item in row {
                if i >= row.len() - 1 {
                    i = 0;
                    output += &format!("{}\n", &item);
                } else {
                    i += 1;
                    output += &format!("{},", &item);
                }
            }
        }
        let res: Result<(), io::Error> = file.write_all(output.as_bytes());
        match res {
            Ok(_) => Ok(true),
            Err(error) => Err(error),
        }
    }

    /**
     * deletes csv file with own filename
     * @uses filename: String
     * @returns Result<bool, io::Error>
     */
    pub fn destroy(&self) -> Result<bool, io::Error> {
        let status: Result<(), io::Error> = fs::remove_file(&self.filename);
        match status {
            Ok(()) => Ok(true),
            Err(error) => Err(error),
        }
    }
}
