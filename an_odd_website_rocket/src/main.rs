mod router;
use crate::router::{
    get_data::get_data, get_script::get_script, get_styles::get_styles, get_webpage::get_webpage,
    new_client::new_client, remove_clients::remove_client, update_clients::update_client,
};

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            get_webpage,
            get_styles,
            get_data,
            get_script,
            new_client,
            remove_client,
            update_client
        ],
    )
}
