extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate serde_derive;

use rocket::get;
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use super::faas_store;
use super::faas_store::FaasFunction;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SchemaPart {
    r#type: String,
    properties: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaListInstance {
    pub create: SchemaPart,
    pub update: SchemaPart,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaListBinding {
    pub create: SchemaPart,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaList {
    pub service_instance: SchemaListInstance,
    pub service_binding: SchemaListBinding,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub description: String,
    pub free: bool,
    pub bindable: bool,
    pub schemas: SchemaList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub name: String,
    pub id: String,
    pub description: String,
    pub tags: Vec<String>,
    pub bindable: bool,
    pub instances_retrievable: bool,
    pub bindings_retrievable: bool,
    pub allow_context_updates: bool,
    pub plan_updateable: bool,
    pub plans: Vec<Plan>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog {
    pub services: Vec<Service>,
}

fn build_plan((k, v): (&String, &String)) -> Plan {
    let empty_schema = SchemaPart {
        r#type: String::from("object"),
        properties: String::from(""),
    };

    Plan {
        id: k.clone(),
        name: k.clone(),
        description: format!("Run function on: {}", v),
        free: true,
        bindable: true,
        schemas: SchemaList {
            service_instance: SchemaListInstance {
                create: empty_schema.clone(),
                update: empty_schema.clone(),
            },
            service_binding: SchemaListBinding {
                create: empty_schema.clone(),
            },
        },
    }
}

fn build_plans(func: &FaasFunction) -> Vec<Plan> {
    func.images.iter().map(build_plan).collect()
}

fn build_service_from_function(func: &FaasFunction) -> Service {
    Service {
        name: func.title.clone(),
        id: func.name.clone(),
        description: func.description.clone(),
        tags: vec![String::from("Function")],
        bindable: true,
        instances_retrievable: true,
        bindings_retrievable: true,
        allow_context_updates: true,
        plan_updateable: true,
        plans: build_plans(func),
    }
}

fn build_service_list_from_functions(store: faas_store::FaasStore) -> Vec<Service> {
    store
        .functions
        .iter()
        .map(build_service_from_function)
        .collect()
}

#[get("/")]
pub fn handler() -> Json<Catalog> {
    let function_list = faas_store::get_store().unwrap();

    let debug = serde_json::to_string(&function_list).unwrap();
    println!("Catalog {}", debug);

    let c = Catalog {
        services: build_service_list_from_functions(function_list),
    };

    Json(c)
}
