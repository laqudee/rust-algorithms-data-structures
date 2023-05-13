use crate::stack::Stack;
use crate::par_checker::par_checker2::par_match;

/// 处理含有非开闭符号的匹配问题

pub fn checker(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];

        // 开符号入栈
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }

        // 闭符号则判断是否平衡
        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }

        // 非括号直接跳过
        index += 1;
    }

    balance && stack.is_empty()
}
