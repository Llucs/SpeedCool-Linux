mod config;
mod core;
mod sensors;
mod gpu;
mod io;
mod network;
mod ipc;
mod daemon;
mod cli;
mod tui;
mod learning;
mod safe_restore;
mod auto_update;
mod gaming;
mod plugins;
mod desktop;
mod utils;

use clap::Parser;
use cli::args::{Cli, Commands};
use config::paths;

fn main() {
    let cli = Cli::parse();

    std::fs::create_dir_all("/var/log/speedcool").ok();
    std::fs::create_dir_all("/var/lib/speedcool").ok();
    std::fs::create_dir_all("/run/speedcool").ok();

    utils::logging::setup_logging("info", false);

    match &cli.command {
        Commands::Daemon => {
            let daemon = daemon::Daemon::new();
            match daemon.run() {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Daemon error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Set { profile } => {
            cli::commands::cmd_set(profile);
        }
        Commands::Status => {
            cli::commands::cmd_status();
        }
        Commands::Monitor => {
            match tui::run_tui() {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("TUI error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Profiles => {
            cli::commands::cmd_profiles();
        }
        Commands::CheckUpdate => {
            cli::commands::cmd_check_update();
        }
        Commands::Benchmark => {
            cli::commands::cmd_benchmark();
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            match shell.as_str() {
                "bash" => clap_complete::generate(clap_complete::Shell::Bash, &mut cmd, "speedcool", &mut std::io::stdout()),
                "zsh" => clap_complete::generate(clap_complete::Shell::Zsh, &mut cmd, "speedcool", &mut std::io::stdout()),
                "fish" => clap_complete::generate(clap_complete::Shell::Fish, &mut cmd, "speedcool", &mut std::io::stdout()),
                _ => eprintln!("Unsupported shell: {}. Use bash, zsh, or fish", shell),
            }
        }
    }
}
