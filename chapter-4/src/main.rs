mod dynamic_planning;
mod move2tower;
mod num2str;
mod stack;
mod sum;

fn main() {
    // 简单递归
    sum::it_work();

    // 利用递归将十进制转为2，8，16进制
    num2str::it_work();

    // 利用递归解决汉诺塔问题
    move2tower::work(1, "A", "B", "C");
    move2tower::work(2, "A", "B", "C");
    move2tower::work(3, "A", "B", "C");
    move2tower::work(4, "A", "B", "C");

    // rust 实现 贪婪方法 找零
    dynamic_planning::it_work()
}
