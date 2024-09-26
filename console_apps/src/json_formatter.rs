#[allow(dead_code)]
pub fn str_to_json(key: String, msg: String) -> String {
    format!("{{\"{}\": \"{}\"}}", key, msg)
}
