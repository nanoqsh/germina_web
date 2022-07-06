use rocket::{async_trait, fairing, Build, Rocket};
use sea_orm::{ConnectOptions, DatabaseConnection, DbErr};
use sea_orm_rocket::{rocket::figment::Figment, Config, Database};
use std::time::Duration;

#[derive(Database)]
#[database("sqlite")]
pub struct Db(Pool);

#[derive(Debug, Clone)]
pub struct Pool {
    conn: DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for Pool {
    type Error = DbErr;
    type Connection = DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config: Config = figment.extract().expect("extract");
        let mut options: ConnectOptions = config.url.into();
        options
            .max_connections(config.max_connections as u32)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout));
        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        let conn = sea_orm::Database::connect(options).await?;

        Ok(Self { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    use migration::MigratorTrait;

    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;

    Ok(rocket)
}
