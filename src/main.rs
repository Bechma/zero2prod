use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to load the configuration file.");
    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to the database");
    run(
        TcpListener::bind(format!(
            "{}:{}",
            config.application_host, config.application_port
        ))
        .expect("Failed to bind port 8000"),
        connection,
    )?
    .await
}
