#![allow(non_snake_case)]

use std::ptr::{null, null_mut};
use libc::{c_int, c_uchar};

// Helper: ImChunkStream<>
// Build and iterate a contiguous stream of variable-sized structures.
// This is used by Settings to store persistent data while reducing allocation count.
// We store the chunk size first, and align the final size on 4 bytes boundaries.
// The tedious/zealous amount of casting is to avoid -Wcast-align warnings.
// template<typename T>
// struct ImChunkStream
pub struct ImChunkStream<T> {
    pub Buf: Vec<c_uchar>,
}

impl ImChunkStream<T> {
    // c_void    clear()                     { Buf.clear(); }
    pub fn clear(&mut self) {
        self.Buf.clear()
    }

    // bool    empty() const               { return Buf.Size == 0; }
    pub fn empty(&self) -> bool {
        self.Buf.is_empty()
    }

    // c_int     size() const                { return Buf.Size; }
    pub fn size(&self) -> usize {
        self.Buf.len()
    }

    // *mut T      alloc_chunk(size_t sz)
    pub fn alloc_chunk(&mut self, mut sz: usize) -> *mut T {
        // size_t HDR_SZ = 4;
        let mut HDR_SZ: usize = 4;
        sz = IM_MEMALIGN(HDR_SZ + sz, 4usize);
        // c_int off = Buf.Size; Buf.resize(off + sz);
        let mut off = self.Buf.len();
        self.Buf.resize(off + sz, 0);
        // ((*mut c_int)(*mut c_void)(Buf.Data + of0f32))[0] = sz;
        (self.Buf.as_mut_ptr() + off)[0] = sz;
        // return (*mut T)(*mut c_void)(Buf.Data + off + HDR_SZ);
        self.Buf.as_mut_ptr() + off + HDR_SZ
    }


    // *mut T      begin()
    pub fn begin(&mut self) -> *mut T {
        let mut HDR_SZ = 4usize;
        if !self.Buf.as_ptr() { return null_mut(); };
        // return (*mut T)(*mut c_void)(Buf.Data + HDR_SZ);
        return self.Buf.as_mut_ptr() + HDR_SZ;
    }


    // *mut T      next_chunk(*mut T p)
    pub fn next_chunk(&mut self, mut p: *mut T) -> *mut T {
        // size_t HDR_SZ = 4;
        let mut HDR_SZ: usize = 4;
        // IM_ASSERT(p >= begin() && p < end());
        // p = (*mut T)(*mut c_void)((*mut char)(*mut c_void)p + chunk_size(p));
        p = p + self.chunk_size(p);

        // if (p == (*mut T)(*mut c_void)((*mut char)end() + HDR_SZ)) {return ( * mut T)0;}
        if p == self.end() + HDR_SZ {
            return null_mut();
        }

        // IM_ASSERT(p < end());

        return p;
    }


    // c_int     chunk_size(*const T p)      { return ((*const c_int)p)[-1]; }
    pub fn chunk_size(&self, p: *const T) -> usize {
        p[-1]
    }


    // *mut T      end()                       { return (*mut T)(*mut c_void)(Buf.Data + Buf.Size); }
    pub fn end(&mut self) -> *mut T {
        self.Buf.as_mut_ptr() + self.Buf.len()
    }


    // c_int     offset_from_ptr(*const T p)
    pub fn offset_from_ptr(&self, p: *const T) -> usize {
        // IM_ASSERT(p >= begin() && p < end());
        // const ptrdiff_t off = (*const char)p - Buf.Data;
        let off = p - self.Buf.as_ptr();
        return off;
    }


    // *mut T      ptr_from_offset(c_int of0f32)
    pub fn ptr_from_offset(&mut self, off: usize) -> *mut T {
        // IM_ASSERT(off >= 4 && off < Buf.Size);
        // return (*mut T)(*mut c_void)(Buf.Data + of0f32);
        self.Buf.as_mut_ptr() + off
    }


    // c_void    swap(ImChunkStream<T>& rhs) { rhs.Buf.swap(Bu0f32); }
    pub fn swap(&mut self, rhs: &mut Self <T>) {
        // rhs.Buf.swap()
        todo!()
    }
}
