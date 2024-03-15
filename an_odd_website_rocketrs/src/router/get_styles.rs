use rocket::fs::{relative, NamedFile};
use std::path::{Path, PathBuf};

#[get("/styles/<path>")]
pub async fn get_styles(path: PathBuf) -> Option<NamedFile> {
    let filename: String = format!("{}.css", path.to_str().unwrap());
    let file = Path::new(relative!("static/styles")).join(filename);
    NamedFile::open(file).await.ok()
}
