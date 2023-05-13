use crate::deque::Deque;

/// 利用双端队列进行回文检查
/// 首部和尾部共同出栈策略

pub fn work(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());
    for c in pal.chars() {
        let _r = d.add_rear(c);
    }

    let mut is_pal = true;
    while d.size() > 1 && is_pal {
        let head = d.remove_front();
        let tail = d.remove_rear();
        if head != tail {
            is_pal = false;
        }
    }

    is_pal
}
