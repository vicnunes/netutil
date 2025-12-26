# NetUtil Installation Guide

This guide provides instructions for installing NetUtil on macOS, Linux, and Arch Linux systems.

## Table of Contents
- [Quick Install](#quick-install)
- [System Requirements](#system-requirements)
- [Dependencies](#dependencies)
- [Installation Methods](#installation-methods)
- [Post-Installation](#post-installation)
- [Troubleshooting](#troubleshooting)
- [Uninstallation](#uninstallation)

---

## Quick Install

### From Pre-compiled Binary

If you have a pre-compiled `netutil-tui` binary, simply copy it to a location in your PATH:

**macOS/Linux:**
```bash
# Option 1: User installation (recommended)
mkdir -p ~/.local/bin
cp netutil-tui ~/.local/bin/netutil
chmod +x ~/.local/bin/netutil

# Option 2: System-wide installation (requires sudo)
sudo cp netutil-tui /usr/local/bin/netutil
sudo chmod +x /usr/local/bin/netutil
```

### From Source

```bash
# 1. Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Build the project
cd /path/to/netutil
cargo build --release

# 3. Install the binary
sudo cp target/release/netutil-tui /usr/local/bin/netutil
sudo chmod +x /usr/local/bin/netutil
```

---

## System Requirements

### Minimum Requirements
- **Operating System**: macOS 10.12+, Linux (kernel 2.6+), Arch Linux
- **RAM**: 512 MB minimum
- **Disk Space**: ~10 MB for binary, ~500 MB for build process
- **Terminal**: UTF-8 capable terminal emulator
- **Privileges**: sudo access required for network configuration changes

### Recommended
- **Terminal**: Modern terminal with true color support (iTerm2, Alacritty, Kitty, GNOME Terminal)
- **Display**: Minimum 80x24 characters

---

## Dependencies

### Runtime Dependencies

#### macOS

**Required:**
- macOS system utilities (pre-installed):
  - `ifconfig`
  - `networksetup`
  - `scutil`
  - `dscacheutil`

**Optional (for enhanced functionality):**
- **Clipboard support**: Built-in (pbcopy/pbpaste)
- **WiFi SSID detection**: 
  - `/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport` (pre-installed)
- **Network diagnostics**: 
  - `ping`, `traceroute`, `nslookup`, `dig` (pre-installed)

**Installation command (for optional tools):**
```bash
# All required tools are pre-installed on macOS
# Optional: Install additional network utilities via Homebrew
brew install bind  # For dig and nslookup
```

#### Linux (Debian/Ubuntu/Pop!_OS/Linux Mint)

**Required:**
```bash
sudo apt-get update
sudo apt-get install -y iproute2
```

**Recommended:**
```bash
sudo apt-get install -y \
    xclip \
    wireless-tools \
    iw \
    net-tools \
    dnsutils \
    iputils-ping \
    traceroute
```

**Optional (for DNS cache flushing):**
```bash
# Install at least one of these:
sudo apt-get install systemd-resolved  # Usually pre-installed
# OR
sudo apt-get install nscd
# OR
sudo apt-get install dnsmasq
```

**Complete installation command:**
```bash
sudo apt-get update && sudo apt-get install -y \
    iproute2 \
    xclip \
    wireless-tools \
    iw \
    net-tools \
    dnsutils \
    iputils-ping \
    traceroute \
    nscd
```

#### Fedora/RHEL/CentOS

**Required:**
```bash
sudo dnf install -y iproute
```

**Recommended:**
```bash
sudo dnf install -y \
    xclip \
    wireless-tools \
    iw \
    net-tools \
    bind-utils \
    iputils \
    traceroute
```

**Complete installation command:**
```bash
sudo dnf install -y \
    iproute \
    xclip \
    wireless-tools \
    iw \
    net-tools \
    bind-utils \
    iputils \
    traceroute \
    nscd
```

#### Arch Linux/Manjaro

**Required:**
```bash
sudo pacman -S iproute2
```

**Recommended:**
```bash
sudo pacman -S \
    xclip \
    wireless_tools \
    iw \
    net-tools \
    bind-tools \
    iputils \
    traceroute
```

**Complete installation command:**
```bash
sudo pacman -S \
    iproute2 \
    xclip \
    wireless_tools \
    iw \
    net-tools \
    bind-tools \
    iputils \
    traceroute \
    dnsmasq
```

#### openSUSE

**Complete installation command:**
```bash
sudo zypper install -y \
    iproute2 \
    xclip \
    wireless-tools \
    iw \
    net-tools \
    bind-utils \
    iputils \
    traceroute
```

### Build Dependencies (only needed if building from source)

#### All Platforms

**Rust toolchain:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version  # Should show: rustc 1.70.0 or later
cargo --version  # Should show: cargo 1.70.0 or later
```

---

## Installation Methods

### Method 1: Install Pre-compiled Binary (Fastest)

**Files needed:**
- `netutil-tui` (the compiled binary)

**Steps:**

1. **Copy the binary to your system:**

   ```bash
   # For user installation (no sudo required)
   mkdir -p ~/.local/bin
   cp netutil-tui ~/.local/bin/netutil
   chmod +x ~/.local/bin/netutil
   
   # Add to PATH (if not already)
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

   OR

   ```bash
   # For system-wide installation (requires sudo)
   sudo cp netutil-tui /usr/local/bin/netutil
   sudo chmod +x /usr/local/bin/netutil
   ```

2. **Verify installation:**
   ```bash
   which netutil
   netutil --help || netutil  # Run the application
   ```

### Method 2: Build and Install from Source

**Files needed:**
- All source files in the project directory:
  - `Cargo.toml`
  - `Cargo.lock`
  - `src/main.rs`
  - `src/app.rs`
  - `src/event.rs`
  - `src/models.rs`
  - `src/network.rs`
  - `src/sudo.rs`
  - `src/ui.rs`

**Steps:**

1. **Install Rust (if not already installed):**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Navigate to the project directory:**
   ```bash
   cd /path/to/netutil
   ```

3. **Build the release binary:**
   ```bash
   cargo build --release
   ```
   
   The binary will be created at: `target/release/netutil-tui`

4. **Install the binary:**
   ```bash
   # Option 1: User installation
   mkdir -p ~/.local/bin
   cp target/release/netutil-tui ~/.local/bin/netutil
   chmod +x ~/.local/bin/netutil
   
   # Add to PATH if needed
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

   OR

   ```bash
   # Option 2: System-wide installation
   sudo cp target/release/netutil-tui /usr/local/bin/netutil
   sudo chmod +x /usr/local/bin/netutil
   ```

   OR

   ```bash
   # Option 3: Use cargo install
   cargo install --path .
   # This installs to ~/.cargo/bin/ (already in PATH)
   ```

5. **Verify installation:**
   ```bash
   which netutil
   netutil  # Run the application
   ```

### Method 3: Direct Cargo Installation (from source directory)

**Steps:**

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Install NetUtil:**
   ```bash
   cd /path/to/netutil
   cargo install --path .
   ```

   This automatically installs the binary to `~/.cargo/bin/netutil`

3. **Verify installation:**
   ```bash
   netutil
   ```

---

## Post-Installation

### 1. Verify PATH Configuration

Make sure the installation directory is in your PATH:

```bash
echo $PATH
```

Should include one of:
- `~/.local/bin`
- `/usr/local/bin`
- `~/.cargo/bin`

If not, add to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Then reload:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

### 2. Test Basic Functionality

**View network interfaces (no sudo required):**
```bash
netutil
```

**Test keyboard shortcuts:**
- Press `?` for help
- Press `q` to quit

### 3. Test Sudo Functionality

**Run with sudo for configuration features:**
```bash
sudo netutil
```

Test features that require sudo:
- Press `Ctrl+f` to flush DNS cache
- Press `e` to edit IP configuration
- Press `d` to edit DNS servers

### 4. Configure Sudo (Optional - for passwordless sudo)

If you want to run NetUtil with sudo without password prompts, create a sudoers file:

```bash
sudo visudo -f /etc/sudoers.d/netutil
```

Add this line (replace `yourusername` with your actual username):
```
yourusername ALL=(ALL) NOPASSWD: /usr/local/bin/netutil
```

**⚠️ Security Warning:** Only do this if you understand the security implications.

### 5. Create Desktop Entry (Optional - for GUI launchers)

**Linux:**
```bash
mkdir -p ~/.local/share/applications
cat > ~/.local/share/applications/netutil.desktop << 'EOF'
[Desktop Entry]
Name=NetUtil
Comment=Network Interface Manager
Exec=sudo netutil
Terminal=true
Type=Application
Categories=System;Network;
Icon=network-wired
Keywords=network;interface;wifi;dns;ip;
EOF
```

**macOS (Alfred/Spotlight):**
Create an alias in your shell configuration:
```bash
echo 'alias netutil-sudo="sudo netutil"' >> ~/.zshrc
source ~/.zshrc
```

---

## Troubleshooting

### "netutil: command not found"

**Solution:**
1. Check if binary is installed:
   ```bash
   ls -l ~/.local/bin/netutil
   ls -l /usr/local/bin/netutil
   ls -l ~/.cargo/bin/netutil
   ```

2. Add to PATH:
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

3. Use full path:
   ```bash
   ~/.local/bin/netutil
   ```

### "Permission denied" when running

**Solution:**
```bash
chmod +x ~/.local/bin/netutil
# OR
sudo chmod +x /usr/local/bin/netutil
```

### Clipboard not working (Linux)

**Solution:**
```bash
# Install clipboard utilities
sudo apt-get install xclip  # Debian/Ubuntu
sudo dnf install xclip      # Fedora
sudo pacman -S xclip        # Arch Linux
```

### WiFi SSID not showing

**macOS:**
- Should work out of the box. If not, try:
  ```bash
  ls -l /System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport
  ```

**Linux:**
```bash
# Install WiFi tools
sudo apt-get install wireless-tools iw  # Debian/Ubuntu
sudo dnf install wireless-tools iw      # Fedora
sudo pacman -S wireless_tools iw        # Arch Linux
```

### DNS cache flush fails

**macOS:**
- Requires sudo. Run: `sudo netutil`

**Linux:**
```bash
# Install at least one DNS caching service
sudo apt-get install nscd           # Debian/Ubuntu
sudo dnf install nscd               # Fedora
sudo pacman -S dnsmasq             # Arch Linux

# Or use systemd-resolved (usually pre-installed):
sudo systemctl status systemd-resolved
```

### Network configuration changes fail

**Solution:**
1. Run with sudo:
   ```bash
   sudo netutil
   ```

2. Ensure required tools are installed:
   ```bash
   # macOS - should be pre-installed
   which networksetup ifconfig
   
   # Linux
   which ip ifconfig
   ```

3. Check permissions:
   ```bash
   # User should be in netdev group (Linux)
   groups
   ```

### Build fails with "cargo: command not found"

**Solution:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify
cargo --version
```

### Build fails with dependency errors

**Solution:**
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

---

## Uninstallation

### Remove Binary

**If installed to ~/.local/bin:**
```bash
rm ~/.local/bin/netutil
```

**If installed to /usr/local/bin:**
```bash
sudo rm /usr/local/bin/netutil
```

**If installed via cargo:**
```bash
cargo uninstall netutil-tui
```

### Remove Configuration Files

NetUtil doesn't create configuration files, so no additional cleanup is needed.

### Remove Desktop Entry (if created)

```bash
rm ~/.local/share/applications/netutil.desktop
```

### Remove Sudoers File (if created)

```bash
sudo rm /etc/sudoers.d/netutil
```

---

## Distribution-Specific Notes

### macOS

- All required system utilities are pre-installed
- No additional dependencies needed for basic functionality
- Clipboard support works out of the box
- WiFi detection uses built-in `airport` and `networksetup` commands
- Recommended terminal: iTerm2 or built-in Terminal.app

### Debian/Ubuntu/Pop!_OS/Linux Mint

- Use `apt-get` package manager
- systemd-resolved is usually pre-installed
- May need to install `wireless-tools` for WiFi SSID detection
- Clipboard requires `xclip` or `xsel`

### Fedora/RHEL/CentOS

- Use `dnf` package manager (or `yum` on older versions)
- systemd-resolved is standard
- SELinux may require additional configuration for network changes

### Arch Linux/Manjaro

- Use `pacman` package manager
- Minimal base system - install all recommended dependencies
- May need to enable systemd-resolved:
  ```bash
  sudo systemctl enable systemd-resolved
  sudo systemctl start systemd-resolved
  ```

### openSUSE

- Use `zypper` package manager
- YaST can be used for network configuration alongside NetUtil
- Firewall (firewalld) may need configuration for network changes

---

## Summary of Files

### For Binary Installation
**Required:**
- `netutil-tui` (the compiled binary, ~5-10 MB)

### For Source Installation
**Required:**
- `Cargo.toml` - Project configuration
- `Cargo.lock` - Dependency lock file
- `src/main.rs` - Application entry point
- `src/app.rs` - Application state and logic
- `src/event.rs` - Event handling
- `src/models.rs` - Data structures
- `src/network.rs` - Network interface detection
- `src/sudo.rs` - Privileged operations
- `src/ui.rs` - User interface rendering

**Optional (recommended):**
- `README.md` - Project documentation
- `INSTALL.md` - This file
- `.gitignore` - Git ignore rules (if using version control)

---

## Platform Feature Matrix

| Feature | macOS | Linux | Arch Linux |
|---------|-------|-------|------------|
| Interface detection | ✅ | ✅ | ✅ |
| WiFi type detection | ✅ | ✅ | ✅ |
| WiFi SSID display | ✅ | ✅ | ✅ |
| IP configuration | ✅ | ✅ | ✅ |
| DNS configuration | ✅ | ✅ | ✅ |
| IPv6 support | ✅ | ✅ | ✅ |
| DNS cache flush | ✅ | ✅ | ✅ |
| Clipboard support | ✅ | ✅ | ✅ |
| Integrated terminal | ✅ | ✅ | ✅ |

---

## Getting Help

If you encounter issues not covered in this guide:

1. Check the main README.md for usage instructions
2. Verify all dependencies are installed
3. Ensure you're running the latest version
4. Try running with `sudo` for configuration features
5. Check your terminal supports UTF-8 and has minimum 80x24 size

---

**Last Updated:** 2025-12-26  
**NetUtil Version:** 0.3.0
