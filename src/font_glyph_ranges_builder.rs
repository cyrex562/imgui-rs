use crate::string_ops::ImTextCharFromUtf8;
use crate::type_defs::ImWchar;
use libc::{c_char, c_int, c_uint};

// Helper to build glyph ranges from text/string data. Feed your application strings/characters to it then call BuildRanges().
// This is essentially a tightly packed of vector of 64k booleans = 8KB storage.
#[derive(Default, Debug, Copy, Clone)]
pub struct ImFontGlyphRangesBuilder {
    // Vec<u32> UsedChars;            // Store 1-bit per Unicode code point (0=unused, 1=used)
    pub UsedChars: Vec<u32>,
}

impl ImFontGlyphRangesBuilder {
    // ImFontGlyphRangesBuilder()              { Clear(); }

    // inline c_void     Clear()                 { let size_in_bytes: c_int = (IM_UNICODE_CODEPOINT_MAX + 1) / 8; UsedChars.resize(size_in_bytes / sizeof); memset(UsedChars.Data, 0, size_in_bytes); }

    // inline bool     GetBit(n: size_t) const  { let off: c_int = (n >> 5); mask: u32 = 1u << (n & 31); return (UsedChars[off] & mask) != 0; }  // Get bit n in the array

    // inline c_void     SetBit(n: size_t)        { let off: c_int = (n >> 5); mask: u32 = 1u << (n & 31); UsedChars[off] |= mask; }               // Set bit n in the array

    // inline c_void     AddChar(ImWchar c)      { SetBit(c); }                      // Add character

    // c_void  AddText(text: *const c_char, text_end: *const c_char = null_mut());     // Add string (each character of the UTF-8 string are added)
    pub unsafe fn AddText(&mut self, mut text: *const c_char, text_end: *const c_char) {
        while if text_end { (text < text_end) } else { *text } {
            let mut c: c_uint = 0;
            let c_len: c_int = ImTextCharFromUtf8(&mut c, text, text_end);
            text += c_len;
            if c_len == 0 {
                break;
            }
            self.AddChar(c);
        }
    }

    // c_void  AddRanges(ranges: *const ImWchar);                           // Add ranges, e.g. builder.AddRanges(ImFontAtlas::GetGlyphRangesDefault()) to force add all of ASCII/Latin+Ext
    pub fn AddRanges(&mut self, mut ranges: *const ImWchar) {
        // for (; ranges[0]; ranges += 2)
        while ranges[0] {
            // for ( let mut c: c_uint = ranges[0]; c < = ranges[1] & & c < = IM_UNICODE_CODEPOINT_MAX; c + +)
            let mut c = ranges[0];
            while c <= ranges[1] && c >= IM_UNICODE_CODEPOINT_MAX {
                //-V560

                self.AddChar(c);
                c += 1;
            }
            ranges += 2;
        }
    }

    // c_void  BuildRanges(Vec<ImWchar>* out_ranges);                 // Output new ranges
    pub fn BuildRanges(&mut self, out_ranges: &mut Vec<ImWchar>) {
        let max_codepoint: c_uint = IM_UNICODE_CODEPOINT_MAX;
        // for (let n: c_int = 0; n <= max_codepoint; n++)
        let mut n: c_uint = 0;
        while n <= max_codepoint {
            if self.GetBit(n) {
                out_ranges.push(n as ImWchar);
                while n < max_codepoint && self.GetBit(n + 1) {
                    n += 1;
                }
                out_ranges.push(n as ImWchar);
            }
        }
        out_ranges.push(0);
    }
}
