use std::{
    fs::{self, File},
    io::{Error, Write},
};

pub struct Csv {
    pub filename: String,
    pub data: Vec<Vec<String>>,
}

#[allow(dead_code)]
impl Csv {
    pub fn load(filename: String) -> Result<Csv, Error> {
        match fs::read_to_string(&filename) {
            Ok(res) => {
                let data = Self::csv_to_vec(res);
                Ok(Csv { filename, data })
            }
            Err(e) => Err(e),
        }
    }

    fn csv_to_vec(res: String) -> Vec<Vec<String>> {
        let mut data = Vec::new();
        for row in res.lines() {
            let mut row_items = Vec::new();
            let items = row.split(',').collect::<Vec<&str>>();
            for item in items {
                row_items.push(item.to_string());
            }
            data.push(row_items);
        }
        data
    }

    pub fn write(&self) -> Result<(), Error> {
        let mut output = String::new();
        let mut i = 0;
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
        match File::create(&self.filename) {
            Ok(mut f) => match f.write_all(output.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        match fs::remove_file(&self.filename) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}
