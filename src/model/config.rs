use std::{collections::HashMap};

use serde::{Serialize, Deserialize};
use serde_json::json;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Name of the Sloth repo.
    pub name: String,
    /// HTTP base to whish the path with be appended.
    pub base: String,
    /// List of all the environment variables.
    pub vars: HashMap<String, String>
}

impl Config {

    pub fn init(name: &str) -> String {
        let vars: HashMap<String, String> = HashMap::new();
        json!({
            "name": name,
            "domain": "",
            "vars": vars
            
        }).to_string()
    }


}
