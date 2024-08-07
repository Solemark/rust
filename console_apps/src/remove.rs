#[allow(dead_code)]
pub fn chars(s: String, cs: Vec<char>) -> String {
    s.chars().filter(|c| !cs.contains(c)).collect()
}
