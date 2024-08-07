use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct BTreeCountMap<K: Ord> {
    map: BTreeMap<K, usize>,
}

impl<K: Ord + Clone> BTreeCountMap<K> {
    pub fn new() -> Self {
        BTreeCountMap {
            map: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn insert_one(&mut self, key: K) {
        self.insert_n(key, 1);
    }

    pub fn insert_n(&mut self, key: K, n: usize) {
        *self.map.entry(key).or_insert(0) += n;
    }

    #[inline]
    pub fn remove_one(&mut self, key: &K) {
        self.remove_n(key, 1);
    }

    pub fn remove_n(&mut self, key: &K, n: usize) {
        if let Some(count) = self.map.get_mut(key) {
            *count = count.saturating_sub(n);
            if *count == 0 {
                self.map.remove(key);
            }
        }
    }

    pub fn remove_all(&mut self, key: &K) {
        self.map.remove(key);
    }

    pub fn get(&self, key: &K) -> usize {
        *self.map.get(key).unwrap_or(&0)
    }

    pub fn get_mut(&mut self, key: &K) -> &mut usize {
        self.map.entry(key.clone()).or_insert(0)
    }

    pub fn iter(&self) -> impl Iterator<Item=(&K, &usize)> {
        self.map.iter()
    }

    pub fn merge(&mut self, other: &Self) {
        for (key, count) in other.iter() {
            *self.map.entry(key.clone()).or_insert(0) += count;
        }
    }

    pub fn first_key(&self) -> Option<&K> {
        self.map.keys().next()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key) && self.map.get(key).unwrap() > &0
    }

    pub fn subtract(&mut self, other: &Self) {
        for (key, count) in other.iter() {
            if let Some(c) = self.map.get_mut(key) {
                *c = c.saturating_sub(*count);
                if *c == 0 {
                    self.map.remove(key);
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn key_len(&self) -> usize {
        self.map.len()
    }

    pub fn count_len(&self) -> usize {
        self.map.values().sum()
    }
}

impl<K: Ord + Clone> FromIterator<(K, usize)> for BTreeCountMap<K> {
    fn from_iter<T: IntoIterator<Item=(K, usize)>>(iter: T) -> Self {
        let mut map = BTreeCountMap::new();
        for (key, count) in iter {
            map.insert_n(key, count);
        }
        map
    }
}

impl<K: Ord + Clone> FromIterator<K> for BTreeCountMap<K> {
    fn from_iter<T: IntoIterator<Item=K>>(iter: T) -> Self {
        let mut map = BTreeCountMap::new();
        for key in iter {
            map.insert_one(key);
        }
        map
    }
}