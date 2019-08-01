extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;

use rocket::{delete, get, patch, put};
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;


use super::openfaas_client;
use super::faas_store;


#[derive(Serialize, Deserialize, Debug)]
pub struct LastOperation {
    pub state: String,
    pub description: String,
    pub instance_usable: Option<bool>,
    pub update_repeatable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInstanceRequest {
    pub service_id: String,
    pub plan_id: String,
    pub context: Option<HashMap<String, String>>,
    pub parameters: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInstanceResponse {
    pub dashboard_url: String,
    pub operation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceResponse {
    pub service_id: String,
    pub plan_id: String,
    pub dashboard_url: String,
    pub parameters: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousValues {
    pub service_id: String,
    pub plan_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateInstanceRequest {
    pub service_id: String,
    pub plan_id: String,
    pub parameters: Option<HashMap<String, String>>,
    pub previous_values: PreviousValues,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BindResource {
    pub app_guid: Option<String>,
    pub route: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BindInstanceRequest {
    pub context: Option<HashMap<String, String>>,
    pub service_id: String,
    pub plan_id: String,
    pub bind_resource: Option<BindResource>,
    pub parameters: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BindInstanceResponse {
    pub credentials: Option<HashMap<String, String>>,
    pub parameters: Option<HashMap<String, String>>,
    pub endpoints: Option<Vec<HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteResponse {
    operation: Option<String>,
}

fn build_dashboard_url () -> String {
    let url = env::var("OPENFAAS_URL").unwrap();
    format!("{}/ui/", url)
}

// Checks if function exists. Says succeeded if does, in progress if not
#[get("/<instance_id>/last_operation")]
pub fn instance_last_operation(instance_id: String) -> Json<LastOperation> {
    let len = openfaas_client::get_deployed_functions().unwrap()
        .iter()
        .filter(| fun | fun.name == instance_id)
        .collect::<Vec<&openfaas_client::DeployedFunction>>()
        .len();

    let exists = len == 1;
    let state = if exists {"succeeded"} else {"in progress"} ;

    let lo = LastOperation {
        state: String::from(state),
        description: String::from(format!("function {} is {}", instance_id, state)),
        instance_usable: Some(exists),
        update_repeatable: Some(exists),
    };
    Json(lo)
}

// Since binidng is no_op, always succeeded
#[get("/<instance_id>/service_bindings/<binding_id>/last_operation")]
pub fn binding_last_operation(instance_id: String, binding_id: String) -> Json<LastOperation> {
    
    
    let lo = LastOperation {
        state: String::from("succeeded"),
        description: String::from("Binding of function is a noop"),
        instance_usable: Some(true),
        update_repeatable: Some(true),
    };

    Json(lo)
}

// Gets info about instance.
#[get("/<instance_id>")]
pub fn get_instance(instance_id: String) -> Json<InstanceResponse> {
    
    
    let function = openfaas_client::get_deployed_functions().unwrap()
        .iter()
        .filter(| fun | fun.name == instance_id)
        .collect::<Vec<&openfaas_client::DeployedFunction>>()
        .first();


    let dashboard_url = env::var("OPENFAAS_URL").unwrap();
    let dashboard_url = &format!("{}/ui/", dashboard_url);
    

    //TODO: what should even go here (plan_id)?
    let res = InstanceResponse {
        dashboard_url: String::from(dashboard_url),
        plan_id: String::from("plan_id"),
        service_id: instance_id,
        parameters: None,
    };

    Json(res)
}

#[put("/<instance_id>", format = "application/json", data = "<instance>")]
pub fn create_instance(
    instance_id: String,
    instance: Json<CreateInstanceRequest>,
) -> Json<CreateInstanceResponse> {

    let function_name = format!("{}-{}", instance.service_id.clone(), &instance_id);
    
    
    let image_name = faas_store::get_image_for_arch(instance.service_id.clone(), instance.plan_id.clone());

    openfaas_client::deploy_function(function_name, image_name);

    let res = CreateInstanceResponse {
        dashboard_url: String::from("http://openfaas.tm.suse.com/ui/"),
        operation: Some(instance_id),
    };

    Json(res)
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

// Binding is no_op so just return what's expected
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

    let url = env::var("OPENFAAS_URL").unwrap();
    let url = &format!("{}/functions/{}", url, instance_id);

    let mut credentials = HashMap::new();

    credentials.insert(String::from("url"), String::from(url));

    let res = BindInstanceResponse {
        credentials: Some(credentials),
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


// Binds are no ops
#[delete("/<instance_id>/service_bindings/<binding_id>")]
pub fn delete_binding(instance_id: String, binding_id: String) -> Json<DeleteResponse> {
    let res = DeleteResponse {
        operation: None,
    };

    Json(res)
}

#[delete("/<instance_id>")]
pub fn delete_instance(instance_id: String) -> Json<DeleteResponse> {

    let func: Option<openfaas_client::DeployedFunction> = openfaas_client::get_deployed_functions().unwrap()
        .into_iter()
        .find(|fun| fun.name.to_string().ends_with(&instance_id));
        
    
    if(func.is_some()) {
        openfaas_client::undeploy_function(func.unwrap().name);
    }


    let res = DeleteResponse {
        operation: None,
    };

    Json(res)
}
