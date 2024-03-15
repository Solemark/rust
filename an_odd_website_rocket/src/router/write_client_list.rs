use crate::router::structs::Client;
use rocket::{
    fs::relative,
    tokio::{
        fs,
        io::{self, AsyncWriteExt},
    },
};
pub(crate) async fn write_client_list(client_list: Vec<Client>) {
    let mut file = fs::File::create(relative!("data/clients.csv"))
        .await
        .unwrap();
    let mut output: String = String::new();
    for client in client_list {
        output += &format!(
            "{},{},{},{},{}\n",
            client.id, client.first_name, client.last_name, client.email_address, client.visible
        );
    }
    let res: Result<(), io::Error> = file.write_all(output.as_bytes()).await;
    match res {
        Ok(_) => (),
        Err(_err) => panic!(), //TODO - Make an error
    }
}
