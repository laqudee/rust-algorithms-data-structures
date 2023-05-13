/// Rust 实现队列

#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    // 判断是否有剩余空间，有则数据加入到队列
    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err(String::from("No space available!"));
        }
        self.data.insert(0, val);

        Ok(())
    }

    // 数据出栈
    pub fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
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
        let mut q = Queue::new(3);
        let _r1 = q.enqueue(1);
        let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3);
        if let Err(error) = q.enqueue(4) {
            println!("Enqueue error: {}", error);
        }
        assert_eq!(q.size(), 3);
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.is_empty(), false);
    }
}
