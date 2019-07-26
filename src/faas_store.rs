extern crate reqwest;
extern crate serde_derive;

use reqwest::Error;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct FaasFunction {
    pub title: String,
    pub name: String,
    pub description: String,
    pub images: HashMap<String, String>,
    pub repo_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct FaasStore {
    pub functions: Vec<FaasFunction>,
    pub version: String,
}

pub fn get_store() -> Result<FaasStore, Error> {
    let function_list: FaasStore =
        reqwest::get("https://raw.githubusercontent.com/openfaas/store/master/functions.json")?
            .json::<FaasStore>()?;
    Ok(function_list)
}
