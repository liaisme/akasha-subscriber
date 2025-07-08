use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};

use crate::model::proxy::tool::param;

#[derive(Serialize, Deserialize)]
pub(super) struct Anytls {
    name: String,
    password: String,
    server: String,
    port: Option<u16>,
    sni: Option<String>,
    #[serde(rename = "skip-cert-verify")]
    skip_cert_verify: Option<bool>,
}
impl Anytls {
    pub(super) fn into_string(self) -> String {
        let Anytls {
            name,
            password,
            server,
            port,
            sni,
            skip_cert_verify,
        } = self;
        let name = utf8_percent_encode(&name, NON_ALPHANUMERIC);
        format!(
            "anytls://{password}@{server}{}{}#{name}",
            port.map(|p| ":".to_string() + &p.to_string())
                .unwrap_or("".to_string()),
            match [
                sni.map(param("sni")),
                skip_cert_verify.map(|b| b as u8).map(param("insecure"))
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>()
            {
                params if params.is_empty() => "".into(),
                params => format!("?{}", params.join("&")),
            }
        )
    }
}
