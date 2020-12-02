pub struct Permutations<T> {
    items: Vec<T>,
    size: usize,
    indexes: Vec<usize>,
    index: usize,
}

impl<T> Permutations<T> {
    pub fn new(items: Vec<T>, size: usize) -> Self {
        Permutations { indexes: vec![0; items.len()], items, size, index: 0 }
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.size {
            return None;
        }

        let current = self.items.clone();

        if self.index == self.size {
            self.index += 1;
            return Some(current);
        }

        if self.indexes[self.index] < self.index {
            let first_index = if self.index % 2 == 0 { 0 } else { self.indexes[self.index] };
            self.items.swap(first_index, self.index);
            self.indexes[self.index] += 1;
            self.index = 0;

            return Some(current);
        }

        self.indexes[self.index] = 0;
        self.index += 1;

        return self.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {

        assert_eq!(
            Permutations::new(vec!['a', 'b', 'c', 'd'], 4).collect::<Vec<_>>(),
            [
                ['a', 'b', 'c', 'd'],
                ['b', 'a', 'c', 'd'],
                ['c', 'a', 'b', 'd'],
                ['a', 'c', 'b', 'd'],
                ['b', 'c', 'a', 'd'],
                ['c', 'b', 'a', 'd'],
                ['d', 'b', 'a', 'c'],
                ['b', 'd', 'a', 'c'],
                ['a', 'd', 'b', 'c'],
                ['d', 'a', 'b', 'c'],
                ['b', 'a', 'd', 'c'],
                ['a', 'b', 'd', 'c'],
                ['a', 'c', 'd', 'b'],
                ['c', 'a', 'd', 'b'],
                ['d', 'a', 'c', 'b'],
                ['a', 'd', 'c', 'b'],
                ['c', 'd', 'a', 'b'],
                ['d', 'c', 'a', 'b'],
                ['d', 'c', 'b', 'a'],
                ['c', 'd', 'b', 'a'],
                ['b', 'd', 'c', 'a'],
                ['d', 'b', 'c', 'a'],
                ['c', 'b', 'd', 'a'],
                ['b', 'c', 'd', 'a'],
            ]
        );
    }
}
