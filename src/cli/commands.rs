use crate::core::engine::ProfileEngine;
use crate::core::profile::Profile;
use crate::sensors::{cpu::CpuSensor, memory::MemorySensor, battery::BatterySensor};
use crate::auto_update::AutoUpdate;
use crate::ipc::{protocol::IpcRequest, client::IpcClient};

pub fn cmd_set(profile_name: &str) {
    match Profile::from_str(profile_name) {
        Some(profile) => {
            let result = if let Ok(resp) = IpcClient::send(IpcRequest::SetProfile(profile_name.to_string())) {
                resp
            } else {
                ProfileEngine::apply(&profile).map(|_| {
                    IpcResponse { success: true, message: "".into(), data: None }
                }).unwrap_or_else(|e| IpcResponse { success: false, message: e, data: None })
            };
            if result.success {
                println!("{} Profile set to: {}", profile.icon(), profile.as_str());
            } else {
                eprintln!("Error: {}", result.message);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("Invalid profile: {}. Use: eco, balanced, or performance", profile_name);
            std::process::exit(1);
        }
    }
}

pub fn cmd_status() {
    let cpu = CpuSensor::read(0);
    let mem = MemorySensor::read();
    let bat = BatterySensor::read();
    let disks = crate::sensors::disk::DiskSensor::list();

    println!("\n╔══════════════════════════════════════╗");
    println!("║      SpeedCool Linux Status         ║");
    println!("╠══════════════════════════════════════╣");

    println!("║ CPU:                                   ");
    println!("║   Model: {}", cpu.model);
    println!("║   Cores: {}", cpu.cores);
    println!("║   Governor: {}", cpu.governor);
    println!("║   Frequency: {} MHz", cpu.frequencies.first().unwrap_or(&0) / 1000);
    println!("║   Temperature: {:.1}°C", cpu.temp);
    println!("║   Usage: {:.1}%", cpu.usage);
    println!("║   Turbo: {}", if cpu.turbo_enabled { "Enabled" } else { "Disabled" });

    println!("║ Memory:                                 ");
    println!("║   Total: {} MB", mem.total_kb / 1024);
    println!("║   Available: {} MB", mem.available_kb / 1024);
    println!("║   Swap: {} MB / {} MB", (mem.swap_total_kb - mem.swap_free_kb) / 1024, mem.swap_total_kb / 1024);

    if bat.present {
        println!("║ Battery:                                ");
        println!("║   Capacity: {:.0}%", bat.capacity);
        println!("║   Status: {}", bat.status);
    }

    println!("║ Disks:                                 ");
    for disk in &disks {
        println!("║   {} - {} ({} scheduler)", disk.name, disk.model, disk.scheduler);
    }

    println!("╚══════════════════════════════════════╝");
}

pub fn cmd_profiles() {
    println!("\nAvailable Profiles:\n");
    for p in &[Profile::Eco, Profile::Balanced, Profile::Performance] {
        println!("  {}  {} - {}", p.icon(), p.as_str(), match p {
            Profile::Eco => "Power saving, reduced frequencies",
            Profile::Balanced => "Dynamic adjustment based on load",
            Profile::Performance => "Maximum performance, turbo enabled",
        });
        println!("       Governor: {}, Turbo: {}, EPP: {}",
            p.governor(), if p.turbo() { "on" } else { "off" }, p.epp());
        println!();
    }
}

pub fn cmd_check_update() {
    match AutoUpdate::check_for_updates("1.0.0") {
        Ok(Some(version)) => {
            println!("New version available: {}", version);
            print!("Update now? [y/N]: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            if input.trim().eq_ignore_ascii_case("y") {
                match AutoUpdate::do_update(&version) {
                    Ok(()) => println!("Update successful!"),
                    Err(e) => eprintln!("Update failed: {}", e),
                }
            }
        }
        Ok(None) => println!("You are running the latest version."),
        Err(e) => eprintln!("Failed to check updates: {}", e),
    }
}

pub fn cmd_benchmark() {
    println!("Running SpeedCool benchmark...");
    let start = std::time::Instant::now();
    let cpu = CpuSensor::read(0);
    let mem = MemorySensor::read();

    let mut result = 0u64;
    for i in 0..1_000_000 {
        result = result.wrapping_add(i);
        result ^= result >> 12;
        result ^= result << 25;
        result ^= result >> 27;
    }

    let elapsed = start.elapsed();
    println!("\nBenchmark Results:");
    println!("  CPU: {} ({})", cpu.model, cpu.governor);
    println!("  Memory: {} MB total", mem.total_kb / 1024);
    println!("  Compute time: {:?}", elapsed);
    println!("  Score: {}", result);
}

use crate::ipc::protocol::IpcResponse;
