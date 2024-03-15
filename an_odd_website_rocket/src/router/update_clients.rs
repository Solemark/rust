use crate::router::{
    get_client_list::get_client_list, structs::Client, structs::UpdateClient,
    write_client_list::write_client_list,
};
use rocket::{
    form::Form,
    fs::{relative, NamedFile},
};
use std::path::{Path, PathBuf};

#[post("/data/clients/update", data = "<update_client>")]
pub async fn update_client(update_client: Form<UpdateClient>) -> Option<NamedFile> {
    let index: usize = update_client.client_id.parse().unwrap();
    let updated_record: Client = Client {
        id: index,
        first_name: update_client.first_name.clone(),
        last_name: update_client.last_name.clone(),
        email_address: update_client.email_address.clone(),
        visible: true,
    };
    let mut client_list: Vec<Client> = get_client_list().await;
    client_list[index - 1] = updated_record;
    write_client_list(client_list).await;
    let file: PathBuf = Path::new(relative!("static")).join("clients.html");
    NamedFile::open(file).await.ok()
}
