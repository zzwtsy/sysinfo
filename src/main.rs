use dto::Host;
use sysinfo::{Disks, System};

mod dto;

pub const VERSION: &'static str =
    include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

fn main() {
    let sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    let disk_total = disks
        .list()
        .iter()
        .map(|disk| disk.total_space())
        .reduce(|a, b| a + b);
    let host = Host {
        platform: System::name().unwrap(),
        platform_version: System::os_version().unwrap(),
        cpu: sys
            .cpus()
            .iter()
            .map(|cpu| cpu.brand().to_string())
            .collect(),
        mem_total: sys.total_memory(),
        disk_total: disk_total.unwrap(),
        swap_total: sys.total_swap(),
        arch: System::cpu_arch().unwrap(),
        boot_time: System::boot_time(),
        country_code: "cn".to_string(),
        version: VERSION.to_string(),
    };
    println!("{:}", serde_json::to_string_pretty(&host).unwrap());
}
