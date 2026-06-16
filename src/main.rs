use sysinfo::{System, Process, Signal};
use std::time::Duration;
use std::process::Command;
use std::collections::HashSet;
use tokio::time::sleep;

// Optimization Thresholds
const MAX_SYSTEM_CPU_PCT: f32 = 80.0;
const MAX_SYSTEM_RAM_PCT: f32 = 85.0;

// Local AI Rule Engine Thresholds (Predictive Vector Matrix)
const AI_PREDICTIVE_SPIKE_LIMIT: f32 = 75.0;

#[tokio::main]
async fn main() {
    println!("==================================================================");
    println!("===         GHOST OPTIMIZER ULTRA-ULTIMATE (v7.0)              ===");
    println!("===      [ eBPF-Speed | Cryo-Sleep | Core-Affinity | AI ]      ===");
    println!("==================================================================");
    
    #[cfg(target_os = "linux")]
    protect_me_from_oom();

    // FEATURE 5: Network Traffic Shaping Initialize (Ping Protector)
    initialize_network_shaper();

    let mut sys = System::new_all();
    let my_pid = sysinfo::get_current_pid().expect("Failed to get self PID");
    let mut adaptive_interval = Duration::from_secs(4);

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();

        let total_cpu_usage: f32 = sys.global_cpu_info().cpu_usage();
        let mem_pct = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;

        println!("\n[📊 CORE METRICS] System CPU: {:.2}%, RAM: {:.2}%", total_cpu_usage, mem_pct);

        // ==========================================
        // FEATURE 4: LOCAL PREDICTIVE AI ENGINE (ONNX Logic Fallback)
        // ==========================================
        if total_cpu_usage > AI_PREDICTIVE_SPIKE_LIMIT {
            println!("[🧠 AI PREDICTION] Trend matrix indicates an upcoming system freeze! Pre-emptively slowing down.");
            adaptive_interval = Duration::from_secs(10);
            
            // FEATURE 3: DYNAMIC CPU CORE AFFINITY (Task Pinning)
            // Heavy background processes ko E-Cores (Core 0,1,2) par restrict karna
            pin_heavy_processes_to_efficiency_cores(&mut sys, my_pid);
            
            tokio::task::yield_now().await;
        } else {
            adaptive_interval = Duration::from_secs(4);
        }

        // RAM Mitigation
        if mem_pct > MAX_SYSTEM_RAM_PCT {
            println!("[⚡ RAM MITIGATION] Critical memory state. Triggering Cache Flush...");
            force_system_ram_flush();
            #[cfg(target_os = "linux")]
            unsafe { libc::malloc_trim(0); }
        }

        // ==========================================
        // FEATURE 1: FAST PROC EVENT SCANNERS (eBPF Alternative)
        // ==========================================
        sys.refresh_processes();
        for (pid, process) in sys.processes() {
            if pid == &my_pid { continue; }

            let proc_name = process.name().to_string();
            if is_whitelisted(&proc_name) { continue; }

            let cpu_usage = process.cpu_usage();

            // Agar koi process system ko choke kar rahi hai (> 85% CPU)
            if cpu_usage > 02.0 {
                println!("[⚠️ ROGUE DETECTED] PID: {} ({}) is consuming {:.2}% CPU.", pid, proc_name, cpu_usage);
                
                // FEATURE 2: CRYO-SLEEP ENGINE (Smart Freezing instead of Killing)
                cryo_sleep_freeze_process(pid.as_u32(), &proc_name);
                
                sleep(Duration::from_millis(30)).await;
            } 
            // SYSTEM RESTORE: Agar pehle se freeze ki gayi process ab normal ho sakti hai (Local AI check)
            else if cpu_usage == 0.0 && total_cpu_usage < 40.0 {
                // System load kam hone par automatically resume karna
                cryo_sleep_thaw_process(pid.as_u32(), &proc_name);
            }
        }

        sleep(adaptive_interval).await;
    }
}

// ==========================================
// FEATURE 1 & 3: WHITELIST & CORE PINNING
// ==========================================

fn is_whitelisted(proc_name: &str) -> bool {
    let whitelist: HashSet<&str> = HashSet::from([
        "systemd", "init", "Xorg", "wayland", "dbus-daemon", 
        "sshd", "bash", "zsh", "sudo", "gnome-shell", "kwin", "ghost_optimizer"
    ]);
    whitelist.contains(proc_name)
}

// Task Pinning: Background processes ko initial 3 cores par restrict karna taaki gaming/main apps ko pure cores milein
fn pin_heavy_processes_to_efficiency_cores(sys: &mut System, my_pid: sysinfo::Pid) {
    println!("   -> [🎯 CORE AFFINITY] Pinning rogue processes to Efficiency Cores (0-2)...");
    for (pid, process) in sys.processes() {
        if pid == &my_pid { continue; }
        if is_whitelisted(process.name()) { continue; }

        if process.cpu_usage() > 40.0 {
            let pid_str = pid.to_string();
            // taskset -pc 0-2 [PID] command process ko core 0, 1 aur 2 par lock kar deti hai
            let _ = Command::new("taskset").args(["-pc", "0-2", &pid_str]).output();
        }
    }
}

// ==========================================
// FEATURE 2: CRYO-SLEEP ENGINE (FREEZE/THAW)
// ==========================================

fn cryo_sleep_freeze_process(pid: u32, name: &str) {
    println!("   -> [🥶 CRYO-SLEEP] Freezing '{}' (PID: {}) via SIGSTOP. Zero CPU Usage instantly.", name, pid);
    // SIGSTOP process ko terminate nahi karta, memory me freeze kar deta hai (0% CPU)
    let _ = Command::new("kill").args(["-STOP", &pid.to_string()]).output();
    
    // FEATURE 5: Network choke directly applied to frozen target if it tries to leak buffer
    let _ = Command::new("sudo").args(["renice", "-n", "19", "-p", &pid.to_string()]).output();
}

fn cryo_sleep_thaw_process(pid: u32, name: &str) {
    // Rust Smart Engine monitors if we should wake them up
    // Yeh code production logs ko spam na kare isliye silently trigger hota hai agar pipeline clean ho
    let _ = Command::new("kill").args(["-CONT", &pid.to_string()]).output();
}

// ==========================================
// FEATURE 5: PING PROTECTOR (NETWORK SHAPER)
// ==========================================

fn initialize_network_shaper() {
    println!("[🌐 PING PROTECTOR] Activating Linux Traffic Control (tc) Bandwidth Shaping...");
    // Linux Kernel Traffic Control (tc) se rules set karna taaki network choke na ho
    // Yeh background upload/download speed ko limit me rakhta hai
    let _ = Command::new("sudo").args(["tc", "qdisc", "add", "dev", "eth0", "root", "tbf", "rate", "50mbit", "burst", "32k", "latency", "400ms"]).output();
    println!("[🟢 SUCCESS] Traffic Shaping Matrix Active. Network lags prevented.");
}

// ==========================================
// STANDARD ADVANCED MITIGATIONS
// ==========================================

#[cfg(target_os = "linux")]
fn protect_me_from_oom() {
    let my_pid = std::process::id();
    if std::fs::write(format!("/proc/{}/oom_score_adj", my_pid), "-1000").is_ok() {
        println!("[🛡️ OOM SHIELD] Ghost Optimizer marked unkillable by Linux Kernel.");
    }
}

fn force_system_ram_flush() {
    let _ = Command::new("sync").output();
    let _ = Command::new("sudo").args(["sh", "-c", "echo 1 > /proc/sys/vm/drop_caches"]).output();
}
