use std::ops::Not;

use crate::{
    dto::{ip_sb::IPSB, ipip::IPIP},
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

impl GeoIp {
    fn default() -> Self {
        GeoIp {
            country_code: "".to_string(),
            ip_v4: "".to_string(),
            ip_v6: "".to_string(),
        }
    }
}

pub async fn fetch_geo_ip() -> GeoIp {
    let http_util = HttpUtil::new();
    let ip_sb = fetch_ip_sb(&http_util).await;
    if ip_sb.is_err().not() {
        return ip_sb.unwrap();
    }
    let ipip = fetch_ipip(&http_util).await;
    if ipip.is_err().not() {
        return ipip.unwrap();
    }
    return GeoIp::default();
}

async fn fetch_ip_sb(http_util: &HttpUtil) -> anyhow::Result<GeoIp> {
    let ipv4_json = http_util.send_get::<IPSB>(&IP_SB_V4).await;
    let ipv4 = match ipv4_json {
        Ok(ipv4_json) => ipv4_json,
        Err(_) => IPSB::default(),
    };

    let ipv6_json = http_util.send_get::<IPSB>(&IP_SB_V6).await;
    let ipv6 = match ipv6_json {
        Ok(ipv6_json) => ipv6_json,
        Err(_) => IPSB::default(),
    };

    if ipv4.ip == "" && ipv6.ip == "" {
        return Err(anyhow::anyhow!("ip_sb error"));
    }

    Ok(GeoIp {
        country_code: if ipv4.country_code == "" {
            ipv6.country_code
        } else {
            ipv4.country_code
        },
        ip_v4: ipv4.ip,
        ip_v6: ipv6.ip,
    })
}

async fn fetch_ipip(http_util: &HttpUtil) -> anyhow::Result<GeoIp> {
    let ipv4_json = http_util.send_get_on_ipv4::<IPIP>(&IPIP).await;
    let ipv4 = match ipv4_json {
        Ok(ipv4_json) => ipv4_json,
        Err(_) => IPIP::default(),
    };

    let ipv6_json = http_util.send_get_on_ipv6::<IPIP>(&IPIP).await;
    let ipv6 = match ipv6_json {
        Ok(ipv6_json) => ipv6_json,
        Err(_) => IPIP::default(),
    };

    if ipv4.ip == "" && ipv6.ip == "" {
        return Err(anyhow::anyhow!("ipip error"));
    }

    Ok(GeoIp {
        country_code: if ipv4.location.country_code == "" {
            ipv6.location.country_code
        } else {
            ipv4.location.country_code
        },
        ip_v4: ipv4.ip,
        ip_v6: ipv6.ip,
    })
}
