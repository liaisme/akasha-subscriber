use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct H2 {
    pub host: Option<Vec<String>>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Reality {
    #[serde(rename = "public-key")]
    pub public_key: Option<String>,
    #[serde(rename = "short-id")]
    pub short_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Ws {
    pub path: Option<String>,
    pub headers: Option<Headers>,
}

#[derive(Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "Host")]
    pub host: Option<String>,
}
