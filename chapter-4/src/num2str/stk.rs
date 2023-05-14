/// 使用栈来替代递归代码
use crate::stack::Stack;

pub fn work(mut num: i32, base: i32) -> String {
    let digits: [&str; 16] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];

    let mut rem_stack = Stack::new();
    while num > 0 {
        if num < base {
            rem_stack.push(num); // 不超过base直接入栈
        } else {
            // 超过base余数入栈
            rem_stack.push(num % base)
        }
        num /= base;
    }

    // 出栈余数并组成字符串
    let mut numstr = String::from("");
    while !rem_stack.is_empty() {
        numstr += digits[rem_stack.pop().unwrap() as usize];
    }

    numstr
}
