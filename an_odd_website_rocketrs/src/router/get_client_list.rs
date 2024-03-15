use rocket::{fs::relative, tokio::fs};

use crate::router::structs::Client;

pub(crate) async fn get_client_list() -> Vec<Client> {
    let results: String = fs::read_to_string(relative!("data/clients.csv"))
        .await
        .unwrap();
    let mut client_list: Vec<Client> = vec![];
    for result in results.lines() {
        let row: Vec<&str> = result.split(',').collect();
        client_list.push(Client {
            id: row[0].parse().unwrap(),
            first_name: row[1].to_string(),
            last_name: row[2].to_string(),
            email_address: row[3].to_string(),
            visible: row[4].parse().unwrap(),
        });
    }
    client_list
}
