use zero2prod::run;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("0.0.0.0:8000").expect("Failed to bind port 8000"))?.await
}
