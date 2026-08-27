#[inline(always)]
pub fn check(elf: &[u8]) -> bool {
    return elf[0..=3] == [0x7f, b'E', b'L', b'F']; 
}