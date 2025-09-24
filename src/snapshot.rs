use crate::{db::DB, types::Value};
use std::{
    fs, sync::{Arc, RwLock}, thread, time::Instant
};

// serializes in format:
// 4 byte key length
// key (string)
//
// 4 byte value length
// Value.value (string)
//
// value of length l
// 4 byte ttl (in seconds)


pub fn take_snapshot(flag: Arc<RwLock<bool>>, db: Arc<RwLock<DB>>) {
    thread::spawn(move || {
        let snapshot = db.read().unwrap();
        let mut buf: Vec<u8> = Vec::new();
        buf.extend(&(snapshot.store.len() as u32).to_le_bytes());
        for (k, v) in &snapshot.store {
            serialize_string(&mut buf, k);
            serialize_value(&mut buf, v);
        }
        let res = fs::write("./.fake_rdb", buf);
        if let Err(e) = res {
            eprintln!("snapshot error: {:?}", e);
        }
        *flag.write().unwrap() = false;
    });
}

// probably worse for performance than passing a buf
pub fn serialize_string(buf: &mut Vec<u8>, s: &str) {
    buf.extend(&(s.len() as u32).to_le_bytes()); // 4-byte length prefix
    buf.extend(s.as_bytes());
}

pub fn serialize_value(buf: &mut Vec<u8>, v: &Value) {
    // can only store 132 years before overflow
    // do not use this database if you live beyond mortal timescales
    let ttl = match v.expires_at {
        None => 0,
        Some(time) => time
            .saturating_duration_since(Instant::now())
            .as_secs()
            .min(u32::MAX as u64) as u32,
    };
    serialize_string(buf, &v.value);
    buf.extend(ttl.to_le_bytes());
}
