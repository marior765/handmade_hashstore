#[cfg(test)]
mod tests {
    use rtest::store::*;

    #[test]
    fn test_init() {
        type ST = Store<String, i32>;

        let s: ST = ST::new();
        assert_eq!(s.head, None);
        assert_eq!(s.observable, None);
        assert_eq!(s.ver_list(), 0);

        let s: ST = ST::default();
        assert_eq!(s.head, None);
        assert_eq!(s.observable, None);
        assert_eq!(s.ver_list(), 0);
    }

    #[test]
    fn test_checpoint() {
        let mutation = 1;
        let mut s: Store<String, i32> = Store::new();

        s.checkpoint();
        assert_eq!(s.head, None);
        assert_eq!(s.observable, None);
        assert_eq!(s.ver_list(), 0);

        s.mutate_observable(mutation);
        assert_eq!(s.head, None);
        assert_eq!(s.observable, Some(mutation));
        assert_eq!(s.ver_list(), 0);

        s.checkpoint();
        assert_eq!(s.head, Some("0.1".to_string()));
        assert_eq!(s.observable, None);
        assert_eq!(s.ver_list(), 1);
    }

    #[test]
    fn test_rollback() {
        let ver_arr = (1..=10).collect::<Vec<_>>();
        let mut s: Store<String, i32> = Store::new();
    
        for ver in ver_arr.clone() {
            s.mutate_observable(ver);
            s.checkpoint();
        }
    
        let numeric_head = 0.1;
        let new_ver = numeric_head * ver_arr.len() as f32;
        let stringify = format!("{:.1}", new_ver);
    
        assert_eq!(s.head, Some(stringify));
        assert_eq!(s.observable, None);
        assert_eq!(s.ver_list(), ver_arr.len());
    
        let observable_index = 5;
        let possible_rollback = 0.1 * ver_arr[observable_index] as f32;
        let possible_rollback_str = possible_rollback.to_string();
    
        s.rollback(possible_rollback_str.clone());
    
        assert_eq!(s.head, Some(possible_rollback_str));
        assert_eq!(s.observable, Some(ver_arr[observable_index]));
        assert_eq!(s.ver_list(), observable_index + 1);
    }

    #[test]
    fn test_prune() {
        let ver_arr = (1..=10).collect::<Vec<_>>();
        let mut s: Store<String, i32> = Store::new();
    
        for ver in ver_arr.clone() {
            s.mutate_observable(ver);
            s.checkpoint();
        }
    
        s.prune();
    
        let last_value = ver_arr.last().unwrap();
    
        let numeric_head = 0.1;
        let new_ver = numeric_head * (*last_value as f32);
        let stringify = format!("{:.1}", new_ver);
    
        assert_eq!(s.head, Some(stringify));
        assert_eq!(s.observable, Some(*last_value));
        assert_eq!(s.ver_list(), 1);
    }
}