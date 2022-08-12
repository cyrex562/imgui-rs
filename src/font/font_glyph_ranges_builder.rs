// Helper to build glyph ranges from text/string data. Feed your application strings/characters to it then call build_ranges().
// This is essentially a tightly packed of vector of 64k booleans = 8KB storage.
#[derive(Clone,Debug,Default)]
pub struct FontGlyphRangesBuilder
{
    pub used_chars: Vec<u32>, //ImVector<ImU32> used_chars;            // Store 1-bit per Unicode code point (0=unused, 1=used)


}

impl FontGlyphRangesBuilder {
    // ImFontGlyphRangesBuilder()              { clear(); }
    //     inline void     clear()                 { int size_in_bytes = (IM_UNICODE_CODEPOINT_MAX + 1) / 8; used_chars.resize(size_in_bytes / sizeof); memset(used_chars.data, 0, (size_t)size_in_bytes); }
    pub fn clear(&mut self) {
        self.used_chars.clear()
    }
    //     inline bool     get_bit(size_t n) const  { int off = (n >> 5); ImU32 mask = 1u << (n & 31); return (used_chars[off] & mask) != 0; }  // Get bit n in the array
    pub fn get_bit(&mut self, n: usize) -> bool {
        let off = n >> 5;
        let mask: u32 = 1 << (n * 31);
        self.used_chars[off] & mask != 0
    }
    //     inline void     set_bit(size_t n)        { int off = (n >> 5); ImU32 mask = 1u << (n & 31); used_chars[off] |= mask; }               // Set bit n in the array
    pub fn set_bit(&mut self, n: usize) {
        let off = n >> 5;
        let mask: u32 = 1 << (n & 31);
        self.used_chars[off] |= mask;
    }
    //     inline void     add_char(ImWchar c)      { set_bit(c); }                      // Add character
    pub fn add_char(&mut self, c: u8) {
        self.set_bit(c as usize)
    }
    //      void  add_text(const char* text, const char* text_end = None);     // Add string (each character of the UTF-8 string are added)
    pub fn add_text(&mut self, text: &String) {
        todo!()
    }
    //      void  add_ranges(const ImWchar* ranges);                           // Add ranges, e.g. builder.add_ranges(ImFontAtlas::get_glyph_ranges_default()) to force add all of ASCII/Latin+Ext
    pub fn add_ranges(&mut self, ranges: &[char]) {
        todo!()
    }
    //      void  build_ranges(ImVector<ImWchar>* out_ranges);                 // Output new ranges
    pub fn build_ranges(&mut self, out_ranges: &mut Vec<char>) {
        todo!()
    }
}
