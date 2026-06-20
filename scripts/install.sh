#!/bin/bash
set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${CYAN}"
echo "╔══════════════════════════════════════╗"
echo "║     SpeedCool Linux Installer        ║"
echo "║  Intelligent System Optimizer v1.0   ║"
echo "╚══════════════════════════════════════╝"
echo -e "${NC}"

if [ "$(id -u)" != "0" ]; then
    echo -e "${RED}This installer must be run as root. Use sudo.${NC}"
    exit 1
fi

ARCH=$(uname -m)
case "$ARCH" in
    x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
    aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
    armv7l|armhf) TARGET="armv7-unknown-linux-gnueabihf" ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

echo -e "${BLUE}Detected: $ARCH ($TARGET)${NC}"

BINARY_URL="https://github.com/Llucs/SpeedCool-Linux/releases/latest/download/speedcool-$TARGET.tar.gz"
TEMP_DIR=$(mktemp -d)

echo -e "${YELLOW}Downloading SpeedCool...${NC}"
if ! curl -fsSL "$BINARY_URL" -o "$TEMP_DIR/speedcool.tar.gz" 2>/dev/null; then
    echo -e "${YELLOW}No prebuilt binary found for $TARGET. Building from source...${NC}"
    
    if ! command -v cargo &>/dev/null; then
        echo -e "${RED}Rust/Cargo not found. Install Rust first: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
        exit 1
    fi
    
    if [ ! -d "/tmp/speedcool-src" ]; then
        git clone https://github.com/Llucs/SpeedCool-Linux.git /tmp/speedcool-src
    fi
    
    cd /tmp/speedcool-src
    cargo build --release
    mkdir -p "$TEMP_DIR/speedcool"
    cp target/release/speedcool "$TEMP_DIR/speedcool/"
    cp config/speedcool.service "$TEMP_DIR/speedcool/"
    cp config/default.toml "$TEMP_DIR/speedcool/"
    cd /
fi

echo -e "${YELLOW}Installing...${NC}"
mkdir -p /opt/speedcool

if [ -f "$TEMP_DIR/speedcool.tar.gz" ]; then
    tar -xzf "$TEMP_DIR/speedcool.tar.gz" -C /opt/speedcool/
else
    cp "$TEMP_DIR/speedcool/speedcool" /opt/speedcool/
    cp "$TEMP_DIR/speedcool/speedcool.service" /opt/speedcool/
    cp "$TEMP_DIR/speedcool/default.toml" /opt/speedcool/
fi

ln -sf /opt/speedcool/speedcool /usr/local/bin/speedcool
chmod +x /usr/local/bin/speedcool

mkdir -p /etc/speedcool
mkdir -p /var/lib/speedcool
mkdir -p /var/log/speedcool
mkdir -p /run/speedcool

if [ ! -f /etc/speedcool/config.toml ]; then
    if [ -f /opt/speedcool/default.toml ]; then
        cp /opt/speedcool/default.toml /etc/speedcool/config.toml
    else
        cat > /etc/speedcool/config.toml << 'EOF'
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
EOF
    fi
    echo -e "${GREEN}Config created: /etc/speedcool/config.toml${NC}"
fi

if command -v systemctl &>/dev/null; then
    if [ -f /opt/speedcool/speedcool.service ]; then
        cp /opt/speedcool/speedcool.service /etc/systemd/system/speedcool.service
    else
        cat > /etc/systemd/system/speedcool.service << 'EOF'
[Unit]
Description=SpeedCool Linux — Performance Optimizer
After=multi-user.target

[Service]
Type=simple
ExecStart=/usr/local/bin/speedcool daemon
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF
    fi
    
    systemctl daemon-reload
    systemctl enable speedcool
    systemctl start speedcool
    echo -e "${GREEN}SpeedCool service started and enabled.${NC}"
fi

for shell in bash zsh fish; do
    if command -v "$shell" &>/dev/null; then
        /usr/local/bin/speedcool completions "$shell" > "/usr/share/bash-completion/completions/speedcool" 2>/dev/null || true
    fi
done

rm -rf "$TEMP_DIR"

echo ""
echo -e "${GREEN}╔══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  SpeedCool Linux installed!         ║${NC}"
echo -e "${GREEN}╠══════════════════════════════════════╣${NC}"
echo -e "${GREEN}║  Commands:                          ║${NC}"
echo -e "${GREEN}║  speedcool daemon  - Start daemon   ║${NC}"
echo -e "${GREEN}║  speedcool monitor - TUI dashboard  ║${NC}"
echo -e "${GREEN}║  speedcool set     - Set profile    ║${NC}"
echo -e "${GREEN}║  speedcool status  - Show status    ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════╝${NC}"
echo ""
echo -e "${CYAN}Profile commands: speedcool set eco | balanced | performance${NC}"
echo -e "${CYAN}Monitoring: speedcool monitor${NC}"
echo ""
echo -e "${YELLOW}SpeedCool Linux — by Llucs${NC}"
