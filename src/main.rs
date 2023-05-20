mod server;

#[actix_web::main]
async fn main() {
    server::connection::launch().await.unwrap();
}
