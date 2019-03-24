use crate::schema::{department, student};
use actix::{Actor, SyncContext};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

/// This is db executor actor. Can be run in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "student"]
pub struct Student {
    pub id: i32,
    pub number: String,
    pub name: String,
    pub department: i32,
    pub create_date: NaiveDateTime,
    pub last_updated: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "department"]
pub struct Department {
    pub id: i32,
    pub name: Option<String>,
    pub create_date: Option<NaiveDateTime>,
    pub last_updated: Option<NaiveDateTime>,
}
