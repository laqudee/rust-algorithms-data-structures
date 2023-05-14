/// Rust 实现 栈

#[derive(Debug)]
pub struct Stack<T> {
    top: usize,   // 栈顶
    data: Vec<T>, // 栈数据容器
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val); // 数据保存在Vec末尾
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1; // 栈顶减1，后再弹出数据
        self.data.pop()
    }

    pub fn _peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    pub fn is_empty(&self) -> bool {
        0 == self.top
    }

    pub fn _size(&self) -> usize {
        self.top // 栈顶恰好就是栈中元素个数
    }
}
