use crate::deque::Deque;

pub fn work() {
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1);
    let _r2 = d.add_front(2);
    let _r3 = d.add_rear(3);
    let _r4 = d.add_rear(4);

    assert_eq!(d.size(), 4);
    assert_eq!(Err("queue is full".to_string()), d.add_front(5));
    assert_eq!(Some(2), d.remove_front());
    assert_eq!(Some(4), d.remove_rear());
    assert_eq!(false, d.is_empty());
}
