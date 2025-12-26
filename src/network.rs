use crate::models::{DnsConfiguration, InterfaceAddress, InterfaceType, NetworkInterface};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::net::IpAddr;

#[cfg(target_os = "macos")]
use std::process::Command;

#[cfg(target_os = "linux")]
use std::fs;

/// Get all network interfaces on the system
pub fn get_network_interfaces() -> Result<Vec<NetworkInterface>> {
    let addrs = if_addrs::get_if_addrs().context("Failed to get network interfaces")?;
    let mut interfaces_map: HashMap<String, NetworkInterface> = HashMap::new();

    for iface in addrs {
        let entry = interfaces_map
            .entry(iface.name.clone())
            .or_insert_with(|| NetworkInterface {
                name: iface.name.clone(),
                interface_type: InterfaceType::from_name(&iface.name),
                ip_addresses: Vec::new(),
                mac_address: None,
                is_up: !iface.is_loopback(),
                is_loopback: iface.is_loopback(),
                mtu: None,
                ipv6_enabled: true,
                ssid: None,
            });

        let ip = iface.addr.ip();
        let netmask = match &iface.addr {
            if_addrs::IfAddr::V4(v4) => Some(IpAddr::V4(v4.netmask)),
            if_addrs::IfAddr::V6(v6) => Some(IpAddr::V6(v6.netmask)),
        };

        let broadcast = match &iface.addr {
            if_addrs::IfAddr::V4(v4) => v4.broadcast.map(IpAddr::V4),
            if_addrs::IfAddr::V6(_) => None,
        };

        entry.ip_addresses.push(InterfaceAddress {
            ip,
            netmask,
            broadcast,
        });
    }

    // Get MAC addresses and detect WiFi interfaces
    #[cfg(target_os = "macos")]
    {
        for (name, iface) in interfaces_map.iter_mut() {
            if let Ok(mac) = get_mac_address_macos(name) {
                iface.mac_address = Some(mac);
            }

            // Detect WiFi and get SSID
            if is_wifi_interface_macos(name) {
                iface.interface_type = InterfaceType::WiFi;
                if let Ok(ssid) = get_wifi_ssid_macos(name) {
                    iface.ssid = Some(ssid);
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        for (name, iface) in interfaces_map.iter_mut() {
            if let Ok(mac) = get_mac_address_linux(name) {
                iface.mac_address = Some(mac);
            }

            // Detect WiFi and get SSID
            if is_wifi_interface_linux(name) {
                iface.interface_type = InterfaceType::WiFi;
                if let Ok(ssid) = get_wifi_ssid_linux(name) {
                    iface.ssid = Some(ssid);
                }
            }
        }
    }

    let mut result: Vec<NetworkInterface> = interfaces_map.into_values().collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(result)
}

#[cfg(target_os = "macos")]
fn get_mac_address_macos(interface_name: &str) -> Result<String> {
    let output = Command::new("ifconfig")
        .arg(interface_name)
        .output()
        .context("Failed to execute ifconfig")?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("ether ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                return Ok(parts[1].to_string());
            }
        }
    }

    Ok("N/A".to_string())
}

#[cfg(target_os = "linux")]
fn get_mac_address_linux(interface_name: &str) -> Result<String> {
    let path = format!("/sys/class/net/{}/address", interface_name);
    match fs::read_to_string(&path) {
        Ok(content) => Ok(content.trim().to_string()),
        Err(_) => Ok("N/A".to_string()),
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn get_mac_address_fallback(_interface_name: &str) -> Result<String> {
    Ok("N/A".to_string())
}

/// Check if an interface is WiFi on macOS
#[cfg(target_os = "macos")]
fn is_wifi_interface_macos(interface_name: &str) -> bool {
    // Check if networksetup recognizes this as a WiFi interface
    let output = Command::new("networksetup")
        .arg("-listallhardwareports")
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut is_wifi_port = false;

        for line in stdout.lines() {
            let line = line.trim();
            if line.contains("Wi-Fi") || line.contains("AirPort") {
                is_wifi_port = true;
            } else if is_wifi_port && line.starts_with("Device:") {
                if line.contains(interface_name) {
                    return true;
                }
                is_wifi_port = false;
            }
        }
    }

    false
}

/// Get WiFi SSID on macOS
#[cfg(target_os = "macos")]
fn get_wifi_ssid_macos(_interface_name: &str) -> Result<String> {
    let output = Command::new(
        "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport",
    )
    .arg("-I")
    .output()
    .context("Failed to get WiFi info")?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("SSID:") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                return Ok(parts[1].trim().to_string());
            }
        }
    }

    Ok("Not connected".to_string())
}

/// Check if an interface is WiFi on Linux
#[cfg(target_os = "linux")]
fn is_wifi_interface_linux(interface_name: &str) -> bool {
    // Check if wireless directory exists
    let wireless_path = format!("/sys/class/net/{}/wireless", interface_name);
    std::path::Path::new(&wireless_path).exists()
}

/// Get WiFi SSID on Linux
#[cfg(target_os = "linux")]
fn get_wifi_ssid_linux(interface_name: &str) -> Result<String> {
    // Try iwgetid first
    let output = Command::new("iwgetid")
        .arg(interface_name)
        .arg("-r")
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ssid.is_empty() {
                return Ok(ssid);
            }
        }
    }

    // Try iw as fallback
    let output = Command::new("iw")
        .arg("dev")
        .arg(interface_name)
        .arg("link")
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let line = line.trim();
            if line.starts_with("SSID:") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    return Ok(parts[1].trim().to_string());
                }
            }
        }
    }

    Ok("Not connected".to_string())
}

/// Get DNS configuration from the system
pub fn get_dns_configuration() -> Result<DnsConfiguration> {
    #[cfg(target_os = "macos")]
    {
        get_dns_configuration_macos()
    }

    #[cfg(target_os = "linux")]
    {
        get_dns_configuration_linux()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        Ok(DnsConfiguration {
            nameservers: Vec::new(),
            search_domains: Vec::new(),
        })
    }
}

#[cfg(target_os = "macos")]
fn get_dns_configuration_macos() -> Result<DnsConfiguration> {
    let output = Command::new("scutil")
        .arg("--dns")
        .output()
        .context("Failed to execute scutil")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut nameservers = Vec::new();
    let mut search_domains = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("nameserver[") {
            if let Some(ip_str) = line.split(':').nth(1) {
                if let Ok(ip) = ip_str.trim().parse::<IpAddr>() {
                    if !nameservers.contains(&ip) {
                        nameservers.push(ip);
                    }
                }
            }
        } else if line.starts_with("search domain[") {
            if let Some(domain) = line.split(':').nth(1) {
                let domain = domain.trim().to_string();
                if !search_domains.contains(&domain) {
                    search_domains.push(domain);
                }
            }
        }
    }

    Ok(DnsConfiguration {
        nameservers,
        search_domains,
    })
}

#[cfg(target_os = "linux")]
fn get_dns_configuration_linux() -> Result<DnsConfiguration> {
    // Try systemd-resolved first
    if let Ok(config) = get_dns_from_systemd_resolved() {
        return Ok(config);
    }

    // Fallback to /etc/resolv.conf
    match resolv_conf::Config::parse("/etc/resolv.conf") {
        Ok(config) => {
            let nameservers = config.nameservers.iter().map(|ns| ns.into()).collect();

            let search_domains = config
                .get_system_search_domains()
                .unwrap_or_default()
                .iter()
                .map(|d| d.to_string())
                .collect();

            Ok(DnsConfiguration {
                nameservers,
                search_domains,
            })
        }
        Err(_) => Ok(DnsConfiguration {
            nameservers: Vec::new(),
            search_domains: Vec::new(),
        }),
    }
}

#[cfg(target_os = "linux")]
fn get_dns_from_systemd_resolved() -> Result<DnsConfiguration> {
    let output = Command::new("resolvectl")
        .arg("status")
        .output()
        .context("Failed to execute resolvectl")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut nameservers = Vec::new();
    let mut search_domains = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("DNS Servers:") {
            if let Some(ips_str) = line.strip_prefix("DNS Servers:") {
                for ip_str in ips_str.split_whitespace() {
                    if let Ok(ip) = ip_str.parse::<IpAddr>() {
                        nameservers.push(ip);
                    }
                }
            }
        } else if line.starts_with("DNS Domain:") {
            if let Some(domain) = line.strip_prefix("DNS Domain:") {
                search_domains.push(domain.trim().to_string());
            }
        }
    }

    if nameservers.is_empty() {
        anyhow::bail!("No DNS servers found via systemd-resolved");
    }

    Ok(DnsConfiguration {
        nameservers,
        search_domains,
    })
}

/// Set a static IP address for an interface
#[cfg(target_os = "macos")]
#[allow(dead_code)]
pub fn set_static_ip(interface: &str, ip: &str, netmask: &str) -> Result<()> {
    let output = Command::new("sudo")
        .args(&["ifconfig", interface, ip, "netmask", netmask])
        .output()
        .context("Failed to set static IP")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to set static IP: {}", stderr);
    }

    Ok(())
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn set_static_ip(interface: &str, ip: &str, netmask: &str) -> Result<()> {
    // Calculate CIDR prefix from netmask
    let prefix = netmask_to_cidr(netmask)?;
    let ip_with_prefix = format!("{}/{}", ip, prefix);

    let output = Command::new("sudo")
        .args(&["ip", "addr", "add", &ip_with_prefix, "dev", interface])
        .output()
        .context("Failed to set static IP")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to set static IP: {}", stderr);
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

/// Enable or disable a network interface
#[cfg(target_os = "macos")]
#[allow(dead_code)]
pub fn set_interface_status(interface: &str, enabled: bool) -> Result<()> {
    let status = if enabled { "up" } else { "down" };
    let output = Command::new("sudo")
        .args(&["ifconfig", interface, status])
        .output()
        .context("Failed to set interface status")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to set interface status: {}", stderr);
    }

    Ok(())
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn set_interface_status(interface: &str, enabled: bool) -> Result<()> {
    let status = if enabled { "up" } else { "down" };
    let output = Command::new("sudo")
        .args(&["ip", "link", "set", interface, status])
        .output()
        .context("Failed to set interface status")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to set interface status: {}", stderr);
    }

    Ok(())
}
