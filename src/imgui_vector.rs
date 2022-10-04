// 
// template<typename T>
// struct Vec
// {
//     c_int                 Size;
//     c_int                 Capacity;
//     T*                  Data;
// 
//     // Provide standard typedefs but we don't use them ourselves.
//     typedef T                   value_type;
//     typedef value_type*         iterator;
//     typedef *const value_type   const_iterator;
// 
//     // Constructors, destructor
//     inline Vec()                                       { Size = Capacity = 0; Data= null_mut(); }
//     inline Vec(const Vec<T>& src)                 { Size = Capacity = 0; Data= null_mut(); operator=(src); }
//     inline Vec<T>& operator=(const Vec<T>& src)   { clear(); resize(src.Size); if (src.Data) memcpy(Data, src.Data, Size * sizeof(T)); return *this; }
//     inline ~Vec()                                      { if (Data) IM_FREE(Data); } // Important: does not destruct anything
// 
//     inline c_void         clear()                             { if (Data) { Size = Capacity = 0; IM_FREE(Data); Data= null_mut(); } }  // Important: does not destruct anything
//     inline c_void         clear_delete()                      { for (let n: c_int = 0; n < Size; n++) IM_DELETE(Data[n]); clear(); }     // Important: never called automatically! always explicit.
//     inline c_void         clear_destruct()                    { for (let n: c_int = 0; n < Size; n++) Data[n].~T(); clear(); }           // Important: never called automatically! always explicit.
// 
//     inline bool         empty() const                       { return Size == 0; }
//     inline c_int          size() const                        { return Size; }
//     inline c_int          size_in_bytes() const               { return Size * sizeof(T); }
//     inline c_int          max_size() const                    { return 0x7FFFFFFF / sizeof(T); }
//     inline c_int          capacity() const                    { return Capacity; }
//     inline T&           operator[](i: c_int)                   { IM_ASSERT(i >= 0 && i < Size); return Data[i]; }
//     inline const T&     operator[](i: c_int) const             { IM_ASSERT(i >= 0 && i < Size); return Data[i]; }
// 
//     inline T*           begin()                             { return Data; }
//     inline *const T     begin() const                       { return Data; }
//     inline T*           end()                               { return Data + Size; }
//     inline *const T     end() const                         { return Data + Size; }
//     inline T&           front()                             { IM_ASSERT(Size > 0); return Data[0]; }
//     inline const T&     front() const                       { IM_ASSERT(Size > 0); return Data[0]; }
//     inline T&           back()                              { IM_ASSERT(Size > 0); return Data[Size - 1]; }
//     inline const T&     back() const                        { IM_ASSERT(Size > 0); return Data[Size - 1]; }
//     inline c_void         swap(Vec<T>& rhs)              { let rhs_size: c_int = rhs.Size; rhs.Size = Size; Size = rhs_size; let rhs_cap: c_int = rhs.Capacity; rhs.Capacity = Capacity; Capacity = rhs_cap; T* rhs_data = rhs.Data; rhs.Data = Data; Data = rhs_data; }
// 
//     inline c_int          _grow_capacity(sz: c_int) const        { let new_capacity: c_int = Capacity ? (Capacity + Capacity / 2) : 8; return new_capacity > sz ? new_capacity : sz; }
//     inline c_void         resize(new_size: c_int)                { if (new_size > Capacity) reserve(_grow_capacity(new_size)); Size = new_size; }
//     inline c_void         resize(new_size: c_int, const T& v)    { if (new_size > Capacity) reserve(_grow_capacity(new_size)); if (new_size > Size) for (let n: c_int = Size; n < new_size; n++) memcpy(&Data[n], &v, sizeof(v)); Size = new_size; }
//     inline c_void         shrink(new_size: c_int)                { IM_ASSERT(new_size <= Size); Size = new_size; } // Resize a vector to a smaller size, guaranteed not to cause a reallocation
//     inline c_void         reserve(new_capacity: c_int)           { if (new_capacity <= Capacity) return; T* new_data = (T*)IM_ALLOC(new_capacity * sizeof(T)); if (Data) { memcpy(new_data, Data, Size * sizeof(T)); IM_FREE(Data); } Data = new_data; Capacity = new_capacity; }
//     inline c_void         reserve_discard(new_capacity: c_int)   { if (new_capacity <= Capacity) return; if (Data) IM_FREE(Data); Data = (T*)IM_ALLOC(new_capacity * sizeof(T)); Capacity = new_capacity; }
// 
//     // NB: It is illegal to call push_back/push_front/insert with a reference pointing inside the ImVector data itself! e.g. v.push(v[10]) is forbidden.
//     inline c_void         push_back(const T& v)               { if (Size == Capacity) reserve(_grow_capacity(Size + 1)); memcpy(&Data[Size], &v, sizeof(v)); Size+= 1; }
//     inline c_void         pop_back()                          { IM_ASSERT(Size > 0); Size-= 1; }
//     inline c_void         push_front(const T& v)              { if (Size == 0) push_back(v); else insert(Data, v); }
//     inline T*           erase(*const T it)                  { IM_ASSERT(it >= Data && it < Data + Size); const ptrdiff_t off = it - Data; memmove(Data + off, Data + off + 1, (Size - off - 1) * sizeof(T)); Size-= 1; return Data + off; }
//     inline T*           erase(*const T it, *const T it_last){ IM_ASSERT(it >= Data && it < Data + Size && it_last >= it && it_last <= Data + Size); const ptrdiff_t count = it_last - it; const ptrdiff_t off = it - Data; memmove(Data + off, Data + off + count, (Size - off - count) * sizeof(T)); Size -= count; return Data + off; }
//     inline T*           erase_unsorted(*const T it)         { IM_ASSERT(it >= Data && it < Data + Size);  const ptrdiff_t off = it - Data; if (it < Data + Size - 1) memcpy(Data + off, Data + Size - 1, sizeof(T)); Size-= 1; return Data + off; }
//     inline T*           insert(*const T it, const T& v)     { IM_ASSERT(it >= Data && it <= Data + Size); const ptrdiff_t off = it - Data; if (Size == Capacity) reserve(_grow_capacity(Size + 1)); if (off < Size) memmove(Data + off + 1, Data + off, (Size - of0f32) * sizeof(T)); memcpy(&Data[off], &v, sizeof(v)); Size+= 1; return Data + off; }
//     inline bool         contains(const T& v) const          { *const T data = Data;  *const T data_end = Data + Size; while (data < data_end) if (*data++ == v) return true; return false; }
//     inline T*           find(const T& v)                    { T* data = Data;  *const T data_end = Data + Size; while (data < data_end) if (*data == v) break; else ++data; return data; }
//     inline *const T     find(const T& v) const              { *const T data = Data;  *const T data_end = Data + Size; while (data < data_end) if (*data == v) break; else ++data; return data; }
//     inline bool         find_erase(const T& v)              { *const T it = find(v); if (it < Data + Size) { erase(it); return true; } return false; }
//     inline bool         find_erase_unsorted(const T& v)     { *const T it = find(v); if (it < Data + Size) { erase_unsorted(it); return true; } return false; }
//     inline c_int          index_from_ptr(*const T it) const   { IM_ASSERT(it >= Data && it < Data + Size); const ptrdiff_t off = it - Data; return off; }
// };
