use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceType {
    Ethernet,
    WiFi,
    Loopback,
    Bridge,
    Virtual,
    Tunnel,
    Unknown,
}

impl InterfaceType {
    pub fn from_name(name: &str) -> Self {
        let name_lower = name.to_lowercase();

        if name_lower.starts_with("lo") {
            InterfaceType::Loopback
        } else if name_lower.starts_with("eth")
            || name_lower.starts_with("en") && !name_lower.contains("wlan")
        {
            InterfaceType::Ethernet
        } else if name_lower.contains("wlan")
            || name_lower.contains("wifi")
            || name_lower.starts_with("wl")
        {
            InterfaceType::WiFi
        } else if name_lower.starts_with("br") || name_lower.starts_with("bridge") {
            InterfaceType::Bridge
        } else if name_lower.starts_with("veth")
            || name_lower.starts_with("docker")
            || name_lower.starts_with("virbr")
        {
            InterfaceType::Virtual
        } else if name_lower.starts_with("tun")
            || name_lower.starts_with("tap")
            || name_lower.contains("vpn")
        {
            InterfaceType::Tunnel
        } else {
            InterfaceType::Unknown
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            InterfaceType::Ethernet => "Ethernet",
            InterfaceType::WiFi => "WiFi",
            InterfaceType::Loopback => "Loopback",
            InterfaceType::Bridge => "Bridge",
            InterfaceType::Virtual => "Virtual",
            InterfaceType::Tunnel => "Tunnel/VPN",
            InterfaceType::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkInterface {
    pub name: String,
    pub interface_type: InterfaceType,
    pub ip_addresses: Vec<InterfaceAddress>,
    pub mac_address: Option<String>,
    pub is_up: bool,
    pub is_loopback: bool,
    pub mtu: Option<u32>,
    pub ipv6_enabled: bool,
    pub ssid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InterfaceAddress {
    pub ip: IpAddr,
    pub netmask: Option<IpAddr>,
    pub broadcast: Option<IpAddr>,
}

impl InterfaceAddress {
    pub fn is_ipv6(&self) -> bool {
        matches!(self.ip, IpAddr::V6(_))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfiguration {
    pub nameservers: Vec<IpAddr>,
    pub search_domains: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct InterfaceTableRow {
    pub name: String,
    pub interface_type: String,
    pub ip_address: String,
    pub mac_address: String,
    pub subnet_mask: String,
    pub dns_servers: String,
    pub status: String,
}

impl InterfaceTableRow {
    pub fn from_interface(iface: &NetworkInterface, dns: &DnsConfiguration) -> Self {
        let ip_address = iface
            .ip_addresses
            .first()
            .map(|addr| addr.ip.to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let subnet_mask = iface
            .ip_addresses
            .first()
            .and_then(|addr| addr.netmask.as_ref())
            .map(|mask| mask.to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let mac_address = iface
            .mac_address
            .clone()
            .unwrap_or_else(|| "N/A".to_string());

        let dns_servers = dns
            .nameservers
            .iter()
            .map(|ip| ip.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let status = if iface.is_up { "UP" } else { "DOWN" }.to_string();

        Self {
            name: iface.name.clone(),
            interface_type: iface.interface_type.as_str().to_string(),
            ip_address,
            mac_address,
            subnet_mask,
            dns_servers: if dns_servers.is_empty() {
                "N/A".to_string()
            } else {
                dns_servers
            },
            status,
        }
    }

    pub fn get_field(&self, column: &str) -> &str {
        match column {
            "Interface" => &self.name,
            "Type" => &self.interface_type,
            "IP Address" => &self.ip_address,
            "MAC Address" => &self.mac_address,
            "Subnet Mask" => &self.subnet_mask,
            "DNS Servers" => &self.dns_servers,
            "Status" => &self.status,
            _ => "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Interface,
    Type,
    IpAddress,
    MacAddress,
    SubnetMask,
    DnsServers,
    Status,
}

impl SortColumn {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortColumn::Interface => "Interface",
            SortColumn::Type => "Type",
            SortColumn::IpAddress => "IP Address",
            SortColumn::MacAddress => "MAC Address",
            SortColumn::SubnetMask => "Subnet Mask",
            SortColumn::DnsServers => "DNS Servers",
            SortColumn::Status => "Status",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            SortColumn::Interface => SortColumn::Type,
            SortColumn::Type => SortColumn::IpAddress,
            SortColumn::IpAddress => SortColumn::MacAddress,
            SortColumn::MacAddress => SortColumn::SubnetMask,
            SortColumn::SubnetMask => SortColumn::DnsServers,
            SortColumn::DnsServers => SortColumn::Status,
            SortColumn::Status => SortColumn::Interface,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            SortColumn::Interface => SortColumn::Status,
            SortColumn::Type => SortColumn::Interface,
            SortColumn::IpAddress => SortColumn::Type,
            SortColumn::MacAddress => SortColumn::IpAddress,
            SortColumn::SubnetMask => SortColumn::MacAddress,
            SortColumn::DnsServers => SortColumn::SubnetMask,
            SortColumn::Status => SortColumn::DnsServers,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpConfigMode {
    DHCP,
    Static,
}

impl IpConfigMode {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            IpConfigMode::DHCP => "DHCP",
            IpConfigMode::Static => "Static",
        }
    }
}
