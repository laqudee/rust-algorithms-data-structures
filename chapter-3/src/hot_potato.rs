/// 烫手山芋游戏
use crate::queue::Queue;

pub fn work(names: Vec<&str>, num: usize) -> &str {
    let mut q = Queue::new(names.len());
    for name in names {
        let _rm = q.enqueue(name);
    }

    while q.size() > 1 {
        for _ in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        // 出入栈达到num此事，删除一人
        let _rm = q.dequeue();
    }

    q.dequeue().unwrap()
}
