use crate::models::user::User;
use rocket_contrib::json::JsonValue;

#[get("/user/<id>")]
pub fn get_user(id: i32) -> JsonValue {
    let mut user = User::new(
        id,
        "Some",
        "Person",
        "some_person@email.com",
        100,
        "USER",
        "ahjsdbasjhbdjh"
    );
    user.set_first_name("Jeff");
    json!(user)
}
