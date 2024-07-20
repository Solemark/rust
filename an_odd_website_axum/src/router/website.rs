use super::settings::{get_settings_list, Setting};
use axum::{body::Body, extract::Path, http::StatusCode, response::Response};
use std::fs::read_to_string;

/**
 * Return the index page [static/index.html]
 * Return: [`Response`]
 */
pub(crate) async fn index_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(
            check_dashboard(format!(
                "{}{}",
                get_page_head("Dashboard").await,
                read_to_string("static/index.html")
                    .unwrap_or_else(|err| format!("{}", err))
                    .replace("<!--NAVBAR-->", &get_navbar(String::from("")).await),
            ))
            .await,
        ))
        .unwrap_or_default()
}

async fn check_dashboard(webpage: String) -> String {
    match get_settings_list()
        .await
        .into_iter()
        .filter(|s| s.name == "enable-dashboard")
        .collect::<Vec<Setting>>()
        .first()
        .unwrap_or(&Setting {
            name: String::from("enable-dashboard"),
            status: false,
        })
        .status
    {
        true => webpage.replace(
            "<!--DASHBOARD-->",
            &read_to_string("static/components/dashboard.html").unwrap_or_default(),
        ),
        false => webpage,
    }
}

/**
 * Return the requested page [static/{page}.html]
 * Param: [`Path<String>`] page
 * Return: [`Response`]
 */
pub(crate) async fn webpage_handler(Path(page): Path<String>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(format!(
            "{}{}",
            get_page_head(&page).await,
            read_to_string(format!("static/{}.html", page))
                .unwrap_or_else(|err| format!("{}", err))
                .replace("<!--NAVBAR-->", &get_navbar(page).await),
        )))
        .unwrap_or_default()
}

/**
 * Return the requested style [static/styles/{page}.html]
 * Param: [`Path<String>`] style
 * Return: [`Response`]
 */
pub(crate) async fn style_handler(Path(style): Path<String>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(Body::from(
            read_to_string(format!("static/styles/{}.css", style))
                .unwrap_or_else(|err| format!("{}", err)),
        ))
        .unwrap_or_default()
}

/**
 * Return the requested script [static/{page}.html]
 * Param: [`Path<String>`] script
 * Return: [`Response`]
 */
pub(crate) async fn script_handler(Path(script): Path<String>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(Body::from(
            read_to_string(format!("static/scripts/{}.js", script))
                .unwrap_or_else(|err| format!("{}", err)),
        ))
        .unwrap_or_default()
}

/**
 * Build the navbar html element for the page
 * Param: [`String`] page
 * Return: [`String`]
 */
async fn get_navbar(page: String) -> String {
    let mut navbar = read_to_string("static/components/navbar.html")
        .unwrap_or_default()
        .replace(
            &format!("<a href=\"/{}\">", page),
            &format!("<a class=\"active\" href=\"/{}\">", page),
        );
    for setting in get_settings_list().await.into_iter().filter(|s| !s.status) {
        navbar = match setting.name.as_str() {
            "enable-clients" => navbar.replace("<a href=\"/clients\">Clients</a>", ""),
            "enable-employees" => navbar.replace("<a href=\"/employees\">Employees</a>", ""),
            "enable-exporters" => navbar.replace("<a href=\"/accounting\">Accounting</a>", ""),
            _ => navbar,
        }
    }
    navbar
}

async fn get_page_head(page: &str) -> String {
    let head = read_to_string("static/components/head.html")
        .unwrap_or_default()
        .replace("PAGENAME", page);
    match page {
        "clients" => head.replace(
            "<!--SCRIPT-->",
            "<script defer src=\"/scripts/clients\"></script>",
        ),
        "employees" => head.replace(
            "<!--SCRIPT-->",
            "<script defer src=\"/scripts/employees\"></script>",
        ),
        "accounting" => head.replace(
            "<!--SCRIPT-->",
            "<script defer src=\"scripts/accounting\"></script>",
        ),
        "settings" => head.replace(
            "<!--SCRIPT-->",
            "<script defer src=\"scripts/settings\"></script>",
        ),
        _ => head,
    }
}
