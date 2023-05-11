/// 排序和比较法
pub fn play(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    // s1 和s2中的字符分别加入 alist，blist
    let mut alist = vec![];
    let mut blist = vec![];
    for c in s1.chars() {
        alist.push(c);
    }
    for c in s2.chars() {
        blist.push(c);
    }
    alist.sort();
    blist.sort();

    // 逐个比较排序的几何集合，任何字符不匹配就退出循环
    let mut pos = 0;
    let mut matched = true;
    while pos < alist.len() && matched {
        if alist[pos] == blist[pos] {
            pos += 1;
        } else {
            matched = false;
        }
    }

    matched
}
