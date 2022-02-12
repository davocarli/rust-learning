// After learning about closures, I challenged myself to independentely create
// a cacher that will work for any type and store multiple results,
// by using a HashMap and generics.

use std::{collections::HashMap, hash::Hash};

pub struct Cacher<T, K, V>
where T: Fn(K) -> V,
{
    closure: T,
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V> 
where T: Fn(K) -> V,
      K: Eq + Hash + Clone,
      V: Clone,
{
    pub fn new(closure: T) -> Cacher<T, K, V> {
        Cacher { closure, values: HashMap::new() }
    }

    pub fn value(&mut self, arg: K) -> V {
        let value = self.values
            .entry(arg)
            .or_insert_with_key(|k| (&self.closure)(k.clone()));
        value.clone()
    }

    pub fn has_key(&self, arg: K) -> bool {
        self.values.contains_key(&arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn store_add_two() {
        let mut cache = Cacher::new(|num: u32| -> u32 {
            num + 2
        });

        let four = cache.value(2);
        // let five = cache.value(3);
        // let also_four = cache.value(2);

        assert_eq!(four, 4);
    }

    #[test]
    fn store_multiple() {
        let mut cache = Cacher::new(|num: u32| -> u32 {
            num + 2
        });

        let four = cache.value(2);
        let five = cache.value(3);
        let also_four = cache.value(2);

        assert_eq!([four, five, also_four], [4, 5, 4]);
    }

    #[test]
    fn check_for_key() {
        let mut cache = Cacher::new(|num: u32| -> u32 {
            num + 2
        });

        cache.value(2);

        assert!(cache.has_key(2));
    }

    #[test]
    fn check_missing_key() {
        let mut cache = Cacher::new(|num: u32| -> u32 {
            num + 2
        });

        cache.value(2);

        assert!(!cache.has_key(3));
    }
}
