
use actix_web::{web, App, HttpServer};
use rocksdb::{RocksDbManager, RocksDbPool};
pub mod api;
pub mod frost;
pub mod rocksdb;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = RocksDbManager::new("wallet_db");
    let pool = RocksDbPool::builder().max_size(1).build(manager).unwrap();
    println!("db is ready");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Inject pool
            .service(api::init)
            .service(api::create_wallet)
            .service(api::sign_message)
            .service(api::recover)
            .service(api::verify)
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}
