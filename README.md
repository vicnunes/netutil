# NetUtil - Network Interface Manager

A cross-platform Terminal User Interface (TUI) application for managing network interfaces on macOS, Linux, and Arch Linux. Built with Rust, using `ratatui` for the UI and `crossterm` for terminal handling.

## Features

### Interface Management
- **Interface Discovery**: Automatically detects all network interfaces on your system
- **Interface Type Detection**: Identifies Ethernet, WiFi, Loopback, Bridge, Virtual, and Tunnel/VPN interfaces
- **WiFi Information**: Displays SSID for connected WiFi interfaces
- **Detailed Information**: View IP addresses (IPv4 and IPv6), MAC addresses, subnet masks, and interface status
- **DNS Configuration**: Display and edit system DNS servers and search domains
- **DNS Cache Flush**: Clear system DNS cache with `Ctrl+f` (requires sudo)
- **Detailed View**: Press `i` to see comprehensive details for the selected interface

### Network Configuration (requires sudo)
- **DHCP/Static IP**: Toggle between DHCP and static IP configuration
- **IPv4 Configuration**: Set static IP addresses, netmasks, and gateways
- **IPv6 Support**: Enable/disable IPv6, configure static IPv6 addresses
- **DNS Management**: Add, edit, and remove multiple DNS servers and search domains
- **Interface Control**: Enable/disable network interfaces

### User Interface
- **Sortable Table**: Sort by any column (interface name, type, IP address, MAC address, subnet mask, DNS servers, status)
- **Search & Filter**: Case-insensitive search across interface names, types, IP addresses, and MAC addresses
- **Pagination**: Navigate large numbers of interfaces with keyboard shortcuts
- **Clipboard Support**: Copy interface details to clipboard
- **Confirmation Dialogs**: All configuration changes require confirmation before execution
- **Integrated Terminal**: Execute network commands (ping, traceroute, etc.) without leaving the app
- **Help Menu**: Comprehensive keyboard shortcuts and usage guide

## Requirements

- Rust 1.70 or later (for building)
- macOS, Linux, or Arch Linux
- sudo privileges (for modifying network configuration)

## Installation

### Building from Source

1. Install Rust if you haven't already:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone or navigate to the project directory:
```bash
cd /path/to/netutil
```

3. Build the project:
```bash
cargo build --release
```

4. The binary will be available at `target/release/netutil-tui`

### Running

```bash
# Run directly with cargo
cargo run --release

# Or run the binary
./target/release/netutil-tui

# For network configuration changes, run with sudo
sudo ./target/release/netutil-tui
```

## Keyboard Shortcuts

### Navigation
- `↑` or `k` - Move up
- `↓` or `j` - Move down
- `PgUp` or `Ctrl+u` - Previous page
- `PgDn` or `Ctrl+d` - Next page
- `Home` or `g` - Go to first item
- `End` or `G` - Go to last item

### Viewing
- `i` - Show detailed interface information
- `r` - Refresh interface data
- `s` - Cycle sort column forward
- `S` - Cycle sort column backward
- `/` - Enter search mode
- `?` - Show help menu

### Network Configuration (requires sudo)
- `e` - Edit IP configuration (DHCP/Static)
- `d` - Edit DNS servers and search domains
- `6` - Edit IPv6 settings
- `t` - Toggle interface up/down

### Clipboard Operations
- `c` - Copy selected interface name
- `Ctrl+i` - Copy IP address
- `Ctrl+m` - Copy MAC address

### DNS Tools
- `Ctrl+f` - Flush DNS cache (requires sudo)

### Terminal
- `x` - Open integrated terminal
- In terminal mode:
  - Type commands and press `Enter` to execute
  - `↑`/`↓` - Scroll through output
  - `Ctrl+l` - Clear terminal
  - `Esc` - Exit terminal

### Other
- `q` - Quit application
- `Esc` - Cancel current operation/clear search
- `Tab` - Navigate between fields in edit modes
- `Enter` - Confirm action
- `Space` - Toggle options in edit modes

## Detailed Features

### IP Configuration
When you press `e` to edit IP configuration, you can:
1. Toggle between DHCP and Static mode using `Space`
2. In Static mode, enter:
   - IP Address
   - Netmask
   - Gateway (optional)
3. Use `Tab` to move between fields
4. Press `Enter` to apply changes
5. Confirm the action in the dialog

### DNS Configuration
When you press `d` to edit DNS configuration, you can:
1. Add multiple DNS servers
2. Add search domains
3. Navigate with `↑`/`↓` arrows
4. Press `a` to add a new entry
5. Press `x` to delete the selected entry
6. Press `Ctrl+s` to save changes
7. Confirm the action in the dialog

### IPv6 Configuration
When you press `6` to edit IPv6 configuration, you can:
1. Toggle IPv6 on/off using `Space`
2. If enabled, optionally set a static IPv6 address
3. Set the prefix length (default: 64)
4. Leave address empty for SLAAC (automatic configuration)
5. Press `Enter` to apply changes
6. Confirm the action in the dialog

### Detailed View
Press `i` on any interface to see:
- Interface name and type
- Status (UP/DOWN)
- MAC address
- WiFi SSID (for WiFi interfaces)
- MTU
- All IP addresses (IPv4 and IPv6) with netmasks and broadcast addresses
- All DNS servers
- Search domains

## Platform-Specific Notes

### macOS
- Uses `ifconfig` for interface details
- Uses `scutil --dns` for DNS configuration
- Uses `networksetup` for configuration changes and WiFi detection
- Uses `airport` command for WiFi SSID detection
- DNS cache flush uses `dscacheutil` and `mDNSResponder`
- Network modifications require `sudo` privileges

### Linux
- Uses `/sys/class/net` for MAC addresses and WiFi detection
- Uses `iwgetid` or `iw` for WiFi SSID detection
- Supports both `systemd-resolved` and `/etc/resolv.conf` for DNS
- DNS cache flush supports `resolvectl`, `systemd-resolve`, `nscd`, and `dnsmasq`
- Uses `ip` command for network configuration
- Uses `sysctl` for IPv6 enable/disable
- Network modifications require `sudo` privileges

### Arch Linux
- Same as Linux
- Ensure `iproute2` package is installed for network configuration

## Architecture

The application is structured into several modules:

- `main.rs` - Entry point and terminal setup
- `app.rs` - Application state and logic
- `models.rs` - Data structures for network interfaces and configuration
- `network.rs` - System-specific network interface detection
- `sudo.rs` - Sudo command execution for network configuration
- `ui.rs` - UI rendering with ratatui
- `event.rs` - Keyboard event handling

## Security Considerations

- Reading network interface information does not require elevated privileges
- **All network configuration changes require sudo privileges**
- The application uses `sudo -S` to execute privileged commands
- You will be prompted for your password when making configuration changes
- All configuration changes require explicit confirmation
- The application validates inputs before executing system commands

## Examples

### Setting a Static IP Address
1. Navigate to your interface
2. Press `e` to edit IP configuration
3. Press `Space` to switch to Static mode
4. Use `Tab` to navigate fields and enter:
   - IP: 192.168.1.100
   - Netmask: 255.255.255.0
   - Gateway: 192.168.1.1
5. Press `Enter` to proceed
6. Confirm the action when prompted
7. Enter your sudo password when prompted

### Changing DNS Servers
1. Press `d` to edit DNS configuration
2. Use `↑`/`↓` to navigate existing servers
3. Press `a` to add a new server
4. Type the DNS server IP (e.g., 8.8.8.8)
5. Press `Tab` to switch to search domains if needed
6. Press `Ctrl+s` to save
7. Confirm the action when prompted
8. Enter your sudo password when prompted

### Disabling IPv6
1. Navigate to your interface
2. Press `6` to edit IPv6 configuration
3. Press `Space` to toggle IPv6 off
4. Press `Enter` to apply
5. Confirm the action when prompted
6. Enter your sudo password when prompted

### Using the Terminal
1. Press `x` to open the terminal
2. Type any command, for example:
   - `ping 8.8.8.8` - Test connectivity
   - `traceroute google.com` - Trace route to host
   - `nslookup google.com` - DNS lookup
   - `ifconfig en0` - Show specific interface details
   - `netstat -rn` - Show routing table
3. Press `Enter` to execute
4. Use `↑`/`↓` to scroll through output
5. Press `Ctrl+l` to clear the terminal
6. Press `Esc` to return to main view

**Note:** Terminal commands run with the same privileges as the app. Use `sudo` prefix for commands requiring elevation.

### Flushing DNS Cache
1. Press `Ctrl+f` to flush the DNS cache
2. Enter your sudo password when prompted
3. The cache will be cleared using the appropriate method for your OS:
   - **macOS**: Uses `dscacheutil -flushcache` and restarts `mDNSResponder`
   - **Linux**: Tries `resolvectl`, `systemd-resolve`, `nscd`, or `dnsmasq` (whichever is available)
4. A status message will confirm success or show any errors

## Troubleshooting

### "command not found: cargo"
Install Rust using rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### No interfaces showing
Ensure you have network interfaces configured on your system. Run `ip addr` (Linux) or `ifconfig` (macOS) to verify.

### Clipboard not working
Make sure you have the necessary system clipboard libraries:
- Linux: `sudo apt-get install xclip` or `sudo apt-get install xsel`
- macOS: Built-in, should work out of the box

### DNS servers not displaying
- macOS: Ensure `scutil` is available
- Linux: Check `/etc/resolv.conf` or `resolvectl status`

### Configuration changes not working
- Ensure you're running the application with sudo: `sudo ./target/release/netutil-tui`
- Check that you have the required tools installed:
  - macOS: `networksetup`, `ifconfig`
  - Linux: `ip`, `dhclient` or `dhcpcd`, `sysctl`

### Permission denied errors
All network configuration changes require sudo privileges. Make sure you:
1. Run the application with `sudo` when making configuration changes
2. Enter your password when prompted
3. Have sudo privileges on the system

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is provided as-is for educational and practical use.

## Roadmap

Completed features:
- ✅ Interface type detection (Ethernet, WiFi, VPN, etc.)
- ✅ Detailed interface view
- ✅ DHCP/Static IP configuration
- ✅ Multiple DNS servers support
- ✅ IPv6 configuration
- ✅ IPv6 enable/disable
- ✅ Sudo password prompt support
- ✅ Confirmation dialogs

Future enhancements:
- [ ] Route table viewing and editing
- [ ] Network statistics and monitoring
- [ ] Configuration profiles (save/load network settings)
- [ ] Export data to JSON/CSV
- [ ] Windows support
- [ ] Mouse support for clicking column headers
- [ ] Wireless network scanning and connection
- [ ] VPN configuration support
- [ ] Network diagnostics (ping, traceroute integration)

## Changelog

### Version 0.3.0 (Current)
- **Fixed WiFi interface detection on macOS** - Now correctly identifies WiFi interfaces (e.g., en1)
- **Added WiFi SSID display** - Shows connected network name for WiFi interfaces
- **Added DNS cache flush** - Press `Ctrl+f` to clear DNS cache (supports macOS and Linux)
- Updated help documentation with new features

### Version 0.2.0
- Added interface type detection
- Added detailed interface view (press `i`)
- Improved IP configuration with DHCP/Static toggle
- Enhanced DNS configuration with multiple servers
- Added IPv6 configuration support
- Added IPv6 enable/disable functionality
- Implemented sudo password prompts
- Added confirmation dialogs for all changes
- **Added integrated terminal** (press `x`) for running network commands
- Updated UI with better navigation

### Version 0.1.0
- Initial release
- Basic interface listing
- DNS server display
- Search and filter
- Clipboard support
- Basic sorting
