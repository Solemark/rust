use crate::router::structs::{Client, RemoveClient};
use crate::router::{get_client_list::get_client_list, write_client_list::write_client_list};
use rocket::{
    form::Form,
    fs::{relative, NamedFile},
};
use std::path::{Path, PathBuf};

#[post("/data/clients/remove", data = "<client>")]
pub async fn remove_client(client: Form<RemoveClient>) -> Option<NamedFile> {
    let mut client_list: Vec<Client> = get_client_list().await;
    client_list[client.client_id - 1].visible = false;
    write_client_list(client_list).await;
    let file: PathBuf = Path::new(relative!("static")).join("clients.html");
    NamedFile::open(file).await.ok()
}
