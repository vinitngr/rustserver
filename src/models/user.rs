use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub profession: String,
}
