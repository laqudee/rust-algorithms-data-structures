mod avl;
mod binary_heap;
mod binary_search_tree;
mod binary_tree;

fn main() {
    // Test 树
    binary_tree::it_work();

    // Test 二叉堆
    binary_heap::it_work();

    // Test 二叉查找树
    binary_search_tree::it_work();

    // Test AVL二叉平衡树
    avl::it_work();
}
