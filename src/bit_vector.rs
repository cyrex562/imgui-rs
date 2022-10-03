use libc::c_int;
use crate::bit_array::{ImBitArrayClearBit, ImBitArraySetBit, ImBitArrayTestBit};

// Helper: ImBitVector
// Store 1-bit per value.
#[derive(Default, Debug, Copy, Clone)]
pub struct ImBitVector {
    // Vec<u32> Storage;
    pub Storage: Vec<u32>,
}

impl ImBitVector {
    // c_void            Create(c_int sz)              { Storage.resize((sz + 31) >> 5); memset(Storage.Data, 0, Storage.Size * sizeof(Storage.Data[0])); }
    pub fn Create(&mut self, sz: c_int) {
        self.Storage.resize(((sz + 31) >> 5) as usize, 0);
    }

    // c_void            Clear()                     { Storage.clear(); }
    pub fn Clear(&mut self) {
        self.Storage.clear()
    }

    // bool            TestBit(c_int n) const        { IM_ASSERT(n < (Storage.Size << 5)); return ImBitArrayTestBit(Storage.Data, n); }
    pub fn TestBit(&mut self, n: c_int) -> bool {
        ImBitArrayTestBit(self.Storage.as_slice(), n as usize)
    }


    // c_void            SetBit(c_int n)               { IM_ASSERT(n < (Storage.Size << 5)); ImBitArraySetBit(Storage.Data, n); }
    pub fn SetBit(&mut self, n: c_int) {
        ImBitArraySetBit(&mut self.Storage, n);
    }

    // c_void            ClearBit(c_int n)             { IM_ASSERT(n < (Storage.Size << 5)); ImBitArrayClearBit(Storage.Data, n); }
    pub fn ClearBit(&mut self, n: c_int) {
        ImBitArrayClearBit(&mut self.Storage, n as usize);
    }
}
