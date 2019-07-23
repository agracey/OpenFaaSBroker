#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

extern crate rocket;

use rocket::{routes};

mod catalog;

fn main() {
    rocket::ignite()
        .mount("/catalog", routes![catalog::handler])
        .launch();
}
