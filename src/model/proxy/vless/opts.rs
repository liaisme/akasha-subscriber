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
    #[serde(rename = "v2ray-http-upgrade")]
    pub v2ray_http_upgrade: Option<bool>,
    #[serde(rename = "v2ray-http-upgrade-fast-open")]
    pub v2ray_http_upgrade_fast_open: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "Host")]
    pub host: Option<String>,
}
