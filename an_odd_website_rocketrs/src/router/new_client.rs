use crate::router::{
    get_client_list::get_client_list, structs::Client, structs::NewClient,
    write_client_list::write_client_list,
};
use rocket::{
    form::Form,
    fs::{relative, NamedFile},
};
use std::path::{Path, PathBuf};

#[post("/data/clients/new", data = "<new_client>")]
pub async fn new_client(new_client: Form<NewClient>) -> Option<NamedFile> {
    let mut client_list: Vec<Client> = get_client_list().await;
    client_list.push(Client {
        id: client_list.len() + 1,
        first_name: new_client.first_name.clone(),
        last_name: new_client.last_name.clone(),
        email_address: new_client.email_address.clone(),
        visible: true,
    });
    write_client_list(client_list).await;
    let file: PathBuf = Path::new(relative!("static")).join("clients.html");
    NamedFile::open(file).await.ok()
}
