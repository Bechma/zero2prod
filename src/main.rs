use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to load the configuration file.");
    let address = format!("{}:{}", config.application.host, config.application.port);

    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to the database");
    run(
        TcpListener::bind(address).expect("Failed to bind port 8000"),
        connection,
    )?
    .await
}
