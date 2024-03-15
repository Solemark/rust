#[allow(dead_code)]
pub fn get_table(table: i32, max: i32) -> Vec<i32> {
    (0..=max).map(|i| i * table).collect()
}
