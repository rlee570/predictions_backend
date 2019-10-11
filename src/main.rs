#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_cors;

use rocket::http::Method;
use rocket_contrib::json::JsonValue;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};

mod user;
use user::User;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8000",
        "http://localhost:3000",
        "http://0.0.0.0:8000",
        "http://0.0.0.0:3000",
        "http://127.0.0.1:8000",
        "http://127.0.0.1:3000",
    ]);
    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[get("/")]
fn index() -> &'static str {
    "Hello,World"
}

#[get("/<id>")]
fn get_user(id: i32) -> JsonValue {
    let mut user = User::new(id, "Some", "Person", "some_person@email.com", 100);
    user.set_first_name("Jeff");
    json!(user)
}

pub fn start(){
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api/user", routes![get_user])
        .attach(make_cors())
        .launch();
}

fn main() {
    start();
}
