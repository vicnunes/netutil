use crate::models::{
    DnsConfiguration, InterfaceTableRow, IpConfigMode, NetworkInterface, SortColumn,
};
use crate::network;
use crate::sudo;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Search,
    EditIp,
    EditDns,
    EditIpv6,
    Details,
    Help,
    ConfirmDialog,
    Terminal,
}

#[derive(Debug, Clone)]
pub struct IpEditState {
    pub mode: IpConfigMode,
    pub ip_buffer: String,
    pub netmask_buffer: String,
    pub gateway_buffer: String,
    pub current_field: usize, // 0=mode, 1=ip, 2=netmask, 3=gateway
}

#[derive(Debug, Clone)]
pub struct DnsEditState {
    pub dns_servers: Vec<String>,
    pub search_domains: Vec<String>,
    pub current_field: usize, // 0=servers, 1=domains
    pub server_index: usize,
    pub domain_index: usize,
    pub edit_buffer: String,
}

#[derive(Debug, Clone)]
pub struct Ipv6EditState {
    pub enabled: bool,
    pub ip_buffer: String,
    pub prefix_buffer: String,
    pub current_field: usize, // 0=enabled, 1=ip, 2=prefix
}

pub struct App {
    pub interfaces: Vec<NetworkInterface>,
    pub dns_config: DnsConfiguration,
    pub table_rows: Vec<InterfaceTableRow>,
    pub filtered_rows: Vec<usize>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub mode: AppMode,
    pub search_query: String,
    pub sort_column: SortColumn,
    pub sort_ascending: bool,
    pub page_size: usize,
    pub should_quit: bool,
    pub status_message: Option<String>,

    // Edit states
    pub ip_edit_state: IpEditState,
    pub dns_edit_state: DnsEditState,
    pub ipv6_edit_state: Ipv6EditState,

    // Confirmation dialog
    pub confirm_message: String,
    pub confirm_action: Option<ConfirmAction>,

    // Terminal
    pub terminal_command: String,
    pub terminal_output: Vec<String>,
    pub terminal_scroll: usize,
    pub terminal_needs_clear: bool,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    SetDhcp(String),
    SetStaticIp(String, String, String, Option<String>),
    SetDns(Vec<String>, Vec<String>),
    ToggleInterface(String, bool),
    DisableIpv6(String),
    EnableIpv6(String),
    SetStaticIpv6(String, String, u8),
}

impl App {
    pub fn new() -> Result<Self> {
        let interfaces = network::get_network_interfaces()?;
        let dns_config = network::get_dns_configuration()?;
        let table_rows = Self::create_table_rows(&interfaces, &dns_config);
        let filtered_rows: Vec<usize> = (0..table_rows.len()).collect();

        Ok(Self {
            interfaces,
            dns_config,
            table_rows,
            filtered_rows,
            selected_index: 0,
            scroll_offset: 0,
            mode: AppMode::Normal,
            search_query: String::new(),
            sort_column: SortColumn::Interface,
            sort_ascending: true,
            page_size: 20,
            should_quit: false,
            status_message: None,

            ip_edit_state: IpEditState {
                mode: IpConfigMode::DHCP,
                ip_buffer: String::new(),
                netmask_buffer: String::new(),
                gateway_buffer: String::new(),
                current_field: 0,
            },

            dns_edit_state: DnsEditState {
                dns_servers: Vec::new(),
                search_domains: Vec::new(),
                current_field: 0,
                server_index: 0,
                domain_index: 0,
                edit_buffer: String::new(),
            },

            ipv6_edit_state: Ipv6EditState {
                enabled: true,
                ip_buffer: String::new(),
                prefix_buffer: String::from("64"),
                current_field: 0,
            },

            confirm_message: String::new(),
            confirm_action: None,

            terminal_command: String::new(),
            terminal_output: Vec::new(),
            terminal_scroll: 0,
            terminal_needs_clear: false,
        })
    }

    fn create_table_rows(
        interfaces: &[NetworkInterface],
        dns_config: &DnsConfiguration,
    ) -> Vec<InterfaceTableRow> {
        interfaces
            .iter()
            .map(|iface| InterfaceTableRow::from_interface(iface, dns_config))
            .collect()
    }

    pub fn refresh_data(&mut self) -> Result<()> {
        self.interfaces = network::get_network_interfaces()?;
        self.dns_config = network::get_dns_configuration()?;
        self.table_rows = Self::create_table_rows(&self.interfaces, &self.dns_config);
        self.apply_filter();
        self.apply_sort();
        Ok(())
    }

    pub fn apply_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_rows = (0..self.table_rows.len()).collect();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_rows = self
                .table_rows
                .iter()
                .enumerate()
                .filter(|(_, row)| {
                    row.name.to_lowercase().contains(&query)
                        || row.ip_address.to_lowercase().contains(&query)
                        || row.mac_address.to_lowercase().contains(&query)
                        || row.interface_type.to_lowercase().contains(&query)
                })
                .map(|(idx, _)| idx)
                .collect();
        }

        if self.selected_index >= self.filtered_rows.len() && !self.filtered_rows.is_empty() {
            self.selected_index = self.filtered_rows.len() - 1;
        }
    }

    pub fn apply_sort(&mut self) {
        let column = self.sort_column;
        let ascending = self.sort_ascending;

        self.filtered_rows.sort_by(|&a, &b| {
            let row_a = &self.table_rows[a];
            let row_b = &self.table_rows[b];

            let cmp = match column {
                SortColumn::Interface => row_a.name.cmp(&row_b.name),
                SortColumn::Type => row_a.interface_type.cmp(&row_b.interface_type),
                SortColumn::IpAddress => row_a.ip_address.cmp(&row_b.ip_address),
                SortColumn::MacAddress => row_a.mac_address.cmp(&row_b.mac_address),
                SortColumn::SubnetMask => row_a.subnet_mask.cmp(&row_b.subnet_mask),
                SortColumn::DnsServers => row_a.dns_servers.cmp(&row_b.dns_servers),
                SortColumn::Status => row_a.status.cmp(&row_b.status),
            };

            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
    }

    pub fn next_item(&mut self) {
        if !self.filtered_rows.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.filtered_rows.len();
            self.adjust_scroll();
        }
    }

    pub fn previous_item(&mut self) {
        if !self.filtered_rows.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.filtered_rows.len() - 1;
            } else {
                self.selected_index -= 1;
            }
            self.adjust_scroll();
        }
    }

    pub fn next_page(&mut self) {
        if !self.filtered_rows.is_empty() {
            self.selected_index =
                (self.selected_index + self.page_size).min(self.filtered_rows.len() - 1);
            self.adjust_scroll();
        }
    }

    pub fn previous_page(&mut self) {
        if self.selected_index >= self.page_size {
            self.selected_index -= self.page_size;
        } else {
            self.selected_index = 0;
        }
        self.adjust_scroll();
    }

    pub fn adjust_scroll(&mut self) {
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + self.page_size {
            self.scroll_offset = self.selected_index - self.page_size + 1;
        }
    }

    #[allow(dead_code)]
    pub fn set_sort_column(&mut self, column: SortColumn) {
        if self.sort_column == column {
            self.sort_ascending = !self.sort_ascending;
        } else {
            self.sort_column = column;
            self.sort_ascending = true;
        }
        self.apply_sort();
    }

    pub fn get_selected_row(&self) -> Option<&InterfaceTableRow> {
        self.filtered_rows
            .get(self.selected_index)
            .and_then(|&idx| self.table_rows.get(idx))
    }

    pub fn get_selected_interface(&self) -> Option<&NetworkInterface> {
        self.get_selected_row()
            .and_then(|row| self.interfaces.iter().find(|iface| iface.name == row.name))
    }

    pub fn add_search_char(&mut self, c: char) {
        self.search_query.push(c);
        self.apply_filter();
    }

    pub fn remove_search_char(&mut self) {
        self.search_query.pop();
        self.apply_filter();
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.apply_filter();
    }

    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    // IP Edit functions
    pub fn start_edit_ip(&mut self) {
        if let Some(iface) = self.get_selected_interface() {
            let ip_buffer = iface
                .ip_addresses
                .iter()
                .find(|addr| !addr.is_ipv6())
                .map(|addr| addr.ip.to_string())
                .unwrap_or_default();

            let netmask_buffer = iface
                .ip_addresses
                .iter()
                .find(|addr| !addr.is_ipv6())
                .and_then(|addr| addr.netmask.as_ref())
                .map(|mask| mask.to_string())
                .unwrap_or_else(|| "255.255.255.0".to_string());

            self.ip_edit_state.mode = IpConfigMode::DHCP;
            self.ip_edit_state.ip_buffer = ip_buffer;
            self.ip_edit_state.netmask_buffer = netmask_buffer;
            self.ip_edit_state.gateway_buffer.clear();
            self.ip_edit_state.current_field = 0;
            self.mode = AppMode::EditIp;
        }
    }

    pub fn start_edit_dns(&mut self) {
        self.dns_edit_state.dns_servers = self
            .dns_config
            .nameservers
            .iter()
            .map(|ip| ip.to_string())
            .collect();

        if self.dns_edit_state.dns_servers.is_empty() {
            self.dns_edit_state.dns_servers.push(String::new());
        }

        self.dns_edit_state.search_domains = self.dns_config.search_domains.clone();
        self.dns_edit_state.current_field = 0;
        self.dns_edit_state.server_index = 0;
        self.dns_edit_state.domain_index = 0;
        self.dns_edit_state.edit_buffer.clear();
        self.mode = AppMode::EditDns;
    }

    pub fn start_edit_ipv6(&mut self) {
        if let Some(iface) = self.get_selected_interface() {
            let enabled = iface.ipv6_enabled;
            let ip_buffer = iface
                .ip_addresses
                .iter()
                .find(|addr| addr.is_ipv6())
                .map(|addr| addr.ip.to_string())
                .unwrap_or_default();

            self.ipv6_edit_state.enabled = enabled;
            self.ipv6_edit_state.ip_buffer = ip_buffer;
            self.ipv6_edit_state.prefix_buffer = String::from("64");
            self.ipv6_edit_state.current_field = 0;
            self.mode = AppMode::EditIpv6;
        }
    }

    pub fn show_details(&mut self) {
        self.mode = AppMode::Details;
    }

    pub fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        let mut clipboard = arboard::Clipboard::new()?;
        clipboard.set_text(text)?;
        Ok(())
    }

    pub fn copy_selected_field(&mut self, field: &str) -> Result<()> {
        if let Some(row) = self.get_selected_row() {
            let text = row.get_field(field).to_string();
            self.copy_to_clipboard(&text)?;
            self.set_status(format!("Copied {} to clipboard", field));
        }
        Ok(())
    }

    pub fn prepare_dhcp_config(&mut self) {
        if let Some(iface) = self.get_selected_interface() {
            let name = iface.name.clone();
            self.confirm_message = format!(
                "Set interface '{}' to use DHCP?\nThis will remove any static IP configuration.",
                name
            );
            self.confirm_action = Some(ConfirmAction::SetDhcp(name));
            self.mode = AppMode::ConfirmDialog;
        }
    }

    pub fn prepare_static_ip_config(&mut self) {
        if let Some(iface) = self.get_selected_interface() {
            let name = iface.name.clone();
            let ip = self.ip_edit_state.ip_buffer.clone();
            let netmask = self.ip_edit_state.netmask_buffer.clone();
            let gateway = if self.ip_edit_state.gateway_buffer.is_empty() {
                None
            } else {
                Some(self.ip_edit_state.gateway_buffer.clone())
            };

            self.confirm_message = format!(
                "Set static IP on '{}'?\nIP: {}\nNetmask: {}\nGateway: {}",
                name,
                ip,
                netmask,
                gateway.as_deref().unwrap_or("None")
            );

            self.confirm_action = Some(ConfirmAction::SetStaticIp(name, ip, netmask, gateway));

            self.mode = AppMode::ConfirmDialog;
        }
    }

    pub fn prepare_dns_config(&mut self) {
        let servers: Vec<String> = self
            .dns_edit_state
            .dns_servers
            .iter()
            .filter(|s| !s.is_empty())
            .cloned()
            .collect();

        let domains = self.dns_edit_state.search_domains.clone();

        self.confirm_message = format!(
            "Update DNS configuration?\nServers: {}\nSearch domains: {}",
            servers.join(", "),
            if domains.is_empty() {
                "None".to_string()
            } else {
                domains.join(", ")
            }
        );

        self.confirm_action = Some(ConfirmAction::SetDns(servers, domains));
        self.mode = AppMode::ConfirmDialog;
    }

    pub fn prepare_ipv6_config(&mut self) {
        if let Some(iface) = self.get_selected_interface() {
            let name = iface.name.clone();

            if !self.ipv6_edit_state.enabled {
                self.confirm_message = format!("Disable IPv6 on '{}'?", name);
                self.confirm_action = Some(ConfirmAction::DisableIpv6(name));
            } else if !self.ipv6_edit_state.ip_buffer.is_empty() {
                let prefix: u8 = self.ipv6_edit_state.prefix_buffer.parse().unwrap_or(64);
                let ip_buffer = self.ipv6_edit_state.ip_buffer.clone();
                self.confirm_message = format!(
                    "Set static IPv6 on '{}'?\nAddress: {}/{}",
                    name, ip_buffer, prefix
                );
                self.confirm_action = Some(ConfirmAction::SetStaticIpv6(name, ip_buffer, prefix));
            } else {
                self.confirm_message = format!("Enable IPv6 on '{}'?", name);
                self.confirm_action = Some(ConfirmAction::EnableIpv6(name));
            }

            self.mode = AppMode::ConfirmDialog;
        }
    }

    pub fn execute_confirmed_action(&mut self) -> Result<()> {
        if let Some(action) = self.confirm_action.take() {
            let result = match action {
                ConfirmAction::SetDhcp(iface) => {
                    sudo::set_dhcp(&iface)?;
                    format!("DHCP enabled on {}", iface)
                }
                ConfirmAction::SetStaticIp(iface, ip, netmask, gateway) => {
                    sudo::set_static_ip(&iface, &ip, &netmask, gateway.as_deref())?;
                    format!("Static IP set on {}", iface)
                }
                ConfirmAction::SetDns(servers, domains) => {
                    let server_refs: Vec<&str> = servers.iter().map(|s| s.as_str()).collect();
                    sudo::set_dns_servers("", &server_refs)?;

                    if !domains.is_empty() {
                        let domain_refs: Vec<&str> = domains.iter().map(|s| s.as_str()).collect();
                        sudo::set_search_domains("", &domain_refs)?;
                    }
                    "DNS configuration updated".to_string()
                }
                ConfirmAction::ToggleInterface(iface, enabled) => {
                    sudo::set_interface_status(&iface, enabled)?;
                    format!(
                        "Interface {} {}",
                        iface,
                        if enabled { "enabled" } else { "disabled" }
                    )
                }
                ConfirmAction::DisableIpv6(iface) => {
                    sudo::disable_ipv6(&iface)?;
                    format!("IPv6 disabled on {}", iface)
                }
                ConfirmAction::EnableIpv6(iface) => {
                    sudo::enable_ipv6(&iface)?;
                    format!("IPv6 enabled on {}", iface)
                }
                ConfirmAction::SetStaticIpv6(iface, ip, prefix) => {
                    sudo::set_static_ipv6(&iface, &ip, prefix)?;
                    format!("Static IPv6 set on {}", iface)
                }
            };

            self.set_status(result);
            self.mode = AppMode::Normal;
            self.refresh_data()?;
        }

        Ok(())
    }

    pub fn cancel_confirm(&mut self) {
        self.confirm_action = None;
        self.mode = AppMode::Normal;
    }

    pub fn toggle_interface(&mut self) -> Result<()> {
        if let Some(iface) = self.get_selected_interface() {
            let name = iface.name.clone();
            let new_status = !iface.is_up;
            self.confirm_message = format!(
                "{} interface '{}'?",
                if new_status { "Enable" } else { "Disable" },
                name
            );
            self.confirm_action = Some(ConfirmAction::ToggleInterface(name, new_status));
            self.mode = AppMode::ConfirmDialog;
        }
        Ok(())
    }

    // Terminal functions
    pub fn open_terminal(&mut self) {
        self.mode = AppMode::Terminal;
        self.terminal_command.clear();
        self.terminal_output.clear();
        self.terminal_scroll = 0;
    }

    pub fn add_terminal_char(&mut self, c: char) {
        self.terminal_command.push(c);
    }

    pub fn remove_terminal_char(&mut self) {
        self.terminal_command.pop();
    }

    pub fn execute_terminal_command(&mut self) -> Result<()> {
        if self.terminal_command.is_empty() {
            return Ok(());
        }

        let cmd = self.terminal_command.clone();
        self.terminal_output.push(format!("$ {}", cmd));

        // Parse command and arguments
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command = parts[0];
        let args = &parts[1..];

        // Execute the command
        let result = std::process::Command::new(command).args(args).output();

        match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                for line in stdout.lines() {
                    self.terminal_output.push(line.to_string());
                }

                for line in stderr.lines() {
                    self.terminal_output.push(format!("ERROR: {}", line));
                }

                if !output.status.success() {
                    self.terminal_output.push(format!(
                        "Command exited with status: {}",
                        output.status.code().unwrap_or(-1)
                    ));
                }
            }
            Err(e) => {
                self.terminal_output
                    .push(format!("Failed to execute command: {}", e));
            }
        }

        self.terminal_command.clear();

        // Auto-scroll to bottom
        if self.terminal_output.len() > 20 {
            self.terminal_scroll = self.terminal_output.len().saturating_sub(20);
        }

        Ok(())
    }

    pub fn terminal_scroll_up(&mut self) {
        if self.terminal_scroll > 0 {
            self.terminal_scroll -= 1;
        }
    }

    pub fn terminal_scroll_down(&mut self) {
        let max_scroll = self.terminal_output.len().saturating_sub(20);
        if self.terminal_scroll < max_scroll {
            self.terminal_scroll += 1;
        }
    }

    pub fn clear_terminal(&mut self) {
        self.terminal_output.clear();
        self.terminal_command.clear();
        self.terminal_scroll = 0;
        self.terminal_needs_clear = true;
    }

    pub fn flush_dns_cache(&mut self) -> Result<()> {
        sudo::flush_dns_cache()?;
        self.set_status("DNS cache flushed successfully".to_string());
        Ok(())
    }
}
