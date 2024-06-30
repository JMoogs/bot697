pub mod bdo;

// const DATABASE_URL: &str = "sqlite://BDO_item_data.sqlite";
use std::sync::OnceLock;

use sqlx::{Pool, Sqlite};

static DATABASE_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

pub fn get_database() -> &'static Pool<Sqlite> {
    DATABASE_POOL
        .get()
        .expect("called 'get_db()' before initializing the database")
}

pub fn set_database(db: Pool<Sqlite>) {
    DATABASE_POOL
        .set(db)
        .unwrap_or_else(|_| panic!("called 'set_db()' more than once"))
}
