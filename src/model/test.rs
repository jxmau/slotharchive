use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub name: String,
    pub path: String,
    pub status: Option<u8>,
}

impl Test {

    pub fn create(name: &str, path: &str, status: Option<u8>) -> Self {
        Self {name: name.into(), path: path.into(), status: status.into()}
    }

    pub fn to_json(&self) -> String {
        json!({
            "name": &self.name,
            "path": &self.path,
            "status": &self.status
        }).to_string()
    }
}