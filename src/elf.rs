#![allow(non_camel_case_types)]
#![allow(dead_code)]

/* 32-bit ELF base types. */
pub type Elf32_Addr = u32;
pub type Elf32_Half = u16;
pub type Elf32_Off = u32;
pub type Elf32_Sword = i32;
pub type Elf32_Word = u32;

/* 64-bit ELF base types. */
pub type Elf64_Addr = u64;
pub type Elf64_Half = u16;
pub type Elf64_SHalf = i16;
pub type Elf64_Off = u64;
pub type Elf64_Sword = i32;
pub type Elf64_Word = u32;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;

// ELFOSABI
pub const ELFOSABI_NONE: u8 = 0;
pub const ELFOSABI_HPUX: u8 = 1;
pub const ELFOSABI_NETBSD: u8 = 2;
pub const ELFOSABI_LINUX: u8 = 3;
pub const ELFOSABI_HURD: u8 = 4;
pub const ELFOSABI_SOLARIS: u8 = 6;
pub const ELFOSABI_AIX: u8 = 7;
pub const ELFOSABI_IRIX: u8 = 8;
pub const ELFOSABI_FREEBSD: u8 = 9;
pub const ELFOSABI_TRU64: u8 = 10;
pub const ELFOSABI_MODESTO: u8 = 11;
pub const ELFOSABI_OPENBSD: u8 = 12;
pub const ELFOSABI_ARM: u8 = 97;
pub const ELFOSABI_STANDALONE: u8 = 255;

// ELFTYPE
pub const ET_NONE: u16 = 0;
pub const ET_REL: u16 = 1;
pub const ET_EXEC: u16 = 2;
pub const ET_DYN: u16 = 3;
pub const ET_CORE: u16 = 4;
pub const ET_LOPROC: u16 = 0xFF00;
pub const ET_HIPROC: u16 = 0xFFFF;

// ELFEM
pub const EM_NONE: u16 = 0;
pub const EM_386: u16 = 3;
pub const EM_X86_64: u16 = 62;

const EI_NIDENT: usize = 16;

#[derive(Debug)]
pub struct elf32_hdr {
    pub e_ident: [u8; EI_NIDENT], 
    pub e_type: Elf32_Half,
    pub e_machine: Elf32_Half,
    pub e_version: Elf32_Word,
    pub e_entry: Elf32_Addr,
    pub e_phoff: Elf32_Off,
    pub e_shoff: Elf32_Off,
    pub e_flags: Elf32_Word,
    pub e_ehsize: Elf32_Half,
    pub e_phentsize: Elf32_Half,
    pub e_phnum: Elf32_Half,
    pub e_shentsize: Elf32_Half,
    pub e_shnum: Elf32_Half,
    pub e_shstrndx: Elf32_Half,
}

#[derive(Debug)]
pub struct elf64_hdr {
    pub e_ident: [u8; EI_NIDENT], 
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}

pub type Elf32_Ehdr = elf32_hdr;
pub type Elf64_Ehdr = elf64_hdr;

#[derive(Debug)]
pub enum Ehdr {
    Elf32(Elf32_Ehdr),
    Elf64(Elf64_Ehdr),
}

#[derive(Debug)]
pub enum Elf_Addr {
    x86(u32),
    x64(u64),
}

#[derive(Debug)]
pub enum Elf_Off {
    x86(u32),
    x64(u64),
}

#[repr(C)]
#[derive(Debug)]
pub struct elf_header {
    pub ehdr: Ehdr,
    pub is_64bit: bool,
    pub is_le: bool,
    pub os_abi: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct Elf32_Phdr {
    pub p_type: Elf32_Word,
    pub p_offset: Elf32_Off,
    pub p_vaddr: Elf32_Addr,
    pub p_paddr: Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz: Elf32_Word,
    pub p_flags: Elf32_Word,
    pub p_align: Elf32_Word,
}

#[repr(C)]
#[derive(Debug)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}

#[derive(Debug)]
pub enum Phdr {
    x86(Elf32_Phdr),
    x64(Elf64_Phdr),
}