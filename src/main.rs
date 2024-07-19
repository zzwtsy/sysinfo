use std::{collections::HashMap, env};

use proto::ServerMonitorClient;

mod dto;
mod fetch_ip;
mod proto;
mod utils;

#[tokio::main]
async fn main() {
    let args = parse_args();
    let mut server_monitor_client = ServerMonitorClient::new(
        args.get("--url").expect("Parse url failed").to_string(),
        args.get("--port").expect("Parse port failed").to_string(),
        args.get("--id")
            .expect("Parse id failed")
            .parse()
            .expect("Parse id failed"),
    )
    .await;
    server_monitor_client.report_server_monitor().await;
}

fn parse_args() -> HashMap<String, String> {
    let mut args_map = HashMap::<String, String>::new();
    for arg in env::args() {
        let arg_trim = arg.trim();
        if arg_trim.starts_with("--url=") {
            let split = arg_trim.split_once("=");
            if let Some((key, value)) = split {
                args_map.insert(key.to_string(), value.to_string());
            }
        }
        if arg_trim.starts_with("--port=") {
            let split = arg_trim.split_once("=");
            if let Some((key, value)) = split {
                args_map.insert(key.to_string(), value.to_string());
            }
        }
        if arg_trim.starts_with("--id=") {
            let split = arg_trim.split_once("=");
            if let Some((key, value)) = split {
                args_map.insert(key.to_string(), value.to_string());
            }
        }
    }

    if args_map.get("--url").is_none() {
        panic!("Missing args: --url")
    }
    if args_map.get("--port").is_none() {
        panic!("Missing args: --port")
    }
    if args_map.get("--id").is_none() {
        panic!("Missing args: --id")
    }

    return args_map;
}
