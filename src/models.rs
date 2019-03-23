use diesel::r2d2::{Pool, ConnectionManager};
use diesel::prelude::*;
use actix::{Actor, SyncContext};

/// This is db executor actor. Can be run in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}