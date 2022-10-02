
// Helper: ImSpanAllocator<>
// Facilitate storing multiple chunks into a single large block (the "arena")
// - Usage: call Reserve() N times, allocate GetArenaSizeInBytes() worth, pass it to SetArenaBasePtr(), call GetSpan() N times to retrieve the aligned ranges.
template<c_int CHUNKS>
struct ImSpanAllocator
{
    *mut char   BasePtr;
    c_int     CurrOff;
    c_int     CurrIdx;
    c_int     Offsets[CHUNKS];
    c_int     Sizes[CHUNKS];

    ImSpanAllocator()                               { memset(this, 0, sizeof(*this)); }
    inline c_void  Reserve(c_int n, size_t sz, let mut a: c_int = 4) { IM_ASSERT(n == CurrIdx && n < CHUNKS); CurrOff = IM_MEMALIGN(CurrOff, a); Offsets[n] = CurrOff; Sizes[n] = sz; CurrIdx+= 1; CurrOff += sz; }
    inline c_int   GetArenaSizeInBytes()              { return CurrOff; }
    inline c_void  SetArenaBasePtr(*mut c_void base_ptr)    { BasePtr = (*mut char)base_ptr; }
    inline *mut c_void GetSpanPtrBegin(c_int n)             { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (*mut c_void)(BasePtr + Offsets[n]); }
    inline *mut c_void GetSpanPtrEnd(c_int n)               { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (*mut c_void)(BasePtr + Offsets[n] + Sizes[n]); }
    template<typename T>
    inline c_void  GetSpan(c_int n, ImSpan<T>* span)    { span.set((*mut T)GetSpanPtrBegin(n), (*mut T)GetSpanPtrEnd(n)); }
};
