use std::{collections::HashMap, hash::Hash};

pub fn reverse_hash_map<T, E>(hash_map: &HashMap<T, E>) -> HashMap<E, T>
where
    T: Eq,
    T: Hash,
    T: Copy,
    E: Eq,
    E: Hash,
    E: Copy,
{
    let mut reversed_map: HashMap<E, T> = HashMap::new();

    for key in hash_map.keys() {
        let value = hash_map.get(key).unwrap();

        reversed_map.insert(*value, *key);
    }

    return reversed_map;
}
