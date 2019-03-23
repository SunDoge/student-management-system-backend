use actix_web::{HttpRequest, Responder};
use crate::app::AppState;

pub fn fetch_all_students(_: HttpRequest<AppState>) -> impl Responder {
    "fetch_all_students".to_string()
}
