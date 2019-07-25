extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;

use reqwest::Error;
use rocket::{get};
use rocket_contrib::json::Json;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};


#[get("/")]
pub fn handler() -> String {
    String::from("Hello, World")
}
