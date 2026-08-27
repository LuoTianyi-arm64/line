use crate::check;
use crate::elf::*;

pub fn get_header_info(elf: &[u8]) -> Result<Ehdr, String> {
    if !check(&elf) {
        return Err("Not a vivid ELF".to_string());
    }
    let e_ident: [u8; 16] = elf[0..16].try_into().unwrap();

    // EI_CLASS
    #[cfg(debug_assertions)]
    {
        match elf[4] {
            1 => println!("This ELF is 32-bit."),
            2 => println!("This ELF is 64-bit."),
            _ => return Err("Not a vivid ELF".to_string()),
        }
    }


    // EI_DATA

    let is_le: bool;
    #[cfg(debug_assertions)]
    {
        match elf[5] {
            1 => {
                println!("This ELF is Little-Endian.");
                is_le = true;
            },
            2 => {
                println!("This ELF is Big-Endian.");
                is_le = false;
            },
            _ => return Err("Not a vivid ELF".to_string()),
        }
    }

    #[cfg(not(debug_assertions))]
    {
        match elf[5] {
            1 => {
                is_le = true;
            },
            2 => {
                is_le = false;
            },
            _ => return Err("Not a vivid ELF".to_string()),
        }
    }

    // EI_OSABI
    #[cfg(debug_assertions)]
    {
        match elf[7] {
            0 => println!("OS-ABI is System-V."),
            1 => println!("OS-ABI is HP-UX."),
            2 => println!("OS-ABI is NetBSD."),
            3 => println!("OS-ABI is Linux."),
            4 => println!("OS-ABI is GNU-Hurd."),
            6 => println!("OS-ABI is Sun Solaris."),
            7 => println!("OS-ABI is IBM AIX."),
            8 => println!("OS-ABI is SGI IRIX."),
            9 => println!("OS-ABI is FreeBSD."),
            10 => println!("OS-ABI is TRU64 UNIX."),
            11 => println!("OS-ABI is Novell Modesto."),
            12 => println!("OS-ABI is OpenBSD."),
            97 => println!("OS-ABI is ARM."),
            255 => println!("No system."),
            _ => return Err("Not a vivid ELF".to_string()),
        }
    }
    
    match elf[4] {
        // 32-bit
        1 => {
            let e_type: Elf32_Half = read_u16(16, &elf, is_le);
            let e_machine: Elf32_Half = read_u16(18, &elf, is_le);
            let e_version: Elf32_Word = read_u32(20, &elf, is_le);
            let e_entry: Elf32_Addr = read_u32(24, &elf, is_le);
            let e_phoff: Elf32_Off = read_u32(28, &elf, is_le);
            let e_shoff: Elf32_Off = read_u32(32, &elf, is_le);
            let e_flags: Elf32_Word = read_u32(36, &elf, is_le);
            let e_ehsize: Elf32_Half = read_u16(40, &elf, is_le);
            let e_phentsize: Elf32_Half = read_u16(42, &elf, is_le);
            let e_phnum: Elf32_Half = read_u16(44, &elf, is_le);
            let e_shentsize: Elf32_Half = read_u16(16, &elf, is_le);
            let e_shnum: Elf32_Half = read_u16(48, &elf, is_le);
            let e_shstrndx: Elf32_Half = read_u16(50, &elf, is_le);
            return Ok(Ehdr::Elf32(Elf32_Ehdr {
                e_ident, 
                e_type,
                e_machine,
                e_version,
                e_entry,
                e_phoff,
                e_shoff,
                e_flags,
                e_ehsize,
                e_phentsize,
                e_phnum,
                e_shentsize,
                e_shnum,
                e_shstrndx,
            }));
        },
        // 64-bit
        2 => {
            let e_type: Elf64_Half = read_u16(16, &elf, is_le);
            let e_machine: Elf64_Half = read_u16(18, &elf, is_le);
            let e_version: Elf64_Word = read_u32(20, &elf, is_le);
            let e_entry: Elf64_Addr = read_u64(24, &elf, is_le);
            let e_phoff: Elf64_Off = read_u64(32, &elf, is_le);
            let e_shoff: Elf64_Off = read_u64(40, &elf, is_le);
            let e_flags: Elf64_Word = read_u32(48, &elf, is_le);
            let e_ehsize: Elf64_Half = read_u16(52, &elf, is_le);
            let e_phentsize: Elf64_Half = read_u16(54, &elf, is_le);
            let e_phnum: Elf64_Half = read_u16(56, &elf, is_le);
            let e_shentsize: Elf64_Half = read_u16(58, &elf, is_le);
            let e_shnum: Elf64_Half = read_u16(60, &elf, is_le);
            let e_shstrndx: Elf64_Half = read_u16(62, &elf, is_le);
            return Ok(Ehdr::Elf64(Elf64_Ehdr {
                e_ident, 
                e_type,
                e_machine,
                e_version,
                e_entry,
                e_phoff,
                e_shoff,
                e_flags,
                e_ehsize,
                e_phentsize,
                e_phnum,
                e_shentsize,
                e_shnum,
                e_shstrndx,
            }));
        },
        _ => return Err("Not a vivid ELF".to_string()),
    }
}

#[inline(always)]
fn read_u16(offset: usize, buf: &[u8], is_le: bool) -> u16 {
    let raw: [u8; 2] = buf[offset..offset + 2].try_into().unwrap();
    if is_le { u16::from_le_bytes(raw) } else { u16::from_be_bytes(raw) }
}

#[inline(always)]
fn read_u32(offset: usize, buf: &[u8], is_le: bool) -> u32 {
    let raw: [u8; 4] = buf[offset..offset + 4].try_into().unwrap();
    if is_le { u32::from_le_bytes(raw) } else { u32::from_be_bytes(raw) }
}

#[inline(always)]
fn read_u64(offset: usize, buf: &[u8], is_le: bool) -> u64 {
    let raw: [u8; 8] = buf[offset..offset + 8].try_into().unwrap();
    if is_le { u64::from_le_bytes(raw) } else { u64::from_be_bytes(raw) }
}

#[inline(always)]
fn read_i16(offset: usize, buf: &[u8], is_le: bool) -> i16 {
    let raw: [u8; 2] = buf[offset..offset + 2].try_into().unwrap();
    if is_le { i16::from_le_bytes(raw) } else { i16::from_be_bytes(raw) }
}

#[inline(always)]
fn read_i32(offset: usize, buf: &[u8], is_le: bool) -> i32 {
    let raw: [u8; 4] = buf[offset..offset + 4].try_into().unwrap();
    if is_le { i32::from_le_bytes(raw) } else { i32::from_be_bytes(raw) }
}

#[inline(always)]
fn read_i64(offset: usize, buf: &[u8], is_le: bool) -> i64 {
    let raw: [u8; 8] = buf[offset..offset + 8].try_into().unwrap();
    if is_le { i64::from_le_bytes(raw) } else { i64::from_be_bytes(raw) }
}