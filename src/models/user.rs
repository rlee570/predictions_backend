use diesel::Queryable;
use serde::Serialize;
use frank_jwt as jwt;
use chrono::{Duration,Utc};
use crate::models::user_config as config;
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
pub struct UserJWT{
    id:i32,
    email: String,
    first_name:String,
    last_name: String,
    token: String
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Payload{
    pub expiry:i64,
    pub id: i32,
    pub email:String
}

impl Payload{
    pub fn new(expiry:i64,id:i32,email:&str) ->Payload{
        Payload{
            expiry,
            id,
            email:email.to_string()
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
        let exp = Utc::now() + Duration::minutes(15);
        let payload = json!(Payload::new(exp.timestamp(),self.id,self.email.as_str()));
        let token = jwt::encode(
            headers.0,
            &config::SECRET.to_string(),
            &payload,
            jwt::Algorithm::HS256
        ).expect("jwt");
        let jwt_token = UserJWT {
            id:self.id,
            email:self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            token
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
