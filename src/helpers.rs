use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Calculate a centered rectangle within the given area
///
/// # Arguments
/// * `percent_x` - Width percentage of the centered rectangle
/// * `percent_y` - Height percentage of the centered rectangle  
/// * `area` - The parent rectangle to center within
///
/// # Returns
/// A centered rectangle with the specified dimensions
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let vertical = popup_layout[1];
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical);

    horizontal[1]
}

/// Format bytes into human-readable string with appropriate units
///
/// # Arguments
/// * `bytes` - Number of bytes to format
///
/// # Returns
/// Formatted string with unit (KB, MB, GB, TB)
pub fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let bytes = bytes as f64;

    if bytes >= TB {
        format!("{:.1}TB", bytes / TB)
    } else if bytes >= GB {
        format!("{:.1}GB", bytes / GB)
    } else if bytes >= MB {
        format!("{:.1}MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.1}KB", bytes / KB)
    } else {
        format!("{:.0}KB", bytes)
    }
}

/// Format uptime duration into human-readable string
///
/// # Arguments
/// * `uptime_seconds` - Uptime in seconds
///
/// # Returns
/// Formatted uptime string (e.g., "2 days, 03:45:12" or "03:45:12")
pub fn format_uptime(uptime_seconds: u64) -> String {
    const SECONDS_PER_DAY: u64 = 86400;
    const SECONDS_PER_HOUR: u64 = 3600;
    const SECONDS_PER_MINUTE: u64 = 60;

    let days = uptime_seconds / SECONDS_PER_DAY;
    let hours = (uptime_seconds % SECONDS_PER_DAY) / SECONDS_PER_HOUR;
    let minutes = (uptime_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let seconds = uptime_seconds % SECONDS_PER_MINUTE;

    if days > 0 {
        format!("{} days, {:02}:{:02}:{:02}", days, hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

/// Format process runtime into MM:SS.ss format
///
/// # Arguments
/// * `runtime_seconds` - Process runtime in seconds
///
/// # Returns
/// Formatted runtime string (e.g., "02:30.45")
pub fn format_runtime(runtime_seconds: u64) -> String {
    const SECONDS_PER_HOUR: u64 = 3600;
    const SECONDS_PER_MINUTE: u64 = 60;

    let hours = runtime_seconds / SECONDS_PER_HOUR;
    let minutes = (runtime_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let seconds = runtime_seconds % SECONDS_PER_MINUTE;

    format!("{:02}:{:02}.{:02}", hours, minutes, seconds)
}
