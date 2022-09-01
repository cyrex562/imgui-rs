pub fn decode_85_byte(c: u8) -> u32 {
    if c >= '\\' as u8{
        (c - 36) as u32
    } else {
        (c - 35) as u32
    }
}


// static void         Decode85(const unsigned char* src, unsigned char* dst)
pub fn decode85(src: &mut Vec<u8>, dst: &mut Vec<u8>)
{
    let mut src_idx: usize = 0;
    let mut dst_idx: usize = 0;

    // while (*src)
    for idx in 0 .. src.len()
    {
        let tmp = Decode85Byte(src[src_idx+0]) +
            85 * (Decode85Byte(src[src_idx+1]) +
                85 * (Decode85Byte(src[src_idx+2]) +
                    85 * (Decode85Byte(src[src_idx+3]) +
                        85 * Decode85Byte(src[src_idx+4]))));
        dst[dst_idx+0] = ((tmp >> 0) & 0xFF);
        dst[dst_idx+1] = ((tmp >> 8) & 0xFF);
        dst[dst_idx+2] = ((tmp >> 16) & 0xFF);
        dst[dst_idx+3] = ((tmp >> 24) & 0xFF);   // We can't assume little-endianness.
        src_idx += 5;
        dst_idx += 4;
    }
}
