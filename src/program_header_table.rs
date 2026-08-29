use crate::elf::*;
use crate::function::*;

pub fn get_program_header_table(header: elf_header, elf: &[u8]) -> Result<Vec<Phdr>, String> {
    let (e_phnum, e_phentsize, e_phoff) = match header.ehdr {
        Ehdr::Elf32(ehdr) => {
            (ehdr.e_phnum as usize, ehdr.e_phentsize as usize, ehdr.e_phoff as usize)
        }
        Ehdr::Elf64(ehdr) => {
            (ehdr.e_phnum as usize, ehdr.e_phentsize as usize, ehdr.e_phoff as usize)
        }
    };

    let mut phdrs = Vec::new();
    let is_le = header.is_le;

    for i in 0..e_phnum {
        let offset = e_phoff + i * e_phentsize;
        if header.is_64bit {
            let p_type: Elf64_Word = read_u32(offset + 0, &elf, is_le);
            let p_flags: Elf64_Word = read_u32(offset + 4, &elf, is_le);
            let p_offset: Elf64_Off = read_u64(offset + 8, &elf, is_le);
            let p_vaddr: Elf64_Addr = read_u64(offset + 16, &elf, is_le);
            let p_paddr: Elf64_Addr = read_u64(offset + 24, &elf, is_le);
            let p_filesz: Elf64_Xword = read_u64(offset + 32, &elf, is_le);
            let p_memsz: Elf64_Xword = read_u64(offset + 40, &elf, is_le);
            let p_align: Elf64_Xword = read_u64(offset + 48, &elf, is_le);
            phdrs.push(Phdr::x64( Elf64_Phdr{
                p_type,
                p_flags,
                p_offset,
                p_vaddr,
                p_paddr,
                p_filesz,
                p_memsz,
                p_align,
            }));
        } else {
            let p_type: Elf32_Word = read_u32(offset + 0, &elf, is_le);
            let p_offset: Elf32_Off = read_u32(offset + 4, &elf, is_le);
            let p_vaddr: Elf32_Addr = read_u32(offset + 8, &elf, is_le);
            let p_paddr: Elf32_Addr = read_u32(offset + 12, &elf, is_le);
            let p_filesz: Elf32_Word = read_u32(offset + 16, &elf, is_le);
            let p_memsz: Elf32_Word = read_u32(offset + 20, &elf, is_le);
            let p_flags: Elf32_Word = read_u32(offset + 24, &elf, is_le);
            let p_align: Elf32_Word = read_u32(offset + 28, &elf, is_le);
            phdrs.push(Phdr::x86( Elf32_Phdr{
                p_type,
                p_offset,
                p_vaddr,
                p_paddr,
                p_filesz,
                p_memsz,
                p_flags,
                p_align,
            }));
        }
    }

    Ok(phdrs)
}