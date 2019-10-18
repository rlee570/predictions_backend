use crate::models::user_config as config;
use chrono::{Duration, Utc, TimeZone};
use diesel::Queryable;
use frank_jwt as jwt;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{Outcome, Request};
use serde::Serialize;
//use diesel_derive_enum::DbEnum;

//#[derive(DbEnum)]
//pub enum UserRole {
//    ADMIN,
//    USER,
//}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub points: i32,
    pub role: String,
    #[serde(skip_serializing)]
    pub hash: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserJWT {
    pub id: i32,
    pub email: String,
    pub token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Payload {
    type Error = ();
    /// Extract Auth token from the "Authorization" header.
    ///
    /// Handlers with Auth guard will fail with 503 error.
    /// Handlers with Option<Auth> will be called with None.
    fn from_request(request: &'a Request<'r>) -> Outcome<Payload, (Status, Self::Error), ()> {
        if let Some(Payload) = extract_user_jwt_token_from_request(request) {
            Outcome::Success(Payload)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_user_jwt_token_from_request(request: &Request) -> Option<Payload> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(decode_token)
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

fn decode_token(token: &str) -> Option<Payload> {
    jwt::decode(token.trim(), &config::SECRET.to_string(), jwt::Algorithm::HS256)
        .map(|(_, payload)| {
            serde_json::from_value::<Payload>(payload)
                .map_err(|err| {
                    eprintln!("Decode failed: {:?}", err);
                })
                .ok()
        })
        .unwrap_or_else(|err| {
            eprintln!("Jwt decode failed: {:?}", err);
            None
        })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub expiry: i64,
    pub id: i32,
    pub email: String,
}

impl Payload {
    pub fn new(expiry: i64, id: i32, email: &str) -> Payload {
        Payload {
            expiry,
            id,
            email: email.to_string(),
        }
    }
}

impl User {
    pub fn new(
        id: i32,
        first_name: &str,
        last_name: &str,
        email: &str,
        points: i32,
        role: &str,
        hash: &str,
    ) -> User {
        User {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            points,
            role: role.to_string(),
            hash: hash.to_string(),
        }
    }

    pub fn to_user_jwt(&self) -> UserJWT {
        let headers = json!({});
        let exp = Utc::now() + Duration::days(30);
        let timestamp = exp.timestamp();
        let payload = json!(Payload::new(timestamp, self.id, self.email.as_str()));
        let token = jwt::encode(
            headers.0,
            &config::SECRET.to_string(),
            &payload,
            jwt::Algorithm::HS256,
        )
        .expect("jwt");
        let jwt_token = UserJWT {
            id: self.id,
            email: self.email.clone(),
            token,
        };
        jwt_token
    }

    pub fn set_first_name(&mut self, first_name: &str) {
        self.first_name = first_name.to_string();
    }

    pub fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string();
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }

    pub fn set_points(&mut self, points: i32) {
        self.points = points
    }

    pub fn set_role(&mut self, role: &str) {
        self.role = role.to_string()
    }
}
