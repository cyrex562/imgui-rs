use std::collections::HashSet;
use std::hash::Hash;

pub fn extend_hash_set<T: Eq + Hash+ Clone>(left: &mut HashSet<T>, right: &HashSet<T>){
    for x in right.iter() {
        left.insert(x.clone());
    }
}
