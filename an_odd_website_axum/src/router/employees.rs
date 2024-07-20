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
pub(crate) struct Employee {
    id: usize,
    first_name: String,
    last_name: String,
    email: String,
    role: String,
    visible: bool,
}
impl Helpers for Employee {
    fn to_json(&self) -> String {
        format!(
            "{{\"id\": \"{}\",\"first_name\": \"{}\",\"last_name\": \"{}\",\"email\": \"{}\",\"role\": \"{}\",\"visible\": \"{}\"}}",
            &self.id, &self.first_name, &self.last_name, &self.email, &self.role, &self.visible
        )
    }
    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{}\n",
            &self.id, &self.first_name, &self.last_name, &self.email, &self.role, &self.visible
        )
    }
}
const FILENAME: &str = "employees";

/**
 * Send all employee data
 * return [`Response`]
 */
pub(crate) async fn employee_data_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(
            get_employee_list()
                .await
                .into_iter()
                .filter(|employee| employee.visible)
                .collect(),
        )))
        .unwrap_or_default()
}

/**
 * Get employees as a [Vec<Employee>]
 * Return: [`Vec<Employee>`]
 */
async fn get_employee_list() -> Vec<Employee> {
    read_to_string("data/employees.csv")
        .unwrap_or_default()
        .lines()
        .map(|s| {
            let e = s.split(',').collect::<Vec<&str>>();
            Employee {
                id: e[0].parse::<usize>().unwrap_or_default(),
                first_name: String::from(e[1]),
                last_name: String::from(e[1]),
                email: String::from(e[3]),
                role: String::from(e[4]),
                visible: e[5].parse::<bool>().unwrap_or_default(),
            }
        })
        .collect()
}

/**
 * Get the new employee [`Form`] data
 * Param: [`Form<Employee>`]
 * Return: [`Redirect`] to [/employees]
 */
pub(crate) async fn new_employee_handler(Form(employee): Form<Employee>) -> Redirect {
    let employee_list = get_employee_list().await;
    let ell = employee_list.len();
    write_to_file(
        employee_list
            .into_iter()
            .chain(vec![Employee {
                id: ell,
                first_name: employee.first_name,
                last_name: employee.last_name,
                email: employee.email,
                role: employee.role,
                visible: true,
            }])
            .collect(),
        FILENAME.to_string(),
    );
    Redirect::to("/employees")
}

/**
 * Remove the employee recieved in [`Form`] data
 * Param: [`Form<Employee>`]
 * Return: [`Redirect`] to [/employees]
 */
pub(crate) async fn remove_employee_handler(Form(employee): Form<Employee>) -> Redirect {
    write_to_file(
        get_employee_list()
            .await
            .into_iter()
            .map(|e| {
                if e.id == employee.id {
                    Employee {
                        id: e.id,
                        first_name: e.first_name,
                        last_name: e.last_name,
                        email: e.email,
                        role: e.role,
                        visible: false,
                    }
                } else {
                    e
                }
            })
            .collect(),
        FILENAME.to_string(),
    );
    Redirect::to("/employees")
}

/**
 * Update the employee recieved in [`Form`] data
 * Param: [`Form<Employee>`]
 * Return: [`Redirect`] to [/employees]
 */
pub(crate) async fn update_employee_handler(Form(employee): Form<Employee>) -> Redirect {
    write_to_file(
        get_employee_list()
            .await
            .into_iter()
            .map(|e| {
                if e.id == employee.id {
                    employee.clone()
                } else {
                    e
                }
            })
            .collect(),
        FILENAME.to_string(),
    );
    Redirect::to("/employees")
}
