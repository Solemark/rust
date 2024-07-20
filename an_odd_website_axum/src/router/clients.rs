use super::helpers::{to_json_string, write_to_file, Helpers};
use axum::{
    body::Body,
    extract::Form,
    http::StatusCode,
    response::{Redirect, Response},
};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Client {
    id: usize,
    first_name: String,
    last_name: String,
    email: String,
    visible: bool,
}
impl Helpers for Client {
    fn to_json(&self) -> String {
        format!(
            "{{\"id\": \"{}\",\"first_name\": \"{}\",\"last_name\": \"{}\",\"email\": \"{}\",\"visible\": \"{}\"}}",
            &self.id, &self.first_name, &self.last_name, &self.email, &self.visible
        )
    }

    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{}\n",
            &self.id, &self.first_name, &self.last_name, &self.email, &self.visible
        )
    }
}
const FILENAME: &str = "clients";

/**
 * Send all client data
 * return [`Response`]
 */
pub(crate) async fn client_data_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(
            get_client_list()
                .await
                .into_iter()
                .filter(|client| client.visible)
                .collect::<Vec<Client>>(),
        )))
        .unwrap_or_default()
}

/**
 * Get clients as a [Vec<Client>]
 * Return: [`Vec<Client>`]
 */
async fn get_client_list() -> Vec<Client> {
    read_to_string("data/clients.csv")
        .unwrap_or_default()
        .lines()
        .map(|s| {
            let c = s.split(',').collect::<Vec<&str>>();
            Client {
                id: c[0].parse::<usize>().unwrap_or_default(),
                first_name: String::from(c[1]),
                last_name: String::from(c[2]),
                email: String::from(c[3]),
                visible: c[4].parse::<bool>().unwrap_or_default(),
            }
        })
        .collect()
}

/**
 * Get the new client [`Form`] data
 * Param: [`Form<Client>`]
 * Return: [`Redirect`] to [/clients]
 */
pub(crate) async fn new_client_handler(Form(client): Form<Client>) -> Redirect {
    let client_list = get_client_list().await;
    let cll = client_list.len();
    write_to_file(
        client_list
            .into_iter()
            .chain(vec![Client {
                id: cll,
                first_name: client.first_name,
                last_name: client.last_name,
                email: client.email,
                visible: true,
            }])
            .collect(),
        FILENAME.to_string(),
    );
    Redirect::to("/clients")
}

/**
 * Remove the client recieved in [`Form`] data
 * Param: [`Form<Client>`]
 * Return: [`Redirect`] to [/clients]
 */
pub(crate) async fn remove_client_handler(Form(client): Form<Client>) -> Redirect {
    write_to_file(
        get_client_list()
            .await
            .into_iter()
            .map(|c| {
                if c.id == client.id {
                    Client {
                        id: c.id,
                        first_name: c.first_name,
                        last_name: c.last_name,
                        email: c.email,
                        visible: false,
                    }
                } else {
                    c
                }
            })
            .collect(),
        FILENAME.to_string(),
    );
    Redirect::to("/clients")
}

/**
 * Update the client recieved in [`Form`] data
 * Param: [`Form<Client>`]
 * Return: [`Redirect`] to [/clients]
 */
pub(crate) async fn update_client_handler(Form(client): Form<Client>) -> Redirect {
    write_to_file(
        get_client_list()
            .await
            .into_iter()
            .map(|c| if c.id == client.id { client.clone() } else { c })
            .collect(),
        FILENAME.to_string(),
    );
    Redirect::to("/clients")
}
