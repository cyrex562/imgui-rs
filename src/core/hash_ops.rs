#![allow(non_snake_case)]

use crate::core::type_defs::ImguiHandle;
use libc::{c_char, c_void};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Known size hash
// It is ok to call ImHashData on a string with known length but the ### operator won't be supported.
// ImguiHandle ImHashData(const void* data_p, data_size: size_t, seed: u32)
pub fn hash_data(data_p: &[u8], seed: u32) -> ImguiHandle {
    // // crc: u32 = ~seed;
    // let mut crc = !seed;
    // let mut data = data_p;
    // let crc32_lut = GCrc32LookupTable;
    // while data_size != 0 {
    //     crc = (crc >> 8) ^ crc32_lut[(crc & 0xF0f32) ^ *data];
    //     data += 1;
    //     data_size -= 1;
    // }
    // return !crc as ImguiHandle;
    let mut s = DefaultHasher::new();
    data_p.hash(&mut s);
    s.finish() as ImguiHandle
}

// Zero-terminated string hash, with support for ### to reset back to seed value
// We support a syntax of "label###id" where only "###id" is included in the hash, and only "label" gets displayed.
// Because this syntax is rarely used we are optimizing for the common case.
// - If we reach ### in the string we discard the hash so far and reset to the seed.
// - We don't do 'current += 2; continue;' after handling ### to keep the code smaller/faster (measured ~10% diff in Debug build)
// ImguiHandle ImHashStr(const char* data_p, data_size: size_t, seed: u32)
pub fn hash_string(data_p: &String, mut seed: u32) -> ImguiHandle {
    let mut s = DefaultHasher::new();
    data_p.hash(&mut s);
    s.finish() as ImguiHandle
    // seed = !seed;
    // let mut crc = seed;
    // let mut data = data_p;
    // let crc32_lut = GCrc32LookupTable;
    // if data_size != 0
    // {
    //     while data_size != 0
    //     {
    //         let c = *data;
    //         data += 1;
    //         if c == '#' as c_char && data_size >= 2 && data[0] == '#' && data[1] == '#' {
    //             crc = seed;
    //         }
    //         crc = (crc >> 8) ^ crc32_lut[(crc & 0xF0f32) ^ c];
    //         data_size -= 1;
    //     }
    // }
    // else
    // {
    //     let c = *data;
    //
    //     while c != 0
    //     {
    //         if c == '#' as c_char && data[0] == '#' && data[1] == '#' {
    //             crc = seed;
    //         }
    //         crc = (crc >> 8) ^ crc32_lut[(crc & 0xF0f32) ^ c];
    //         data += 1;
    //     }
    // }
    // return !crc as ImguiHandle;
}
