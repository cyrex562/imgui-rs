inline bool     ImBitArrayTestBit(*const u32 arr, c_int n)      { u32 mask = 1 << (n & 31); return (arr[n >> 5] & mask) != 0; }
inline c_void     ImBitArrayClearBit(*mut u32 arr, c_int n)           { u32 mask = 1 << (n & 31); arr[n >> 5] &= ~mask; }
inline c_void     ImBitArraySetBit(*mut u32 arr, c_int n)             { u32 mask = 1 << (n & 31); arr[n >> 5] |= mask; }
inline c_void     ImBitArraySetBitRange(*mut u32 arr, c_int n, c_int n2) // Works on range [n..n2)
{
    n2-= 1;
    while (n <= n2)
    {
        let a_mod: c_int = (n & 31);
        let b_mod: c_int = (n2 > (n | 31) ? 31 : (n2 & 31)) + 1;
        u32 mask = (((u64)1 << b_mod) - 1) & ~(((u64)1 << a_mod) - 1);
        arr[n >> 5] |= mask;
        n = (n + 32) & ~31;
    }
}

// Helper: ImBitArray class (wrapper over ImBitArray functions)
// Store 1-bit per value.
template<c_int BITCOUNT, let OFFSET: c_int = 0>
struct ImBitArray
{
    u32           Storage[(BITCOUNT + 31) >> 5];
    ImBitArray()                                { ClearAllBits(); }
    c_void            ClearAllBits()              { memset(Storage, 0, sizeof(Storage)); }
    c_void            SetAllBits()                { memset(Storage, 255, sizeof(Storage)); }
    bool            TestBit(c_int n) const        { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
    c_void            SetBit(c_int n)               { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArraySetBit(Storage, n); }
    c_void            ClearBit(c_int n)             { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); ImBitArrayClearBit(Storage, n); }
    c_void            SetBitRange(c_int n, c_int n2)  { n += OFFSET; n2 += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT && n2 > n && n2 <= BITCOUNT); ImBitArraySetBitRange(Storage, n, n2); } // Works on range [n..n2)
    bool            operator[](c_int n) const     { n += OFFSET; IM_ASSERT(n >= 0 && n < BITCOUNT); return ImBitArrayTestBit(Storage, n); }
};
