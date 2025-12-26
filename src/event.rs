use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn handle_events(app: &mut crate::app::App) -> anyhow::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            handle_key_event(app, key)?;
        }
    }
    Ok(())
}

fn handle_key_event(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    use crate::app::AppMode;

    match app.mode {
        AppMode::Normal => handle_normal_mode(app, key)?,
        AppMode::Search => handle_search_mode(app, key)?,
        AppMode::EditIp => handle_edit_ip_mode(app, key)?,
        AppMode::EditDns => handle_edit_dns_mode(app, key)?,
        AppMode::EditIpv6 => handle_edit_ipv6_mode(app, key)?,
        AppMode::Details => handle_details_mode(app, key)?,
        AppMode::Help => handle_help_mode(app, key)?,
        AppMode::ConfirmDialog => handle_confirm_mode(app, key)?,
        AppMode::Terminal => handle_terminal_mode(app, key)?,
    }

    Ok(())
}

fn handle_normal_mode(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
        }

        // Navigation
        KeyCode::Down | KeyCode::Char('j') => {
            app.next_item();
            app.clear_status();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous_item();
            app.clear_status();
        }
        KeyCode::PageDown | KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.next_page();
            app.clear_status();
        }
        KeyCode::PageUp | KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.previous_page();
            app.clear_status();
        }
        KeyCode::Home | KeyCode::Char('g') => {
            app.selected_index = 0;
            app.scroll_offset = 0;
            app.clear_status();
        }
        KeyCode::End | KeyCode::Char('G') => {
            if !app.filtered_rows.is_empty() {
                app.selected_index = app.filtered_rows.len() - 1;
                app.adjust_scroll();
            }
            app.clear_status();
        }

        // Sorting
        KeyCode::Char('s') => {
            app.sort_column = app.sort_column.next();
            app.apply_sort();
            app.set_status(format!("Sorted by {}", app.sort_column.as_str()));
        }
        KeyCode::Char('S') => {
            app.sort_column = app.sort_column.prev();
            app.apply_sort();
            app.set_status(format!("Sorted by {}", app.sort_column.as_str()));
        }

        // Search
        KeyCode::Char('/') => {
            app.mode = crate::app::AppMode::Search;
            app.clear_status();
        }
        KeyCode::Esc => {
            app.clear_search();
            app.clear_status();
        }

        // Refresh
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.refresh_data()?;
            app.set_status("Data refreshed".to_string());
        }

        // Help
        KeyCode::Char('?') => {
            app.mode = crate::app::AppMode::Help;
        }

        // Clipboard operations
        KeyCode::Char('c') => {
            if let Err(e) = app.copy_selected_field("Interface") {
                app.set_status(format!("Failed to copy: {}", e));
            }
        }
        KeyCode::Char('i') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.copy_selected_field("IP Address") {
                app.set_status(format!("Failed to copy: {}", e));
            }
        }

        // Details
        KeyCode::Char('i') | KeyCode::Char('I') => {
            app.show_details();
        }
        KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.copy_selected_field("MAC Address") {
                app.set_status(format!("Failed to copy: {}", e));
            }
        }

        // Edit operations
        KeyCode::Char('e') => {
            app.start_edit_ip();
        }
        KeyCode::Char('d') => {
            app.start_edit_dns();
        }
        KeyCode::Char('6') => {
            app.start_edit_ipv6();
        }

        // Toggle interface
        KeyCode::Char('t') => {
            if let Err(e) = app.toggle_interface() {
                app.set_status(format!("Failed to toggle interface: {}", e));
            }
        }

        // Terminal
        KeyCode::Char('x') => {
            app.open_terminal();
        }

        // Flush DNS cache
        KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.flush_dns_cache() {
                app.set_status(format!("Failed to flush DNS cache: {}", e));
            }
        }

        _ => {}
    }

    Ok(())
}

fn handle_search_mode(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.mode = crate::app::AppMode::Normal;
        }
        KeyCode::Enter => {
            app.mode = crate::app::AppMode::Normal;
        }
        KeyCode::Backspace => {
            app.remove_search_char();
        }
        KeyCode::Char(c) => {
            app.add_search_char(c);
        }
        _ => {}
    }

    Ok(())
}

fn handle_edit_ip_mode(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    use crate::models::IpConfigMode;

    match key.code {
        KeyCode::Esc => {
            app.mode = crate::app::AppMode::Normal;
        }
        KeyCode::Tab => {
            if app.ip_edit_state.current_field == 0 {
                app.ip_edit_state.current_field = 1;
            } else if matches!(app.ip_edit_state.mode, IpConfigMode::Static) {
                app.ip_edit_state.current_field = (app.ip_edit_state.current_field + 1).min(3);
            }
        }
        KeyCode::BackTab => {
            if app.ip_edit_state.current_field > 0 {
                app.ip_edit_state.current_field -= 1;
            }
        }
        KeyCode::Char(' ') if app.ip_edit_state.current_field == 0 => {
            app.ip_edit_state.mode = match app.ip_edit_state.mode {
                IpConfigMode::DHCP => IpConfigMode::Static,
                IpConfigMode::Static => IpConfigMode::DHCP,
            };
        }
        KeyCode::Enter => match app.ip_edit_state.mode {
            IpConfigMode::DHCP => app.prepare_dhcp_config(),
            IpConfigMode::Static => app.prepare_static_ip_config(),
        },
        KeyCode::Backspace => match app.ip_edit_state.current_field {
            1 => {
                app.ip_edit_state.ip_buffer.pop();
            }
            2 => {
                app.ip_edit_state.netmask_buffer.pop();
            }
            3 => {
                app.ip_edit_state.gateway_buffer.pop();
            }
            _ => {}
        },
        KeyCode::Char(c) if app.ip_edit_state.current_field > 0 => {
            match app.ip_edit_state.current_field {
                1 => app.ip_edit_state.ip_buffer.push(c),
                2 => app.ip_edit_state.netmask_buffer.push(c),
                3 => app.ip_edit_state.gateway_buffer.push(c),
                _ => {}
            }
        }
        _ => {}
    }

    Ok(())
}

fn handle_edit_dns_mode(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.mode = crate::app::AppMode::Normal;
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.prepare_dns_config();
        }
        KeyCode::Tab => {
            app.dns_edit_state.current_field = (app.dns_edit_state.current_field + 1) % 2;
            app.dns_edit_state.server_index = 0;
            app.dns_edit_state.domain_index = 0;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.dns_edit_state.current_field == 0 {
                if app.dns_edit_state.server_index > 0 {
                    app.dns_edit_state.server_index -= 1;
                }
            } else if app.dns_edit_state.domain_index > 0 {
                app.dns_edit_state.domain_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.dns_edit_state.current_field == 0 {
                if app.dns_edit_state.server_index
                    < app.dns_edit_state.dns_servers.len().saturating_sub(1)
                {
                    app.dns_edit_state.server_index += 1;
                }
            } else if app.dns_edit_state.domain_index
                < app.dns_edit_state.search_domains.len().saturating_sub(1)
            {
                app.dns_edit_state.domain_index += 1;
            }
        }
        KeyCode::Char('a') => {
            if app.dns_edit_state.current_field == 0 {
                app.dns_edit_state.dns_servers.push(String::new());
                app.dns_edit_state.server_index = app.dns_edit_state.dns_servers.len() - 1;
            } else {
                app.dns_edit_state.search_domains.push(String::new());
                app.dns_edit_state.domain_index = app.dns_edit_state.search_domains.len() - 1;
            }
        }
        KeyCode::Char('x') => {
            if app.dns_edit_state.current_field == 0 {
                if !app.dns_edit_state.dns_servers.is_empty() {
                    app.dns_edit_state
                        .dns_servers
                        .remove(app.dns_edit_state.server_index);
                    if app.dns_edit_state.server_index >= app.dns_edit_state.dns_servers.len()
                        && app.dns_edit_state.server_index > 0
                    {
                        app.dns_edit_state.server_index -= 1;
                    }
                }
            } else if !app.dns_edit_state.search_domains.is_empty() {
                app.dns_edit_state
                    .search_domains
                    .remove(app.dns_edit_state.domain_index);
                if app.dns_edit_state.domain_index >= app.dns_edit_state.search_domains.len()
                    && app.dns_edit_state.domain_index > 0
                {
                    app.dns_edit_state.domain_index -= 1;
                }
            }
        }
        KeyCode::Enter => {
            // Start editing the selected field - for now we'll open a simple input
            // In a full implementation, you'd want a proper text input dialog
        }
        KeyCode::Backspace => {
            if app.dns_edit_state.current_field == 0 {
                if let Some(server) = app
                    .dns_edit_state
                    .dns_servers
                    .get_mut(app.dns_edit_state.server_index)
                {
                    server.pop();
                }
            } else if let Some(domain) = app
                .dns_edit_state
                .search_domains
                .get_mut(app.dns_edit_state.domain_index)
            {
                domain.pop();
            }
        }
        KeyCode::Char(c) => {
            if app.dns_edit_state.current_field == 0 {
                if let Some(server) = app
                    .dns_edit_state
                    .dns_servers
                    .get_mut(app.dns_edit_state.server_index)
                {
                    server.push(c);
                }
            } else if let Some(domain) = app
                .dns_edit_state
                .search_domains
                .get_mut(app.dns_edit_state.domain_index)
            {
                domain.push(c);
            }
        }
        _ => {}
    }

    Ok(())
}

fn handle_edit_ipv6_mode(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.mode = crate::app::AppMode::Normal;
        }
        KeyCode::Tab => {
            if app.ipv6_edit_state.enabled {
                app.ipv6_edit_state.current_field = (app.ipv6_edit_state.current_field + 1) % 3;
            }
        }
        KeyCode::BackTab => {
            if app.ipv6_edit_state.current_field > 0 {
                app.ipv6_edit_state.current_field -= 1;
            }
        }
        KeyCode::Char(' ') if app.ipv6_edit_state.current_field == 0 => {
            app.ipv6_edit_state.enabled = !app.ipv6_edit_state.enabled;
        }
        KeyCode::Enter => {
            app.prepare_ipv6_config();
        }
        KeyCode::Backspace => match app.ipv6_edit_state.current_field {
            1 => {
                app.ipv6_edit_state.ip_buffer.pop();
            }
            2 => {
                app.ipv6_edit_state.prefix_buffer.pop();
            }
            _ => {}
        },
        KeyCode::Char(c) if app.ipv6_edit_state.current_field > 0 => {
            match app.ipv6_edit_state.current_field {
                1 => app.ipv6_edit_state.ip_buffer.push(c),
                2 => app.ipv6_edit_state.prefix_buffer.push(c),
                _ => {}
            }
        }
        _ => {}
    }

    Ok(())
}

fn handle_details_mode(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.mode = crate::app::AppMode::Normal;
        }
        KeyCode::Char('e') => {
            app.start_edit_ip();
        }
        KeyCode::Char('d') => {
            app.start_edit_dns();
        }
        KeyCode::Char('6') => {
            app.start_edit_ipv6();
        }
        _ => {}
    }

    Ok(())
}

fn handle_help_mode(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
            app.mode = crate::app::AppMode::Normal;
        }
        _ => {}
    }

    Ok(())
}

fn handle_confirm_mode(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Enter => {
            if let Err(e) = app.execute_confirmed_action() {
                app.set_status(format!("Error: {}", e));
                app.cancel_confirm();
            }
        }
        KeyCode::Esc => {
            app.cancel_confirm();
        }
        _ => {}
    }

    Ok(())
}

fn handle_terminal_mode(app: &mut crate::app::App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.mode = crate::app::AppMode::Normal;
        }
        KeyCode::Enter => {
            if let Err(e) = app.execute_terminal_command() {
                app.terminal_output.push(format!("Error: {}", e));
            }
        }
        KeyCode::Up => {
            app.terminal_scroll_up();
        }
        KeyCode::Down => {
            app.terminal_scroll_down();
        }
        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.clear_terminal();
        }
        KeyCode::Backspace => {
            app.remove_terminal_char();
        }
        KeyCode::Char(c) => {
            app.add_terminal_char(c);
        }
        _ => {}
    }

    Ok(())
}
