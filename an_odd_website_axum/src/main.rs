mod router;

#[tokio::main]
async fn main() {
    router::routes::routing().await;
}
