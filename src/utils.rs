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
