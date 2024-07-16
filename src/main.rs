use std::{
    collections::HashSet,
    ops::Not,
    time::{SystemTime, UNIX_EPOCH},
};

use fetch_ip::fetch_geo_ip;
use sysinfo::{CpuRefreshKind, Disks, Networks, RefreshKind, System};

use dto::{server_info::SystemInfo, Host, State};

mod dto;
mod fetch_ip;
mod utils;

pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

#[tokio::main]
async fn main() {
    let mut disks = Disks::new();
    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    let mut networks = Networks::new_with_refreshed_list();
    // loop {
    disks.refresh_list();
    sys.refresh_memory();
    sys.refresh_cpu();
    networks.refresh_list();
    get_os_info(&sys, &disks, &networks).await;
    // std::thread::sleep(Duration::from_secs(1));
    // }
}

async fn get_os_info(sys: &System, disks: &Disks, networks: &Networks) {
    let disk_total = disks
        .list()
        .iter()
        .filter(|disk| disk.file_system().eq_ignore_ascii_case("overlay").not())
        .map(|disk| disk.total_space())
        .sum::<u64>();
    let disk_used = disks
        .list()
        .iter()
        .filter(|disk| disk.file_system().eq_ignore_ascii_case("overlay").not())
        .map(|disk| disk.total_space() - disk.available_space())
        .sum::<u64>();
    let net_in_transfer = networks
        .list()
        .iter()
        .map(|(_, net)| net.total_received())
        .sum::<u64>();
    let net_out_transfer = networks
        .list()
        .iter()
        .map(|(_, net)| net.total_transmitted())
        .sum::<u64>();
    let net_in_speed = networks
        .list()
        .iter()
        .map(|(_, net)| net.received())
        .sum::<u64>();
    let net_out_speed = networks
        .list()
        .iter()
        .map(|(_, net)| net.transmitted())
        .sum::<u64>();
    let cpu = sys
        .cpus()
        .iter()
        .map(|cpu| cpu.brand().to_string())
        .collect::<HashSet<String>>();
    let geo_ip = fetch_geo_ip().await;
    let host = Host {
        os_name: System::name().unwrap_or_default().trim().to_string(),
        long_os_version: System::long_os_version()
            .unwrap_or_default()
            .trim()
            .to_string(),
        os_version: System::os_version().unwrap_or_default(),
        cpu,
        cpu_cores: sys.cpus().len() as u64,
        kernel_version: System::kernel_version().unwrap_or_default(),
        mem_total: sys.total_memory(),
        disk_total,
        swap_total: sys.total_swap(),
        arch: System::cpu_arch().unwrap_or_default(),
        boot_time: System::boot_time(),
        ip_v4: geo_ip.ip_v4,
        ip_v6: geo_ip.ip_v6,
        country_code: geo_ip.country_code,
        agent_version: VERSION.to_string(),
    };
    let state = State {
        cpu: sys.global_cpu_info().cpu_usage(),
        mem_used: sys.used_memory(),
        swap_used: sys.used_swap(),
        disk_used,
        net_in_transfer,
        net_out_transfer,
        net_in_speed,
        net_out_speed,
        load1: System::load_average().one,
        load5: System::load_average().five,
        load15: System::load_average().fifteen,
    };
    let server_info = SystemInfo {
        host,
        state,
        upload_time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .to_string(),
    };
    println!("{:}", serde_json::to_string_pretty(&server_info).unwrap());
}
