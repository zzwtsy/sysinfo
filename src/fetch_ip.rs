use crate::{
    dto::{ip_sb, ipip},
    utils::http_util::HttpUtil,
};

static IP_SB_V6: &'static str = "https://api-ipv6.ip.sb/geoip";
static IP_SB_V4: &'static str = "https://api-ipv4.ip.sb/geoip";
static IPIP: &'static str = "https://api.myip.la/en?json";

pub struct GeoIp {
    pub country_code: String,
    pub ip_v4: String,
    pub ip_v6: String,
}

pub async fn fetch_geo_ip() -> GeoIp {
    let http_util = HttpUtil::new();
    let ipv4_text = http_util.send_get_on_ipv4(&IPIP).await.unwrap();
    let ipv4_json = serde_json::from_str::<ipip::IPIP>(&ipv4_text).unwrap();
    let ipv6_text = http_util.send_get_on_ipv6(&IPIP).await.unwrap();
    let ipv6_json = serde_json::from_str::<ipip::IPIP>(&ipv6_text).unwrap();
    let geo_ip = GeoIp {
        country_code: ipv4_json.location.country_code,
        ip_v4: ipv4_json.ip,
        ip_v6: ipv6_json.ip,
    };
    geo_ip
}
