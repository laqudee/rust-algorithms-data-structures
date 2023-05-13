pub mod infix_to_postfix;
pub mod postfix_eval;

// Test infix_to_postfix
pub fn it_work() {
    let infix = "( A + B ) * ( C + D )";
    let postfix = infix_to_postfix::work(infix);
    match postfix {
        Some(val) => {
            println!("infix: {infix} -> postfix: {val}");
        }
        None => {
            println!("infix: {infix} -> postfix: None");
        }
    }
}

// Test postfix_eval
pub fn eval_work() {
    let postfix = "1 2 + 1 2 + *";
    let res = postfix_eval::eval(postfix);
    match res {
        Some(val) => println!("res is {val}"),
        None => println!("{postfix} isn't a valid postfix expression"),
    }
}
