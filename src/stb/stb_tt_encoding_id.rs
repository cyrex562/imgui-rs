use libc::c_int;

// enum { // encodingID for STBTT_PLATFORM_ID_UNICODE
pub const STBTT_UNICODE_EID_UNICODE_1_0: c_int = 0;
pub const STBTT_UNICODE_EID_UNICODE_1_1: c_int = 1;
pub const STBTT_UNICODE_EID_ISO_10646: c_int = 2;
pub const STBTT_UNICODE_EID_UNICODE_2_0_BMP: c_int = 3;
pub const STBTT_UNICODE_EID_UNICODE_2_0_FULL: c_int = 4;
// };

// enum { // encodingID for STBTT_PLATFORM_ID_MICROSOFT
pub const STBTT_MS_EID_SYMBOL: c_int = 0;
pub const STBTT_MS_EID_UNICODE_BMP: c_int = 1;
pub const STBTT_MS_EID_SHIFTJIS: c_int = 2;
pub const STBTT_MS_EID_UNICODE_FULL: c_int = 10;
// };

// enum { // encodingID for STBTT_PLATFORM_ID_MAC; same as Script Manager codes
pub const STBTT_MAC_EID_ROMAN: c_int = 0;
pub const STBTT_MAC_EID_ARABIC: c_int = 4;
pub const STBTT_MAC_EID_JAPANESE: c_int = 1;
pub const STBTT_MAC_EID_HEBREW: c_int = 5;
pub const STBTT_MAC_EID_CHINESE_TRAD: c_int = 2;
pub const STBTT_MAC_EID_GREEK: c_int = 6;
pub const STBTT_MAC_EID_KOREAN: c_int = 3;
pub const STBTT_MAC_EID_RUSSIAN: c_int = 7;
// };
