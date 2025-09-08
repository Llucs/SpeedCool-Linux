# SpeedCool Linux

⚡ **SpeedCool Linux** is a lightweight system performance and power management tool for Linux designed to optimize CPU, GPU, RAM, and disk usage dynamically. It supports multiple operation modes, hardware monitoring, and automatic system tuning to improve performance, efficiency, and cooling.

***

## Features

- Three performance profiles:  
  - **Eco Mode**: Balanced power saving with reduced CPU frequencies and turbo disabled  
  - **Auto Mode**: Dynamic adjustment based on system load  
  - **Performance Mode**: Maximum performance with turbo enabled and minimal swap usage  
- Real-time hardware monitoring: CPU load, temperature, RAM, GPU info, disk IO, and network usage  
- Automatic temperature and battery monitoring with corrective actions  
- Integration with GameMode or Feral Gamemode for gaming optimization  
- Kernel tuning including I/O scheduler and TCP buffers optimization  
- Disk SSD/HDD optimizations including TRIM support  
- Interactive terminal menu with easy mode switching and status display  
- Systemd service support for background daemon operation  
- Automatic dependency installation and sensor configuration  

***

## Installation

Run the installer script with root permissions:

```bash
sudo ./speedcool.sh
```

Or install manually via:

```bash
sudo speedcool --install
```

The installer will copy files, set permissions, enable the service, and install dependencies.

***

## Usage

### Start the daemon (background monitoring and optimization)

```bash
sudo speedcool daemon
```

### Open the interactive menu

```bash
sudo speedcool menu
```

Use the menu keys or numeric options to switch profiles, view system status, or run maintenance tasks.

### Automatic monitoring mode

```bash
sudo speedcool --auto
```

This mode automatically adjusts profiles based on system usage.

***

## Profiles

- **Eco**: Low power, reduced frequencies, turbo off  
- **Work**: Balanced frequencies, turbo off  
- **Gaming**: High frequencies, turbo on, optimized for performance  

Profiles adjust CPU frequency scaling, GPU settings, disk parameters, and kernel network buffers.

***

## Configuration

The program loads configuration from:

```
/opt/speedcool/config.json
```

Example config keys include temperature thresholds, battery levels, CPU load limits, and notification delay.

***

## Requirements

- Linux distribution with systemd  
- Root privileges for installation and operation  
- Dependencies: `sysstat`, `procps`, `pciutils`, `coreutils`, `util-linux`, `lm-sensors`, `linux-tools-common`, `bc`, `jq`  
- Optional for NVIDIA GPUs: `nvidia-driver`, `nvidia-utils`  
- Recommended: `gamemode` or `feral-gamemode` for gaming enhancements  

***

## Notes

- BIOS/UEFI configuration (XMP profiles, hyperthreading, virtualization) must be adjusted manually in the system BIOS.  
- Compositor and VSync adjustments for gaming require manual configuration depending on the desktop environment.  
- Kernel IRQ affinity tuning is system-specific and disabled by default.  
- The program cleans caches and runs TRIM periodically for SSD health.  

***

## Author

Llucs (SpeedCool Dev)  
Project start date: 08/09/25

***

## License

This project is released under the MIT License.

