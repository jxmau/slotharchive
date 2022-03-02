use serde::{Serialize, Deserialize};
use serde_json::json;

use super::test::Test;

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub name : String,
    pub path : String,
    pub tests : Vec<Test>,
}

impl Collection {

    pub fn create(name: &str, path: &str) -> Self {
        let tests = vec![Test::create("Hello World", "/", Some(200))];
        Self {name: name.into(), path: path.into(), tests}
    }

    pub fn to_json(&self) -> String {
        json!({
            "name": &self.name,
            "path": &self.path,
            "tests": &self.tests
        }).to_string()
    }

}