
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
    inline c_void  Reserve(n: c_int, size_t sz, let mut a: c_int = 4) { IM_ASSERT(n == CurrIdx && n < CHUNKS); CurrOff = IM_MEMALIGN(CurrOff, a); Offsets[n] = CurrOff; Sizes[n] = sz; CurrIdx+= 1; CurrOff += sz; }
    inline c_int   GetArenaSizeInBytes()              { return CurrOff; }
    inline c_void  SetArenaBasePtr(*mut c_void base_ptr)    { BasePtr = (*mut char)base_ptr; }
    inline *mut c_void GetSpanPtrBegin(n: c_int)             { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (BasePtr + Offsets[n]); }
    inline *mut c_void GetSpanPtrEnd(n: c_int)               { IM_ASSERT(n >= 0 && n < CHUNKS && CurrIdx == CHUNKS); return (BasePtr + Offsets[n] + Sizes[n]); }
    template<typename T>
    inline c_void  GetSpan(n: c_int, ImSpan<T>* span)    { span.set((*mut T)GetSpanPtrBegin(n), (*mut T)GetSpanPtrEnd(n)); }
};
