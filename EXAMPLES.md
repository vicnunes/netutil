# Usage Examples

This document provides common usage scenarios for the NetUtil TUI application.

## Example 1: Finding Your WiFi Interface

1. Launch the app: `cargo run --release`
2. Press `/` to enter search mode
3. Type `wlan` or `wifi` or `en0` (depending on your system)
4. The table will filter to show matching interfaces
5. Press `Esc` to exit search mode

## Example 2: Copying Your Local IP Address

1. Navigate to your primary network interface using `↑`/`↓` arrows
2. Press `Ctrl+i` to copy the IP address to clipboard
3. You'll see a confirmation message: "Copied IP Address to clipboard"
4. Paste it anywhere you need (it's in your clipboard)

## Example 3: Finding All Interfaces with a Specific IP Range

1. Press `/` to search
2. Type `192.168` to find all interfaces in the 192.168.x.x range
3. Or type `10.` to find 10.x.x.x addresses
4. Navigate through results with arrow keys

## Example 4: Sorting by Status to See Active Interfaces

1. Press `s` repeatedly until the "Status" column is highlighted
2. All interfaces will be sorted by their UP/DOWN status
3. Press `s` again to reverse the sort order (use the ▲/▼ indicator)

## Example 5: Viewing DNS Configuration

1. Launch the app to see the main table
2. Look at the "DNS Servers" column (rightmost)
3. You'll see all configured DNS servers for your system
4. To copy DNS info, select any row and press `Ctrl+d` for more options

## Example 6: Navigating Large Numbers of Interfaces

1. If you have many virtual interfaces or VPNs, use pagination:
   - `Ctrl+d` - Jump down one page
   - `Ctrl+u` - Jump up one page
   - `g` - Jump to the first interface
   - `G` - Jump to the last interface
2. The status bar shows your current page number

## Example 7: Comparing Multiple Interfaces

1. Navigate to first interface of interest
2. Press `c` to copy its name
3. Paste in a note or terminal
4. Navigate to next interface
5. Press `Ctrl+i` to copy its IP
6. Repeat for all interfaces you want to compare

## Example 8: Monitoring Interface Changes

1. Make a note of current interface states
2. Make a change outside the app (connect to VPN, enable WiFi, etc.)
3. Press `r` in the app to refresh
4. See the updated interface list

## Example 9: Quickly Finding MAC Addresses

1. Press `s` to sort by "MAC Address" column
2. All similar MAC addresses (same vendor) will be grouped
3. Select the interface you need
4. Press `Ctrl+m` to copy the MAC address

## Example 10: Using the Help System

1. Press `?` at any time to see the help menu
2. Scroll through all available keyboard shortcuts
3. Press `Esc` or `q` to return to the main view

## Example 11: Searching by Partial IP

1. If you remember part of an IP address (e.g., "192.168.1")
2. Press `/` and type that partial IP
3. Only matching interfaces will be shown
4. Search is case-insensitive and supports partial matches

## Example 12: Identifying Virtual Interfaces

1. Look for interfaces with names like:
   - `docker0` - Docker bridge
   - `veth*` - Virtual Ethernet (Docker containers)
   - `tun*` / `tap*` - VPN interfaces
   - `br-*` - Network bridges
   - `virbr*` - Virtual machine bridges
2. Use search to filter: `/docker` or `/veth`

## Example 13: Preparing for Network Configuration

Before modifying network settings:

1. Press `r` to ensure you have current data
2. Navigate to the interface you want to modify
3. Press `c` to copy the interface name
4. Note the current IP, subnet mask, and status
5. Press `e` to start editing IP address
6. The app will show you what command would be executed
7. Press `Esc` to cancel (default safe mode)

## Example 14: Working with Multiple Network Connections

If you have Ethernet, WiFi, and VPN all active:

1. Sort by "Status" to see all UP interfaces first
2. Or search for each type:
   - `/eth` for Ethernet
   - `/wlan` or `/en` for WiFi
   - `/tun` or `/vpn` for VPN
3. Compare their IPs and DNS settings
4. Copy whichever information you need

## Example 15: Troubleshooting Network Issues

1. Launch the app to see all interfaces
2. Check which interfaces are UP vs DOWN
3. Verify IP addresses are in expected ranges
4. Check DNS servers are correct (should see your router or public DNS)
5. Press `r` periodically to refresh while troubleshooting
6. Use search to quickly filter to problem interfaces

## Command Line Integration

### Get interface info without TUI

While the app is a TUI, you can use standard tools alongside it:

```bash
# Compare with system tools
ip addr                    # Linux
ifconfig                   # macOS

# Check DNS separately
cat /etc/resolv.conf       # Linux
scutil --dns              # macOS

# Monitor changes
watch -n 1 'ip addr'      # Linux - watch changes
```

## Tips and Tricks

1. **Quick navigation**: Use `j`/`k` (Vim-style) instead of arrows for faster navigation
2. **Fast refresh**: Keep the app open and press `r` whenever you need fresh data
3. **Search then sort**: First filter with `/`, then sort with `s` for precise results
4. **Clipboard workflow**: Copy multiple fields from different interfaces sequentially
5. **Use Help liberally**: Press `?` whenever you forget a shortcut

## Common Workflows

### Workflow: Document Your Network Setup
1. Launch app
2. Navigate through each interface
3. Copy name (`c`), IP (`Ctrl+i`), and MAC (`Ctrl+m`)
4. Paste into your documentation
5. Note the DNS servers from the table
6. Press `r` before finalizing to ensure accuracy

### Workflow: Verify Network After Changes
1. Make network changes in system settings
2. Press `r` in the app
3. Verify the changes took effect
4. Check interface status (UP/DOWN)
5. Confirm IP addresses and DNS

### Workflow: Quick IP Lookup
1. Press `/`
2. Type interface name
3. Press `Enter`
4. Press `Ctrl+i` to copy IP
5. Done - less than 5 seconds!

## Integration with Other Tools

You can use NetUtil alongside:
- `ping` - After copying an IP, ping it in another terminal
- `traceroute` - Trace routes from specific interfaces
- `nmap` - Scan networks you've identified
- `wireshark` - Capture traffic on interfaces you've selected
- Network managers - Cross-reference with system network tools

## Performance Notes

- The app is lightweight and responsive
- Refreshing (`r`) is instant for typical systems
- Search/filter is real-time as you type
- Handles hundreds of interfaces (useful for containers/VMs)
- Minimal CPU usage when idle
