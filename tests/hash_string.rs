#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        hash::{DefaultHasher, Hash, Hasher},
    };

    #[test]
    fn test_string_hashes() {
        let string_a = "Hello".to_string();
        let string_a_hash = hash_string(string_a);
        let string_b = "Hello".to_string();
        let string_b_hash = hash_string(string_b);

        assert_eq!(string_a_hash, string_b_hash);
    }

    #[test]
    fn test_maps() {
        let mut map_a = HashMap::new();
        map_a.insert("a", "b");
        map_a.insert("b", "a");
        let mut entries_a = map_a.iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>();
        entries_a.sort();
        let hash_a = hash_string(entries_a);
        let mut map_b = HashMap::new();
        map_b.insert("b", "a");
        map_b.insert("a", "b");
        let mut entries_b = map_b.iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>();
        entries_b.sort();
        let hash_b = hash_string(entries_b);

        assert_eq!(hash_a, hash_b);
    }

    fn hash_string(input: impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        hasher.finish()
    }
}
