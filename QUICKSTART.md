# Quick Start Guide

## Installation Steps

### 1. Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Build the Application

```bash
cd /Users/vicnunes/dev/tui/netutil
cargo build --release
```

This will download all dependencies and compile the application. The first build may take a few minutes.

### 3. Run the Application

For viewing only (no sudo required):
```bash
cargo run --release
```

For making configuration changes (sudo required):
```bash
sudo cargo run --release
# or
sudo ./target/release/netutil-tui
```

## Quick Tutorial

### First Steps
1. When you launch the app, you'll see a table of all network interfaces with their types
2. Use `↑` and `↓` arrow keys (or `k` and `j`) to navigate
3. Press `?` to see the help menu with all keyboard shortcuts
4. Press `i` to see detailed information about the selected interface

### New Features Overview

**Interface Types**
The app now shows the type of each interface:
- Ethernet - Wired network connections
- WiFi - Wireless network connections  
- Loopback - Local loopback interface (lo)
- Bridge - Network bridges
- Virtual - Docker, veth, virtual interfaces
- Tunnel/VPN - VPN and tunnel interfaces

**Detailed View (Press `i`)**
- Shows all IP addresses (IPv4 and IPv6)
- Displays netmasks and broadcast addresses
- Shows DNS servers and search domains
- Displays MTU and other interface details

**IP Configuration (Press `e`)**
- Toggle between DHCP and Static IP
- Set IP address, netmask, and gateway
- Changes require sudo and confirmation

**DNS Configuration (Press `d`)**
- Add multiple DNS servers
- Configure search domains
- Navigate with arrow keys
- Press `a` to add, `x` to delete
- Press `Ctrl+s` to save

**IPv6 Configuration (Press `6`)**
- Enable or disable IPv6
- Set static IPv6 addresses
- Configure prefix length
- Quick toggle with Space key

## Common Tasks

### View Interface Details
1. Navigate to an interface with arrow keys
2. Press `i` to open detailed view
3. See all IP addresses, DNS servers, and configuration
4. Press `Esc` to return to main view

### Set a Static IP Address
1. Navigate to your interface
2. Press `e` to edit IP configuration
3. Press `Space` to switch from DHCP to Static
4. Press `Tab` to move to the IP field
5. Type your IP address (e.g., 192.168.1.100)
6. Press `Tab` and enter netmask (e.g., 255.255.255.0)
7. Press `Tab` and optionally enter gateway
8. Press `Enter` to proceed
9. Confirm when prompted
10. Enter your sudo password

### Enable DHCP
1. Navigate to your interface
2. Press `e` to edit IP configuration
3. Ensure DHCP is selected (press `Space` to toggle)
4. Press `Enter` to apply
5. Confirm when prompted
6. Enter your sudo password

### Change DNS Servers
1. Press `d` to edit DNS configuration
2. Use `↑`/`↓` to navigate existing servers
3. Press `a` to add a new server
4. Type the DNS IP (e.g., 8.8.8.8 for Google DNS)
5. Repeat to add more servers (e.g., 8.8.4.4)
6. Press `Tab` to switch to search domains
7. Press `a` to add a domain, `x` to delete
8. Press `Ctrl+s` to save
9. Confirm when prompted
10. Enter your sudo password

### Disable IPv6
1. Navigate to your interface
2. Press `6` to edit IPv6 configuration
3. Press `Space` to toggle IPv6 off
4. Press `Enter` to apply
5. Confirm when prompted
6. Enter your sudo password

### Enable/Disable an Interface
1. Navigate to the interface
2. Press `t` to toggle
3. Confirm the action
4. Enter your sudo password

## Keyboard Shortcuts Reference

### Basic Navigation
- `↑`/`↓` or `j`/`k` - Move up/down
- `PgUp`/`PgDn` - Page up/down
- `Home`/`End` or `g`/`G` - First/last item

### Viewing
- `i` - Detailed interface view
- `r` - Refresh data
- `s`/`S` - Sort forward/backward
- `/` - Search
- `?` - Help

### Configuration (requires sudo)
- `e` - Edit IP (DHCP/Static)
- `d` - Edit DNS
- `6` - Edit IPv6
- `t` - Toggle interface

### In Edit Modes
- `Tab` - Next field
- `Space` - Toggle option
- `Enter` - Apply/Confirm
- `Esc` - Cancel
- `a` - Add (in DNS mode)
- `x` - Delete (in DNS mode)
- `Ctrl+s` - Save (in DNS mode)

### Clipboard
- `c` - Copy interface name
- `Ctrl+i` - Copy IP
- `Ctrl+m` - Copy MAC

## Tips

1. **Always run with sudo for configuration changes** - The app will show what would happen but can't apply changes without sudo

2. **Use the detailed view** - Press `i` to see all information about an interface before making changes

3. **Test with non-critical interfaces first** - Try configuration changes on a secondary interface before modifying your primary connection

4. **Confirmation dialogs** - Every configuration change shows a confirmation dialog - review it carefully before pressing Enter

5. **Refresh after changes** - Press `r` to reload interface data after making changes

## Troubleshooting

### "Permission denied" when making changes
You need to run with sudo:
```bash
sudo ./target/release/netutil-tui
```

### Interface types showing as "Unknown"
This is normal for unusual interface types. The app can still manage them.

### DNS changes don't persist after reboot
On some systems, you need to configure DNS persistence in your network manager settings.

### DHCP not working
Ensure either `dhclient` or `dhcpcd` is installed:
```bash
# Debian/Ubuntu
sudo apt-get install isc-dhcp-client

# Arch Linux
sudo pacman -S dhcpcd
```

### IPv6 toggle not working
Ensure `sysctl` is available and you have appropriate permissions.

## Next Steps

1. Explore the detailed view (`i`) for each interface
2. Try sorting by different columns
3. Practice with search/filter (`/`)
4. Configure DNS servers on a test setup
5. Experiment with DHCP/Static switching on a non-critical interface
6. Read the full README.md for advanced features

## Safety Notes

⚠️ **Important Safety Tips:**

1. **Don't modify your primary network connection** without having physical access to the machine
2. **Test on secondary interfaces first** before modifying production network settings
3. **Note your current configuration** before making changes
4. **Have a backup connection** available in case of misconfiguration
5. **Use confirmation dialogs** - they're there to prevent accidents

## Getting Help

- Press `?` in the app for keyboard shortcuts
- Read README.md for comprehensive documentation
- Check EXAMPLES.md for common usage scenarios
- Review the code in src/ for technical details
