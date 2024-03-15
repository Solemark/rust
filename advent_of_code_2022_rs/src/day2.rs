use std::collections::HashMap;

#[allow(dead_code)]
pub fn day2(
    data: Vec<Vec<String>>,
    rps: HashMap<String, String>,
    poi: HashMap<String, i32>,
    res: HashMap<String, i32>,
) -> i32 {
    calc(data, rps, poi, res, 0, 0)
}

fn calc(
    data: Vec<Vec<String>>,
    rps: HashMap<String, String>,
    poi: HashMap<String, i32>,
    res: HashMap<String, i32>,
    i: usize,
    total: i32,
) -> i32 {
    let mut n_total: i32 = 0;
    n_total += poi[&rps[&data[i][1]]];
    n_total += res[&format!("{}-{}", &rps[&data[i][0]], &rps[&data[i][1]])];
    if data.len() - 1 <= i {
        return total + n_total;
    }
    calc(data, rps, poi, res, i + 1, total + n_total)
}
