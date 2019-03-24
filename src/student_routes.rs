use crate::app::AppState;
use actix_web::{HttpRequest, Responder};

pub fn fetch_all_students(_: HttpRequest<AppState>) -> impl Responder {
    "fetch_all_students".to_string()
}
