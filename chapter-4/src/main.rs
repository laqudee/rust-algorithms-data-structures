mod num2str;
mod stack;
mod sum;

fn main() {
    // 简单递归
    sum::it_work();

    // 利用递归将十进制转为2，8，16进制
    num2str::it_work();
}
