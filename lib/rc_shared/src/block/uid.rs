use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};
use std::ops::BitXor;
use std::process::id;
use fnv::{FnvBuildHasher, FnvHasher};
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::block::blocks::BlockUid;

// Calculates a hash from an identifier that is `const`
// Const is currently super limiting so it's pretty much no-std. Lame
pub fn hash_uid(identifier: &str) -> BlockUid {
    FnvBuildHasher::default().hash_one(identifier)
}

#[test]
fn test_uniqueness() {
    let mut map = HashSet::new();

    for i in 0..1_000_000 {
        let string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(9)
            .map(char::from)
            .collect();

        let hash = hash_uid(&string);

        assert!(!map.contains(&hash), "Collision on loop {}", i);

        map.insert(hash);
    }
}