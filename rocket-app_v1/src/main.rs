use rocket::http::Status;
use rocket::{build, catch, catchers, main, Request};

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("Response from default catcher -> {} ({})", status, req.uri())
}

#[main]
async fn main() {
    let _ = build().register("/", catchers![default]).launch().await;
}
