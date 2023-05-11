pub fn play(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    // 大小为26的集合，用于将字符映射为ASCII值
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];
    for c in s1.chars() {
        let pos = (c as usize) - 97; // 97为a的ASCII值
        c1[pos] += 1;
    }
    for c in s2.chars() {
        let pos = (c as usize) - 97;
        c2[pos] += 1;
    }

    // 逐个比较ascii的值
    let mut pos = 0;
    let mut ok = true;
    while pos < 26 && ok {
        if c1[pos] == c2[pos] {
            pos += 1;
        } else {
            ok = false;
        }
    }

    ok
}
