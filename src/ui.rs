use crate::app::{App, AppMode};
use crate::models::IpConfigMode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    match app.mode {
        AppMode::Help => draw_help_screen(f, app),
        AppMode::Details => draw_details_screen(f, app),
        AppMode::EditIp => draw_edit_ip_screen(f, app),
        AppMode::EditDns => draw_edit_dns_screen(f, app),
        AppMode::EditIpv6 => draw_edit_ipv6_screen(f, app),
        AppMode::ConfirmDialog => draw_confirm_dialog(f, app),
        AppMode::Terminal => draw_terminal_screen(f, app),
        _ => draw_main_screen(f, app),
    }
}

fn draw_main_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status/Search bar
            Constraint::Length(2), // Help bar
        ])
        .split(f.area());

    draw_title(f, chunks[0]);
    draw_table(f, chunks[1], app);
    draw_status_bar(f, chunks[2], app);
    draw_help_bar(f, chunks[3], app);
}

fn draw_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("NetUtil - Network Interface Manager")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, area);
}

fn draw_table(f: &mut Frame, area: Rect, app: &App) {
    let header_cells = [
        "Interface",
        "Type",
        "IP Address",
        "MAC Address",
        "Subnet Mask",
        "DNS Servers",
        "Status",
    ]
    .iter()
    .enumerate()
    .map(|(i, h)| {
        let mut style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);

        let col_matches = match i {
            0 => app.sort_column.as_str() == "Interface",
            1 => app.sort_column.as_str() == "Type",
            2 => app.sort_column.as_str() == "IP Address",
            3 => app.sort_column.as_str() == "MAC Address",
            4 => app.sort_column.as_str() == "Subnet Mask",
            5 => app.sort_column.as_str() == "DNS Servers",
            6 => app.sort_column.as_str() == "Status",
            _ => false,
        };

        if col_matches {
            style = style.fg(Color::Green);
        }

        let sort_indicator = if col_matches {
            if app.sort_ascending {
                " ▲"
            } else {
                " ▼"
            }
        } else {
            ""
        };

        Cell::from(format!("{}{}", h, sort_indicator)).style(style)
    });

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app
        .filtered_rows
        .iter()
        .skip(app.scroll_offset)
        .take(app.page_size)
        .enumerate()
        .map(|(i, &row_idx)| {
            let row = &app.table_rows[row_idx];
            let is_selected = i + app.scroll_offset == app.selected_index;

            let style = if is_selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let cells = vec![
                Cell::from(row.name.clone()),
                Cell::from(row.interface_type.clone()),
                Cell::from(row.ip_address.clone()),
                Cell::from(row.mac_address.clone()),
                Cell::from(row.subnet_mask.clone()),
                Cell::from(row.dns_servers.clone()),
                Cell::from(row.status.clone()),
            ];

            Row::new(cells).style(style).height(1)
        });

    let widths = [
        Constraint::Percentage(12),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(18),
        Constraint::Percentage(15),
        Constraint::Percentage(22),
        Constraint::Percentage(8),
    ];

    let title = if app.filtered_rows.is_empty() {
        format!(" Network Interfaces (0/0) - No matches ")
    } else {
        format!(
            " Network Interfaces ({}/{}) - Page {}/{} - Press 'i' for details ",
            app.selected_index + 1,
            app.filtered_rows.len(),
            app.scroll_offset / app.page_size + 1,
            (app.filtered_rows.len() + app.page_size - 1) / app.page_size
        )
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_symbol(">> ");

    f.render_widget(table, area);
}

fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let text = match app.mode {
        AppMode::Search => {
            format!("Search: {} (Esc to cancel)", app.search_query)
        }
        _ => {
            if let Some(ref msg) = app.status_message {
                msg.clone()
            } else {
                format!(
                    "Total: {} | Filtered: {} | Sort: {} {}",
                    app.table_rows.len(),
                    app.filtered_rows.len(),
                    app.sort_column.as_str(),
                    if app.sort_ascending { "▲" } else { "▼" }
                )
            }
        }
    };

    let style = match app.mode {
        AppMode::Search => Style::default().fg(Color::Yellow),
        _ => Style::default(),
    };

    let paragraph = Paragraph::new(text)
        .style(style)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(paragraph, area);
}

fn draw_help_bar(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.mode {
        AppMode::Normal => {
            "q:Quit | ?:Help | /:Search | i:Details | x:Terminal | Ctrl+f:FlushDNS | r:Refresh | e:IP | d:DNS | 6:IPv6"
        }
        AppMode::Search => "Type to search | Esc:Cancel | Enter:Done",
        AppMode::Terminal => "Enter:Execute | ↑↓:Scroll | Ctrl+l:Clear | Esc:Back",
        _ => "Esc:Cancel | Tab:Next field",
    };

    let paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_details_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(f.area());

    let title = Paragraph::new("Interface Details")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    if let Some(iface) = app.get_selected_interface() {
        let mut lines = vec![
            Line::from(vec![
                Span::styled(
                    "Interface: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&iface.name),
            ]),
            Line::from(vec![
                Span::styled(
                    "Type: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(iface.interface_type.as_str()),
            ]),
            Line::from(vec![
                Span::styled(
                    "Status: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    if iface.is_up { "UP" } else { "DOWN" },
                    if iface.is_up {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Red)
                    },
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "MAC Address: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(iface.mac_address.as_deref().unwrap_or("N/A")),
            ]),
        ];

        // Show SSID for WiFi interfaces
        if let Some(ref ssid) = iface.ssid {
            lines.push(Line::from(vec![
                Span::styled(
                    "WiFi SSID: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(ssid, Style::default().fg(Color::Cyan)),
            ]));
        }

        if let Some(mtu) = iface.mtu {
            lines.push(Line::from(vec![
                Span::styled(
                    "MTU: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(mtu.to_string()),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "IP Addresses:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]));

        for (i, addr) in iface.ip_addresses.iter().enumerate() {
            let ip_type = if addr.is_ipv6() { "IPv6" } else { "IPv4" };
            let ip_str = addr.ip.to_string();
            lines.push(Line::from(vec![
                Span::raw(format!("  {}. {} ", i + 1, ip_type)),
                Span::styled(ip_str, Style::default().fg(Color::Green)),
            ]));

            if let Some(ref netmask) = addr.netmask {
                lines.push(Line::from(vec![
                    Span::raw("     Netmask: "),
                    Span::raw(netmask.to_string()),
                ]));
            }

            if let Some(ref broadcast) = addr.broadcast {
                lines.push(Line::from(vec![
                    Span::raw("     Broadcast: "),
                    Span::raw(broadcast.to_string()),
                ]));
            }
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "DNS Configuration:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]));

        for (i, server) in app.dns_config.nameservers.iter().enumerate() {
            lines.push(Line::from(format!("  {}. {}", i + 1, server)));
        }

        if !app.dns_config.search_domains.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                "Search Domains:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            for domain in &app.dns_config.search_domains {
                lines.push(Line::from(format!("  - {}", domain)));
            }
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false });

        f.render_widget(paragraph, chunks[1]);
    }

    let help = Paragraph::new("Press Esc or q to return | Press e/d/6 to edit configuration")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_edit_ip_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(f.area());

    let title = Paragraph::new("Edit IP Configuration")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    if let Some(iface) = app.get_selected_interface() {
        let mode_str = format!(
            "[{}] DHCP  [{}] Static",
            if matches!(app.ip_edit_state.mode, IpConfigMode::DHCP) {
                "X"
            } else {
                " "
            },
            if matches!(app.ip_edit_state.mode, IpConfigMode::Static) {
                "X"
            } else {
                " "
            }
        );

        let mut items = vec![
            ListItem::new(Line::from(vec![
                Span::styled("Interface: ", Style::default().fg(Color::Yellow)),
                Span::raw(&iface.name),
            ])),
            ListItem::new(Line::from("")),
            ListItem::new(Line::from(vec![
                Span::styled("Configuration Mode: ", Style::default().fg(Color::Yellow)),
                Span::raw(&mode_str),
            ])),
        ];

        if matches!(app.ip_edit_state.mode, IpConfigMode::Static) {
            items.push(ListItem::new(Line::from("")));
            items.push(ListItem::new(Line::from(vec![
                Span::styled("IP Address: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    &app.ip_edit_state.ip_buffer,
                    if app.ip_edit_state.current_field == 1 {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    },
                ),
            ])));

            items.push(ListItem::new(Line::from(vec![
                Span::styled("Netmask: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    &app.ip_edit_state.netmask_buffer,
                    if app.ip_edit_state.current_field == 2 {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    },
                ),
            ])));

            items.push(ListItem::new(Line::from(vec![
                Span::styled("Gateway (optional): ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    if app.ip_edit_state.gateway_buffer.is_empty() {
                        "None"
                    } else {
                        &app.ip_edit_state.gateway_buffer
                    },
                    if app.ip_edit_state.current_field == 3 {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    },
                ),
            ])));
        }

        let list = List::new(items).block(Block::default().borders(Borders::ALL));

        f.render_widget(list, chunks[1]);
    }

    let help = Paragraph::new("Tab:Next field | Space:Toggle mode | Enter:Apply | Esc:Cancel")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_edit_dns_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(f.area());

    let title = Paragraph::new("Edit DNS Configuration")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let mut items = vec![ListItem::new(Line::from(vec![Span::styled(
        "DNS Servers:",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]))];

    for (i, server) in app.dns_edit_state.dns_servers.iter().enumerate() {
        let is_selected =
            app.dns_edit_state.current_field == 0 && app.dns_edit_state.server_index == i;
        items.push(ListItem::new(Line::from(vec![
            Span::raw(format!("  {}. ", i + 1)),
            Span::styled(
                if server.is_empty() { "<empty>" } else { server },
                if is_selected {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                },
            ),
        ])));
    }

    items.push(ListItem::new(Line::from("")));
    items.push(ListItem::new(Line::from(vec![Span::styled(
        "Search Domains:",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )])));

    if app.dns_edit_state.search_domains.is_empty() {
        items.push(ListItem::new(Line::from("  (none)")));
    } else {
        for (i, domain) in app.dns_edit_state.search_domains.iter().enumerate() {
            let is_selected =
                app.dns_edit_state.current_field == 1 && app.dns_edit_state.domain_index == i;
            items.push(ListItem::new(Line::from(vec![
                Span::raw(format!("  {}. ", i + 1)),
                Span::styled(
                    domain,
                    if is_selected {
                        Style::default().bg(Color::DarkGray).fg(Color::White)
                    } else {
                        Style::default()
                    },
                ),
            ])));
        }
    }

    let list = List::new(items).block(Block::default().borders(Borders::ALL));

    f.render_widget(list, chunks[1]);

    let help = Paragraph::new("↑↓:Navigate | Tab:Switch section | Enter:Edit | a:Add | x:Delete | Esc:Cancel | Ctrl+s:Save")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_edit_ipv6_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(f.area());

    let title = Paragraph::new("Edit IPv6 Configuration")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    if let Some(iface) = app.get_selected_interface() {
        let enabled_str = format!(
            "[{}] Enabled  [{}] Disabled",
            if app.ipv6_edit_state.enabled {
                "X"
            } else {
                " "
            },
            if !app.ipv6_edit_state.enabled {
                "X"
            } else {
                " "
            }
        );

        let mut items = vec![
            ListItem::new(Line::from(vec![
                Span::styled("Interface: ", Style::default().fg(Color::Yellow)),
                Span::raw(&iface.name),
            ])),
            ListItem::new(Line::from("")),
            ListItem::new(Line::from(vec![
                Span::styled("IPv6 Status: ", Style::default().fg(Color::Yellow)),
                Span::raw(&enabled_str),
            ])),
        ];

        if app.ipv6_edit_state.enabled {
            items.push(ListItem::new(Line::from("")));
            items.push(ListItem::new(Line::from(vec![Span::styled(
                "IPv6 Address (optional for static): ",
                Style::default().fg(Color::Yellow),
            )])));
            items.push(ListItem::new(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    if app.ipv6_edit_state.ip_buffer.is_empty() {
                        "Auto (SLAAC)"
                    } else {
                        &app.ipv6_edit_state.ip_buffer
                    },
                    if app.ipv6_edit_state.current_field == 1 {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    },
                ),
            ])));

            items.push(ListItem::new(Line::from(vec![
                Span::styled("Prefix Length: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    &app.ipv6_edit_state.prefix_buffer,
                    if app.ipv6_edit_state.current_field == 2 {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    },
                ),
            ])));
        }

        let list = List::new(items).block(Block::default().borders(Borders::ALL));

        f.render_widget(list, chunks[1]);
    }

    let help = Paragraph::new("Tab:Next field | Space:Toggle | Enter:Apply | Esc:Cancel")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_confirm_dialog(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 40, f.area());

    let block = Block::default()
        .title(" Confirm Action ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner);

    let message = Paragraph::new(app.confirm_message.clone())
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(message, chunks[0]);

    let buttons = Paragraph::new("Press Enter to confirm | Press Esc to cancel")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);

    f.render_widget(buttons, chunks[1]);
}

fn draw_help_screen(f: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(f.area());

    let title = Paragraph::new("NetUtil - Help & Keyboard Shortcuts")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let help_text = vec![
        Line::from(vec![Span::styled(
            "Navigation:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  ↑/k         - Move up"),
        Line::from("  ↓/j         - Move down"),
        Line::from("  PgUp/Ctrl+u - Previous page"),
        Line::from("  PgDn/Ctrl+d - Next page"),
        Line::from("  Home/g      - First item"),
        Line::from("  End/G       - Last item"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Viewing:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  i           - Show detailed interface info"),
        Line::from("  r           - Refresh data"),
        Line::from("  s/S         - Cycle sort column (forward/backward)"),
        Line::from("  /           - Search/filter"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Configuration (requires sudo):",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  e           - Edit IP (DHCP/Static)"),
        Line::from("  d           - Edit DNS servers"),
        Line::from("  6           - Edit IPv6 settings"),
        Line::from("  t           - Toggle interface up/down"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Clipboard:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  c           - Copy interface name"),
        Line::from("  Ctrl+i      - Copy IP address"),
        Line::from("  Ctrl+m      - Copy MAC address"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Tools:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  x           - Open terminal"),
        Line::from("                Execute commands (ping, traceroute, etc.)"),
        Line::from("                Use ↑↓ to scroll, Ctrl+l to clear"),
        Line::from("  Ctrl+f      - Flush DNS cache (requires sudo)"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Other:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  ?           - Show this help"),
        Line::from("  q           - Quit"),
        Line::from(""),
        Line::from(vec![Span::styled("Note:", Style::default().fg(Color::Red))]),
        Line::from("  Network configuration changes require sudo privileges."),
        Line::from("  You'll be prompted for your password when needed."),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, chunks[1]);

    let footer = Paragraph::new("Press Esc or q to return")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

fn draw_terminal_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(f.area());

    let title = Paragraph::new("Terminal")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Output area
    let title_text = if app.terminal_output.is_empty() {
        " Output (empty) ".to_string()
    } else {
        format!(
            " Output ({}/{}) ",
            (app.terminal_scroll + 1).min(app.terminal_output.len()),
            app.terminal_output.len()
        )
    };

    let output_lines: Vec<Line> = if app.terminal_output.is_empty() {
        vec![]
    } else {
        app.terminal_output
            .iter()
            .skip(app.terminal_scroll)
            .take(chunks[1].height.saturating_sub(2) as usize)
            .map(|line| Line::from(line.clone()))
            .collect()
    };

    let output = Paragraph::new(output_lines)
        .block(Block::default().borders(Borders::ALL).title(title_text))
        .wrap(Wrap { trim: false });

    f.render_widget(output, chunks[1]);

    // Command input
    let input = Paragraph::new(format!("> {}", app.terminal_command))
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title(" Command "));
    f.render_widget(input, chunks[2]);

    // Help bar
    let help = Paragraph::new(
        "Enter:Execute | ↑↓:Scroll | Ctrl+l:Clear | Esc:Back | Type command and press Enter",
    )
    .style(Style::default().fg(Color::DarkGray))
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[3]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
