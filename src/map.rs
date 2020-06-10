use std::collections::HashMap;
use std::hash::Hash;
use rand::{Rng, thread_rng};

pub type FrequencyMap<T> = HashMap<T, usize>;

pub trait Hashable: Eq + Hash {}

impl<T> Hashable for T where T: Eq + Hash {}

pub(crate) trait FrequencyMapOps<T: Hashable> {
    fn incr(&mut self, key: T);
    fn count(&self) -> usize;
}

impl<T> FrequencyMapOps<T> for FrequencyMap<T> where T: Hashable {
    fn incr(&mut self, key: T) {
        let count = self.entry(key).or_insert(0);
        *count += 1;
    }

    fn count(&self) -> usize {
        self.values().sum()
    }
}

pub trait MapRandom<T> {
    fn choose_weighted(&self) -> MapRandomResult<&T>;
}

#[derive(Debug, PartialEq)]
pub enum MapRandomError {
    EmptyMap,
    Unknown
}

pub type MapRandomResult<T> = Result<T, MapRandomError>;

impl<T> MapRandom<T> for FrequencyMap<T> where T: Hashable{
    fn choose_weighted(&self) -> MapRandomResult<&T> {
        let mut rng = thread_rng();
        let total_tokens: usize = self.count();

        if self.len() == 0 {
            return Err(MapRandomError::EmptyMap);
        }

        let chosen_token = rng.gen_range(0, total_tokens + 1);
        let mut sum = 0;
        for (key, value) in self.iter() {
            sum += *value;
            if sum >= chosen_token {
                return Ok(key);
            }
        }

        return Err(MapRandomError::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incr() {
        let mut map = FrequencyMap::new();

        // it should add if key does not exist
        assert!(!map.contains_key("a"));
        map.incr("a");
        assert_eq!(map.get("a"), Some(&1));

        // it should increment if key exists
        map.incr("a");
        assert_eq!(map.get("a"), Some(&2));

        // and keep on incrementing...
        for _ in 0..10 {
            map.incr("a");
        }
        assert_eq!(map.get("a"), Some(&12));

        // it should not have recollection of previous value
        map.remove("a");
        map.incr("a");
        assert_eq!(map.get("a"), Some(&1));
    }

    #[test]
    fn test_count() {
        let mut map = FrequencyMap::new();

        // empty map's count is 0
        assert_eq!(map.count(), 0);

        // insert first 10 natural numbers as values
        // set keys to something different to differentiate
        // that its summing values not keys
        for (k, v) in (101..=110).zip(1..=10) {
            map.insert(k,v);
        }
        assert_eq!(map.count(), 55)
    }

    #[test]
    fn test_choose_weighted() {
        // Err on empty map
        let map: FrequencyMap<&str> = FrequencyMap::new();
        assert_eq!(map.choose_weighted().err(), Some(MapRandomError::EmptyMap));

        // Otherwise return a value.
        let map: FrequencyMap<&str> = vec![
            ("one", 1),
            ("two", 2)
        ].into_iter().collect();
        assert!(map.choose_weighted().is_ok());
    }
}
