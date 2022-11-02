use libc::size_t;

// Helper: ImBitVector
// Store 1-bit per value.
#[derive(Default, Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ImBitVector {
    pub Storage: Vec<u8>,
}

impl ImBitVector {
    pub fn Create(sz: size_t) -> Self {
        let mut out = Self::default();
        out.Storage.reserve(sz);
        out
    }

    pub fn Clear(&mut self) {
        self.Storage.clear();
    }

    pub fn TestBit(&self, n: size_t) -> bool {
        self.Storage[n] != 0
    }

    pub fn SetBit(&mut self, n: size_t) {
        self.Storage[n] = 1;
    }

    pub fn ClearBit(&mut self, n: size_t) {
        self.Storage[n] = 0;
    }
}
