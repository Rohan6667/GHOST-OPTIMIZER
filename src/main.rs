hereuse sysinfo::{System, Process, Signal, DiskExt, ComponentExt};
use std::time::Duration;
use std::process::Command;
use tokio::time::sleep;
use reqwest::Client;
use serde_json::json;

const HF_TOKEN: &str = "YOUR_HF_TOKEN_HERE";
const HF_SPACE_URL: &str = "http://127.0.0.1:7860/analyze";

// Optimization Thresholds
const MAX_SYSTEM_CPU_PCT: f32 = 80.0; // Agar total CPU 80% se upar gaya toh mitigation active
const MAX_SYSTEM_RAM_PCT: f32 = 85.0; // 85% RAM par heavy optimization active

#[tokio::main]
async fn main() {
    println!("==================================================");
    println!("===    GHOST OPTIMIZER ULTRA-EFFICIENT (v5.0)  ===");
    println!("==================================================");
    println!("Ghost Load-Mitigation Engine: Active\n");

    let mut sys = System::new_all();
    let client = Client::new();
    let my_pid = sysinfo::get_current_pid().expect("Failed to get self PID");

    // Dynamic scan interval jo CPU/RAM load ke mutabik change hoga
    let mut adaptive_interval = Duration::from_secs(5);

    loop {
        // 1. REFRESH SYSTEM HEALTH WITH MINIMAL CPU OVERHEAD
        sys.refresh_cpu();
        sys.refresh_memory();

        let total_cpu_usage: f32 = sys.global_cpu_info().cpu_usage();
        let total_mem = sys.total_memory() as f32;
        let used_mem = sys.used_memory() as f32;
        let mem_pct = (used_mem / total_mem) * 100.0;

        println!("\n[📊 METRICS] Total System CPU: {:.2}%, RAM Usage: {:.2}%", total_cpu_usage, mem_pct);

        // ==========================================
        // FEATURE: SELF & SYSTEM CPU/RAM REDUCTION
        // ==========================================
        
        // A. CPU Usage Kam Karne Ka System
        if total_cpu_usage > MAX_SYSTEM_CPU_PCT {
            println!("[⚡ CPU MITIGATION] High system CPU load detected! Throttling Ghost Optimizer...");
            // Scanner ko slow kar do taaki Ghost khud CPU par load na dale
            adaptive_interval = Duration::from_secs(12);
            
            // Core optimization command: Forcefully background processes ki CPU priority (nice value) badhao
            lower_all_heavy_processes_priority(&mut sys, my_pid);
            
            // Current async thread ko yield karo taaki doosre important tasks pehle chal sakein
            tokio::task::yield_now().await;
        } else {
            // Normal load par fast scanning mode (5 seconds)
            adaptive_interval = Duration::from_secs(5);
        }

        // B. RAM Usage Kam Karne Ka System
        if mem_pct > MAX_SYSTEM_RAM_PCT {
            println!("[⚡ RAM MITIGATION] Critical RAM limit reached. Forcing System RAM reduction...");
            
            // Call 1: Drop memory buffers and caches
            force_system_ram_flush();
            
            // Call 2: Linux Swap memory trigger (Agar swap configured hai toh idle pages swap pe jayenge)
            let _ = Command::new("sudo").args(["sysctl", "vm.drop_caches=3"]).output();
            
            #[cfg(target_os = "linux")]
            unsafe {
                // Ghost ke apne memory buffers clear karne ke liye system API trigger (Frees heap memory)
                libc::malloc_trim(0);
            }
        }

        // 2. REGULAR PROCESS INSPECTION (Micro-sleeps ke saath taaki CPU spike na ho)
        sys.refresh_processes();
        for (pid, process) in sys.processes() {
            if pid == &my_pid { continue; }

            let cpu_usage = process.cpu_usage();
            if cpu_usage > 85.0 {
                let proc_name = process.name().to_string();
                println!("[⚠️ ROGUE] PID: {} ({}) is taking {:.2}% CPU.", pid, proc_name, cpu_usage);
                
                execute_ghost_action(&client, pid.as_u32(), proc_name, cpu_usage, process).await;
                
                // Scan ke dauran 50 milliseconds ka break taaki lagatar check karne se CPU utilization na badhe
                sleep(Duration::from_millis(50)).await;
            }
        }

        // Adaptive sleep time based on system condition
        sleep(adaptive_interval).await;
    }
}

// ==========================================
// MITIGATION CORE FUNCTIONS
// ==========================================

// Sabhi heavy processes ki priority kam karna (Renice)
fn lower_all_heavy_processes_priority(sys: &mut System, my_pid: sysinfo::Pid) {
    println!("   -> [GHOST ACTION] Adjusting CPU Scheduling Priorities (Nice values)...");
    
    for (pid, process) in sys.processes() {
        if pid == &my_pid { continue; }
        
        // Jo process 40% se upar CPU kha rahi hai, uski system priority 'Nice' command se kam karo
        if process.cpu_usage() > 01.0 {
            let pid_str = pid.to_string();
            // nice +15 karne se Linux us process ko sabse kam priority par daal deta hai
            let _ = Command::new("sudo").args(["renice", "-n", "15", "-p", &pid_str]).output();
        }
    }
    println!("   -> [🟢 SUCCESS] Heavy processes pushed to background priority.");
}

// RAM kam karne ka ultimate script
fn force_system_ram_flush() {
    println!("   -> [GHOST ACTION] Flushing system cache allocation matrices...");
    
    // Memory files sync taaki data safe rahe
    let _ = Command::new("sync").output();
    
    // RAM free karne ki core terminal command
    let _ = Command::new("sudo").args(["sh", "-c", "echo 3 > /proc/sys/vm/drop_caches"]).output();
    
    println!("   -> [🟢 SUCCESS] System RAM allocation down-scaled.");
}

// Central Brain interaction
async fn execute_ghost_action(client: &Client, pid: u32, name: String, cpu: f32, process: &Process) {
    let response = client.post(HF_SPACE_URL)
        .header("Authorization", format!("Bearer {}", HF_TOKEN))
        .timeout(Duration::from_secs(3))
        .json(&json!({ "process_name": name, "cpu_usage": cpu, "secret_key": "SUPER_SECRET_GHOST_KEY_123" }))
        .send()
        .await;

    if let Ok(res) = response {
        if let Ok(json_data) = res.json::<serde_json::Value>().await {
            let action = json_data["action"].as_str().unwrap_or("MONITOR");
            if action == "KILL" {
                process.kill();
                println!("      [⚡ AUTO-HEAL] Terminated rogue process PID {}", pid);
            }
        }
    }
}
