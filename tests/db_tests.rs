use db_utils::*;
use redis::{db::DB, types::RESPValue};
use std::{thread::sleep, time::Duration};

mod db_utils;

#[test]
fn test_set_get_del() {
    let mut db = DB::new();
    let set = "SET test test".to_string();
    let get = "GET test".to_string();
    let del = "DEL test".to_string();

    // SET
    db.process(&get_job_request(&set));
    assert_eq!(db.store.len(), 1);
    assert_eq!(
        db.process(&get_job_request(&get)).value, // process a test JobRequest
        get_bulk_res("test")                      // compare it to a test RESP string
    );

    // DEL
    db.process(&get_job_request(&del));
    assert_eq!(db.store.len(), 0);
    assert_eq!(db.process(&get_job_request(&get)).value, get_nil_res());
}

#[test]
fn test_ttl() {
    let mut db = DB::new();
    let set = "SETEX test 0 test".to_string();
    let get = "GET test".to_string();

    // SET
    db.process(&get_job_request(&set));

    // GET after TTL
    sleep(Duration::from_millis(1));
    assert_ne!(
        db.process(&get_job_request(&get)).value,
        get_bulk_res("test")
    );
}

#[test]
fn test_expire() {
    let mut db = DB::new();
    let set = "SET test test".to_string();
    let get = "GET test".to_string();
    let expire = "EXPIRE test 0".to_string();

    // other tests cover basic features
    // this is just to test using expire on an already expired key

    let cmd1 = &get_job_request(&set);
    let cmd2 = &get_job_request(&get);
    let cmd3 = &get_job_request(&expire);
    db.process(cmd1);

    // add expiration to active key
    assert_eq!(db.process(cmd3).value, get_int_res(1));
    // attempt to access expired
    assert_eq!(db.process(cmd2).value, get_nil_res());
    // attempt to expire expired
    assert_eq!(db.process(cmd3).value, get_int_res(0));
}

#[test]
fn test_incr_decr() {
    let mut db = DB::new();
    let incr = "INCR test".to_string();
    let decr = "DECR test".to_string();
    let ttl = "EXPIRE test 0".to_string();

    // INCR, test should be 1
    let cmd1 = &get_job_request(&incr);
    assert_eq!(db.process(cmd1).value, get_int_res(1));

    // DECR 2x, test should be -1
    let cmd2 = &get_job_request(&decr);
    db.process(cmd2);
    assert_eq!(db.process(cmd2).value, get_int_res(-1));

    // ADD TTL, then INCR. should reset to 0 -> 1
    let cmd3 = &get_job_request(&ttl);
    let cmd4 = &get_job_request(&incr);
    db.process(cmd3);
    assert_eq!(db.process(cmd4).value, get_int_res(1));
}

#[test]
fn test_invalid_incr_decr() {
    let mut db = DB::new();
    let incr = "INCR test".to_string();
    let set = "SET test impossible".to_string();
    let cmd1 = &get_job_request(&set);
    let cmd2 = &get_job_request(&incr);

    db.process(cmd1);
    // INCR should error
    assert!(db.process(cmd2).value.starts_with("-"),);
}

#[test]
fn test_append() {
    let mut db = DB::new();
    let set = "set test ha".to_string();
    let append = "append test lf".to_string();
    let get = "get test".to_string();
    let cmd1 = &get_job_request(&set);
    let cmd2 = &get_job_request(&append);
    let cmd3 = &get_job_request(&get);
    db.process(cmd1);
    assert_eq!(db.process(cmd2).value, get_int_res(4));
    assert_eq!(db.process(cmd3).value, get_bulk_res("half"));
}
