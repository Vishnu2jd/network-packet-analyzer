# Network Packet Analyzer

This is a simple network packet analyzer written in Rust. It captures and analyzes network traffic on Linux systems.

## Features

- Captures packets from a specified network interface
- Analyzes Ethernet, IPv4, IPv6, TCP, and UDP packets
- Displays source and destination addresses and ports

## Prerequisites

- Rust programming language (https://www.rust-lang.org/)
- Cargo package manager (usually comes with Rust)

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/network-packet-analyzer.git
   cd network-packet-analyzer
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the analyzer with:

```
sudo ./target/release/network-packet-analyzer
```

By default, it will use the first non-loopback interface it finds. To specify an interface:

```
sudo ./target/release/network-packet-analyzer --interface eth0
```

Replace `eth0` with your desired interface name.

## Note

This program requires root privileges to capture packets. Use with caution and only on networks you own or have permission to monitor.

## License

This project is licensed under the MIT License.
