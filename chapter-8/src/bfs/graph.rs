use std::cell::RefCell;
use std::rc::Rc;

// 因为节点存在多个共享的链接，Box不可共享，Rc才可共享
// 又因为Rc不可变，所以使用具有内部可变性的RefCell包裹
type Link = Option<Rc<RefCell<Node>>>;

// 节点
pub struct Node {
    pub data: usize,
    pub next: Link,
}

impl Node {
    pub fn new(data: usize) -> Self {
        Self { data, next: None }
    }
}

// 图定义及实现
pub struct Graph {
    first: Link,
    last: Link,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    pub fn get_first(&self) -> Link {
        self.first.clone()
    }

    pub fn print_node(&self) {
        let mut curr = self.first.clone();
        while let Some(val) = curr {
            print!("[{}]", val.borrow().data);
            curr = val.borrow().next.clone();
        }

        print!("\n");
    }

    // 插入节点，RefCell使用borrow_mut修改
    pub fn insert(&mut self, data: usize) {
        let node = Rc::new(RefCell::new(Node::new(data)));
        if self.is_empty() {
            self.first = Some(node.clone());
            self.last = Some(node);
        } else {
            self.last.as_mut().unwrap().borrow_mut().next = Some(node.clone());
            self.last = Some(node);
        }
    }
}
