use rusqlite::Connection;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use std::env;
use rusqlite::DatabaseName::Main;

pub fn establish_connection() -> SqliteConnectionManager {
    let db_name = env::var("DB_NAME");
    match db_name {
        Ok(name) => SqliteConnectionManager::file(name),
        Err(e) => panic!("{}", e),
    }
}

pub fn add_foreign_keys(conn: &mut Connection) {
    let res = conn.pragma_update(Option::from(Main), "foreign_keys", "ON");
    if let Err(e) = res {
        panic!("{}", e);
    }
}

pub fn create_pool(manager: SqliteConnectionManager) -> Pool<SqliteConnectionManager> {
    #[derive(Debug)]
    struct SqliteForeignKey {}
    impl r2d2::CustomizeConnection::<Connection, rusqlite::Error> for SqliteForeignKey {
        fn on_acquire(&self, conn: &mut Connection) -> Result<(), rusqlite::Error> {
            Ok(add_foreign_keys(conn))
        }
    }

    Pool::builder().connection_customizer(Box::new(SqliteForeignKey {})).build(manager).expect("Failed to create pool.")
}
