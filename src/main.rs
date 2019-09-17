#![feature(proc_macro_hygiene, decl_macro)]
use motivations::MOTIVATIONS;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::{get, routes};
use rocket_contrib;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let mut rng = thread_rng();
    let &crab = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
    ]
    .choose(&mut rng)
    .unwrap();
    let &motivation = MOTIVATIONS.choose(&mut rng).unwrap();
    let mut context = HashMap::new();
    context.insert("motivation", motivation);
    context.insert("image", crab);
    Template::render("motivation", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount(
            "/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
