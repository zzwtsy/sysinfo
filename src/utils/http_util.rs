use std::{net::IpAddr, str::FromStr};

use reqwest::{Client, ClientBuilder, Error};

pub struct HttpUtil {
    client: Client,
    ipv4_client: Client,
    ipv6_client: Client,
}

impl HttpUtil {
    pub fn new() -> Self {
        let ipv4_addr = IpAddr::from_str("0.0.0.0").unwrap();
        let ipv6_addr = IpAddr::from_str("::").unwrap();
        let http_util = HttpUtil {
            client: Client::new(),
            ipv4_client: ClientBuilder::new()
                .local_address(ipv4_addr)
                .build()
                .expect(""),
            ipv6_client: ClientBuilder::new()
                .local_address(ipv6_addr)
                .build()
                .expect(""),
        };
        http_util
    }
    /// 发送 get 请求
    pub async fn send_get(&self, url: &str) -> Result<String, Error> {
        let resp = self.client.get(url).send().await?;
        let text = resp.text().await?;
        Ok(text)
    }

    pub async fn send_get_on_ipv4(&self, url: &str) -> Result<String, Error> {
        let resp = self.ipv4_client.get(url).send().await?;
        let text = resp.text().await?;
        Ok(text)
    }
    pub async fn send_get_on_ipv6(&self, url: &str) -> Result<String, Error> {
        let resp = self.ipv6_client.get(url).send().await?;
        let text = resp.text().await?;
        Ok(text)
    }
}
