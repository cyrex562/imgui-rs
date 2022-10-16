use libc::size_t;

// Helper: ImBitVector
// Store 1-bit per value.
#[derive(Default, Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ImBitVector {
    // Vec<u32> Storage;
    pub Storage: Vec<u8>,
}

impl ImBitVector {
    // c_void            Create(sz: c_int)              { Storage.resize((sz + 31) >> 5); memset(Storage.Data, 0, Storage.Size * sizeof(Storage.Data[0])); }
    pub fn Create(sz: size_t) -> Self {
        let mut out = Self::default();
        out.Storage.reserve(sz);
        out
    }

    // c_void            Clear()                     { Storage.clear(); }
    pub fn Clear(&mut self) {
        self.Storage.clear();
    }

    // bool            TestBit(n: c_int) const        { IM_ASSERT(n < (Storage.Size << 5)); return ImBitArrayTestBit(Storage.Data, n); }
    pub fn TestBit(&self, n: size_t) -> bool {
        self.Storage[n] != 0
    }

    // c_void            SetBit(n: c_int)               { IM_ASSERT(n < (Storage.Size << 5)); ImBitArraySetBit(Storage.Data, n); }
    pub fn SetBit(&mut self, n: size_t) {
        self.Storage[n] = 1;
    }

    // c_void            ClearBit(n: c_int)             { IM_ASSERT(n < (Storage.Size << 5)); ImBitArrayClearBit(Storage.Data, n); }
    pub fn ClearBit(&mut self, n: size_t) {
        self.Storage[n] = 0;
    }
}
