#![allow(non_snake_case)]

// Helpers: Bit manipulation
// static inline bool      ImIsPowerOfTwo(c_int v)           
pub fn ImIsPowerOfTwo(y: c_int) -> bool
{ 
    return v != 0 && (v & (v - 1)) == 0; 
}



// static inline bool      ImIsPowerOfTwo(u64 v)         
pub fn ImIsPowerOfTwo2(v: u64) -> bool
{ 
    return v != 0 && (v & (v - 1)) == 0; 
}



// static inline c_int       ImUpperPowerOfTwo(c_int v)        
pub fn ImUpperPowerOfTwo(v: c_int) -> c_int
{ 
v-= 1; v |= v >> 1; v |= v >> 2; v |= v >> 4; v |= v >> 8; v |= v >> 16; v+= 1; return v;
}
