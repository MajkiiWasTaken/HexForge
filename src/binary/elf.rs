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
    pub os_abi: &'static str,
    pub abi_version: u8,
    pub program_headers: u16,
    pub section_headers: u16,
    pub flags: u32,
    pub interpreter: Option<String>,
    pub arm_eabi: Option<u32>,
    pub float_abi: Option<&'static str>,
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

    let os_abi = match data[7] {
        0 => "System V",
        1 => "HP-UX",
        2 => "NetBSD",
        3 => "GNU/Linux",
        6 => "Solaris",
        7 => "AIX",
        8 => "IRIX",
        9 => "FreeBSD",
        12 => "OpenBSD",
        _ => "Unknown",
    };

    let abi_version = data[8];

    let (flags, program_headers, section_headers) = if is_64_bit {
        (read_u32(0x30), read_u16(0x38), read_u16(0x3C))
    } else {
        (read_u32(0x24), read_u16(0x2C), read_u16(0x30))
    };

    let arm_eabi = if e_machine == 40 {
        Some((flags >> 24) & 0xFF)
    } else {
        None
    };

    let float_abi = if e_machine == 40 {
        match flags & 0x00000600 {
            0x00000200 => Some("Soft-float"),
            0x00000400 => Some("Hard-float"),
            0x00000000 => Some("Unspecified"),
            _ => Some("Conflicting flags"),
        }
    } else {
        None
    };

    let (phoff, phentsize) = if is_64_bit {
        (read_u64(0x20), read_u16(0x36))
    } else {
        (read_u32(0x1C) as u64, read_u16(0x2A))
    };

    let mut interpreter = None;

    let min_phentsize = if is_64_bit { 56u64 } else { 32u64 };

    if program_headers > 0 && u64::from(phentsize) < min_phentsize {
        return Err("Invalid ELF program header entry size".into());
    }

    for i in 0..program_headers {
        let entry_offset = phoff
            .checked_add(u64::from(i) * u64::from(phentsize))
            .ok_or("ELF program header offset overflow")?;

        let entry_offset =
            usize::try_from(entry_offset).map_err(|_| "ELF program header offset is too large")?;

        let entry_end = entry_offset
            .checked_add(min_phentsize as usize)
            .ok_or("ELF program header end overflow")?;

        if entry_end > data.len() {
            return Err("Incomplete ELF program headers; read more file data".into());
        }

        let p_type = read_u32(entry_offset);

        if p_type == 3 {
            let (p_offset, p_filesz) = if is_64_bit {
                (read_u64(entry_offset + 8), read_u64(entry_offset + 32))
            } else {
                (
                    read_u32(entry_offset + 4) as u64,
                    read_u32(entry_offset + 16) as u64,
                )
            };

            let start = usize::try_from(p_offset).map_err(|_| "Interpreter offset is too large")?;

            let length =
                usize::try_from(p_filesz).map_err(|_| "Interpreter length is too large")?;

            let end = start
                .checked_add(length)
                .ok_or("Interpreter range overflow")?;

            let raw = data
                .get(start..end)
                .ok_or("Interpreter is outside loaded file data")?;

            let raw = raw.strip_suffix(&[0]).unwrap_or(raw);

            let path = std::str::from_utf8(raw).map_err(|_| "Invalid interpreter path encoding")?;

            interpreter = Some(path.to_string());
            break;
        }
    }

    Ok(ElfInfo {
        class,
        endian,
        architecture,
        file_type,
        entry_point,
        is_64_bit,
        os_abi,
        abi_version,
        program_headers,
        section_headers,
        flags,
        interpreter,
        arm_eabi,
        float_abi,
    })
}

#[derive(Debug)]
pub struct ElfDependencies {
    pub has_dynamic_segment: bool,
    pub libraries: Vec<String>,
}

pub fn parse_elf_dependencies(data: &[u8]) -> Result<ElfDependencies, String> {
    if data.len() < 16 || !data.starts_with(b"\x7FELF") {
        return Err("Invalid ELF header".into());
    }

    let is_64 = match data[4] {
        1 => false,
        2 => true,
        _ => return Err("Invalid ELF class".into()),
    };

    let little = match data[5] {
        1 => true,
        2 => false,
        _ => return Err("Invalid ELF endian".into()),
    };

    let header_size = if is_64 { 64 } else { 52 };

    if data.len() < header_size {
        return Err("Truncated ELF header".into());
    }

    let read_u16 = |offset: usize| -> Result<u16, String> {
        let slice = data
            .get(offset..offset + 2)
            .ok_or("Invalid ELF u16 offset")?;
        let bytes: [u8; 2] = slice.try_into().map_err(|_| "Invalid u16 bytes")?;
        Ok(if little {
            u16::from_le_bytes(bytes)
        } else {
            u16::from_be_bytes(bytes)
        })
    };

    let read_u32 = |offset: usize| -> Result<u32, String> {
        let slice = data
            .get(offset..offset + 4)
            .ok_or("Invalid ELF u32 offset")?;
        let bytes: [u8; 4] = slice.try_into().map_err(|_| "Invalid u32 bytes")?;
        Ok(if little {
            u32::from_le_bytes(bytes)
        } else {
            u32::from_be_bytes(bytes)
        })
    };

    let read_u64 = |offset: usize| -> Result<u64, String> {
        let slice = data
            .get(offset..offset + 8)
            .ok_or("Invalid ELF u64 offset")?;
        let bytes: [u8; 8] = slice.try_into().map_err(|_| "Invalid u64 bytes")?;
        Ok(if little {
            u64::from_le_bytes(bytes)
        } else {
            u64::from_be_bytes(bytes)
        })
    };

    let read_word = |offset: usize| -> Result<u64, String> {
        if is_64 {
            read_u64(offset)
        } else {
            Ok(read_u32(offset)? as u64)
        }
    };

    let phoff = read_word(if is_64 { 0x20 } else { 0x1C })?;
    let phentsize = read_u16(if is_64 { 0x36 } else { 0x2A })? as u64;
    let phnum = read_u16(if is_64 { 0x38 } else { 0x2C })? as u64;

    let min_phentsize = if is_64 { 56 } else { 32 };

    if phnum > 0 && phentsize < min_phentsize {
        return Err("Invalid program header size".into());
    }

    // (virtual address, file offset, file size)
    let mut load_segments = Vec::new();
    let mut dynamic_segment = None;

    for i in 0..phnum {
        let pos = phoff
            .checked_add(
                i.checked_mul(phentsize)
                    .ok_or("Program header multiplication overflow")?,
            )
            .ok_or("Program header offset overflow")?;

        let pos = usize::try_from(pos).map_err(|_| "Program header offset too large")?;

        data.get(pos..pos + min_phentsize as usize)
            .ok_or("Truncated program header")?;

        let kind = read_u32(pos)?;

        let (file_offset, vaddr, filesz) = if is_64 {
            (read_u64(pos + 8)?, read_u64(pos + 16)?, read_u64(pos + 32)?)
        } else {
            (
                read_u32(pos + 4)? as u64,
                read_u32(pos + 8)? as u64,
                read_u32(pos + 16)? as u64,
            )
        };

        match kind {
            1 => load_segments.push((vaddr, file_offset, filesz)),
            2 => dynamic_segment = Some((file_offset, filesz)),
            _ => {}
        }
    }

    let Some((dynamic_offset, dynamic_size)) = dynamic_segment else {
        return Ok(ElfDependencies {
            has_dynamic_segment: false,
            libraries: Vec::new(),
        });
    };

    let dynamic_start =
        usize::try_from(dynamic_offset).map_err(|_| "Dynamic segment offset too large")?;

    let dynamic_size =
        usize::try_from(dynamic_size).map_err(|_| "Dynamic segment size too large")?;

    let dynamic_end = dynamic_start
        .checked_add(dynamic_size)
        .ok_or("Dynamic segment range overflow")?;

    data.get(dynamic_start..dynamic_end)
        .ok_or("Truncated dynamic segment")?;

    let entry_size = if is_64 { 16 } else { 8 };

    let mut strtab_address = None;
    let mut strtab_size = None;
    let mut needed_offsets = Vec::new();

    for pos in (dynamic_start..dynamic_end).step_by(entry_size) {
        if dynamic_end - pos < entry_size {
            return Err("Incomplete dynamic entry".into());
        }

        let tag = read_word(pos)?;
        let value = read_word(pos + entry_size / 2)?;

        match tag {
            0 => break,                        // DT_NULL
            1 => needed_offsets.push(value),   // DT_NEEDED
            5 => strtab_address = Some(value), // DT_STRTAB
            10 => strtab_size = Some(value),   // DT_STRSZ
            _ => {}
        }
    }

    if needed_offsets.is_empty() {
        return Ok(ElfDependencies {
            has_dynamic_segment: true,
            libraries: Vec::new(),
        });
    }

    let strtab_address = strtab_address.ok_or("Missing DT_STRTAB")?;

    let strtab_size = strtab_size.ok_or("Missing DT_STRSZ")?;

    // Translate virtual string table address through PT_LOAD
    let strtab_file_offset = load_segments
        .iter()
        .find_map(|&(vaddr, file_offset, filesz)| {
            let delta = strtab_address.checked_sub(vaddr)?;

            if delta >= filesz {
                return None;
            }

            let remaining = filesz - delta;
            if strtab_size > remaining {
                return None;
            }

            file_offset.checked_add(delta)
        })
        .ok_or("Cannot map DT_STRTAB to file offset")?;

    let strtab_start =
        usize::try_from(strtab_file_offset).map_err(|_| "String table offset too large")?;

    let strtab_len = usize::try_from(strtab_size).map_err(|_| "String table too large")?;

    let strtab_end = strtab_start
        .checked_add(strtab_len)
        .ok_or("String table range overflow")?;

    let strtab = data
        .get(strtab_start..strtab_end)
        .ok_or("Truncated dynamic string table")?;

    let mut libraries = Vec::new();

    for offset in needed_offsets {
        let start = usize::try_from(offset).map_err(|_| "DT_NEEDED offset too large")?;

        let name_bytes = strtab
            .get(start..)
            .ok_or("DT_NEEDED offset outside string table")?;

        let nul = name_bytes
            .iter()
            .position(|&byte| byte == 0)
            .ok_or("Unterminated library name")?;

        let name =
            std::str::from_utf8(&name_bytes[..nul]).map_err(|_| "Invalid library name encoding")?;

        libraries.push(name.to_string());
    }

    Ok(ElfDependencies {
        has_dynamic_segment: true,
        libraries,
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
