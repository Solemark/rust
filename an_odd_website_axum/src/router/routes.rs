use crate::router::{
    clients::{
        client_data_handler, new_client_handler, remove_client_handler, update_client_handler,
    },
    employees::{
        employee_data_handler, new_employee_handler, remove_employee_handler,
        update_employee_handler,
    },
    exporters::{get_exports_handler, get_jobs_handler, new_jobs_handler},
    settings::{setting_data_handler, update_settings_handler},
    website::{index_handler, script_handler, style_handler, webpage_handler},
};
use axum::{
    routing::{get, post},
    Router,
};

pub(crate) async fn routing() {
    let app = Router::new()
        // Webpage routes
        .route("/", get(index_handler))
        .route("/:page", get(webpage_handler))
        .route("/styles/:style", get(style_handler))
        .route("/scripts/:script", get(script_handler))
        // Client routes
        .route("/data/clients", get(client_data_handler))
        .route("/data/clients/new", post(new_client_handler))
        .route("/data/clients/update", post(update_client_handler))
        .route("/data/clients/remove", post(remove_client_handler))
        // Employee routes
        .route("/data/employees", get(employee_data_handler))
        .route("/data/employees/new", post(new_employee_handler))
        .route("/data/employees/update", post(update_employee_handler))
        .route("/data/employees/remove", post(remove_employee_handler))
        // Exporters routes
        .route("/data/jobs", get(get_jobs_handler))
        .route("/data/jobs/new", post(new_jobs_handler))
        .route("/data/exports", get(get_exports_handler))
        // Setting routes
        .route("/data/settings", get(setting_data_handler))
        .route("/data/settings/update", post(update_settings_handler));

    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .expect("Error in TcpListener");
    axum::serve(listener, app).await.unwrap_or_default();
}
