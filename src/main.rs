use guards::AuthGuard;
use rocket::outcome::Outcome;
use rocket::{get, routes, serde};
use rocket::{ http::Status, request::{self, FromRequest}, serde::{Deserialize}, Request};
 
mod guards;

#[get("/public")]
fn public_route() -> &'static str {
    "Esta es una ruta pública, no se requiere autenticación."
}

#[get("/private")]
fn private_route(user: AuthGuard) -> String {
    format!("Bienvenido, {}. Esta es una ruta privada.", user.0.name)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let server=rocket::build()
        .mount("/", routes![public_route, private_route]);
    server.launch().await?;

    Ok(())
}
