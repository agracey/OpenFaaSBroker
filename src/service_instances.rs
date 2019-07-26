extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;

use rocket::{delete, get, patch, put};
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct LastOperation {
    pub state: String,
    pub description: String,
    pub instance_usable: Option<bool>,
    pub update_repeatable: Option<bool>,
}

#[get("/<instance_id>/last_operation")]
pub fn instance_last_operation(instance_id: String) -> Json<LastOperation> {
    let lo = LastOperation {
        state: String::from("succeeded"),
        description: String::from("succeeded"),
        instance_usable: Some(true),
        update_repeatable: Some(true),
    };
    Json(lo)
}

#[get("/<instance_id>/service_bindings/<binding_id>/last_operation")]
pub fn binding_last_operation(instance_id: String, binding_id: String) -> Json<LastOperation> {
    let lo = LastOperation {
        state: String::from("succeeded"),
        description: String::from("succeeded"),
        instance_usable: Some(true),
        update_repeatable: Some(true),
    };

    Json(lo)
}

#[derive(Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    pub service_id: String,
    pub plan_id: String,
    pub context: Option<HashMap<String, String>>,
    pub parameters: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateInstanceResponse {
    pub dashboard_url: String,
    pub operation: Option<String>,
}

#[put("/<instance_id>", format = "application/json", data = "<instance>")]
pub fn create_instance(
    instance_id: String,
    instance: Json<CreateInstanceRequest>,
) -> Json<CreateInstanceResponse> {
    let res = CreateInstanceResponse {
        dashboard_url: String::from("http://openfaas.tm.suse.com/ui/"),
        operation: None,
    };

    Json(res)
}

#[derive(Serialize, Deserialize)]
pub struct InstanceResponse {
    pub service_id: String,
    pub plan_id: String,
    pub dashboard_url: String,
    pub parameters: Option<HashMap<String, String>>,
}

#[get("/<instance_id>")]
pub fn get_instance(instance_id: String) -> Json<InstanceResponse> {
    let res = InstanceResponse {
        dashboard_url: String::from("http://openfaas.tm.suse.com/ui/"),
        plan_id: String::from("plan_id"),
        service_id: String::from("service_id"),
        parameters: None,
    };

    Json(res)
}

#[derive(Serialize, Deserialize)]
pub struct PreviousValues {
    pub service_id: String,
    pub plan_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateInstanceRequest {
    pub service_id: String,
    pub plan_id: String,
    pub dashboard_url: String,
    pub parameters: Option<HashMap<String, String>>,
    pub previous_values: PreviousValues,
}

#[patch("/<instance_id>", format = "application/json", data = "<instance>")]
pub fn update_instance(
    instance_id: String,
    instance: Json<UpdateInstanceRequest>,
) -> Json<CreateInstanceResponse> {
    let res = CreateInstanceResponse {
        dashboard_url: String::from("url"),
        operation: None,
    };

    Json(res)
}

#[derive(Serialize, Deserialize)]
pub struct BindResource {
    pub app_guid: Option<String>,
    pub route: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BindInstanceRequest {
    pub context: Option<HashMap<String, String>>,
    pub service_id: String,
    pub plan_id: String,
    pub bind_resource: Option<BindResource>,
    pub parameters: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct BindInstanceResponse {
    pub credentials: Option<HashMap<String, String>>,
    pub parameters: Option<HashMap<String, String>>,
    pub endpoints: Option<Vec<HashMap<String, String>>>,
}

#[put(
    "/<instance_id>/service_bindings/<binding_id>",
    format = "application/json",
    data = "<instance>"
)]
pub fn bind_instance(
    instance_id: String,
    binding_id: String,
    instance: Json<BindInstanceRequest>,
) -> Json<BindInstanceResponse> {
    let res = BindInstanceResponse {
        credentials: None,
        parameters: None,
        endpoints: None,
    };

    Json(res)
}

#[get("/<instance_id>/service_bindings/<binding_id>")]
pub fn get_binding(instance_id: String, binding_id: String) -> Json<BindInstanceResponse> {
    let res = BindInstanceResponse {
        credentials: None,
        parameters: None,
        endpoints: None,
    };

    Json(res)
}

#[derive(Serialize, Deserialize)]
pub struct DeleteResponse {
    operation: String,
}

#[delete("/<instance_id>/service_bindings/<binding_id>")]
pub fn delete_binding(instance_id: String, binding_id: String) -> Json<DeleteResponse> {
    let res = DeleteResponse {
        operation: String::from("service_id"),
    };

    Json(res)
}

#[delete("/<instance_id>")]
pub fn delete_instance(instance_id: String) -> Json<DeleteResponse> {
    let res = DeleteResponse {
        operation: String::from("service_id"),
    };

    Json(res)
}
