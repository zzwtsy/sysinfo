use serde::{Deserialize, Serialize};

/// 系统信息
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    #[serde(rename = "host")]
    pub host: Host,
    #[serde(rename = "state")]
    pub state: State,
    /// 主机 IPV4 地址
    #[serde(rename = "ipV4")]
    pub ip_v4: String,
    /// 主机 IPV6 地址
    #[serde(rename = "ipV6")]
    pub ip_v6: String,
    /// 上报时间
    #[serde(rename = "uploadTime")]
    pub upload_time: String,
}

/// 主机信息
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    /// 平台
    #[serde(rename = "platform")]
    pub platform: String,
    /// 平台版本
    #[serde(rename = "platformVersion")]
    pub platform_version: String,
    /// CPU信息
    #[serde(rename = "cpu")]
    pub cpu: Vec<String>,
    /// 总内存
    #[serde(rename = "memTotal")]
    pub mem_total: u64,
    /// 总磁盘空间
    #[serde(rename = "diskTotal")]
    pub disk_total: u64,
    /// 总交换空间
    #[serde(rename = "swapTotal")]
    pub swap_total: u64,
    /// 系统架构
    #[serde(rename = "arch")]
    pub arch: String,
    /// 开机时间
    #[serde(rename = "bootTime")]
    pub boot_time: u64,
    /// 国家/地区代码
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// 探针版本
    #[serde(rename = "version")]
    pub version: String,
}

/// 状态信息
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    /// CPU 使用率
    #[serde(rename = "cpu")]
    pub cpu: f32,
    /// 已使用内存
    #[serde(rename = "memUsed")]
    pub mem_used: u64,
    /// 已使用交换空间
    #[serde(rename = "swapUsed")]
    pub swap_used: u64,
    /// 已使用磁盘空间
    #[serde(rename = "diskUsed")]
    pub disk_used: u64,
    /// 网络接收数据量
    #[serde(rename = "netInTransfer")]
    pub net_in_transfer: u64,
    /// 网络发送数据量
    #[serde(rename = "netOutTransfer")]
    pub net_out_transfer: u64,
    /// 网络接收速度
    #[serde(rename = "netInSpeed")]
    pub net_in_speed: u64,
    /// 网络发送速度
    #[serde(rename = "netOutSpeed")]
    pub net_out_speed: u64,
    /// 运行时间
    #[serde(rename = "uptime")]
    pub uptime: u64,
    /// 1分钟平均负载
    #[serde(rename = "load1")]
    pub load1: f64,
    /// 5分钟平均负载
    #[serde(rename = "load5")]
    pub load5: f64,
    /// 15分钟平均负载
    #[serde(rename = "load15")]
    pub load15: f64,
}
