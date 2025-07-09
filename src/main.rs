use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    Terminal,
};
use sysinfo::System;

mod build_info;
mod helpers;
mod process;
mod ui;

use ui::{draw_dashboard, draw_help_window, AppState};

/// Application configuration constants
const REFRESH_INTERVAL_MS: u64 = 1000;
const EVENT_POLL_TIMEOUT_MS: u64 = 100;

/// Main application entry point
///
/// Initializes the terminal, runs the main application loop,
/// and ensures proper cleanup on exit
fn main() -> Result<(), io::Error> {
    print_build_info();

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the main application
    let result = run_application(&mut terminal);

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Print build information to stdout
fn print_build_info() {
    println!("Project: {}", build_info::PROJECT_NAME);
    println!("Developer: {}", build_info::DEVELOPER);
    println!("Version: {}", build_info::VERSION);
    println!("Build time: {}", build_info::BUILD_TIME);
    println!("Project started: {}", build_info::PROJECT_START);
    println!("Development years: {}", build_info::DEVELOPMENT_YEARS);
    println!("Origin: {}", build_info::PROJECT_ORIGIN);
}

/// Main application loop
///
/// Handles terminal rendering, event processing, and system updates
fn run_application(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    let mut system = System::new_all();
    let mut last_update = Instant::now();
    let mut app_state = AppState { show_help: false };

    loop {
        // Render the current state
        terminal.draw(|frame| {
            let size = frame.size();
            let outer_block = ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .style(Style::default().bg(Color::Black));

            frame.render_widget(outer_block, size);

            let inner_area = Rect {
                x: size.x + 1,
                y: size.y + 1,
                width: size.width - 2,
                height: size.height - 2,
            };

            if app_state.show_help {
                draw_help_window(frame, inner_area);
            } else {
                draw_dashboard(frame, &system, inner_area);
            }
        })?;

        // Handle user input
        if event::poll(Duration::from_millis(EVENT_POLL_TIMEOUT_MS))? {
            if let Event::Key(key) = event::read()? {
                handle_key_event(&mut app_state, key.code);

                // Exit if 'q' was pressed
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Update system information periodically
        if !app_state.show_help
            && last_update.elapsed() > Duration::from_millis(REFRESH_INTERVAL_MS)
        {
            system.refresh_all();
            last_update = Instant::now();
        }
    }

    Ok(())
}

/// Handle keyboard events and update application state
///
/// * `app_state` - Current application state to modify
/// * `key_code` - The key code that was pressed
fn handle_key_event(app_state: &mut AppState, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('q') => {
            // Exit handled in main loop
        }
        KeyCode::F(1) => {
            app_state.show_help = true;
        }
        _ => {
            // Any other key closes help window if it's open
            if app_state.show_help {
                app_state.show_help = false;
            }
        }
    }
}
