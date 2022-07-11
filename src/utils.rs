use std::collections::HashSet;
use std::hash::Hash;

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
