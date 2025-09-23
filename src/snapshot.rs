use crate::db::DB;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn take_snapshot(flag: Arc<Mutex<bool>>, db: Arc<Mutex<DB>>) {
    thread::spawn(move || {
        let _ = db.lock().unwrap().store.clone();
        // does nothing atm
        *flag.lock().unwrap() = false;
    });
    return;
}
