use crate::models::DbExecutor;
use crate::student_routes::fetch_all_students;
use actix::prelude::*;
use actix_web::{http::Method, middleware::Logger, App};

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
    App::with_state(AppState { db })
        .middleware(Logger::default())
        .scope("/api/v1", |api| {
            api.resource("/students", |r| {
                r.method(Method::GET).with(fetch_all_students);
            })
            .resource("/departments", |r| {})
        })
}
