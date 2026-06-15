use sysinfo::{ProcessExt, System, SystemExt};
use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    println!("=== GHOST OPTIMIZER (v0.1.0) ===");
    println!("Initializing zero-cost infrastructure optimizer...\n");

    let mut sys = System::new_all();
    let client = Client::new();

    loop {
        sys.refresh_all();
        println!("--------------------------------------------------");
        println!("Checking for CPU-heavy and unoptimized processes...");
        println!("--------------------------------------------------");

        let mut heavy_processes = false;

        for (pid, process) in sys.processes() {
            let cpu_usage = process.cpu_usage();
            
            if cpu_usage > 80.0 {
                heavy_processes = true;
                let proc_name = process.name().to_string_lossy().to_string(); 
                
                println!(
                    "[⚠️ ALERT] Process ID: {} | Name: {} | CPU Usage: {:.2}%",
                    pid, proc_name, cpu_usage
                );
                
                // Hugging Face Private Space se secure logic lekar aao
                optimize_process(&client, pid.to_string(), proc_name, cpu_usage).await;
            }
        }

        if !heavy_processes {
            println!("[🟢 HEALTHY] All enterprise infrastructure processes running under optimal limits.");
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn optimize_process(client: &Client, pid: String, name: String, cpu: f32) {
    println!("[💡 GHOST ACTION] Fetching secure brain analysis for '{}' (PID: {})...", name, pid);
    
    // NOTE: TERA_HF_USERNAME aur TERA_SPACE_NAME ko apne Hugging Face account ke hisab se badal lena
    let url = "https://TERA_HF_USERNAME-TERA_SPACE_NAME.hf.space/analyze";
    
    let response = client.post(url)
        .json(&json!({
            "process_name": name,
            "cpu_usage": cpu,
            "secret_key": "SUPER_SECRET_GHOST_KEY_123"
        }))
        .send()
        .await;

    if let Ok(res) = response {
        if let Ok(json_data) = res.json::<serde_json::Value>().await {
            println!("   -> {}", json_data["recommendation"].as_str().unwrap_or("Optimal setup"));
        }
    } else {
        println!("   -> [❌ ERROR] Could not connect to Ghost Private Brain Server.");
    }
}
