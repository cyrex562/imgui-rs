use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::window::WindowFlags::Popup;

pub fn extend_hash_set<T: Eq + Hash+ Clone>(left: &mut HashSet<T>, right: &HashSet<T>){
    for x in right.iter() {
        left.insert(x.clone());
    }
}

pub fn set_hash_set<T: Eq + Hash + Clone>(left: &mut HashSet<T>, right: &HashSet<T>) {
    left.clear();
    left.clone_from(right);
}

pub fn remove_hash_set_val<T: Eq + Hash + Clone>(left: &mut HashSet<T>, right: &T) {
    if left.contains(right) {
        left.remove(right)
    }
}

pub fn add_hash_set<T: Eq + Hash + Clone>(left: &HashSet<T>, right: &HashSet<T>) -> HashSet<T> {
    let mut out: HashSet<T> = HashSet::new();
    for r in right.iter() {
        out.insert(r);
    }
    for l in left.iter() {
        out.insert(l);
    }
    out
}

pub fn sub_hash_set<T: Eq + Hash + Clone>(left: &HashSet<T>, right: &HashSet<T>) -> HashSet<T> {
    let mut out: HashSet<T> = HashSet::new();
    for l in left.iter() {
        out.insert(l);
    }
    for r in right.iter() {
        out.remove(r);
    }
    out
}

pub fn get_or_add<K, V: Default>(hash_map: &mut HashMap<K,V>, key: &K) -> &mut V {
    let mut out_opt: Option<&mut V> = hash_map.get_mut(key);
    if out_opt.is_none() {
        let new_out = V::default();
        hash_map[key] = new_out;
        out_opt = hash_map.get_mut(key);
    }
    out_opt.unwrap()
}

static void UnpackBitVectorToFlatIndexList(const ImBitVector* in, ImVector<int>* out)
{
    // IM_ASSERT(sizeof(in.Storage.data[0]) == sizeof);
    const ImU32* it_begin = in.Storage.begin();
    const ImU32* it_end = in.Storage.end();
    for (const ImU32* it = it_begin; it < it_end; it += 1)
        if (ImU32 entries_32 = *it)
            for (ImU32 bit_n = 0; bit_n < 32; bit_n += 1)
                if (entries_32 & (1 << bit_n))
                    out.push_back((((it - it_begin) << 5) + bit_n));
}


static void UnpackAccumulativeOffsetsIntoRanges(int base_codepoint, const short* accumulative_offsets, int accumulative_offsets_count, ImWchar* out_ranges)
{
    for (int n = 0; n < accumulative_offsets_count; n += 1, out_ranges += 2)
    {
        out_ranges[0] = out_ranges[1] = (ImWchar)(base_codepoint + accumulative_offsets[n]);
        base_codepoint += accumulative_offsets[n];
    }
    out_ranges[0] = 0;
}
