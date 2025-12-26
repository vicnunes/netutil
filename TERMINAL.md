# Integrated Terminal Feature

## Overview

The NetUtil TUI application now includes an integrated terminal that allows you to execute network commands without leaving the application. This feature is designed to streamline network diagnostics and troubleshooting workflows.

## Accessing the Terminal

**Keyboard Shortcut:** Press `x` from the main view

## Features

### Command Execution
- Execute any system command
- View output in real-time
- Scroll through command history
- Clear terminal output

### Supported Operations
The terminal can run any command available on your system, including:
- Network diagnostics (ping, traceroute, nslookup)
- Interface information (ifconfig, ip)
- Routing table (netstat, route)
- DNS queries (dig, host)
- Port scanning (netcat, telnet)
- Any other system command

## Usage

### Basic Workflow

1. **Open Terminal**
   - Press `x` from the main interface view
   
2. **Enter Command**
   - Type your command at the `>` prompt
   - Example: `ping -c 4 8.8.8.8`
   
3. **Execute**
   - Press `Enter` to run the command
   - Output appears in the terminal window
   
4. **View Output**
   - Output is displayed line by line
   - Long output is automatically scrollable
   
5. **Scroll Through Output**
   - Use `↑` arrow to scroll up
   - Use `↓` arrow to scroll down
   
6. **Run Another Command**
   - Simply type a new command and press `Enter`
   - Previous output remains visible above
   
7. **Clear Terminal**
   - Press `Ctrl+l` to clear all output
   
8. **Exit Terminal**
   - Press `Esc` to return to main view

## Common Network Commands

### Connectivity Testing

```bash
# Ping a host
ping -c 4 google.com

# Ping with count (Linux/macOS)
ping -c 10 192.168.1.1

# Continuous ping (Ctrl+C to stop - not recommended in terminal)
ping 8.8.8.8
```

### DNS Lookups

```bash
# Basic DNS lookup
nslookup google.com

# Reverse DNS lookup
nslookup 8.8.8.8

# Query specific DNS server
nslookup google.com 1.1.1.1

# Dig command (more detailed)
dig google.com

# Short dig output
dig +short google.com
```

### Route Tracing

```bash
# Trace route to host
traceroute google.com

# Trace route with max hops
traceroute -m 15 8.8.8.8

# Fast traceroute (Linux)
traceroute -I google.com
```

### Interface Information

```bash
# Show specific interface (macOS)
ifconfig en0

# Show all interfaces (macOS)
ifconfig -a

# Show specific interface (Linux)
ip addr show eth0

# Show all interfaces (Linux)
ip addr

# Show interface statistics
netstat -i
```

### Routing Table

```bash
# Show routing table (macOS/BSD)
netstat -rn

# Show routing table (Linux)
ip route

# Show routing table (alternative)
route -n
```

### Port Testing

```bash
# Test if port is open
nc -zv google.com 80

# Test HTTPS port
nc -zv google.com 443

# Telnet to port
telnet google.com 80
```

### Network Statistics

```bash
# Show network connections
netstat -an

# Show listening ports
netstat -an | grep LISTEN

# Show established connections
netstat -an | grep ESTABLISHED
```

### ARP Table

```bash
# Show ARP cache
arp -a

# Show ARP for specific interface
arp -a -i en0
```

## Tips and Best Practices

### 1. Command Output Length
- Long-running commands (like `ping` without count) will continue until stopped
- Use commands with limited output when possible
- Examples:
  - ✅ `ping -c 4 google.com` (4 packets)
  - ❌ `ping google.com` (infinite)

### 2. Scrolling
- Terminal shows ~20 lines at a time
- Use `↑`/`↓` to navigate through longer output
- Scroll position indicator shows current position

### 3. Command History
- All commands and their output remain in the terminal
- Scroll up to see previous commands
- Use `Ctrl+l` to clear when terminal gets cluttered

### 4. Privileges
- Commands run with the same privileges as the app
- Most diagnostic commands don't require sudo
- If you need elevated privileges, run the app with sudo:
  ```bash
  sudo ./target/release/netutil-tui
  ```

### 5. Error Messages
- Failed commands show error output
- Exit codes are displayed for failed commands
- Example: `Command exited with status: 1`

### 6. Combining with Interface Selection
Workflow example:
1. Select an interface in the main view
2. Press `i` to see details and note the interface name
3. Press `x` to open terminal
4. Run commands specific to that interface:
   ```bash
   ping -I en0 8.8.8.8
   ifconfig en0
   ```

## Examples by Use Case

### Diagnosing Connectivity Issues

```bash
# Step 1: Test basic connectivity
ping -c 4 8.8.8.8

# Step 2: Test DNS resolution
nslookup google.com

# Step 3: Trace route to see where packets go
traceroute 8.8.8.8

# Step 4: Check if specific port is accessible
nc -zv google.com 443
```

### Checking DNS Configuration

```bash
# Query current DNS servers
nslookup google.com

# Test different DNS servers
nslookup google.com 8.8.8.8
nslookup google.com 1.1.1.1

# Detailed DNS query
dig google.com +trace
```

### Monitoring Network Traffic

```bash
# Show active connections
netstat -an | grep ESTABLISHED

# Show listening services
netstat -an | grep LISTEN

# Show network interface statistics
netstat -i
```

### Verifying Network Configuration

```bash
# Check interface details
ifconfig en0

# Check IP configuration (Linux)
ip addr show eth0

# Verify routing
netstat -rn

# Check ARP table
arp -a
```

## Limitations

1. **Interactive Commands**: Commands requiring interactive input (like passwords) won't work properly
2. **Long Output**: Very long output may cause the terminal to use more memory
3. **Signal Handling**: Can't send signals (Ctrl+C) to running commands
4. **Command Completion**: No tab completion or command history navigation
5. **Editing**: No command line editing features (use backspace to delete)

## Advanced Usage

### Chaining Commands

You can use shell operators:

```bash
# Run multiple commands
ping -c 2 8.8.8.8 && echo "Success"

# Pipe output
ifconfig en0 | grep inet

# Output redirection (creates files in current directory)
netstat -rn > routes.txt
```

### Quick Network Tests

```bash
# Test internet connectivity
ping -c 1 8.8.8.8 && echo "Internet OK"

# Test DNS
nslookup google.com > /dev/null && echo "DNS OK"

# Test specific host
ping -c 1 192.168.1.1 && echo "Gateway reachable"
```

## Troubleshooting

### Command Not Found
If you get "command not found" errors:
- The command may not be installed on your system
- Check if the command exists: `which ping`
- Install missing tools (e.g., `brew install inetutils` on macOS)

### Permission Denied
If you get permission errors:
- Some commands require sudo privileges
- Run the entire app with sudo if needed
- Example: `sudo ./target/release/netutil-tui`

### Output Not Showing
If command runs but no output appears:
- Some commands output to stderr instead of stdout
- The terminal captures both stdout and stderr
- Error lines are prefixed with "ERROR:"

### Terminal Gets Cluttered
If the terminal becomes hard to read:
- Press `Ctrl+l` to clear all output
- Start fresh with your next command

## Integration with Main Features

The terminal works seamlessly with other NetUtil features:

1. **After Viewing Interface Details** (`i`)
   - Open terminal (`x`)
   - Run diagnostics on that specific interface

2. **After Configuration Changes**
   - Make a network change (IP, DNS, etc.)
   - Open terminal to verify:
     ```bash
     ifconfig en0
     nslookup google.com
     ```

3. **Before Making Changes**
   - Test current configuration in terminal
   - Make informed decisions about changes needed

## Keyboard Reference

| Key | Action |
|-----|--------|
| `x` | Open terminal (from main view) |
| `Enter` | Execute command |
| `↑` | Scroll output up |
| `↓` | Scroll output down |
| `Ctrl+l` | Clear terminal |
| `Backspace` | Delete character from command |
| `Esc` | Exit terminal and return to main view |
| Any character | Add to command |

## Security Considerations

- Terminal commands execute with the privileges of the app
- Commands are not sanitized or restricted
- Use caution when running commands, especially with sudo
- Avoid commands that modify system files unless intentional
- The terminal doesn't store command history between sessions

## Future Enhancements

Potential improvements for future versions:
- Command history with ↑/↓ navigation
- Tab completion
- Command aliases
- Saved command snippets
- Background command execution
- Multiple terminal tabs
- Command output export
- Syntax highlighting
