// Нужно реализовать структуру данных с интерфейсом insert, remove, get (как у коллекции Map), а также добавить поддержку версионирования (вспомни git) со следующим интерфейсом:

// Checkpoint - сохранить текущую версию;
// Rollback - откатить на определенную версию;
// Prune - забыть все версии кроме последней.

// use std::collections::HashMap;


use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::mem;

const INITIAL_Nstore: usize = 1;

type Entry<K, V> = Vec<(K, V)>;

pub struct Store<K, V>
   where
    K: PartialEq + Clone + Hash,
    V: Clone, 
{
    //   hash_fn: Box<dyn (Fn(&K) -> Option<usize>)>,
      store: Vec<Entry<K, V>>,
      pub length: usize,
}

// fn bucket<K, V, Q>(store: Box<[Entry<K, V>]>, key: &Q) -> Option<usize>
// where
//     K: PartialEq + Clone + Hash,
//     V: Clone, 
//     // K: Borrow<Q>,
//     Q: Hash + Eq + ?Sized + PartialEq + Clone,
// {
//     if store.is_empty() {
//         return None;
//     }
//     let mut hasher = DefaultHasher::new();
//     key.hash(&mut hasher);
//     Some((hasher.finish() % store.len() as u64) as usize)
// }

impl<K, V> Store<K, V> where K: PartialEq + Clone + Hash, V: Clone {

    pub fn new() -> Self {
        Store {
            store: Vec::new(),
            length: 0,
            // hash_fn: Box::new(bucket)
        }
    }

    fn hash<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + PartialEq,
    {
        if self.store.is_empty() {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        Some((hasher.finish() % self.store.len() as u64) as usize)
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.is_empty() {
            self.resize();
        }
        let idx = self.hash(&key).expect("Store is empty");
        // let h = (hashcode)(&key);
        // let idx = h & (self.store.len() - 1);
        match self.store[idx].iter().position(|e| e.0 == key) {
            Some(pos) => self.store[idx][pos] = (key, value),
            None => {
                self.store[idx].push((key, value));
                self.length += 1
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let idx = self.hash(key).expect("Store is empty");
        // let h = (self.hash_fn)(key); 
        // let idx = h &  (self.store.len() - 1);
        self.store[idx]
        .iter()
        .find(|e| e.0 == *key)
        .map(|e| e.1.clone())
    }
        
    pub fn remove(&mut self, key: K) -> Option<V> {
        let idx = self.hash(&key).expect("Store is empty");
        // let h = (self.hash_fn)(&key); 
        // let idx = h & (self.store.len() - 1);
        match self.store[idx].iter().position(|e| e.0 == key) {
            Some(pos) => {
                self.length -= 1;
                Some(self.store[idx].remove(pos).1)
            }
            _ => None,
        }
    }

    fn resize(&mut self) {
        let target_size = match self.store.len() {
            0 => INITIAL_Nstore,
            n => 2 * n,
        };

        let mut new_store = Vec::with_capacity(target_size);
        new_store.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.store.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % new_store.len() as u64) as usize;
            new_store[bucket].push((key, value));
        }

        mem::replace(&mut self.store, new_store);
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

}

fn main() {
    let mut store = Store::new();
    store.insert("test", 1);
    let v = store.get(&"test").unwrap();
    // let m = HashMap::new();
}