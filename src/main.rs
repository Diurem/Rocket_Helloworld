#[macro_use] extern crate rocket;

#[get("/world")]
fn sayworld() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![sayworld])
}
