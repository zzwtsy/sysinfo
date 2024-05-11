use std::time::Duration;

use dto::Host;
use sysinfo::{CpuRefreshKind, Disks, Networks, RefreshKind, System};

use crate::dto::State;

mod dto;

pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

fn main() {
    let disks = Disks::new_with_refreshed_list();
    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    let mut net_works = Networks::new_with_refreshed_list();
    loop {
        get_os_data(&sys, &disks, &net_works);
        std::thread::sleep(Duration::from_secs(2));
        net_works.refresh();
        sys.refresh_memory();
        sys.refresh_cpu_usage();
    }
}

fn get_os_data(sys: &System, disks: &Disks, net_works: &Networks) {
    let disk_total = disks
        .list()
        .iter()
        .map(|disk| disk.total_space())
        .sum::<u64>();
    let disk_used = disks
        .list()
        .iter()
        .map(|disk| disk.total_space() - disk.available_space())
        .sum::<u64>();
    let net_in_transfer = net_works
        .list()
        .iter()
        .map(|(_, net)| net.total_received())
        .sum::<u64>();
    let net_out_transfer = net_works
        .list()
        .iter()
        .map(|(_, net)| net.total_transmitted())
        .sum::<u64>();
    let net_in_speed = net_works
        .list()
        .iter()
        .map(|(_, net)| net.received())
        .sum::<u64>();
    let net_out_speed = net_works
        .list()
        .iter()
        .map(|(_, net)| net.transmitted())
        .sum::<u64>();
    let host = Host {
        platform: System::name().unwrap_or_default(),
        platform_version: System::os_version().unwrap_or_default(),
        cpu: sys
            .cpus()
            .iter()
            .map(|cpu| cpu.brand().to_string())
            .collect(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        hostname: System::long_os_version().unwrap_or_default(),
        mem_total: sys.total_memory(),
        disk_total,
        swap_total: sys.total_swap(),
        arch: System::cpu_arch().unwrap_or_default(),
        boot_time: System::boot_time(),
        country_code: "cn".to_string(),
        version: VERSION.to_string(),
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
        uptime: System::uptime(),
        load1: System::load_average().one,
        load5: System::load_average().five,
        load15: System::load_average().fifteen,
    };
    println!("{:}", serde_json::to_string_pretty(&host).unwrap());
    println!("{:}", serde_json::to_string_pretty(&state).unwrap());
}
