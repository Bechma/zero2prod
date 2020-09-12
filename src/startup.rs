use crate::routes;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/healthcheck", web::get().to(routes::healthcheck))
            .route("/subscriptions", web::post().to(routes::subscriptions))
            // Our pool is already wrapped in an Arc pointer:
            // using .data would add another Arc pointer on top
            // of the existing one - an unnecessary indirection.
            // .app_data instead does not perform an additional layer of wrapping.
            .app_data(conn.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
