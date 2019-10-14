use hyper;

use rocket::http::{hyper::header::Origin, ContentType, Header, Status};
use rocket::local::Client;
use rocket_cors::*;
use std::str::FromStr;
use zuehlke_predictions_backend as backend;
use hyper::Response;

//Not working currently
//#[test]
//fn check_user_is_returned_from_cors_request() {
//    let client = Client::new(backend::start()).unwrap();
//    let origin_header =
//        Header::from(hyper::header::Origin::from_str("http://localhost:8000").unwrap());
//    let method_header = Header::from(hyper::header::AccessControlRequestMethod(
//        hyper::method::Method::Get,
//    ));
//    let request_headers =
//        hyper::header::AccessControlRequestHeaders(vec![
//            FromStr::from_str("Authorization").unwrap()
//        ]);
//    let req = client
//        .options("/api/users/1")
//        .header(origin_header)
//        .header(method_header)
//        .header(request_headers);
//    let responce = req.dispatch();
//    assert!(responce.status().class().is_success());
//}

#[test]
fn check_rocket_server_launches() {
    let rocket = backend::start();
    let client = Client::new(rocket).expect("Should be valid rocket instance");
    let req = client.get("/");
    let responce = req.dispatch();
    println!("{:?}", responce)
}

#[test]
fn should_return_a_valid_user(){
    let rocket = backend::start();
    let client = Client::new(rocket).expect("Should be valid rocket instance");
    let req = client.get("/api/user/1");
    let responce = req.dispatch();
    assert_eq!(Status::Ok,responce.status());
}