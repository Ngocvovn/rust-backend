use rust_backend::configuration::get_configuration;
use rust_backend::startup::run;
use sqlx::{Connection, PgPool};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.get_connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("{}", address);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
