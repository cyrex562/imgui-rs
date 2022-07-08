use std::ops::Index;

// Helper: Growable text buffer for logging/accumulating text
// (this could be called 'ImGuiTextBuilder' / 'ImGuiStringBuilder')
#[derive(Default,Debug,Clone)]
pub struct ImGuiTextBuffer
{
    // ImVector<char>      Buf;
    pub Buf: String,
    //  static char EmptyString[1];
    pub EmptyString: [u8;1],

}

impl ImGuiTextBuffer {
    // ImGuiTextBuffer()   { }
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }

    // const char*         begin() const           { return Buf.data ? &Buf.front() : EmptyString; }
    pub fn begin(&self) -> *const u8 {
        &self.Buf[0]
    }
    // const char*         end() const             { return Buf.data ? &Buf.back() : EmptyString; }   // Buf is zero-terminated, so end() will point on the zero-terminator
    pub fn end(&self) -> *const u8 {
        &self.Buf[self.Buf.len()]
    }
    // int                 size() const            { return Buf.size ? Buf.size - 1 : 0; }
    pub fn size(&self) -> usize {
        self.Buf.len()
    }
    // bool                empty() const           { return Buf.size <= 1; }
    pub fn empty(&self) -> bool {
        self.Buf.is_empty()
    }
    // void                clear()                 { Buf.clear(); }
    pub fn clear(&mut self) {
        self.Buf.clear()
    }
    // void                reserve(int capacity)   { Buf.reserve(capacity); }
    pub fn reserve(&mut self, capacity: usize) {
        self.Buf.reserve(capacity)
    }
    // const char*         c_str() const           { return Buf.data ? Buf.data : EmptyString; }
    pub fn c_str(&self) -> *const u8 {
        self.Buf.as_ptr()
    }
    //  void      append(const char* str, const char* str_end = NULL);
    pub fn append(&mut self, start_str: *const u8, end_str: *const u8) {
        self.Buf.push_str(&String::from(start_str))
        //  int len = str_end ? (str_end - str) : strlen(str);
        //
        //     // Add zero-terminator the first time
        //     const int write_off = (Buf.size != 0) ? Buf.size : 1;
        //     const int needed_sz = write_off + len;
        //     if (write_off + len >= Buf.Capacity)
        //     {
        //         int new_capacity = Buf.Capacity * 2;
        //         Buf.reserve(needed_sz > new_capacity ? needed_sz : new_capacity);
        //     }
        //
        //     Buf.resize(needed_sz);
        //     memcpy(&Buf[write_off - 1], str, len);
        //     Buf[write_off - 1 + len] = 0;
    }
    //  void      appendf(const char* fmt, ...) IM_FMTARGS(2);
    //  void      appendfv(const char* fmt, va_list args) IM_FMTLIST(2);
}


// inline char         operator[](int i) const { IM_ASSERT(Buf.data != NULL); return Buf.data[i]; }
impl Index<i32> for ImGuiTextBuffer {
    type Output = u8;

    fn index(&self, index: i32) -> &Self::Output {
        self.Buf[index]
    }
}


// char ImGuiTextBuffer::EmptyString[1] = { 0 };

// void ImGuiTextBuffer::appendf(const char* fmt, ...)
// {
//     va_list args;
//     va_start(args, fmt);
//     appendfv(fmt, args);
//     va_end(args);
// }

// Helper: Text buffer for logging/accumulating text
// void ImGuiTextBuffer::appendfv(const char* fmt, va_list args)
// {
//     va_list args_copy;
//     va_copy(args_copy, args);
//
//     int len = ImFormatStringV(NULL, 0, fmt, args);         // FIXME-OPT: could do a first pass write attempt, likely successful on first pass.
//     if (len <= 0)
//     {
//         va_end(args_copy);
//         return;
//     }
//
//     // Add zero-terminator the first time
//     const int write_off = (Buf.size != 0) ? Buf.size : 1;
//     const int needed_sz = write_off + len;
//     if (write_off + len >= Buf.Capacity)
//     {
//         int new_capacity = Buf.Capacity * 2;
//         Buf.reserve(needed_sz > new_capacity ? needed_sz : new_capacity);
//     }
//
//     Buf.resize(needed_sz);
//     ImFormatStringV(&Buf[write_off - 1], len + 1, fmt, args_copy);
//     va_end(args_copy);
// }
