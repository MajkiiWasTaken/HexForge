# HexForge

**Cross-platform Binary Analysis & Protocol Engineering Toolkit** — a Rust CLI for examining raw bytes, firmware files and protocol data.

## Features

- HEX dump with `--offset` and `--length`
- Number conversion (DEC / HEX / BIN / OCT)
- Numeric encode/decode (`u8`–`u64`, `i8`–`i64`, `f32`, `f64`), endian and 32-bit Modbus byte order
- CRC16 Modbus, CRC16 CCITT-FALSE, CRC32, XOR8
- Search and compare binary files
- Inspect file signatures and ELF headers, ARM ABI, interpreter and dynamic dependencies

## Install

Download a release for **Windows x86-64** or **Linux x86-64** from [GitHub Releases](https://github.com/MajkiiWasTaken/HexForge/releases). Extract it and run `hexforge --help` (or `hexforge.exe --help`).

Alternatively, download the scripts in `scripts/` and run them from your terminal:

**Windows PowerShell** (per-user, no admin):

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
# To remove:
powershell -ExecutionPolicy Bypass -File .\scripts\uninstall.ps1
```

**Linux x86-64** (per-user, no sudo):

```bash
bash scripts/install.sh
# To remove:
bash scripts/uninstall.sh
```

Linux installs to `~/.local/bin`; ensure this directory is on your `PATH`.

## Examples

```bash
hexforge dump firmware.bin --offset 0x100 --length 64
hexforge convert FF --from hex
hexforge decode "00 00 41 20" --type f32 --order cdab --endian big
hexforge encode 10 --type f32 --endian big
hexforge crc modbus "01 03 00 00 00 02"
hexforge crc ccitt "31 32 33 34 35 36 37 38 39"
hexforge crc crc32 "31 32 33 34 35 36 37 38 39"
hexforge crc xor "12 34 56"
hexforge search firmware.bin "7F 45 4C 46"
hexforge compare firmware_v1.bin firmware_v2.bin
hexforge inspect firmware.bin
```

Run `hexforge <command> --help` to see all options.

## Build from source

Requires a recent stable Rust toolchain.

```bash
cargo test
cargo build --release
```

### Author: Michal Švrček

Distributed under MIT license.
