use std::{sync::Arc, thread};

use redis::queue::Queue;

#[test]
fn test_queue() {
    let q: Arc<Queue<i8>> = Arc::new(Queue::new());

    let pq = Arc::clone(&q);
    let producer = thread::spawn(move || {
        pq.push(1);
        pq.push(2);
    });

    let cq = Arc::clone(&q);
    let consumer = thread::spawn(move || {
        assert_eq!(cq.wait_pop(), 1);
        assert_eq!(cq.wait_pop(), 2);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}
