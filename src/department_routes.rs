use crate::app::AppState;
use crate::department_handler::CreateDepartment;
use actix_web::error::ResponseError;
use actix_web::{FutureResponse, HttpResponse, Json, State, AsyncResponder};
use futures::future::Future;

pub fn create_department(
    (department, state): (Json<CreateDepartment>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(department.into_inner())
        .from_err()
        .and_then(|db_response| {
            match db_response {
                // 这里是错误的示范,用string作为status非常蠢,用status也是不正确的做法
                Ok(department) => Ok(HttpResponse::Ok().json(json!({
                    "status": "ok",
                    "new_id": department.id,
                }))),
                Err(err) => Ok(err.error_response()),
            }
        })
        .responder()
}
