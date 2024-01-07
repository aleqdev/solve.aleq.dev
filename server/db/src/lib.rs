use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod orm;
pub mod schema;

pub use diesel;
pub use diesel_migrations;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
