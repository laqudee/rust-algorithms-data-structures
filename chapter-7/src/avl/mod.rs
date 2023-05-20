/// avl 二叉平衡树

// AVL 树 定义
#[derive(Debug)]
enum AvlTree<T> {
    Null,
    Tree(Box<AvlTree<T>>),
}

// AVL 树 节点定义
#[derive(Debug)]
pub struct AvlNode<T> {
    val: T,
    left: AvlTree<T>,
    right: AvlTree<T>,
    bfactor: i8,
}

pub fn it_work() {}
