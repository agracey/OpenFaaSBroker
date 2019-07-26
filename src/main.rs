#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

extern crate rocket;

use rocket::routes;

mod catalog;
mod faas_store;
mod openfaas_client;
mod service_instances;

fn main() {
    rocket::ignite()
        .mount("/v2/catalog", routes![catalog::handler])
        .mount(
            "/v2/service_instances",
            routes![
                service_instances::instance_last_operation,
                service_instances::binding_last_operation,
                service_instances::create_instance,
                service_instances::get_instance,
                service_instances::update_instance,
                service_instances::bind_instance,
                service_instances::get_binding,
                service_instances::delete_binding,
                service_instances::delete_instance
            ],
        )
        .launch();
}
