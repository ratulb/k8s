use rocket::{build, main};

#[main]
async fn main() {
    let _ = build().launch().await;
}
