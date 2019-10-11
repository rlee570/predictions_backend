#![feature(proc_macro_hygiene, decl_macro)]
use hyper;

use rocket::http::{hyper::header::Orign, ContentType, Header, Status};
use rocket::local::Client;
use rocket_cors::*;
use std::str::FromStr;

#[test]
fn check_user_is_returned_from_cors_request() {
    let client = Client::new(rocket()).unwrap();
    let origin_header =
        Header::from(hyper::header::Origin::from_str("http://localhost:8000").unwrap());
    let method_header = Header::from(hyper::header::AccessControlRequestMethod(
        hyper::method::Method::Get,
    ));
    let request_headers =
        hyper::header::AccessControlRequestHeaders(vec![
            FromStr::from_str("Authorization").unwrap()
        ]);
    let req = client
        .options("/api/user/1")
        .header(origin_header)
        .header(method_header)
        .header(request_headers);
    let responce = req.dispatch();
    assert!(responce.status().class().is_success());


}
