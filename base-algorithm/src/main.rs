use std::time::SystemTime;

mod anagram;

fn sum_of_n(n: i64) -> i64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn sum_of_n2(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn test_sum_of_n() {
    for _i in 0..5 {
        let now = SystemTime::now();
        let _sum = sum_of_n(500000);
        let duration = now.elapsed().unwrap();
        let time = duration.as_millis();
        println!("func used {time} ms");
    }
}

fn test_sum_of_n2() {
    for _i in 0..5 {
        let now = SystemTime::now();
        let _sum = sum_of_n2(500000);
        let duration = now.elapsed().unwrap();
        let time = duration.as_millis();
        println!("func used {time} ms");
    }
}

fn main() {
    test_sum_of_n();
    test_sum_of_n2();

    let s1 = "rust";
    let s2 = "trus";
    let res2 = anagram::solution2::play(s1, s2);
    let res3 = anagram::solution3::play(s1, s2);
    let res4 = anagram::solution4::play(s1, s2);
    println!("s1 and s2 is anagram: {res2}");
    println!("s1 and s2 is anagram: {res3}");
    println!("s1 and s2 is anagram: {res4}");
}
