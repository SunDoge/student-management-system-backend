use crate::errors::ServiceError;
use crate::models::{DbExecutor, Department};
use actix::{Handler, Message};
use chrono::{Duration, Local};
use diesel::prelude::*;

#[derive(Debug, Deserialize, PartialEq, Queryable)]
pub struct CreateDepartment {
    pub name: String,
}

impl Message for CreateDepartment {
    type Result = Result<Department, ServiceError>;
}

impl Handler<CreateDepartment> for DbExecutor {
    type Result = Result<Department, ServiceError>;

    fn handle(&mut self, msg: CreateDepartment, _: &mut Self::Context) -> Self::Result {
        use crate::schema::department::dsl::*;

        let conn = &self.0.get().unwrap();

        diesel::insert_into(department)
            .values(name.eq("some"))
            .execute(conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        let mut items = department
            .filter(name.eq(msg.name))
            .load(conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(items.pop().unwrap())
    }
}
