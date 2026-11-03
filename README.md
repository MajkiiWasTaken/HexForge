# HexForge

![Rust](https://img.shields.io/badge/Rust-CLI-000000?style=for-the-badge&logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-x86--64-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-x86--64-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

**HexForge** is a lightweight, cross-platform command-line toolkit for binary data inspection, firmware analysis, numerical conversions and communication protocol diagnostics.

Built with **Rust**, HexForge brings commonly used binary analysis utilities into a single, fast and portable application for developers working with embedded systems, industrial communication, firmware and low-level software.

---

### About

Working with embedded devices, communication protocols and binary files often requires multiple tools for examining raw data, converting values, calculating checksums or identifying executable formats.

HexForge provides these capabilities through a unified command-line interface.

Whether you're debugging Modbus communication, inspecting a firmware image, verifying a CRC checksum or comparing binary files, HexForge helps simplify everyday development and diagnostic tasks.

The application runs locally, requires no external services and can be used directly from a terminal.

### Features

| Feature | Description |
|---|---|
| **HEX Dump** | Display binary files as hexadecimal bytes with ASCII representation and configurable offsets |
| **Number Conversion** | Convert values between decimal, hexadecimal, binary and octal |
| **Encode / Decode** | Interpret and generate integer and IEEE 754 floating-point values |
| **Byte Ordering** | Support Big Endian, Little Endian and Modbus register ordering |
| **Checksums** | Calculate CRC16 Modbus, CRC16 CCITT, CRC32 and XOR8 |
| **Binary Search** | Locate hexadecimal byte sequences inside binary files |
| **Binary Compare** | Compare two files and identify differing byte offsets |
| **File Inspection** | Identify common binary formats using file signatures |
| **ELF Analysis** | Inspect ELF32/ELF64 headers, architectures, ABI information, runtime configuration and declared dependencies |

HexForge is designed for both general-purpose binary diagnostics and development workflows involving microcontrollers, industrial gateways, embedded Linux and communication protocols.

---

### Install

Download a precompiled binary from [GitHub Releases](https://github.com/MajkiiWasTaken/HexForge/releases), or use the included installation scripts.

**Windows**

Requires PowerShell 5.1 or newer.

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

Installs HexForge into `%LOCALAPPDATA%\Programs\HexForge` and adds it to the user `PATH`.

**Linux**

Requires x86-64 Linux, Bash and standard download/archive utilities.

```bash
bash scripts/install.sh
```

Installs HexForge into `~/.local/bin`.

No Rust toolchain is required when installing precompiled releases.

---

### Usage

HexForge provides several commands for common binary and protocol operations.

**Inspect a firmware or executable file**

```bash
hexforge inspect firmware.bin
```

Identifies the file format and, for ELF binaries, displays architecture, endianness, entry point, ARM ABI, interpreter and dependency information.

**Display a HEX dump**

```bash
hexforge dump firmware.bin --offset 0x100 --length 64
```

**Convert numerical values**

```bash
hexforge convert FF --from hex
```

**Encode and decode floating-point data**

```bash
hexforge decode "41 20 00 00" --type f32 --endian big
hexforge encode 10 --type f32 --endian big
```

**Calculate a Modbus CRC16 checksum**

```bash
hexforge crc modbus "01 03 00 00 00 02"
```

**Search and compare binary files**

```bash
hexforge search firmware.bin "7F 45 4C 46"
hexforge compare old.bin new.bin
```

Run `hexforge --help` for the complete command reference.

---

### Uninstall

**Windows**

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\uninstall.ps1
```

**Linux**

```bash
bash scripts/uninstall.sh
```

---

### Build from source

Requires the Rust toolchain and Cargo.

```bash
git clone https://github.com/MajkiiWasTaken/HexForge.git
cd HexForge

cargo test
cargo build --release
```

The compiled executable is available in `target/release/`.

---

### Author

**Michal Švrček**

Distributed under the **MIT License**.