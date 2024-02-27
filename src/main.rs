#[macro_use] extern crate rocket;
mod test;
#[get("/")]
fn index() -> &'static str {
    "Hello, wo8888888888888888888rld!"
}
#[get("/")]
fn find() -> &'static str {
    "Hello, find!"
}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/find", routes![find])
    .mount("/post", routes![test ::sendData])
    .mount("/delay", routes![test ::delay])
    .mount("/postmethod", routes![test ::postmethod])
    
    // rocket::build()
}