use rocket::{build, catch, catchers, main, response::content::RawHtml, State, routes, get};
use std::sync::atomic::{AtomicUsize, Ordering};

#[catch(404)]
fn not_found() -> RawHtml<String> {
    RawHtml(String::from("<h1>Hmm... What are you looking for?</h1>"))
}

#[get("/")]
fn index(hit_count: &State<HitCount>) -> RawHtml<String> {
    let count = hit_count.0.fetch_add(1, Ordering::Relaxed) + 1;
    RawHtml(format!("<h1>Your visit is recorded!<br /><br />Visits: {}</h1>", count))
}

#[main]
async fn main() {
    let _ = build().register("/", catchers![not_found])
        .mount("/count", routes![index])
        .manage(HitCount(AtomicUsize::new(0)))
        .launch().await;
}
struct HitCount(AtomicUsize);


