use serde::{Deserialize, Serialize};
use serde_yaml_ok::{self as yaml, Mapping, Value};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    password: String,
    #[serde(rename = "32bit-password")]
    threety_two_bit_password: Option<String>,
    #[serde(rename = "16bit-password")]
    sixteen_bit_password: Option<String>,
    path: String,
}
impl User {
    pub const fn name(&self) -> &String {
        &self.name
    }

    pub const fn path(&self) -> &String {
        &self.path
    }

    pub fn insert_password(&self, proxy: &mut Mapping) {
        match proxy
            .get("type")
            .expect("Must be provide `type` of proxy")
            .as_str()
            .expect("`type` of proxy must be a string")
        {
            "vless" | "vmess" => {
                if proxy.get("uuid").is_none() {
                    proxy.insert("uuid".into(), self.password.as_str().into());
                }
            }
            "hysteria2" => {
                if proxy.get("password").is_none() {
                    let password = match proxy.remove("no-user-name") {
                        Some(Value::Bool(true)) => &self.password,
                        _ => &format!("{}:{}", self.name(), self.password),
                    };
                    proxy.insert("password".into(), password.as_str().into());
                }
            }
            "anytls" => {
                if proxy.get("password").is_none() {
                    proxy.insert("password".into(), self.password.clone().into());
                }
            }
            "ss" => {
                if proxy.get("password").is_none() {
                    if let Some(passwd) = match &*proxy
                        .get("cipher")
                        .and_then(|t| yaml::from_value::<String>(t.to_owned()).ok())
                        .unwrap_or_default()
                    {
                        "2022-blake3-aes-128-gcm" => self
                            .sixteen_bit_password
                            .as_ref()
                            .map(format_password(proxy)),
                        "2022-blake3-aes-256-gcm" => self
                            .threety_two_bit_password
                            .as_ref()
                            .map(format_password(proxy)),

                        _ => Some(self.password.clone()),
                    } {
                        proxy.insert("password".into(), passwd.as_str().into());
                    }
                }
            }
            r#type => panic!("Unsupported proxy type: {type}"),
        };
    }
}

fn format_password(proxy: &mut Mapping) -> impl FnMut(&String) -> String + '_ {
    move |passwd| {
        format!(
            "{}:{passwd}",
            proxy
                .remove("server-password")
                .expect("2022-blake3-* use muti-user must provide `server-password`")
                .as_str()
                .unwrap()
        )
    }
}
