use crate::queue::Queue;

pub fn work() {
    let mut q = Queue::new(3);
    let _r1 = q.enqueue(1);
    let _r2 = q.enqueue(2);
    let _r3 = q.enqueue(3);
    if let Err(error) = q.enqueue(4) {
        println!("Enqueue error: {}", error);
    }
    println!("size: {}, empty: {}", q.size(), q.is_empty());
    let de_res1 = q.dequeue();
    println!("de_res1: {:?}, content: {:?}", de_res1, q);
}
