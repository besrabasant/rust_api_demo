use axum::Router;


/// `ApplicationService` manages configuration modules for an Axum `Router`.
/// Each module is a function that configures part of the service.
pub struct ApplicationService {
    pub modules: Vec<Box<dyn Fn(Router) -> Router + Send + Sync>>,
 }

impl ApplicationService {
    pub fn new() -> Self {
        ApplicationService {
           modules: Vec::new(),
        }
    }

    /// Registers a module with the service. Each module is a function that configures routes or other parts of the web application.
    /// 
    /// # Arguments
    /// * `config` - A function that conforms to `Fn(Router) -> Router + 'static + Send + Sync`, 
    ///   intended to configure part of an Axum `Router`.
    pub fn register_module<M>(&mut self, config: M)
    where
        M: Fn(Router) -> Router + 'static + Send + Sync,
    {
        self.modules.push(Box::new(config));
    }

    /// Configures an Axum `Router` by applying all registered modules.
    /// This method iterates over each module and applies its configuration to the `Router`.
    /// 
    /// Returns a configured `Router`.
    pub fn configure_router(&self) -> Router {
        self.modules.iter().fold(Router::new(), |router, module| module(router))
    }
}
