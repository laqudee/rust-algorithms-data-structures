/// Rust 实现的链表栈

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        // 初始化时无下一链接
        Node { data, next: None }
    }
}

// 链表栈
#[derive(Debug, Clone)]
pub struct ListStack<T> {
    size: usize,
    top: Link<T>, // 栈顶控制整个栈
}

impl<T: Clone> ListStack<T> {
    pub fn new() -> Self {
        ListStack { size: 0, top: None }
    }

    // take 取出top中节点，留下空位，所以可以回填节点
    pub fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    // as_ref将top节点转为引用对象
    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        0 == self.size
    }
}
