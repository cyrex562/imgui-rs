use libc::{c_int, size_t};
use crate::type_defs::ImWchar;

// Helper to build glyph ranges from text/string data. Feed your application strings/characters to it then call BuildRanges().
// This is essentially a tightly packed of vector of 64k booleans = 8KB storage.
#[derive(Default, Debug, Clone, Copy)]
struct ImFontGlyphRangesBuilder {
    // Vec<u32> UsedChars;            // Store 1-bit per Unicode code point (0=unused, 1=used)
    pub UsedChars: Vec<u32>,
}

impl ImFontGlyphRangesBuilder {
    // ImFontGlyphRangesBuilder()              { Clear(); }


    // inline c_void     Clear()                 { let size_in_bytes: c_int = (IM_UNICODE_CODEPOINT_MAX + 1) / 8; UsedChars.resize(size_in_bytes / sizeof); memset(UsedChars.Data, 0, size_in_bytes); }
    pub unsafe fn Clear(&mut self) {
        let size_in_bytes: c_int = (IM_UNICODE_CODEPOINT_MAX + 1) / 8;
        // self.UsedChars.resize(size_in_bytes / sizeof);
        libc::memset(self.UsedChars.as_mut_ptr(), 0, size_in_bytes as size_t);
    }

    // inline bool     GetBit(size_t n) const  
    pub fn GetBit(&self, n: size_t) -> bool {
        let off: c_int = (n >> 5) as c_int;
        let mask = 1 << (n & 31);
        return (self.UsedChars[off] & mask) != 0;
    }
    // Get bit n in the array


    // inline c_void     SetBit(size_t n)       
    pub fn SetBit(&mut self, n: size_t) {
        let off: c_int = (n >> 5) as c_int;
        let mask = 1 << (n & 31);
        self.UsedChars[off] |= mask;
    }
    // Set bit n in the array


    // inline c_void     AddChar(ImWchar c)     
    pub fn AddChar(&mut self, c: ImWchar) { self.SetBit(c as size_t); }                      // Add character


    // c_void  AddText(text: *const c_char, *const char text_end = null_mut());     // Add string (each character of the UTF-8 string are added)


    // c_void  AddRanges(*const ImWchar ranges);                           // Add ranges, e.g. builder.AddRanges(ImFontAtlas::GetGlyphRangesDefault()) to force add all of ASCII/Latin+Ext


    // c_void  BuildRanges(out_ranges: &mut Vec<ImWchar>);                 // Output new ranges
}
