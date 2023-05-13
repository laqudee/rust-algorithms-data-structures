mod converter;
mod deque;
mod hot_potato;
mod infix;
mod linked_list;
mod pal_checker;
mod par_checker;
mod queue;
mod stack;

use converter::{base_converter, divide_by_two};
use deque::Deque;
use linked_list::List;
use par_checker::{par_checker1, par_checker2, par_checker3};
use queue::Queue;
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

    // Test Divide By Two
    let bin_str = divide_by_two::work(138);
    println!("10 is binary: {}", bin_str);

    //Test Base Converter
    let bin_str = base_converter::work(138, 2);
    let hex_str = base_converter::work(138, 16);
    println!("10 is binary: {}, hex: {}", bin_str, hex_str);

    // Test infix
    infix::it_work();

    // Test postfix_eval
    infix::eval_work();

    //Test Queue
    let mut q = Queue::new(3);
    let _r1 = q.enqueue(1);
    let _r2 = q.enqueue(2);
    let _r3 = q.enqueue(3);
    if let Err(error) = q.enqueue(4) {
        println!("Enqueue error: {}", error);
    }
    println!("size: {}, empty: {}", q.size(), q.is_empty());
    let de_res1 = q.dequeue();
    println!("de_res1: {:?}, content: {:?}", de_res1, q);

    //Test hot_potato game
    let names = vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let rem = hot_potato::work(names, 23);
    println!("The left person is {rem}");

    // Test Deque
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1);
    let _r2 = d.add_front(2);
    let _r3 = d.add_rear(3);
    let _r4 = d.add_rear(4);

    assert_eq!(d.size(), 4);
    assert_eq!(Err("queue is full".to_string()), d.add_front(5));
    assert_eq!(Some(2), d.remove_front());
    assert_eq!(Some(4), d.remove_rear());
    assert_eq!(false, d.is_empty());

    // Test pal_checker
    let pal = "rustsur";
    let is_pal = pal_checker::work(pal);
    println!("pal is pal: {}", is_pal);

    // Test Link List
    fn basics() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));
        list.peek_mut().map(|val| {
            *val = 4;
        });
        assert_eq!(list.peek(), Some(&4));
    }

    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        println!("into_iter test Ok!");
    }

    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        println!("iter test Ok!");
    }

    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        println!("list size: {}, is empty: {}", list.size(), list.is_empty());
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
        println!("iter_mut test Ok!");
    }

    basics();
    into_iter();
    iter();
    iter_mut();
}
