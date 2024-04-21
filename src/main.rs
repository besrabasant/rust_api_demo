mod modules;

use modules::core::application::service::ApplicationService;
use modules::core::infrastructure::web_server;
use dotenv::dotenv;
use std::env;

#[actix_web::main] // Marks the entry point of the application
async fn main() -> std::io::Result<()> {
    // Loads environment variables from a .env
    dotenv().ok();

    let server_port = env::var("APP_PORT").unwrap_or_else(|_| String::from("8080")); // Default to 8080 if not set
    let server_port: i16 = server_port.parse().unwrap_or_else(|_| {
        eprintln!("Invalid port number provided. Using default port 8080.");
        8080
    });

    // let app_service = ApplicationService::new();
    ApplicationService::new();
    // other services and dependencies would be initialized and passed here
    
    web_server::start_server(server_port).await
}

