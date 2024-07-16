use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// 系统信息
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemInfo {
    pub host: Host,
    pub state: State,
    /// 上报时间
    pub upload_time: String,
}

/// 主机信息
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Host {
    /// 平台
    pub os_name: String,
    /// 平台版本
    pub os_version: String,
    /// 系统版本
    pub long_os_version: String,
    /// 内核版本
    pub kernel_version: String,
    /// CPU信息
    pub cpu: HashSet<String>,
    /// CPU核心数
    pub cpu_cores: u64,
    /// 总内存
    pub mem_total: u64,
    /// 总磁盘空间
    pub disk_total: u64,
    /// 总交换空间
    pub swap_total: u64,
    /// 系统架构
    pub arch: String,
    /// 开机时间 Unix 时间戳(秒)
    pub boot_time: u64,
    /// 主机 IPV4 地址
    pub ip_v4: String,
    /// 主机 IPV6 地址
    pub ip_v6: String,
    /// 国家代码
    pub country_code: String,
    /// 探针版本
    pub agent_version: String,
}

/// 状态信息
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    /// CPU 使用率
    pub cpu: f32,
    /// 已使用内存
    pub mem_used: u64,
    /// 已使用交换空间
    pub swap_used: u64,
    /// 已使用磁盘空间
    pub disk_used: u64,
    /// 网络接收数据量
    pub net_in_transfer: u64,
    /// 网络发送数据量
    pub net_out_transfer: u64,
    /// 网络接收速度
    pub net_in_speed: u64,
    /// 网络发送速度
    pub net_out_speed: u64,
    /// 1分钟平均负载
    pub load1: f64,
    /// 5分钟平均负载
    pub load5: f64,
    /// 15分钟平均负载
    pub load15: f64,
}
