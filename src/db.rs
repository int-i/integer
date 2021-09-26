use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::PgConnection;

#[database("integer")]
pub struct Conn(PgConnection);
