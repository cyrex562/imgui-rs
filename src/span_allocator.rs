
// Helper: ImSpanAllocator<>
// Facilitate storing multiple chunks into a single large block (the "arena")
// - Usage: call Reserve() N times, allocate GetArenaSizeInBytes() worth, pass it to SetArenaBasePtr(), call GetSpan() N times to retrieve the aligned ranges.
// template<CHUNKS: c_int>

use libc::{c_char, c_void, size_t};
use crate::span::ImSpan;

#[derive(Default,Debug,Clone,Copy)]
pub struct ImSpanAllocator
{
    // *mut char   BasePtr;
    pub BasePtr: *mut c_char,
    // c_int     CurrOff;
    pub CurrOff: size_t,
    // c_int     CurrIdx;
    pub CurrIdx: size_t,
    // c_int     Offsets[CHUNKS];
    pub Offsets: Vec<size_t>,
    // c_int     Sizes[CHUNKS];
    pub Sizes: Vec<size_t>
}

impl ImSpanAllocator {

    // ImSpanAllocator()                               { memset(this, 0, sizeof(*this)); }


    // inline c_void  Reserve(n: c_int, sz: size_t, let mut a: c_int = 4) { IM_ASSERT(n == CurrIdx && n < CHUNKS); CurrOff = IM_MEMALIGN(CurrOff, a); Offsets[n] = CurrOff; Sizes[n] = sz; CurrIdx+= 1; CurrOff += sz; }
    pub fn Reserve(&mut self, n: size_t, a: size_t) {
        self.CurrOff = a;
        self.Offsets[n] = self.CurrOff;
        self.Sizes[n] = sz;
        self.CurrIdx += 1;
        self.CurrOff += sz;
    }

    // inline c_int   GetArenaSizeInBytes()              { return CurrOff; }
    pub fn GetArenaSizeInBytes(&mut self) -> size_t {
        self.CurrOff
    }

    // inline c_void  SetArenaBasePtr(base_ptr: *mut c_void)    { BasePtr = (*mut char)base_ptr; }
    pub fn SetArenaBasePtr(&mut self, base_ptr: *mut c_void) {
        self.BasePtr = base_ptr
    }

    // inline GetSpanPtrBegin: *mut c_void(n: c_int)             { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (BasePtr + Offsets[n]); }
    pub fn GetSpanPtrBegin(&mut self, n: size_t) -> *mut c_void {
        self.BasePtr + self.Offsets[n]
    }

    // inline GetSpanPtrEnd: *mut c_void(n: c_int)               { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (BasePtr + Offsets[n] + Sizes[n]); }
    pub fn GetSpanPtrEnd(&mut self, n: size_t) -> &mut c_void {
        self.BasePtr + self.Offsets[n] + self.Sizes[n]
    }

    // template<typename T>
    // inline c_void  GetSpan(n: c_int, ImSpan<T>* span)    { span.set((*mut T)GetSpanPtrBegin(n), (*mut T)GetSpanPtrEnd(n)); }
    pub fn GetSpan<T>(&mut self, n: size_t, span: *mut ImSpan<T>) {
        span.set2(self.GetSpanPtrBegin(n), self.GetSpanPtrEnd(n))
    }
}
