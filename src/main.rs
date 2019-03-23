#[macro_use]
extern crate diesel;

mod app;
mod schema;
mod models;

use std::env;
use dotenv::dotenv;
use actix::prelude::*;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sys = actix::System::new("student_management_system");


    println!("Hello, world!");
}
