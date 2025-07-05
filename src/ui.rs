use once_cell::sync::Lazy;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use std::collections::HashMap;
use sysinfo::System;
use chrono::{self, Datelike};

use crate::helpers::{centered_rect, format_bytes, format_runtime, format_uptime};
use crate::process::{
    fetch_memory_map, fetch_priority_map, get_process_memory, get_process_priority,
};

// Constants for UI layout and styling
const CPU_COLUMNS: usize = 4;
const MIN_BAR_LENGTH: usize = 4;
const MIN_MEMORY_BAR_LENGTH: usize = 10;
const LABEL_WIDTH: usize = 5;
const INFO_PADDING: &str = "  ";

// Color thresholds for CPU usage
const CPU_HIGH_THRESHOLD: f32 = 80.0;
const CPU_MEDIUM_THRESHOLD: f32 = 50.0;

// Color thresholds for memory usage
const MEMORY_HIGH_THRESHOLD: f64 = 0.8;
const MEMORY_MEDIUM_THRESHOLD: f64 = 0.5;

// Color thresholds for process CPU/MEM usage
const PROCESS_HIGH_THRESHOLD: f32 = 50.0;
const PROCESS_MEDIUM_THRESHOLD: f32 = 20.0;

/// Application state for UI rendering
pub struct AppState {
    pub show_help: bool,
}

/// Draw the help window overlay
pub fn draw_help_window(f: &mut Frame, area: Rect) {
    let help_area = centered_rect(60, 20, area);
    let padding = "    ";

    let help_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw(padding),
            Span::styled(
                "Sysly - macOS System Monitor Experiment",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(padding),
        ]),
        Line::from(vec![
            Span::raw(padding),
            Span::styled(
                format!("Version {} - Conceived Jul 1, 2019", crate::build_info::VERSION),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(padding),
        ]),
        Line::from(vec![
            Span::raw(padding),
            Span::styled(
                format!("(C) 2019-{} Thinh Nguyen", chrono::Utc::now().year()),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(padding),
        ]),
        Line::from(vec![
            Span::raw(padding),
            Span::styled(
                crate::build_info::PROJECT_INSPIRED,
                Style::default().fg(Color::Magenta),
            ),
            Span::raw(padding),
        ]),
        Line::from(vec![
            Span::raw(padding),
            Span::raw(padding),
            Span::styled(
                "Released under the Apache License 2.0.",
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(padding),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw(padding),
            Span::styled(
                "Press any key to return.",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(padding),
        ]),
        Line::from(""),
    ];

    let help_block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let help_paragraph = Paragraph::new(help_lines)
        .block(help_block)
        .alignment(Alignment::Left);

    f.render_widget(help_paragraph, help_area);
}

/// Draw the main dashboard layout
pub fn draw_dashboard(f: &mut Frame, sys: &System, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Info bar
            Constraint::Min(10),   // Process table
        ])
        .split(area);

    draw_info_bar(sys, f, layout[0]);
    draw_process_table(sys, f, layout[1]);
}

/// Draw the information bar with CPU, memory, and system info
pub fn draw_info_bar(sys: &System, f: &mut Frame, area: Rect) {
    let cpus = sys.cpus();
    let cpu_count = cpus.len();
    let cpu_rows = (cpu_count + CPU_COLUMNS - 1) / CPU_COLUMNS;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(cpu_rows as u16), // CPU bars
            Constraint::Min(3),                  // Memory bars + info
        ])
        .split(area);

    draw_cpu_bars(cpus, f, layout[0]);
    draw_memory_and_info(sys, f, layout[1]);
}

/// Draw CPU usage bars in a grid layout
fn draw_cpu_bars(
    cpus: &[sysinfo::Cpu],
    f: &mut Frame,
    area: Rect,
) {
    let cpu_count = cpus.len();
    let cpu_rows = (cpu_count + CPU_COLUMNS - 1) / CPU_COLUMNS;
    let total_padding = (CPU_COLUMNS - 1) * 3;
    let label_length = 4;
    let percent_length = 6;
    let bracket_length = 2;

    let bar_length = ((area.width as usize - total_padding) / CPU_COLUMNS)
        .saturating_sub(label_length + percent_length + bracket_length)
        .max(MIN_BAR_LENGTH);

    let mut cpu_lines = Vec::new();

    for row in 0..cpu_rows {
        let mut spans = Vec::new();

        for col in 0..CPU_COLUMNS {
            let cpu_index = row + col * cpu_rows;

            if cpu_index < cpus.len() {
                let cpu = &cpus[cpu_index];
                let usage = cpu.cpu_usage();
                let used_bars = ((usage / 100.0) * bar_length as f32).round() as usize;

                let bar = create_progress_bar(used_bars, bar_length);
                let color = get_cpu_color(usage);
                let label = format!("{:>2}   ", cpu_index);

                spans.extend_from_slice(&[
                    Span::styled(label, Style::default().fg(Color::Cyan)),
                    Span::raw("["),
                    Span::styled(bar, Style::default().fg(color)),
                    Span::raw("] "),
                    Span::styled(format!("{:>5.1}%", usage), Style::default().fg(Color::Gray)),
                ]);
            } else {
                let empty_space =
                    " ".repeat(LABEL_WIDTH + 1 + bar_length + bracket_length + percent_length);
                spans.push(Span::raw(empty_space));
            }

            if col < CPU_COLUMNS - 1 {
                spans.push(Span::raw("   "));
            }
        }

        cpu_lines.push(Line::from(spans));
    }

    let cpu_paragraph = Paragraph::new(cpu_lines).alignment(Alignment::Left);
    f.render_widget(cpu_paragraph, area);
}

/// Draw memory bars and system information
fn draw_memory_and_info(
    sys: &System,
    f: &mut Frame,
    area: Rect,
) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Memory bars
            Constraint::Percentage(50), // System info
        ])
        .split(area);

    draw_memory_bars(sys, f, layout[0]);
    draw_system_info(sys, f, layout[1]);
}

/// Draw memory and swap usage bars
fn draw_memory_bars(sys: &System, f: &mut Frame, area: Rect) {
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    let bar_length = area.width.saturating_sub(LABEL_WIDTH as u16 + 3) as usize;
    let bar_length = bar_length.max(MIN_MEMORY_BAR_LENGTH);

    let memory_line = create_memory_bar("Mem", used_memory, total_memory, bar_length, LABEL_WIDTH);

    let swap_line = create_memory_bar("Swp", used_swap, total_swap, bar_length, LABEL_WIDTH);

    let memory_paragraph = Paragraph::new(vec![memory_line, swap_line]);
    f.render_widget(memory_paragraph, area);
}

/// Draw system information panel
fn draw_system_info(sys: &System, f: &mut Frame, area: Rect) {
    let processes = sys.processes();
    let task_count = processes.len();
    let running_count = processes
        .values()
        .filter(|p| p.status().to_string() == "Running")
        .count();

    let tasks_info = format!(
        "Tasks: {}, N/A thr, 0 kthr; {} running",
        task_count, running_count
    );

    let load_avg = sysinfo::System::load_average();
    let load_info = format!(
        "Load average: {:.2} {:.2} {:.2}",
        load_avg.one, load_avg.five, load_avg.fifteen
    );

    let uptime = sysinfo::System::uptime();
    let uptime_info = format!("Uptime: {}", format_uptime(uptime));

    let info_lines = vec![
        Line::from(vec![
            Span::raw(INFO_PADDING),
            Span::styled(tasks_info, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw(INFO_PADDING),
            Span::styled(load_info, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw(INFO_PADDING),
            Span::styled(
                uptime_info,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let info_paragraph = Paragraph::new(info_lines).alignment(Alignment::Left);
    f.render_widget(info_paragraph, area);
}

/// Draw the process table
pub fn draw_process_table(
    sys: &System,
    f: &mut Frame,
    area: Rect,
) {
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| {
        b.cpu_usage()
            .partial_cmp(&a.cpu_usage())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let header = create_table_header();
    let total_memory = sys.total_memory() as f64;

    static UID_TO_USER: Lazy<HashMap<u32, String>> = Lazy::new(|| unsafe {
        users::all_users()
            .map(|u| (u.uid(), u.name().to_string_lossy().to_string()))
            .collect()
    });

    let priority_map = fetch_priority_map();
    let memory_map = fetch_memory_map();

    let rows = processes.iter().enumerate().map(|(index, process)| {
        create_process_row(
            index,
            process,
            &UID_TO_USER,
            &priority_map,
            &memory_map,
            total_memory,
        )
    });

    let table = Table::new(rows, get_table_constraints())
        .header(header)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM))
        .column_spacing(1);

    f.render_widget(table, area);
}

// Helper functions

fn create_progress_bar(used: usize, total: usize) -> String {
    (0..total)
        .map(|i| if i < used { '|' } else { ' ' })
        .collect()
}

fn get_cpu_color(usage: f32) -> Color {
    match usage {
        u if u > CPU_HIGH_THRESHOLD => Color::Red,
        u if u > CPU_MEDIUM_THRESHOLD => Color::Yellow,
        _ => Color::Green,
    }
}

fn get_memory_color(used: u64, total: u64) -> Color {
    if total == 0 {
        return Color::Green;
    }

    let ratio = used as f64 / total as f64;
    match ratio {
        r if r > MEMORY_HIGH_THRESHOLD => Color::Red,
        r if r > MEMORY_MEDIUM_THRESHOLD => Color::Yellow,
        _ => Color::Green,
    }
}

fn create_memory_bar(
    label: &str,
    used: u64,
    total: u64,
    bar_length: usize,
    label_width: usize,
) -> Line {
    let label_text = format!("{}/{}", format_bytes(used), format_bytes(total));
    let used_bars = if total > 0 {
        ((used as f64 / total as f64) * bar_length as f64).round() as usize
    } else {
        0
    };

    let mut bar = create_progress_bar(used_bars, bar_length);

    // Overlay label inside the bar
    let label_start = if bar_length > label_text.len() {
        bar_length - label_text.len()
    } else {
        0
    };

    for (i, ch) in label_text.chars().enumerate() {
        if label_start + i < bar.len() {
            bar.replace_range(label_start + i..label_start + i + 1, &ch.to_string());
        }
    }

    let color = get_memory_color(used, total);

    Line::from(vec![
        Span::styled(
            format!("{:<width$}", label, width = label_width),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("["),
        Span::styled(bar, Style::default().fg(color)),
        Span::raw("]"),
    ])
}

fn create_table_header() -> Row<'static> {
    Row::new([
        Cell::from("PID").bold(),
        Cell::from("USER").bold(),
        Cell::from("PRI").bold(),
        Cell::from("NI").bold(),
        Cell::from("VIRT").bold(),
        Cell::from("RES").bold(),
        Cell::from("S").bold(),
        Cell::from("CPU% ").bold(),
        Cell::from("MEM% ").bold(),
        Cell::from("TIME+").bold(),
        Cell::from("Command").bold(),
    ])
}

fn get_table_constraints() -> [Constraint; 11] {
    [
        Constraint::Length(7),  // PID
        Constraint::Length(12), // USER
        Constraint::Length(5),  // PRI
        Constraint::Length(4),  // NI
        Constraint::Length(8),  // VIRT
        Constraint::Length(8),  // RES
        Constraint::Length(2),  // S
        Constraint::Length(6),  // CPU%
        Constraint::Length(6),  // MEM%
        Constraint::Length(8),  // TIME+
        Constraint::Min(10),    // Command
    ]
}

fn create_process_row<'a>(
    index: usize,
    process: &'a sysinfo::Process,
    uid_to_user: &'a HashMap<u32, String>,
    priority_map: &'a HashMap<u32, crate::process::ProcessPriority>,
    memory_map: &'a HashMap<u32, crate::process::ProcessMemory>,
    total_memory: f64,
) -> Row<'a> {
    let pid = process.pid().as_u32();
    let user = process
        .user_id()
        .and_then(|uid| uid_to_user.get(&uid))
        .cloned()
        .unwrap_or_else(|| "?".to_string());

    let priority_info = get_process_priority(pid, priority_map);
    let memory_info = get_process_memory(
        pid,
        memory_map,
        process.virtual_memory() / 1024,
        process.memory() / 1024,
    );

    let status = get_process_status(process);
    let cpu_usage = process.cpu_usage();
    let memory_usage = if total_memory > 0.0 {
        (process.memory() as f64 / total_memory) * 100.0
    } else {
        0.0
    };
    let runtime = format_runtime(process.run_time());
    let command = process.cmd().join(" ");

    let cells = vec![
        Cell::from(pid.to_string()).style(Style::default().fg(Color::White)),
        Cell::from(user).style(Style::default().fg(Color::Cyan)),
        Cell::from(priority_info.priority).style(Style::default().fg(Color::White)),
        Cell::from(priority_info.nice).style(Style::default().fg(Color::White)),
        Cell::from(format_bytes(memory_info.virtual_memory))
            .style(Style::default().fg(Color::Green)),
        Cell::from(format_bytes(memory_info.resident_memory))
            .style(Style::default().fg(Color::Green)),
        Cell::from(status.clone()).style(get_status_color(&status)),
        Cell::from(format!("{:.1}", cpu_usage)).style(get_usage_color(cpu_usage)),
        Cell::from(format!("{:.1}", memory_usage)).style(get_usage_color(memory_usage as f32)),
        Cell::from(runtime).style(Style::default().fg(Color::White)),
        Cell::from(command).style(Style::default().fg(Color::Cyan)),
    ];

    let mut row = Row::new(cells);

    // Alternating row colors
    if index % 2 == 1 {
        row = row.style(Style::default().bg(Color::Black));
    } else {
        row = row.style(Style::default().bg(Color::Rgb(30, 30, 30)));
    }

    // Selected row highlighting (currently always 0)
    if index == 0 {
        row = row.style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::REVERSED),
        );
    }

    row
}

fn get_process_status(process: &sysinfo::Process) -> String {
    match process.status().to_string().as_str() {
        "Running" => "R".to_string(),
        "Sleeping" => "S".to_string(),
        "Zombie" => "Z".to_string(),
        status => status.chars().next().unwrap_or('?').to_string(),
    }
}

fn get_status_color(status: &str) -> Style {
    match status {
        "R" => Style::default().fg(Color::Yellow),
        "S" => Style::default().fg(Color::Green),
        "Z" => Style::default().fg(Color::Red),
        _ => Style::default().fg(Color::Gray),
    }
}

fn get_usage_color(usage: f32) -> Style {
    match usage {
        u if u > PROCESS_HIGH_THRESHOLD => Style::default().fg(Color::Red),
        u if u > PROCESS_MEDIUM_THRESHOLD => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::White),
    }
}
