use std::collections::HashMap;

pub struct LRU<K, V, const N: usize> {
    data: HashMap<K, (V, usize)>,
    size: usize,
    clock_index: usize,
    clock: Vec<(K, bool)>
}

impl<K: Default + Clone + Eq + std::hash::Hash, V, const N: usize> LRU<K, V, N> {

    pub fn new() -> Self {
        LRU {
            data: HashMap::with_capacity(N),
            size: 0,
            clock_index: 0,
            clock: vec![(K::default(), false); N]
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        #[cfg(test)]
        self.check_valid();

        if let Some((value, clock_index)) = self.data.get_mut(key) {
            self.clock[*clock_index].1 = true;

            Some(value)
        }

        else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        #[cfg(test)]
        self.check_valid();

        if self.size < N {
            self.clock[self.size] = (key.clone(), false);
            self.data.insert(key, (value, self.clock_index));
            self.size += 1;
            self.clock_index = (self.clock_index + 1) % N;
        }

        else {
            self.evict();
            self.clock[self.clock_index] = (key.clone(), false);
            self.data.insert(key, (value, self.clock_index));
            self.size += 1;
            self.clock_index = (self.clock_index + 1) % N;
        }

    }

    pub fn evict(&mut self) {

        loop {
            if !self.clock[self.clock_index].1 {
                self.data.remove(&self.clock[self.clock_index].0);
                self.size -= 1;
                break;
            }

            else {
                self.clock[self.clock_index].1 = false;
                self.clock_index = (self.clock_index + 1) % N;
            }
        }

    }

    #[cfg(test)]
    pub fn check_valid(&self) {

        for (key, (value, clock_index)) in self.data.iter() {
            assert!(&self.clock[*clock_index].0 == key);
        }

        assert_eq!(self.data.len(), self.size);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lru_test() {
        let mut cache = LRU::<u32, u64, 8>::new();

        for i in 0..6 {
            cache.put(i, i as u64 * 2);
        }

        cache.get(&1).unwrap();
        cache.get(&2).unwrap();

        for i in 0..6 {
            cache.put(i + 100, i as u64 * 2 + 100);
        }

        assert!(cache.get(&104).is_some());
        assert!(cache.get(&105).is_some());
        assert!(cache.get(&1).is_some());
        assert!(cache.get(&2).is_some());
        assert!(cache.get(&3).is_none());
        assert!(cache.get(&4).is_none());

        for i in 0..6 {
            cache.put(i + 200, i as u64 * 2 + 200);
        }

        assert!(cache.get(&1).is_some());
        assert!(cache.get(&2).is_some());
    }

}