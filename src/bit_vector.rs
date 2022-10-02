// Helper: ImBitVector
// Store 1-bit per value.
struct  ImBitVector
{
    Vec<u32> Storage;
    c_void            Create(c_int sz)              { Storage.resize((sz + 31) >> 5); memset(Storage.Data, 0, Storage.Size * sizeof(Storage.Data[0])); }
    c_void            Clear()                     { Storage.clear(); }
    bool            TestBit(c_int n) const        { IM_ASSERT(n < (Storage.Size << 5)); return ImBitArrayTestBit(Storage.Data, n); }
    c_void            SetBit(c_int n)               { IM_ASSERT(n < (Storage.Size << 5)); ImBitArraySetBit(Storage.Data, n); }
    c_void            ClearBit(c_int n)             { IM_ASSERT(n < (Storage.Size << 5)); ImBitArrayClearBit(Storage.Data, n); }
};
