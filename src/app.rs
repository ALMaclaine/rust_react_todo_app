use r2d2::ManageConnection;
use dotenv::dotenv;
use r2d2_sqlite::SqliteConnectionManager;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/migrations");
}

fn run_migrations(manager: &SqliteConnectionManager) {
    let conn_res = manager.connect();
    match conn_res {
        Ok(mut conn) => {
            db::lite::add_foreign_keys(&mut conn);
            embedded::migrations::runner().run(&mut conn).unwrap();
        },
        Err(e) => panic!("{}", e),
    }
}

pub fn app() {
    dotenv().ok();
    let manager = db::lite::establish_connection();
    run_migrations(&manager);
    let _pool = db::lite::create_pool(manager);
}
