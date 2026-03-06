mod calculation;
mod handlers;
mod models;
mod server;

use server::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::from_env();
    server::start_server(settings).await
}
