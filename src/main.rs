mod modules;

use dotenv::dotenv;
use modules::core::infrastructure::web_server;
use modules::core::application::service::ApplicationService;
use modules::user;
use std::env;

#[tokio::main]
async fn main() {
    // Loads environment variables from a .env
    dotenv().ok();

    let server_port = env::var("APP_PORT").unwrap_or_else(|_| String::from("8080")); // Default to 8080 if not set
    let server_port: u16 = server_port.parse().unwrap_or_else(|_| {
        eprintln!("Invalid port number provided. Using default port 8080.");
        8080
    });

    let mut app_service = ApplicationService::new();
    app_service.register_module(user::module);

    // Use the parsed server_port instead of hardcoding it
    if let Err(e) = web_server::start_server(&app_service, server_port).await {
        eprintln!("Failed to start server: {}", e);
    }
}