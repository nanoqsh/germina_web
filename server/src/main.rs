mod pool;

use crate::pool::{run_migrations, Pool};
use rocket::{
    fairing::AdHoc,
    fs::FileServer,
    get, routes,
    serde::{json::Json, Serialize},
};
use rocket_db_pools::{Connection, Database};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message<'a> {
    text: &'a str,
}

#[get("/")]
async fn index(_db: Connection<Pool>) -> Json<Message<'static>> {
    let message = Message { text: "hello" };
    message.into()
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(Pool::init())
        .attach(AdHoc::try_on_ignite("Run migrations", run_migrations))
        .mount("/", FileServer::from("static"))
        .mount("/api", routes![index])
        .launch()
        .await
        .expect("launch");
}
