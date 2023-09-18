#![feature(once_cell_try)]

pub use diesel;
pub use r2d2;
pub mod schema;
pub mod types;

use diesel::{r2d2::ConnectionManager, MysqlConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use r2d2::{Error, Pool, PooledConnection, State};
use std::{env, sync::OnceLock};
use tracing::debug;

pub fn pool<'a>() -> Result<&'a Pool<ConnectionManager<MysqlConnection>>, Error> {
    static POOL: OnceLock<Pool<ConnectionManager<MysqlConnection>>> = OnceLock::new();

    Ok(POOL.get_or_try_init(|| {
        debug!("Init DB pool");
        Pool::builder().build(ConnectionManager::<MysqlConnection>::new(
            &env::var("DATABASE_URL").unwrap_or_default(), // Default is empty string ""
        ))
    })?)
}

pub fn migrate() -> Result<(), Error> {
    const MIGRATIONS: EmbeddedMigrations = diesel_migrations::embed_migrations!();

    debug!(
        "{:?}",
        MigrationHarness::revert_all_migrations(&mut connection()?, MIGRATIONS)
    );

    debug!(
        "{:?}",
        MigrationHarness::run_pending_migrations(&mut connection()?, MIGRATIONS)
    );

    Ok(())
}

pub fn pool_state() -> Result<State, Error> {
    Ok(pool()?.state())
}

pub fn connection() -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, Error> {
    Ok(pool()?.get()?)
}
