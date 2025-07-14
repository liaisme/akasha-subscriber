use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};

use crate::model::proxy::tool::{encode_array, param};

#[derive(Serialize, Deserialize)]
pub(super) struct Hysteria2 {
    name: String,
    password: String,
    server: String,
    port: Option<u16>,
    ports: Option<String>,
    sni: Option<String>,
    up: Option<String>,
    down: Option<String>,
    alpn: Option<Vec<String>>,
    fingerprint: Option<String>,
    obfs: Option<String>,
    #[serde(rename = "obfs-password")]
    obfs_password: Option<String>,
    #[serde(rename = "skip-cert-verify")]
    skip_cert_verify: Option<bool>,
}
impl Hysteria2 {
    pub(super) fn into_string(self) -> String {
        let Hysteria2 {
            name,
            password,
            server,
            port,
            ports,
            sni,
            up,
            down,
            alpn,
            obfs,
            obfs_password,
            skip_cert_verify,
            fingerprint,
        } = self;
        let name = utf8_percent_encode(&name, NON_ALPHANUMERIC);
        let skip_cert_verify = skip_cert_verify.map(|flag| flag as u8);
        let alpn = alpn.map(encode_array);
        let ports = ports.map_or_else(
            || port.map(|port| port.to_string()).unwrap_or_default(),
            |ports| ports.replace("/", ","),
        );
        format!(
            "hysteria2://{password}@{server}:{ports}{}#{name}",
            match [
                sni.map(param("sni")),
                obfs.map(param("obfs")),
                obfs_password.map(param("obfs-password")),
                alpn.map(param("alpn")),
                skip_cert_verify.map(param("insecure")),
                fingerprint.map(param("pinSHA256")),
                up.map(param("up")),
                down.map(param("down")),
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
