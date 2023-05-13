/// Rust实现双端队列
/// 使用Vec，左端作为队尾；右端作为队首
/// cap用于控制双端队列的长度

#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Deque<T> {
        Deque {
            cap,
            data: Vec::with_capacity(cap),
        }
    }

    // Vec 末尾为队首
    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err(String::from("queue is full"));
        }
        self.data.push(val);

        Ok(())
    }

    // Vev 首部作为队尾
    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err(String::from("queue is full"));
        }

        self.data.insert(0, val);

        Ok(())
    }

    // 从队首移除数据
    pub fn remove_front(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // 从队尾移除数据
    pub fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
}
