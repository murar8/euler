use std::{collections::HashSet, hash::Hash};

pub trait Duplicates {
    fn contains_duplicates(self) -> bool;
}

impl<T: Hash + Eq> Duplicates for &[T] {
    fn contains_duplicates(self) -> bool {
        let mut items = HashSet::new();

        for v in self.iter() {
            if items.contains(v) {
                return true;
            } else {
                items.insert(v);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicates() {
        assert_eq!((&[] as &[u64]).contains_duplicates(), false);
        assert_eq!((&[1]).contains_duplicates(), false);
        assert_eq!((&[1, 2, 3, 4]).contains_duplicates(), false);
        assert_eq!((&[4, 2, 3, 4]).contains_duplicates(), true);
        assert_eq!((&["a", "b", "b", "b"]).contains_duplicates(), true);
    }
}
