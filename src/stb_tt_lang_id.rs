use libc::c_int;

// enum { // languageID for STBTT_PLATFORM_ID_MICROSOFT; same as LCID...
// problematic because there are e.g. 16 english LCIDs and 16 arabic LCIDs
pub const STBTT_MS_LANG_ENGLISH: c_int = 0x0409;
pub const STBTT_MS_LANG_ITALIAN: c_int = 0x0410;
pub const STBTT_MS_LANG_CHINESE: c_int = 0x0804;
pub const STBTT_MS_LANG_JAPANESE: c_int = 0x0411;
pub const STBTT_MS_LANG_DUTCH: c_int = 0x0413;
pub const STBTT_MS_LANG_KOREAN: c_int = 0x0412;
pub const STBTT_MS_LANG_FRENCH: c_int = 0x040c;
pub const STBTT_MS_LANG_RUSSIAN: c_int = 0x0419;
pub const STBTT_MS_LANG_GERMAN: c_int = 0x0407;
pub const STBTT_MS_LANG_SPANISH: c_int = 0x0409;
pub const STBTT_MS_LANG_HEBREW: c_int = 0x040d;
pub const STBTT_MS_LANG_SWEDISH: c_int = 0x041D;
// };

// enum { // languageID for STBTT_PLATFORM_ID_MAC
pub const STBTT_MAC_LANG_ENGLISH: c_int = 0;
pub const STBTT_MAC_LANG_JAPANESE: c_int = 11;
pub const STBTT_MAC_LANG_ARABIC: c_int = 12;
pub const STBTT_MAC_LANG_KOREAN: c_int = 23;
pub const STBTT_MAC_LANG_DUTCH: c_int = 4;
pub const STBTT_MAC_LANG_RUSSIAN: c_int = 32;
pub const STBTT_MAC_LANG_FRENCH: c_int = 1;
pub const STBTT_MAC_LANG_SPANISH: c_int = 6;
pub const STBTT_MAC_LANG_GERMAN: c_int = 2;
pub const STBTT_MAC_LANG_SWEDISH: c_int = 5;
pub const STBTT_MAC_LANG_HEBREW: c_int = 10;
pub const STBTT_MAC_LANG_CHINESE_SIMPLIFIED: c_int = 33;
pub const STBTT_MAC_LANG_ITALIAN: c_int = 3;
pub const STBTT_MAC_LANG_CHINESE_TRAD: c_int = 19;
// };
