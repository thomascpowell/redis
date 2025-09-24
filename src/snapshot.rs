use crate::db::DB;
use std::{
    sync::{Arc, RwLock},
    thread,
};

pub fn take_snapshot(flag: Arc<RwLock<bool>>, db: Arc<RwLock<DB>>) {
    thread::spawn(move || {
        let _ = db.read().unwrap().store.clone();
        // does nothing atm
        println!("taking snapshot (not really)");
        *flag.write().unwrap() = false;
    });
    return;
}
