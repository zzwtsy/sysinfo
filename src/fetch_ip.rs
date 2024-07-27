use std::ops::Not;

use crate::{
    dto::{IPAPI, IPIP, IPSB},
    utils::http_util::HttpUtil,
};

static IP_SB_V6_URL: &'static str = "https://api-ipv6.ip.sb/geoip";
static IP_SB_V4_URL: &'static str = "https://api-ipv4.ip.sb/geoip";
static IPIP_URL: &'static str = "https://api.myip.la/en?json";
static IPAPI_URL: &'static str = "https://ipapi.co/json";

#[derive(Default)]
pub struct GeoIp {
    pub country_code: String,
    pub ipv4: String,
    pub ipv6: String,
}

pub async fn fetch_geo_ip() -> GeoIp {
    let http_util = HttpUtil::new();
    let ipip = fetch_ipip(&http_util).await;
    if ipip.is_err().not() {
        return ipip.unwrap();
    }
    let ipapi = fetch_ipapi(&http_util).await;
    if ipapi.is_err().not() {
        return ipapi.unwrap();
    }
    let ip_sb = fetch_ip_sb(&http_util).await;
    if ip_sb.is_err().not() {
        return ip_sb.unwrap();
    }
    return GeoIp::default();
}

async fn fetch_ip_sb(http_util: &HttpUtil) -> anyhow::Result<GeoIp> {
    let ipv4 = http_util
        .send_get::<IPSB>(&IP_SB_V4_URL)
        .await
        .unwrap_or_else(|_| IPSB::default());

    let ipv6 = http_util
        .send_get::<IPSB>(&IP_SB_V6_URL)
        .await
        .unwrap_or_else(|_| IPSB::default());

    if ipv4.ip == "" && ipv6.ip == "" {
        eprintln!("ip.sb failed to obtain the local ip address");
        return Err(anyhow::anyhow!(
            "ip.sb failed to obtain the local ip address"
        ));
    }

    Ok(GeoIp {
        country_code: if ipv4.country_code == "" {
            ipv6.country_code
        } else {
            ipv4.country_code
        },
        ipv4: ipv4.ip,
        ipv6: ipv6.ip,
    })
}

async fn fetch_ipip(http_util: &HttpUtil) -> anyhow::Result<GeoIp> {
    let ipv4 = http_util
        .send_get_on_ipv4::<IPIP>(&IPIP_URL)
        .await
        .unwrap_or_else(|_| IPIP::default());

    let ipv6 = http_util
        .send_get_on_ipv6::<IPIP>(&IPIP_URL)
        .await
        .unwrap_or_else(|_| IPIP::default());

    if ipv4.ip == "" && ipv6.ip == "" {
        eprintln!("ipip.net failed to obtain the local ip address");
        return Err(anyhow::anyhow!(
            "ipip.net failed to obtain the local ip address"
        ));
    }

    Ok(GeoIp {
        country_code: if ipv4.location.country_code == "" {
            ipv6.location.country_code
        } else {
            ipv4.location.country_code
        },
        ipv4: ipv4.ip,
        ipv6: ipv6.ip,
    })
}

async fn fetch_ipapi(http_util: &HttpUtil) -> anyhow::Result<GeoIp> {
    let ipv4 = http_util
        .send_get_on_ipv4::<IPAPI>(&IPAPI_URL)
        .await
        .unwrap_or_else(|_| IPAPI::default());

    let ipv6 = http_util
        .send_get_on_ipv6::<IPAPI>(&IPAPI_URL)
        .await
        .unwrap_or_else(|_| IPAPI::default());
    if ipv4.ip == "" && ipv6.ip == "" {
        eprintln!("ipapi.co failed to obtain the local ip address");
        return Err(anyhow::anyhow!(
            "ipapi.co failed to obtain the local ip address"
        ));
    }
    return Ok(GeoIp {
        country_code: if ipv4.country_code == "" {
            ipv6.country_code
        } else {
            ipv4.country_code
        },
        ipv4: ipv4.ip,
        ipv6: ipv6.ip,
    });
}
