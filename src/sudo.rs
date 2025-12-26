use anyhow::{Context, Result};
use std::process::{Command, Stdio};

/// Execute a command with sudo, prompting for password if needed
pub fn execute_with_sudo(command: &str, args: &[&str]) -> Result<String> {
    let child = Command::new("sudo")
        .arg("-S") // Read password from stdin
        .arg(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn sudo command")?;

    // Wait for command to complete
    let output = child
        .wait_with_output()
        .context("Failed to wait for command")?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command failed: {}", stderr)
    }
}

/// Set interface to use DHCP
#[cfg(target_os = "macos")]
pub fn set_dhcp(interface: &str) -> Result<()> {
    execute_with_sudo("networksetup", &["-setdhcp", interface])?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn set_dhcp(interface: &str) -> Result<()> {
    // First, remove any static IP
    let _ = execute_with_sudo("ip", &["addr", "flush", "dev", interface]);

    // Then start DHCP client (try dhclient first, then dhcpcd)
    if execute_with_sudo("dhclient", &[interface]).is_err() {
        execute_with_sudo("dhcpcd", &[interface])?;
    }

    Ok(())
}

/// Set static IP address
#[cfg(target_os = "macos")]
pub fn set_static_ip(
    interface: &str,
    ip: &str,
    netmask: &str,
    gateway: Option<&str>,
) -> Result<()> {
    if let Some(gw) = gateway {
        execute_with_sudo("networksetup", &["-setmanual", interface, ip, netmask, gw])?;
    } else {
        execute_with_sudo("ifconfig", &[interface, ip, "netmask", netmask])?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn set_static_ip(
    interface: &str,
    ip: &str,
    netmask: &str,
    gateway: Option<&str>,
) -> Result<()> {
    // Calculate CIDR prefix from netmask
    let prefix = netmask_to_cidr(netmask)?;
    let ip_with_prefix = format!("{}/{}", ip, prefix);

    // Set IP address
    execute_with_sudo("ip", &["addr", "add", &ip_with_prefix, "dev", interface])?;

    // Bring interface up
    execute_with_sudo("ip", &["link", "set", interface, "up"])?;

    // Set gateway if provided
    if let Some(gw) = gateway {
        execute_with_sudo(
            "ip",
            &["route", "add", "default", "via", gw, "dev", interface],
        )?;
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn netmask_to_cidr(netmask: &str) -> Result<u8> {
    let ip: std::net::Ipv4Addr = netmask.parse()?;
    let octets = ip.octets();
    let mut cidr = 0u8;

    for octet in &octets {
        cidr += octet.count_ones() as u8;
    }

    Ok(cidr)
}

/// Set DNS servers
#[cfg(target_os = "macos")]
pub fn set_dns_servers(interface: &str, servers: &[&str]) -> Result<()> {
    let mut args = vec!["-setdnsservers", interface];
    args.extend(servers);
    execute_with_sudo("networksetup", &args)?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn set_dns_servers(_interface: &str, servers: &[&str]) -> Result<()> {
    // Create resolv.conf content
    let mut content = String::new();
    for server in servers {
        content.push_str(&format!("nameserver {}\n", server));
    }

    // Write to a temporary file
    std::fs::write("/tmp/resolv.conf.new", &content)?;

    // Move to /etc/resolv.conf with sudo
    execute_with_sudo("cp", &["/tmp/resolv.conf.new", "/etc/resolv.conf"])?;

    Ok(())
}

/// Set DNS search domains
#[cfg(target_os = "macos")]
pub fn set_search_domains(interface: &str, domains: &[&str]) -> Result<()> {
    let mut args = vec!["-setsearchdomains", interface];
    args.extend(domains);
    execute_with_sudo("networksetup", &args)?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn set_search_domains(_interface: &str, domains: &[&str]) -> Result<()> {
    // Read existing resolv.conf
    let existing = std::fs::read_to_string("/etc/resolv.conf").unwrap_or_default();

    // Create new content with search domains
    let mut content = String::new();

    // Add nameservers from existing config
    for line in existing.lines() {
        if line.starts_with("nameserver") {
            content.push_str(line);
            content.push('\n');
        }
    }

    // Add search domains
    if !domains.is_empty() {
        content.push_str("search ");
        content.push_str(&domains.join(" "));
        content.push('\n');
    }

    // Write to temporary file and move
    std::fs::write("/tmp/resolv.conf.new", &content)?;
    execute_with_sudo("cp", &["/tmp/resolv.conf.new", "/etc/resolv.conf"])?;

    Ok(())
}

/// Enable or disable interface
pub fn set_interface_status(interface: &str, enabled: bool) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let status = if enabled { "up" } else { "down" };
        execute_with_sudo("ifconfig", &[interface, status])?;
    }

    #[cfg(target_os = "linux")]
    {
        let status = if enabled { "up" } else { "down" };
        execute_with_sudo("ip", &["link", "set", interface, status])?;
    }

    Ok(())
}

/// Disable IPv6 on an interface
#[cfg(target_os = "linux")]
pub fn disable_ipv6(interface: &str) -> Result<()> {
    // Check if IPv6 is available for this interface
    let sysctl_path = format!("/proc/sys/net/ipv6/conf/{}/disable_ipv6", interface);
    if !std::path::Path::new(&sysctl_path).exists() {
        anyhow::bail!("IPv6 is not available on this interface (kernel module may be disabled)");
    }

    let sysctl_key = format!("net.ipv6.conf.{}.disable_ipv6", interface);
    execute_with_sudo("sysctl", &["-w", &format!("{}=1", sysctl_key)])?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn disable_ipv6(interface: &str) -> Result<()> {
    execute_with_sudo("networksetup", &["-setv6off", interface])?;
    Ok(())
}

/// Enable IPv6 on an interface
#[cfg(target_os = "linux")]
pub fn enable_ipv6(interface: &str) -> Result<()> {
    // Check if IPv6 is available for this interface
    let sysctl_path = format!("/proc/sys/net/ipv6/conf/{}/disable_ipv6", interface);
    if !std::path::Path::new(&sysctl_path).exists() {
        anyhow::bail!("IPv6 is not available on this interface (kernel module may be disabled)");
    }

    let sysctl_key = format!("net.ipv6.conf.{}.disable_ipv6", interface);
    execute_with_sudo("sysctl", &["-w", &format!("{}=0", sysctl_key)])?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn enable_ipv6(interface: &str) -> Result<()> {
    execute_with_sudo("networksetup", &["-setv6automatic", interface])?;
    Ok(())
}

/// Set static IPv6 address
#[cfg(target_os = "linux")]
pub fn set_static_ipv6(interface: &str, ipv6: &str, prefix: u8) -> Result<()> {
    let addr = format!("{}/{}", ipv6, prefix);
    execute_with_sudo("ip", &["-6", "addr", "add", &addr, "dev", interface])?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn set_static_ipv6(interface: &str, ipv6: &str, prefix: u8) -> Result<()> {
    let addr = format!("{}/{}", ipv6, prefix);
    execute_with_sudo("ifconfig", &[interface, "inet6", &addr])?;
    Ok(())
}

/// Flush DNS cache on macOS
#[cfg(target_os = "macos")]
pub fn flush_dns_cache() -> Result<()> {
    execute_with_sudo("dscacheutil", &["-flushcache"])?;
    execute_with_sudo("killall", &["-HUP", "mDNSResponder"])?;
    Ok(())
}

/// Flush DNS cache on Linux
#[cfg(target_os = "linux")]
pub fn flush_dns_cache() -> Result<()> {
    // Try systemd-resolved first (most common on modern Linux)
    if execute_with_sudo("resolvectl", &["flush-caches"]).is_ok() {
        return Ok(());
    }

    // Try systemd-resolve (older systemd versions)
    if execute_with_sudo("systemd-resolve", &["--flush-caches"]).is_ok() {
        return Ok(());
    }

    // Try nscd (Name Service Cache Daemon)
    if execute_with_sudo("nscd", &["-i", "hosts"]).is_ok() {
        return Ok(());
    }

    // Try dnsmasq
    if execute_with_sudo("killall", &["-HUP", "dnsmasq"]).is_ok() {
        return Ok(());
    }

    anyhow::bail!("Could not flush DNS cache. No supported DNS caching service found.")
}
