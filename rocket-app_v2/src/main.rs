use rocket::{build, catch, catchers, main, response::content::RawHtml};

#[catch(404)]
fn not_found() -> RawHtml<String> {
    RawHtml(String::from("<h1>Hmm... What are you looking for?</h1>"))
}

#[main]
async fn main() {
    let _ = build().register("/", catchers![not_found]).launch().await;
}
