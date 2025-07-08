mod anytls;
mod hysteria2;
mod impl_trait;
mod tool;
mod vless;

use crate::{dns, r#type::Error, unwrap_or, Result, User};
use anytls::Anytls;
use hysteria2::Hysteria2;
use serde_yaml_ok::{self as yaml, Mapping};
use std::{borrow::Cow, collections::HashMap, net::Ipv6Addr};
use vless::Vless;

pub struct ClashStyleProxies(Vec<Mapping>);
pub trait TryIntoClashStyleProxies {
    async fn try_into_clash_style_proxies(
        self,
        user: &User,
        query: &HashMap<Cow<'_, str>, Cow<'_, str>>,
    ) -> Result<ClashStyleProxies>;
}
impl TryIntoClashStyleProxies for Vec<Mapping> {
    async fn try_into_clash_style_proxies(
        self,
        user: &User,
        query: &HashMap<Cow<'_, str>, Cow<'_, str>>,
    ) -> Result<ClashStyleProxies> {
        let mut new_proxies = vec![];
        for mut proxy in self {
            if !proxy
                .remove("show")
                .and_then(|v| yaml::from_value::<Vec<String>>(v).ok())
                .map_or(true, |list| list.contains(user.name()))
                || proxy
                    .remove("ignore")
                    .and_then(|v| yaml::from_value::<Vec<String>>(v).ok())
                    .map_or(false, |list| list.contains(user.name()))
            {
                continue;
            }

            user.insert_password(&mut proxy);

            insert_up_and_down(
                &mut proxy,
                query.contains_key("tcp-brutal"),
                query.get("up"),
                query.get("down"),
            )
            .unwrap_or_else(|_| {
                let smux = unwrap_or!(return, proxy.get_mut("smux")).as_mapping_mut();
                unwrap_or!(return, smux).remove("brutal-opts");
            });

            if let Some(server) = proxy.get("server").and_then(|s| s.as_str()) {
                let proxy = parse_name(proxy.clone(), server)?;
                new_proxies.push(proxy);
            }

            let Some(domain): Option<String> = proxy
                .remove("server-resolve")
                .and_then(|s| yaml::from_value(s).ok())
            else {
                continue;
            };

            async fn resolve(
                proxy: &Mapping,
                is_ipv4: bool,
                new_proxies: &mut Vec<Mapping>,
                domain: &String,
            ) -> Result {
                for ip in dns::query(domain, if is_ipv4 { "A" } else { "AAAA" }).await? {
                    let mut proxy = parse_name(proxy.clone(), &ip)?;
                    proxy.insert("server".into(), ip.into());
                    new_proxies.push(proxy);
                }
                Ok(())
            }
            if proxy.remove("ipv4").and_then(|ipv4| ipv4.as_bool()) != Some(false) {
                resolve(&proxy, true, &mut new_proxies, &domain).await?;
            }
            if proxy.remove("ipv6").and_then(|ipv6| ipv6.as_bool()) != Some(false) {
                resolve(&proxy, false, &mut new_proxies, &domain).await?;
            }
        }
        Ok(ClashStyleProxies(new_proxies))
    }
}
fn parse_name(mut proxy: Mapping, server: &str) -> Result<Mapping> {
    let name =
        yaml::from_value::<String>(proxy.remove("name").expect("Must provide `name` of proxy"))?
            .replace("{server}", server);
    proxy.insert("name".into(), name.into());
    Ok(proxy)
}
fn insert_up_and_down(
    proxy: &mut Mapping,
    enable_tcp_brutal: bool,
    up: Option<&Cow<'_, str>>,
    down: Option<&Cow<'_, str>>,
) -> Result {
    let up: &str = up.ok_or("Up was unset")?;
    let down: &str = down.ok_or("Down was unset")?;
    match proxy
        .get("type")
        .and_then(|v| v.as_str())
        .expect("Must specify proxy type")
    {
        "hysteria2" | "hysteria" if !proxy.contains_key("up") && !proxy.contains_key("down") => {
            proxy.insert("up".into(), up.into());
            proxy.insert("down".into(), down.into());
        }
        _ => {
            let brutal_opts = proxy
                .get_mut("smux")
                .and_then(|smux| smux.as_mapping_mut())
                .and_then(|smux| smux.get_mut("brutal-opts"))
                .and_then(|brutal_opts| brutal_opts.as_mapping_mut())
                .ok_or("Brutal opts not found")?;
            (enable_tcp_brutal
                && brutal_opts
                    .get("enabled")
                    .and_then(|enabled| enabled.as_bool())
                    .is_some_and(|flag| flag))
            .then_some(true)
            .ok_or("Brutal was not enabled")?;
            brutal_opts.insert("up".into(), up.into());
            brutal_opts.insert("down".into(), down.into());
        }
    };
    Ok(())
}

pub struct UrlStyleProxies(String);
impl UrlStyleProxies {
    pub fn into_string(self) -> String {
        self.0
    }
}
impl TryInto<UrlStyleProxies> for ClashStyleProxies {
    type Error = Error;
    fn try_into(self) -> Result<UrlStyleProxies> {
        let mut urls = vec![];
        for mut proxy in self.0 {
            if let Some(ip) = proxy
                .get("server")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<Ipv6Addr>().ok())
            {
                proxy.insert("server".into(), format!("[{ip}]").into());
            }
            urls.push(
                match proxy
                    .get("type")
                    .and_then(|t| t.as_str())
                    .expect("Must specify proxy type")
                {
                    "hysteria2" => Some(yaml::from_value::<Hysteria2>(proxy.into())?.into_string()),
                    "anytls" => Some(yaml::from_value::<Anytls>(proxy.into())?.into_string()),
                    "vless" => Some(yaml::from_value::<Vless>(proxy.into())?.into_string()),
                    _ => None,
                },
            );
        }
        let urls: Vec<_> = urls.into_iter().flatten().collect();
        Ok(UrlStyleProxies(urls.join("\n")))
    }
}
