use redis::DB;
use std::{thread::sleep, time::Duration};
use utils::*;

mod utils;

#[test]
fn test_set_get_del() {
    let mut db = DB::new();
    let set = "SET test test";
    let get = "GET test";
    let del = "DEL test";

    // SET
    db.process(get_test_job_request(set));
    assert_eq!(db.store.len(), 1);
    assert_eq!(
        db.process(get_test_job_request(get)).value, // process a test JobRequest
        get_simple_res("test") // compare it to a test RESP string
    );

    // DEL
    db.process(get_test_job_request(del));
    assert_eq!(db.store.len(), 0);
    assert_eq!(db.process(get_test_job_request(get)).value, get_nil_res());
}

#[test]
fn test_ttl() {
    let mut db = DB::new();
    let set = "SET test test 1";
    let get = "GET test";

    // SET
    db.process(get_test_job_request(set));

    // GET
    assert_eq!(
        db.process(get_test_job_request(get)).value,
        get_simple_res("test")
    );

    // GET after TTL
    sleep(Duration::new(1, 1));
    assert_ne!(
        db.process(get_test_job_request(get)).value,
        get_simple_res("test")
    );
}
