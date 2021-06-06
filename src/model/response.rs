use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: usize,
    pub data: T,
    pub version: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data<T> {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUpdate {
    pub code: usize,
    pub message: String,
    pub version: usize,
}
