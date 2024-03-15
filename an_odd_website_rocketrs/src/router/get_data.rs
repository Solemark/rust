use rocket::serde::json::Json;

use crate::router::get_client_list::get_client_list;
use crate::router::structs::Client;

#[get("/data/clients")]
pub async fn get_data() -> Json<Vec<Client>> {
    let client_list = get_client_list().await;
    let mut output: Vec<Client> = vec![];
    for client in client_list {
        if client.visible {
            output.push(client);
        }
    }
    Json(output)
}
