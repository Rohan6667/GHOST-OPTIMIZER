use sysinfo::{System, Process, Signal};
use std::time::Duration;
use std::process::Command;
use std::collections::HashSet;
use tokio::time::sleep;
use serde::{Deserialize, Serialize};

const MAX_SYSTEM_CPU_PCT: f32 = 80.0;
const MAX_SYSTEM_RAM_PCT: f32 = 85.0;
const AI_PREDICTIVE_SPIKE_LIMIT: f32 = 75.0;

#[derive(Serialize)]
struct AnalyzeRequest {
    process_name: String,
    cpu_usage: f32,
    memory_bytes: u64,
    secret_key: String,
}

#[derive(Deserialize)]
struct AnalyzeResponse {
    action: String,          
    reason: String,          
    target_cores: Option<String>, 
}

#[tokio::main]
async fn main() {
    println!("==================================================================");
    println!("===         GHOST OPTIMIZER ULTRA-ULTIMATE (v7.0)              ===");
    println!("===      [ Cloud-Tethered | Cryo-Sleep | Core-Pinning ]        ===");
    println!("==================================================================");
    
    // Cloud Engine URL (GitHub Secrets ya Environment se uthayega)
    let brain_url = std::env::var("HF_SPACE_URL")
        .unwrap_or_else(|_| "https://sudarshan143-ghost.hf.space/analyze".to_string());
    
    let secret_key = std::env::var("GHOST_SECRET_KEY")
        .unwrap_or_else(|_| "SUPER_SECRET_GHOST_KEY_123".to_string());

    #[cfg(target_os = "linux")]
    protect_me_from_oom();

    initialize_network_shaper();

    let mut sys = System::new_all();
    let my_pid = sysinfo::get_current_pid().expect("Failed to get self PID");
    let mut adaptive_interval = Duration::from_secs(4);
    
    // Frozen processes ko track karne ke liye tracking engine
    let mut frozen_pids: HashSet<u32> = HashSet::new();
    let http_client = reqwest::Client::new();

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();

        let total_cpu_usage: f32 = sys.global_cpu_info().cpu_usage();
        let mem_pct = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;

        println!("\n[📊 CORE METRICS] System CPU: {:.2}%, RAM: {:.2}%", total_cpu_usage, mem_pct);

        if total_cpu_usage > AI_PREDICTIVE_SPIKE_LIMIT {
            println!("[🧠 AI PREDICTION] High system load trend detected. Extending adaptive interval.");
            adaptive_interval = Duration::from_secs(6);
            pin_heavy_processes_to_efficiency_cores(&mut sys, my_pid);
            tokio::task::yield_now().await;
        } else {
            adaptive_interval = Duration::from_secs(4);
        }

        if mem_pct > MAX_SYSTEM_RAM_PCT {
            println!("[⚡ RAM MITIGATION] Critical memory state. Triggering Cache Flush...");
            force_system_ram_flush();
            #[cfg(target_os = "linux")]
            unsafe { libc::malloc_trim(0); }
        }

        sys.refresh_processes();
        
        // Snapshot vector to avoid ownership conflicts during iteration
        let process_list: Vec<(u32, String, f32, u64)> = sys.processes()
            .iter()
            .filter(|(&pid, _)| pid != my_pid)
            .map(|(&pid, proc)| (pid.as_u32(), proc.name().to_string(), proc.cpu_usage(), proc.memory()))
            .collect();

        for (pid_u32, proc_name, cpu_usage, mem_bytes) in process_list {
            if is_whitelisted(&proc_name) { continue; }

            // 1. Local Filter: Agar process harmless hai aur pehle se frozen nahi hai, toh skip karo
            if cpu_usage < 15.0 && !frozen_pids.contains(&pid_u32) {
                continue;
            }

            // 2. Cloud Brain Transaction Packet
            let payload = AnalyzeRequest {
                process_name: proc_name.clone(),
                cpu_usage,
                memory_bytes: mem_bytes,
                secret_key: secret_key.clone(),
            };

            // Non-blocking async API call to Hugging Face
            if let Ok(res) = http_client.post(&brain_url).json(&payload).send().await {
                if let Ok(ai_decision) = res.json::<AnalyzeResponse>().await {
                    match ai_decision.action.as_str() {
                        "FREEZE" => {
                            if !frozen_pids.contains(&pid_u32) {
                                println!("[🧠 CLOUD AI -> FREEZE] Target: {} | Reason: {}", proc_name, ai_decision.reason);
                                cryo_sleep_freeze_process(pid_u32, &proc_name);
                                frozen_pids.insert(pid_u32);
                            }
                        },
                        "PIN_CORE" => {
                            if let Some(cores) = ai_decision.target_cores {
                                println!("[🧠 CLOUD AI -> PIN] Core Affinity matrix applied to '{}' -> Cores {}", proc_name, cores);
                                let _ = Command::new("taskset").args(["-pc", &cores, &pid_u32.to_string()]).output();
                            }
                        },
                        "KILL" => {
                            println!("[💥 CLOUD AI -> TERMINATE] Threat Neutralized: {}", proc_name);
                            let _ = Command::new("kill").args(["-9", &pid_u32.to_string()]).output();
                            frozen_pids.remove(&pid_u32);
                        },
                        "MONITOR" => {
                            // System stable hone par frozen processes ko wapas Thaw (Writhe) karo
                            if frozen_pids.contains(&pid_u32) && total_cpu_usage < 45.0 {
                                println!("[🔥 CLOUD AI -> THAW] Thawing '{}' back into active memory heap.", proc_name);
                                cryo_sleep_thaw_process(pid_u32, &proc_name);
                                frozen_pids.remove(&pid_u32);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        sleep(adaptive_interval).await;
    }
}

fn is_whitelisted(proc_name: &str) -> bool {
    let whitelist: HashSet<&str> = HashSet::from([
        "systemd", "init", "Xorg", "wayland", "dbus-daemon", 
        "sshd", "bash", "zsh", "sudo", "gnome-shell", "kwin", "ghost_optimizer", "termux"
    ]);
    whitelist.contains(proc_name)
}

fn pin_heavy_processes_to_efficiency_cores(sys: &mut System, my_pid: sysinfo::Pid) {
    for (pid, process) in sys.processes() {
        if pid == &my_pid { continue; }
        if is_whitelisted(process.name()) { continue; }

        if process.cpu_usage() > 40.0 {
            let _ = Command::new("taskset").args(["-pc", "0-2", &pid.to_string()]).output();
        }
    }
}

fn cryo_sleep_freeze_process(pid: u32, name: &str) {
    let _ = Command::new("kill").args(["-STOP", &pid.to_string()]).output();
    let _ = Command::new("sudo").args(["renice", "-n", "19", "-p", &pid.to_string()]).output();
}

fn cryo_sleep_thaw_process(pid: u32, _name: &str) {
    let _ = Command::new("kill").args(["-CONT", &pid.to_string()]).output();
    let _ = Command::new("sudo").args(["renice", "-n", "0", "-p", &pid.to_string()]).output();
}

fn initialize_network_shaper() {
    // Sudo commands execution output discarded gracefully to handle non-rooted Termux environments
    let _ = Command::new("sudo").args(["tc", "qdisc", "add", "dev", "eth0", "root", "tbf", "rate", "50mbit", "burst", "32k", "latency", "400ms"]).output();
}

#[cfg(target_os = "linux")]
fn protect_me_from_oom() {
    let my_pid = std::process::id();
    let _ = std::fs::write(format!("/proc/{}/oom_score_adj", my_pid), "-1000");
}

fn force_system_ram_flush() {
    let _ = Command::new("sync").output();
    let _ = Command::new("sudo").args(["sh", "-c", "echo 1 > /proc/sys/vm/drop_caches"]).output();
}

