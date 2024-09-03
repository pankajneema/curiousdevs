use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use std::time::Duration;
use sysinfo::{System, SystemExt, DiskExt, NetworkExt, ProcessExt};
use tokio::time::interval;
use sysinfo::NetworksExt;

#[tokio::main]
async fn main() {
    let uri = "ws://127.0.0.1:8080/ws"; // WebSocket server URI

    // Connect to the WebSocket server
    let (mut ws_stream, _) = match connect_async(uri).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to connect: {:?}", e);
            return;
        }
    };
    

    let mut sys = System::new_all();
    sys.refresh_all();

    // Collect memory usage information
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let free_memory = sys.free_memory();
    let available_memory = sys.available_memory();

    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let free_swap = sys.free_swap();

    let memory_message = format!(
        "Memory Usage:: Total: {} MB , Used: {} MB , Free: {} MB , Shared: {} MB , Buff/Cache: {} MB , Available: {} MB \n\
         Swap Usage::  Total: {} MB , Used: {} MB , Free: {} MB",
        total_memory / 1024,
        used_memory / 1024,
        free_memory / 1024,
        0, // Shared memory is not available from sysinfo
        0, // Buff/Cache is not available from sysinfo
        available_memory / 1024,
        total_swap / 1024,
        used_swap / 1024,
        free_swap / 1024,
    );

    // Collect disk usage information
    let disk_usage: Vec<String> = sys.disks().iter().map(|disk| {
        let total_space = disk.total_space() as f64;
        let available_space = disk.available_space() as f64;
        let used_space = total_space - available_space;
        let used_percentage = (used_space / total_space) * 100.0;

        let total_gb = total_space / (1024.0 * 1024.0 * 1024.0);
        let used_gb = used_space / (1024.0 * 1024.0 * 1024.0);
        let available_gb = available_space / (1024.0 * 1024.0 * 1024.0);

        let mount_point = disk.mount_point().to_string_lossy();
        format!(
            "Filesystem: {}  Size: {:.1} GB Used: {:.1} GB Available: {:.1} GB Use%: {:.0}%",
            mount_point,
            total_gb,
            used_gb,
            available_gb,
            used_percentage
        )
    }).collect();

    // Collect network usage information
    let network_usage: Vec<String> = sys.networks().iter().map(|(name, network)| {
        let received = network.received();
        let transmitted = network.transmitted();
        format!(
            "Network {}: Received: {} bytes, Transmitted: {} bytes",
            name,
            received,
            transmitted
        )
    }).collect();

    // Combine all information into one message
    let combined_message = format!(
        "{}\n\nDisk Usage:\n{}\n\nNetwork Usage:\n{}",
        memory_message,
        disk_usage.join("\n"),
        network_usage.join("\n")
    );

    // Send a message to the server
    println!("Sending message: {}", combined_message);

    if let Err(e) = ws_stream.send(Message::Text(combined_message)).await {
        eprintln!("Failed to send message: {:?}", e);
        return;
    }

    // Optionally, listen for a response from the server
    match ws_stream.next().await {
        Some(Ok(Message::Text(response))) => {
            println!("Received response: {}", response);
        }
        Some(Err(e)) => {
            eprintln!("Failed to receive message: {:?}", e);
        }
        _ => {
            println!("Received non-text message or connection closed");
        }
    }
}

