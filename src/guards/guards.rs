use rocket::outcome::Outcome;
use rocket::{ http::Status, request::{self, FromRequest}, serde::{Deserialize}, Request};


#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name: String,
}

pub struct AuthGuard(pub User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
     type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()>{
        let token = request.headers().get_one("Authorization");

        if let Some(token) = token {
            if token == "valid_token" {
                let user = User {
                    name: "John Doe".into(),
                };
                Outcome::Success(AuthGuard(user))
            } else {
                Outcome::Error((Status::Unauthorized, ()))  
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))  
        }
    }
}