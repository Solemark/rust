use super::helpers::{to_json_string, write_to_file, Helpers};
use axum::{
    body::Body,
    http::StatusCode,
    response::{Redirect, Response},
    Form,
};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Setting {
    pub(crate) name: String,
    pub(crate) status: bool,
}
impl Helpers for Setting {
    fn to_json(&self) -> String {
        format!(
            "{{\"name\": \"{}\",\"status\": {}}}",
            &self.name, &self.status
        )
    }
    fn to_csv(&self) -> String {
        format!("{},{}\n", &self.name, self.status)
    }
}
const FILENAME: &str = "settings";

/**
 * Send all setting data
 * return [`Response`]
 */
pub(crate) async fn setting_data_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(get_settings_list().await)))
        .unwrap_or_default()
}

/**
 * Get settings as a [Vec<Setting>]
 * Return: [`Vec<Setting>`]
 */
pub(crate) async fn get_settings_list() -> Vec<Setting> {
    read_to_string("data/settings.csv")
        .unwrap_or_default()
        .lines()
        .map(|s| {
            let str = s.split(',').collect::<Vec<&str>>();
            Setting {
                name: str[0].to_string(),
                status: str[1].parse::<bool>().unwrap_or_default(),
            }
        })
        .collect()
}

/**
 * Update the setting recieved in [`Form`] data
 * Param: [`Form<Setting>`]
 * Return: [`Redirect`] to [/settings]
 */
pub(crate) async fn update_settings_handler(Form(setting): Form<Setting>) -> Redirect {
    write_to_file(
        get_settings_list()
            .await
            .into_iter()
            .map(|s| {
                if s.name == setting.name {
                    Setting {
                        name: s.name,
                        status: !s.status,
                    }
                } else {
                    s
                }
            })
            .collect(),
        FILENAME.to_string(),
    );
    Redirect::to("/settings")
}
