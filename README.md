# ValveFetch

ValveFetch is a lightweight, user-friendly wrapper for SteamCMD that simplifies the process of managing Steam workshop content and game servers. Written in Rust for maximum performance and reliability, it handles authentication, addon downloads, and path management with minimal configuration.

## Features

- 🔒 Secure Steam account credential management
- 📦 Automated workshop content downloads
- 🗂️ Custom installation path configuration
- 📝 Detailed logging system
- 💾 Path persistence for recurring downloads
- 🔄 Batch download support
- ⚡ Quick server setup and management
- 🦀 Written in Rust for optimal performance and safety

## Installation

```bash
# Using cargo
cargo install valvefetch

# From source
git clone https://github.com/yourusername/valvefetch
cd valvefetch
cargo build --release
```

## Quick Start

```bash
# Basic usage
valvefetch --login username

# Download a specific workshop item
valvefetch --download 123456789 --game garrysmod

# Set custom installation path
valvefetch --path "/path/to/addons" --save
```

## Configuration

ValveFetch stores its configuration in `~/.config/valvefetch/config.toml` (Linux/macOS) or `%APPDATA%\ValveFetch\config.toml` (Windows).

```toml
default_path = "/path/to/addons"
steam_path = "/path/to/steamcmd"
log_level = "INFO"
```

## Command Line Arguments

```
Usage: valvefetch [OPTIONS] COMMAND [ARGS]...

Options:
  --login TEXT         Steam username for authentication
  --download INT       Workshop item ID to download
  --game TEXT         Target game identifier (e.g., garrysmod)
  --path TEXT         Custom installation path
  --save              Save current path as default
  --batch FILE        Path to batch file containing workshop IDs
  --quiet            Reduce output verbosity
  --version          Show version information
  --help             Show this message and exit
```

## Examples

### Download Single Addon
```bash
valvefetch --login myusername --download 123456789 --game garrysmod
```

### Batch Download
```bash
valvefetch --login myusername --batch addons.txt --game garrysmod
```

### Set New Default Path
```bash
valvefetch --path "/games/garrysmod/addons" --save
```

## Requirements

- Rust 1.70+
- SteamCMD
- Active Steam account

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Valve Corporation for SteamCMD
- All contributors who have helped with the project
- The Steam Workshop community

## Support

If you encounter any issues or have questions, please:

1. Check the [FAQ](docs/FAQ.md)
2. Search existing [issues](https://github.com/yourusername/valvefetch/issues)
3. Create a new issue if necessary
