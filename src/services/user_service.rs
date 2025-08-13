use crate::models::user::User;

pub fn get_user_info() -> User {
    User {
        name: "VinitNagar".to_string(),
        age: 21,
        email: "vinitnagar56@gmail.com".to_string(),
        profession: "Just Engineer".to_string(),
    }
}
