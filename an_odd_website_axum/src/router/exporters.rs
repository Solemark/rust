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
pub(crate) struct Job {
    id: usize,
    name: String,
    created: String,
    status: String,
}
impl Helpers for Job {
    fn to_json(&self) -> String {
        format!(
            "{{\"id\": \"{}\",\"name\": \"{}\",\"created\": \"{}\",\"status\": \"{}\"}}",
            &self.id, &self.name, &self.created, &self.status
        )
    }
    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{}\n",
            &self.id, &self.name, &self.created, &self.status
        )
    }
}

pub(crate) async fn get_jobs_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(get_jobs_list())))
        .unwrap_or_default()
}

pub(crate) async fn new_jobs_handler(Form(job): Form<Job>) -> Redirect {
    let jobs_list = get_jobs_list();
    let jll = jobs_list.len();
    write_to_file(
        jobs_list
            .clone()
            .into_iter()
            .chain(vec![Job {
                id: jll,
                name: job.name,
                created: job.created,
                status: job.status,
            }])
            .collect(),
        "jobs".to_string(),
    );
    Redirect::to("/accounting")
}

fn get_jobs_list() -> Vec<Job> {
    read_to_string("data/jobs.csv")
        .unwrap_or_default()
        .lines()
        .map(parse_job)
        .collect()
}

fn parse_job(s: &str) -> Job {
    let j: Vec<&str> = s.split(',').collect();
    Job {
        id: j[0].parse::<usize>().unwrap_or_default(),
        name: String::from(j[1]),
        created: String::from(j[2]),
        status: String::from(j[3]),
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Export {
    id: usize,
    name: String,
}
impl Helpers for Export {
    fn to_json(&self) -> String {
        format!(
            "{{
                \"id\": \"{}\",
                \"name\": \"{}\"
            }}",
            &self.id, &self.name
        )
    }
    fn to_csv(&self) -> String {
        format!("{},{}\n", &self.id, &self.name)
    }
}

pub(crate) async fn get_exports_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(get_exports_list())))
        .unwrap_or_default()
}

fn get_exports_list() -> Vec<Export> {
    read_to_string("data/exports.csv")
        .unwrap_or_default()
        .lines()
        .map(|s| {
            let e = s.split(',').collect::<Vec<&str>>();
            Export {
                id: e[0].parse::<usize>().unwrap_or_default(),
                name: String::from(e[1]),
            }
        })
        .collect()
}
