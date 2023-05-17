/// 排序
mod bubble;
mod bucket;
mod counting;
mod heap;
mod insertion;
mod merge;
mod quick;
mod radix;
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

    // Test 堆排序
    heap::it_work();

    //Test 桶排序
    bucket::it_work();

    // Test 计数排序
    counting::it_work();

    // Test 基数排序
    radix::it_work();
}
