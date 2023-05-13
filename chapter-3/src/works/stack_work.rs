use crate::{
    par_checker::{par_checker1, par_checker2, par_checker3},
    stack::Stack,
};

pub fn work() {
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
    println!("checker1 res1: {}, res2: {}", res1, res2);

    // Test Par Checker2
    let sa = "(){}[]";
    let sb = "(){)[}";
    let sc = "({()}[{}])";
    let res1 = par_checker2::checker(sa);
    let res2 = par_checker2::checker(sb);
    let res3 = par_checker2::checker(sc);
    println!("checker2 res1: {}, res2: {}, res3: {}", res1, res2, res3);

    // Test Par Checker3
    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    let res1 = par_checker3::checker(sa);
    let res2 = par_checker3::checker(sb);
    println!("checker3 res1: {}, res2: {}", res1, res2);
}
