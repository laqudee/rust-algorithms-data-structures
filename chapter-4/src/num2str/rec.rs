/// num2str_rec
/// 整数到任意进制（2-16）字符串的转换算法

const BASESTR: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

pub fn work(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        // 余数加在末尾
        work(num / base, base) + BASESTR[(num % base) as usize]
    }
}
