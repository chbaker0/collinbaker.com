use rocket::fs::{relative, FileServer};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/static", FileServer::from(relative!("public")))
}
