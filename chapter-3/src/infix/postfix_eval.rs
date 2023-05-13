/// 计算后缀表达式的算法
use crate::stack::Stack;

pub fn eval(postfix: &str) -> Option<i32> {
    // 少于五个字符，不是有效的后缀表达式，因为表达式至少两个操作数加一个操作符，还需要2个空格隔开
    if postfix.len() < 5 {
        return None;
    }

    let mut op_stack = Stack::new();
    for token in postfix.split_whitespace() {
        if "0" <= token && token <= "9" {
            op_stack.push(token.parse::<i32>().unwrap());
        } else {
            // 对于剑法和除法，顺序有要求
            // 所以先出栈的是第二个操作数
            let op2 = op_stack.pop().unwrap();
            let op1 = op_stack.pop().unwrap();
            let res = do_calc(token, op1, op2);
            op_stack.push(res);
        }
    }
    Some(op_stack.pop().unwrap())
}

// 执行四则运算
fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    match op {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        _ => {
            if 0 == op2 {
                panic!("除数不能为0");
            }
            op1 / op2
        }
    }
}
