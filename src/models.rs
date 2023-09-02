use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub song: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub song: String,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}
