use std::{
    fs::{self},
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

mod db_utils;
use {db_utils::exists, db_utils::get_temp_full_path};

use redis::{
    db::DB,
    snapshot::{self},
    types::Value,
};

#[test]
fn test_snapshot() {
    let flag: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
    let database: Arc<RwLock<DB>> = Arc::new(RwLock::new(DB::new()));

    let test_key = "some key".to_string();
    let test_value = Value {
        value: "some value".to_string(),
        expires_at: None,
    };

    // let path = "/data/test_cache";
    // let full_path = get_full_path(path);
    let full_path = get_temp_full_path("temp_cache");

    // delete any existing test_cache
    let _ = fs::remove_file(&full_path);
    assert!(exists(&full_path) == false);

    // directly write to db
    database
        .write()
        .unwrap()
        .store
        .insert(test_key.clone(), test_value);

    // take the snapshot
    let f = flag.clone();
    snapshot::take_snapshot(f, database, &full_path);

    // check if take_snapshot is done
    let mut retries = 0;
    loop {
        retries += 1;
        thread::sleep(Duration::from_millis(100));
        let f = flag.read().unwrap();
        println!("is snapshot done?");
        if !*f {
            println!("yes.");
            break;
        }
        println!("no.");
        if retries > 3 {
            panic!("snapshot took too long")
        }
    }

    assert!(fs::exists(&full_path).is_ok());

    let restored_db = snapshot::deserialize(&full_path).unwrap();
    assert!(restored_db.store.get(&test_key).is_some());
}
