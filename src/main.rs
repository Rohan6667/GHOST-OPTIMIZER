use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Duration;
use std::env;
use std::collections::HashSet;
use sysinfo::{System, Process, Pid};

#[derive(Serialize)]
struct TelemetryPayload {
    process_name: String,
    cpu_usage: f32,
    memory_bytes: u64,
    secret_key: String,
}

#[derive(Deserialize, Debug)]
struct BrainResponse {
    action: String,
    reason: String,
}

// Global Memory Base to keep track of frozen processes
static mut FROZEN_PROCESSES: Option<HashSet<u32>> = None;

// --- FUNCTION 1: SYSTEM CACHE FLUSH (RAM MITIGATION) ---
fn flush_system_cache() {
    println!("[⚡ RAM MITIGATION] Critical memory state detected! Flushing caches...");
    let _ = Command::new("sync").status();
    let _ = Command::new("sh")
        .args(&["-c", "echo 3 > /proc/sys/vm/drop_caches"])
        .status();
}

// --- FUNCTION 2: CRYO-SLEEP EXECUTION (FREEZE PROCESS) ---
fn freeze_process(pid: u32, name: &str) {
    unsafe {
        if FROZEN_PROCESSES.is_none() {
            FROZEN_PROCESSES = Some(HashSet::new());
        }
        if let Some(ref mut set) = FROZEN_PROCESSES {
            if !set.contains(&pid) {
                println!("[🥶 CRYO-SLEEP] AI Matrix ordered Freezing on Process: {} (PID: {})", name, pid);
                let _ = Command::new("kill").args(&["-STOP", &pid.to_string()]).status();
                set.insert(pid);
            }
        }
    }
}

// --- FUNCTION 3: THAW EXECUTION (RESUME PROCESS) ---
fn thaw_all_processes() {
    unsafe {
        if let Some(ref mut set) = FROZEN_PROCESSES {
            if !set.is_empty() {
                println!("[🔥 THAW] Load pattern stabilized. Resuming all frozen processes...");
                for pid in set.iter() {
                    let _ = Command::new("kill").args(&["-CONT", &pid.to_string()]).status();
                }
                set.clear();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=========================================================");
    println!("===    GHOST OPTIMIZER AUTONOMOUS REAL-TIME CLIENT     ===");
    println!("=========================================================");

    let client = reqwest::Client::new();
    let mut sys = System::new_all();
    
    let secret_key = env::var("GHOST_SECRET_KEY").unwrap_or_else(|_| "SUPER_SECRET_GHOST_KEY_62026".to_string());
    let brain_url = env::var("HF_SPACE_URL").unwrap_or_else(|_| "https://sudarshan143-ghost.hf.space/analyze".to_string());

    loop {
        // Modern sysinfo automatic refresh syntax
        sys.refresh_all();
        
        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();
        let mem_percentage = (used_mem as f32 / total_mem as f32) * 100.0;

        println!("\n[📊 CORE METRICS] Total RAM: {} KB | Used RAM: {} KB ({:.2}%)", total_mem, used_mem, mem_percentage);

        if mem_percentage > 95.0 {
            flush_system_cache();
        }

        // Process Scan Engine
        let mut highest_cpu: f32 = 0.0;
        let mut target_pid: u32 = 0;
        let mut target_name = String::new();
        let mut target_mem: u64 = 0;

        for (pid, process) in sys.processes() {
            let cpu = process.cpu_usage();
            let p_name = process.name();
            
            if p_name == "ghost_optimizer" || p_name == "python3" || p_name == "bash" {
                continue;
            }

            if cpu > highest_cpu {
                highest_cpu = cpu;
                target_pid = pid.as_u32();
                target_name = p_name.to_string();
                target_mem = process.memory();
            }
        }

        if !target_name.is_empty() && highest_cpu > 10.0 {
            println!("[🎯 TARGET FOUND] Analyzing Process: {} (PID: {}) using {:.2}% CPU", target_name, target_pid, highest_cpu);

            let real_payload = TelemetryPayload {
                process_name: target_name.clone(),
                cpu_usage: highest_cpu,
                memory_bytes: target_mem * 1024,
                secret_key: secret_key.clone(),
            };

            match client.post(&brain_url).json(&real_payload).send().await {
                Ok(res) => {
                    if let Ok(brain_response) = res.json::<BrainResponse>().await {
                        println!("[🧠 BRAIN DECISION] Action: {}, Reason: {}", brain_response.action, brain_response.reason);
                        
                        if brain_response.action == "FREEZE" {
                            freeze_process(target_pid, &target_name);
                        } else {
                            thaw_all_processes();
                        }
                    }
                }
                Err(err) => println!("[🔴 ERROR] Cloud Brain connection failure: {:?}", err),
            }
        } else {
            println!("[🟢 STABLE] System load within limits. No anomalies detected.");
            thaw_all_processes(); 
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
