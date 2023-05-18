/// Rust 实现树
use std::fmt::{Debug, Display};

// 子节点连接
type Link<T> = Option<Box<BinaryTree<T>>>;

// 二叉树定义
// key保存数据
// left和right保存左右子节点链接
#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Clone> BinaryTree<T> {
    pub fn new(key: T) -> Self {
        BinaryTree {
            key,
            left: None,
            right: None,
        }
    }

    // 新子节点作为跟节点的左子节点
    pub fn insert_left_tree(&mut self, key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    // 新子节点作为根节点的右子节点
    pub fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    // 获取左右子节点及根节点，注意使用了clone
    pub fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    pub fn get_key(&self) -> T {
        self.key.clone()
    }

    pub fn set_key(&mut self, key: T) {
        self.key = key;
    }
}

pub fn it_work() {
    let mut bt = BinaryTree::new('a');

    let root = bt.get_key();
    println!("root val is {:?}", root);

    let left = bt.get_left();
    println!("left child is {:#?}", left);

    let right = bt.get_right();
    println!("right child is {:#?}", right);

    bt.insert_left_tree('b');
    bt.insert_right_tree('e');

    let left = bt.get_left();
    println!("left child is {:#?}", left);
    let right = bt.get_right();
    println!("right child is {:#?}", right);
}
