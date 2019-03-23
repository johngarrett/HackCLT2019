#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use rocket_contrib::json::JsonValue;

mod heatmap_data;

use heatmap_data::simplify_heatmap_data;

#[get("/heatmap_data")]
fn heatmap() -> Option<JsonValue> {
    simplify_heatmap_data()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![heatmap])
        .launch();
}
