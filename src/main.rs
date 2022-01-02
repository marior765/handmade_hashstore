mod store;
use store::{Store, Core};


// fn test_prune() {
//     let ver_arr = (1..=10).collect::<Vec<_>>();
//     let mut s: Store<String, i32> = Store::new();

//     for ver in ver_arr.clone() {
//         s.mutate_observable(ver);
//         s.checkpoint();
//     }

//     s.prune();

//     let last_value = ver_arr.last().unwrap();

//     let numeric_head = 0.1;
//     let new_ver = numeric_head * (*last_value as f32);
//     let stringify = format!("{:.1}", new_ver);

//     assert_eq!(s.head, Some(stringify));
//     assert_eq!(s.observable, Some(last_value));
//     assert_eq!(s.ver_list(), 1);
// }

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
    store.mutate_observable(1000);
    store.checkpoint();
    store.ver_list();
    store.rollback("0.3".to_string());
    store.ver_list();
    println!("Rollbacked to 0.3!");
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
    store.mutate_observable(1000);
    store.checkpoint();
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
    store.mutate_observable(1000);
    store.checkpoint();
    store.ver_list();
}