/// 排序
mod bubble;
mod insertion;
mod merge;
mod quick;
mod selection;
mod shell;

fn main() {
    // Test 冒泡排序
    bubble::it_work();

    //Test 快速排序
    quick::it_work();

    // Test 插入排序
    insertion::it_work();

    // Test 希尔排序
    shell::it_work();

    // Test 归并排序
    merge::it_work();

    // Test 选择排序
    selection::it_work();
}
