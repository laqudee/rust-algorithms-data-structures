mod converter;
mod deque;
mod infix;
mod linked_list;
mod par_checker;
mod queue;
mod stack;
mod works;
mod list_stack;

use converter::{base_converter, divide_by_two};
use works::{
    deque_work, hot_potato, link_list_work, list_stack_work, pal_checker, queue_work, stack_work,
};

fn main() {
    // Test Stack
    stack_work::work();

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
    queue_work::work();

    //Test hot_potato game
    let names = vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let rem = hot_potato::work(names, 23);
    println!("The left person is {rem}");

    // Test Deque
    deque_work::work();

    // Test pal_checker
    let pal = "rustsur";
    let is_pal = pal_checker::work(pal);
    println!("pal is pal: {}", is_pal);

    // Test Link List
    link_list_work::work();

    // Test List Stack
    list_stack_work::work();
}
