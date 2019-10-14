use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub points: i32,
}

impl User {
    pub fn new(id: i32, first_name: &str, last_name: &str, email: &str, password:&str, points: i32) -> User {
        User {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            password:password.to_string(),
            points,
        }
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
}
