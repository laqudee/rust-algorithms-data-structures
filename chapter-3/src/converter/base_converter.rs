/// 将十进制转为二进制、八进制、十六进制
use crate::stack::Stack;

pub fn work(mut dec_num: u32, base: u32) -> String {
    // digits对应各种余数的字符形式，尤其10-15
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut rem_stack = Stack::new();

    // 余数 rem 入栈
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    // 余数出栈并取出对应字符来拼接成字符串
    let mut base_str = String::new();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }
    base_str
}
