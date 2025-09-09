use std::{thread::sleep, time::Duration};
use redis::DB;
use utils::*;

mod utils;

#[test]
fn test_basic_operations() {
    let mut db = DB::new();
    let set: String = "SET test test".to_string();
    let get: String = "GET test".to_string();
    let del: String = "DEL test".to_string();

    // SET
    db.process(set);
    assert_eq!(db.store.len(), 1);
    assert_eq!(db.process(get), get_simple_res("test"));

    // DEL
    db.process(del);
    let get: String = "GET test".to_string();
    assert_eq!(db.store.len(), 0);
    assert_eq!(db.process(get), get_nil_res());
}

#[test]
fn test_ttl() {
    let mut db = DB::new();
    let set: String = "SET test test 1".to_string();
    let get: String = "GET test".to_string();

    // SET
    db.process(set);

    // GET
    assert_eq!(db.process(get), get_simple_res("test"));

    // GET after TTL
    sleep(Duration::new(1, 1));
    let get: String = "GET test".to_string();
    assert!(db.process(get) != get_simple_res("test"));
}
