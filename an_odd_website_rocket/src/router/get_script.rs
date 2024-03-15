use rocket::fs::{relative, NamedFile};
use std::path::{Path, PathBuf};

#[get("/script/<path>")]
pub async fn get_script(path: PathBuf) -> Option<NamedFile> {
    let filename: String = format!("{}", path.to_string_lossy());
    let file = Path::new(relative!("static")).join(filename);
    NamedFile::open(file).await.ok()
}
