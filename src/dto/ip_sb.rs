use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IPSB {
    /// IP 地址
    pub ip: String,
    /// 国家代码
    pub country_code: String,
}

impl IPSB {
    pub fn default() -> Self {
        IPSB {
            ip: "".to_string(),
            country_code: "".to_string(),
        }
    }
}
