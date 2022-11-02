// Helper: ImBitArray class (wrapper over ImBitArray functions)
// Store 1-bit per value.
// template<BITCOUNT: c_int, let OFFSET: c_int = 0>

use crate::bit_vector::ImBitVector;

pub type ImBitArray = ImBitVector;

// pub struct ImBitArray
// {
//     u32           Storage[(BITCOUNT + 31) >> 5];
//     ImBitArray()                                { ClearAllBits(); }
//     c_void            ClearAllBits()              { memset(Storage, 0, sizeof(Storage)); }
//     c_void            SetAllBits()                { memset(Storage, 255, sizeof(Storage)); }
//     bool            TestBit(n: c_int) const        { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
//     c_void            SetBit(n: c_int)               { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArraySetBit(Storage, n); }
//     c_void            ClearBit(n: c_int)             { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArrayClearBit(Storage, n); }
//     c_void            SetBitRange(n: c_int, n2: c_int)  { n += OFFSET; n2 += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT && n2 > n && n2 <= BITCOUNT); ImBitArraySetBitRange(Storage, n, n2); } // Works on range [n..n2)
//     bool            operator[](n: c_int) const     { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
// };
