use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph, Tabs, Sparkline, Axis, Chart, Dataset},
    Frame,
};
use super::app::{App, AppScreen};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.size();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    draw_header(frame, app, main_layout[0]);
    draw_body(frame, app, main_layout[1]);
    draw_footer(frame, app, main_layout[2]);
}

fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let profile_icon = app.current_profile.icon();
    let profile_name = app.current_profile.as_str();
    let title = format!(" {} SpeedCool Linux v1.0.0  |  Profile: {} {}  |  {}  |  {}", 
        profile_icon, profile_icon, profile_name.to_uppercase(), app.distro_name(),
        if app.on_ac() { "\u{26a1} AC" } else { format!("\u{1f50b} Battery: {:.0}%", app.battery_pct()) }
    );
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));
    let paragraph = Paragraph::new(Line::from(Span::styled(&title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD))))
        .block(block);
    frame.render_widget(paragraph, area);
}

fn draw_body(frame: &mut Frame, app: &App, area: Rect) {
    match app.screen {
        AppScreen::Main => draw_main(frame, app, area),
        AppScreen::Cpu => draw_cpu(frame, app, area),
        AppScreen::Memory => draw_memory(frame, app, area),
        AppScreen::Gpu => draw_gpu(frame, app, area),
        AppScreen::Disk => draw_disk(frame, app, area),
        AppScreen::Thermal => draw_thermal(frame, app, area),
        AppScreen::Battery => draw_battery(frame, app, area),
    }
}

fn draw_main(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(6),
        ])
        .horizontal_margin(2)
        .vertical_margin(1)
        .split(area);

    draw_cpu_block(frame, app, chunks[0]);
    draw_memory_block(frame, app, chunks[1]);
    draw_gpu_block(frame, app, chunks[2]);
    draw_thermal_block(frame, app, chunks[3]);
}

fn draw_cpu_block(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" CPU ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(3)])
        .split(inner);

    let usage_text = format!("Usage: {:.1}%  |  Freq: {} MHz  |  Temp: {:.1}°C  |  Governor: {}  |  Turbo: {}",
        app.cpu_info.usage,
        app.cpu_info.frequencies.first().unwrap_or(&0) / 1000,
        app.cpu_info.temp,
        app.cpu_info.governor,
        if app.cpu_info.turbo_enabled { "\u{2705}" } else { "\u{274c}" }
    );
    frame.render_widget(Paragraph::new(usage_text).style(Style::default().fg(Color::White)), chunks[0]);

    let load_text = format!("Load: {:>6.1}%  |  Cores: {}  |  Min: {} MHz  |  Max: {} MHz",
        app.cpu_info.usage, app.cpu_info.cores,
        app.cpu_info.min_freq / 1000, app.cpu_info.max_freq / 1000
    );
    frame.render_widget(Paragraph::new(load_text), chunks[1]);

    let gauge = Gauge::default()
        .block(Block::default().title(" Load ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(if app.cpu_info.usage > 80.0 { Color::Red } else if app.cpu_info.usage > 50.0 { Color::Yellow } else { Color::Green }))
        .percent(app.cpu_info.usage as u16);
    frame.render_widget(gauge, chunks[2]);

    if app.cpu_history.len() > 1 {
        let max = app.cpu_history.iter().cloned().fold(0.0_f64, f64::max).max(1.0);
        let data: Vec<(f64, f64)> = app.cpu_history.iter().enumerate().map(|(i, v)| (i as f64, *v)).collect();
        let dataset = Dataset::default()
            .marker(ratatui::symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&data);
        let chart = Chart::new(vec![dataset])
            .block(Block::default().title(" History ").borders(Borders::ALL))
            .x_axis(Axis::default().bounds([0.0, 60.0]))
            .y_axis(Axis::default().bounds([0.0, 100.0]));
        frame.render_widget(chart, chunks[3]);
    }
}

fn draw_memory_block(frame: &mut Frame, app: &App, area: Rect) {
    let pct = app.mem_pct();
    let block = Block::default()
        .title(format!(" Memory ({:.1} GB / {:.1} GB)", app.mem_used_gb(), app.mem_total_gb()))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Blue));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(if pct > 80.0 { Color::Red } else if pct > 60.0 { Color::Yellow } else { Color::Blue }))
        .percent(pct as u16)
        .label(format!("{:.1}%", pct));
    frame.render_widget(gauge, inner);

    let info_text = format!("Available: {:.1} GB  |  Swap: {:.1}/{:.1} GB",
        app.mem_info.available_kb as f64 / (1024.0 * 1024.0),
        (app.mem_info.swap_total_kb - app.mem_info.swap_free_kb) as f64 / (1024.0 * 1024.0),
        app.mem_info.swap_total_kb as f64 / (1024.0 * 1024.0)
    );
    frame.render_widget(Paragraph::new(info_text), Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(1)]).margin(1).split(inner)[0]);
}

fn draw_gpu_block(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(format!(" GPU ({})", app.gpu_info.vendor))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Magenta));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = format!("{}  |  Temp: {:.1}°C  |  Usage: {:.1}%  |  VRAM: {}/{} MB  |  Clock: {} MHz",
        app.gpu_info.model,
        app.gpu_info.temp_c,
        app.gpu_info.usage_pct,
        app.gpu_info.memory_used_mb,
        app.gpu_info.memory_total_mb,
        app.gpu_info.core_clock_mhz
    );
    frame.render_widget(Paragraph::new(text), inner);
}

fn draw_thermal_block(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Thermal Zones ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Red));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut text = String::new();
    for zone in &app.thermal_zones {
        text.push_str(&format!("{}: {:.1}°C  ", zone.kind, zone.temp_c));
    }
    frame.render_widget(Paragraph::new(text), inner);
}

fn draw_cpu(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" CPU Details ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Green));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = vec![
        ListItem::new(format!("Model:     {}", app.cpu_info.model)),
        ListItem::new(format!("Cores:     {}", app.cpu_info.cores)),
        ListItem::new(format!("Governor:  {}", app.cpu_info.governor)),
        ListItem::new(format!("Freq:      {} MHz", app.cpu_info.frequencies.first().unwrap_or(&0) / 1000)),
        ListItem::new(format!("Min:       {} MHz", app.cpu_info.min_freq / 1000)),
        ListItem::new(format!("Max:       {} MHz", app.cpu_info.max_freq / 1000)),
        ListItem::new(format!("Temp:      {:.1}°C", app.cpu_info.temp)),
        ListItem::new(format!("Usage:     {:.1}%", app.cpu_info.usage)),
        ListItem::new(format!("Turbo:     {}", if app.cpu_info.turbo_enabled { "Enabled" } else { "Disabled" })),
        ListItem::new(format!("Avail Gov: {}", app.cpu_info.available_governors.join(", "))),
    ];
    frame.render_widget(List::new(items).highlight_style(Style::default().fg(Color::Yellow)), inner);
}

fn draw_memory(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Memory Details ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Blue));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = vec![
        ListItem::new(format!("Total:     {:.1} GB", app.mem_total_gb())),
        ListItem::new(format!("Available: {:.1} GB", app.mem_info.available_kb as f64 / (1024.0 * 1024.0))),
        ListItem::new(format!("Used:      {:.1}%", app.mem_pct())),
        ListItem::new(format!("Swap:      {:.1}/{:.1} GB",
            (app.mem_info.swap_total_kb - app.mem_info.swap_free_kb) as f64 / (1024.0 * 1024.0),
            app.mem_info.swap_total_kb as f64 / (1024.0 * 1024.0))),
        ListItem::new(format!("Cached:    {:.1} MB", app.mem_info.cached_kb as f64 / 1024.0)),
    ];
    frame.render_widget(List::new(items).highlight_style(Style::default().fg(Color::Yellow)), inner);
}

fn draw_gpu(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" GPU Details ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Magenta));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = vec![
        ListItem::new(format!("Vendor:  {}", app.gpu_info.vendor)),
        ListItem::new(format!("Model:   {}", app.gpu_info.model)),
        ListItem::new(format!("Temp:    {:.1}°C", app.gpu_info.temp_c)),
        ListItem::new(format!("Usage:   {:.1}%", app.gpu_info.usage_pct)),
        ListItem::new(format!("Memory:  {}/{} MB", app.gpu_info.memory_used_mb, app.gpu_info.memory_total_mb)),
        ListItem::new(format!("Core:    {} MHz", app.gpu_info.core_clock_mhz)),
        ListItem::new(format!("MemClk:  {} MHz", app.gpu_info.memory_clock_mhz)),
        ListItem::new(format!("Power:   {:.1} W", app.gpu_info.power_watts)),
    ];
    frame.render_widget(List::new(items).highlight_style(Style::default().fg(Color::Yellow)), inner);
}

fn draw_disk(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Disks ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = app.disks.iter().map(|d| {
        ListItem::new(format!("{} | Model: {} | Scheduler: {} | Size: {:.0} GB",
            d.name, d.model, d.scheduler, d.size_sectors as f64 * 512.0 / 1_000_000_000.0))
    }).collect();
    frame.render_widget(List::new(items).highlight_style(Style::default().fg(Color::Yellow)), inner);
}

fn draw_thermal(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Thermal Zones ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Red));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = app.thermal_zones.iter().map(|z| {
        let temp_color = if z.temp_c > 80.0 { "CRIT" } else if z.temp_c > 60.0 { "WARN" } else { "OK" };
        ListItem::new(format!("Zone {} | Type: {} | Temp: {:.1}°C [{}]", z.zone, z.kind, z.temp_c, temp_color))
    }).collect();
    frame.render_widget(List::new(items).highlight_style(Style::default().fg(Color::Yellow)), inner);
}

fn draw_battery(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Battery ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = vec![
        ListItem::new(format!("Present:    {}", if app.bat_info.present { "Yes" } else { "No" })),
        ListItem::new(format!("Capacity:   {:.0}%", app.bat_info.capacity)),
        ListItem::new(format!("Status:     {}", app.bat_info.status)),
        ListItem::new(format!("Health:     {}", app.bat_info.health)),
        ListItem::new(format!("Cycles:     {}", app.bat_info.cycle_count)),
        ListItem::new(format!("Voltage:    {:.2} V", app.bat_info.voltage_now as f64 / 1_000_000.0)),
        ListItem::new(format!("Power:      {:.2} W", app.bat_info.power_now)),
    ];
    frame.render_widget(List::new(items).highlight_style(Style::default().fg(Color::Yellow)), inner);
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let cmd_keys = match app.screen {
        AppScreen::Main => " [q] Quit  [1-7] Details  [e] Eco  [b] Balanced  [p] Performance ",
        _ => " [Esc] Back  [q] Quit ",
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::DarkGray));
    let paragraph = Paragraph::new(Line::from(Span::styled(cmd_keys, Style::default().fg(Color::Gray))))
        .block(block);
    frame.render_widget(paragraph, area);
}
