use std::collections::HashMap;

#[allow(dead_code)]
pub fn day2(
    data: Vec<Vec<String>>,
    rps: HashMap<String, String>,
    poi: HashMap<String, i32>,
    res: HashMap<String, i32>,
) -> i32 {
    let mut total: i32 = 0;
    for i in 0..data.len() {
        total += poi[&rps[&data[i][1]]];
        total += res[&format!("{}-{}", &rps[&data[i][0]], &rps[&data[i][1]])];
    }
    total
}
