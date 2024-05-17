use std::error::Error;
use std::result::Result;

use crate::modules::core::application::service::ApplicationService;

pub async fn start_server(app_service: &ApplicationService, server_port: u16) -> Result<(), Box<dyn Error>> {
    // Define the app routes
    let app = app_service.configure_router();

    // Define the server address
    let app_log_address = format!("127.0.0.1:{}", server_port);
    let app_bind_address = format!("0.0.0.0:{}", server_port);
    
    println!("Listening on http://{}", app_log_address);

    // Run our app with hyper, listening globally on the specified port
    let listener = tokio::net::TcpListener::bind(&app_bind_address)
        .await
        .unwrap();
    match axum::serve(listener, app).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}