use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "speedcool", version, about = "⚡ SpeedCool Linux — Intelligent system optimizer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Start the SpeedCool daemon in background")]
    Daemon,

    #[command(about = "Set performance profile")]
    Set {
        #[arg(value_name = "PROFILE", help = "Profile: eco, balanced, performance")]
        profile: String,
    },

    #[command(about = "Get current profile and status")]
    Status,

    #[command(about = "Show real-time monitoring TUI")]
    Monitor,

    #[command(about = "Show available profiles")]
    Profiles,

    #[command(about = "Check for updates")]
    CheckUpdate,

    #[command(about = "Run system benchmark")]
    Benchmark,

    #[command(about = "Generate shell completions")]
    Completions {
        #[arg(value_name = "SHELL", help = "Shell: bash, zsh, fish")]
        shell: String,
    },
}
