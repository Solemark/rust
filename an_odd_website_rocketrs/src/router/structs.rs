use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Client {
    pub id: usize,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub visible: bool,
}

#[derive(FromForm)]
pub struct NewClient {
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
}

#[derive(FromForm)]
pub struct UpdateClient {
    pub client_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub visible: String,
}

#[derive(FromForm)]
pub struct RemoveClient {
    pub client_id: usize,
}
