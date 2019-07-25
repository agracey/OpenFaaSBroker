#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

extern crate rocket;

use rocket::{routes};

mod catalog;
mod service_instances;
mod faas_store;

fn main() {
    rocket::ignite()
        .mount("/v2/catalog", routes![catalog::handler])
        .mount("/v2/service_instances", routes![service_instances::handler])
        .launch();
}
