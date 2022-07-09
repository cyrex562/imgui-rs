// static inline bool      ImIsPowerOfTwo(int v)           { return v != 0 && (v & (v - 1)) == 0; }
pub fn is_power_of_two(v: i32) -> bool {
    v != 0 && (v & (v - 1)) == 0
}


// static inline bool      ImIsPowerOfTwo(ImU64 v)         { return v != 0 && (v & (v - 1)) == 0; }
pub fn is_power_of_two_u64(v: u64) -> bool {
    v != 0 && (v & (v - 1)) == 0
}


// static inline int       ImUpperPowerOfTwo(int v)        { v--; v |= v >> 1; v |= v >> 2; v |= v >> 4; v |= v >> 8; v |= v >> 16; v += 1; return v; }
pub fn upper_power_of_two(mut v: i32) -> i32 {
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v += 1;
    v
}
