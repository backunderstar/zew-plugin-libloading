use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExampleResponse {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}
