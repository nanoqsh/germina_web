use rocket::{
    fs::FileServer,
    get, routes,
    serde::{json::Json, Serialize},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'a> {
    text: &'a str,
}

#[get("/")]
pub async fn index() -> Json<Message<'static>> {
    let message = Message { text: "hello" };
    message.into()
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/api", routes![index])
        .launch()
        .await
        .expect("launch");
}
