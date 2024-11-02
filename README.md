# Podget

**Podget** is a command-line tool for downloading podcasts from RSS feeds. Written in Rust, Podget allows users to easily fetch and save podcast episodes to their local machine with minimal setup. 

## Features

- **Simple Command-Line Interface**: Download podcasts by providing an RSS feed URL.
- **Optimized for Performance**: Built with Rust, ensuring fast and efficient downloads.
- **File Naming**: Automatically generates filenames based on podcast episode titles.
- **Cross-Platform**: Works on any system where Rust is supported.

## Installation

To build and run Podget, clone the repository and use Cargo, the Rust package manager:

```bash
git clone https://github.com/yourusername/podget.git
cd podget
cargo build --release
```
Optionally, you can install it globally:
```bash
cargo install --path .
```
## Usage
Run Podget by passing the RSS feed URL as an argument:
```bash
podget "https://your-podcast-rss-feed-url.com/rss"
```
## Contributing
Contributions are welcome! Please open an issue or submit a pull request to contribute to Podget.

## License
This project is licensed under the GPL v3 License - see the LICENSE file for details.
