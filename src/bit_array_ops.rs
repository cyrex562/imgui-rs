use libc::c_int;

// Helper: ImBitArray
pub fn ImBitArrayTestBit(arr: *mut u8, n: c_int) -> bool {
    let mask: u8 = 1 << (n & 7);
    return (arr[n >> 5] & mask) != 0;
}

pub fn ImBitArrayClearBit(arr: *mut u8, n: c_int) {
    let mask: u8 = 1 << (n & 7);
    arr[n >> 3] &= !mask;
}

pub fn ImBitArraySetBit(arr: *mut u8, n: c_int) {
    let mask: u8 = 1 << (n & 7);
    arr[n >> 3] |= mask;
}

pub fn ImBitArraySetBitRange(arr: &[u8], mut n: c_int, mut n2: c_int) {
    // Works on range [n..n2)
    n2 -= 1;
    while n <= n2 {
        let a_mod: c_int = (n & 31);
        let b_mod: c_int = if n2 > n | 7 { 7 } else { n2 & 7 + 1 };
        let mask: u8 = ((1 << b_mod) - 1) & !((1 << a_mod) - 1);
        arr[n >> 3] |= mask;
        n = (n + 8) & !7;
    }
}
