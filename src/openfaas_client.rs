extern crate reqwest;
extern crate serde_derive;

use std::env;

use reqwest::{Error, Url};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct NewFunctionRequest {
    image: String,
    service: String,
}

fn deploy_function(service_name: String, image: String) -> Result<String, Error> {
    let url = Url::parse(&env::var("OPENFAAS_URL").unwrap()).unwrap();

    let req = NewFunctionRequest {
        service: service_name.clone(),
        image: image.clone(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&req)
        .basic_auth("admin", Some(env::var("OPENFAAS_PASSWORD").unwrap()))
        .send()?;

    Ok(service_name)
}

pub fn get_deployed_functions() -> Result<Vec<String>, Error> {
    Ok(vec![String::from("HI")])
}
