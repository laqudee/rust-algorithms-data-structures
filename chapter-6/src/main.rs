/// 排序
mod bubble;
mod quick;
mod insertion;

fn main() {
    // Test 冒泡排序
    bubble::it_work();

    //Test 快速排序
    quick::it_work();

    // Test 插入排序
    insertion::it_work();
}
