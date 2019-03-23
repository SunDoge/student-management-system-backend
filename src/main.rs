#[macro_use]
extern crate diesel;

mod app;
mod models;
mod schema;
mod student_routes;

use crate::models::DbExecutor;
use actix::prelude::*;
use actix_web::server;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sys = actix::System::new("student_management_system");

    // create db connection pool
    let manager = ConnectionManager::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let db = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    let addr = "127.0.0.1:3000";

    server::new(move || app::create_app(db.clone()))
        .bind(addr)
        .expect(&format!("Cannot bind to {}", addr))
        .start();

    println!("start at http://{}", addr);

    sys.run();
}
