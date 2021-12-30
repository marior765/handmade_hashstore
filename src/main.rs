// // Нужно реализовать структуру данных с интерфейсом insert, remove, get (как у коллекции Map), а также добавить поддержку версионирования (вспомни git) со следующим интерфейсом:

// // Checkpoint - сохранить текущую версию;
// // Rollback - откатить на определенную версию;
// // Prune - забыть все версии кроме последней.

use std::collections::HashMap;
use std::hash::{Hash};
use std::mem;
use std::fmt::Display;
// use std::cmp::Ordering;

const initial_ver: &'static str = "0.1";

pub struct Store<K, V> 
        where for<'a> K: Hash + Eq + Clone + PartialOrd + Display, 
            V: Clone {
    map: HashMap<K, V>,
    head: Option<K>,
    observable: Option<V>
}

pub trait Core<K> 
        where for<'a> K: Hash + Eq + Clone + PartialOrd + Display {
    fn checkpoint(&mut self);
    fn rollback(&mut self, key: K);
    fn prune(&mut self);
}

impl<K, V> Store<K, V> 
    where for<'a> K: Hash + Eq + Clone + PartialOrd + Display + PartialEq + From<String>,
        V: Clone
{
    fn new() -> Self {
        Store {
            map:  HashMap::new(),
            head: None,
            observable: None,
        }
    }

    fn increase_ver(&mut self) -> K {
        if self.head == None {
            let new_ver = K::from(initial_ver.to_string());
            self.head = Some(new_ver);
        } else {
            let current_head = self.head.as_ref().unwrap();
            let numeric_head = current_head.to_string().parse::<f32>().unwrap_or(0.0);
            let new_ver = (numeric_head + 0.1).to_string();
            self.head = Some(K::from(new_ver));
        }
        self.head.clone().unwrap()
    }

    fn ver_list(&self) {
        for v in self.map.keys() {
            if Some(v) == self.head.as_ref() {
                println!("* {}", v);
                continue;
            }
            println!("{}", v);
        }
        println!("--\\--");
    }

    fn mutate_observable(&mut self, v: V) {
        self.observable = Some(v);
    }

}

impl<K, V> Core<K> for Store<K, V> 
    where for<'a> K: Hash + Eq + Clone + PartialOrd + Display + From<String>, 
        V: Clone
{
    fn checkpoint(&mut self) {
        let current_value = mem::replace(&mut self.observable, None);
        if let Some(value) = current_value {
            let ver = self.increase_ver();
            self.map.insert(ver, value);
        } else {
            println!("Nothing to checkpoint!");
        }
    }

    fn rollback(&mut self, rollback_key: K) {
        if self.map.contains_key(&rollback_key) {
            let new_list: HashMap<_, _> = self.map
                    .clone()
                    .into_iter()
                    .filter(|(k, _)| k <= &rollback_key)
                    .collect();
            let raw_value = self.map.get(&rollback_key).unwrap();
            self.head = Some(rollback_key);
            self.observable = Some(raw_value.clone());
            self.map = new_list;
            // mem::replace(&mut self.head, Some(rollback_key));
            // mem::replace(&mut self.observable, Some(raw_value.clone()));
            // self.map.keys().all(|k| k < rollback_key);
        } else {
            println!("Version {} doesnt exist!", rollback_key);
        }
    }

    fn prune(&mut self) {
        if self.map.len() < 2 {
            println!("Nothing to prune!");
            ()
        }
        let mut new_map: HashMap<K, V> = HashMap::new();
        let last_key = self.head.clone().unwrap();
        let last_value = self.map.get(&last_key).unwrap();
        new_map.insert(last_key, last_value.clone());
        self.map = new_map;
    }

}

fn main() {
    let mut store: Store<String, i32> = Store::new();
    store.mutate_observable(1);
    store.checkpoint();
    store.ver_list();
    println!("Saved first version!");
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.ver_list();
    store.rollback("0.2".to_string());
    store.ver_list();
    println!("Rollbacked to 0.2!");
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.ver_list();
    store.prune();
    store.ver_list();
    println!("Pruned to the last one!");
}