mod par_checker1;
mod par_checker2;
mod stack;

use stack::Stack;

fn main() {
    // Test Stack
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(4);
    println!("top {:?}, size {:?}", s.peek().unwrap(), s.size());
    println!("pop {:?}, size {:?}", s.pop().unwrap(), s.size());
    println!("is_empty: {}, stack:{:?}", s.is_empty(), s);

    // Test Par Checker1
    let sa = "()(())";
    let sb = "()((())";
    let res1 = par_checker1::checker(sa);
    let res2 = par_checker1::checker(sb);
    println!("res1: {}, res2: {}", res1, res2);

    // Test Par Checker2
    let sa = "(){}[]";
    let sb = "(){)[}";
    let sc = "({()}[{}])";
    let res1 = par_checker2::checker(sa);
    let res2 = par_checker2::checker(sb);
    let res3 = par_checker2::checker(sc);
    println!("res1: {}, res2: {}, res3: {}", res1, res2, res3);
}
