/************************************************
* File: elf.rs
* Author: Michal Švrček
*
* ELF binary header analysis.
*
* ver. 0.5.3
*************************************************/

#[derive(Debug)]
pub struct ElfInfo {
    pub class: &'static str,
    pub endian: &'static str,
    pub architecture: &'static str,
    pub file_type: &'static str,
    pub entry_point: u64,
    pub is_64_bit: bool,
}

pub fn parse_elf(data: &[u8]) -> Result<ElfInfo, String> {
    if data.len() < 16 {
        return Err("ELF identification header is too short".into());
    }

    if !data.starts_with(b"\x7FELF") {
        return Err("Invalid ELF signature".into());
    }

    // Determine ELF class
    let (class, is_64_bit, header_size) = match data[4] {
        1 => ("32-bit", false, 52),
        2 => ("64-bit", true, 64),
        _ => return Err("Unsupported ELF class".into()),
    };

    // Determine byte order
    let (endian, little_endian) = match data[5] {
        1 => ("Little Endian", true),
        2 => ("Big Endian", false),
        _ => return Err("Unsupported ELF endianness".into()),
    };

    if data[6] != 1 {
        return Err("Unsupported ELF identification version".into());
    }

    if data.len() < header_size {
        return Err(format!(
            "Incomplete ELF header: expected {header_size} bytes, got {}",
            data.len()
        ));
    }

    // Read a 16-bit field using the ELF byte order
    let read_u16 = |offset: usize| -> u16 {
        let bytes = [data[offset], data[offset + 1]];

        if little_endian {
            u16::from_le_bytes(bytes)
        } else {
            u16::from_be_bytes(bytes)
        }
    };

    // Read a 32-bit field using the ELF byte order
    let read_u32 = |offset: usize| -> u32 {
        let bytes: [u8; 4] = data[offset..offset + 4]
            .try_into()
            .expect("Validated ELF header length");

        if little_endian {
            u32::from_le_bytes(bytes)
        } else {
            u32::from_be_bytes(bytes)
        }
    };

    // Read a 64-bit field using the ELF byte order
    let read_u64 = |offset: usize| -> u64 {
        let bytes: [u8; 8] = data[offset..offset + 8]
            .try_into()
            .expect("Validated ELF header length");

        if little_endian {
            u64::from_le_bytes(bytes)
        } else {
            u64::from_be_bytes(bytes)
        }
    };

    // ELF header fields
    let e_type = read_u16(0x10);
    let e_machine = read_u16(0x12);

    let entry_point = if is_64_bit {
        read_u64(0x18)
    } else {
        read_u32(0x18) as u64
    };

    let file_type = match e_type {
        0 => "None",
        1 => "Relocatable Object",
        2 => "Executable",
        3 => "Shared Object / PIE",
        4 => "Core Dump",
        0xFE00..=0xFEFF => "OS-specific",
        0xFF00..=0xFFFF => "Processor-specific",
        _ => "Unknown",
    };

    let architecture = match e_machine {
        2 => "SPARC",
        3 => "Intel x86",
        8 => "MIPS",
        20 => "PowerPC",
        21 => "PowerPC64",
        40 => "ARM",
        42 => "SuperH",
        43 => "SPARC V9",
        50 => "IA-64",
        62 => "x86-64",
        83 => "AVR",
        94 => "Xtensa",
        105 => "MSP430",
        183 => "AArch64 (ARM64)",
        188 => "TilePro",
        189 => "MicroBlaze",
        243 => "RISC-V",
        247 => "BPF",
        258 => "LoongArch",
        _ => "Unknown architecture",
    };

    Ok(ElfInfo {
        class,
        endian,
        architecture,
        file_type,
        entry_point,
        is_64_bit,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_elf() {
        assert!(parse_elf(b"NOT ELF").is_err());
    }

    #[test]
    fn test_elf64_aarch64() {
        let mut data = [0u8; 64];

        data[0..4].copy_from_slice(b"\x7FELF");
        data[4] = 2; // ELF64
        data[5] = 1; // Little Endian
        data[6] = 1; // ELF version

        data[16..18].copy_from_slice(&2u16.to_le_bytes());
        data[18..20].copy_from_slice(&183u16.to_le_bytes());
        data[24..32].copy_from_slice(&0x12000u64.to_le_bytes());

        let info = parse_elf(&data).unwrap();

        assert_eq!(info.class, "64-bit");
        assert_eq!(info.architecture, "AArch64 (ARM64)");
        assert_eq!(info.entry_point, 0x12000);
    }

    #[test]
    fn test_truncated_elf() {
        let data = b"\x7FELF\x02\x01\x01";

        assert!(parse_elf(data).is_err());
    }
}
