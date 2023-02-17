use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use mongo_db::MongoRepo;

mod api;
mod mongo_db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(db_data.clone())
            // add new routes here
            .service(api::grid_routes::get_grid)
            .service(api::grid_routes::post_grid)
    })
    .bind(("localhost", 80))?
    .run()
    .await
}
