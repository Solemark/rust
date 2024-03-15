use rocket::fs::{relative, NamedFile};
use std::path::{Path, PathBuf};

#[get("/<path..>")]
pub async fn get_webpage(path: PathBuf) -> Option<NamedFile> {
    let mut filename: String = format!("{}.html", path.to_str().unwrap());
    if path.to_str() == Some("") {
        filename = String::from("index.html");
    }
    let file = Path::new(relative!("static")).join(filename);
    NamedFile::open(file).await.ok()
}
