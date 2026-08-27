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

const EI_NIDENT: usize = 16;

#[repr(C)]
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

#[repr(C)]
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