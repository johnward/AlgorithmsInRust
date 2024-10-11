use std::sync::Arc;

// https://docs.rs/shaku/latest/shaku/
use shaku::{module, Component, HasComponent, Interface};

// Define the trait (interface) for the service
trait Service: Interface {
    fn do_something(&self) -> String;
}

// Implement the trait for a concrete service
#[derive(Component)]
#[shaku(interface = Service)]
struct ConcreteService;

impl Service for ConcreteService {
    fn do_something(&self) -> String {
        String::from("Service is doing something")
    }
}

// Define a client that depends on the service
struct Client {
    service: Arc<dyn Service>,
}

impl Client {
    fn new(service: Arc<dyn Service>) -> Self {
        Self { service }
    }

    fn perform_action(&self) -> String {
        self.service.do_something()
    }
}

// Define a module to provide the service
module! {
    MyModule {
        components = [ConcreteService],
        providers = []
    }
}

fn main() {
    // Create the module
    let module: MyModule = MyModule::builder().build();

    // Get the service from the module
    let service: Arc<dyn Service> = module.resolve();

    // Inject the service into the client
    let client: Client = Client::new(service);

    // Use the client
    println!("{}", client.perform_action());
}
