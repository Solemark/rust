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

pub(crate) async fn setting_data_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(get_settings_list())))
        .unwrap_or_default()
}

pub(crate) async fn update_settings_handler(Form(set): Form<Setting>) -> Redirect {
    write_to_file(
        get_settings_list()
            .into_iter()
            .map(|s| {
                if s.name == set.name {
                    Setting {
                        name: s.name,
                        status: !s.status,
                    }
                } else {
                    s
                }
            })
            .collect(),
        "settings".to_string(),
    );
    Redirect::to("/settings")
}

pub(crate) fn get_settings_list() -> Vec<Setting> {
    read_to_string("data/settings.csv")
        .unwrap_or_default()
        .lines()
        .map(parse_setting)
        .collect()
}

fn parse_setting(s: &str) -> Setting {
    let str = s.split(',').collect::<Vec<&str>>();
    Setting {
        name: str[0].to_string(),
        status: str[1].parse::<bool>().unwrap_or_default(),
    }
}
