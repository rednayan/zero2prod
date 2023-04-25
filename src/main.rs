use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read  configuration");
    let address = format!("127.0.0.1:{port}", port = configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener)?.await
}
