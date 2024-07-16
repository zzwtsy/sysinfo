use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IPIP {
    /// IP 地址
    pub ip: String,
    /// 地理位置信息
    pub location: Location,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// 国家代码
    pub country_code: String,
    // /// 国家名称
    // pub country_name: String,
    // pub province: String,
    // pub city: String,
    // pub latitude: String,
    // pub longitude: String,
}
