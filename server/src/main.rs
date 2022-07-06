mod pool;

use crate::pool::Db;
use rocket::{
    fairing::AdHoc,
    fs::FileServer,
    get, routes,
    serde::{json::Json, Serialize},
};
use sea_orm_rocket::{Connection, Database};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    users: Vec<String>,
}

#[get("/")]
async fn list(conn: Connection<'_, Db>) -> Json<Message> {
    use entity::prelude::User;
    use sea_orm::EntityTrait;

    let db = conn.into_inner();
    let users = User::find().all(db).await.expect("all users");

    Json::from(Message {
        users: users.into_iter().map(|user| user.name).collect(),
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Run migrations", pool::run_migrations))
        .mount("/", FileServer::from("static"))
        .mount("/api", routes![list])
        .launch()
        .await
        .expect("launch");
}
