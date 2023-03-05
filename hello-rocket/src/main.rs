#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn home(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Movie<'r> {
    title: &'r str,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![home])
}
