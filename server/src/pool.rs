use rocket::{fairing, Build, Rocket};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("sqlite")]
pub struct Pool(sqlx::SqlitePool);

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    use migration::{Migrator, MigratorTrait};
    use sea_orm::SqlxSqliteConnector;

    let pool = Pool::fetch(&rocket).expect("fetch").0.clone();
    let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
    Migrator::up(&db, None).await.expect("migration up");

    Ok(rocket)
}
