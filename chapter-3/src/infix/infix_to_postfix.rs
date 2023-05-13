use crate::{par_checker3, stack::Stack};
/// 中缀表达式转为后缀表达式
use std::collections::HashMap;

pub fn work(infix: &str) -> Option<String> {
    // 括号检查
    if !par_checker3::checker(infix) {
        return None;
    }

    // 设置各个符号的优先级
    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    // ops 保存操作符号、postfix保存后缀表达式
    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();
    for token in infix.split_whitespace() {
        // 0-9 和 A-Z 范围字符入栈
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            postfix.push(token);
        } else if "(" == token {
            // 遇到开符号，将操作符入栈
            op_stack.push(token);
        } else if ")" == token {
            // 遇到闭符号，将操作数入栈
            let mut top = op_stack.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = op_stack.pop().unwrap();
            }
        } else {
            // 比较符号优先级来决定操作符是否加入postfix
            while (!op_stack.is_empty()) && (prec[op_stack.peek().unwrap()] >= prec[token]) {
                postfix.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
        }
    }

    // 剩下的操作数入栈
    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap())
    }

    // 出栈并组成字符串
    let mut postfix_str = String::from("");
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }

    Some(postfix_str)
}
