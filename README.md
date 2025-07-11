<a href="https://buymeacoffee.com/x45qN8k" target="_blank">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" height="50" style="border-radius:8px" />
</a>

# Sysly – A macOS system monitor mini project written in Rust, inspired by htop's interface.

Sysly is inspired by htop, but built from scratch in Rust using the sysinfo library for system data. It is a macOS-focused system monitoring dashboard experiment, conceived on **July, 2019**, at the airport while waiting for a flight back to hometown.

![Sysly Dashboard](https://img.shields.io/badge/Version-1.1.0-blue)
![Rust](https://img.shields.io/badge/Rust-2021+-orange)
![License](https://img.shields.io/badge/License-Apache%202.0-green)
![Development](https://img.shields.io/badge/Development-2019--present-yellow)
![macOS](https://img.shields.io/badge/macOS-Optimized-green)

---

<p align="center">
  <img src="./screenshots/Sysly_ Screenshot.png" alt="Sysly Demo" width="700" />
  <br/>
  <em>Sysly – A macOS system monitor mini project written in Rust, inspired by htop's interface</em>
</p>

---

## Features

- **Inspired by htop, but written in Rust for modern performance and safety**
- **Uses sysinfo for cross-platform system information**
- Real-time CPU monitoring with per-core usage bars
- Memory and swap usage visualization with color-coded indicators
- Process management with detailed information (PID, USER, PRI, NI, VIRT, RES, CPU%, MEM%, TIME+)
- System information including uptime, load average, and task statistics
- Cross-platform support with macOS-optimized process data
- Interactive help system accessible via F1
- Responsive UI that adapts to terminal size
- Professional codebase following Rust best practices

## Requirements

- Rust 1.70+ (2021 edition)
- macOS, Linux, or Windows
- Terminal with UTF-8 support

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/hungtrungthinh/sysly.git
cd sysly

# Build and run
cargo run --release
```

### Build for Distribution

```bash
cargo build --release
```

The binary will be available at `target/release/sysly`.

## Usage

```bash
# Run the application
./sysly

# Available controls:
# F1    - Show/hide help window
# q     - Quit application
# Any key - Close help window when open
```

## Architecture

Sysly is built with a modular architecture:

- **`main.rs`** - Application entry point and main loop
- **`ui.rs`** - Terminal UI rendering and layout management
- **`process.rs`** - Process information gathering and macOS-specific optimizations
- **`helpers.rs`** - Utility functions for formatting and calculations
- **`build_info.rs`** - Build-time metadata (auto-generated)

## Development

### Project Origin & Timeline

- **2019-2024** - Core functionality development and testing
- **2025** – Major refactoring with Rust best practices and a complete UI/UX overhaul: modern terminal dashboard powered by ratatui.

### Key Technologies

- **Rust** - Core language for performance and safety
- **ratatui** - Terminal UI framework
- **crossterm** - Cross-platform terminal manipulation
- **sysinfo** - System information gathering
- **chrono** - Date and time handling

### Code Quality

- Follows Rust 2021 edition standards
- Comprehensive error handling
- Memory-safe with zero-cost abstractions
- Modular design for maintainability
- Extensive documentation and comments

## Performance

Sysly is designed for efficiency:

- Minimal CPU overhead during monitoring
- Efficient memory usage
- Responsive UI updates
- Optimized for real-time system monitoring

## Contributing

Contributions are welcome! Please ensure:

1. Code follows Rust conventions
2. All tests pass
3. Documentation is updated
4. Changes are well-documented

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the excellent `htop` project
- Built with the amazing Rust ecosystem
- Uses the sysinfo library for system data
- Thanks to all contributors and the Rust community

## Contact

- **Developer**: Thinh Nguyen
- **Email**: hungtrungthinh@gmail.com
- **Project**: https://github.com/hungtrungthinh/sysly

---

*Sysly - A macOS experiment conceived on July 2019, inspired by htop and powered by Rust & sysinfo* 🖥️✈️ 