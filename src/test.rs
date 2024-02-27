use rocket::tokio::time::{sleep, Duration};
use std::io;
use rocket::serde::{Deserialize};
use rocket::serde::{Serialize, json::Json};

use rocket::data::{Data, ToByteUnit};
// #[derive(Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct Input<'r> {
//     description: &'r str,
//     complete: bool
// }
#[derive(Deserialize, Serialize)]
struct UserData {
    id: i64,
    name: String,
    lastname: String,
    roll_no: u32,
}

#[get("/")]
pub fn sendData() -> &'static str {
    "Hello, world! from post"
}
#[get("/<seconds>")]
pub async fn delay(seconds:u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)    
}

#[post("/", data = "<data>")]
pub async fn postmethod(data: Data<'_>) -> Json<UserData> {
    let bytes = data.open(1024.kibibytes()).into_bytes().await.unwrap();
    let request_body = String::from_utf8(bytes.to_vec()).unwrap();

    let user_data: UserData = serde_json::from_str(&request_body).unwrap();
    Json(user_data)
}