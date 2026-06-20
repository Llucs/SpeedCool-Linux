# ⚡ SpeedCool Linux

**SpeedCool Linux** — Intelligent system optimizer for Linux desktop and laptop.  
The successor of the legendary SpeedCool Magisk Module for Android, now on Linux.

![Build](https://github.com/Llucs/SpeedCool-Linux/actions/workflows/build.yml/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/language-Rust-orange.svg)

---

## Features

### Profiles
- **Eco** 🛋️ — Power saving, reduced frequency, turbo off
- **Balanced** ⚖️ — Dynamic adjustment based on system load
- **Performance** ⚡ — Maximum performance, turbo enabled

### Real-Time Monitor (TUI)
Interactive terminal dashboard with live graphs and system metrics.

### Sensors
- CPU: frequency, governor, temperature, load, turbo status
- GPU: NVIDIA (NVML), AMD (amdgpu), Intel (i915)
- Memory, Disk, Network, Battery, Thermal zones

### Gaming Mode
Auto-detects games (Steam, Lutris, Heroic, Proton) and applies Performance profile. Integrates with Feral GameMode.

### Learning Engine
Adaptive engine that learns your usage patterns over 7 days and schedules optimized profiles.

### SafeRestore
Anti-bootloop protection. Detects repeated rapid reboots and automatically restores safe Eco profile.

### Auto-Update
Checks GitHub for new releases and updates automatically.

### Daemon
Background service with systemd integration. IPC socket for CLI communication.

---

## Quick Install

```bash
sudo curl -fsSL https://raw.githubusercontent.com/Llucs/SpeedCool-Linux/main/scripts/install.sh | sh
```

Or manually:

```bash
git clone https://github.com/Llucs/SpeedCool-Linux.git
cd SpeedCool-Linux
cargo build --release
sudo cp target/release/speedcool /usr/local/bin/
```

---

## Usage

```bash
# Start daemon (background service)
sudo speedcool daemon

# Open interactive TUI monitor
speedcool monitor

# Set performance profile
sudo speedcool set performance

# View system status
speedcool status

# List available profiles
speedcool profiles

# Check for updates
speedcool check-update

# Run benchmark
speedcool benchmark

# Generate shell completions
speedcool completions bash
```

### Available Profiles

| Command | Profile | Description |
|---|---|---|
| `speedcool set eco` | 🛋️ Eco | Power saving, low frequency |
| `speedcool set balanced` | ⚖️ Balanced | Dynamic auto-tuning |
| `speedcool set performance` | ⚡ Performance | Maximum power |

---

## Installation Options

| Distribution | Package | Command |
|---|---|---|
| **Arch Linux** | AUR | `yay -S speedcool` |
| **Ubuntu/Debian** | .deb | `apt install speedcool` |
| **Fedora** | COPR | `dnf install speedcool` |
| **NixOS** | Flake | `nix profile install github:Llucs/SpeedCool-Linux` |
| **Universal** | Script | `curl -fsSL https://speedcool.dev/install.sh | sh` |

---

## Configuration

Config file: `/etc/speedcool/config.toml`

```toml
[daemon]
poll_interval_ms = 2000
log_level = "info"
auto_update = true

[thresholds]
cpu_temp_critical = 85.0
cpu_temp_warn = 70.0
gpu_temp_critical = 85.0
battery_pct_low = 20.0
cpu_load_high = 80.0
cpu_load_low = 20.0
```

---

## Architecture

```
┌─────────────────────┐     ┌─────────────────────┐
│   speedcool CLI     │     │   speedcool TUI     │
│   (clap, commands)  │     │   (ratatui, harts)  │
└─────────┬───────────┘     └─────────┬───────────┘
          │ IPC (Unix socket)         │
          ▼                           ▼
┌───────────────────────────────────────────┐
│          speedcool Daemon                 │
│  ┌────────┐ ┌────────┐ ┌──────────────┐   │
│  │Sensor  │ │Profile │ │Learning      │   │
│  │Engine  │ │Engine  │ │Engine        │   │
│  └───┬────┘ └───┬────┘ └──────┬───────┘   │
│      │          │              │           │
│  ┌───┴────┐ ┌───┴────┐ ┌──────┴───────┐   │
│  │Safe    │ │GPU     │ │Auto-Update   │   │
│  │Restore │ │Tuner   │ │              │   │
│  └────────┘ └────────┘ └──────────────┘   │
└───────────────────────────────────────────┘
```

---

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with specific features
cargo run -- monitor
cargo run -- status
cargo run -- set balanced

# Release build
cargo build --release
```

### Project Structure

```
speedcool-linux/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli/             # CLI interface
│   ├── tui/             # Terminal UI
│   ├── daemon/          # Background service
│   ├── core/            # Profile engine
│   ├── sensors/         # Hardware sensors
│   ├── gpu/             # GPU tuning
│   ├── learning/        # Learning engine
│   ├── safe_restore/    # Anti-bootloop
│   ├── auto_update/     # Self-update
│   ├── gaming/          # Gaming mode
│   └── config/          # Configuration
├── config/              # Default configs
├── scripts/             # Install scripts
└── .github/             # CI workflows
```

---

## Requirements

- Linux kernel 5.10+
- Root access (for governor/frequency changes)
- systemd (optional, for daemon mode)
- Rust 1.70+ (for building from source)

### Optional Dependencies

- `lm-sensors` — Better temperature readings
- `nvidia-smi` — NVIDIA GPU monitoring
- `gamemode` — Feral GameMode integration
- `xdotool` — Foreground app detection (X11)

---

## Credits

Created by **Llucs** — Author of the original SpeedCool Magisk Module for Android.

- GitHub: [@Llucs](https://github.com/Llucs)
- Original SpeedCool: [SpeedCool-Magisk-Module](https://github.com/Llucs/SpeedCool-Magisk-Module)

---

## License

MIT License — see [LICENSE](LICENSE)
