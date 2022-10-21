use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database(sqlx)]
struct Db(sqlx::MysqlPool);

