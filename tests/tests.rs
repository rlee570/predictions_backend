use hyper;

use hyper::Response;
use rocket::http::{hyper::header::Origin, ContentType, Header, Status};
use rocket::local::Client;
use rocket_cors::*;
use std::str::FromStr;
use zuehlke_predictions_backend as backend;

#[test]
fn check_rocket_server_launches() {
    let rocket = backend::start();
    let client = Client::new(rocket).expect("Should be valid rocket instance");
    let req = client.get("/api");
    let responce = req.dispatch();
    assert_eq!(Status::Ok, responce.status());
}

#[test]
fn should_return_a_valid_user() {
    let rocket = backend::start();
    let client = Client::new(rocket).expect("Should be valid rocket instance");
    let req = client.get("/api/user/1");
    let responce = req.dispatch();
    assert_eq!(Status::Ok, responce.status());
}

#[test]
fn should_create_a_user() {}
