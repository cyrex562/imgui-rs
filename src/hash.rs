use crate::types::Id32;
use fasthash::{murmur3};

pub fn hash_string(data: &str, seed: u32) -> Id32
{
    murmur3::hash32_with_seed(data, seed)
}

pub fn hash_data(data: &Vec<u8>, seed: u32) -> Id32 {
    murmur3::hash32_with_seed(data, seed)
}
