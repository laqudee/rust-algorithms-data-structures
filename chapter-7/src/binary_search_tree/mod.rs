/// 二叉查找树
use std::cmp::Ordering;
// use std::ops::Deref;

// 二叉查找树子节点链接
type Link<T, U> = Option<Box<BST<T, U>>>;

// 二叉查找树定义
pub struct BST<T, U> {
    key: Option<T>,
    val: Option<U>,
    left: Link<T, U>,
    right: Link<T, U>,
}

impl<T, U> BST<T, U>
where
    T: Clone + Ord + std::fmt::Debug,
    U: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        BST {
            key: None,
            val: None,
            left: None,
            right: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    pub fn len(&self) -> usize {
        self.calc_len(0)
    }

    // 递归计算节点个数
    pub fn calc_len(&self, mut i: usize) -> usize {
        if self.key.is_none() {
            return i;
        }

        // 当前节点加入总节点数i
        i += 1;

        // 计算左右子节点数
        if !self.left.is_none() {
            i = self.left.as_ref().unwrap().calc_len(i);
        }
        if !self.right.is_none() {
            i = self.right.as_ref().unwrap().calc_len(i);
        }

        i
    }

    // 前中后序遍历
    pub fn preorder(&self) {
        println!("key:{:#?}, val:{:#?}", &self.key, &self.val);
        match &self.left {
            Some(node) => node.preorder(),
            None => (),
        }
        match &self.right {
            Some(node) => node.preorder(),
            None => (),
        }
    }

    pub fn inorder(&self) {
        match &self.left {
            Some(node) => node.inorder(),
            None => (),
        }
        println!("key:{:#?}, val:{:#?}", &self.key, &self.val);
        match &self.right {
            Some(node) => node.inorder(),
            None => (),
        }
    }

    pub fn postorder(&self) {
        match &self.left {
            Some(node) => node.postorder(),
            None => (),
        }
        match &self.right {
            Some(node) => node.postorder(),
            None => (),
        }
        println!("key:{:#?}, val:{:#?}", &self.key, &self.val);
    }

    pub fn insert(&mut self, key: T, val: U) {
        // 没数据直接插入
        if self.key.is_none() {
            self.key = Some(key);
            self.val = Some(val);
        } else {
            match &self.key {
                Some(k) => {
                    // 存在key，更新val
                    if key == *k {
                        self.val = Some(val);
                        return;
                    }

                    // 未找到key，需要插入新节点
                    // 先找到需要插入的子树
                    let child = if key < *k {
                        &mut self.left
                    } else {
                        &mut self.right
                    };

                    // 根据节点递归下去，直到插入
                    match child {
                        Some(ref mut node) => {
                            node.insert(key, val);
                        }
                        None => {
                            let mut node = BST::new();
                            node.insert(key, val);
                            *child = Some(Box::new(node));
                        }
                    }
                }
                None => (),
            }
        }
    }

    pub fn search(&self, key: &T) -> bool {
        match &self.key {
            Some(k) => {
                // 比较key值，并判断是否继续递归查找
                match k.cmp(&key) {
                    Ordering::Less => {
                        // 在右子树查找
                        match &self.right {
                            Some(node) => node.search(key),
                            None => false,
                        }
                    }
                    Ordering::Greater => {
                        // 在左子树查找
                        match &self.left {
                            Some(node) => node.search(key),
                            None => false,
                        }
                    }
                    Ordering::Equal => true, // 找到数据
                }
            }
            None => false,
        }
    }

    pub fn min(&self) -> (Option<&T>, Option<&U>) {
        // 最小值一定在最左侧
        match &self.left {
            Some(node) => node.min(),
            None => match &self.key {
                Some(k) => (Some(&k), self.val.as_ref()),
                None => (None, None),
            },
        }
    }

    pub fn max(&self) -> (Option<&T>, Option<&U>) {
        // 最大值一定在最右侧
        match &self.right {
            Some(node) => node.max(),
            None => match &self.key {
                Some(k) => (Some(&k), self.val.as_ref()),
                None => (None, None),
            },
        }
    }

    // 获取值，和查找流程相似
    pub fn get(&self, key: &T) -> Option<&U> {
        match &self.key {
            None => None,
            Some(k) => match k.cmp(&key) {
                Ordering::Equal => self.val.as_ref(),
                Ordering::Greater => match &self.left {
                    None => None,
                    Some(node) => node.get(key),
                },
                Ordering::Less => match &self.right {
                    None => None,
                    Some(node) => node.get(key),
                },
            },
        }
    }
}

pub fn it_work() {
    let mut bst = BST::<i32, char>::new();
    bst.insert(8, 'e');
    bst.insert(6, 'c');
    bst.insert(7, 'd');
    bst.insert(5, 'b');
    bst.insert(10, 'g');
    bst.insert(9, 'f');
    bst.insert(11, 'h');
    bst.insert(4, 'a');

    println!("empty: {:?}, len: {:?}", bst.is_empty(), bst.len());
    println!("max: {:?}, min: {:?}", bst.max(), bst.min());
    println!("key: 5, val: {:?}", bst.get(&5));
    println!("5 in bst: {:?}", bst.search(&5));

    println!("inorder: ");
    bst.inorder();
    println!("preorder: ");
    bst.preorder();
    println!("postorder: ");
    bst.postorder();
}
