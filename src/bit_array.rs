use std::ops::Index;
use libc::c_int;


// Helper: ImBitArray class (wrapper over ImBitArray functions)
// Store 1-bit per value.
// template<BITCOUNT: c_int, let OFFSET: c_int = 0>
#[derive(Default,Debug,Copy, Clone)]
pub struct ImBitArray
{
    // u32           Storage[(BITCOUNT + 31) >> 5];
    pub Storage: [u32;(BITCOUNT + 31) >> 5]
}

impl ImBitArray {
    // ImBitArray()                                { ClearAllBits(); }
    // c_void            ClearAllBits()              { memset(Storage, 0, sizeof(Storage)); }
    pub fn ClearAllBits(&mut self) {
        for i in 0 .. self.Storage.len() {
            self.Storage[i] = 0;
        }
    }

    // c_void            SetAllBits()                { memset(Storage, 255, sizeof(Storage)); }
    pub fn SetAllBits(&mut self) {
        for i in 0 .. self.Storage.len() {
            self.Storage[i] = 0xff;
        }
    }

    // bool            TestBit(n: c_int) const        { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
    pub fn TestBit(&mut self, mut n: c_int) -> bool {
        n += OFFSET;
        ImBitArrayTestBit(&self.Storage, n as usize)
    }


// c_void            SetBit(n: c_int)               { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArraySetBit(Storage, n); }
pub fn SetBit(&mut self, mut n: c_int) {
    n += OFFSET;
    ImBitArraySetBit(&mut self.Storage, n as usize)
}

// c_void            ClearBit(n: c_int)             { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArrayClearBit(Storage, n); }
pub fn ClearBit(&mut self, mut n: c_int) {
    n += OFFSET;
    ImBitArrayClearBit(&mut self.Storage,n as usize)
}

// c_void            SetBitRange(n: c_int, n2: c_int)  { n += OFFSET; n2 += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT && n2 > n && n2 <= BITCOUNT); ImBitArraySetBitRange(Storage, n, n2); } // Works on range [n..n2)
pub fn SetBitRange(&mut self, mut n: c_int, mut n2: c_int) {
    n += OFFSET;
    n2 += OFFSET;
    ImBitArraySetBitRange(&mut self.Storage, n, n2)
}

    // bool            operator[](n: c_int) const     { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
}

// impl Index<usize> for ImBitArray {
//     type Output = bool;
//
//     fn index(&self, index: usize) -> &Self::Output {
//         ImBitArrayTestBit(&self.Storage, index)
//     }
// }

// impl IndexMut<usize> for ImBitArray {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         ImBitArraySetBit(self.Storage,,)
//     }
// }


// inline bool     ImBitArrayTestBit(* use std::ops::{Index, IndexMut};
pub fn ImBitArrayTestBit(storage: &[u32], index: usize) -> bool {
    storage[index] > 0
}

// const u32 arr, n: c_int)      { let mut mask: u32 = 1 << (n & 31); return (arr[n >> 5] & mask) != 0; }
// inline c_void     ImBitArrayClearBit(*mut u32 arr, n: c_int)           { let mut mask: u32 = 1 << (n & 31); arr[n >> 5] &= !mask; }
pub fn ImBitArrayClearBit(storage: &mut[u32], index: usize) {
    storage[index] = 0;
}

// inline c_void     ImBitArraySetBit(*mut u32 arr, n: c_int)             { let mut mask: u32 = 1 << (n & 31); arr[n >> 5] |= mask; }
pub fn ImBitArraySetBit(storage: &mut[u32], index: c_int) {
    storage[index] = 1;
}

// inline c_void     ImBitArraySetBitRange(*mut u32 arr, n: c_int, n2: c_int) // Works on range [n..n2)
pub fn ImBitArraySetBitRange(storage: &mut [u32], mut n: c_int, mut n2: c_int) {
    n2 -= 1;
    while n <= n2 {
        let a_mod: c_int = (n & 31);
        let b_mod: c_int = if n2 > (n | 31) { 31 } else { (n2 & 31) + 1 };
        let mask = ((1 << b_mod) - 1) & !((1 << a_mod) - 1);
        storage[n >> 5] |= mask;
        n = (n + 32) & !31;
    }
}
