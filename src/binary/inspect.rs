/************************************************
* File: inspect.rs
* Author: Michal Švrček
*
* inspect operations.
*
* ver. 0.5.1
*************************************************/

use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Elf,
    PeDos,
    MachO,
    Wasm,
    JavaClass,
    Uf2,

    Zip,
    Rar,
    SevenZip,
    Gzip,
    Bzip2,
    Xz,
    Zstd,
    Lz4,
    Tar,
    SquashFs,

    Png,
    Jpeg,
    Gif,
    Bmp,
    Tiff,
    WebP,
    Ico,

    Pdf,
    Sqlite,
    Rtf,

    Mp3,
    Wav,
    Flac,
    Ogg,
    Mp4,
    Avi,

    Unknown,
}

impl FileType {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        match bytes {
            b if b.starts_with(b"\x7FELF") => Self::Elf,
            b if b.starts_with(b"MZ") => Self::PeDos,
            b if b.starts_with(b"\xFE\xED\xFA\xCE")
                || b.starts_with(b"\xCE\xFA\xED\xFE")
                || b.starts_with(b"\xFE\xED\xFA\xCF")
                || b.starts_with(b"\xCF\xFA\xED\xFE") =>
            {
                Self::MachO
            }
            b if b.starts_with(b"\0asm") => Self::Wasm,
            b if b.starts_with(b"\xCA\xFE\xBA\xBE") => Self::JavaClass,
            b if b.starts_with(b"UF2\n") || b.starts_with(&[0x55, 0x46, 0x32, 0x0A]) => {
                Self::Unknown
            }

            b if b.starts_with(b"PK\x03\x04") || b.starts_with(b"PK\x05\x06") => Self::Zip,
            b if b.starts_with(b"Rar!\x1A\x07") => Self::Rar,
            b if b.starts_with(b"7z\xBC\xAF\x27\x1C") => Self::SevenZip,
            b if b.starts_with(b"\x1F\x8B") => Self::Gzip,
            b if b.starts_with(b"BZh") => Self::Bzip2,
            b if b.starts_with(b"\xFD7zXZ\0") => Self::Xz,
            b if b.starts_with(b"\x28\xB5\x2F\xFD") => Self::Zstd,
            b if b.starts_with(b"\x04\x22\x4D\x18") => Self::Lz4,
            b if b.get(257..262) == Some(b"ustar") => Self::Tar,
            b if b.starts_with(b"hsqs") || b.starts_with(b"sqsh") => Self::SquashFs,

            b if b.starts_with(b"\x89PNG\r\n\x1A\n") => Self::Png,
            b if b.starts_with(b"\xFF\xD8\xFF") => Self::Jpeg,
            b if b.starts_with(b"GIF87a") || b.starts_with(b"GIF89a") => Self::Gif,
            b if b.starts_with(b"BM") => Self::Bmp,
            b if b.starts_with(b"II\x2A\0") || b.starts_with(b"MM\0\x2A") => Self::Tiff,
            b if b.starts_with(b"RIFF") && b.get(8..12) == Some(b"WEBP") => Self::WebP,
            b if b.starts_with(&[0x00, 0x00, 0x01, 0x00]) => Self::Ico,

            b if b.starts_with(b"%PDF-") => Self::Pdf,
            b if b.starts_with(b"SQLite format 3\0") => Self::Sqlite,
            b if b.starts_with(b"{\\rtf") => Self::Rtf,

            b if b.starts_with(b"ID3") => Self::Mp3,
            b if b.starts_with(b"RIFF") && b.get(8..12) == Some(b"WAVE") => Self::Wav,
            b if b.starts_with(b"fLaC") => Self::Flac,
            b if b.starts_with(b"OggS") => Self::Ogg,
            b if b.get(4..8) == Some(b"ftyp") => Self::Mp4,
            b if b.starts_with(b"RIFF") && b.get(8..12) == Some(b"AVI ") => Self::Avi,

            _ => Self::Unknown,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            // Executables & Firmware
            Self::Elf => "ELF Executable / Object",
            Self::PeDos => "DOS / Windows PE Candidate",
            Self::MachO => "Mach-O Executable",
            Self::Wasm => "WebAssembly Binary",
            Self::JavaClass => "Java Class File",
            Self::Uf2 => "UF2 Firmware Image",

            // Archives & Compression
            Self::Zip => "ZIP Archive",
            Self::Rar => "RAR Archive",
            Self::SevenZip => "7-Zip Archive",
            Self::Gzip => "GZIP Compressed Data",
            Self::Bzip2 => "BZIP2 Compressed Data",
            Self::Xz => "XZ Compressed Data",
            Self::Zstd => "Zstandard Compressed Data",
            Self::Lz4 => "LZ4 Compressed Data",
            Self::Tar => "TAR Archive",
            Self::SquashFs => "SquashFS Filesystem",

            // Images
            Self::Png => "PNG Image",
            Self::Jpeg => "JPEG Image",
            Self::Gif => "GIF Image",
            Self::Bmp => "BMP Bitmap",
            Self::Tiff => "TIFF Image",
            Self::WebP => "WebP Image",
            Self::Ico => "Windows Icon",

            // Documents & Data
            Self::Pdf => "PDF Document",
            Self::Sqlite => "SQLite Database",
            Self::Rtf => "Rich Text Document",

            // Audio & Video
            Self::Mp3 => "MP3 Audio (ID3)",
            Self::Wav => "WAV Audio",
            Self::Flac => "FLAC Audio",
            Self::Ogg => "Ogg Container",
            Self::Mp4 => "MP4 / ISO Base Media",
            Self::Avi => "AVI Video",

            // Unknown
            Self::Unknown => "Unknown Binary Format",
        }
    }
}

pub fn inspect_file(path: &Path) -> Result<(), String> {
    // Read file metadata
    let metadata =
        std::fs::metadata(path).map_err(|e| format!("Cannot read file metadata: {e}"))?;

    if !metadata.is_file() {
        return Err("The specified path is not a file".to_string());
    }

    let size = metadata.len();

    // Read the first 512 bytes for signature detection
    let mut file = File::open(path).map_err(|e| format!("Cannot open file: {e}"))?;

    let mut buffer = [0u8; 512];

    let n = file
        .read(&mut buffer)
        .map_err(|e| format!("Cannot read file: {e}"))?;

    let data = &buffer[..n];

    // Detect file type
    let file_type = FileType::from_bytes(data);

    // Format the first 16 bytes
    let first_bytes = &data[..data.len().min(16)];

    let hex = first_bytes
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<Vec<String>>()
        .join(" ");

    println!("File:        {}", path.display());
    println!("Size:        {size} bytes");
    println!("Size (HEX):  0x{size:X}");
    println!("First bytes: {hex}");
    println!("File type:   {}", file_type.name());

    Ok(())
}
