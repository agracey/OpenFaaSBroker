extern crate reqwest;
extern crate serde_derive;

use std::env;

use reqwest::{Error, Url};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewFunctionRequest {
    pub image: String,
    pub service: String,
    pub labels: Option<HashMap<String, String>>,
    pub annotations: Option<HashMap<String, String>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeployedFunction {
    pub name: String,
    pub image: String,
    pub invocation_count: u64,
    pub replicas: u32,
    pub env_process: String,
    pub available_replicas: u32,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFunctionRequest {
    pub function_name: String,
}
pub fn deploy_function(service_name: String, image: String) -> Result<String, Error> {
    println!("Running deploy_function: {} -- {}", service_name, image);

    let url = env::var("OPENFAAS_URL").unwrap();
    let url = &format!("{}/system/functions", url);
    let url = Url::parse(url).unwrap();

    let req = NewFunctionRequest {
        service: service_name.clone(),
        image: image.clone(),
        annotations:None,
        labels:None,
    };

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&req)
        .basic_auth("admin", Some(env::var("OPENFAAS_PASSWORD").unwrap()))
        .send()?;

    Ok(service_name)
}

pub fn update_function(service_name: String, image: String) -> Result<String, Error> {
    println!("Running deploy_function: {} -- {}", &service_name, &image);

    let url = env::var("OPENFAAS_URL").unwrap();
    let url = &format!("{}/system/functions", url);
    let url = Url::parse(url).unwrap();

    let req = NewFunctionRequest {
        service: service_name.clone(),
        image: image.clone(),
        annotations:None,
        labels:None,
    };

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&req)
        .basic_auth("admin", Some(env::var("OPENFAAS_PASSWORD").unwrap()))
        .send()?;

    Ok(service_name)
}

pub fn undeploy_function(service_name: String) -> Result<String, Error> {
    println!("Running undeploy_function: {} ", service_name);

    let req = DeleteFunctionRequest {
      function_name: service_name.clone()
    };

    let url = env::var("OPENFAAS_URL").unwrap();
    let url = &format!("{}/system/functions", url);
    let url = Url::parse(url).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .delete(url)
        .json(&req)
        .basic_auth("admin", Some(env::var("OPENFAAS_PASSWORD").unwrap()))
        .send()?;    
    Ok(service_name)
}

pub fn get_deployed_functions() -> Result<Vec<DeployedFunction>, Error> {
    println!("Running get_deployed_functions");

    let url = env::var("OPENFAAS_URL").unwrap();
    let url = &format!("{}/system/functions", url);
    let url = Url::parse(url).unwrap();

    let client = reqwest::Client::new();
    let deployed_functions = client
        .get(url)
        .basic_auth("admin", Some(env::var("OPENFAAS_PASSWORD").unwrap()))
        .send()?
        .json::<Vec<DeployedFunction>>()?;
    Ok(deployed_functions)
}
