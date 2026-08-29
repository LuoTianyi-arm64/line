#[inline(always)]
pub fn read_u16(offset: usize, buf: &[u8], is_le: bool) -> u16 {
    let raw: [u8; 2] = buf[offset..offset + 2].try_into().unwrap();
    if is_le { u16::from_le_bytes(raw) } else { u16::from_be_bytes(raw) }
}

#[inline(always)]
pub fn read_u32(offset: usize, buf: &[u8], is_le: bool) -> u32 {
    let raw: [u8; 4] = buf[offset..offset + 4].try_into().unwrap();
    if is_le { u32::from_le_bytes(raw) } else { u32::from_be_bytes(raw) }
}

#[inline(always)]
pub fn read_u64(offset: usize, buf: &[u8], is_le: bool) -> u64 {
    let raw: [u8; 8] = buf[offset..offset + 8].try_into().unwrap();
    if is_le { u64::from_le_bytes(raw) } else { u64::from_be_bytes(raw) }
}

#[inline(always)]
pub fn read_i16(offset: usize, buf: &[u8], is_le: bool) -> i16 {
    let raw: [u8; 2] = buf[offset..offset + 2].try_into().unwrap();
    if is_le { i16::from_le_bytes(raw) } else { i16::from_be_bytes(raw) }
}

#[inline(always)]
pub fn read_i32(offset: usize, buf: &[u8], is_le: bool) -> i32 {
    let raw: [u8; 4] = buf[offset..offset + 4].try_into().unwrap();
    if is_le { i32::from_le_bytes(raw) } else { i32::from_be_bytes(raw) }
}

#[inline(always)]
pub fn read_i64(offset: usize, buf: &[u8], is_le: bool) -> i64 {
    let raw: [u8; 8] = buf[offset..offset + 8].try_into().unwrap();
    if is_le { i64::from_le_bytes(raw) } else { i64::from_be_bytes(raw) }
}