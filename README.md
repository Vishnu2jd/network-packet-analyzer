# Network Packet Analyzer

This is a simple network packet analyzer written in Rust. It captures and analyzes network traffic on Linux systems.

## Features

- Captures packets from a specified network interface
- Analyzes Ethernet, IPv4, IPv6, TCP, and UDP packets
- Displays source and destination addresses and ports


## Installation

1. Clone this repository:
   ```
   git clone --depth=1 https://github.com/yourusername/network-packet-analyzer.git
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

Replace `eth0` with appropriate interface name.
